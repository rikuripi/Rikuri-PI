//! A `Store` call made from inside one of its own transactions is refused
//! with an error, never a silent deadlock (2026-10-08: `state.show` in
//! `rikurid` called `schema_version` inside `read` and hung the daemon;
//! docs/build-records/2026-10-08-s00-07-rikurid.md). Each case runs on its
//! own thread with a deadline, so a deadlock fails the test instead of
//! hanging the suite.

use std::sync::mpsc;
use std::time::Duration;

use rikuri_domain::{Clock, Timestamp};
use rikuri_storage::{StorageError, Store};

struct Fixed;
impl Clock for Fixed {
    fn now(&self) -> Timestamp {
        Timestamp(1)
    }
}

const DEADLINE: Duration = Duration::from_secs(10);

/// Runs `case` on a fresh store on its own thread; its result text, or
/// `None` if it did not finish by the deadline.
fn on_a_thread(case: fn(&Store) -> String) -> Option<String> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let Ok(dir) = tempfile::tempdir() else {
            return;
        };
        let Ok(store) = Store::open(&dir.path().join("state/rikuri.db"), Box::new(Fixed), &[])
        else {
            return;
        };
        let _ = tx.send(case(&store));
    });
    rx.recv_timeout(DEADLINE).ok()
}

fn schema_version_inside_read(store: &Store) -> String {
    let r = store.read(|_tx| Ok::<_, StorageError>(store.schema_version("storage")));
    format!("{r:?}")
}

fn read_inside_write(store: &Store) -> String {
    let r = store.write(|_tx| Ok::<_, StorageError>(store.read(|_| Ok::<_, StorageError>(()))));
    format!("{r:?}")
}

fn write_inside_read(store: &Store) -> String {
    let r = store.read(|_tx| Ok::<_, StorageError>(store.write(|_| Ok::<_, StorageError>(()))));
    format!("{r:?}")
}

#[test]
fn store_calls_inside_its_own_transaction_are_refused_not_deadlocked() {
    for (why, case) in [
        (
            "schema_version inside read",
            schema_version_inside_read as fn(&Store) -> String,
        ),
        ("read inside write", read_inside_write),
        ("write inside read", write_inside_read),
    ] {
        let Some(outcome) = on_a_thread(case) else {
            panic!("{why}: no answer within {DEADLINE:?} (deadlock)");
        };
        // the outer transaction succeeds and carries the inner refusal
        assert!(outcome.starts_with("Ok(Err("), "{why}: {outcome}");
        assert!(outcome.contains("Reentrant"), "{why}: {outcome}");
    }
}

/// Legitimate use: the same calls one after another, not nested, succeed.
#[test]
fn sequential_store_calls_still_work() {
    let outcome = on_a_thread(|store| {
        let a = store.read(|_| Ok::<_, StorageError>(()));
        let b = store.schema_version("storage");
        let c = store.write(|_| Ok::<_, StorageError>(()));
        format!("{a:?} {b:?} {c:?}")
    });
    assert_eq!(outcome.as_deref(), Some("Ok(()) Ok(Some(1)) Ok(())"));
}
