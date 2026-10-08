//! Health on evidence, not liveness (Architecture §27.8, §53.7; PRD
//! SEC-006/007/030). Each component reports its last attempt and last
//! success. A component that has never succeeded is Degraded, not Healthy:
//! absence of success is a finding.
//!
//! The ledger-consistency check replays history and compares it with
//! current state. It is also the kernel's no-duplicate-effect check for the
//! crash matrix (W1): a consequential effect applied twice shows up as state
//! or events that disagree.

use std::collections::{BTreeMap, HashMap, HashSet};

use rikuri_domain::{EventId, RunId, TaskId, Timestamp, wire_enum};
use rikuri_ledger::Event;
use rikuri_storage::{Repository, SqliteRepository, StorageError, StoreHealth, Tx};
use serde::Serialize;
use serde_json::{Value, json};

use crate::entities::{Run, RunStatus, Task};

wire_enum! {
    /// Ordered from best to worst.
    pub enum HealthState {
        Healthy => "healthy",
        Degraded => "degraded",
        Critical => "critical",
    }
}

/// One component's health evidence (Engineering Templates, Template C).
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ComponentHealth {
    pub component: &'static str,
    pub state: HealthState,
    pub detail: String,
    pub last_attempt: Option<Timestamp>,
    pub last_success: Option<Timestamp>,
    pub last_error: Option<String>,
    pub consecutive_failures: u64,
    pub work_completed: u64,
}

impl ComponentHealth {
    /// A component with no evidence yet: Degraded until it succeeds.
    pub fn new(component: &'static str) -> ComponentHealth {
        ComponentHealth {
            component,
            state: HealthState::Degraded,
            detail: "no successful check yet".into(),
            last_attempt: None,
            last_success: None,
            last_error: None,
            consecutive_failures: 0,
            work_completed: 0,
        }
    }

    /// Records one check; `work` is what the check covered (e.g. events).
    pub fn record(&mut self, at: Timestamp, ok: bool, error: Option<String>, work: u64) {
        self.last_attempt = Some(at);
        if ok {
            self.last_success = Some(at);
            self.consecutive_failures = 0;
            self.work_completed = work;
            self.state = HealthState::Healthy;
            self.detail = format!("{work} events agree with current state");
        } else {
            self.consecutive_failures += 1;
            self.last_error = error.clone();
            self.state = HealthState::Critical;
            self.detail = error.unwrap_or_else(|| "check failed".into());
        }
    }

    pub fn from_store(component: &'static str, h: &StoreHealth) -> ComponentHealth {
        let (state, detail) = match (h.last_success, h.consecutive_failures) {
            (None, 0) => (
                HealthState::Degraded,
                "no successful transaction yet".to_owned(),
            ),
            (_, 0) => (
                HealthState::Healthy,
                format!("{} transactions succeeded", h.work_completed),
            ),
            (_, n) if n < 3 => (
                HealthState::Degraded,
                format!("{n} recent transaction failures"),
            ),
            (_, n) => (
                HealthState::Critical,
                format!("{n} consecutive transaction failures"),
            ),
        };
        ComponentHealth {
            component,
            state,
            detail,
            last_attempt: h.last_attempt,
            last_success: h.last_success,
            last_error: h.last_error.clone(),
            consecutive_failures: h.consecutive_failures,
            work_completed: h.work_completed,
        }
    }

    /// Whether this is a production build. A test build (crash injection
    /// armed, or idempotency disabled for a regression test) is Degraded so
    /// it can never pass for a real daemon.
    pub fn build(crash_armed: bool, idempotency_enforced: bool, now: Timestamp) -> ComponentHealth {
        let mut c = ComponentHealth::new("build");
        c.last_attempt = Some(now);
        if !idempotency_enforced {
            c.state = HealthState::Critical;
            c.detail = "idempotency disabled (regression-test build)".into();
        } else if crash_armed {
            c.detail = "crash injection armed (test build)".into();
        } else {
            c.state = HealthState::Healthy;
            c.detail = "production configuration".into();
            c.last_success = Some(now);
        }
        c
    }
}

/// The overall report: the worst component state, then every component.
pub fn report(components: &[ComponentHealth]) -> Value {
    let worst = components
        .iter()
        .map(|c| c.state)
        .max()
        .unwrap_or(HealthState::Degraded);
    json!({ "state": worst, "components": components })
}

/// What the replay found.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct ConsistencyReport {
    pub events: u64,
    pub problems: Vec<String>,
}

impl ConsistencyReport {
    pub fn failed(reason: String) -> ConsistencyReport {
        ConsistencyReport {
            events: 0,
            problems: vec![format!("consistency check could not run: {reason}")],
        }
    }
}

#[derive(Default)]
struct TaskHistory {
    created: u32,
    stepped: u32,
    started: u32,
}

/// Replays every event and compares it with tasks and runs.
pub(crate) fn consistency(tx: &Tx<'_>) -> Result<ConsistencyReport, StorageError> {
    let mut problems = vec![];
    let mut by_task: BTreeMap<TaskId, TaskHistory> = BTreeMap::new();
    let mut events: HashMap<EventId, Event> = HashMap::new();
    let mut effects: HashSet<(String, String)> = HashSet::new();
    let replayed = rikuri_ledger::replay(tx, |e| {
        if let Some(task) = e.task_id {
            let h = by_task.entry(task).or_default();
            match e.event_type.as_str() {
                "task.created" => h.created += 1,
                "task.stepped" => h.stepped += 1,
                "run.started" => h.started += 1,
                _ => {}
            }
        }
        if let Some(key) = &e.idempotency_key
            && !effects.insert((key.clone(), e.event_type.clone()))
        {
            problems.push(format!(
                "duplicate effect: two {} events carry idempotency key {key}",
                e.event_type
            ));
        }
        events.insert(e.event_id, e.clone());
        Ok::<(), rikuri_ledger::LedgerError>(())
    });
    let replayed = match replayed {
        Ok(n) => n,
        Err(e) => return Ok(ConsistencyReport::failed(e.to_string())),
    };

    let tasks = SqliteRepository::<Task>::new().list(tx, u32::MAX)?;
    let task_ids: HashSet<TaskId> = tasks.iter().map(|t| t.value.id).collect();
    for t in &tasks {
        let task = &t.value;
        let h = by_task.get(&task.id);
        let (created, stepped, started) =
            h.map_or((0, 0, 0), |h| (h.created, h.stepped, h.started));
        if created != 1 {
            problems.push(format!(
                "task {} has {created} task.created events, expected 1",
                task.id
            ));
        }
        if stepped != task.steps_completed {
            problems.push(format!(
                "task {} records {} steps but the ledger has {stepped} task.stepped events",
                task.id, task.steps_completed
            ));
        }
        if started != task.runs {
            problems.push(format!(
                "task {} records {} runs but the ledger has {started} run.started events",
                task.id, task.runs
            ));
        }
    }
    for task in by_task.keys() {
        if !task_ids.contains(task) {
            problems.push(format!(
                "ledger references task {task}, which does not exist"
            ));
        }
    }

    for r in SqliteRepository::<Run>::new().list(tx, u32::MAX)? {
        check_run(&r.value, &events, &mut problems);
    }
    Ok(ConsistencyReport {
        events: replayed,
        problems,
    })
}

/// A run's event links must point at events of the right type about the
/// same task and run; a verification must follow its completion.
fn check_run(run: &Run, events: &HashMap<EventId, Event>, problems: &mut Vec<String>) {
    let about_run = |id: EventId, kind: &str, run_id: RunId| match events.get(&id) {
        None => Some(format!(
            "run {run_id} links {kind} event {id}, which is not in the ledger"
        )),
        Some(e)
            if e.event_type != kind
                || e.run_id != Some(run_id)
                || e.task_id != Some(run.task_id) =>
        {
            Some(format!(
                "run {run_id} links event {id}, which is not its {kind} event"
            ))
        }
        Some(_) => None,
    };
    problems.extend(about_run(run.started_event_id, "run.started", run.id));
    match run.status {
        RunStatus::Running => {
            if run.completion_event_id.is_some() || run.verification_event_id.is_some() {
                problems.push(format!("running run {} has completion links", run.id));
            }
        }
        RunStatus::Completed => match (run.completion_event_id, run.verification_event_id) {
            (Some(c), Some(v)) => {
                problems.extend(about_run(c, "run.completed", run.id));
                problems.extend(about_run(v, "run.verified", run.id));
                if events.get(&v).and_then(|e| e.caused_by) != Some(c) {
                    problems.push(format!(
                        "run {} verification does not follow its completion",
                        run.id
                    ));
                }
            }
            _ => problems.push(format!(
                "completed run {} is missing a completion or verification link",
                run.id
            )),
        },
    }
}
