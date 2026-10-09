//! The kernel: every command, in-process. `Kernel::handle` is the
//! in-process adapter; the daemon's socket server passes each request to the
//! same function, so both adapters share all kernel code (T00.09).
//!
//! A consequential command is one transaction (Architecture §8):
//! idempotency check → state change → event append → idempotency record →
//! commit. A retry with the same key after any crash point returns the
//! stored response and changes nothing.

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use rikuri_api::command::{
    self, LedgerTail, NoParams, TaskComplete, TaskCreate, TaskStart, TaskStep,
};
use rikuri_api::{ApiError, ENVELOPE_VERSION, ErrorCode, Request, Response};
use rikuri_domain::{
    ActorId, Clock, DeletionUnitId, EventId, ExecutionIntent, IdSource, PrivacyClass, ProjectId,
    ProjectTrust, RiskClass, RunId, TaskId, Timestamp, bounded,
};
use rikuri_ledger::{
    EventStatus, EventType, IdempotencyCheck, IdempotencyKey, LedgerError, NewEvent,
};
use rikuri_storage::{Repository, SqliteRepository, StorageError, Store, StoreFailure, Tx};
use serde::de::DeserializeOwned;
use serde_json::{Map, Value, json};

use crate::crash::{CrashHook, CrashPoint, Crashed, Progress};
use crate::entities::{Owner, Project, Run, RunStatus, Task, TaskStatus};
use crate::health::{self, ComponentHealth, ConsistencyReport};

/// Idempotency is always enforced in real use. Only the T00.08 regression
/// (which must show the crash gate fails without it) turns it off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Idempotency {
    Enforced,
    DisabledForRegressionTest,
}

/// The kernel's seams. Production: `SystemIdSource`, `SystemClock`,
/// `NoCrash`, `Idempotency::Enforced`.
pub struct KernelConfig {
    pub ids: Box<dyn IdSource>,
    pub clock: Box<dyn Clock>,
    pub crash: Box<dyn CrashHook>,
    pub idempotency: Idempotency,
}

/// Why the kernel could not open.
#[derive(Debug, thiserror::Error)]
pub enum KernelError {
    #[error(transparent)]
    Storage(#[from] StorageError),
}

/// The canonical database inside a Rikuri home directory.
pub fn database_path(home: &Path) -> PathBuf {
    home.join("state").join("rikuri.db")
}

/// The fixed key `system.init` records: initialization happens once.
const INIT_KEY: &str = "rikuri-system-init-v1";
const MAX_TITLE_CHARS: usize = 200;
const MAX_REQUEST_ID_CHARS: usize = 64;

/// The most tasks `state.show` lists; `tasks_truncated` says when there are more.
const MAX_SHOWN_TASKS: u32 = 1000;

pub struct Kernel {
    store: Store,
    ids: Box<dyn IdSource>,
    crash: Box<dyn CrashHook>,
    idempotency: Idempotency,
    consistency: Mutex<ComponentHealth>,
    /// Set when current state and ledger history disagree (owner decision
    /// 2026-10-08): reads and health are served, every write is refused
    /// until the owner repairs the store. Sticky for the daemon's lifetime.
    quarantine: Mutex<Option<String>>,
}

/// Internal failure: a typed API error, or a simulated crash.
enum Fail {
    Api(ApiError),
    Crashed(Crashed),
}

/// A storage failure or an internal kernel failure counts against the
/// store's health; a refused request (validation, not found, conflict) or a
/// simulated crash does not.
impl StoreFailure for Fail {
    fn store_failure(&self) -> Option<String> {
        match self {
            Fail::Api(e) if matches!(e.code, ErrorCode::StorageFailure | ErrorCode::Internal) => {
                Some(bounded(&e.message))
            }
            _ => None,
        }
    }
}

impl From<ApiError> for Fail {
    fn from(e: ApiError) -> Fail {
        Fail::Api(e)
    }
}

impl From<Crashed> for Fail {
    fn from(c: Crashed) -> Fail {
        Fail::Crashed(c)
    }
}

impl From<StorageError> for Fail {
    fn from(e: StorageError) -> Fail {
        Fail::Api(storage_error(e))
    }
}

impl From<LedgerError> for Fail {
    fn from(e: LedgerError) -> Fail {
        match e {
            LedgerError::Storage(s) => Fail::Api(storage_error(s)),
            other => Fail::Api(ApiError::new(
                ErrorCode::Internal,
                format!("ledger refused the kernel's event: {other}"),
            )),
        }
    }
}

fn storage_error(e: StorageError) -> ApiError {
    match e {
        StorageError::VersionConflict {
            kind,
            id,
            expected,
            actual,
        } => ApiError::new(
            ErrorCode::VersionConflict,
            format!("{kind} {id} is at version {actual}, not {expected}"),
        )
        .with_details(json!({"id": id, "expected": expected, "actual": actual})),
        StorageError::NotFound { kind, id } => {
            ApiError::new(ErrorCode::NotFound, format!("{kind} {id} not found"))
                .with_details(json!({"id": id}))
        }
        other => ApiError::new(ErrorCode::StorageFailure, other.to_string()),
    }
}

/// What a consequential command's plan produced: the response result and
/// the events to append. State changes were already made in the
/// transaction.
struct Planned {
    result: Value,
    events: Vec<NewEvent>,
}

struct Applied {
    result: Value,
    replayed: bool,
}

impl Kernel {
    /// Opens (or creates) the canonical store under `home`, then checks that
    /// state agrees with ledger history before serving anything.
    pub fn open(home: &Path, config: KernelConfig) -> Result<Kernel, KernelError> {
        let store = Store::open(
            &database_path(home),
            config.clock,
            &[&rikuri_ledger::MIGRATIONS],
        )?;
        let kernel = Kernel {
            store,
            ids: config.ids,
            crash: config.crash,
            idempotency: config.idempotency,
            consistency: Mutex::new(ComponentHealth::new("ledger_consistency")),
            quarantine: Mutex::new(None),
        };
        kernel.check_consistency();
        Ok(kernel)
    }

    /// The in-process adapter: one request in, one response out.
    pub fn handle(&self, req: &Request) -> Response {
        let rid = req.request_id.clone();
        match self.dispatch(req) {
            Ok(applied) => {
                let mut response = Response::success(rid, applied.result);
                response.replayed = applied.replayed;
                response
            }
            Err(Fail::Api(e)) => Response::failure(rid, e),
            Err(Fail::Crashed(c)) => Response::failure(
                rid,
                ApiError::new(
                    ErrorCode::DaemonUnavailable,
                    format!(
                        "simulated crash at {}; retry with the same idempotency key",
                        c.point
                    ),
                ),
            ),
        }
    }

    fn dispatch(&self, req: &Request) -> Result<Applied, Fail> {
        if req.v != ENVELOPE_VERSION {
            return Err(ApiError::new(
                ErrorCode::UnsupportedEnvelopeVersion,
                format!(
                    "envelope version {} is not supported; this daemon speaks {ENVELOPE_VERSION}",
                    req.v
                ),
            )
            .into());
        }
        if req
            .request_id
            .as_deref()
            .is_some_and(|r| r.chars().count() > MAX_REQUEST_ID_CHARS)
        {
            return Err(invalid(format!(
                "request_id is longer than {MAX_REQUEST_ID_CHARS} characters"
            ))
            .into());
        }
        match req.command.as_str() {
            command::SYSTEM_INIT => {
                let _: NoParams = params(req)?;
                no_client_key(req, "uses its own built-in key")?;
                let key = key(INIT_KEY)?;
                self.consequential(req, &key, |tx, now| self.plan_init(tx, now, &key))
            }
            command::SYSTEM_HEALTH => {
                let _: NoParams = params(req)?;
                no_client_key(req, "changes nothing")?;
                Ok(read_only(self.health()))
            }
            command::TASK_CREATE => {
                let p: TaskCreate = params(req)?;
                let key = client_key(req)?;
                self.consequential(req, &key, |tx, now| self.plan_create(tx, now, &key, &p))
            }
            command::TASK_START => {
                let p: TaskStart = params(req)?;
                let key = client_key(req)?;
                self.consequential(req, &key, |tx, now| self.plan_start(tx, now, &key, &p))
            }
            command::TASK_STEP => {
                let p: TaskStep = params(req)?;
                let key = client_key(req)?;
                self.consequential(req, &key, |tx, now| self.plan_step(tx, now, &key, &p))
            }
            command::TASK_COMPLETE => {
                let p: TaskComplete = params(req)?;
                let key = client_key(req)?;
                self.consequential(req, &key, |tx, now| self.plan_complete(tx, now, &key, &p))
            }
            command::STATE_SHOW => {
                let _: NoParams = params(req)?;
                no_client_key(req, "changes nothing")?;
                Ok(read_only(self.store.read(|tx| self.state(tx))?))
            }
            command::LEDGER_TAIL => {
                let p: LedgerTail = params(req)?;
                no_client_key(req, "changes nothing")?;
                if !(1..=1000).contains(&p.limit) {
                    return Err(invalid("limit must be 1–1000".into()).into());
                }
                let events = self
                    .store
                    .read(|tx| rikuri_ledger::tail(tx, p.limit).map_err(Fail::from))?;
                Ok(read_only(json!({ "events": events })))
            }
            other => Err(ApiError::new(
                ErrorCode::UnknownCommand,
                format!(
                    "unknown command {:?}; known: {}",
                    truncate(other),
                    command::ALL.join(", ")
                ),
            )
            .into()),
        }
    }

    /// The one-transaction path for every consequential command.
    fn consequential(
        &self,
        req: &Request,
        key: &IdempotencyKey,
        plan: impl FnOnce(&Tx<'_>, Timestamp) -> Result<Planned, Fail>,
    ) -> Result<Applied, Fail> {
        let command = req.command.as_str();
        let hash = rikuri_ledger::request_hash(command, &req.params);
        if let Some(reason) = self.quarantined() {
            // A replay only reads the stored response, so a quarantined
            // daemon still answers it (owner decision 2026-10-08); anything
            // that would write is refused.
            if self.idempotency == Idempotency::Enforced
                && let IdempotencyCheck::Replay(record) = self
                    .store
                    .read(|tx| rikuri_ledger::check(tx, key, command, &hash))?
            {
                return Ok(Applied {
                    result: record.response,
                    replayed: true,
                });
            }
            return Err(ApiError::new(
                ErrorCode::Quarantined,
                format!(
                    "writes are refused (quarantine) because the ledger-consistency check \
                     failed: {}; repair the store, then restart rikurid",
                    brief(&reason)
                ),
            )
            .into());
        }
        // What the command has done at each crash point, read from facts
        // when the point is reached, so a point that moves sees different
        // progress (crash::Progress, W1 placement): the store says whether
        // its transaction is open, SQLite's change count whether anything
        // was written, the ledger's newest sequence number whether an event
        // was appended, and `recorded` is the result of writing the
        // idempotency record.
        let at = |point: CrashPoint, progress: Progress| {
            let progress = Progress {
                transaction_open: self.store.in_transaction(),
                ..progress
            };
            self.crash.at(point, command, progress)
        };
        at(CrashPoint::BeforeTransaction, Progress::default())?;
        let (applied, done) = self.store.write(|tx| {
            let newest_event = || -> Result<Option<u64>, Fail> {
                Ok(rikuri_ledger::tail(tx, 1)?.last().map(|e| e.seq))
            };
            let changes_at_start = tx.total_changes();
            let newest_at_start = newest_event()?;
            let facts = |recorded: bool| -> Result<Progress, Fail> {
                Ok(Progress {
                    state_written: tx.total_changes() > changes_at_start,
                    events_appended: newest_event()? > newest_at_start,
                    idempotency_recorded: recorded,
                    ..Progress::default()
                })
            };
            if self.idempotency == Idempotency::Enforced {
                match rikuri_ledger::check(tx, key, command, &hash)? {
                    IdempotencyCheck::Fresh => {}
                    IdempotencyCheck::Replay(record) => {
                        let replay = Applied {
                            result: record.response,
                            replayed: true,
                        };
                        return Ok((replay, facts(false)?));
                    }
                    IdempotencyCheck::Conflict(record) => {
                        return Err(Fail::Api(
                            ApiError::new(
                                ErrorCode::IdempotencyConflict,
                                format!(
                                    "idempotency key was already used for a different {} request",
                                    record.command
                                ),
                            )
                            .with_details(json!({"event_id": record.event_id})),
                        ));
                    }
                }
            }
            let now = self.store.now();
            let planned = plan(tx, now)?;
            at(CrashPoint::AfterStateWrite, facts(false)?)?;
            let mut primary = None;
            for event in &planned.events {
                let stored = rikuri_ledger::append(tx, event, now)?;
                primary.get_or_insert(stored.event_id);
            }
            let recorded = if self.idempotency == Idempotency::Enforced {
                let primary = primary.ok_or_else(|| {
                    Fail::Api(ApiError::new(
                        ErrorCode::Internal,
                        "consequential command produced no event",
                    ))
                })?;
                rikuri_ledger::record(tx, key, command, &hash, primary, &planned.result, now)?;
                true
            } else {
                false
            };
            let applied = Applied {
                result: planned.result,
                replayed: false,
            };
            Ok((applied, facts(recorded)?))
        })?;
        let done = Progress {
            committed: true,
            ..done
        };
        at(CrashPoint::AfterCommit, done)?;
        at(CrashPoint::BeforeResponse, done)?;
        Ok(applied)
    }

    fn event_id(&self) -> Result<EventId, Fail> {
        EventId::generate(self.ids.as_ref()).map_err(id_failure)
    }

    fn new_event(
        &self,
        event_type: &str,
        actor: ActorId,
        key: &IdempotencyKey,
    ) -> Result<NewEvent, Fail> {
        Ok(NewEvent {
            event_id: self.event_id()?,
            event_type: EventType::new(event_type)?,
            status: EventStatus::Succeeded,
            risk: RiskClass::NovelModification,
            actor_id: actor,
            project_id: None,
            task_id: None,
            run_id: None,
            subject_ids: vec![],
            input_refs: vec![],
            result_refs: vec![],
            security_decision_id: None,
            cost_record_id: None,
            idempotency_key: Some(key.clone()),
            caused_by: None,
            metadata: Map::new(),
        })
    }

    fn plan_init(
        &self,
        tx: &Tx<'_>,
        now: Timestamp,
        key: &IdempotencyKey,
    ) -> Result<Planned, Fail> {
        if owner(tx)?.is_some() {
            return Err(ApiError::new(ErrorCode::InvalidState, "already initialized").into());
        }
        let ids = self.ids.as_ref();
        let owner = Owner {
            id: ActorId::generate(ids).map_err(id_failure)?,
            project_id: ProjectId::generate(ids).map_err(id_failure)?,
            initialized_at: now,
        };
        let project = Project {
            id: owner.project_id,
            name: "Inbox".into(),
            trust: ProjectTrust::default(),
            intent: ExecutionIntent::default(),
        };
        SqliteRepository::<Owner>::new().insert(tx, &owner, now)?;
        SqliteRepository::<Project>::new().insert(tx, &project, now)?;
        let mut e = self.new_event("system.initialized", owner.id, key)?;
        e.project_id = Some(project.id);
        e.subject_ids = vec![owner.id.into(), project.id.into()];
        e.result_refs = e.subject_ids.clone();
        Ok(Planned {
            result: json!({"actor_id": owner.id, "project_id": project.id, "event_id": e.event_id}),
            events: vec![e],
        })
    }

    fn plan_create(
        &self,
        tx: &Tx<'_>,
        now: Timestamp,
        key: &IdempotencyKey,
        p: &TaskCreate,
    ) -> Result<Planned, Fail> {
        let owner = owner(tx)?.ok_or_else(not_initialized)?;
        let title = p.title.trim();
        if title.is_empty()
            || title.chars().count() > MAX_TITLE_CHARS
            || title.chars().any(char::is_control)
        {
            return Err(invalid(format!(
                "title must be 1–{MAX_TITLE_CHARS} characters without control characters"
            ))
            .into());
        }
        let ids = self.ids.as_ref();
        let task = Task {
            id: TaskId::generate(ids).map_err(id_failure)?,
            project_id: owner.project_id,
            title: title.to_owned(),
            privacy_class: p.privacy_class.unwrap_or(PrivacyClass::Normal),
            deletion_unit_id: DeletionUnitId::generate(ids).map_err(id_failure)?,
            status: TaskStatus::Created,
            steps_completed: 0,
            runs: 0,
            current_run: None,
            last_run: None,
        };
        let stored = SqliteRepository::<Task>::new().insert(tx, &task, now)?;
        let mut e = self.new_event("task.created", owner.id, key)?;
        e.project_id = Some(owner.project_id);
        e.task_id = Some(task.id);
        e.subject_ids = vec![task.id.into()];
        e.result_refs = vec![task.id.into()];
        Ok(Planned {
            result: json!({"task_id": task.id, "version": stored.version, "event_id": e.event_id}),
            events: vec![e],
        })
    }

    fn plan_start(
        &self,
        tx: &Tx<'_>,
        now: Timestamp,
        key: &IdempotencyKey,
        p: &TaskStart,
    ) -> Result<Planned, Fail> {
        let owner = owner(tx)?.ok_or_else(not_initialized)?;
        let mut task = load_task(tx, p.task_id, p.expected_version)?;
        if task.status == TaskStatus::Running {
            return Err(ApiError::new(
                ErrorCode::InvalidState,
                "the task already has a running run",
            )
            .with_details(json!({"run_id": task.current_run}))
            .into());
        }
        let mut e = self.new_event("run.started", owner.id, key)?;
        let run = Run {
            id: RunId::generate(self.ids.as_ref()).map_err(id_failure)?,
            task_id: task.id,
            attempt: task.runs + 1,
            status: RunStatus::Running,
            steps: 0,
            outcome: None,
            verification: None,
            started_event_id: e.event_id,
            completion_event_id: None,
            verification_event_id: None,
        };
        SqliteRepository::<Run>::new().insert(tx, &run, now)?;
        task.status = TaskStatus::Running;
        task.runs += 1;
        task.current_run = Some(run.id);
        task.last_run = Some(run.id);
        let stored = SqliteRepository::<Task>::new().update(tx, &task, p.expected_version, now)?;
        e.project_id = Some(task.project_id);
        e.task_id = Some(task.id);
        e.run_id = Some(run.id);
        e.subject_ids = vec![task.id.into(), run.id.into()];
        e.result_refs = vec![run.id.into()];
        e.metadata.insert("attempt".into(), json!(run.attempt));
        Ok(Planned {
            result: json!({"task_id": task.id, "run_id": run.id, "version": stored.version, "event_id": e.event_id}),
            events: vec![e],
        })
    }

    fn plan_step(
        &self,
        tx: &Tx<'_>,
        now: Timestamp,
        key: &IdempotencyKey,
        p: &TaskStep,
    ) -> Result<Planned, Fail> {
        let owner = owner(tx)?.ok_or_else(not_initialized)?;
        let mut task = load_task(tx, p.task_id, p.expected_version)?;
        let mut run = running_run(tx, &task, p.run_id)?;
        run.steps += 1;
        task.steps_completed += 1;
        SqliteRepository::<Run>::new().update(
            tx,
            &run,
            current_version::<Run>(tx, run.id)?,
            now,
        )?;
        let stored = SqliteRepository::<Task>::new().update(tx, &task, p.expected_version, now)?;
        let mut e = self.new_event("task.stepped", owner.id, key)?;
        e.project_id = Some(task.project_id);
        e.task_id = Some(task.id);
        e.run_id = Some(run.id);
        e.subject_ids = vec![task.id.into(), run.id.into()];
        e.metadata
            .insert("steps_completed".into(), json!(task.steps_completed));
        Ok(Planned {
            result: json!({
                "task_id": task.id, "run_id": run.id,
                "steps_completed": task.steps_completed, "version": stored.version,
                "event_id": e.event_id
            }),
            events: vec![e],
        })
    }

    fn plan_complete(
        &self,
        tx: &Tx<'_>,
        now: Timestamp,
        key: &IdempotencyKey,
        p: &TaskComplete,
    ) -> Result<Planned, Fail> {
        let owner = owner(tx)?.ok_or_else(not_initialized)?;
        let mut task = load_task(tx, p.task_id, p.expected_version)?;
        let mut run = running_run(tx, &task, p.run_id)?;
        let mut completed = self.new_event("run.completed", owner.id, key)?;
        let mut verified = self.new_event("run.verified", owner.id, key)?;
        run.status = RunStatus::Completed;
        run.outcome = Some(p.outcome);
        run.verification = Some(p.verification);
        run.completion_event_id = Some(completed.event_id);
        run.verification_event_id = Some(verified.event_id);
        task.status = TaskStatus::Completed;
        task.current_run = None;
        SqliteRepository::<Run>::new().update(
            tx,
            &run,
            current_version::<Run>(tx, run.id)?,
            now,
        )?;
        let stored = SqliteRepository::<Task>::new().update(tx, &task, p.expected_version, now)?;
        for e in [&mut completed, &mut verified] {
            e.project_id = Some(task.project_id);
            e.task_id = Some(task.id);
            e.run_id = Some(run.id);
            e.subject_ids = vec![task.id.into(), run.id.into()];
        }
        completed.result_refs = vec![run.id.into()];
        completed
            .metadata
            .insert("outcome".into(), json!(p.outcome));
        verified.caused_by = Some(completed.event_id);
        verified
            .metadata
            .insert("verification".into(), json!(p.verification));
        Ok(Planned {
            result: json!({
                "task_id": task.id, "run_id": run.id,
                "outcome": p.outcome, "verification": p.verification,
                "version": stored.version,
                "event_ids": [completed.event_id, verified.event_id]
            }),
            events: vec![completed, verified],
        })
    }

    fn state(&self, tx: &Tx<'_>) -> Result<Value, Fail> {
        let owner = owner(tx)?;
        let project = match &owner {
            Some(o) => SqliteRepository::<Project>::new().get(tx, o.project_id)?,
            None => None,
        };
        let runs = SqliteRepository::<Run>::new();
        let mut tasks = vec![];
        // one more than shown, to say whether the list is complete
        let mut listed = SqliteRepository::<Task>::new().list(tx, MAX_SHOWN_TASKS + 1)?;
        let tasks_truncated = listed.len() > MAX_SHOWN_TASKS as usize;
        listed.truncate(MAX_SHOWN_TASKS as usize);
        for t in listed {
            let last_run = match t.value.last_run {
                Some(r) => runs.get(tx, r)?.map(|v| v.value),
                None => None,
            };
            tasks.push(json!({
                "task_id": t.value.id, "title": t.value.title,
                "privacy_class": t.value.privacy_class, "status": t.value.status,
                "steps_completed": t.value.steps_completed, "runs": t.value.runs,
                "current_run": t.value.current_run, "version": t.version,
                "last_run": last_run
            }));
        }
        Ok(json!({
            "initialized": owner.is_some(),
            "owner": owner.map(|o| json!({"actor_id": o.id, "project_id": o.project_id})),
            "project": project.map(|p| json!({
                "project_id": p.value.id, "name": p.value.name,
                "trust": p.value.trust, "intent": p.value.intent
            })),
            "tasks": tasks,
            "tasks_truncated": tasks_truncated,
            // through the transaction: `Store` methods called inside one of
            // its own transactions are refused (they would deadlock)
            "schema": {
                "storage": tx.schema_version("storage")?,
                "ledger": tx.schema_version("ledger")?,
            }
        }))
    }

    /// Replays the ledger and compares it with current state. Updates the
    /// `ledger_consistency` health component and returns the report.
    pub fn check_consistency(&self) -> ConsistencyReport {
        let now = self.store.now();
        let report = self
            .store
            .read(health::consistency)
            .unwrap_or_else(|e: StorageError| ConsistencyReport::failed(e.to_string()));
        if let Ok(mut c) = self.consistency.lock() {
            c.record(
                now,
                report.problems.is_empty(),
                report.problems.first().cloned(),
                report.events,
            );
        }
        if let Some(first) = report.problems.first()
            && let Ok(mut q) = self.quarantine.lock()
            && q.is_none()
        {
            *q = Some(first.clone());
        }
        report
    }

    /// Why writes are refused, if the daemon is quarantined. A poisoned lock
    /// counts as quarantined: refusing writes is the safe answer.
    pub fn quarantined(&self) -> Option<String> {
        match self.quarantine.lock() {
            Ok(q) => q.clone(),
            Err(_) => Some("the quarantine lock is poisoned".into()),
        }
    }

    fn health(&self) -> Value {
        self.check_consistency();
        let store = ComponentHealth::from_store("store", &self.store.health());
        let consistency = self
            .consistency
            .lock()
            .map(|c| c.clone())
            .unwrap_or_else(|_| ComponentHealth::new("ledger_consistency"));
        let build = ComponentHealth::build(
            cfg!(feature = "crash-injection"),
            self.crash.is_armed(),
            self.idempotency == Idempotency::Enforced,
            self.store.now(),
        );
        let mut components = vec![store, consistency, build];
        let quarantine = self.quarantined();
        if let Some(reason) = &quarantine {
            // Critical while writes are refused, even if the cause has since
            // cleared: health is never green while the daemon is read-only.
            components.push(ComponentHealth::quarantined(
                &brief(reason),
                self.store.now(),
            ));
        }
        let mut report = health::report(&components);
        if let (Some(reason), Some(obj)) = (quarantine, report.as_object_mut()) {
            obj.insert("quarantined".into(), json!(brief(&reason)));
        }
        report
    }

    /// The store, for tests that inspect state directly.
    pub fn store(&self) -> &Store {
        &self.store
    }
}

fn read_only(result: Value) -> Applied {
    Applied {
        result,
        replayed: false,
    }
}

fn invalid(message: String) -> ApiError {
    ApiError::new(ErrorCode::InvalidRequest, message)
}

fn not_initialized() -> Fail {
    ApiError::new(ErrorCode::NotInitialized, "run `rikuri init` first").into()
}

fn id_failure(e: rikuri_domain::IdError) -> Fail {
    ApiError::new(ErrorCode::Internal, format!("could not issue an ID: {e}")).into()
}

fn truncate(s: &str) -> String {
    s.chars().take(64).collect()
}

fn params<T: DeserializeOwned>(req: &Request) -> Result<T, Fail> {
    if !req.params.is_object() {
        return Err(invalid("params must be a JSON object".into()).into());
    }
    serde_json::from_value(req.params.clone()).map_err(|e| {
        invalid(format!(
            "invalid params for {}: {}",
            truncate(&req.command),
            bounded(&e.to_string())
        ))
        .into()
    })
}

/// Kernel-written text (a consistency problem) shown in full up to 300
/// bytes, escaped like `bounded`, which is sized for echoed client input.
fn brief(s: &str) -> String {
    const SHOWN: usize = 300;
    let mut out = String::new();
    for c in s.escape_debug() {
        if out.len() + c.len_utf8() > SHOWN {
            out.push_str(&format!("… ({} bytes)", s.len()));
            return out;
        }
        out.push(c);
    }
    out
}

/// Refuses a client idempotency key on a command that takes none, rather
/// than silently ignoring it.
fn no_client_key(req: &Request, why: &str) -> Result<(), Fail> {
    match req.idempotency_key {
        Some(_) => Err(invalid(format!(
            "{} takes no idempotency_key: it {why}",
            truncate(&req.command)
        ))
        .into()),
        None => Ok(()),
    }
}

fn key(text: &str) -> Result<IdempotencyKey, Fail> {
    IdempotencyKey::new(text).map_err(|e| invalid(e.to_string()).into())
}

fn client_key(req: &Request) -> Result<IdempotencyKey, Fail> {
    match req.idempotency_key.as_deref() {
        Some(k) => key(k),
        None => Err(invalid(format!(
            "{} changes state and needs an idempotency_key; reuse the same key to retry safely",
            truncate(&req.command)
        ))
        .into()),
    }
}

fn owner(tx: &Tx<'_>) -> Result<Option<Owner>, Fail> {
    Ok(SqliteRepository::<Owner>::new()
        .list(tx, 1)?
        .into_iter()
        .next()
        .map(|v| v.value))
}

fn current_version<T: rikuri_storage::Entity>(tx: &Tx<'_>, id: T::Id) -> Result<u64, Fail> {
    Ok(SqliteRepository::<T>::new()
        .get(tx, id)?
        .ok_or_else(|| {
            Fail::Api(ApiError::new(
                ErrorCode::NotFound,
                format!("{} {id} not found", T::KIND),
            ))
        })?
        .version)
}

/// Loads a task the caller expects at `expected` version.
fn load_task(tx: &Tx<'_>, id: TaskId, expected: u64) -> Result<Task, Fail> {
    let stored = SqliteRepository::<Task>::new()
        .get(tx, id)?
        .ok_or_else(|| {
            ApiError::new(ErrorCode::NotFound, format!("task {id} not found"))
                .with_details(json!({"id": id}))
        })?;
    if stored.version != expected {
        return Err(storage_error(StorageError::VersionConflict {
            kind: "task",
            id: id.to_string(),
            expected,
            actual: stored.version,
        })
        .into());
    }
    Ok(stored.value)
}

/// The task's current running run, which must be `run_id`.
fn running_run(tx: &Tx<'_>, task: &Task, run_id: RunId) -> Result<Run, Fail> {
    let run = SqliteRepository::<Run>::new()
        .get(tx, run_id)?
        .ok_or_else(|| {
            ApiError::new(ErrorCode::NotFound, format!("run {run_id} not found"))
                .with_details(json!({"id": run_id}))
        })?
        .value;
    if run.task_id != task.id {
        return Err(ApiError::new(
            ErrorCode::InvalidState,
            format!("run {run_id} belongs to another task"),
        )
        .with_details(json!({"run_id": run_id, "task_id": run.task_id}))
        .into());
    }
    if task.current_run != Some(run_id) || run.status != RunStatus::Running {
        return Err(ApiError::new(
            ErrorCode::InvalidState,
            format!("run {run_id} is not the task's running run"),
        )
        .with_details(json!({"current_run": task.current_run}))
        .into());
    }
    Ok(run)
}
