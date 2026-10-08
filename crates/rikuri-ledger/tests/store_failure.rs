//! Which ledger errors count against the store's health (2026-10-08,
//! S00.07 round 3): storage failures and corrupt rows do; a refused event
//! does not.

use rikuri_ledger::LedgerError;
use rikuri_storage::{StorageError, StoreFailure};

#[test]
fn ledger_errors_say_whether_the_store_failed() {
    assert!(
        LedgerError::Corrupt("bad row".into())
            .store_failure()
            .is_some()
    );
    assert!(
        LedgerError::Storage(StorageError::Poisoned)
            .store_failure()
            .is_some()
    );
    for refusal in [
        LedgerError::Invalid("x".into()),
        LedgerError::DuplicateEvent("evt".into()),
        LedgerError::IdempotencyKeyTaken("k".into()),
        LedgerError::Storage(StorageError::NotFound {
            kind: "task",
            id: "t".into(),
        }),
    ] {
        assert_eq!(refusal.store_failure(), None, "{refusal:?} is a refusal");
    }
}
