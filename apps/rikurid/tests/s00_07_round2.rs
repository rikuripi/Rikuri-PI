//! Defects and gaps from the first independent verification of S00.07
//! (2026-10-08, docs/build-records/2026-10-08-s00-07-rikurid.md), plus the
//! owner's choice that a daemon whose state and history disagree serves
//! reads but refuses writes ("read-only quarantine").
#![cfg(unix)]

use std::path::{Path, PathBuf};
use std::str::FromStr;

use rikuri_api::{Request, Response, command};
use rikuri_domain::{DeletionUnitId, PrivacyClass, ProjectId, SystemClock, SystemIdSource, TaskId};
use rikuri_storage::{Repository, SqliteRepository, StoreHealth};
use rikurid::crash::NoCrash;
use rikurid::entities::{Task, TaskStatus};
use rikurid::health::{ComponentHealth, HealthState};
use rikurid::kernel::database_path;
use rikurid::{Idempotency, Kernel, KernelConfig, server};
use serde_json::{Value, json};

fn config(idempotency: Idempotency) -> KernelConfig {
    KernelConfig {
        ids: Box::new(SystemIdSource::new()),
        clock: Box::new(SystemClock),
        crash: Box::new(NoCrash),
        idempotency,
    }
}

fn open(home: &Path) -> Result<Kernel, rikurid::KernelError> {
    Kernel::open(home, config(Idempotency::Enforced))
}

fn call(k: &Kernel, command: &str, params: Value) -> Response {
    k.handle(&Request::new(command, params))
}

fn call_keyed(k: &Kernel, command: &str, params: Value, key: &str) -> Response {
    k.handle(&Request::new(command, params).with_idempotency_key(key))
}

fn code(r: &Response) -> String {
    r.error
        .as_ref()
        .map(|e| e.code.as_str().to_owned())
        .unwrap_or_default()
}

fn result(r: &Response) -> Value {
    r.result.clone().unwrap_or(Value::Null)
}

fn component<'a>(health: &'a Value, name: &str) -> &'a Value {
    health["components"]
        .as_array()
        .and_then(|cs| cs.iter().find(|c| c["component"] == name))
        .unwrap_or(&Value::Null)
}

fn home(dir: &tempfile::TempDir) -> PathBuf {
    dir.path().join("home")
}

/// Important 1: a failed write must show in health even after health's own
/// successful consistency read.
#[test]
fn store_health_reports_failed_writes() {
    let dir = tempfile::tempdir().unwrap();
    let k = open(&home(&dir)).unwrap();
    assert!(call(&k, command::SYSTEM_INIT, json!({})).error.is_none());

    // another connection holds the write lock, so the kernel's write fails
    let raw = rikuri_storage::rusqlite::Connection::open(database_path(&home(&dir))).unwrap();
    raw.execute_batch("BEGIN IMMEDIATE").unwrap();
    let failed = call_keyed(
        &k,
        command::TASK_CREATE,
        json!({"title": "t"}),
        "key-locked-0001",
    );
    assert_eq!(code(&failed), "storage_failure", "{failed:?}");

    let health = result(&call(&k, command::SYSTEM_HEALTH, json!({})));
    let store = component(&health, "store");
    assert_ne!(
        store["state"], "healthy",
        "store hides a failed write: {store}"
    );
    assert_ne!(
        health["state"], "healthy",
        "overall hides a failed write: {health}"
    );

    // once the lock is gone the retry succeeds and health recovers
    raw.execute_batch("ROLLBACK").unwrap();
    let retried = call_keyed(
        &k,
        command::TASK_CREATE,
        json!({"title": "t"}),
        "key-locked-0001",
    );
    assert!(retried.error.is_none(), "{retried:?}");
    let health = result(&call(&k, command::SYSTEM_HEALTH, json!({})));
    assert_eq!(component(&health, "store")["state"], "healthy", "{health}");
}

/// A request the kernel refuses (here a stale version) is not a store
/// failure: health stays green.
#[test]
fn refused_requests_do_not_mark_the_store_failed() {
    let dir = tempfile::tempdir().unwrap();
    let k = open(&home(&dir)).unwrap();
    call(&k, command::SYSTEM_INIT, json!({}));
    let created = call_keyed(
        &k,
        command::TASK_CREATE,
        json!({"title": "t"}),
        "key-create-0001",
    );
    let task = result(&created)["task_id"].clone();
    let stale = call_keyed(
        &k,
        command::TASK_START,
        json!({"task_id": task, "expected_version": 7}),
        "key-start-00001",
    );
    assert_eq!(code(&stale), "version_conflict", "{stale:?}");
    let health = result(&call(&k, command::SYSTEM_HEALTH, json!({})));
    assert_eq!(component(&health, "store")["state"], "healthy", "{health}");
}

/// A store that has never succeeded is not reported Healthy.
#[test]
fn a_store_without_success_is_not_healthy() {
    let c = ComponentHealth::from_store("store", &StoreHealth::default());
    assert_ne!(c.state, HealthState::Healthy, "{c:?}");
}

/// Replay returns the stored outcome; a changed request under the same key
/// is a conflict. (Coverage: skipping the idempotency check survived the
/// suite.)
#[test]
fn replay_and_conflict() {
    let dir = tempfile::tempdir().unwrap();
    let k = open(&home(&dir)).unwrap();
    call(&k, command::SYSTEM_INIT, json!({}));
    let first = call_keyed(
        &k,
        command::TASK_CREATE,
        json!({"title": "a"}),
        "key-create-0001",
    );
    let again = call_keyed(
        &k,
        command::TASK_CREATE,
        json!({"title": "a"}),
        "key-create-0001",
    );
    assert!(again.replayed, "{again:?}");
    assert_eq!(result(&first)["task_id"], result(&again)["task_id"]);
    let changed = call_keyed(
        &k,
        command::TASK_CREATE,
        json!({"title": "b"}),
        "key-create-0001",
    );
    assert_eq!(code(&changed), "idempotency_conflict", "{changed:?}");
    let shown = result(&call(&k, command::STATE_SHOW, json!({})));
    assert_eq!(shown["tasks"].as_array().map(Vec::len), Some(1), "{shown}");
}

/// Owner decision 2026-10-08: when state and history disagree at open, the
/// daemon serves reads and health (Critical, naming the problem) but refuses
/// every write with `quarantined`. Also covers duplicate-effect detection.
#[test]
fn disagreeing_state_quarantines_writes() {
    let dir = tempfile::tempdir().unwrap();
    {
        // the T00.08 shape: idempotency off, the same key applied twice
        let k = Kernel::open(&home(&dir), config(Idempotency::DisabledForRegressionTest)).unwrap();
        call(&k, command::SYSTEM_INIT, json!({}));
        call_keyed(
            &k,
            command::TASK_CREATE,
            json!({"title": "a"}),
            "key-dup-000001",
        );
        call_keyed(
            &k,
            command::TASK_CREATE,
            json!({"title": "a"}),
            "key-dup-000001",
        );
    }
    let k = open(&home(&dir)).unwrap();
    let write = call_keyed(
        &k,
        command::TASK_CREATE,
        json!({"title": "b"}),
        "key-create-0002",
    );
    assert_eq!(code(&write), "quarantined", "{write:?}");
    let shown = call(&k, command::STATE_SHOW, json!({}));
    assert!(shown.error.is_none(), "reads still work: {shown:?}");
    let health = result(&call(&k, command::SYSTEM_HEALTH, json!({})));
    assert_eq!(health["state"], "critical", "{health}");
    assert!(health.to_string().contains("duplicate effect"), "{health}");
}

/// One daemon per home: a second kernel on the same home is refused while
/// the first is open, and allowed after it closes.
#[test]
fn one_kernel_per_home() {
    let dir = tempfile::tempdir().unwrap();
    let first = open(&home(&dir)).unwrap();
    assert!(
        Kernel::open(&home(&dir), config(Idempotency::Enforced)).is_err(),
        "a second kernel opened the same home"
    );
    drop(first);
    assert!(Kernel::open(&home(&dir), config(Idempotency::Enforced)).is_ok());
}

/// Important 2: binding never deletes a file that is not a socket.
#[test]
fn bind_never_deletes_a_file_that_is_not_a_socket() {
    let dir = tempfile::tempdir().unwrap();
    let private = dir.path().join("priv");
    std::fs::create_dir(&private).unwrap();
    std::fs::set_permissions(
        &private,
        std::os::unix::fs::PermissionsExt::from_mode(0o700),
    )
    .unwrap();
    let notes = private.join("notes.txt");
    std::fs::write(&notes, "owner's notes, not a socket").unwrap();
    assert!(server::bind(&notes).is_err(), "bound over a regular file");
    assert_eq!(
        std::fs::read_to_string(&notes).unwrap(),
        "owner's notes, not a socket",
        "the file was replaced"
    );
}

/// Parse errors from caller input are bounded like other errors.
#[test]
fn parameter_errors_are_bounded() {
    let dir = tempfile::tempdir().unwrap();
    let k = open(&home(&dir)).unwrap();
    call(&k, command::SYSTEM_INIT, json!({}));
    let huge = "9".repeat(60_000);
    let r = call_keyed(
        &k,
        command::TASK_START,
        json!({"task_id": "task_01M4EMWQS0K2Q8YJ86QZHNJD9P", "expected_version": huge}),
        "key-start-00001",
    );
    let message = r
        .error
        .as_ref()
        .map(|e| e.message.len())
        .unwrap_or_default();
    assert_eq!(code(&r), "invalid_request", "{r:?}");
    assert!(message < 1024, "error message is {message} bytes");
}

/// An idempotency key on a command that takes none is refused, not
/// silently ignored.
#[test]
fn a_key_on_a_command_without_one_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let k = open(&home(&dir)).unwrap();
    for (cmd, params) in [
        (command::SYSTEM_INIT, json!({})),
        (command::STATE_SHOW, json!({})),
        (command::SYSTEM_HEALTH, json!({})),
        (command::LEDGER_TAIL, json!({})),
    ] {
        let r = call_keyed(&k, cmd, params, "key-ignored-0001");
        assert_eq!(code(&r), "invalid_request", "{cmd}: {r:?}");
    }
}

/// state.show says when it lists only part of the tasks.
#[test]
fn state_show_reports_truncation() {
    let dir = tempfile::tempdir().unwrap();
    let k = open(&home(&dir)).unwrap();
    let init = result(&call(&k, command::SYSTEM_INIT, json!({})));
    let project = ProjectId::from_str(init["project_id"].as_str().unwrap()).unwrap();
    let ids = SystemIdSource::new();
    let repo = SqliteRepository::<Task>::new();
    k.store()
        .write(|tx| {
            for i in 0..1001 {
                let task = Task {
                    id: TaskId::generate(&ids).map_err(|e| {
                        rikuri_storage::StorageError::Corrupt {
                            kind: "task",
                            id: String::new(),
                            reason: e.to_string(),
                        }
                    })?,
                    project_id: project,
                    title: format!("bulk {i}"),
                    privacy_class: PrivacyClass::Normal,
                    deletion_unit_id: DeletionUnitId::generate(&ids).map_err(|e| {
                        rikuri_storage::StorageError::Corrupt {
                            kind: "task",
                            id: String::new(),
                            reason: e.to_string(),
                        }
                    })?,
                    status: TaskStatus::Created,
                    steps_completed: 0,
                    runs: 0,
                    current_run: None,
                    last_run: None,
                };
                repo.insert(tx, &task, rikuri_domain::Timestamp(1))?;
            }
            Ok::<_, rikuri_storage::StorageError>(())
        })
        .unwrap();
    let shown = result(&call(&k, command::STATE_SHOW, json!({})));
    assert_eq!(
        shown["tasks"].as_array().map(Vec::len),
        Some(1000),
        "{}",
        shown["tasks_truncated"]
    );
    assert_eq!(shown["tasks_truncated"], true, "truncation is not reported");
}
