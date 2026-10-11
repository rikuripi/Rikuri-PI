//! T00.04 migration compatibility: forward upgrade preserving data, failed
//! migration rolled back to the last good version, and refusal of a
//! database from a newer build or with a changed migration definition.

use rikuri_domain::Timestamp;
use rikuri_storage::{Migration, Migrations, StorageError, Store};
use rikuri_testkit::FixtureClock;

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

// The database lives in a subdirectory the store creates owner-only; the
// temporary directory itself follows the umask and may be shared.
fn open(dir: &tempfile::TempDir, c: &Migrations) -> Result<Store, StorageError> {
    Store::open(
        &dir.path().join("state/rikuri.db"),
        Box::new(FixtureClock::fixed(Timestamp(1))),
        &[c],
    )
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
    assert_eq!(s.schema_version("storage").unwrap(), Some(2));
    drop(s);
    let s = open(&dir, &c).unwrap();
    assert_eq!(
        count(&s, "SELECT COUNT(*) FROM schema_migrations").unwrap(),
        3
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
fn malformed_migration_lists_are_refused_before_any_table_exists() {
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
        let raw =
            rikuri_storage::rusqlite::Connection::open(dir.path().join("state/rikuri.db")).unwrap();
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

// ---- 2026-10-08 verifier defects (docs/build-records/2026-10-08-storage-ledger-verifier-defects.md)

const A1: Migration = Migration {
    version: 1,
    name: "a table",
    sql: "CREATE TABLE a (x INTEGER) STRICT;",
};
const A2: Migration = Migration {
    version: 2,
    name: "a extra",
    sql: "CREATE TABLE a_extra (y INTEGER) STRICT;",
};
const B1: Migration = Migration {
    version: 1,
    name: "b table",
    sql: "CREATE TABLE b (x INTEGER) STRICT;",
};
const B2_BROKEN: Migration = Migration {
    version: 2,
    name: "b broken",
    sql: "INSERT INTO b (x) VALUES ('not a number');",
};
const B1_EDITED: Migration = Migration {
    version: 1,
    name: "b table",
    sql: "CREATE TABLE b (x TEXT) STRICT;",
};

fn named(component: &'static str, list: &'static [Migration]) -> Migrations {
    Migrations { component, list }
}

fn open_all(dir: &tempfile::TempDir, components: &[&Migrations]) -> Result<Store, StorageError> {
    Store::open(
        &dir.path().join("state/rikuri.db"),
        Box::new(FixtureClock::fixed(Timestamp(1))),
        components,
    )
}

/// Architecture §43: a failed upgrade must not leave a half-upgraded store.
/// One component's failing step must undo every other component's steps, so
/// the previous build still opens.
#[test]
fn failed_multi_component_upgrade_changes_nothing() {
    let dir = tempfile::tempdir().unwrap();
    drop(open_all(&dir, &[&named("aaa", &[A1]), &named("bbb", &[B1])]).unwrap());

    let new_build = [&named("aaa", &[A1, A2]), &named("bbb", &[B1, B2_BROKEN])];
    let err = open_all(&dir, &new_build).unwrap_err();
    assert!(
        matches!(
            err,
            StorageError::MigrationFailed {
                component: "bbb",
                version: 2,
                ..
            }
        ),
        "{err:?}"
    );
    // retrying gives the same answer, not a different, stuck state
    let again = open_all(&dir, &new_build).unwrap_err();
    assert!(
        matches!(
            again,
            StorageError::MigrationFailed {
                component: "bbb",
                ..
            }
        ),
        "{again:?}"
    );

    let old = open_all(&dir, &[&named("aaa", &[A1]), &named("bbb", &[B1])])
        .expect("the previous build must still open after a failed upgrade");
    assert_eq!(old.schema_version("aaa").unwrap(), Some(1));
    assert_eq!(
        count(
            &old,
            "SELECT COUNT(*) FROM sqlite_schema WHERE name = 'a_extra'"
        )
        .unwrap(),
        0
    );
}

/// A refusal found in one component (here a changed definition) must not
/// leave another component already upgraded.
#[test]
fn refusal_in_one_component_upgrades_no_other() {
    let dir = tempfile::tempdir().unwrap();
    drop(open_all(&dir, &[&named("aaa", &[A1]), &named("bbb", &[B1])]).unwrap());

    let err = open_all(
        &dir,
        &[&named("aaa", &[A1, A2]), &named("bbb", &[B1_EDITED])],
    )
    .unwrap_err();
    assert!(
        matches!(
            err,
            StorageError::MigrationMismatch {
                component: "bbb",
                ..
            }
        ),
        "{err:?}"
    );
    let old = open_all(&dir, &[&named("aaa", &[A1]), &named("bbb", &[B1])])
        .expect("the previous build must still open after a refused upgrade");
    assert_eq!(old.schema_version("aaa").unwrap(), Some(1));
}

/// Recorded versions must run 1..=n: a missing bookkeeping row means the
/// schema history was tampered with or lost, so the store is not trusted.
#[test]
fn recorded_versions_must_be_contiguous() {
    const V3: Migration = Migration {
        version: 3,
        name: "note index",
        sql: "CREATE INDEX notes_by_text ON notes (text);",
    };
    let dir = tempfile::tempdir().unwrap();
    drop(open(&dir, &comp(&[V1, V2, V3])).unwrap());
    let raw =
        rikuri_storage::rusqlite::Connection::open(dir.path().join("state/rikuri.db")).unwrap();
    raw.execute(
        "DELETE FROM schema_migrations WHERE component = 'fixture' AND version = 2",
        [],
    )
    .unwrap();
    drop(raw);
    let err = open(&dir, &comp(&[V1, V2, V3])).unwrap_err();
    assert!(
        matches!(err, StorageError::InvalidMigrations { .. }),
        "{err:?}"
    );
}

/// Storage migration 2 adds the index that counts a deletion unit's
/// objects (E1, N5). A store whose storage component is still at version 1
/// (rebuilt here by removing the index and its migration record) upgrades
/// at open with its objects intact.
#[test]
fn storage_migration_2_adds_the_deletion_unit_index_to_a_version_1_store() {
    let dir = tempfile::tempdir().unwrap();
    let c = comp(&[V1]);
    let path = dir.path().join("state/rikuri.db");
    drop(open(&dir, &c).unwrap());
    let raw = rikuri_storage::rusqlite::Connection::open(&path).unwrap();
    raw.execute_batch(
        "INSERT INTO objects (id, kind, version, schema_version, privacy_class, deletion_unit_id,
                              created_at_ms, updated_at_ms, body)
         VALUES ('held_1', 'held', 1, 1, 'private', 'dunit_1', 1, 1, '{}');
         DROP INDEX objects_by_deletion_unit;
         DELETE FROM schema_migrations WHERE component = 'storage' AND version = 2;",
    )
    .unwrap();
    drop(raw);
    let s = open(&dir, &c).unwrap();
    assert_eq!(s.schema_version("storage").unwrap(), Some(2));
    assert_eq!(
        count(
            &s,
            "SELECT COUNT(*) FROM sqlite_master WHERE type = 'index' AND name = 'objects_by_deletion_unit'"
        )
        .unwrap(),
        1
    );
    assert_eq!(
        count(
            &s,
            "SELECT COUNT(*) FROM objects WHERE deletion_unit_id = 'dunit_1'"
        )
        .unwrap(),
        1
    );
}
