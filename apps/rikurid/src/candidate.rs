//! Transactional State Evolution's "validate candidate" step (Architecture
//! §43; E1, ADR 0005 and its 2026-10-10 clarification): before a command
//! commits, check what it wrote and appended, read from the transaction.
//! Two layers: general rules (this file: every object written is named by
//! an event and every event is about an object written, counts move by
//! their events, task ↔ run invariants hold, each written task's counts
//! match the ledger through its index) and each command's exact effect
//! (`spec.rs`: its events, its objects, and every field of them, with only
//! the command's own changes). The cost is per object and event written,
//! plus an index-only count of each written task's events, which grows
//! with that task's history (E1 record: roughly 0.1 to 0.2 ms per 1,000
//! events).
//! Drift in objects the command did not write is the full replay's to
//! find, at open and on `system.health`.

use std::collections::HashMap;

use rikuri_domain::{RunId, TaskId};
use rikuri_ledger::Event;
use rikuri_storage::{Repository, SqliteRepository, StorageError, Tx, Written};

use crate::entities::{Run, RunStatus, Task, TaskStatus};
use crate::health::{TaskHistory, check_task_history};

mod spec;

/// The candidate check the kernel runs before committing task command
/// `command` (idempotency key `key`), which began when the ledger's newest
/// sequence number was `since`: the general rules and the command's spec.
/// Empty when the candidate agrees with both.
pub(crate) fn check_command(
    tx: &Tx<'_>,
    command: &str,
    key: &str,
    since: Option<u64>,
) -> Result<Vec<String>, StorageError> {
    let mut problems = check_writes(tx, since)?;
    spec::check(tx, command, key, since, &mut problems)?;
    Ok(problems)
}

/// The candidate check for `system.init` (idempotency key `key`), on top
/// of the full replay the kernel runs for it: exactly one owner and one
/// Inbox project with default trust and intent, and their
/// `system.initialized` event.
pub(crate) fn check_init(
    tx: &Tx<'_>,
    key: &str,
    since: Option<u64>,
) -> Result<Vec<String>, StorageError> {
    let mut problems = vec![];
    spec::check_init(tx, key, since, &mut problems)?;
    Ok(problems)
}

/// The answer command `command` returns (and, with idempotency enforced,
/// records for every retry) names exactly what it wrote and appended after
/// ledger sequence number `since`: its task, run, version and events, and
/// what it reports (verifier round 6: R1, R3).
pub(crate) fn check_answer(
    tx: &Tx<'_>,
    command: &str,
    answer: &serde_json::Value,
    since: Option<u64>,
) -> Result<Vec<String>, StorageError> {
    let mut problems = vec![];
    spec::check_answer(tx, command, answer, since, &mut problems)?;
    Ok(problems)
}

/// The idempotency records written after `mark` (the newest when the
/// command began), read from the ledger rather than trusted: with
/// `expected` (idempotency enforced: the command's key and the response it
/// returns) exactly one, under that key, storing that response; without it
/// (idempotency off) none.
pub(crate) fn check_record_written(
    tx: &Tx<'_>,
    expected: Option<(&rikuri_ledger::IdempotencyKey, &serde_json::Value)>,
    mark: Option<i64>,
) -> Result<Vec<String>, StorageError> {
    let written =
        rikuri_ledger::records_after(tx, mark).map_err(|e| ledger_failure(e, "idempotency"))?;
    let Some((key, response)) = expected else {
        return Ok(match written.len() {
            0 => vec![],
            n => vec![format!(
                "this command wrote {n} idempotency records with idempotency off"
            )],
        });
    };
    let [record] = written.as_slice() else {
        return Ok(vec![format!(
            "this command wrote {} idempotency records; with idempotency enforced it writes exactly its own",
            written.len()
        )]);
    };
    let mut problems = vec![];
    if record.key != key.as_str() {
        problems.push(format!(
            "this command's idempotency record is under key {}, not {}",
            record.key,
            key.as_str()
        ));
    }
    if &record.response != response {
        problems.push(format!(
            "the idempotency record for key {} stores a response other than the one this command returns",
            record.key
        ));
    }
    Ok(problems)
}

/// With idempotency enforced: the record command `command` wrote under
/// `key` is its own (that command and request) and points at the first
/// event the command appended after ledger sequence number `since`
/// (verifier round 4: without the record a retry runs again, N6; under
/// another command or request a retry is refused, N9).
pub(crate) fn check_recorded(
    tx: &Tx<'_>,
    command: &str,
    key: &rikuri_ledger::IdempotencyKey,
    request_hash: &str,
    since: Option<u64>,
) -> Result<Vec<String>, StorageError> {
    let k = key.as_str();
    let first = rikuri_ledger::after(tx, since)
        .map_err(|e| ledger_failure(e, "events"))?
        .into_iter()
        .next();
    let found =
        rikuri_ledger::check(tx, key, command, request_hash).map_err(|e| ledger_failure(e, k))?;
    Ok(match found {
        rikuri_ledger::IdempotencyCheck::Fresh => {
            vec![format!("no idempotency record for key {k}")]
        }
        rikuri_ledger::IdempotencyCheck::Conflict(_) => vec![format!(
            "the idempotency record for key {k} names another command or request than {command}"
        )],
        rikuri_ledger::IdempotencyCheck::Replay(record) => match first {
            Some(e) if e.event_id == record.event_id => vec![],
            Some(e) => vec![format!(
                "the idempotency record for key {k} points at event {}, not this command's first event {}",
                record.event_id, e.event_id
            )],
            None => vec![format!(
                "the idempotency record for key {k} points at event {}, but this command appended no event",
                record.event_id
            )],
        },
    })
}

/// The problems with what this transaction wrote and appended after ledger
/// sequence number `since` (the ledger's newest when the command began);
/// empty when the candidate agrees with history. Both lists come from the
/// transaction itself (the repository's write list and the ledger), not
/// from what the command says it did.
pub(crate) fn check_writes(tx: &Tx<'_>, since: Option<u64>) -> Result<Vec<String>, StorageError> {
    let written = tx.written();
    let appended = rikuri_ledger::after(tx, since).map_err(|e| ledger_failure(e, "events"))?;
    let appended = appended.as_slice();
    let mut problems = vec![];
    let (tasks, runs) = decode_writes(&written, &mut problems);
    let lookup = Lookup {
        tx,
        tasks: &tasks,
        runs: &runs,
    };
    for (&id, (before, after)) in &tasks {
        check_task(
            tx,
            &lookup,
            id,
            before.as_ref(),
            after,
            appended,
            &mut problems,
        )?;
    }
    for (&id, (before, after)) in &runs {
        check_run_write(&lookup, id, before.as_ref(), after, appended, &mut problems)?;
    }
    check_events(&tasks, &runs, appended, &mut problems);
    Ok(problems)
}

/// Each written task and run, first before and last after (an object may
/// be written twice in one command).
type Changes<I, T> = HashMap<I, (Option<T>, T)>;

fn decode_writes(
    written: &[Written],
    problems: &mut Vec<String>,
) -> (Changes<TaskId, Task>, Changes<RunId, Run>) {
    fn decode<T: serde::de::DeserializeOwned>(w: &Written) -> Option<(Option<T>, T)> {
        let before = match w.before.as_deref().map(serde_json::from_str).transpose() {
            Ok(b) => b,
            Err(_) => return None,
        };
        let after = serde_json::from_str(&w.after).ok()?;
        Some((before, after))
    }
    fn merge<I: std::hash::Hash + Eq, T>(
        m: &mut Changes<I, T>,
        id: I,
        (before, after): (Option<T>, T),
    ) {
        match m.get_mut(&id) {
            Some(e) => e.1 = after,
            None => {
                m.insert(id, (before, after));
            }
        }
    }
    let (mut tasks, mut runs) = (HashMap::new(), HashMap::new());
    for w in written {
        match w.kind {
            "task" => match decode::<Task>(w) {
                Some(c) => merge(&mut tasks, c.1.id, c),
                None => problems.push(format!(
                    "task {} written by this command does not decode",
                    w.id
                )),
            },
            "run" => match decode::<Run>(w) {
                Some(c) => merge(&mut runs, c.1.id, c),
                None => problems.push(format!(
                    "run {} written by this command does not decode",
                    w.id
                )),
            },
            other => problems.push(format!(
                "this command wrote a {other} ({}), which no candidate rule allows here",
                w.id
            )),
        }
    }
    (tasks, runs)
}

/// After-state of a task or run: from this command's writes, else the store.
struct Lookup<'a, 't> {
    tx: &'a Tx<'t>,
    tasks: &'a Changes<TaskId, Task>,
    runs: &'a Changes<RunId, Run>,
}

impl Lookup<'_, '_> {
    fn run(&self, id: RunId) -> Result<Option<Run>, StorageError> {
        match self.runs.get(&id) {
            Some((_, after)) => Ok(Some(after.clone())),
            None => Ok(SqliteRepository::<Run>::new()
                .get(self.tx, id)?
                .map(|v| v.value)),
        }
    }
    fn task(&self, id: TaskId) -> Result<Option<Task>, StorageError> {
        match self.tasks.get(&id) {
            Some((_, after)) => Ok(Some(after.clone())),
            None => Ok(SqliteRepository::<Task>::new()
                .get(self.tx, id)?
                .map(|v| v.value)),
        }
    }
}

/// A ledger read that fails: a storage failure stays one; anything else
/// means a stored event is corrupt.
fn ledger_failure(e: rikuri_ledger::LedgerError, id: &str) -> StorageError {
    match e {
        rikuri_ledger::LedgerError::Storage(s) => s,
        other => StorageError::Corrupt {
            kind: "event",
            id: id.to_owned(),
            reason: other.to_string(),
        },
    }
}

fn count(appended: &[Event], kind: &str, matches: impl Fn(&Event) -> bool) -> usize {
    appended
        .iter()
        .filter(|e| e.event_type == kind && matches(e))
        .count()
}

/// `after - before == events`, without overflow.
fn moved_by(before: u32, after: u32, events: usize) -> bool {
    i64::try_from(events).is_ok_and(|events| i64::from(after) - i64::from(before) == events)
}

fn check_task(
    tx: &Tx<'_>,
    lookup: &Lookup<'_, '_>,
    id: TaskId,
    before: Option<&Task>,
    after: &Task,
    appended: &[Event],
    problems: &mut Vec<String>,
) -> Result<(), StorageError> {
    let created = count(appended, "task.created", |e| e.task_id == Some(id));
    match before {
        None if created != 1 => problems.push(format!(
            "task {id} is new but this command appended {created} task.created events for it"
        )),
        None if after.steps_completed != 0
            || after.runs != 0
            || after.current_run.is_some()
            || after.last_run.is_some()
            || after.status != TaskStatus::Created =>
        {
            problems.push(format!("new task {id} does not start empty"));
        }
        Some(_) if created != 0 => problems.push(format!(
            "task {id} already existed but this command appended task.created for it"
        )),
        _ => {}
    }
    // an existing task is written only by a command that says so
    if before.is_some() && !appended.iter().any(|e| e.task_id == Some(id)) {
        problems.push(format!(
            "task {id} was written but no event of this command names it"
        ));
    }
    // a task leaves its current run only by ending it in the same command
    if let Some(r) = before.and_then(|b| b.current_run)
        && after.current_run != Some(r)
        && lookup.runs.get(&r).map(|(_, run)| run.status) != Some(RunStatus::Completed)
    {
        problems.push(format!(
            "task {id} left its current run {r} without ending it in this command"
        ));
    }
    let (bs, br) = before.map_or((0, 0), |b| (b.steps_completed, b.runs));
    let stepped = count(appended, "task.stepped", |e| e.task_id == Some(id));
    if !moved_by(bs, after.steps_completed, stepped) {
        problems.push(format!(
            "task {id} steps went from {bs} to {} but this command appended {stepped} task.stepped events",
            after.steps_completed
        ));
    }
    let started = count(appended, "run.started", |e| e.task_id == Some(id));
    if !moved_by(br, after.runs, started) {
        problems.push(format!(
            "task {id} runs went from {br} to {} but this command appended {started} run.started events",
            after.runs
        ));
    }
    match (after.status, after.current_run) {
        (TaskStatus::Running, None) => {
            problems.push(format!("task {id} is running but has no current run"));
        }
        (TaskStatus::Created | TaskStatus::Completed, Some(r)) => problems.push(format!(
            "task {id} is {} but has current run {r}",
            after.status
        )),
        _ => {}
    }
    if let Some(r) = after.current_run {
        match lookup.run(r)? {
            None => problems.push(format!("task {id} current run {r} does not exist")),
            Some(run) if run.task_id != id || run.status != RunStatus::Running => problems.push(
                format!("task {id} current run {r} is not a running run of that task"),
            ),
            Some(_) => {}
        }
    }
    match after.last_run {
        None if after.runs > 0 => problems.push(format!(
            "task {id} records {} runs but no last run",
            after.runs
        )),
        Some(r) => match lookup.run(r)? {
            None => problems.push(format!("task {id} last run {r} does not exist")),
            Some(run) if run.task_id != id => {
                problems.push(format!("task {id} last run {r} belongs to another task"));
            }
            Some(_) => {}
        },
        None => {}
    }
    // the whole task against the ledger, counted through its index (this
    // command's events included: they are appended in this transaction)
    let counts =
        rikuri_ledger::task_counts(tx, id).map_err(|e| ledger_failure(e, &id.to_string()))?;
    let n = |k: &str| u32::try_from(counts.get(k).copied().unwrap_or(0)).unwrap_or(u32::MAX);
    let history = TaskHistory {
        created: n("task.created"),
        stepped: n("task.stepped"),
        started: n("run.started"),
    };
    check_task_history(after, Some(&history), problems);
    Ok(())
}

fn check_run_write(
    lookup: &Lookup<'_, '_>,
    id: RunId,
    before: Option<&Run>,
    after: &Run,
    appended: &[Event],
    problems: &mut Vec<String>,
) -> Result<(), StorageError> {
    let of = |kind: &'static str| -> Vec<&Event> {
        appended
            .iter()
            .filter(|e| e.event_type == kind && e.run_id == Some(id))
            .collect()
    };
    let started = of("run.started");
    match before {
        None => match started.as_slice() {
            [e] => {
                if after.started_event_id != e.event_id || e.task_id != Some(after.task_id) {
                    problems.push(format!(
                        "new run {id} does not link its own run.started event"
                    ));
                }
                if after.status != RunStatus::Running
                    || after.steps != 0
                    || after.completion_event_id.is_some()
                    || after.verification_event_id.is_some()
                    || after.outcome.is_some()
                    || after.verification.is_some()
                {
                    problems.push(format!("new run {id} does not start empty and running"));
                }
            }
            _ => problems.push(format!(
                "run {id} is new but this command appended {} run.started events for it",
                started.len()
            )),
        },
        Some(b) => {
            if !started.is_empty() {
                problems.push(format!(
                    "run {id} already existed but this command appended run.started for it"
                ));
            }
            if b.task_id != after.task_id || b.started_event_id != after.started_event_id {
                problems.push(format!("run {id} changed its task or its start"));
            }
            if !appended.iter().any(|e| e.run_id == Some(id)) {
                problems.push(format!(
                    "run {id} was written but no event of this command names it"
                ));
            }
        }
    }
    if after.status == RunStatus::Running
        && (after.completion_event_id.is_some()
            || after.verification_event_id.is_some()
            || after.outcome.is_some()
            || after.verification.is_some())
    {
        problems.push(format!(
            "run {id} is running but has a completion link or an outcome"
        ));
    }
    let bs = before.map_or(0, |b| b.steps);
    let stepped = of("task.stepped").len();
    if !moved_by(bs, after.steps, stepped) {
        problems.push(format!(
            "run {id} steps went from {bs} to {} but this command appended {stepped} task.stepped events for it",
            after.steps
        ));
    }
    let (completed, verified) = (of("run.completed"), of("run.verified"));
    let was_running = before.is_none_or(|b| b.status == RunStatus::Running);
    match (was_running, after.status) {
        (true, RunStatus::Completed) => match (completed.as_slice(), verified.as_slice()) {
            ([c], [v]) => {
                if after.completion_event_id != Some(c.event_id)
                    || after.verification_event_id != Some(v.event_id)
                {
                    problems.push(format!(
                        "run {id} does not link the completion events this command appended"
                    ));
                }
                let recorded = |e: &Event, f: &str| {
                    e.metadata
                        .get(f)
                        .and_then(|v| v.as_str())
                        .map(str::to_owned)
                };
                if recorded(c, "outcome") != after.outcome.map(|o| o.as_str().to_owned())
                    || recorded(v, "verification")
                        != after.verification.map(|o| o.as_str().to_owned())
                {
                    problems.push(format!(
                        "run {id} outcome or verification differs from its completion events"
                    ));
                }
                if v.caused_by != Some(c.event_id) {
                    problems.push(format!(
                        "run {id} verification does not follow its completion"
                    ));
                }
            }
            _ => problems.push(format!(
                "run {id} completed with {} run.completed and {} run.verified events",
                completed.len(),
                verified.len()
            )),
        },
        (_, RunStatus::Running) | (false, RunStatus::Completed) => {
            if !completed.is_empty() || !verified.is_empty() {
                problems.push(format!(
                    "run {id} has completion events but did not complete in this command"
                ));
            }
            if !was_running && before != Some(after) {
                problems.push(format!("completed run {id} was changed"));
            }
        }
    }
    if after.status == RunStatus::Running
        && lookup.task(after.task_id)?.and_then(|t| t.current_run) != Some(id)
    {
        problems.push(format!(
            "run {id} is running but task {} does not show it as its current run",
            after.task_id
        ));
    }
    Ok(())
}

/// Every event this command appended is about objects it wrote.
fn check_events(
    tasks: &Changes<TaskId, Task>,
    runs: &Changes<RunId, Run>,
    appended: &[Event],
    problems: &mut Vec<String>,
) {
    let mut effects = std::collections::HashSet::new();
    for e in appended {
        let kind = e.event_type.as_str();
        if let Some(key) = &e.idempotency_key
            && !effects.insert((key.as_str(), kind))
        {
            problems.push(format!(
                "this command appended two {kind} events with idempotency key {key}"
            ));
        }
        if !matches!(
            kind,
            "task.created" | "task.stepped" | "run.started" | "run.completed" | "run.verified"
        ) {
            problems.push(format!(
                "this command appended {kind}, which no candidate rule allows here"
            ));
            continue;
        }
        match e.task_id {
            Some(t) if tasks.contains_key(&t) => {}
            Some(t) => problems.push(format!(
                "this command appended {kind} for task {t} but did not write it"
            )),
            None => problems.push(format!("this command appended {kind} with no task")),
        }
        if kind != "task.created" {
            match e.run_id {
                Some(r) if runs.contains_key(&r) => {}
                Some(r) => problems.push(format!(
                    "this command appended {kind} for run {r} but did not write it"
                )),
                None => problems.push(format!("this command appended {kind} with no run")),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rikuri_api::command::{Outcome, VerificationStatus};
    use rikuri_domain::{
        ActorId, DeletionUnitId, EventId, PrivacyClass, ProjectId, RiskClass, RunId, TaskId,
        Timestamp,
    };
    use rikuri_ledger::{Event, EventStatus, EventType, IdempotencyKey, LedgerError, NewEvent};
    use rikuri_storage::{Repository, SqliteRepository, StorageError, Store, StoreFailure, Tx};
    use rikuri_testkit::{FixtureClock, FixtureIds, T0, T0_TIME};
    use serde_json::{Map, json};

    use super::check_writes;
    use crate::entities::{Run, RunStatus, Task, TaskStatus};

    /// Carries the problems out of a transaction that is then rolled back.
    #[derive(Debug)]
    enum Out {
        Problems(Vec<String>),
        Failed(String),
    }
    impl From<StorageError> for Out {
        fn from(e: StorageError) -> Out {
            Out::Failed(e.to_string())
        }
    }
    impl From<LedgerError> for Out {
        fn from(e: LedgerError) -> Out {
            Out::Failed(e.to_string())
        }
    }
    impl StoreFailure for Out {
        fn store_failure(&self) -> Option<String> {
            match self {
                Out::Failed(why) => Some(why.clone()),
                Out::Problems(_) => None,
            }
        }
    }

    struct World {
        _dir: tempfile::TempDir,
        store: Store,
        ids: FixtureIds,
        actor: ActorId,
        project: ProjectId,
        n: std::cell::Cell<u32>,
    }

    fn world() -> World {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(
            &dir.path().join("state/rikuri.db"),
            Box::new(FixtureClock::fixed(T0_TIME)),
            &[&rikuri_ledger::MIGRATIONS],
        )
        .unwrap();
        let ids = FixtureIds::sequential(T0);
        let actor = ActorId::generate(&ids).unwrap();
        let project = ProjectId::generate(&ids).unwrap();
        World {
            _dir: dir,
            store,
            ids,
            actor,
            project,
            n: std::cell::Cell::new(0),
        }
    }

    fn now() -> Timestamp {
        T0_TIME
    }

    /// The ledger's newest sequence number: where a command begins.
    fn newest(tx: &Tx<'_>) -> Result<Option<u64>, Out> {
        Ok(rikuri_ledger::tail(tx, 1)?.last().map(|e| e.seq))
    }

    impl World {
        fn key(&self) -> IdempotencyKey {
            self.n.set(self.n.get() + 1);
            IdempotencyKey::new(&format!("key-rules-{:06}", self.n.get())).unwrap()
        }

        /// Appends an event of `kind` about `task`/`run`.
        fn event(
            &self,
            tx: &Tx<'_>,
            kind: &str,
            task: Option<TaskId>,
            run: Option<RunId>,
            caused_by: Option<EventId>,
            metadata: serde_json::Value,
        ) -> Result<Event, Out> {
            let e = NewEvent {
                event_id: EventId::generate(&self.ids).unwrap(),
                event_type: EventType::new(kind).unwrap(),
                status: EventStatus::Succeeded,
                risk: RiskClass::NovelModification,
                actor_id: self.actor,
                project_id: Some(self.project),
                task_id: task,
                run_id: run,
                subject_ids: vec![],
                input_refs: vec![],
                result_refs: vec![],
                security_decision_id: None,
                cost_record_id: None,
                idempotency_key: Some(self.key()),
                caused_by,
                metadata: metadata.as_object().cloned().unwrap_or_else(Map::new),
            };
            Ok(rikuri_ledger::append(tx, &e, now())?)
        }

        /// Appends `e` again: a new event ID, the same type, task, run
        /// and idempotency key (the same effect twice).
        fn again(&self, tx: &Tx<'_>, e: &Event) -> Result<Event, Out> {
            let twin = NewEvent {
                event_id: EventId::generate(&self.ids).unwrap(),
                event_type: EventType::new(&e.event_type).unwrap(),
                status: EventStatus::Succeeded,
                risk: RiskClass::NovelModification,
                actor_id: self.actor,
                project_id: e.project_id,
                task_id: e.task_id,
                run_id: e.run_id,
                subject_ids: vec![],
                input_refs: vec![],
                result_refs: vec![],
                security_decision_id: None,
                cost_record_id: None,
                idempotency_key: e
                    .idempotency_key
                    .as_deref()
                    .map(|k| IdempotencyKey::new(k).unwrap()),
                caused_by: None,
                metadata: Map::new(),
            };
            Ok(rikuri_ledger::append(tx, &twin, now())?)
        }

        fn new_task(&self) -> Task {
            Task {
                id: TaskId::generate(&self.ids).unwrap(),
                project_id: self.project,
                title: "t".into(),
                privacy_class: PrivacyClass::Normal,
                deletion_unit_id: DeletionUnitId::generate(&self.ids).unwrap(),
                status: TaskStatus::Created,
                steps_completed: 0,
                runs: 0,
                current_run: None,
                last_run: None,
            }
        }

        /// Runs `command` in a transaction, checks what it wrote, rolls back.
        fn check(&self, command: impl FnOnce(&Tx<'_>) -> Result<Vec<Event>, Out>) -> Vec<String> {
            let got = self.store.write(|tx| -> Result<(), Out> {
                let since = newest(tx)?;
                command(tx)?;
                Err(Out::Problems(check_writes(tx, since)?))
            });
            match got {
                Err(Out::Problems(p)) => p,
                other => panic!("the command itself failed: {other:?}"),
            }
        }

        /// Runs `command` and commits it (to build state for later checks).
        fn commit(&self, command: impl FnOnce(&Tx<'_>) -> Result<Vec<Event>, Out>) {
            self.store
                .write(|tx| -> Result<(), Out> {
                    let since = newest(tx)?;
                    command(tx)?;
                    let problems = check_writes(tx, since)?;
                    assert!(
                        problems.is_empty(),
                        "setup is not a clean command: {problems:?}"
                    );
                    Ok(())
                })
                .unwrap();
        }

        fn task(&self, tx: &Tx<'_>, id: TaskId) -> (Task, u64) {
            let v = SqliteRepository::<Task>::new()
                .get(tx, id)
                .unwrap()
                .unwrap();
            (v.value, v.version)
        }

        fn run(&self, tx: &Tx<'_>, id: RunId) -> (Run, u64) {
            let v = SqliteRepository::<Run>::new().get(tx, id).unwrap().unwrap();
            (v.value, v.version)
        }
    }

    // -- the four commands, done right; tests break one thing each --------

    fn create(w: &World, tx: &Tx<'_>, task: &Task) -> Result<Vec<Event>, Out> {
        SqliteRepository::<Task>::new().insert(tx, task, now())?;
        Ok(vec![w.event(
            tx,
            "task.created",
            Some(task.id),
            None,
            None,
            json!({}),
        )?])
    }

    fn start(w: &World, tx: &Tx<'_>, task: TaskId) -> Result<(Vec<Event>, Run), Out> {
        let (mut t, v) = w.task(tx, task);
        let run_id = RunId::generate(&w.ids).unwrap();
        let e = w.event(tx, "run.started", Some(task), Some(run_id), None, json!({}))?;
        let run = Run {
            id: run_id,
            task_id: task,
            attempt: t.runs + 1,
            status: RunStatus::Running,
            steps: 0,
            outcome: None,
            verification: None,
            started_event_id: e.event_id,
            completion_event_id: None,
            verification_event_id: None,
        };
        SqliteRepository::<Run>::new().insert(tx, &run, now())?;
        t.status = TaskStatus::Running;
        t.runs += 1;
        t.current_run = Some(run_id);
        t.last_run = Some(run_id);
        SqliteRepository::<Task>::new().update(tx, &t, v, now())?;
        Ok((vec![e], run))
    }

    fn step(w: &World, tx: &Tx<'_>, task: TaskId) -> Result<Vec<Event>, Out> {
        let (mut t, tv) = w.task(tx, task);
        let run_id = t.current_run.unwrap();
        let (mut r, rv) = w.run(tx, run_id);
        r.steps += 1;
        t.steps_completed += 1;
        SqliteRepository::<Run>::new().update(tx, &r, rv, now())?;
        SqliteRepository::<Task>::new().update(tx, &t, tv, now())?;
        Ok(vec![w.event(
            tx,
            "task.stepped",
            Some(task),
            Some(run_id),
            None,
            json!({}),
        )?])
    }

    fn complete(w: &World, tx: &Tx<'_>, task: TaskId) -> Result<Vec<Event>, Out> {
        let (mut t, tv) = w.task(tx, task);
        let run_id = t.current_run.unwrap();
        let (mut r, rv) = w.run(tx, run_id);
        let c = w.event(
            tx,
            "run.completed",
            Some(task),
            Some(run_id),
            None,
            json!({"outcome": "succeeded"}),
        )?;
        let v = w.event(
            tx,
            "run.verified",
            Some(task),
            Some(run_id),
            Some(c.event_id),
            json!({"verification": "passed"}),
        )?;
        r.status = RunStatus::Completed;
        r.outcome = Some(Outcome::Succeeded);
        r.verification = Some(VerificationStatus::Passed);
        r.completion_event_id = Some(c.event_id);
        r.verification_event_id = Some(v.event_id);
        t.status = TaskStatus::Completed;
        t.current_run = None;
        SqliteRepository::<Run>::new().update(tx, &r, rv, now())?;
        SqliteRepository::<Task>::new().update(tx, &t, tv, now())?;
        Ok(vec![c, v])
    }

    /// A world with one task, created and started (and its id).
    fn started_world() -> (World, TaskId) {
        let w = world();
        let t = w.new_task();
        w.commit(|tx| create(&w, tx, &t));
        w.commit(|tx| Ok(start(&w, tx, t.id)?.0));
        (w, t.id)
    }

    fn has(problems: &[String], needle: &str) {
        assert!(
            problems.iter().any(|p| p.contains(needle)),
            "no problem mentions {needle:?}: {problems:?}"
        );
    }

    // -- legitimate commands pass ------------------------------------------

    #[test]
    fn each_command_done_right_passes() {
        let w = world();
        let t = w.new_task();
        assert_eq!(w.check(|tx| create(&w, tx, &t)), Vec::<String>::new());
        w.commit(|tx| create(&w, tx, &t));
        assert_eq!(
            w.check(|tx| Ok(start(&w, tx, t.id)?.0)),
            Vec::<String>::new()
        );
        w.commit(|tx| Ok(start(&w, tx, t.id)?.0));
        assert_eq!(w.check(|tx| step(&w, tx, t.id)), Vec::<String>::new());
        w.commit(|tx| step(&w, tx, t.id));
        assert_eq!(w.check(|tx| complete(&w, tx, t.id)), Vec::<String>::new());
        w.commit(|tx| complete(&w, tx, t.id));
        // a second run on the same task
        assert_eq!(
            w.check(|tx| Ok(start(&w, tx, t.id)?.0)),
            Vec::<String>::new()
        );
    }

    // -- tasks ---------------------------------------------------------------

    #[test]
    fn a_new_task_needs_exactly_one_creation_event() {
        let w = world();
        let t = w.new_task();
        let none = w.check(|tx| {
            SqliteRepository::<Task>::new().insert(tx, &t, now())?;
            Ok(vec![])
        });
        has(&none, "is new but this command appended 0 task.created");
        let two = w.check(|tx| {
            let mut e = create(&w, tx, &t)?;
            e.push(w.event(tx, "task.created", Some(t.id), None, None, json!({}))?);
            Ok(e)
        });
        has(&two, "is new but this command appended 2 task.created");
    }

    #[test]
    fn a_new_task_starts_empty() {
        let w = world();
        let other = RunId::generate(&w.ids).unwrap();
        let not_empty: [fn(&mut Task, RunId); 5] = [
            |t, _| t.steps_completed = 1,
            |t, _| t.runs = 1,
            |t, r| t.current_run = Some(r),
            |t, r| t.last_run = Some(r),
            |t, _| t.status = TaskStatus::Completed,
        ];
        for (n, change) in not_empty.into_iter().enumerate() {
            let mut t = w.new_task();
            change(&mut t, other);
            let p = w.check(|tx| create(&w, tx, &t));
            assert!(
                p.iter()
                    .any(|p| p.contains(&format!("new task {} does not start empty", t.id))),
                "case {n}: {p:?}"
            );
        }
    }

    #[test]
    fn an_existing_task_is_not_created_again() {
        let (w, t) = started_world();
        let p = w.check(|tx| {
            let mut e = step(&w, tx, t)?;
            e.push(w.event(tx, "task.created", Some(t), None, None, json!({}))?);
            Ok(e)
        });
        has(&p, "already existed but this command appended task.created");
    }

    #[test]
    fn a_task_step_count_moves_with_its_step_events() {
        let (w, t) = started_world();
        let p = w.check(|tx| {
            let e = step(&w, tx, t)?;
            let (mut task, v) = w.task(tx, t);
            task.steps_completed += 1;
            SqliteRepository::<Task>::new().update(tx, &task, v, now())?;
            Ok(e)
        });
        has(
            &p,
            "steps went from 0 to 2 but this command appended 1 task.stepped",
        );
    }

    #[test]
    fn a_task_run_count_moves_with_its_start_events() {
        let (w, t) = started_world();
        w.commit(|tx| complete(&w, tx, t));
        let p = w.check(|tx| {
            let (e, _) = start(&w, tx, t)?;
            let (mut task, v) = w.task(tx, t);
            task.runs += 1;
            SqliteRepository::<Task>::new().update(tx, &task, v, now())?;
            Ok(e)
        });
        has(
            &p,
            "runs went from 1 to 3 but this command appended 1 run.started",
        );
    }

    #[test]
    fn a_running_task_has_a_current_run_and_a_finished_one_does_not() {
        let (w, t) = started_world();
        let p = w.check(|tx| {
            let e = step(&w, tx, t)?;
            let (mut task, v) = w.task(tx, t);
            task.current_run = None;
            SqliteRepository::<Task>::new().update(tx, &task, v, now())?;
            Ok(e)
        });
        has(&p, "is running but has no current run");
        let p = w.check(|tx| {
            let e = complete(&w, tx, t)?;
            let (mut task, v) = w.task(tx, t);
            let (run, _) = w.run(tx, task.last_run.unwrap());
            task.current_run = Some(run.id);
            SqliteRepository::<Task>::new().update(tx, &task, v, now())?;
            Ok(e)
        });
        has(&p, "is completed but has current run");
    }

    #[test]
    fn a_current_run_exists_is_running_and_is_the_tasks_own() {
        let (w, t) = started_world();
        let other = w.new_task();
        w.commit(|tx| create(&w, tx, &other));
        w.commit(|tx| Ok(start(&w, tx, other.id)?.0));
        let missing = RunId::generate(&w.ids).unwrap();
        for (current, needle) in [
            (Some(missing), "does not exist"),
            (None::<RunId>, "is not a running run of that task"),
        ] {
            let p = w.check(|tx| {
                let e = step(&w, tx, t)?;
                let (mut task, v) = w.task(tx, t);
                task.current_run =
                    Some(current.unwrap_or_else(|| w.task(tx, other.id).0.current_run.unwrap()));
                SqliteRepository::<Task>::new().update(tx, &task, v, now())?;
                Ok(e)
            });
            has(&p, needle);
        }
    }

    #[test]
    fn a_last_run_exists_is_the_tasks_own_and_runs_have_one() {
        let (w, t) = started_world();
        let other = w.new_task();
        w.commit(|tx| create(&w, tx, &other));
        w.commit(|tx| Ok(start(&w, tx, other.id)?.0));
        let missing = RunId::generate(&w.ids).unwrap();
        for (last, needle) in [
            (Some(Some(missing)), "last run"),
            (Some(None), "but no last run"),
            (None, "belongs to another task"),
        ] {
            let p = w.check(|tx| {
                let e = step(&w, tx, t)?;
                let (mut task, v) = w.task(tx, t);
                task.last_run = match last {
                    Some(l) => l,
                    None => w.task(tx, other.id).0.last_run,
                };
                SqliteRepository::<Task>::new().update(tx, &task, v, now())?;
                Ok(e)
            });
            has(&p, needle);
        }
    }

    #[test]
    fn a_written_tasks_counts_match_the_ledger() {
        let (w, t) = started_world();
        // drift committed earlier (a restore, a bug): the task claims 5 steps
        w.store
            .write(|tx| -> Result<(), Out> {
                let (mut task, v) = w.task(tx, t);
                task.steps_completed = 5;
                SqliteRepository::<Task>::new().update(tx, &task, v, now())?;
                Ok(())
            })
            .unwrap();
        has(
            &w.check(|tx| step(&w, tx, t)),
            "records 6 steps but the ledger has 1 task.stepped events",
        );
    }

    // -- runs ----------------------------------------------------------------

    #[test]
    fn a_new_run_needs_exactly_one_start_event() {
        let (w, t) = started_world();
        w.commit(|tx| complete(&w, tx, t));
        let p = w.check(|tx| {
            let (e, run) = start(&w, tx, t)?;
            let extra = Run {
                id: RunId::generate(&w.ids).unwrap(),
                ..run
            };
            SqliteRepository::<Run>::new().insert(tx, &extra, now())?;
            Ok(e)
        });
        has(&p, "is new but this command appended 0 run.started");
    }

    #[test]
    fn a_new_run_links_its_own_start_and_starts_empty() {
        let (w, t) = started_world();
        w.commit(|tx| complete(&w, tx, t));
        let p = w.check(|tx| {
            let (e, mut run) = start(&w, tx, t)?;
            let (_, rv) = w.run(tx, run.id);
            run.started_event_id = EventId::generate(&w.ids).unwrap();
            run.steps = 3;
            SqliteRepository::<Run>::new().update(tx, &run, rv, now())?;
            Ok(e)
        });
        has(&p, "does not link its own run.started");
        has(&p, "does not start empty");
    }

    #[test]
    fn a_new_run_starts_empty_in_every_field() {
        let (w, t) = started_world();
        w.commit(|tx| complete(&w, tx, t));
        let not_empty: [fn(&mut Run, EventId); 6] = [
            |r, _| r.status = RunStatus::Completed,
            |r, _| r.steps = 1,
            |r, e| r.completion_event_id = Some(e),
            |r, e| r.verification_event_id = Some(e),
            |r, _| r.outcome = Some(Outcome::Succeeded),
            |r, _| r.verification = Some(VerificationStatus::Passed),
        ];
        for (n, change) in not_empty.into_iter().enumerate() {
            let p = w.check(|tx| {
                let (e, mut run) = start(&w, tx, t)?;
                let (_, rv) = w.run(tx, run.id);
                change(&mut run, e[0].event_id);
                SqliteRepository::<Run>::new().update(tx, &run, rv, now())?;
                Ok(e)
            });
            assert!(
                p.iter()
                    .any(|p| p.contains("does not start empty and running")),
                "case {n}: {p:?}"
            );
        }
    }

    #[test]
    fn an_existing_run_is_not_started_again_nor_moved() {
        let (w, t) = started_world();
        let p = w.check(|tx| {
            let mut e = step(&w, tx, t)?;
            let run = w.task(tx, t).0.current_run;
            e.push(w.event(tx, "run.started", Some(t), run, None, json!({}))?);
            Ok(e)
        });
        has(&p, "already existed but this command appended run.started");
        let other = w.new_task();
        w.commit(|tx| create(&w, tx, &other));
        let p = w.check(|tx| {
            let e = step(&w, tx, t)?;
            let (mut run, rv) = w.run(tx, w.task(tx, t).0.current_run.unwrap());
            run.task_id = other.id;
            SqliteRepository::<Run>::new().update(tx, &run, rv, now())?;
            Ok(e)
        });
        has(&p, "changed its task or its start");
    }

    #[test]
    fn a_run_step_count_moves_with_its_step_events() {
        let (w, t) = started_world();
        let p = w.check(|tx| {
            let e = step(&w, tx, t)?;
            let (mut run, rv) = w.run(tx, w.task(tx, t).0.current_run.unwrap());
            run.steps += 1;
            SqliteRepository::<Run>::new().update(tx, &run, rv, now())?;
            Ok(e)
        });
        has(
            &p,
            "steps went from 0 to 2 but this command appended 1 task.stepped events for it",
        );
    }

    #[test]
    fn a_completion_appends_and_links_one_of_each_event() {
        let (w, t) = started_world();
        // no verification event: only run.completed is appended
        let p = w.check(|tx| {
            let (mut task, tv) = w.task(tx, t);
            let run_id = task.current_run.unwrap();
            let (mut r, rv) = w.run(tx, run_id);
            let c = w.event(
                tx,
                "run.completed",
                Some(t),
                Some(run_id),
                None,
                json!({"outcome": "succeeded"}),
            )?;
            r.status = RunStatus::Completed;
            r.outcome = Some(Outcome::Succeeded);
            r.verification = Some(VerificationStatus::Passed);
            r.completion_event_id = Some(c.event_id);
            r.verification_event_id = Some(c.event_id);
            task.status = TaskStatus::Completed;
            task.current_run = None;
            SqliteRepository::<Run>::new().update(tx, &r, rv, now())?;
            SqliteRepository::<Task>::new().update(tx, &task, tv, now())?;
            Ok(vec![c])
        });
        has(&p, "completed with 1 run.completed and 0 run.verified");
        // links pointing elsewhere
        let p = w.check(|tx| {
            let e = complete(&w, tx, t)?;
            let (mut run, rv) = w.run(tx, w.task(tx, t).0.last_run.unwrap());
            run.verification_event_id = run.completion_event_id;
            SqliteRepository::<Run>::new().update(tx, &run, rv, now())?;
            Ok(e)
        });
        has(&p, "does not link the completion events");
    }

    #[test]
    fn a_completion_records_what_its_events_say() {
        let (w, t) = started_world();
        for (field, needle) in [
            ("outcome", "outcome or verification differs"),
            ("verification", "outcome or verification differs"),
        ] {
            let p = w.check(|tx| {
                let e = complete(&w, tx, t)?;
                let (mut run, rv) = w.run(tx, w.task(tx, t).0.last_run.unwrap());
                if field == "outcome" {
                    run.outcome = Some(Outcome::Failed);
                } else {
                    run.verification = Some(VerificationStatus::Failed);
                }
                SqliteRepository::<Run>::new().update(tx, &run, rv, now())?;
                Ok(e)
            });
            has(&p, needle);
        }
        // the verification is not caused by the completion
        let p = w.check(|tx| {
            let (mut task, tv) = w.task(tx, t);
            let run_id = task.current_run.unwrap();
            let (mut r, rv) = w.run(tx, run_id);
            let c = w.event(
                tx,
                "run.completed",
                Some(t),
                Some(run_id),
                None,
                json!({"outcome": "succeeded"}),
            )?;
            let v = w.event(
                tx,
                "run.verified",
                Some(t),
                Some(run_id),
                Some(r.started_event_id),
                json!({"verification": "passed"}),
            )?;
            r.status = RunStatus::Completed;
            r.outcome = Some(Outcome::Succeeded);
            r.verification = Some(VerificationStatus::Passed);
            r.completion_event_id = Some(c.event_id);
            r.verification_event_id = Some(v.event_id);
            task.status = TaskStatus::Completed;
            task.current_run = None;
            SqliteRepository::<Run>::new().update(tx, &r, rv, now())?;
            SqliteRepository::<Task>::new().update(tx, &task, tv, now())?;
            Ok(vec![c, v])
        });
        has(&p, "verification does not follow its completion");
    }

    #[test]
    fn completion_events_need_a_completion() {
        let (w, t) = started_world();
        let p = w.check(|tx| {
            let mut e = step(&w, tx, t)?;
            let run = w.task(tx, t).0.current_run;
            e.push(w.event(
                tx,
                "run.completed",
                Some(t),
                run,
                None,
                json!({"outcome": "succeeded"}),
            )?);
            Ok(e)
        });
        has(
            &p,
            "has completion events but did not complete in this command",
        );
    }

    #[test]
    fn a_completed_run_is_not_changed() {
        let (w, t) = started_world();
        w.commit(|tx| complete(&w, tx, t));
        let p = w.check(|tx| {
            let (e, _) = start(&w, tx, t)?;
            let old = w.task(tx, t).0.runs;
            let _ = old;
            // the previous run, already completed, is rewritten
            let first = {
                let evs = rikuri_ledger::by_task(tx, t)?;
                evs.into_iter()
                    .find(|e| e.event_type == "run.completed")
                    .and_then(|e| e.run_id)
                    .unwrap()
            };
            let (mut prev, pv) = w.run(tx, first);
            prev.outcome = Some(Outcome::Cancelled);
            SqliteRepository::<Run>::new().update(tx, &prev, pv, now())?;
            Ok(e)
        });
        has(&p, "completed run");
        has(&p, "was changed");
    }

    #[test]
    fn a_running_run_is_its_tasks_current_run() {
        let (w, t) = started_world();
        let p = w.check(|tx| {
            let e = step(&w, tx, t)?;
            let (mut task, v) = w.task(tx, t);
            task.status = TaskStatus::Completed;
            task.current_run = None;
            SqliteRepository::<Task>::new().update(tx, &task, v, now())?;
            Ok(e)
        });
        has(&p, "is running but task");
    }

    /// D4 (verifier round 2): a second start on a running task moves the
    /// task to a new run and leaves its first run running, unwritten.
    #[test]
    fn a_task_that_leaves_its_current_run_ends_it() {
        let (w, t) = started_world();
        let p = w.check(|tx| Ok(start(&w, tx, t)?.0));
        has(&p, "left its current run");
    }

    /// D5 (verifier round 2): a run that stays running gains a completion
    /// link or an outcome.
    #[test]
    fn a_running_run_has_no_completion_links_or_outcome() {
        let (w, t) = started_world();
        let linked: [fn(&mut Run); 4] = [
            |r| r.completion_event_id = Some(r.started_event_id),
            |r| r.verification_event_id = Some(r.started_event_id),
            |r| r.outcome = Some(Outcome::Failed),
            |r| r.verification = Some(VerificationStatus::Failed),
        ];
        for (n, change) in linked.into_iter().enumerate() {
            let p = w.check(|tx| {
                let e = step(&w, tx, t)?;
                let (mut run, rv) = w.run(tx, w.task(tx, t).0.current_run.unwrap());
                change(&mut run);
                SqliteRepository::<Run>::new().update(tx, &run, rv, now())?;
                Ok(e)
            });
            assert!(
                p.iter()
                    .any(|p| p.contains("is running but has a completion link or an outcome")),
                "case {n}: {p:?}"
            );
        }
    }

    /// D8 (verifier round 2): a step that also rewrites another task's
    /// privacy class and title, which no event of the command names.
    #[test]
    fn an_existing_task_written_is_named_by_an_event() {
        let (w, t) = started_world();
        let other = w.new_task();
        w.commit(|tx| create(&w, tx, &other));
        let p = w.check(|tx| {
            let e = step(&w, tx, t)?;
            let (mut o, ov) = w.task(tx, other.id);
            o.privacy_class = PrivacyClass::Private;
            o.title = "overwritten by a step".into();
            SqliteRepository::<Task>::new().update(tx, &o, ov, now())?;
            Ok(e)
        });
        has(
            &p,
            &format!(
                "task {} was written but no event of this command names it",
                other.id
            ),
        );
    }

    /// The same for a run: another task's running run, rewritten unchanged.
    #[test]
    fn an_existing_run_written_is_named_by_an_event() {
        let (w, t) = started_world();
        let other = w.new_task();
        w.commit(|tx| create(&w, tx, &other));
        w.commit(|tx| Ok(start(&w, tx, other.id)?.0));
        let foreign = w
            .store
            .read(|tx| Ok::<_, StorageError>(w.task(tx, other.id).0.current_run.unwrap()))
            .unwrap();
        let p = w.check(|tx| {
            let e = step(&w, tx, t)?;
            let (r, rv) = w.run(tx, foreign);
            SqliteRepository::<Run>::new().update(tx, &r, rv, now())?;
            Ok(e)
        });
        has(
            &p,
            &format!("run {foreign} was written but no event of this command names it"),
        );
    }

    // -- events and kinds ------------------------------------------------------

    /// D6 (verifier round 2): the step applied twice, with two events that
    /// carry the same idempotency key; every count agrees.
    #[test]
    fn a_command_appends_each_effect_once() {
        let (w, t) = started_world();
        let p = w.check(|tx| {
            let mut e = step(&w, tx, t)?;
            let (mut task, tv) = w.task(tx, t);
            let (mut run, rv) = w.run(tx, task.current_run.unwrap());
            task.steps_completed += 1;
            run.steps += 1;
            SqliteRepository::<Run>::new().update(tx, &run, rv, now())?;
            SqliteRepository::<Task>::new().update(tx, &task, tv, now())?;
            let twin = w.again(tx, &e[0])?;
            e.push(twin);
            Ok(e)
        });
        has(&p, "appended two task.stepped events with idempotency key");
    }

    /// D7 (verifier round 2): an event appended during the command but not
    /// in the list the command returns is still checked, because the check
    /// reads what was appended from the ledger.
    #[test]
    fn events_appended_outside_the_commands_list_are_checked_too() {
        let (w, t) = started_world();
        let other = w.new_task();
        w.commit(|tx| create(&w, tx, &other));
        let p = w.check(|tx| {
            let e = step(&w, tx, t)?;
            w.event(tx, "task.stepped", Some(other.id), None, None, json!({}))?;
            Ok(e)
        });
        has(
            &p,
            &format!(
                "appended task.stepped for task {} but did not write it",
                other.id
            ),
        );
    }

    #[test]
    fn every_event_is_about_something_the_command_wrote() {
        let (w, t) = started_world();
        let other = w.new_task();
        w.commit(|tx| create(&w, tx, &other));
        // an event for a task the command did not write
        let p = w.check(|tx| {
            let mut e = step(&w, tx, t)?;
            e.push(w.event(tx, "task.stepped", Some(other.id), None, None, json!({}))?);
            Ok(e)
        });
        has(&p, "but did not write it");
        // an event with no task
        let p = w.check(|tx| {
            let mut e = step(&w, tx, t)?;
            e.push(w.event(tx, "task.stepped", None, None, None, json!({}))?);
            Ok(e)
        });
        has(&p, "with no task");
        // a run event for a run the command did not write
        w.commit(|tx| Ok(start(&w, tx, other.id)?.0));
        let p = w.check(|tx| {
            let mut e = step(&w, tx, t)?;
            let foreign = w.task(tx, other.id).0.current_run;
            e.push(w.event(tx, "task.stepped", Some(t), foreign, None, json!({}))?);
            Ok(e)
        });
        has(&p, "for run");
        // a step event with no run
        let p = w.check(|tx| {
            let mut e = step(&w, tx, t)?;
            e.push(w.event(tx, "task.stepped", Some(t), None, None, json!({}))?);
            Ok(e)
        });
        has(&p, "with no run");
    }

    #[test]
    fn writing_an_unknown_kind_or_event_is_refused() {
        let (w, t) = started_world();
        let p = w.check(|tx| {
            let mut e = step(&w, tx, t)?;
            e.push(w.event(tx, "system.observed", None, None, None, json!({}))?);
            Ok(e)
        });
        has(
            &p,
            "appended system.observed, which no candidate rule allows here",
        );
        let p = w.check(|tx| {
            let e = step(&w, tx, t)?;
            let owner = crate::entities::Owner {
                id: w.actor,
                project_id: w.project,
                initialized_at: now(),
            };
            SqliteRepository::<crate::entities::Owner>::new().insert(tx, &owner, now())?;
            Ok(e)
        });
        has(&p, "wrote a actor");
    }

    #[test]
    fn a_written_object_that_does_not_decode_is_refused() {
        // a body of another shape stored under the kind "task"
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        struct NotATask {
            id: TaskId,
            other: u8,
        }
        impl rikuri_storage::Entity for NotATask {
            type Id = TaskId;
            const KIND: &'static str = "task";
            const SCHEMA: rikuri_domain::SchemaVersion = rikuri_domain::SchemaVersion::V1;
            fn id(&self) -> TaskId {
                self.id
            }
            fn privacy_class(&self) -> PrivacyClass {
                PrivacyClass::Normal
            }
        }
        let w = world();
        let odd = NotATask {
            id: TaskId::generate(&w.ids).unwrap(),
            other: 1,
        };
        let p = w.check(|tx| {
            SqliteRepository::<NotATask>::new().insert(tx, &odd, now())?;
            Ok(vec![w.event(
                tx,
                "task.created",
                Some(odd.id),
                None,
                None,
                json!({}),
            )?])
        });
        has(&p, "does not decode");
    }

    #[test]
    fn a_ledger_that_cannot_be_read_is_a_storage_failure_not_corruption() {
        let w = world();
        let t = w.new_task();
        let mut seen = None;
        let _ = w.store.write(|tx| -> Result<(), Out> {
            let since = newest(tx)?;
            create(&w, tx, &t)?;
            tx.execute_batch("ALTER TABLE events RENAME TO events_elsewhere")
                .map_err(StorageError::Sqlite)?;
            seen = Some(check_writes(tx, since));
            Err(Out::Problems(vec![]))
        });
        let seen = seen.expect("the check ran");
        assert!(matches!(seen, Err(StorageError::Sqlite(_))), "{seen:?}");
    }

    /// Both ledger reads (`after` and `task_counts`) map their errors
    /// through `ledger_failure`: a storage failure stays one, and only
    /// anything else is reported as a corrupt event (verifier round 3: the
    /// test above stops in `after` and no longer reaches `task_counts`).
    #[test]
    fn a_ledger_failure_stays_a_storage_failure_and_only_the_rest_is_corruption() {
        let storage = super::ledger_failure(
            LedgerError::Storage(StorageError::Sqlite(
                rikuri_storage::rusqlite::Error::QueryReturnedNoRows,
            )),
            "events",
        );
        assert!(matches!(storage, StorageError::Sqlite(_)), "{storage:?}");
        let corrupt = super::ledger_failure(LedgerError::Corrupt("bad row".into()), "event-1");
        assert!(
            matches!(&corrupt, StorageError::Corrupt { kind: "event", id, reason }
                if id == "event-1" && reason.contains("bad row")),
            "{corrupt:?}"
        );
    }

    /// With idempotency enforced, the record a command writes in its
    /// transaction is its own: under its key, command and request, pointing
    /// at its first event (verifier round 4: N6, no record, so a retry runs
    /// again; N9, a record under another command, so a retry is refused).
    #[test]
    fn a_commands_idempotency_record_is_its_own() {
        type Record<'a> = &'a dyn Fn(&Tx<'_>, &IdempotencyKey, &[Event]) -> Result<(), Out>;
        let w = world();
        let hash = rikuri_ledger::request_hash("task.create", &json!({"title": "t"}));
        let other_hash = rikuri_ledger::request_hash("task.create", &json!({"title": "u"}));
        let run = |record: Record<'_>| -> Vec<String> {
            let t = w.new_task();
            let got = w.store.write(|tx| -> Result<(), Out> {
                let since = newest(tx)?;
                let events = create(&w, tx, &t)?;
                let key = IdempotencyKey::new(events[0].idempotency_key.as_deref().unwrap())?;
                record(tx, &key, &events)?;
                Err(Out::Problems(super::check_recorded(
                    tx,
                    "task.create",
                    &key,
                    &hash,
                    since,
                )?))
            });
            match got {
                Err(Out::Problems(p)) => p,
                other => panic!("the command itself failed: {other:?}"),
            }
        };
        let own = run(&|tx, key, e| {
            Ok(rikuri_ledger::record(
                tx,
                key,
                "task.create",
                &hash,
                e[0].event_id,
                &json!({}),
                now(),
            )?)
        });
        assert_eq!(own, Vec::<String>::new());
        has(&run(&|_, _, _| Ok(())), "no idempotency record for key");
        let p = run(&|tx, key, e| {
            Ok(rikuri_ledger::record(
                tx,
                key,
                "task.step",
                &hash,
                e[0].event_id,
                &json!({}),
                now(),
            )?)
        });
        has(&p, "names another command or request than task.create");
        let p = run(&|tx, key, e| {
            Ok(rikuri_ledger::record(
                tx,
                key,
                "task.create",
                &other_hash,
                e[0].event_id,
                &json!({}),
                now(),
            )?)
        });
        has(&p, "names another command or request than task.create");
        let p = run(&|tx, key, e| {
            let extra = w.event(tx, "task.stepped", e[0].task_id, None, None, json!({}))?;
            Ok(rikuri_ledger::record(
                tx,
                key,
                "task.create",
                &hash,
                extra.event_id,
                &json!({}),
                now(),
            )?)
        });
        has(&p, "not this command's first event");
    }

    /// The command wrote exactly its own idempotency record, read from the
    /// ledger rather than trusted: with idempotency enforced exactly one,
    /// under its key, storing the response it returns; with it off, none
    /// (verifier round 5: X4, no record and no flag; X3, an extra record
    /// under another key; X2, a record storing another response).
    #[test]
    fn a_command_writes_exactly_its_own_record() {
        type Write<'a> = &'a dyn Fn(&Tx<'_>, &IdempotencyKey, EventId) -> Result<(), Out>;
        let w = world();
        let response = json!({"task_id": "the task", "version": 1});
        let other_response = json!({"task_id": "another task", "version": 1});
        let record = |tx: &Tx<'_>,
                      key: &IdempotencyKey,
                      event: EventId,
                      stored: &serde_json::Value|
         -> Result<(), Out> {
            Ok(rikuri_ledger::record(
                tx,
                key,
                "task.create",
                "hash",
                event,
                stored,
                now(),
            )?)
        };
        let other_key =
            |key: &IdempotencyKey| IdempotencyKey::new(&format!("{}-x", key.as_str())).unwrap();
        let run = |enforced: bool, write: Write<'_>| -> Vec<String> {
            let t = w.new_task();
            let got = w.store.write(|tx| -> Result<(), Out> {
                let mark = rikuri_ledger::record_mark(tx)?;
                let events = create(&w, tx, &t)?;
                let key = IdempotencyKey::new(events[0].idempotency_key.as_deref().unwrap())?;
                write(tx, &key, events[0].event_id)?;
                let expected = enforced.then_some((&key, &response));
                Err(Out::Problems(super::check_record_written(
                    tx, expected, mark,
                )?))
            });
            match got {
                Err(Out::Problems(p)) => p,
                other => panic!("the command itself failed: {other:?}"),
            }
        };
        // its own record; none with idempotency off
        assert_eq!(
            run(true, &|tx, key, e| record(tx, key, e, &response)),
            Vec::<String>::new()
        );
        assert_eq!(run(false, &|_, _, _| Ok(())), Vec::<String>::new());
        // X4: no record although idempotency is enforced
        has(&run(true, &|_, _, _| Ok(())), "wrote 0 idempotency records");
        // X3: its own record and another under another key
        let p = run(true, &|tx, key, e| {
            record(tx, key, e, &response)?;
            record(tx, &other_key(key), e, &response)
        });
        has(&p, "wrote 2 idempotency records");
        // a record under another key only
        let p = run(true, &|tx, key, e| {
            record(tx, &other_key(key), e, &response)
        });
        has(&p, "is under key");
        // X2: a record storing another response
        let p = run(true, &|tx, key, e| record(tx, key, e, &other_response));
        has(
            &p,
            "stores a response other than the one this command returns",
        );
        // a record although idempotency is off
        let p = run(false, &|tx, key, e| record(tx, key, e, &response));
        has(&p, "with idempotency off");
    }
}
