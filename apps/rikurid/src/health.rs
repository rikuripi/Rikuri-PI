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

use crate::entities::{Owner, Project, Run, RunStatus, Task, TaskStatus};

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

    /// The store's health from its evidence. Reads and writes are judged
    /// separately, so a successful read (including this check's own
    /// consistency replay) can never hide a failing write. Refused requests
    /// are not failures.
    pub fn from_store(component: &'static str, h: &StoreHealth) -> ComponentHealth {
        let (failing, n, error) = if h.writes.consecutive_failures > 0 {
            ("write", h.writes.consecutive_failures, &h.writes.last_error)
        } else {
            ("read", h.reads.consecutive_failures, &h.reads.last_error)
        };
        let (state, detail) = if n > 0 {
            (
                if n < 3 {
                    HealthState::Degraded
                } else {
                    HealthState::Critical
                },
                format!(
                    "the last {n} {failing} transaction(s) failed: {}",
                    error.as_deref().unwrap_or("no detail")
                ),
            )
        } else if h.last_success.is_none() {
            (
                HealthState::Degraded,
                "no successful transaction yet".to_owned(),
            )
        } else {
            (
                HealthState::Healthy,
                format!(
                    "{} reads and {} writes succeeded; {} requests refused",
                    h.reads.succeeded, h.writes.succeeded, h.refused
                ),
            )
        };
        ComponentHealth {
            component,
            state,
            detail,
            last_attempt: h.last_attempt,
            last_success: h.last_success,
            last_error: h.last_error.clone(),
            consecutive_failures: n,
            work_completed: h.work_completed,
        }
    }

    /// The `writes` component while the daemon is quarantined: Critical, so
    /// overall health is never green while writes are refused.
    pub fn quarantined(reason: &str, now: Timestamp) -> ComponentHealth {
        ComponentHealth {
            component: "writes",
            state: HealthState::Critical,
            detail: format!("refused (quarantine): {reason}"),
            last_attempt: Some(now),
            last_success: None,
            last_error: Some(reason.to_owned()),
            consecutive_failures: 0,
            work_completed: 0,
        }
    }

    /// Whether this is a production build. A test build (compiled with the
    /// `crash-injection` feature, whether or not a point is armed; crash
    /// injection armed; or idempotency disabled for a regression test) is
    /// Degraded so it can never pass for a real daemon. (2026-10-08,
    /// verifier: an unarmed `crash-injection` build said "production
    /// configuration".)
    pub fn build(
        test_build: bool,
        crash_armed: bool,
        idempotency_enforced: bool,
        now: Timestamp,
    ) -> ComponentHealth {
        let mut c = ComponentHealth::new("build");
        c.last_attempt = Some(now);
        if !idempotency_enforced {
            c.state = HealthState::Critical;
            c.detail = "idempotency disabled (regression-test build)".into();
        } else if crash_armed {
            c.detail = "crash injection armed (test build)".into();
        } else if test_build {
            c.detail = "crash-injection test build (no crash point armed)".into();
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

    let runs: HashMap<RunId, Run> = SqliteRepository::<Run>::new()
        .list(tx, u32::MAX)?
        .into_iter()
        .map(|r| (r.value.id, r.value))
        .collect();
    for run in runs.values() {
        check_run(run, &events, &mut problems);
    }
    check_run_events(&runs, &events, &mut problems);
    let tasks: HashMap<TaskId, &Task> = tasks.iter().map(|t| (t.value.id, &t.value)).collect();
    check_task_runs(&tasks, &runs, &mut problems);
    check_owner(tx, &events, &mut problems)?;
    Ok(ConsistencyReport {
        events: replayed,
        problems,
    })
}

/// The other direction (2026-10-08, crash matrix): every run event in the
/// ledger must be one that state links, and what the ledger recorded
/// (outcome, verification, steps) must be what state shows. Following only
/// the run's own links missed a completion that state forgot or an outcome
/// it changed.
fn check_run_events(
    runs: &HashMap<RunId, Run>,
    events: &HashMap<EventId, Event>,
    problems: &mut Vec<String>,
) {
    let mut steps: HashMap<RunId, u32> = HashMap::new();
    for e in events.values() {
        let Some(run_id) = e.run_id else {
            continue;
        };
        let kind = e.event_type.as_str();
        if kind == "task.stepped" {
            *steps.entry(run_id).or_default() += 1;
        }
        if !matches!(kind, "run.started" | "run.completed" | "run.verified") {
            continue;
        }
        let Some(run) = runs.get(&run_id) else {
            problems.push(format!(
                "the ledger has {kind} event {} for run {run_id}, which does not exist",
                e.event_id
            ));
            continue;
        };
        let linked = match kind {
            "run.started" => Some(run.started_event_id),
            "run.completed" => run.completion_event_id,
            _ => run.verification_event_id,
        };
        if linked != Some(e.event_id) {
            problems.push(format!(
                "run {run_id} does not link its {kind} event {}",
                e.event_id
            ));
        }
        let recorded = |field: &str| e.metadata.get(field).and_then(|v| v.as_str());
        match kind {
            "run.completed" if recorded("outcome") != run.outcome.map(|o| o.as_str()) => {
                problems.push(format!(
                    "run {run_id} outcome is {:?} but its run.completed event recorded {:?}",
                    run.outcome.map(|o| o.as_str()),
                    recorded("outcome")
                ));
            }
            "run.verified" if recorded("verification") != run.verification.map(|v| v.as_str()) => {
                problems.push(format!(
                    "run {run_id} verification is {:?} but its run.verified event recorded {:?}",
                    run.verification.map(|v| v.as_str()),
                    recorded("verification")
                ));
            }
            _ => {}
        }
    }
    for run in runs.values() {
        let in_ledger = steps.get(&run.id).copied().unwrap_or(0);
        if in_ledger != run.steps {
            problems.push(format!(
                "run {} records {} steps but the ledger has {in_ledger} task.stepped events for it",
                run.id, run.steps
            ));
        }
    }
}

/// State's own pointers between tasks and runs must agree (2026-10-08,
/// verifier: a task that stopped pointing at its running run passed). A
/// task is running exactly when it has a current run, which is a running run
/// of that task; every running run is its task's current run; a task's last
/// run exists and is its own, and a task with runs has one.
fn check_task_runs(
    tasks: &HashMap<TaskId, &Task>,
    runs: &HashMap<RunId, Run>,
    problems: &mut Vec<String>,
) {
    for task in tasks.values() {
        let id = task.id;
        match (task.status, task.current_run) {
            (TaskStatus::Running, None) => {
                problems.push(format!("task {id} is running but has no current run"));
            }
            (TaskStatus::Created | TaskStatus::Completed, Some(r)) => problems.push(format!(
                "task {id} is {} but has current run {r}",
                task.status
            )),
            _ => {}
        }
        if let Some(r) = task.current_run {
            match runs.get(&r) {
                None => problems.push(format!("task {id} current run {r} does not exist")),
                Some(run) if run.task_id != id || run.status != RunStatus::Running => problems
                    .push(format!(
                        "task {id} current run {r} is not a running run of that task"
                    )),
                Some(_) => {}
            }
        }
        match task.last_run {
            None if task.runs > 0 => problems.push(format!(
                "task {id} records {} runs but no last run",
                task.runs
            )),
            Some(r) => match runs.get(&r) {
                None => problems.push(format!("task {id} last run {r} does not exist")),
                Some(run) if run.task_id != id => {
                    problems.push(format!("task {id} last run {r} belongs to another task"));
                }
                Some(_) => {}
            },
            None => {}
        }
    }
    for run in runs.values() {
        match tasks.get(&run.task_id) {
            None => problems.push(format!(
                "run {} belongs to task {}, which does not exist",
                run.id, run.task_id
            )),
            Some(task) if run.status == RunStatus::Running && task.current_run != Some(run.id) => {
                problems.push(format!(
                    "run {} is running but task {} does not show it as its current run",
                    run.id, task.id
                ));
            }
            Some(_) => {}
        }
    }
}

/// The owner and its project must be the ones `system.initialized`
/// recorded, and there is an owner exactly when the ledger says the home
/// was initialized (2026-10-08, verifier: nothing compared them).
fn check_owner(
    tx: &Tx<'_>,
    events: &HashMap<EventId, Event>,
    problems: &mut Vec<String>,
) -> Result<(), StorageError> {
    let owners = SqliteRepository::<Owner>::new().list(tx, u32::MAX)?;
    let inits: Vec<&Event> = events
        .values()
        .filter(|e| e.event_type == "system.initialized")
        .collect();
    match (owners.as_slice(), inits.as_slice()) {
        ([], []) => {}
        ([], _) => problems.push(format!(
            "the ledger has {} system.initialized event(s) but there is no owner",
            inits.len()
        )),
        ([o], [e]) => {
            let owner = &o.value;
            if e.actor_id != owner.id || e.project_id != Some(owner.project_id) {
                problems.push(format!(
                    "owner {} with project {} is not the one its system.initialized event {} recorded",
                    owner.id, owner.project_id, e.event_id
                ));
            }
            if SqliteRepository::<Project>::new()
                .get(tx, owner.project_id)?
                .is_none()
            {
                problems.push(format!(
                    "the owner's project {} does not exist",
                    owner.project_id
                ));
            }
        }
        _ => problems.push(format!(
            "{} owner(s) and {} system.initialized event(s); expected one of each",
            owners.len(),
            inits.len()
        )),
    }
    Ok(())
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
