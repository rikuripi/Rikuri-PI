//! Transactional State Evolution's "validate candidate" step (Architecture
//! §43; E1, ADR 0005 and its 2026-10-10 clarification): before a
//! consequential command commits, what it wrote is checked against the
//! events it appended and against history, and a candidate that disagrees
//! is refused while the last known-good state is still authoritative. Without it a kernel defect's candidate was
//! promoted, caught only afterwards, and the daemon quarantined itself
//! (E1 experiment 1).
//!
//! Each test builds a store whose next command's candidate disagrees with
//! history through one comparison, then checks the command is refused,
//! nothing is promoted, and no quarantine was needed.
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

fn ok(r: Response) -> TestResult<Value> {
    match (r.error, r.result) {
        (None, Some(v)) => Ok(v),
        (e, _) => Err(format!("refused: {e:?}").into()),
    }
}

fn state(k: &Kernel) -> Value {
    k.handle(&Request::new(command::STATE_SHOW, json!({})))
        .result
        .unwrap_or(Value::Null)
}

fn events(k: &Kernel) -> usize {
    k.handle(&Request::new(command::LEDGER_TAIL, json!({"limit": 1000})))
        .result
        .and_then(|r| r["events"].as_array().map(Vec::len))
        .unwrap_or(0)
}

fn raw(home: &Path, sql: &str) -> TestResult {
    let raw = rikuri_storage::rusqlite::Connection::open(database_path(home))?;
    raw.execute_batch(sql)?;
    Ok(())
}

fn tamper(home: &Path, kind: &str, edit: &str) -> TestResult {
    raw(
        home,
        &format!("UPDATE objects SET body = json_set(body, {edit}) WHERE kind = '{kind}'"),
    )
}

/// init → create → start; returns the kernel and the start result.
fn started(home: &Path, n: u32) -> TestResult<(Kernel, Value)> {
    let k = open(home)?;
    k.handle(&Request::new(command::SYSTEM_INIT, json!({})));
    let mut last = Value::Null;
    for i in 0..n {
        let t = ok(keyed(
            &k,
            command::TASK_CREATE,
            json!({"title": format!("t{i}")}),
            &format!("key-create-{i:04}"),
        ))?;
        last = ok(keyed(
            &k,
            command::TASK_START,
            json!({"task_id": t["task_id"], "expected_version": t["version"]}),
            &format!("key-start-{i:05}"),
        ))?;
    }
    Ok((k, last))
}

/// started, then completed.
fn completed(home: &Path) -> TestResult<(Kernel, Value)> {
    let (k, s) = started(home, 1)?;
    let done = ok(keyed(
        &k,
        command::TASK_COMPLETE,
        json!({"task_id": s["task_id"], "run_id": s["run_id"],
               "expected_version": s["version"],
               "outcome": "succeeded", "verification": "passed"}),
        "key-done-000001",
    ))?;
    Ok((k, done))
}

fn the_task(k: &Kernel) -> Value {
    state(k)["tasks"][0].clone()
}

/// The command is refused as a candidate that disagrees with history
/// (`needle` names the disagreement), and nothing it would have written was
/// promoted: same task, same events, no quarantine needed.
fn refused_unpromoted(k: &Kernel, r: Response, before: (Value, usize), needle: &str) -> TestResult {
    let e = r.error.ok_or("the command was promoted")?;
    if e.code.as_str() != "internal"
        || !e.message.contains("candidate refused before promotion")
        || !e.message.contains(needle)
    {
        return Err(format!("not refused as expected ({needle}): {e:?}").into());
    }
    if (the_task(k), events(k)) != before {
        return Err("the refused candidate changed state or history".into());
    }
    if k.quarantined().is_some() {
        return Err("a refused candidate must not need a quarantine".into());
    }
    Ok(())
}

fn step(k: &Kernel, t: &Value, key: &str) -> Response {
    keyed(
        k,
        command::TASK_STEP,
        json!({"task_id": t["task_id"], "run_id": t["current_run"],
               "expected_version": t["version"]}),
        key,
    )
}

fn start(k: &Kernel, t: &Value, key: &str) -> Response {
    keyed(
        k,
        command::TASK_START,
        json!({"task_id": t["task_id"], "expected_version": t["version"]}),
        key,
    )
}

/// The task's step count disagrees with its `task.stepped` events.
#[test]
fn a_step_whose_count_disagrees_with_history_is_not_promoted() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let (k, _) = started(&home, 1)?;
    tamper(&home, "task", "'$.steps_completed', 5")?;
    let t = the_task(&k);
    let before = (t.clone(), events(&k));
    refused_unpromoted(&k, step(&k, &t, "key-step-000001"), before, "steps")
}

/// The task's run count disagrees with its `run.started` events.
#[test]
fn a_start_whose_run_count_disagrees_is_not_promoted() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let (k, _) = completed(&home)?;
    tamper(&home, "task", "'$.runs', 3")?;
    let t = the_task(&k);
    let before = (t.clone(), events(&k));
    refused_unpromoted(&k, start(&k, &t, "key-start-00009"), before, "runs")
}

/// Drift in an earlier run the command does not write is not judged before
/// the commit (the check covers what a command writes, not all history;
/// owner decision 2026-10-10, ADR 0005 clarification). The full replay
/// finds it on the next `system.health` and quarantines writes.
#[test]
fn drift_in_an_earlier_run_is_left_to_the_replay() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let (k, _) = completed(&home)?;
    tamper(&home, "run", "'$.outcome', 'failed'")?;
    let t = the_task(&k);
    ok(start(&k, &t, "key-start-00009"))?;
    let health = k
        .handle(&Request::new(command::SYSTEM_HEALTH, json!({})))
        .result
        .ok_or("system.health failed")?;
    let reason = health["quarantined"].as_str().unwrap_or_default();
    if !reason.contains("outcome") {
        return Err(format!("the replay did not quarantine the drift: {health}").into());
    }
    Ok(())
}

/// The task's pointers to its runs disagree with their status.
#[test]
fn a_candidate_with_disagreeing_task_pointers_is_not_promoted() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let (k, _) = started(&home, 1)?;
    tamper(&home, "task", "'$.status', 'created'")?;
    let t = the_task(&k);
    let before = (t.clone(), events(&k));
    refused_unpromoted(&k, step(&k, &t, "key-step-000001"), before, "current run")
}

/// The ledger records the task's creation twice.
#[test]
fn a_candidate_for_a_task_created_twice_is_not_promoted() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let (k, s) = started(&home, 1)?;
    let st = state(&k);
    let owner: ActorId = serde_json::from_value(st["owner"]["actor_id"].clone())?;
    let project: ProjectId = serde_json::from_value(st["owner"]["project_id"].clone())?;
    let task: TaskId = serde_json::from_value(s["task_id"].clone())?;
    let stray = NewEvent {
        event_id: EventId::generate(&SystemIdSource::new())?,
        event_type: EventType::new("task.created")?,
        status: EventStatus::Succeeded,
        risk: RiskClass::NovelModification,
        actor_id: owner,
        project_id: Some(project),
        task_id: Some(task),
        run_id: None::<RunId>,
        subject_ids: vec![],
        input_refs: vec![],
        result_refs: vec![],
        security_decision_id: None,
        cost_record_id: None,
        idempotency_key: Some(IdempotencyKey::new("key-stray-created")?),
        caused_by: None,
        metadata: serde_json::Map::new(),
    };
    k.store()
        .write(|tx| rikuri_ledger::append(tx, &stray, k.store().now()))?;
    let t = the_task(&k);
    let before = (t.clone(), events(&k));
    refused_unpromoted(&k, step(&k, &t, "key-step-000001"), before, "task.created")
}

/// The check covers what a command writes: drift on another task is the
/// replay's to find (at open and on health), and does not stop this task's
/// commands.
#[test]
fn drift_on_another_task_does_not_block_this_one() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let (k, _) = started(&home, 2)?;
    let tasks = state(&k)["tasks"].as_array().cloned().unwrap_or_default();
    let (drifted, clean) = (&tasks[0], &tasks[1]);
    let id = drifted["task_id"].as_str().ok_or("no task id")?;
    raw(
        &home,
        &format!(
            "UPDATE objects SET body = json_set(body, '$.steps_completed', 7) WHERE id = '{id}'"
        ),
    )?;
    ok(step(&k, clean, "key-step-000001"))?;
    Ok(())
}

/// `system.init` (it writes the owner and project, which no write-set rule
/// covers) is checked against the whole store: initializing a store that
/// already holds a task with no history is refused, and the store stays
/// uninitialized.
#[test]
fn init_into_a_store_with_stray_state_is_not_promoted() -> TestResult {
    let dir = tempfile::tempdir()?;
    let source = dir.path().join("source");
    started(&source, 1)?;
    let home = dir.path().join("home");
    let k = open(&home)?;
    raw(
        &home,
        &format!(
            "ATTACH DATABASE '{}' AS source; \
             INSERT INTO objects SELECT * FROM source.objects WHERE kind = 'task';",
            database_path(&source).display()
        ),
    )?;
    let r = k.handle(&Request::new(command::SYSTEM_INIT, json!({})));
    let e = r.error.ok_or("init was promoted")?;
    assert_eq!(e.code.as_str(), "internal", "{e:?}");
    assert!(
        e.message.contains("candidate refused before promotion"),
        "{e:?}"
    );
    assert!(e.message.contains("task.created"), "{e:?}");
    assert_eq!(state(&k)["initialized"], false);
    assert!(k.quarantined().is_none());
    Ok(())
}

/// A refused candidate is a failed write in the store's evidence: health
/// is not green for it, even though nothing was quarantined.
#[test]
fn a_refused_candidate_counts_as_a_failed_write() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let (k, _) = started(&home, 1)?;
    tamper(&home, "task", "'$.steps_completed', 5")?;
    let t = the_task(&k);
    assert!(step(&k, &t, "key-step-000001").error.is_some());
    let writes = k.store().health().writes;
    assert_eq!(writes.consecutive_failures, 1, "{writes:?}");
    assert!(
        writes
            .last_error
            .as_deref()
            .is_some_and(|e| e.contains("candidate refused before promotion")),
        "{writes:?}"
    );
    Ok(())
}
