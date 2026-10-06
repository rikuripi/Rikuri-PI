//! T00.04 migration compatibility: forward upgrade preserving data, failed
//! migration rolled back to the last good version, and refusal of a
//! database from a newer build or with a changed migration definition.

use rikuri_domain::{Clock, Timestamp};
use rikuri_storage::{Migration, Migrations, StorageError, Store};

struct Fixed;
impl Clock for Fixed {
    fn now(&self) -> Timestamp {
        Timestamp(1)
    }
}

const V1: Migration = Migration {
    version: 1,
    name: "notes",
    sql: "CREATE TABLE notes (id INTEGER PRIMARY KEY, text TEXT NOT NULL) STRICT;",
};
const V2: Migration = Migration {
    version: 2,
    name: "note tags",
    sql: "ALTER TABLE notes ADD COLUMN tag TEXT NOT NULL DEFAULT 'none';",
};
const V2_BROKEN: Migration = Migration {
    version: 2,
    name: "broken",
    sql: "CREATE TABLE half_done (x INTEGER); THIS IS NOT SQL;",
};

fn comp(list: &'static [Migration]) -> Migrations {
    Migrations {
        component: "fixture",
        list,
    }
}

fn open(dir: &tempfile::TempDir, c: &Migrations) -> Result<Store, StorageError> {
    Store::open(&dir.path().join("rikuri.db"), Box::new(Fixed), &[c])
}

fn count(store: &Store, sql: &str) -> Result<i64, StorageError> {
    store.read(|tx| Ok(tx.query_row(sql, [], |r| r.get(0))?))
}

#[test]
fn fresh_open_applies_and_reopen_is_a_no_op() {
    let dir = tempfile::tempdir().unwrap();
    let c = comp(&[V1]);
    let s = open(&dir, &c).unwrap();
    assert_eq!(s.schema_version("fixture").unwrap(), Some(1));
    assert_eq!(s.schema_version("storage").unwrap(), Some(1));
    drop(s);
    let s = open(&dir, &c).unwrap();
    assert_eq!(
        count(&s, "SELECT COUNT(*) FROM schema_migrations").unwrap(),
        2
    );
}

#[test]
fn upgrade_preserves_existing_data() {
    let dir = tempfile::tempdir().unwrap();
    {
        let s = open(&dir, &comp(&[V1])).unwrap();
        s.write(|tx| {
            tx.execute("INSERT INTO notes (text) VALUES ('kept')", [])?;
            Ok::<_, StorageError>(())
        })
        .unwrap();
    }
    let s = open(&dir, &comp(&[V1, V2])).unwrap();
    assert_eq!(s.schema_version("fixture").unwrap(), Some(2));
    assert_eq!(
        count(
            &s,
            "SELECT COUNT(*) FROM notes WHERE text = 'kept' AND tag = 'none'"
        )
        .unwrap(),
        1
    );
}

#[test]
fn failed_migration_leaves_the_last_good_version() {
    let dir = tempfile::tempdir().unwrap();
    drop(open(&dir, &comp(&[V1])).unwrap());
    let err = open(&dir, &comp(&[V1, V2_BROKEN])).unwrap_err();
    assert!(
        matches!(err, StorageError::MigrationFailed { version: 2, .. }),
        "{err:?}"
    );
    // retryable: the old build still opens, at version 1, with no partial table
    let s = open(&dir, &comp(&[V1])).unwrap();
    assert_eq!(s.schema_version("fixture").unwrap(), Some(1));
    assert_eq!(
        count(
            &s,
            "SELECT COUNT(*) FROM sqlite_schema WHERE name = 'half_done'"
        )
        .unwrap(),
        0
    );
}

#[test]
fn database_from_a_newer_build_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    drop(open(&dir, &comp(&[V1, V2])).unwrap());
    let err = open(&dir, &comp(&[V1])).unwrap_err();
    assert!(
        matches!(
            err,
            StorageError::SchemaTooNew {
                found: 2,
                supported: 1,
                ..
            }
        ),
        "{err:?}"
    );
}

#[test]
fn changed_definition_of_an_applied_migration_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    drop(open(&dir, &comp(&[V1])).unwrap());
    const V1_EDITED: Migration = Migration {
        version: 1,
        name: "notes",
        sql: "CREATE TABLE notes (id INTEGER PRIMARY KEY, body TEXT) STRICT;",
    };
    let err = open(&dir, &comp(&[V1_EDITED])).unwrap_err();
    assert!(
        matches!(err, StorageError::MigrationMismatch { version: 1, .. }),
        "{err:?}"
    );
}

#[test]
fn malformed_migration_lists_are_refused_before_touching_the_database() {
    const GAP: Migration = Migration {
        version: 3,
        name: "gap",
        sql: "SELECT 1;",
    };
    let cases: [(&'static [Migration], &str); 4] = [
        (&[], "empty"),
        (&[V2], "starts at 2"),
        (&[V1, GAP], "gap"),
        (&[V1, V1], "duplicate"),
    ];
    for (list, why) in cases {
        let dir = tempfile::tempdir().unwrap();
        let err = open(&dir, &comp(list)).unwrap_err();
        assert!(
            matches!(err, StorageError::InvalidMigrations { .. }),
            "{why}: {err:?}"
        );
        let raw = rikuri_storage::rusqlite::Connection::open(dir.path().join("rikuri.db")).unwrap();
        let tables: i64 = raw
            .query_row(
                "SELECT COUNT(*) FROM sqlite_schema WHERE type = 'table'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(tables, 0, "{why}: tables were created before validation");
    }
    let dir = tempfile::tempdir().unwrap();
    let bad_name = Migrations {
        component: "Bad Name",
        list: &[V1],
    };
    assert!(matches!(
        open(&dir, &bad_name),
        Err(StorageError::InvalidMigrations { .. })
    ));
}

#[test]
fn store_uses_wal_and_full_sync() {
    let dir = tempfile::tempdir().unwrap();
    let s = open(&dir, &comp(&[V1])).unwrap();
    let mode: String = s
        .read(|tx| Ok::<_, StorageError>(tx.query_row("PRAGMA journal_mode", [], |r| r.get(0))?))
        .unwrap();
    let sync: i64 = s
        .read(|tx| Ok::<_, StorageError>(tx.query_row("PRAGMA synchronous", [], |r| r.get(0))?))
        .unwrap();
    assert_eq!(mode, "wal");
    assert_eq!(sync, 2, "synchronous=FULL");
}
