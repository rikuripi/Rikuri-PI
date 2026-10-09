//! One `Store` per database file: the store is the single writer (ADR
//! 0002), so a second open of the same file is refused while the first is
//! open, and allowed after it closes (2026-10-08: two `rikurid` daemons on
//! one home both committed writes).

use rikuri_domain::{Clock, Timestamp};
use rikuri_storage::Store;

struct Fixed;
impl Clock for Fixed {
    fn now(&self) -> Timestamp {
        Timestamp(1)
    }
}

#[test]
fn a_second_store_on_the_same_database_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("state/rikuri.db");
    let first = Store::open(&path, Box::new(Fixed), &[]).unwrap();
    let second = Store::open(&path, Box::new(Fixed), &[]);
    assert!(second.is_err(), "a second store opened the same database");
    drop(first);
    assert!(Store::open(&path, Box::new(Fixed), &[]).is_ok());
}

#[test]
fn a_second_open_reports_in_use() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("state/rikuri.db");
    let _first = Store::open(&path, Box::new(Fixed), &[]).unwrap();
    let second = Store::open(&path, Box::new(Fixed), &[]);
    assert!(
        matches!(second, Err(rikuri_storage::StorageError::InUse { .. })),
        "{second:?}"
    );
}

/// The lock file is owner-only, and a shared one is refused before the
/// database is created.
#[cfg(unix)]
#[test]
fn the_lock_file_is_owner_only() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("state/rikuri.db");
    drop(Store::open(&path, Box::new(Fixed), &[]).unwrap());
    let lock = dir.path().join("state/rikuri.db.lock");
    let mode = std::fs::metadata(&lock).unwrap().permissions().mode() & 0o777;
    assert_eq!(mode & 0o077, 0, "lock file is {mode:o}");

    let dir = tempfile::tempdir().unwrap();
    let state = dir.path().join("state");
    std::fs::create_dir(&state).unwrap();
    std::fs::set_permissions(&state, std::fs::Permissions::from_mode(0o700)).unwrap();
    let lock = state.join("rikuri.db.lock");
    std::fs::write(&lock, "").unwrap();
    std::fs::set_permissions(&lock, std::fs::Permissions::from_mode(0o644)).unwrap();
    assert!(Store::open(&state.join("rikuri.db"), Box::new(Fixed), &[]).is_err());
    assert!(
        !state.join("rikuri.db").exists(),
        "a database was created although the lock was refused"
    );
}
