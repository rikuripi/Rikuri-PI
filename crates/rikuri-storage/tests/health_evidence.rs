//! Store health evidence (2026-10-08, S00.07 rounds 2–3): reads and writes
//! are tracked separately so a successful read cannot hide a failing write,
//! and a caller's refusal is not a store failure.

use rikuri_domain::Timestamp;
use rikuri_storage::{StorageError, Store, StoreFailure};
use rikuri_testkit::FixtureClock;

#[test]
fn reads_do_not_hide_write_failures_and_refusals_are_not_failures() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(
        &dir.path().join("state/rikuri.db"),
        Box::new(FixtureClock::fixed(Timestamp(1))),
        &[],
    )
    .unwrap();

    let refused = store.write(|_| {
        Err::<(), _>(StorageError::NotFound {
            kind: "task",
            id: "t".into(),
        })
    });
    assert!(refused.is_err());
    let h = store.health();
    assert_eq!(h.refused, 1);
    assert_eq!(
        h.writes.consecutive_failures, 0,
        "a refusal counted as a failure"
    );

    let _ = store.write(|_| Err::<(), _>(StorageError::Poisoned));
    store.read(|_| Ok::<_, StorageError>(())).unwrap();
    let h = store.health();
    assert_eq!(
        h.writes.consecutive_failures, 1,
        "a read hid the failed write"
    );
    assert!(h.writes.last_error.is_some());
    assert_eq!(h.reads.succeeded, 1);
    assert_eq!(h.reads.consecutive_failures, 0);

    store.write(|_| Ok::<_, StorageError>(())).unwrap();
    let h = store.health();
    assert_eq!(h.writes.consecutive_failures, 0);
    assert_eq!(h.writes.succeeded, 1);
}

#[test]
fn storage_errors_say_whether_the_store_failed() {
    let refusals = [
        StorageError::NotFound {
            kind: "task",
            id: "t".into(),
        },
        StorageError::AlreadyExists {
            kind: "task",
            id: "t".into(),
        },
        StorageError::VersionConflict {
            kind: "task",
            id: "t".into(),
            expected: 1,
            actual: 2,
        },
    ];
    for e in &refusals {
        assert_eq!(e.store_failure(), None, "{e:?} is a refusal");
    }
    let failures = [
        StorageError::Poisoned,
        StorageError::Reentrant,
        StorageError::Corrupt {
            kind: "task",
            id: "t".into(),
            reason: "x".into(),
        },
    ];
    for e in &failures {
        assert!(e.store_failure().is_some(), "{e:?} is a store failure");
    }
}
