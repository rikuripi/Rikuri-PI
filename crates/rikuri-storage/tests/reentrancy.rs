//! A `Store` call made from inside one of its own transactions is refused
//! with an error, never a silent deadlock (2026-10-08: `state.show` in
//! `rikurid` called `schema_version` inside `read` and hung the daemon;
//! docs/build-records/2026-10-08-s00-07-rikurid.md). Each case runs on its
//! own thread with a deadline, so a deadlock fails the test instead of
//! hanging the suite.

use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use rikuri_domain::Timestamp;
use rikuri_storage::{StorageError, Store};
use rikuri_testkit::FixtureClock;

const DEADLINE: Duration = Duration::from_secs(10);

/// Runs `work` on its own thread; its answer, or `None` if there was none
/// by the deadline. The answer is sent after `work` returns, when the stores
/// and scratch directories it made are already dropped, and the thread is
/// then joined, so nothing it owned outlives the test (before 2026-10-10 the
/// answer came first, and the test process could exit mid-cleanup). A
/// deadlocked thread cannot be joined and is left behind.
fn within_deadline(work: impl FnOnce() -> Option<String> + Send + 'static) -> Option<String> {
    let (tx, rx) = mpsc::channel();
    let worker = std::thread::spawn(move || {
        if let Some(answer) = work() {
            let _ = tx.send(answer);
        }
    });
    match rx.recv_timeout(DEADLINE) {
        Err(mpsc::RecvTimeoutError::Timeout) => None,
        answer => {
            let _ = worker.join();
            answer.ok()
        }
    }
}

/// Runs `case` on a fresh store on its own thread; its result text, or
/// `None` if it did not finish by the deadline.
fn on_a_thread(case: fn(&Store) -> String) -> Option<String> {
    within_deadline(move || {
        let dir = tempfile::tempdir().ok()?;
        let store = Store::open(
            &dir.path().join("state/rikuri.db"),
            Box::new(FixtureClock::fixed(Timestamp(1))),
            &[],
        )
        .ok()?;
        Some(case(&store))
    })
}

/// Runs `case` on two fresh stores, A and B (each in its own scratch
/// directory), on its own thread; its result text, or `None` if it did not
/// finish by the deadline.
fn on_two_stores(case: fn(&Store, &Store) -> String) -> Option<String> {
    within_deadline(move || {
        let (da, db) = (tempfile::tempdir().ok()?, tempfile::tempdir().ok()?);
        let open = |dir: &tempfile::TempDir, name: &str| {
            Store::open(
                &dir.path().join("state").join(name),
                Box::new(FixtureClock::fixed(Timestamp(1))),
                &[],
            )
            .ok()
        };
        let (a, b) = (open(&da, "a.db")?, open(&db, "b.db")?);
        Some(case(&a, &b))
    })
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
    assert_eq!(outcome.as_deref(), Some("Ok(()) Ok(Some(2)) Ok(())"));
}

/// A case's thread owns its store and scratch directory, and both are gone
/// by the time `on_a_thread` answers, so nothing outlives the test. Before
/// 2026-10-10 the answer came first: the test process could exit while the
/// thread was still cleaning up, and most runs left
/// `/tmp/.tmp*/state/rikuri.db.lock` behind
/// (docs/build-records/2026-10-10-test-temp-residue.md).
#[test]
fn a_case_leaves_no_scratch_directory_behind() {
    let Some(db) = on_a_thread(|store| store.path().display().to_string()) else {
        panic!("no answer within {DEADLINE:?}");
    };
    // <scratch>/state/rikuri.db
    let Some(scratch) = Path::new(&db).ancestors().nth(2) else {
        panic!("no scratch directory above {db}");
    };
    assert!(!scratch.exists(), "{} outlived the case", scratch.display());
}

/// Two stores may nest (A's transaction calls B), but re-entering any store
/// already in the nest is refused: A → B → A must not deadlock. (Before
/// 2026-10-08 round 2 the marker remembered only the innermost store.)
#[test]
fn nested_stores_are_allowed_but_reentry_anywhere_in_the_nest_is_refused() {
    let Some(outcome) = on_two_stores(|a, b| {
        let nested = a.read(|_| {
            Ok::<_, StorageError>(b.read(|_| Ok::<_, StorageError>(a.schema_version("storage"))))
        });
        let allowed = a.read(|_| Ok::<_, StorageError>(b.read(|_| Ok::<_, StorageError>(()))));
        format!("{nested:?} | {allowed:?}")
    }) else {
        panic!("A → B → A: no answer within {DEADLINE:?} (deadlock)");
    };
    assert_eq!(outcome, "Ok(Ok(Err(Reentrant))) | Ok(Ok(()))");
}

/// After an inner store's transaction returns, the outer store is still
/// marked active: re-entering it is refused, not a deadlock. (Catches a
/// marker that pops the wrong entry.)
#[test]
fn reentry_after_an_inner_store_returns_is_still_refused() {
    let Some(outcome) = on_two_stores(|a, b| {
        let after_b = a.read(|_| {
            b.read(|_| Ok::<_, StorageError>(()))?;
            Ok::<_, StorageError>(a.schema_version("storage"))
        });
        format!("{after_b:?}")
    }) else {
        panic!("re-entering A after B returned: no answer within {DEADLINE:?} (deadlock)");
    };
    assert_eq!(outcome, "Ok(Err(Reentrant))");
}

fn where_am_i(store: &Store) -> String {
    let before = store.in_transaction();
    let in_read = store
        .read(|_tx| Ok::<_, StorageError>(store.in_transaction()))
        .unwrap_or(false);
    let in_write = store
        .write(|_tx| Ok::<_, StorageError>(store.in_transaction()))
        .unwrap_or(false);
    let failed = store.write(|_tx| Err::<(), _>(StorageError::Reentrant));
    let after = store.in_transaction();
    format!("{before} {in_read} {in_write} {} {after}", failed.is_err())
}

/// `in_transaction` answers from the store's own state: true inside a read
/// or a write, false before, after, and after a failed write. The kernel's
/// crash points use it to prove where they sit (W1, 2026-10-09).
#[test]
fn in_transaction_is_true_only_inside_a_transaction() {
    assert_eq!(
        on_a_thread(where_am_i).as_deref(),
        Some("false true true true false")
    );
}
