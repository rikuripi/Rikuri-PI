//! Owner-only storage files (ADR 0002): the database, its WAL and shared
//! memory files and its directory are readable only by the owner, and a
//! directory or database that group or others can open is refused rather
//! than silently used or re-permissioned.
#![cfg(unix)]

use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use rikuri_domain::Timestamp;
use rikuri_storage::{StorageError, Store};
use rikuri_testkit::FixtureClock;

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn mode(p: &Path) -> TestResult<u32> {
    Ok(std::fs::metadata(p)?.permissions().mode() & 0o777)
}

fn set_mode(p: &Path, m: u32) -> TestResult {
    std::fs::set_permissions(p, std::fs::Permissions::from_mode(m))?;
    Ok(())
}

fn write_one_row(store: &Store) -> TestResult {
    store.write(|tx| {
        tx.execute(
            "INSERT INTO objects (id, kind, version, schema_version, privacy_class,
                     deletion_unit_id, created_at_ms, updated_at_ms, body)
                 VALUES ('x', 'k', 1, 1, 'normal', NULL, 1, 1, '{}')",
            [],
        )?;
        Ok::<_, StorageError>(())
    })?;
    Ok(())
}

fn assert_refused_as_not_private(r: Result<Store, StorageError>, why: &str) {
    match r {
        Err(StorageError::Io { source, .. }) => assert_eq!(
            source.kind(),
            std::io::ErrorKind::PermissionDenied,
            "{why}: {source}"
        ),
        Err(other) => panic!("{why}: wrong error {other:?}"),
        Ok(_) => panic!("{why}: opened a store that group or others can read"),
    }
}

#[test]
fn database_and_directory_are_owner_only() {
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path().join("state");
    let db = data.join("rikuri.db");
    let store = Store::open(&db, Box::new(FixtureClock::fixed(Timestamp(1))), &[]).unwrap();
    write_one_row(&store).unwrap();

    assert_eq!(mode(&data).unwrap(), 0o700, "data directory");
    let wal = data.join("rikuri.db-wal");
    assert!(wal.exists(), "WAL file expected while the store is open");
    for f in [db, wal, data.join("rikuri.db-shm")] {
        if f.exists() {
            let m = mode(&f).unwrap();
            assert_eq!(m & 0o077, 0, "{} is {:o}", f.display(), m);
        }
    }
}

#[test]
fn shared_directory_or_database_is_refused() {
    // an existing directory that others can list
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path().join("shared");
    std::fs::create_dir(&data).unwrap();
    set_mode(&data, 0o755).unwrap();
    let db = data.join("rikuri.db");
    assert_refused_as_not_private(
        Store::open(&db, Box::new(FixtureClock::fixed(Timestamp(1))), &[]),
        "0755 directory",
    );
    assert!(
        !db.exists(),
        "nothing may be created in a refused directory"
    );

    // a private directory holding a database others can read
    let dir = tempfile::tempdir().unwrap();
    let data = dir.path().join("state");
    std::fs::create_dir(&data).unwrap();
    set_mode(&data, 0o700).unwrap();
    let db = data.join("rikuri.db");
    drop(Store::open(&db, Box::new(FixtureClock::fixed(Timestamp(1))), &[]).unwrap());
    set_mode(&db, 0o644).unwrap();
    assert_refused_as_not_private(
        Store::open(&db, Box::new(FixtureClock::fixed(Timestamp(1))), &[]),
        "0644 database",
    );

    // legitimate use: an existing private directory and database reopen
    set_mode(&db, 0o600).unwrap();
    assert!(Store::open(&db, Box::new(FixtureClock::fixed(Timestamp(1))), &[]).is_ok());
}
