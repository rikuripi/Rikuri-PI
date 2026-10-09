//! The ledger-consistency replay is the crash matrix's oracle (W1), so it
//! must check history against state in both directions, not only the links
//! state points at (2026-10-08: a run state showed as running while the
//! ledger had its completion, and a changed outcome, both passed; the S00.07
//! record deferred this to the crash matrix). The verifier then found the
//! same class between state's own pointers (a task that no longer points at
//! its running run) and for the owner (`system.initialized` never compared).
#![cfg(unix)]

use std::path::Path;

use rikuri_api::{Request, Response, command};
use rikuri_domain::{
    ActorId, EventId, ProjectId, RiskClass, RunId, SystemClock, SystemIdSource, TaskId,
};
use rikuri_ledger::{EventStatus, EventType, IdempotencyKey, NewEvent};
use rikurid::crash::NoCrash;
use rikurid::kernel::database_path;
use rikurid::{Idempotency, Kernel, KernelConfig};
use serde_json::{Value, json};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn open(home: &Path) -> Result<Kernel, rikurid::KernelError> {
    Kernel::open(
        home,
        KernelConfig {
            ids: Box::new(SystemIdSource::new()),
            clock: Box::new(SystemClock),
            crash: Box::new(NoCrash),
            idempotency: Idempotency::Enforced,
        },
    )
}

fn keyed(k: &Kernel, command: &str, params: Value, key: &str) -> Response {
    k.handle(&Request::new(command, params).with_idempotency_key(key))
}

fn result(r: &Response) -> Value {
    r.result.clone().unwrap_or(Value::Null)
}

/// init → create → start; returns the kernel and the start result.
fn started_run(home: &Path) -> TestResult<(Kernel, Value)> {
    let k = open(home)?;
    k.handle(&Request::new(command::SYSTEM_INIT, json!({})));
    let created = result(&keyed(
        &k,
        command::TASK_CREATE,
        json!({"title": "t"}),
        "key-create-0001",
    ));
    let started = keyed(
        &k,
        command::TASK_START,
        json!({"task_id": created["task_id"], "expected_version": created["version"]}),
        "key-start-00001",
    );
    if started.error.is_some() {
        return Err(format!("start failed: {started:?}").into());
    }
    Ok((k, result(&started)))
}

/// init → create → start → complete; returns the kernel, ready for checks.
fn completed_run(home: &Path) -> TestResult<Kernel> {
    let (k, started) = started_run(home)?;
    let done = keyed(
        &k,
        command::TASK_COMPLETE,
        json!({"task_id": started["task_id"], "run_id": started["run_id"],
               "expected_version": started["version"],
               "outcome": "succeeded", "verification": "passed"}),
        "key-done-000001",
    );
    if done.error.is_some() {
        return Err(format!("complete failed: {done:?}").into());
    }
    Ok(k)
}

/// Runs SQL on the store behind the kernel's back.
fn raw(home: &Path, sql: &str) -> TestResult {
    let raw = rikuri_storage::rusqlite::Connection::open(database_path(home))?;
    raw.execute_batch(sql)?;
    Ok(())
}

/// Rewrites the body of every object of `kind` with `edit` (a SQLite
/// json_set argument list).
fn tamper(home: &Path, kind: &str, edit: &str) -> TestResult {
    raw(
        home,
        &format!("UPDATE objects SET body = json_set(body, {edit}) WHERE kind = '{kind}'"),
    )
}

fn tamper_runs(home: &Path, edit: &str) -> TestResult {
    tamper(home, "run", edit)
}

/// A well-formed ID that names nothing in the store.
fn unknown_run() -> TestResult<String> {
    Ok(json!(RunId::generate(&SystemIdSource::new())?)
        .as_str()
        .ok_or("RunId is not a string")?
        .to_owned())
}

/// init → two tasks, each started; returns the kernel and both start
/// results.
fn two_started_runs(home: &Path) -> TestResult<(Kernel, Value, Value)> {
    let k = open(home)?;
    k.handle(&Request::new(command::SYSTEM_INIT, json!({})));
    let mut started = vec![];
    for i in 0..2 {
        let created = result(&keyed(
            &k,
            command::TASK_CREATE,
            json!({"title": format!("t{i}")}),
            &format!("key-create-{i:04}"),
        ));
        let r = keyed(
            &k,
            command::TASK_START,
            json!({"task_id": created["task_id"], "expected_version": created["version"]}),
            &format!("key-start-{i:05}"),
        );
        if r.error.is_some() {
            return Err(format!("start failed: {r:?}").into());
        }
        started.push(result(&r));
    }
    let b = started.pop().ok_or("no second task")?;
    let a = started.pop().ok_or("no first task")?;
    Ok((k, a, b))
}

/// Sets `field` of task `task` to the ID of task `other`'s run.
fn point_task_at_other_run(home: &Path, task: &Value, field: &str, other: &Value) -> TestResult {
    let (task, run) = (
        task["task_id"].as_str().ok_or("no task_id")?,
        other["run_id"].as_str().ok_or("no run_id")?,
    );
    raw(
        home,
        &format!(
            "UPDATE objects SET body = json_set(body, '$.{field}', '{run}') \
             WHERE kind = 'task' AND json_extract(body, '$.id') = '{task}'"
        ),
    )
}

/// Parses a wire ID out of a JSON value.
fn id<T: serde::de::DeserializeOwned>(v: &Value) -> TestResult<T> {
    Ok(serde_json::from_value(v.clone())?)
}

/// Appends an event through the ledger's own API, as a regressed kernel
/// would: a genuine, well-formed event that state does not account for.
fn append_stray(
    k: &Kernel,
    kind: &str,
    key: &str,
    about: (Option<TaskId>, Option<RunId>),
    caused_by: Option<EventId>,
    metadata: Value,
) -> TestResult {
    let s = result(&k.handle(&Request::new(command::STATE_SHOW, json!({}))));
    let e = NewEvent {
        event_id: EventId::generate(&SystemIdSource::new())?,
        event_type: EventType::new(kind)?,
        status: EventStatus::Succeeded,
        risk: RiskClass::NovelModification,
        actor_id: id::<ActorId>(&s["owner"]["actor_id"])?,
        project_id: Some(id::<ProjectId>(&s["owner"]["project_id"])?),
        task_id: about.0,
        run_id: about.1,
        subject_ids: vec![],
        input_refs: vec![],
        result_refs: vec![],
        security_decision_id: None,
        cost_record_id: None,
        idempotency_key: Some(IdempotencyKey::new(key)?),
        caused_by,
        metadata: metadata.as_object().cloned().unwrap_or_default(),
    };
    k.store()
        .write(|tx| rikuri_ledger::append(tx, &e, k.store().now()))?;
    Ok(())
}

/// The completed run's task, run and completion event IDs.
fn completed_ids(k: &Kernel) -> TestResult<(TaskId, RunId, EventId)> {
    let s = result(&k.handle(&Request::new(command::STATE_SHOW, json!({}))));
    let run = &s["tasks"][0]["last_run"];
    Ok((
        id(&run["task_id"])?,
        id(&run["id"])?,
        id(&run["completion_event_id"])?,
    ))
}

fn reported(k: &Kernel, needles: &[&str]) -> TestResult {
    let problems = k.check_consistency().problems;
    if problems
        .iter()
        .any(|p| needles.iter().all(|n| p.contains(n)))
    {
        Ok(())
    } else {
        Err(format!("no problem mentions all of {needles:?}: {problems:?}").into())
    }
}

#[test]
fn a_consistent_lifecycle_has_no_problems() {
    let dir = tempfile::tempdir().unwrap();
    let k = completed_run(&dir.path().join("home")).unwrap();
    let report = k.check_consistency();
    assert!(report.problems.is_empty(), "{:?}", report.problems);
}

/// State says the run is still running, but the ledger has its completion
/// and verification: the replay must say so.
#[test]
fn completion_events_that_state_does_not_show_are_reported() {
    let dir = tempfile::tempdir().unwrap();
    let home = dir.path().join("home");
    let k = completed_run(&home).unwrap();
    tamper_runs(
        &home,
        "'$.status', 'running', '$.outcome', NULL, '$.verification', NULL, \
         '$.completion_event_id', NULL, '$.verification_event_id', NULL",
    )
    .unwrap();
    let report = k.check_consistency();
    assert!(
        report.problems.iter().any(|p| p.contains("run.completed")),
        "an unlinked completion went unnoticed: {:?}",
        report.problems
    );
}

/// The run's recorded outcome and verification must match what the ledger
/// recorded when it completed.
#[test]
fn an_outcome_that_differs_from_the_ledger_is_reported() {
    let dir = tempfile::tempdir().unwrap();
    let home = dir.path().join("home");
    let k = completed_run(&home).unwrap();
    tamper_runs(&home, "'$.outcome', 'failed'").unwrap();
    let report = k.check_consistency();
    assert!(
        report.problems.iter().any(|p| p.contains("outcome")),
        "a changed outcome went unnoticed: {:?}",
        report.problems
    );

    let dir = tempfile::tempdir().unwrap();
    let home = dir.path().join("home");
    let k = completed_run(&home).unwrap();
    tamper_runs(&home, "'$.verification', 'failed'").unwrap();
    let report = k.check_consistency();
    assert!(
        report.problems.iter().any(|p| p.contains("verification")),
        "a changed verification went unnoticed: {:?}",
        report.problems
    );
}

/// The per-run step count must match the ledger's `task.stepped` events
/// for that run (the task's total alone cannot see steps moved between runs).
#[test]
fn a_run_step_count_that_differs_from_the_ledger_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    tamper_runs(&home, "'$.steps', 2")?;
    reported(&k, &["records 2 steps", "task.stepped events for it"])
}

/// A run object that vanished (a bad restore or migration) while the
/// ledger still has its events.
#[test]
fn run_events_for_a_run_that_does_not_exist_are_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    raw(&home, "DELETE FROM objects WHERE kind = 'run'")?;
    reported(&k, &["run.started event", "which does not exist"])
}

/// The run's start link points elsewhere: its own run.started event is
/// then unlinked.
#[test]
fn an_unlinked_run_started_event_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    tamper_runs(
        &home,
        "'$.started_event_id', json_extract(body, '$.completion_event_id')",
    )?;
    reported(&k, &["does not link its run.started event"])
}

/// A running run that its task no longer shows as current: the task looks
/// finished while the run never ended.
#[test]
fn a_running_run_its_task_does_not_point_at_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let (k, _) = started_run(&home)?;
    tamper(
        &home,
        "task",
        "'$.status', 'completed', '$.current_run', NULL",
    )?;
    reported(&k, &["is running", "current run"])
}

/// A task status that contradicts its current-run pointer.
#[test]
fn a_task_status_that_contradicts_its_current_run_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    tamper(&home, "task", "'$.status', 'running'")?;
    reported(&k, &["is running but has no current run"])
}

/// Task pointers to runs that do not exist.
#[test]
fn a_task_pointing_at_a_missing_run_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let (k, _) = started_run(&home)?;
    let missing = unknown_run()?;
    tamper(&home, "task", &format!("'$.current_run', '{missing}'"))?;
    reported(&k, &["current run", &missing, "does not exist"])?;

    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    tamper(&home, "task", &format!("'$.last_run', '{missing}'"))?;
    reported(&k, &["last run", &missing, "does not exist"])
}

/// The owner and project must be the ones `system.initialized` recorded.
#[test]
fn an_owner_that_disagrees_with_its_initialization_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = open(&home)?;
    k.handle(&Request::new(command::SYSTEM_INIT, json!({})));
    let other = json!(ProjectId::generate(&SystemIdSource::new())?);
    let other = other.as_str().ok_or("ProjectId is not a string")?;
    tamper(&home, "actor", &format!("'$.project_id', '{other}'"))?;
    reported(&k, &["system.initialized"])?;

    // the owner vanished, the ledger still says it was initialized
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = open(&home)?;
    k.handle(&Request::new(command::SYSTEM_INIT, json!({})));
    raw(&home, "DELETE FROM objects WHERE kind = 'actor'")?;
    reported(&k, &["system.initialized", "no owner"])
}

/// Legitimate use: several runs per task, failed and cancelled outcomes,
/// steps in different runs, a run left running and a task never started.
/// The stricter checks must not report anything here.
#[test]
fn legitimate_multi_run_histories_have_no_problems() -> TestResult {
    let dir = tempfile::tempdir()?;
    let k = open(&dir.path().join("home"))?;
    k.handle(&Request::new(command::SYSTEM_INIT, json!({})));
    let mut n = 0;
    let mut send = |command: &str, params: Value| -> TestResult<Value> {
        n += 1;
        let r = keyed(&k, command, params, &format!("key-multi-{n:06}"));
        if r.error.is_some() {
            return Err(format!("{command} failed: {r:?}").into());
        }
        Ok(result(&r))
    };
    let mut t = send(command::TASK_CREATE, json!({"title": "many runs"}))?;
    send(command::TASK_CREATE, json!({"title": "never started"}))?;
    for (steps, outcome, verification) in [
        (2, "failed", "failed"),
        (0, "cancelled", "not_run"),
        (1, "succeeded", "passed"),
    ] {
        t = send(
            command::TASK_START,
            json!({"task_id": t["task_id"], "expected_version": t["version"]}),
        )?;
        for _ in 0..steps {
            t = send(
                command::TASK_STEP,
                json!({"task_id": t["task_id"], "run_id": t["run_id"],
                       "expected_version": t["version"]}),
            )?;
        }
        t = send(
            command::TASK_COMPLETE,
            json!({"task_id": t["task_id"], "run_id": t["run_id"],
                   "expected_version": t["version"],
                   "outcome": outcome, "verification": verification}),
        )?;
    }
    let t = send(
        command::TASK_START,
        json!({"task_id": t["task_id"], "expected_version": t["version"]}),
    )?;
    send(
        command::TASK_STEP,
        json!({"task_id": t["task_id"], "run_id": t["run_id"],
               "expected_version": t["version"]}),
    )?;
    let problems = k.check_consistency().problems;
    assert!(problems.is_empty(), "{problems:?}");
    Ok(())
}

// One test per branch of the task ↔ run and owner checks (verifier round 2:
// six branches could be deleted with every test still green, because the
// tests above only tamper in ways other branches also see).

/// A task that says it is finished while it still points at a running run.
#[test]
fn a_finished_task_with_a_current_run_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let (k, _) = started_run(&home)?;
    tamper(&home, "task", "'$.status', 'completed'")?;
    reported(&k, &["is completed but has current run"])
}

/// A running task whose current run is its own run, already completed.
#[test]
fn a_current_run_that_is_not_running_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    tamper(
        &home,
        "task",
        "'$.status', 'running', '$.current_run', json_extract(body, '$.last_run')",
    )?;
    reported(&k, &["current run", "is not a running run of that task"])
}

/// A task whose current run is another task's running run.
#[test]
fn a_current_run_of_another_task_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let (k, a, b) = two_started_runs(&home)?;
    point_task_at_other_run(&home, &a, "current_run", &b)?;
    reported(&k, &["current run", "is not a running run of that task"])
}

/// A task whose last run belongs to another task.
#[test]
fn a_last_run_of_another_task_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let (k, a, b) = two_started_runs(&home)?;
    point_task_at_other_run(&home, &a, "last_run", &b)?;
    reported(&k, &["last run", "belongs to another task"])
}

/// A task that records runs but has no last run.
#[test]
fn a_task_with_runs_but_no_last_run_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    tamper(&home, "task", "'$.last_run', NULL")?;
    reported(&k, &["records 1 runs but no last run"])
}

/// A run whose task vanished.
#[test]
fn a_run_of_a_missing_task_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let (k, _) = started_run(&home)?;
    raw(&home, "DELETE FROM objects WHERE kind = 'task'")?;
    reported(&k, &["belongs to task", "which does not exist"])
}

/// The owner's project vanished.
#[test]
fn a_missing_owner_project_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = open(&home)?;
    k.handle(&Request::new(command::SYSTEM_INIT, json!({})));
    raw(&home, "DELETE FROM objects WHERE kind = 'project'")?;
    reported(&k, &["the owner's project", "does not exist"])
}

/// Two owners, or an owner that the ledger never initialized.
#[test]
fn owners_without_exactly_one_initialization_are_reported() -> TestResult {
    let other = json!(rikuri_domain::ActorId::generate(&SystemIdSource::new())?);
    let other = other.as_str().ok_or("ActorId is not a string")?;

    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = open(&home)?;
    k.handle(&Request::new(command::SYSTEM_INIT, json!({})));
    raw(
        &home,
        &format!(
            "INSERT INTO objects SELECT '{other}', kind, version, schema_version, privacy_class, \
             deletion_unit_id, created_at_ms, updated_at_ms, json_set(body, '$.id', '{other}') \
             FROM objects WHERE kind = 'actor'"
        ),
    )?;
    reported(&k, &["2 owner(s) and 1 system.initialized event(s)"])?;

    // an owner and project copied into a home whose ledger never initialized
    let initialized = dir.path().join("initialized");
    open(&initialized)?.handle(&Request::new(command::SYSTEM_INIT, json!({})));
    let blank = dir.path().join("blank");
    let k = open(&blank)?;
    let source = database_path(&initialized);
    raw(
        &blank,
        &format!(
            "ATTACH DATABASE '{}' AS source; \
             INSERT INTO objects SELECT * FROM source.objects WHERE kind IN ('actor', 'project');",
            source.display()
        ),
    )?;
    reported(&k, &["1 owner(s) and 0 system.initialized event(s)"])
}

// Round 3 (verifier): comparisons that no test isolated. Each tamper below
// is seen by exactly one comparison, so deleting that comparison fails here.

/// A second completion of an already completed run, under another key and
/// with the same outcome (a regressed kernel completing twice): only the
/// completion-link comparison sees it.
#[test]
fn a_second_completion_with_the_same_outcome_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let k = completed_run(&dir.path().join("home"))?;
    let (task, run, _) = completed_ids(&k)?;
    append_stray(
        &k,
        "run.completed",
        "key-stray-completed",
        (Some(task), Some(run)),
        None,
        json!({"outcome": "succeeded"}),
    )?;
    reported(&k, &["does not link its run.completed event"])
}

/// A second verification of an already verified run, same result.
#[test]
fn a_second_verification_with_the_same_result_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let k = completed_run(&dir.path().join("home"))?;
    let (task, run, completion) = completed_ids(&k)?;
    append_stray(
        &k,
        "run.verified",
        "key-stray-verified",
        (Some(task), Some(run)),
        Some(completion),
        json!({"verification": "passed"}),
    )?;
    reported(&k, &["does not link its run.verified event"])
}

/// The owner replaced by another actor with the same project (a bad
/// restore): only the actor comparison sees it.
#[test]
fn an_owner_replaced_by_another_actor_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = open(&home)?;
    k.handle(&Request::new(command::SYSTEM_INIT, json!({})));
    let other = json!(ActorId::generate(&SystemIdSource::new())?);
    let other = other.as_str().ok_or("ActorId is not a string")?;
    raw(
        &home,
        &format!(
            "UPDATE objects SET id = '{other}', body = json_set(body, '$.id', '{other}') \
             WHERE kind = 'actor'"
        ),
    )?;
    reported(&k, &["is not the one its system.initialized event"])
}

/// A task that was never started but points at a run.
#[test]
fn a_created_task_with_a_current_run_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let (k, _) = started_run(&home)?;
    tamper(&home, "task", "'$.status', 'created'")?;
    reported(&k, &["is created but has current run"])
}

/// One owner, but the ledger initialized the home twice.
#[test]
fn a_second_initialization_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let k = open(&dir.path().join("home"))?;
    k.handle(&Request::new(command::SYSTEM_INIT, json!({})));
    append_stray(
        &k,
        "system.initialized",
        "key-stray-init",
        (None, None),
        None,
        json!({}),
    )?;
    reported(&k, &["1 owner(s) and 2 system.initialized event(s)"])
}

// The checks S00.07 wrote (per-task counts, run → ledger links) had never
// been mutated; once the event → run direction existed, most of their
// states were also seen by another check, so deleting one left every test
// green. Each test below builds a state only that comparison sees, or
// asserts on its own message.

/// Inserts a copy of a run (of `task`, if given) under a new ID, with
/// `edit` (more json_set pairs, each starting with a comma) applied to its
/// body: a run object that has no history of its own (a bad migration).
fn phantom_run(home: &Path, task: Option<&Value>, edit: &str) -> TestResult<String> {
    let phantom = unknown_run()?;
    let filter = match task {
        Some(t) => format!(
            " AND json_extract(body, '$.task_id') = '{}'",
            t.as_str().ok_or("task_id is not a string")?
        ),
        None => String::new(),
    };
    raw(
        home,
        &format!(
            "INSERT INTO objects SELECT '{phantom}', kind, version, schema_version, \
             privacy_class, deletion_unit_id, created_at_ms, updated_at_ms, \
             json_set(body, '$.id', '{phantom}'{edit}) FROM objects WHERE kind = 'run'{filter} \
             LIMIT 1"
        ),
    )?;
    Ok(phantom)
}

/// The ID of the most recently appended event.
fn last_event_id(k: &Kernel) -> TestResult<String> {
    let tail = result(&k.handle(&Request::new(command::LEDGER_TAIL, json!({"limit": 1}))));
    Ok(tail["events"][0]["event_id"]
        .as_str()
        .ok_or("no event")?
        .to_owned())
}

/// A second `task.created` for the same task under another key.
#[test]
fn a_second_task_created_event_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let k = completed_run(&dir.path().join("home"))?;
    let (task, _, _) = completed_ids(&k)?;
    append_stray(
        &k,
        "task.created",
        "key-stray-created",
        (Some(task), None),
        None,
        json!({}),
    )?;
    reported(&k, &["has 2 task.created events, expected 1"])
}

/// The task's own step count differs from its `task.stepped` events.
#[test]
fn a_task_step_count_that_differs_from_the_ledger_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    tamper(&home, "task", "'$.steps_completed', 5")?;
    reported(
        &k,
        &["records 5 steps but the ledger has 0 task.stepped events"],
    )
}

/// The task's run count differs from its `run.started` events.
#[test]
fn a_task_run_count_that_differs_from_the_ledger_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    tamper(&home, "task", "'$.runs', 2")?;
    reported(
        &k,
        &["records 2 runs but the ledger has 1 run.started events"],
    )
}

/// A never-started task that vanished while the ledger still has its
/// creation.
#[test]
fn ledger_events_for_a_task_that_does_not_exist_are_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = open(&home)?;
    k.handle(&Request::new(command::SYSTEM_INIT, json!({})));
    keyed(
        &k,
        command::TASK_CREATE,
        json!({"title": "t"}),
        "key-create-0001",
    );
    raw(&home, "DELETE FROM objects WHERE kind = 'task'")?;
    reported(&k, &["ledger references task", "which does not exist"])
}

/// A run whose links name events the ledger does not have.
#[test]
fn a_run_whose_links_are_not_in_the_ledger_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    let missing = json!(EventId::generate(&SystemIdSource::new())?);
    let missing = missing.as_str().ok_or("EventId is not a string")?;
    let phantom = phantom_run(&home, None, &format!(", '$.started_event_id', '{missing}'"))?;
    reported(&k, &[&phantom, missing, "which is not in the ledger"])
}

/// A run that links another run's events (right type, right task).
#[test]
fn a_run_linking_another_runs_events_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    let phantom = phantom_run(&home, None, "")?;
    reported(&k, &[&phantom, "which is not its run.started event"])
}

/// A run whose start link is an event about it, but of another type.
#[test]
fn a_run_linking_an_event_of_the_wrong_type_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    let (task, _, _) = completed_ids(&k)?;
    let phantom = phantom_run(
        &home,
        None,
        ", '$.status', 'running', '$.completion_event_id', NULL, \
         '$.verification_event_id', NULL, '$.outcome', NULL, '$.verification', NULL",
    )?;
    append_stray(
        &k,
        "task.stepped",
        "key-stray-stepped",
        (Some(task), Some(id(&json!(phantom))?)),
        None,
        json!({}),
    )?;
    let stray = last_event_id(&k)?;
    raw(
        &home,
        &format!(
            "UPDATE objects SET body = json_set(body, '$.started_event_id', '{stray}') \
             WHERE id = '{phantom}'"
        ),
    )?;
    reported(&k, &[&stray, "which is not its run.started event"])
}

/// A run whose start link is a `run.started` about it, but recorded for
/// another task.
#[test]
fn a_run_linking_an_event_about_another_task_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let (k, a, b) = two_started_runs(&home)?;
    let phantom = phantom_run(&home, Some(&a["task_id"]), "")?;
    append_stray(
        &k,
        "run.started",
        "key-stray-started",
        (Some(id(&b["task_id"])?), Some(id(&json!(phantom))?)),
        None,
        json!({}),
    )?;
    let stray = last_event_id(&k)?;
    raw(
        &home,
        &format!(
            "UPDATE objects SET body = json_set(body, '$.started_event_id', '{stray}') \
             WHERE id = '{phantom}'"
        ),
    )?;
    reported(&k, &[&stray, "which is not its run.started event"])
}

/// A running run that already carries a verification link (and no
/// completion link).
#[test]
fn a_running_run_with_completion_links_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    let phantom = phantom_run(
        &home,
        None,
        ", '$.status', 'running', '$.completion_event_id', NULL",
    )?;
    reported(
        &k,
        &[&format!("running run {phantom} has completion links")],
    )
}

/// A completed run without its completion link.
#[test]
fn a_completed_run_without_its_links_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    let phantom = phantom_run(&home, None, ", '$.completion_event_id', NULL")?;
    reported(
        &k,
        &[&format!(
            "completed run {phantom} is missing a completion or verification link"
        )],
    )
}

/// A run whose verification is not caused by its completion.
#[test]
fn a_verification_that_does_not_follow_its_completion_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    let (task, run, _) = completed_ids(&k)?;
    append_stray(
        &k,
        "run.verified",
        "key-stray-verified",
        (Some(task), Some(run)),
        None,
        json!({"verification": "passed"}),
    )?;
    let stray = last_event_id(&k)?;
    tamper_runs(&home, &format!("'$.verification_event_id', '{stray}'"))?;
    reported(&k, &["verification does not follow its completion"])
}

// Round 4 (verifier): the state → ledger link comparisons for completion and
// verification, the "could not run" path, and the other side of conditions
// whose one side was tested.

/// Sets fields of the run `run` only (json_set pairs, no leading comma).
fn tamper_run(home: &Path, run: &str, edit: &str) -> TestResult {
    raw(
        home,
        &format!("UPDATE objects SET body = json_set(body, {edit}) WHERE id = '{run}'"),
    )
}

/// A regressed `task.complete` that writes its completion with the wrong
/// type: the run links it, and no `run.completed` exists for the ledger
/// side to notice.
#[test]
fn a_completion_link_to_an_event_of_another_type_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    let (task, _, _) = completed_ids(&k)?;
    let phantom = phantom_run(&home, None, "")?;
    let about = (Some(task), Some(id(&json!(phantom))?));
    append_stray(
        &k,
        "task.completed",
        "key-stray-completed",
        about,
        None,
        json!({}),
    )?;
    let completed = last_event_id(&k)?;
    append_stray(
        &k,
        "run.verified",
        "key-stray-verified",
        about,
        Some(id(&json!(completed))?),
        json!({"verification": "passed"}),
    )?;
    let verified = last_event_id(&k)?;
    tamper_run(
        &home,
        &phantom,
        &format!("'$.completion_event_id', '{completed}', '$.verification_event_id', '{verified}'"),
    )?;
    reported(&k, &[&completed, "which is not its run.completed event"])
}

/// The same for the verification.
#[test]
fn a_verification_link_to_an_event_of_another_type_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    let (task, _, _) = completed_ids(&k)?;
    let phantom = phantom_run(&home, None, "")?;
    let about = (Some(task), Some(id(&json!(phantom))?));
    append_stray(
        &k,
        "run.completed",
        "key-stray-completed",
        about,
        None,
        json!({"outcome": "succeeded"}),
    )?;
    let completed = last_event_id(&k)?;
    append_stray(
        &k,
        "task.verified",
        "key-stray-verified",
        about,
        Some(id(&json!(completed))?),
        json!({}),
    )?;
    let verified = last_event_id(&k)?;
    tamper_run(
        &home,
        &phantom,
        &format!("'$.completion_event_id', '{completed}', '$.verification_event_id', '{verified}'"),
    )?;
    reported(&k, &[&verified, "which is not its run.verified event"])
}

/// An undecodable ledger row (disk corruption, a foreign writer, a
/// downgrade): the check cannot run, and that is a problem, never a pass.
#[test]
fn a_ledger_the_check_cannot_read_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    raw(
        &home,
        "INSERT INTO events (event_id, schema_version, recorded_at_ms, event_type, status, \
         actor_id, project_id, task_id, run_id, subject_ids, input_refs, result_refs, \
         security_decision_id, cost_record_id, idempotency_key, caused_by_event_id, metadata) \
         SELECT 'evt_not-a-ulid', schema_version, recorded_at_ms, event_type, status, actor_id, \
         project_id, task_id, run_id, subject_ids, input_refs, result_refs, \
         security_decision_id, cost_record_id, NULL, NULL, metadata \
         FROM events ORDER BY seq DESC LIMIT 1",
    )?;
    reported(&k, &["consistency check could not run"])?;
    assert!(k.quarantined().is_some(), "writes must stop");
    Ok(())
}

/// A task in state that the ledger never created (a restore or migration).
#[test]
fn a_task_the_ledger_never_created_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = open(&home)?;
    k.handle(&Request::new(command::SYSTEM_INIT, json!({})));
    keyed(
        &k,
        command::TASK_CREATE,
        json!({"title": "t"}),
        "key-create-0001",
    );
    let phantom = json!(TaskId::generate(&SystemIdSource::new())?);
    let phantom = phantom.as_str().ok_or("TaskId is not a string")?;
    raw(
        &home,
        &format!(
            "INSERT INTO objects SELECT '{phantom}', kind, version, schema_version, \
             privacy_class, deletion_unit_id, created_at_ms, updated_at_ms, \
             json_set(body, '$.id', '{phantom}') FROM objects WHERE kind = 'task'"
        ),
    )?;
    reported(&k, &[phantom, "has 0 task.created events, expected 1"])
}

/// A completed run whose outcome or verification a migration dropped.
#[test]
fn a_dropped_outcome_or_verification_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    tamper_runs(&home, "'$.outcome', NULL")?;
    reported(&k, &["outcome is None", "recorded Some(\"succeeded\")"])?;

    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    tamper_runs(&home, "'$.verification', NULL")?;
    reported(&k, &["verification is None", "recorded Some(\"passed\")"])
}

/// A running run carrying a dangling completion link (a half-written
/// completion), without a verification link.
#[test]
fn a_running_run_with_a_completion_link_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    let phantom = phantom_run(
        &home,
        None,
        ", '$.status', 'running', '$.verification_event_id', NULL",
    )?;
    reported(
        &k,
        &[&format!("running run {phantom} has completion links")],
    )
}

/// A regressed `task.complete` that records the completion but never the
/// verification.
#[test]
fn a_completed_run_without_its_verification_link_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    let phantom = phantom_run(&home, None, ", '$.verification_event_id', NULL")?;
    reported(
        &k,
        &[&format!(
            "completed run {phantom} is missing a completion or verification link"
        )],
    )
}

/// A verification caused by the run's start rather than its completion.
#[test]
fn a_verification_caused_by_the_start_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    let (task, run, _) = completed_ids(&k)?;
    let s = result(&k.handle(&Request::new(command::STATE_SHOW, json!({}))));
    let started: EventId = id(&s["tasks"][0]["last_run"]["started_event_id"])?;
    append_stray(
        &k,
        "run.verified",
        "key-stray-verified",
        (Some(task), Some(run)),
        Some(started),
        json!({"verification": "passed"}),
    )?;
    let stray = last_event_id(&k)?;
    tamper_runs(&home, &format!("'$.verification_event_id', '{stray}'"))?;
    reported(&k, &["verification does not follow its completion"])
}

/// Events a regressed kernel writes without one of their IDs: each count is
/// then seen by only one side (per task, or per run).
#[test]
fn a_step_or_start_missing_one_id_is_reported() -> TestResult {
    // a step about the task but no run: only the task's step count sees it
    let dir = tempfile::tempdir()?;
    let k = completed_run(&dir.path().join("home"))?;
    let (task, _, _) = completed_ids(&k)?;
    append_stray(
        &k,
        "task.stepped",
        "key-stray-1",
        (Some(task), None),
        None,
        json!({}),
    )?;
    reported(
        &k,
        &["records 0 steps but the ledger has 1 task.stepped events"],
    )?;

    // a start about the task but no run: only the task's run count sees it
    let dir = tempfile::tempdir()?;
    let k = completed_run(&dir.path().join("home"))?;
    let (task, _, _) = completed_ids(&k)?;
    append_stray(
        &k,
        "run.started",
        "key-stray-2",
        (Some(task), None),
        None,
        json!({}),
    )?;
    reported(
        &k,
        &["records 1 runs but the ledger has 2 run.started events"],
    )?;

    // a step about the run but no task: only the run's step count sees it
    let dir = tempfile::tempdir()?;
    let k = completed_run(&dir.path().join("home"))?;
    let (_, run, _) = completed_ids(&k)?;
    append_stray(
        &k,
        "task.stepped",
        "key-stray-3",
        (None, Some(run)),
        None,
        json!({}),
    )?;
    reported(
        &k,
        &["records 0 steps but the ledger has 1 task.stepped events for it"],
    )
}

// Round 5 (verifier): the completion and verification links each need their
// own missing / another run / another task case; only the start link had
// all of them. Scenario: a regressed `task.complete` for a task's second run
// reuses run 1's `run.completed` instead of appending its own.

/// The run's completion link: missing from the ledger, another run's, or
/// about another task.
#[test]
fn completion_links_that_are_missing_or_not_this_runs_are_reported() -> TestResult {
    // missing from the ledger
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    let missing = json!(EventId::generate(&SystemIdSource::new())?);
    let missing = missing.as_str().ok_or("EventId is not a string")?;
    phantom_run(
        &home,
        None,
        &format!(", '$.completion_event_id', '{missing}'"),
    )?;
    reported(
        &k,
        &[missing, "run.completed", "which is not in the ledger"],
    )?;

    // another run's (run 1's completion reused by a second run)
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    let (_, _, completion) = completed_ids(&k)?;
    phantom_run(&home, None, "")?;
    let completion = json!(completion);
    let completion = completion.as_str().ok_or("EventId is not a string")?;
    reported(&k, &[completion, "which is not its run.completed event"])?;

    // about another task
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let (k, a, b) = two_started_runs(&home)?;
    let phantom = phantom_run(&home, Some(&a["task_id"]), ", '$.status', 'completed'")?;
    let run: RunId = id(&json!(phantom))?;
    append_stray(
        &k,
        "run.completed",
        "key-stray-completed",
        (Some(id(&b["task_id"])?), Some(run)),
        None,
        json!({"outcome": "succeeded"}),
    )?;
    let completed = last_event_id(&k)?;
    append_stray(
        &k,
        "run.verified",
        "key-stray-verified",
        (Some(id(&a["task_id"])?), Some(run)),
        Some(id(&json!(completed))?),
        json!({"verification": "passed"}),
    )?;
    let verified = last_event_id(&k)?;
    tamper_run(
        &home,
        &phantom,
        &format!("'$.completion_event_id', '{completed}', '$.verification_event_id', '{verified}'"),
    )?;
    reported(&k, &[&completed, "which is not its run.completed event"])
}

/// The run's verification link: missing from the ledger, another run's, or
/// about another task.
#[test]
fn verification_links_that_are_missing_or_not_this_runs_are_reported() -> TestResult {
    // missing from the ledger
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    let missing = json!(EventId::generate(&SystemIdSource::new())?);
    let missing = missing.as_str().ok_or("EventId is not a string")?;
    phantom_run(
        &home,
        None,
        &format!(", '$.verification_event_id', '{missing}'"),
    )?;
    reported(&k, &[missing, "run.verified", "which is not in the ledger"])?;

    // another run's
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    let s = result(&k.handle(&Request::new(command::STATE_SHOW, json!({}))));
    let verification = s["tasks"][0]["last_run"]["verification_event_id"]
        .as_str()
        .ok_or("no verification link")?
        .to_owned();
    phantom_run(&home, None, "")?;
    reported(&k, &[&verification, "which is not its run.verified event"])?;

    // about another task
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let (k, a, b) = two_started_runs(&home)?;
    let phantom = phantom_run(&home, Some(&a["task_id"]), ", '$.status', 'completed'")?;
    let run: RunId = id(&json!(phantom))?;
    append_stray(
        &k,
        "run.completed",
        "key-stray-completed",
        (Some(id(&a["task_id"])?), Some(run)),
        None,
        json!({"outcome": "succeeded"}),
    )?;
    let completed = last_event_id(&k)?;
    append_stray(
        &k,
        "run.verified",
        "key-stray-verified",
        (Some(id(&b["task_id"])?), Some(run)),
        Some(id(&json!(completed))?),
        json!({"verification": "passed"}),
    )?;
    let verified = last_event_id(&k)?;
    tamper_run(
        &home,
        &phantom,
        &format!("'$.completion_event_id', '{completed}', '$.verification_event_id', '{verified}'"),
    )?;
    reported(&k, &[&verified, "which is not its run.verified event"])
}

// Round 6 (verifier, Minor): the duplicate-effect check was pinned only
// for `task.created`, and the owner read's "could not run" path untested.

/// A second event of each type under the same idempotency key (a
/// consistent double-apply, where state may count it twice too) is a
/// duplicate effect, whatever the event type.
#[test]
fn a_duplicate_of_any_event_type_under_one_key_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let (k, started) = started_run(&dir.path().join("home"))?;
    let stepped = result(&keyed(
        &k,
        command::TASK_STEP,
        json!({"task_id": started["task_id"], "run_id": started["run_id"],
               "expected_version": started["version"]}),
        "key-step-000001",
    ));
    let done = keyed(
        &k,
        command::TASK_COMPLETE,
        json!({"task_id": stepped["task_id"], "run_id": stepped["run_id"],
               "expected_version": stepped["version"],
               "outcome": "succeeded", "verification": "passed"}),
        "key-done-000001",
    );
    if done.error.is_some() {
        return Err(format!("complete failed: {done:?}").into());
    }
    let tail = result(&k.handle(&Request::new(command::LEDGER_TAIL, json!({"limit": 1000}))));
    let events = tail["events"].as_array().ok_or("no events")?.clone();
    let mut seen = vec![];
    for e in &events {
        let kind = e["event_type"].as_str().ok_or("no event_type")?;
        let key = e["idempotency_key"].as_str().ok_or("no key")?;
        let caused_by = match e.get("caused_by") {
            Some(c) => Some(id::<EventId>(c)?),
            None => None,
        };
        let about = (
            e.get("task_id").map(id::<TaskId>).transpose()?,
            e.get("run_id").map(id::<RunId>).transpose()?,
        );
        append_stray(&k, kind, key, about, caused_by, e["metadata"].clone())?;
        seen.push((kind.to_owned(), key.to_owned()));
    }
    let kinds: Vec<&str> = seen.iter().map(|(k, _)| k.as_str()).collect();
    for kind in [
        "system.initialized",
        "task.created",
        "run.started",
        "task.stepped",
        "run.completed",
        "run.verified",
    ] {
        assert!(
            kinds.contains(&kind),
            "the lifecycle has no {kind}: {kinds:?}"
        );
    }
    for (kind, key) in &seen {
        reported(
            &k,
            &[&format!(
                "duplicate effect: two {kind} events carry idempotency key {key}"
            )],
        )?;
    }
    Ok(())
}

/// An owner row that no longer decodes: the check cannot run, which is
/// reported (and quarantines), never a pass.
#[test]
fn an_owner_the_check_cannot_read_is_reported() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    tamper(&home, "actor", "'$.project_id', 'not-an-id'")?;
    reported(&k, &["consistency check could not run"])?;
    assert!(k.quarantined().is_some(), "writes must stop");
    Ok(())
}
