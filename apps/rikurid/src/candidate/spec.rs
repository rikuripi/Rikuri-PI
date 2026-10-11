//! Each command's exact effect (E1, ADR 0005 clarification 2026-10-10,
//! verifier rounds 3 and 4): the events it appends, the objects it writes,
//! and every field of them. Fields the command does not change must keep
//! their before-state values (the frame rule), so a field that no event
//! describes cannot change unnoticed.

use std::collections::BTreeSet;
use std::fmt::Display;

use rikuri_api::command::{Outcome, VerificationStatus};
use rikuri_domain::{AnyId, EventId, ExecutionIntent, ProjectTrust, RunId, TaskId};
use rikuri_ledger::{EVENT_SCHEMA, Event, EventStatus};
use rikuri_storage::{Entity, Repository, SqliteRepository, StorageError, Tx, deletion_unit_size};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value, json};

use super::{decode_writes, ledger_failure};
use crate::entities::{Owner, Project, Run, RunStatus, Task, TaskStatus};

/// Adds to `problems` where what `system.init` (idempotency key `key`)
/// wrote and appended after ledger sequence number `since` differs from its
/// spec: exactly one new owner and one new project, the Inbox, with default
/// trust and intent (so no project starts trusted for execution; verifier
/// round 4, N8), the owner's project being that one, and its
/// `system.initialized` event by the owner.
pub(super) fn check_init(
    tx: &Tx<'_>,
    key: &str,
    since: Option<u64>,
    problems: &mut Vec<String>,
) -> Result<(), StorageError> {
    let appended = rikuri_ledger::after(tx, since).map_err(|e| ledger_failure(e, "events"))?;
    let kinds: Vec<&str> = appended.iter().map(|e| e.event_type.as_str()).collect();
    if kinds != ["system.initialized"] {
        problems.push(format!(
            "system.init appends [\"system.initialized\"], but this command appended {kinds:?}"
        ));
        return Ok(());
    }
    let written = tx.written();
    let new_of = |kind: &str| -> Vec<&str> {
        written
            .iter()
            .filter(|w| w.kind == kind && w.before.is_none())
            .map(|w| w.after.as_str())
            .collect()
    };
    let (owners, projects) = (
        new_of(<Owner as Entity>::KIND),
        new_of(<Project as Entity>::KIND),
    );
    let ([owner], [project], 2) = (owners.as_slice(), projects.as_slice(), written.len()) else {
        wrote_otherwise(&written, problems);
        return Ok(());
    };
    let (Ok(owner), Ok(project)) = (
        serde_json::from_str::<Owner>(owner),
        serde_json::from_str::<Project>(project),
    ) else {
        problems.push("system.init wrote an owner or a project that does not decode".into());
        return Ok(());
    };
    let e = &appended[0];
    expect(
        problems,
        format!("owner {}", owner.id),
        &Owner {
            project_id: project.id,
            ..owner.clone()
        },
        &owner,
    );
    expect(
        problems,
        format!("project {}", project.id),
        &Project {
            id: project.id,
            name: "Inbox".into(),
            trust: ProjectTrust::default(),
            intent: ExecutionIntent::default(),
        },
        &project,
    );
    let ids: Vec<AnyId> = vec![owner.id.into(), project.id.into()];
    let event = Event {
        seq: e.seq,
        event_id: e.event_id,
        schema_version: EVENT_SCHEMA,
        recorded_at: e.recorded_at,
        event_type: e.event_type.clone(),
        status: EventStatus::Succeeded,
        actor_id: owner.id,
        project_id: Some(project.id),
        task_id: None,
        run_id: None,
        subject_ids: ids.clone(),
        input_refs: vec![],
        result_refs: ids,
        security_decision_id: None,
        cost_record_id: None,
        idempotency_key: Some(key.to_owned()),
        caused_by: None,
        metadata: Map::new(),
    };
    expect(
        problems,
        format!("system.initialized event {}", e.event_id),
        &event,
        e,
    );
    Ok(())
}

/// Adds to `problems` where the answer command `command` returns differs
/// from what it wrote and appended after ledger sequence number `since`:
/// exactly its task, run, version and events, and what it reports
/// (verifier round 6: R1, R3). With idempotency enforced the answer is also
/// what its record stores and every retry returns. A command without the
/// objects or events its answer needs is left to the general rules and the
/// specs, which report it.
pub(super) fn check_answer(
    tx: &Tx<'_>,
    command: &str,
    answer: &Value,
    since: Option<u64>,
    problems: &mut Vec<String>,
) -> Result<(), StorageError> {
    let appended = rikuri_ledger::after(tx, since).map_err(|e| ledger_failure(e, "events"))?;
    let Some(first) = appended.first() else {
        return Ok(());
    };
    let expected = if command == "system.init" {
        let project = tx
            .written()
            .iter()
            .find(|w| w.kind == <Project as Entity>::KIND)
            .and_then(|w| serde_json::from_str::<Project>(&w.after).ok());
        let Some(project) = project else {
            return Ok(());
        };
        json!({"actor_id": first.actor_id, "project_id": project.id, "event_id": first.event_id})
    } else {
        let Some(stored) = first
            .task_id
            .map(|t| SqliteRepository::<Task>::new().get(tx, t))
            .transpose()?
            .flatten()
        else {
            return Ok(());
        };
        let run = first
            .run_id
            .map(|r| SqliteRepository::<Run>::new().get(tx, r))
            .transpose()?
            .flatten()
            .map(|v| v.value);
        let (task, version) = (stored.value, stored.version);
        match (command, run) {
            ("task.create", _) => json!({
                "task_id": task.id, "version": version, "event_id": first.event_id
            }),
            ("task.start", Some(run)) => json!({
                "task_id": task.id, "run_id": run.id, "version": version,
                "event_id": first.event_id
            }),
            ("task.step", Some(run)) => json!({
                "task_id": task.id, "run_id": run.id,
                "steps_completed": task.steps_completed, "version": version,
                "event_id": first.event_id
            }),
            ("task.complete", Some(run)) => json!({
                "task_id": task.id, "run_id": run.id,
                "outcome": run.outcome, "verification": run.verification,
                "version": version,
                "event_ids": appended.iter().map(|e| e.event_id).collect::<Vec<_>>()
            }),
            _ => return Ok(()),
        }
    };
    if &expected != answer {
        problems.push(format!(
            "{command}'s answer differs from what it wrote in: {}",
            differing(&expected, answer)
        ));
    }
    Ok(())
}

fn wrote_otherwise(written: &[rikuri_storage::Written], problems: &mut Vec<String>) {
    let wrote: Vec<String> = written
        .iter()
        .map(|w| format!("{} {}", w.kind, w.id))
        .collect();
    problems.push(format!(
        "system.init writes one owner and one project, both new, but this command wrote [{}]",
        wrote.join(", ")
    ));
}

/// Adds to `problems` where what command `command` (idempotency key
/// `key`) wrote and appended after ledger sequence number `since` differs
/// from that command's spec. Anything the general rules already reject
/// (an object that does not decode, a task command with no task) is left
/// to them.
pub(super) fn check(
    tx: &Tx<'_>,
    command: &str,
    key: &str,
    since: Option<u64>,
    problems: &mut Vec<String>,
) -> Result<(), StorageError> {
    let kinds: &[&str] = match command {
        "task.create" => &["task.created"],
        "task.start" => &["run.started"],
        "task.step" => &["task.stepped"],
        "task.complete" => &["run.completed", "run.verified"],
        other => {
            problems.push(format!("no candidate spec covers command {other}"));
            return Ok(());
        }
    };
    let appended = rikuri_ledger::after(tx, since).map_err(|e| ledger_failure(e, "events"))?;
    let appended_kinds: Vec<&str> = appended.iter().map(|e| e.event_type.as_str()).collect();
    if appended_kinds != kinds {
        problems.push(format!(
            "{command} appends {kinds:?}, but this command appended {appended_kinds:?}"
        ));
        return Ok(());
    }
    let Some(owner) = SqliteRepository::<Owner>::new()
        .list(tx, 1)?
        .into_iter()
        .next()
        .map(|v| v.value)
    else {
        problems.push(format!("{command} ran in a store with no owner"));
        return Ok(());
    };
    let Some(task_id) = appended[0].task_id else {
        return Ok(());
    };
    let run_id = appended[0].run_id;
    let (tasks, runs) = decode_writes(&tx.written(), &mut vec![]);
    let wrote_tasks: BTreeSet<TaskId> = tasks.keys().copied().collect();
    let wrote_runs: BTreeSet<RunId> = runs.keys().copied().collect();
    if wrote_tasks != BTreeSet::from([task_id]) || wrote_runs != run_id.into_iter().collect() {
        problems.push(format!(
            "{command} writes task {task_id}{}, but this command wrote tasks [{}] and runs [{}]",
            run_id.map(|r| format!(" and run {r}")).unwrap_or_default(),
            listed(&wrote_tasks),
            listed(&wrote_runs),
        ));
        return Ok(());
    }
    let Some((before, after)) = tasks.get(&task_id) else {
        return Ok(());
    };
    let run = run_id.and_then(|r| runs.get(&r));
    let mut spec = Spec {
        owner: &owner,
        key,
        problems,
    };
    match (command, before, run) {
        ("task.create", _, _) => spec.create(tx, after, &appended[0])?,
        ("task.start", Some(before), Some((_, run))) => {
            spec.start(before, after, run, &appended[0]);
        }
        ("task.step", Some(before), Some((Some(run_before), run))) => {
            spec.step(before, after, run_before, run, &appended[0]);
        }
        ("task.complete", Some(before), Some((Some(run_before), run))) => {
            spec.complete(before, after, run_before, run, &appended[0], &appended[1]);
        }
        // a new task or run where the command needs an existing one: the
        // general rules report the missing creation or start event
        _ => {}
    }
    Ok(())
}

/// What a command must have done, given who owns the store and the key it
/// ran under.
struct Spec<'a> {
    owner: &'a Owner,
    key: &'a str,
    problems: &'a mut Vec<String>,
}

impl Spec<'_> {
    fn expect<T: Serialize + PartialEq>(&mut self, what: String, expected: &T, actual: &T) {
        expect(self.problems, what, expected, actual);
    }

    /// The run a step or completion names must be the task's own running
    /// current run, as the kernel's request validation requires (verifier
    /// round 4: N1, N2).
    fn names_its_running_current_run(&mut self, command: &str, before: &Task, run_before: &Run) {
        if before.status != TaskStatus::Running
            || before.current_run != Some(run_before.id)
            || run_before.task_id != before.id
            || run_before.status != RunStatus::Running
        {
            self.problems.push(format!(
                "{command} on task {} and run {}, which is not the task's running current run",
                before.id, run_before.id
            ));
        }
    }

    /// `actual` as the command must have appended it: by the owner, under
    /// the command's key, succeeded, about `task` (and `run`), with these
    /// results, cause and metadata, and nothing else.
    fn event(
        &self,
        actual: &Event,
        task: &Task,
        run: Option<RunId>,
        result_refs: Vec<AnyId>,
        caused_by: Option<EventId>,
        metadata: Map<String, Value>,
    ) -> Event {
        let mut subject_ids: Vec<AnyId> = vec![task.id.into()];
        subject_ids.extend(run.map(AnyId::from));
        // Every field is listed (no `..actual`), so a field added to `Event`
        // later fails to compile here until its spec is decided. Only the
        // ledger's own bookkeeping (sequence, ID, time, and the type checked
        // above) is taken from the event as appended; the schema version
        // must be the ledger's current one (verifier round 6, S1).
        Event {
            seq: actual.seq,
            event_id: actual.event_id,
            schema_version: EVENT_SCHEMA,
            recorded_at: actual.recorded_at,
            event_type: actual.event_type.clone(),
            status: EventStatus::Succeeded,
            actor_id: self.owner.id,
            project_id: Some(task.project_id),
            task_id: Some(task.id),
            run_id: run,
            subject_ids,
            input_refs: vec![],
            result_refs,
            security_decision_id: None,
            cost_record_id: None,
            idempotency_key: Some(self.key.to_owned()),
            caused_by,
            metadata,
        }
    }

    /// A new, empty task in the owner's project, with a deletion unit of
    /// its own (verifier round 4, N5: forgetting one task must not forget
    /// another's content); its title and privacy class are the request's.
    fn create(&mut self, tx: &Tx<'_>, after: &Task, e: &Event) -> Result<(), StorageError> {
        // the count includes the new task itself
        let shared = deletion_unit_size(tx, after.deletion_unit_id)?;
        if shared != 1 {
            self.problems.push(format!(
                "task {}'s deletion unit {} is not its own: {shared} objects belong to it",
                after.id, after.deletion_unit_id
            ));
        }
        let task = Task {
            id: after.id,
            project_id: self.owner.project_id,
            title: after.title.clone(),
            privacy_class: after.privacy_class,
            deletion_unit_id: after.deletion_unit_id,
            status: TaskStatus::Created,
            steps_completed: 0,
            runs: 0,
            current_run: None,
            last_run: None,
        };
        self.expect(format!("task {}", task.id), &task, after);
        let event = self.event(e, &task, None, vec![task.id.into()], None, Map::new());
        self.expect(format!("task.created event {}", e.event_id), &event, e);
        Ok(())
    }

    /// The task, not already running, moves to a new run: its next
    /// attempt, running and empty, linked to its `run.started`.
    fn start(&mut self, before: &Task, after: &Task, run: &Run, e: &Event) {
        if before.status == TaskStatus::Running {
            self.problems.push(format!(
                "task.start on task {}, which is already running",
                before.id
            ));
        }
        let attempt = before.runs.saturating_add(1);
        let task = Task {
            status: TaskStatus::Running,
            runs: attempt,
            current_run: Some(run.id),
            last_run: Some(run.id),
            ..before.clone()
        };
        let new_run = Run {
            id: run.id,
            task_id: before.id,
            attempt,
            status: RunStatus::Running,
            steps: 0,
            outcome: None,
            verification: None,
            started_event_id: e.event_id,
            completion_event_id: None,
            verification_event_id: None,
        };
        self.expect(format!("task {}", before.id), &task, after);
        self.expect(format!("run {}", run.id), &new_run, run);
        let mut metadata = Map::new();
        metadata.insert("attempt".into(), json!(attempt));
        let event = self.event(e, &task, Some(run.id), vec![run.id.into()], None, metadata);
        self.expect(format!("run.started event {}", e.event_id), &event, e);
    }

    /// One more step on the task and on its run; nothing else changes.
    fn step(&mut self, before: &Task, after: &Task, run_before: &Run, run: &Run, e: &Event) {
        self.names_its_running_current_run("task.step", before, run_before);
        let task = Task {
            steps_completed: before.steps_completed.saturating_add(1),
            ..before.clone()
        };
        let stepped = Run {
            steps: run_before.steps.saturating_add(1),
            ..run_before.clone()
        };
        self.expect(format!("task {}", before.id), &task, after);
        self.expect(format!("run {}", run.id), &stepped, run);
        let mut metadata = Map::new();
        metadata.insert("steps_completed".into(), json!(task.steps_completed));
        let event = self.event(e, &task, Some(run.id), vec![], None, metadata);
        self.expect(format!("task.stepped event {}", e.event_id), &event, e);
    }

    /// The run completes with what its two events record, linked to them;
    /// the task completes and leaves the run. Nothing else changes.
    fn complete(
        &mut self,
        before: &Task,
        after: &Task,
        run_before: &Run,
        run: &Run,
        c: &Event,
        v: &Event,
    ) {
        self.names_its_running_current_run("task.complete", before, run_before);
        let outcome: Option<Outcome> = recorded(c, "outcome");
        let verification: Option<VerificationStatus> = recorded(v, "verification");
        let (Some(outcome), Some(verification)) = (outcome, verification) else {
            self.problems.push(format!(
                "task.complete's events record no valid outcome or verification \
                 (run.completed event {} records no valid outcome, or run.verified event {} \
                 records no valid verification)",
                c.event_id, v.event_id
            ));
            return;
        };
        let task = Task {
            status: TaskStatus::Completed,
            current_run: None,
            ..before.clone()
        };
        let completed = Run {
            status: RunStatus::Completed,
            outcome: Some(outcome),
            verification: Some(verification),
            completion_event_id: Some(c.event_id),
            verification_event_id: Some(v.event_id),
            ..run_before.clone()
        };
        self.expect(format!("task {}", before.id), &task, after);
        self.expect(format!("run {}", run.id), &completed, run);
        let mut metadata = Map::new();
        metadata.insert("outcome".into(), json!(outcome));
        let event = self.event(c, &task, Some(run.id), vec![run.id.into()], None, metadata);
        self.expect(format!("run.completed event {}", c.event_id), &event, c);
        let mut metadata = Map::new();
        metadata.insert("verification".into(), json!(verification));
        let event = self.event(v, &task, Some(run.id), vec![], Some(c.event_id), metadata);
        self.expect(format!("run.verified event {}", v.event_id), &event, v);
    }
}

/// `actual` must be exactly `expected`; otherwise names the fields that
/// differ.
fn expect<T: Serialize + PartialEq>(
    problems: &mut Vec<String>,
    what: String,
    expected: &T,
    actual: &T,
) {
    if expected != actual {
        problems.push(format!(
            "{what} differs from its spec in: {}",
            differing(expected, actual)
        ));
    }
}

/// The value an event's metadata records under `field`, if it is one.
fn recorded<T: DeserializeOwned>(e: &Event, field: &str) -> Option<T> {
    e.metadata
        .get(field)
        .and_then(|v| serde_json::from_value(v.clone()).ok())
}

/// The top-level fields in which `actual` differs from `expected`.
fn differing<T: Serialize>(expected: &T, actual: &T) -> String {
    let (Ok(Value::Object(e)), Ok(Value::Object(a))) =
        (serde_json::to_value(expected), serde_json::to_value(actual))
    else {
        return "(not comparable)".into();
    };
    let mut fields: Vec<&str> = e
        .keys()
        .chain(a.keys())
        .map(String::as_str)
        .filter(|k| e.get(*k) != a.get(*k))
        .collect();
    fields.sort_unstable();
    fields.dedup();
    fields.join(", ")
}

fn listed<T: Display>(ids: &BTreeSet<T>) -> String {
    ids.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use rikuri_api::command::{Outcome, VerificationStatus};
    use rikuri_domain::{
        ActorId, CostRecordId, DeletionUnitId, EventId, ExecutionIntent, PrivacyClass, ProjectId,
        ProjectTrust, RiskClass, RunId, SecurityDecisionId, TaskId,
    };
    use rikuri_ledger::{EventStatus, EventType, IdempotencyKey, LedgerError, NewEvent};
    use rikuri_storage::{Repository, SqliteRepository, StorageError, Store, StoreFailure, Tx};
    use rikuri_testkit::{FixtureClock, FixtureIds, T0, T0_TIME};
    use serde_json::{Map, json};

    use crate::candidate::{check_command, check_init};
    use crate::entities::{Owner, Project, Run, RunStatus, Task, TaskStatus};

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

    struct F {
        _dir: tempfile::TempDir,
        store: Store,
        ids: FixtureIds,
        owner: Owner,
        n: Cell<u32>,
    }

    /// An initialized store: one owner.
    fn fixture() -> F {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(
            &dir.path().join("state/rikuri.db"),
            Box::new(FixtureClock::fixed(T0_TIME)),
            &[&rikuri_ledger::MIGRATIONS],
        )
        .unwrap();
        let ids = FixtureIds::sequential(T0);
        let owner = Owner {
            id: ActorId::generate(&ids).unwrap(),
            project_id: ProjectId::generate(&ids).unwrap(),
            initialized_at: T0_TIME,
        };
        store
            .write(|tx| {
                SqliteRepository::<Owner>::new()
                    .insert(tx, &owner, T0_TIME)
                    .map(|_| ())
            })
            .unwrap();
        F {
            _dir: dir,
            store,
            ids,
            owner,
            n: Cell::new(0),
        }
    }

    impl F {
        fn key(&self) -> String {
            self.n.set(self.n.get() + 1);
            format!("key-spec-{:06}", self.n.get())
        }

        /// Runs `command` (one of the helpers below, perhaps changed to
        /// plant a defect) under a fresh key, checks it as command `name`,
        /// and rolls back.
        fn check(
            &self,
            name: &str,
            command: impl FnOnce(&Tx<'_>, &str) -> Result<(), Out>,
        ) -> Vec<String> {
            let key = self.key();
            let got = self.store.write(|tx| -> Result<(), Out> {
                let since = rikuri_ledger::tail(tx, 1)?.last().map(|e| e.seq);
                command(tx, &key)?;
                Err(Out::Problems(check_command(tx, name, &key, since)?))
            });
            match got {
                Err(Out::Problems(p)) => p,
                other => panic!("the command itself failed: {other:?}"),
            }
        }

        /// Runs and commits a command that must pass the whole check.
        fn commit(&self, name: &str, command: impl FnOnce(&Tx<'_>, &str) -> Result<(), Out>) {
            let key = self.key();
            self.store
                .write(|tx| -> Result<(), Out> {
                    let since = rikuri_ledger::tail(tx, 1)?.last().map(|e| e.seq);
                    command(tx, &key)?;
                    let problems = check_command(tx, name, &key, since)?;
                    assert!(
                        problems.is_empty(),
                        "{name} as the kernel does it: {problems:?}"
                    );
                    Ok(())
                })
                .unwrap();
        }
    }

    /// What one command writes and appends, built as the kernel builds it
    /// and not yet written: a test changes one thing to plant a defect.
    struct Effect {
        task: Task,
        task_version: Option<u64>,
        run: Option<(Run, Option<u64>)>,
        events: Vec<NewEvent>,
    }

    fn apply(tx: &Tx<'_>, e: Effect) -> Result<(), Out> {
        let tasks = SqliteRepository::<Task>::new();
        match e.task_version {
            None => tasks.insert(tx, &e.task, T0_TIME).map(|_| ())?,
            Some(v) => tasks.update(tx, &e.task, v, T0_TIME).map(|_| ())?,
        }
        if let Some((run, version)) = &e.run {
            let runs = SqliteRepository::<Run>::new();
            match version {
                None => runs.insert(tx, run, T0_TIME).map(|_| ())?,
                Some(v) => runs.update(tx, run, *v, T0_TIME).map(|_| ())?,
            }
        }
        for event in &e.events {
            rikuri_ledger::append(tx, event, T0_TIME)?;
        }
        Ok(())
    }

    /// The kernel's event of type `kind` about `task` (and `run`).
    fn event(f: &F, kind: &str, key: &str, task: &Task, run: Option<RunId>) -> NewEvent {
        NewEvent {
            event_id: EventId::generate(&f.ids).unwrap(),
            event_type: EventType::new(kind).unwrap(),
            status: EventStatus::Succeeded,
            risk: RiskClass::NovelModification,
            actor_id: f.owner.id,
            project_id: Some(task.project_id),
            task_id: Some(task.id),
            run_id: run,
            subject_ids: match run {
                Some(r) => vec![task.id.into(), r.into()],
                None => vec![task.id.into()],
            },
            input_refs: vec![],
            result_refs: vec![],
            security_decision_id: None,
            cost_record_id: None,
            idempotency_key: Some(IdempotencyKey::new(key).unwrap()),
            caused_by: None,
            metadata: Map::new(),
        }
    }

    fn get_task(tx: &Tx<'_>, id: TaskId) -> Result<(Task, u64), Out> {
        let v = SqliteRepository::<Task>::new().get(tx, id)?.unwrap();
        Ok((v.value, v.version))
    }

    fn get_run(tx: &Tx<'_>, id: RunId) -> Result<(Run, u64), Out> {
        let v = SqliteRepository::<Run>::new().get(tx, id)?.unwrap();
        Ok((v.value, v.version))
    }

    // -- the four task commands, as the kernel builds them -----------------

    fn create(f: &F, key: &str, privacy_class: PrivacyClass) -> Effect {
        let task = Task {
            id: TaskId::generate(&f.ids).unwrap(),
            project_id: f.owner.project_id,
            title: "a task".into(),
            privacy_class,
            deletion_unit_id: DeletionUnitId::generate(&f.ids).unwrap(),
            status: TaskStatus::Created,
            steps_completed: 0,
            runs: 0,
            current_run: None,
            last_run: None,
        };
        let mut e = event(f, "task.created", key, &task, None);
        e.result_refs = vec![task.id.into()];
        Effect {
            task,
            task_version: None,
            run: None,
            events: vec![e],
        }
    }

    fn start(f: &F, tx: &Tx<'_>, key: &str, id: TaskId) -> Result<Effect, Out> {
        let (mut task, version) = get_task(tx, id)?;
        let run_id = RunId::generate(&f.ids).unwrap();
        let mut e = event(f, "run.started", key, &task, Some(run_id));
        let run = Run {
            id: run_id,
            task_id: id,
            attempt: task.runs + 1,
            status: RunStatus::Running,
            steps: 0,
            outcome: None,
            verification: None,
            started_event_id: e.event_id,
            completion_event_id: None,
            verification_event_id: None,
        };
        e.result_refs = vec![run_id.into()];
        e.metadata.insert("attempt".into(), json!(run.attempt));
        task.status = TaskStatus::Running;
        task.runs += 1;
        task.current_run = Some(run_id);
        task.last_run = Some(run_id);
        Ok(Effect {
            task,
            task_version: Some(version),
            run: Some((run, None)),
            events: vec![e],
        })
    }

    fn step(f: &F, tx: &Tx<'_>, key: &str, id: TaskId) -> Result<Effect, Out> {
        let (mut task, tv) = get_task(tx, id)?;
        let run_id = task.current_run.unwrap();
        let (mut run, rv) = get_run(tx, run_id)?;
        run.steps += 1;
        task.steps_completed += 1;
        let mut e = event(f, "task.stepped", key, &task, Some(run_id));
        e.metadata
            .insert("steps_completed".into(), json!(task.steps_completed));
        Ok(Effect {
            task,
            task_version: Some(tv),
            run: Some((run, Some(rv))),
            events: vec![e],
        })
    }

    fn complete(f: &F, tx: &Tx<'_>, key: &str, id: TaskId) -> Result<Effect, Out> {
        let (mut task, tv) = get_task(tx, id)?;
        let run_id = task.current_run.unwrap();
        let (mut run, rv) = get_run(tx, run_id)?;
        let mut c = event(f, "run.completed", key, &task, Some(run_id));
        let mut v = event(f, "run.verified", key, &task, Some(run_id));
        c.result_refs = vec![run_id.into()];
        c.metadata.insert("outcome".into(), json!(Outcome::Failed));
        v.caused_by = Some(c.event_id);
        v.metadata
            .insert("verification".into(), json!(VerificationStatus::Failed));
        run.status = RunStatus::Completed;
        run.outcome = Some(Outcome::Failed);
        run.verification = Some(VerificationStatus::Failed);
        run.completion_event_id = Some(c.event_id);
        run.verification_event_id = Some(v.event_id);
        task.status = TaskStatus::Completed;
        task.current_run = None;
        Ok(Effect {
            task,
            task_version: Some(tv),
            run: Some((run, Some(rv))),
            events: vec![c, v],
        })
    }

    /// A private task, created and committed.
    fn created(f: &F) -> TaskId {
        let mut id = None;
        f.commit("task.create", |tx, key| {
            let e = create(f, key, PrivacyClass::Private);
            id = Some(e.task.id);
            apply(tx, e)
        });
        id.unwrap()
    }

    /// A private task, created and started (committed).
    fn started(f: &F) -> TaskId {
        let t = created(f);
        f.commit("task.start", |tx, key| apply(tx, start(f, tx, key, t)?));
        t
    }

    fn current_run(f: &F, id: TaskId) -> RunId {
        f.store
            .read(|tx| Ok::<_, Out>(get_task(tx, id)?.0.current_run.unwrap()))
            .unwrap()
    }

    /// A step of task `id` naming run `run_id`, as the kernel builds it
    /// from a request (the request names the run).
    fn step_on(f: &F, tx: &Tx<'_>, key: &str, id: TaskId, run_id: RunId) -> Result<Effect, Out> {
        let (mut task, tv) = get_task(tx, id)?;
        let (mut run, rv) = get_run(tx, run_id)?;
        run.steps += 1;
        task.steps_completed += 1;
        let mut e = event(f, "task.stepped", key, &task, Some(run_id));
        e.metadata
            .insert("steps_completed".into(), json!(task.steps_completed));
        Ok(Effect {
            task,
            task_version: Some(tv),
            run: Some((run, Some(rv))),
            events: vec![e],
        })
    }

    /// A completion of task `id` naming run `run_id`, as the kernel builds it.
    fn complete_on(
        f: &F,
        tx: &Tx<'_>,
        key: &str,
        id: TaskId,
        run_id: RunId,
    ) -> Result<Effect, Out> {
        let (mut task, tv) = get_task(tx, id)?;
        let (mut run, rv) = get_run(tx, run_id)?;
        let mut c = event(f, "run.completed", key, &task, Some(run_id));
        let mut v = event(f, "run.verified", key, &task, Some(run_id));
        c.result_refs = vec![run_id.into()];
        c.metadata
            .insert("outcome".into(), json!(Outcome::Succeeded));
        v.caused_by = Some(c.event_id);
        v.metadata
            .insert("verification".into(), json!(VerificationStatus::Passed));
        run.status = RunStatus::Completed;
        run.outcome = Some(Outcome::Succeeded);
        run.verification = Some(VerificationStatus::Passed);
        run.completion_event_id = Some(c.event_id);
        run.verification_event_id = Some(v.event_id);
        task.status = TaskStatus::Completed;
        task.current_run = None;
        Ok(Effect {
            task,
            task_version: Some(tv),
            run: Some((run, Some(rv))),
            events: vec![c, v],
        })
    }

    /// Changes committed state directly, bypassing every check: a store
    /// already inconsistent before the command runs.
    fn tamper(f: &F, change: impl FnOnce(&Tx<'_>) -> Result<(), Out>) {
        f.store.write(change).unwrap();
    }

    /// An empty store: the owner is chosen but not yet written.
    fn bare() -> F {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::open(
            &dir.path().join("state/rikuri.db"),
            Box::new(FixtureClock::fixed(T0_TIME)),
            &[&rikuri_ledger::MIGRATIONS],
        )
        .unwrap();
        let ids = FixtureIds::sequential(T0);
        let owner = Owner {
            id: ActorId::generate(&ids).unwrap(),
            project_id: ProjectId::generate(&ids).unwrap(),
            initialized_at: T0_TIME,
        };
        F {
            _dir: dir,
            store,
            ids,
            owner,
            n: Cell::new(0),
        }
    }

    /// What `system.init` writes and appends, as the kernel builds it.
    struct Init {
        owner: Owner,
        project: Project,
        event: NewEvent,
    }

    fn init(f: &F, key: &str) -> Init {
        let owner = f.owner.clone();
        let project = Project {
            id: owner.project_id,
            name: "Inbox".into(),
            trust: ProjectTrust::default(),
            intent: ExecutionIntent::default(),
        };
        let event = NewEvent {
            event_id: EventId::generate(&f.ids).unwrap(),
            event_type: EventType::new("system.initialized").unwrap(),
            status: EventStatus::Succeeded,
            risk: RiskClass::NovelModification,
            actor_id: owner.id,
            project_id: Some(project.id),
            task_id: None,
            run_id: None,
            subject_ids: vec![owner.id.into(), project.id.into()],
            input_refs: vec![],
            result_refs: vec![owner.id.into(), project.id.into()],
            security_decision_id: None,
            cost_record_id: None,
            idempotency_key: Some(IdempotencyKey::new(key).unwrap()),
            caused_by: None,
            metadata: Map::new(),
        };
        Init {
            owner,
            project,
            event,
        }
    }

    fn apply_init(tx: &Tx<'_>, i: Init) -> Result<(), Out> {
        SqliteRepository::<Owner>::new().insert(tx, &i.owner, T0_TIME)?;
        SqliteRepository::<Project>::new().insert(tx, &i.project, T0_TIME)?;
        rikuri_ledger::append(tx, &i.event, T0_TIME)?;
        Ok(())
    }

    impl F {
        /// Runs an initialization under a fresh key, checks it as
        /// `system.init`, and rolls back.
        fn check_init(
            &self,
            command: impl FnOnce(&Tx<'_>, &str) -> Result<(), Out>,
        ) -> Vec<String> {
            let key = self.key();
            let got = self.store.write(|tx| -> Result<(), Out> {
                let since = rikuri_ledger::tail(tx, 1)?.last().map(|e| e.seq);
                command(tx, &key)?;
                Err(Out::Problems(check_init(tx, &key, since)?))
            });
            match got {
                Err(Out::Problems(p)) => p,
                other => panic!("the initialization itself failed: {other:?}"),
            }
        }
    }

    /// A named field and a change to it that plants a defect.
    type Wrong<T> = (&'static str, Box<dyn Fn(&mut T)>);

    fn has(problems: &[String], needle: &str) {
        assert!(
            problems.iter().any(|p| p.contains(needle)),
            "no problem mentions {needle:?}: {problems:?}"
        );
    }

    // -- legitimate commands pass --------------------------------------------

    #[test]
    fn each_command_built_as_the_kernel_builds_it_passes() {
        let f = fixture();
        let t = started(&f);
        f.commit("task.step", |tx, key| apply(tx, step(&f, tx, key, t)?));
        f.commit("task.step", |tx, key| apply(tx, step(&f, tx, key, t)?));
        f.commit("task.complete", |tx, key| {
            apply(tx, complete(&f, tx, key, t)?)
        });
        // a second run of the same task, and a second task
        f.commit("task.start", |tx, key| apply(tx, start(&f, tx, key, t)?));
        started(&f);
    }

    // -- commands and the events they append -------------------------------

    #[test]
    fn a_command_without_a_spec_is_refused() {
        let f = fixture();
        let t = started(&f);
        let p = f.check("task.rename", |tx, key| apply(tx, step(&f, tx, key, t)?));
        has(&p, "no candidate spec covers command task.rename");
    }

    /// V9 and V11 (verifier round 3): a step that also creates a hidden
    /// task, and a step applied twice with a twin event under another key.
    #[test]
    fn a_command_appends_exactly_its_own_events() {
        let f = fixture();
        let t = started(&f);
        let p = f.check("task.step", |tx, key| {
            apply(tx, step(&f, tx, key, t)?)?;
            apply(tx, create(&f, key, PrivacyClass::Normal))
        });
        has(
            &p,
            r#"task.step appends ["task.stepped"], but this command appended ["task.stepped", "task.created"]"#,
        );
        let p = f.check("task.step", |tx, key| {
            let mut e = step(&f, tx, key, t)?;
            let mut twin = e.events[0].clone();
            twin.event_id = EventId::generate(&f.ids).unwrap();
            twin.idempotency_key = Some(IdempotencyKey::new("key-spec-twin01").unwrap());
            e.task.steps_completed += 1;
            if let Some((run, _)) = e.run.as_mut() {
                run.steps += 1;
            }
            e.events.push(twin);
            apply(tx, e)
        });
        has(
            &p,
            r#"task.step appends ["task.stepped"], but this command appended ["task.stepped", "task.stepped"]"#,
        );
    }

    /// Every field of a command's events: the owner acts, under the
    /// command's key, with the command's references and facts (V7: a step
    /// event marked failed, with a step count that is not the task's).
    #[test]
    fn an_event_carries_exactly_the_commands_references_and_facts() {
        let f = fixture();
        let t = started(&f);
        let earlier = f
            .store
            .read(|tx| {
                let (task, _) = get_task(tx, t)?;
                Ok::<_, Out>(get_run(tx, task.current_run.unwrap())?.0.started_event_id)
            })
            .unwrap();
        let other_actor = ActorId::generate(&f.ids).unwrap();
        let other_project = ProjectId::generate(&f.ids).unwrap();
        let decision = SecurityDecisionId::generate(&f.ids).unwrap();
        let cost = CostRecordId::generate(&f.ids).unwrap();
        let wrong: Vec<Wrong<NewEvent>> = vec![
            ("status", Box::new(|e| e.status = EventStatus::Failed)),
            ("actor_id", Box::new(move |e| e.actor_id = other_actor)),
            (
                "project_id",
                Box::new(move |e| e.project_id = Some(other_project)),
            ),
            (
                "idempotency_key",
                Box::new(|e| {
                    e.idempotency_key = Some(IdempotencyKey::new("key-spec-other1").unwrap())
                }),
            ),
            ("subject_ids", Box::new(|e| e.subject_ids.clear())),
            (
                "result_refs",
                Box::new(|e| e.result_refs = e.subject_ids.clone()),
            ),
            (
                "input_refs",
                Box::new(|e| e.input_refs = e.subject_ids.clone()),
            ),
            ("caused_by", Box::new(move |e| e.caused_by = Some(earlier))),
            (
                "security_decision_id",
                Box::new(move |e| e.security_decision_id = Some(decision)),
            ),
            (
                "cost_record_id",
                Box::new(move |e| e.cost_record_id = Some(cost)),
            ),
            (
                "metadata",
                Box::new(|e| {
                    e.metadata.insert("steps_completed".into(), json!(999));
                }),
            ),
        ];
        for (field, change) in wrong {
            let p = f.check("task.step", |tx, key| {
                let mut e = step(&f, tx, key, t)?;
                change(&mut e.events[0]);
                apply(tx, e)
            });
            assert!(
                p.iter().any(|p| p.contains("task.stepped event")
                    && p.contains(&format!("differs from its spec in: {field}"))),
                "{field}: {p:?}"
            );
        }
    }

    /// A completion's second event names the same task and run as its
    /// first. The written objects are taken from the first event, so only
    /// the event spec sees the second name another task or run.
    #[test]
    fn a_completions_second_event_names_its_own_task_and_run() {
        let f = fixture();
        let t = started(&f);
        let other = started(&f);
        let other_run = f
            .store
            .read(|tx| Ok::<_, Out>(get_task(tx, other)?.0.current_run.unwrap()))
            .unwrap();
        let wrong: Vec<Wrong<NewEvent>> = vec![
            ("task_id", Box::new(move |e| e.task_id = Some(other))),
            ("run_id", Box::new(move |e| e.run_id = Some(other_run))),
        ];
        for (field, change) in wrong {
            let p = f.check("task.complete", |tx, key| {
                let mut e = complete(&f, tx, key, t)?;
                change(&mut e.events[1]);
                apply(tx, e)
            });
            assert!(
                p.iter().any(|p| p.contains("run.verified event")
                    && p.contains(&format!("differs from its spec in: {field}"))),
                "{field}: {p:?}"
            );
        }
    }

    // -- the objects a command writes ----------------------------------------

    /// A step that also rewrites another task (no event names it); the
    /// refusal names what the command wrote.
    #[test]
    fn a_command_writes_only_its_own_objects() {
        let f = fixture();
        let t = started(&f);
        let other = created(&f);
        let run = f
            .store
            .read(|tx| Ok::<_, Out>(get_task(tx, t)?.0.current_run.unwrap()))
            .unwrap();
        let p = f.check("task.step", |tx, key| {
            apply(tx, step(&f, tx, key, t)?)?;
            let (mut o, v) = get_task(tx, other)?;
            o.title = "rewritten".into();
            SqliteRepository::<Task>::new().update(tx, &o, v, T0_TIME)?;
            Ok(())
        });
        has(
            &p,
            &format!(
                "task.step writes task {t} and run {run}, but this command wrote tasks [{t}, {other}] and runs [{run}]"
            ),
        );
    }

    /// V1–V3 (verifier round 3): a step that downgrades its own task's
    /// privacy class, rewrites its title, replaces its deletion unit or
    /// moves it to another project; and every other field a step does not
    /// change.
    #[test]
    fn a_command_changes_no_field_of_its_task_beyond_its_own() {
        let f = fixture();
        let t = started(&f);
        let other_project = ProjectId::generate(&f.ids).unwrap();
        let other_unit = DeletionUnitId::generate(&f.ids).unwrap();
        let other_run = RunId::generate(&f.ids).unwrap();
        let wrong: Vec<Wrong<Task>> = vec![
            (
                "title",
                Box::new(|t| t.title = "overwritten by its own step".into()),
            ),
            (
                "privacy_class",
                Box::new(|t| t.privacy_class = PrivacyClass::Normal),
            ),
            (
                "deletion_unit_id",
                Box::new(move |t| t.deletion_unit_id = other_unit),
            ),
            (
                "project_id",
                Box::new(move |t| t.project_id = other_project),
            ),
            ("runs", Box::new(|t| t.runs += 1)),
            ("last_run", Box::new(move |t| t.last_run = Some(other_run))),
            ("status", Box::new(|t| t.status = TaskStatus::Completed)),
        ];
        for (field, change) in wrong {
            let p = f.check("task.step", |tx, key| {
                let mut e = step(&f, tx, key, t)?;
                change(&mut e.task);
                apply(tx, e)
            });
            has(&p, &format!("task {t} differs from its spec in: {field}"));
        }
    }

    /// V8 (verifier round 3): a step that changes its run's attempt.
    #[test]
    fn a_step_changes_no_field_of_its_run_beyond_the_step_count() {
        let f = fixture();
        let t = started(&f);
        let p = f.check("task.step", |tx, key| {
            let mut e = step(&f, tx, key, t)?;
            if let Some((run, _)) = e.run.as_mut() {
                run.attempt += 10;
            }
            apply(tx, e)
        });
        has(&p, "differs from its spec in: attempt");
    }

    /// V4, V5 (verifier round 3) and D4: a start whose new run has the
    /// wrong attempt, one that leaves the task's last run stale, and a
    /// start on a task that is already running.
    #[test]
    fn a_start_moves_its_task_and_starts_its_run_exactly() {
        let f = fixture();
        let t = created(&f);
        let p = f.check("task.start", |tx, key| {
            let mut e = start(&f, tx, key, t)?;
            if let Some((run, _)) = e.run.as_mut() {
                run.attempt += 1;
            }
            apply(tx, e)
        });
        has(&p, "differs from its spec in: attempt");
        let p = f.check("task.start", |tx, key| {
            let mut e = start(&f, tx, key, t)?;
            e.task.last_run = None;
            apply(tx, e)
        });
        has(&p, &format!("task {t} differs from its spec in: last_run"));
        f.commit("task.start", |tx, key| apply(tx, start(&f, tx, key, t)?));
        let p = f.check("task.start", |tx, key| apply(tx, start(&f, tx, key, t)?));
        has(
            &p,
            &format!("task.start on task {t}, which is already running"),
        );
    }

    /// V6 (verifier round 3): a completion that leaves its task `created`,
    /// one that changes its run's step count, and one whose completion
    /// event records no valid outcome.
    #[test]
    fn a_completion_ends_its_task_and_run_exactly() {
        let f = fixture();
        let t = started(&f);
        let p = f.check("task.complete", |tx, key| {
            let mut e = complete(&f, tx, key, t)?;
            e.task.status = TaskStatus::Created;
            apply(tx, e)
        });
        has(&p, &format!("task {t} differs from its spec in: status"));
        let p = f.check("task.complete", |tx, key| {
            let mut e = complete(&f, tx, key, t)?;
            if let Some((run, _)) = e.run.as_mut() {
                run.steps += 1;
            }
            apply(tx, e)
        });
        has(&p, "differs from its spec in: steps");
        let p = f.check("task.complete", |tx, key| {
            let mut e = complete(&f, tx, key, t)?;
            e.events[0]
                .metadata
                .insert("outcome".into(), json!("not an outcome"));
            apply(tx, e)
        });
        has(&p, "records no valid outcome");
    }

    /// A new task belongs to the owner's project and starts empty.
    #[test]
    fn a_new_task_is_the_owners_and_starts_empty() {
        let f = fixture();
        let other_project = ProjectId::generate(&f.ids).unwrap();
        let p = f.check("task.create", |tx, key| {
            let mut e = create(&f, key, PrivacyClass::Normal);
            e.task.project_id = other_project;
            e.events[0].project_id = Some(other_project);
            apply(tx, e)
        });
        has(&p, "differs from its spec in: project_id");
        let p = f.check("task.create", |tx, key| {
            let mut e = create(&f, key, PrivacyClass::Normal);
            e.task.steps_completed = 2;
            apply(tx, e)
        });
        has(&p, "differs from its spec in: steps_completed");
    }

    /// N1, N2 (verifier round 4): a step or a completion of task a that
    /// names task b's running run, with a running and with a never
    /// started; then, one at a time, each condition the run a step names
    /// must meet: the task is running, the run is its current run, the run
    /// is the task's own, and the run is running.
    #[test]
    fn a_step_or_completion_names_its_tasks_running_current_run() {
        const NOT_ITS_OWN: &str = "which is not the task's running current run";
        for (name, a_started) in [
            ("task.step", true),
            ("task.step", false),
            ("task.complete", true),
            ("task.complete", false),
        ] {
            let f = fixture();
            let a = if a_started { started(&f) } else { created(&f) };
            let b = started(&f);
            let rb = current_run(&f, b);
            let p = f.check(name, |tx, key| {
                let e = if name == "task.step" {
                    step_on(&f, tx, key, a, rb)?
                } else {
                    complete_on(&f, tx, key, a, rb)?
                };
                apply(tx, e)
            });
            assert!(
                p.iter().any(|p| p.contains(NOT_ITS_OWN)),
                "{name} with a started={a_started}: {p:?}"
            );
        }

        // the task is not running, though its current run is
        let f = fixture();
        let a = started(&f);
        let ra = current_run(&f, a);
        tamper(&f, |tx| {
            let (mut t, v) = get_task(tx, a)?;
            t.status = TaskStatus::Created;
            SqliteRepository::<Task>::new().update(tx, &t, v, T0_TIME)?;
            Ok(())
        });
        let p = f.check("task.step", |tx, key| {
            apply(tx, step_on(&f, tx, key, a, ra)?)
        });
        has(&p, NOT_ITS_OWN);

        // the run is the task's own and running, but not its current run
        let f = fixture();
        let a = started(&f);
        let ra = current_run(&f, a);
        let second = RunId::generate(&f.ids).unwrap();
        tamper(&f, |tx| {
            let (r, _) = get_run(tx, ra)?;
            let r = Run {
                id: second,
                attempt: 2,
                ..r
            };
            SqliteRepository::<Run>::new().insert(tx, &r, T0_TIME)?;
            Ok(())
        });
        let p = f.check("task.step", |tx, key| {
            apply(tx, step_on(&f, tx, key, a, second)?)
        });
        has(&p, NOT_ITS_OWN);

        // the task's current run is another task's
        let f = fixture();
        let a = started(&f);
        let b = started(&f);
        let rb = current_run(&f, b);
        tamper(&f, |tx| {
            let (mut t, v) = get_task(tx, a)?;
            t.current_run = Some(rb);
            SqliteRepository::<Task>::new().update(tx, &t, v, T0_TIME)?;
            Ok(())
        });
        let p = f.check("task.step", |tx, key| {
            apply(tx, step_on(&f, tx, key, a, rb)?)
        });
        has(&p, NOT_ITS_OWN);

        // the task's current run is its own but no longer running
        let f = fixture();
        let a = started(&f);
        let ra = current_run(&f, a);
        tamper(&f, |tx| {
            let (mut r, v) = get_run(tx, ra)?;
            r.status = RunStatus::Completed;
            SqliteRepository::<Run>::new().update(tx, &r, v, T0_TIME)?;
            Ok(())
        });
        let p = f.check("task.step", |tx, key| {
            apply(tx, step_on(&f, tx, key, a, ra)?)
        });
        has(&p, NOT_ITS_OWN);
    }

    /// N5 (verifier round 4): a new task that shares an existing task's
    /// deletion unit, so forgetting one would forget the other's content.
    #[test]
    fn a_new_task_has_a_deletion_unit_of_its_own() {
        let f = fixture();
        let t = created(&f);
        let unit = f
            .store
            .read(|tx| Ok::<_, Out>(get_task(tx, t)?.0.deletion_unit_id))
            .unwrap();
        let p = f.check("task.create", |tx, key| {
            let mut e = create(&f, key, PrivacyClass::Normal);
            e.task.deletion_unit_id = unit;
            apply(tx, e)
        });
        has(&p, &format!("deletion unit {unit} is not its own"));
    }

    /// N8 (verifier round 4): `system.init` writes exactly one owner and one
    /// Inbox project with default trust and intent, and its event; an
    /// Inbox trusted for execution from the start is refused.
    #[test]
    fn system_init_writes_one_owner_and_one_inbox_with_default_trust() {
        let f = bare();
        assert_eq!(
            f.check_init(|tx, key| apply_init(tx, init(&f, key))),
            Vec::<String>::new()
        );
        let other_project = ProjectId::generate(&f.ids).unwrap();
        let stranger = ActorId::generate(&f.ids).unwrap();
        let wrong: Vec<Wrong<Init>> = vec![
            (
                "trust",
                Box::new(|i| i.project.trust = ProjectTrust::TrustedScopedExecute),
            ),
            (
                "intent",
                Box::new(|i| i.project.intent = ExecutionIntent::LocalOnly),
            ),
            ("name", Box::new(|i| i.project.name = "Elsewhere".into())),
            (
                "project_id",
                Box::new(move |i| i.owner.project_id = other_project),
            ),
            ("actor_id", Box::new(move |i| i.event.actor_id = stranger)),
            ("subject_ids", Box::new(|i| i.event.subject_ids.clear())),
        ];
        for (field, change) in wrong {
            let p = f.check_init(|tx, key| {
                let mut i = init(&f, key);
                change(&mut i);
                apply_init(tx, i)
            });
            has(&p, &format!("differs from its spec in: {field}"));
        }
        // something else written as well
        let p = f.check_init(|tx, key| {
            let extra = create(&f, key, PrivacyClass::Normal).task;
            apply_init(tx, init(&f, key))?;
            SqliteRepository::<Task>::new().insert(tx, &extra, T0_TIME)?;
            Ok(())
        });
        has(&p, "system.init writes one owner and one project");
    }

    // -- the answer a command returns (and records) ----------------------------

    impl F {
        /// Runs `command`, which returns the answer it would give, checks
        /// that answer as command `name`'s, and rolls back.
        fn check_answer(
            &self,
            name: &str,
            command: impl FnOnce(&Tx<'_>, &str) -> Result<serde_json::Value, Out>,
        ) -> Vec<String> {
            let key = self.key();
            let got = self.store.write(|tx| -> Result<(), Out> {
                let since = rikuri_ledger::tail(tx, 1)?.last().map(|e| e.seq);
                let answer = command(tx, &key)?;
                Err(Out::Problems(crate::candidate::check_answer(
                    tx, name, &answer, since,
                )?))
            });
            match got {
                Err(Out::Problems(p)) => p,
                other => panic!("the command itself failed: {other:?}"),
            }
        }
    }

    /// The answer each command gives, as the kernel builds it from the
    /// effect it wrote.
    fn answer(name: &str, e: &Effect) -> serde_json::Value {
        let version = e.task_version.map_or(1, |v| v + 1);
        let run = e.run.as_ref().map(|(r, _)| r);
        match name {
            "task.create" => json!({
                "task_id": e.task.id, "version": version, "event_id": e.events[0].event_id
            }),
            "task.start" => json!({
                "task_id": e.task.id, "run_id": run.unwrap().id, "version": version,
                "event_id": e.events[0].event_id
            }),
            "task.step" => json!({
                "task_id": e.task.id, "run_id": run.unwrap().id,
                "steps_completed": e.task.steps_completed, "version": version,
                "event_id": e.events[0].event_id
            }),
            _ => json!({
                "task_id": e.task.id, "run_id": run.unwrap().id,
                "outcome": run.unwrap().outcome, "verification": run.unwrap().verification,
                "version": version,
                "event_ids": [e.events[0].event_id, e.events[1].event_id]
            }),
        }
    }

    /// R1, R3 (verifier round 6): a command whose answer names something
    /// other than what it wrote records that answer too, so the first call
    /// and every retry give it. Each command's answer must name exactly the
    /// task, run, version and events it wrote, and what it reports.
    #[test]
    fn a_commands_answer_names_what_it_wrote() {
        let f = fixture();
        let other = created(&f);
        let t = created(&f);
        let other_id = json!(other);
        type Build<'a> = Box<dyn Fn(&F, &Tx<'_>, &str) -> Result<Effect, Out> + 'a>;
        let commands: Vec<(&str, Build<'_>)> = vec![
            (
                "task.start",
                Box::new(move |f: &F, tx: &Tx<'_>, key: &str| start(f, tx, key, t)),
            ),
            (
                "task.step",
                Box::new(move |f: &F, tx: &Tx<'_>, key: &str| step(f, tx, key, t)),
            ),
            (
                "task.complete",
                Box::new(move |f: &F, tx: &Tx<'_>, key: &str| complete(f, tx, key, t)),
            ),
        ];
        // R1: a create that answers with another task's ID; then each field
        // of each command's answer, one at a time
        let p = f.check_answer("task.create", |tx, key| {
            let e = create(&f, key, PrivacyClass::Private);
            let mut a = answer("task.create", &e);
            a["task_id"] = other_id.clone();
            apply(tx, e)?;
            Ok(a)
        });
        has(
            &p,
            "task.create's answer differs from what it wrote in: task_id",
        );
        assert_eq!(
            f.check_answer("task.create", |tx, key| {
                let e = create(&f, key, PrivacyClass::Private);
                let a = answer("task.create", &e);
                apply(tx, e)?;
                Ok(a)
            }),
            Vec::<String>::new()
        );
        for (name, build) in &commands {
            assert_eq!(
                f.check_answer(name, |tx, key| {
                    let e = build(&f, tx, key)?;
                    let a = answer(name, &e);
                    apply(tx, e)?;
                    Ok(a)
                }),
                Vec::<String>::new(),
                "{name} as the kernel answers"
            );
            let fields: Vec<&str> = match *name {
                "task.start" => vec!["task_id", "run_id", "version", "event_id"],
                "task.step" => vec![
                    "task_id",
                    "run_id",
                    "steps_completed",
                    "version",
                    "event_id",
                ],
                _ => vec![
                    "task_id",
                    "run_id",
                    "outcome",
                    "verification",
                    "version",
                    "event_ids",
                ],
            };
            for field in fields {
                let p = f.check_answer(name, |tx, key| {
                    let e = build(&f, tx, key)?;
                    let mut a = answer(name, &e);
                    a[field] = json!("something else");
                    apply(tx, e)?;
                    Ok(a)
                });
                has(
                    &p,
                    &format!("{name}'s answer differs from what it wrote in: {field}"),
                );
            }
            // the command runs for real, so the next one has its state
            f.commit(name, |tx, key| apply(tx, build(&f, tx, key)?));
        }
        // an answer that says more than the command did
        let p = f.check_answer("task.create", |tx, key| {
            let e = create(&f, key, PrivacyClass::Private);
            let mut a = answer("task.create", &e);
            a["note"] = json!("extra");
            apply(tx, e)?;
            Ok(a)
        });
        has(
            &p,
            "task.create's answer differs from what it wrote in: note",
        );
    }

    /// R3 (verifier round 6): `system.init` answers its owner, its Inbox
    /// and its event; another project's ID is refused.
    #[test]
    fn system_inits_answer_names_what_it_wrote() {
        let f = bare();
        let answer = |i: &Init| json!({"actor_id": i.owner.id, "project_id": i.project.id, "event_id": i.event.event_id});
        assert_eq!(
            f.check_answer("system.init", |tx, key| {
                let i = init(&f, key);
                let a = answer(&i);
                apply_init(tx, i)?;
                Ok(a)
            }),
            Vec::<String>::new()
        );
        let elsewhere = ProjectId::generate(&f.ids).unwrap();
        let stranger = ActorId::generate(&f.ids).unwrap();
        for (field, value) in [
            ("project_id", json!(elsewhere)),
            ("actor_id", json!(stranger)),
            ("event_id", json!("another event")),
        ] {
            let p = f.check_answer("system.init", |tx, key| {
                let i = init(&f, key);
                let mut a = answer(&i);
                a[field] = value.clone();
                apply_init(tx, i)?;
                Ok(a)
            });
            has(
                &p,
                &format!("system.init's answer differs from what it wrote in: {field}"),
            );
        }
    }

    /// S1 (verifier round 6): an event stamped with a schema version other
    /// than the ledger's current one. Only the ledger sets the field, so
    /// the defect is planted below it: the append-only trigger is dropped
    /// inside the test's own transaction, which is then rolled back.
    #[test]
    fn an_event_carries_the_ledgers_schema_version() {
        let restamp = |tx: &Tx<'_>, event: EventId| -> Result<(), Out> {
            tx.execute_batch("DROP TRIGGER events_no_update")
                .map_err(StorageError::from)?;
            tx.execute(
                "UPDATE events SET schema_version = 7 WHERE event_id = ?1",
                [event.to_string()],
            )
            .map_err(StorageError::from)?;
            Ok(())
        };
        let f = fixture();
        let t = started(&f);
        let p = f.check("task.step", |tx, key| {
            let e = step(&f, tx, key, t)?;
            let id = e.events[0].event_id;
            apply(tx, e)?;
            restamp(tx, id)
        });
        has(&p, "differs from its spec in: schema_version");
        let f = bare();
        let p = f.check_init(|tx, key| {
            let i = init(&f, key);
            let id = i.event.event_id;
            apply_init(tx, i)?;
            restamp(tx, id)
        });
        has(&p, "differs from its spec in: schema_version");
    }
}
