//! T00.04 Repository contract suite: versioned put/get, optimistic
//! conflicts, transaction rollback, corruption detection. Written against
//! the `Repository` trait so every implementation runs the same suite.

use proptest::prelude::*;
use rikuri_domain::{DeletionUnitId, PrivacyClass, ProjectId, SchemaVersion, Timestamp};
use rikuri_storage::{
    Entity, Repository, SqliteRepository, StorageError, Store, Versioned, Written,
    deletion_unit_size,
};
use rikuri_testkit::{FixtureClock, FixtureIds, T0, T0_TIME};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Thing {
    id: ProjectId,
    name: String,
    privacy: PrivacyClass,
}

impl Entity for Thing {
    type Id = ProjectId;
    const KIND: &'static str = "project";
    const SCHEMA: SchemaVersion = SchemaVersion::V1;
    fn id(&self) -> ProjectId {
        self.id
    }
    fn privacy_class(&self) -> PrivacyClass {
        self.privacy
    }
}

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn open(dir: &tempfile::TempDir) -> TestResult<Store> {
    Ok(Store::open(
        &dir.path().join("state/rikuri.db"),
        Box::new(FixtureClock::fixed(T0_TIME)),
        &[],
    )?)
}

fn thing(ids: &FixtureIds, name: &str) -> TestResult<Thing> {
    Ok(Thing {
        id: ProjectId::generate(ids)?,
        name: name.into(),
        privacy: PrivacyClass::Private,
    })
}

fn current<R: Repository<Thing>>(
    store: &Store,
    repo: &R,
    id: ProjectId,
) -> TestResult<Versioned<Thing>> {
    Ok(store.read(|tx| repo.get(tx, id))?.ok_or("entity missing")?)
}

/// The contract every `Repository<Thing>` must meet.
fn contract<R: Repository<Thing>>(store: &Store, repo: &R) -> TestResult {
    let ids = FixtureIds::sequential(T0);
    let now = Timestamp(1_790_000_000_000);
    let a = thing(&ids, "a")?;

    // put/get at version 1
    let stored = store.write(|tx| repo.insert(tx, &a, now))?;
    assert_eq!(stored.version, 1);
    assert_eq!(stored.value, a);
    assert_eq!(current(store, repo, a.id)?, stored);

    // duplicate insert refused
    let dup = store.write(|tx| repo.insert(tx, &a, now));
    assert!(
        matches!(dup, Err(StorageError::AlreadyExists { .. })),
        "{dup:?}"
    );

    // update from the read version
    let mut a2 = a.clone();
    a2.name = "a2".into();
    let v2 = store.write(|tx| repo.update(tx, &a2, 1, Timestamp(now.0 + 5)))?;
    assert_eq!(
        (v2.version, v2.created_at, v2.updated_at),
        (2, now, Timestamp(now.0 + 5))
    );

    // stale writer refused, state unchanged
    let mut stale = a.clone();
    stale.name = "stale".into();
    let conflict = store.write(|tx| repo.update(tx, &stale, 1, now));
    assert!(
        matches!(
            conflict,
            Err(StorageError::VersionConflict {
                expected: 1,
                actual: 2,
                ..
            })
        ),
        "{conflict:?}"
    );
    assert_eq!(current(store, repo, a.id)?.value, a2);

    // update of a missing entity
    let ghost = thing(&ids, "ghost")?;
    let missing = store.write(|tx| repo.update(tx, &ghost, 1, now));
    assert!(
        matches!(missing, Err(StorageError::NotFound { .. })),
        "{missing:?}"
    );
    assert!(store.read(|tx| repo.get(tx, ghost.id))?.is_none());

    // rollback: a failed transaction leaves nothing behind
    let b = thing(&ids, "b")?;
    let rolled: Result<(), StorageError> = store.write(|tx| {
        repo.insert(tx, &b, now)?;
        repo.update(tx, &a2, 2, now)?;
        Err(StorageError::Poisoned) // any error after the writes
    });
    assert!(rolled.is_err());
    assert!(
        store.read(|tx| repo.get(tx, b.id))?.is_none(),
        "insert survived rollback"
    );
    assert_eq!(
        current(store, repo, a.id)?.version,
        2,
        "update survived rollback"
    );

    // list returns this kind in ID order
    let c = thing(&ids, "c")?;
    store.write(|tx| repo.insert(tx, &c, now))?;
    let listed: Vec<_> = store
        .read(|tx| repo.list(tx, 10))?
        .into_iter()
        .map(|v| v.value.name)
        .collect();
    assert_eq!(listed, ["a2", "c"]);
    Ok(())
}

#[test]
fn sqlite_repository_meets_the_contract() {
    let dir = tempfile::tempdir().unwrap();
    let store = open(&dir).unwrap();
    contract(&store, &SqliteRepository::<Thing>::new()).unwrap();
}

#[test]
fn committed_state_survives_reopen() {
    let dir = tempfile::tempdir().unwrap();
    let ids = FixtureIds::sequential(T0);
    let a = thing(&ids, "kept").unwrap();
    {
        let store = open(&dir).unwrap();
        store
            .write(|tx| SqliteRepository::new().insert(tx, &a, Timestamp(1)))
            .unwrap();
    }
    let store = open(&dir).unwrap();
    let got = store
        .read(|tx| SqliteRepository::<Thing>::new().get(tx, a.id))
        .unwrap()
        .unwrap();
    assert_eq!((got.value, got.version), (a, 1));
}

#[test]
fn tampered_rows_are_reported_not_trusted() {
    let dir = tempfile::tempdir().unwrap();
    let store = open(&dir).unwrap();
    let repo = SqliteRepository::<Thing>::new();
    let ids = FixtureIds::sequential(T0);
    let a = thing(&ids, "a").unwrap();
    let other = thing(&ids, "other").unwrap();
    store.write(|tx| repo.insert(tx, &a, Timestamp(1))).unwrap();

    // body claims another ID
    let swapped = serde_json::to_string(&other).unwrap();
    store
        .write(|tx| {
            tx.execute(
                "UPDATE objects SET body = ?1 WHERE id = ?2",
                (swapped, a.id.to_string()),
            )?;
            Ok::<_, StorageError>(())
        })
        .unwrap();
    let got = store.read(|tx| repo.get(tx, a.id));
    assert!(matches!(got, Err(StorageError::Corrupt { .. })), "{got:?}");

    // unknown body schema version
    store
        .write(|tx| {
            tx.execute(
                "UPDATE objects SET body = ?1, schema_version = 2 WHERE id = ?2",
                (serde_json::to_string(&a).unwrap(), a.id.to_string()),
            )?;
            Ok::<_, StorageError>(())
        })
        .unwrap();
    let got = store.read(|tx| repo.get(tx, a.id));
    assert!(matches!(got, Err(StorageError::Corrupt { .. })), "{got:?}");

    // the schema refuses an unknown privacy class outright
    let refused = store.write(|tx| {
        tx.execute(
            "UPDATE objects SET privacy_class = 'public' WHERE id = ?1",
            [a.id.to_string()],
        )?;
        Ok::<_, StorageError>(())
    });
    assert!(refused.is_err());
}

#[test]
fn store_health_tracks_success_and_failure() {
    let dir = tempfile::tempdir().unwrap();
    let store = open(&dir).unwrap();
    assert_eq!(
        store.health().last_success,
        None,
        "a store that has done nothing is not proven"
    );
    store.read(|_| Ok::<_, StorageError>(())).unwrap();
    let h = store.health();
    assert!(h.last_success.is_some());
    assert_eq!(h.work_completed, 1);
    let _ = store.write(|_| Err::<(), _>(StorageError::Poisoned));
    let h = store.health();
    assert_eq!(h.consecutive_failures, 1);
    assert!(h.last_error.is_some());
}

#[derive(Debug, Clone)]
enum Op {
    Insert(u8),
    Update { key: u8, expected: u64 },
}

fn op() -> impl Strategy<Value = Op> {
    prop_oneof![
        (0u8..4).prop_map(Op::Insert),
        ((0u8..4), (0u64..6)).prop_map(|(key, expected)| Op::Update { key, expected }),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    /// Model check: across random operation sequences the store agrees with
    /// a simple model. A conflict happens exactly when the expected version
    /// is not the current one, and versions only ever increase by one.
    #[test]
    fn versions_follow_the_model(ops in proptest::collection::vec(op(), 1..40)) {
        let dir = tempfile::tempdir().unwrap();
        let store = open(&dir).unwrap();
        let repo = SqliteRepository::<Thing>::new();
        let ids: Vec<ProjectId> = {
            let s = FixtureIds::sequential(T0);
            (0..4).map(|_| ProjectId::generate(&s).unwrap()).collect()
        };
        let mut model: [Option<u64>; 4] = [None; 4];
        for (n, op) in ops.into_iter().enumerate() {
            match op {
                Op::Insert(k) => {
                    let t = Thing { id: ids[k as usize], name: format!("i{n}"), privacy: PrivacyClass::Normal };
                    let r = store.write(|tx| repo.insert(tx, &t, Timestamp(1)));
                    match model[k as usize] {
                        None => { prop_assert_eq!(r.unwrap().version, 1); model[k as usize] = Some(1); }
                        Some(_) => prop_assert!(matches!(r, Err(StorageError::AlreadyExists { .. })), "expected AlreadyExists, got {:?}", r),
                    }
                }
                Op::Update { key, expected } => {
                    let t = Thing { id: ids[key as usize], name: format!("u{n}"), privacy: PrivacyClass::Normal };
                    let r = store.write(|tx| repo.update(tx, &t, expected, Timestamp(1)));
                    match model[key as usize] {
                        None => prop_assert!(matches!(r, Err(StorageError::NotFound { .. })), "expected NotFound, got {:?}", r),
                        Some(v) if v == expected => {
                            prop_assert_eq!(r.unwrap().version, v + 1);
                            model[key as usize] = Some(v + 1);
                        }
                        Some(v) => prop_assert!(
                            matches!(r, Err(StorageError::VersionConflict { actual, .. }) if actual == v),
                            "expected VersionConflict at {}, got {:?}", v, r
                        ),
                    }
                }
            }
        }
        for (k, id) in ids.iter().enumerate() {
            let got = store.read(|tx| repo.get(tx, *id)).unwrap().map(|v| v.version);
            prop_assert_eq!(got, model[k]);
        }
    }
}

// ---- 2026-10-08 verifier defects (docs/build-records/2026-10-08-storage-ledger-verifier-defects.md)

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Doc {
    id: ProjectId,
    privacy: PrivacyClass,
    unit: Option<rikuri_domain::DeletionUnitId>,
}

impl Entity for Doc {
    type Id = ProjectId;
    const KIND: &'static str = "project";
    const SCHEMA: SchemaVersion = SchemaVersion::V1;
    fn id(&self) -> ProjectId {
        self.id
    }
    fn privacy_class(&self) -> PrivacyClass {
        self.privacy
    }
    fn deletion_unit(&self) -> Option<rikuri_domain::DeletionUnitId> {
        self.unit
    }
}

fn set_column(store: &Store, sql: &str, id: ProjectId) -> TestResult {
    store.write(|tx| {
        tx.execute(sql, [id.to_string()])?;
        Ok::<_, StorageError>(())
    })?;
    Ok(())
}

/// The privacy and deletion-unit columns are what later filters and forget
/// will trust, so they are stored and must agree with the body on every read.
#[test]
fn privacy_and_deletion_columns_must_match_the_body() {
    let dir = tempfile::tempdir().unwrap();
    let store = open(&dir).unwrap();
    let repo = SqliteRepository::<Doc>::new();
    let ids = FixtureIds::sequential(T0);
    let unit = rikuri_domain::DeletionUnitId::generate(&ids).unwrap();
    let doc = Doc {
        id: ProjectId::generate(&ids).unwrap(),
        privacy: PrivacyClass::Secret,
        unit: Some(unit),
    };
    store
        .write(|tx| repo.insert(tx, &doc, Timestamp(1)))
        .unwrap();

    let (class, stored_unit): (String, Option<String>) = store
        .read(|tx| {
            Ok::<_, StorageError>(tx.query_row(
                "SELECT privacy_class, deletion_unit_id FROM objects WHERE id = ?1",
                [doc.id.to_string()],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )?)
        })
        .unwrap();
    assert_eq!(class, "secret");
    assert_eq!(stored_unit, Some(unit.to_string()));
    assert!(store.read(|tx| repo.get(tx, doc.id)).unwrap().is_some());

    let other_unit = rikuri_domain::DeletionUnitId::generate(&ids).unwrap();
    let other = format!("UPDATE objects SET deletion_unit_id = '{other_unit}' WHERE id = ?1");
    for (why, tamper, restore) in [
        (
            "privacy downgraded",
            "UPDATE objects SET privacy_class = 'normal' WHERE id = ?1",
            "UPDATE objects SET privacy_class = 'secret' WHERE id = ?1",
        ),
        (
            "deletion unit dropped",
            "UPDATE objects SET deletion_unit_id = NULL WHERE id = ?1",
            "",
        ),
        ("deletion unit swapped", other.as_str(), ""),
    ] {
        set_column(&store, tamper, doc.id).unwrap();
        let got = store.read(|tx| repo.get(tx, doc.id));
        assert!(
            matches!(got, Err(StorageError::Corrupt { .. })),
            "{why}: {got:?}"
        );
        let listed = store.read(|tx| repo.list(tx, 10));
        assert!(
            matches!(listed, Err(StorageError::Corrupt { .. })),
            "{why}: {listed:?}"
        );
        if !restore.is_empty() {
            set_column(&store, restore, doc.id).unwrap();
        }
        let fix = format!("UPDATE objects SET deletion_unit_id = '{unit}' WHERE id = ?1");
        set_column(&store, &fix, doc.id).unwrap();
    }
}

/// A row whose kind does not match the entity type is not trusted.
#[test]
fn rows_of_another_kind_are_refused() {
    let dir = tempfile::tempdir().unwrap();
    let store = open(&dir).unwrap();
    let repo = SqliteRepository::<Thing>::new();
    let ids = FixtureIds::sequential(T0);
    let a = thing(&ids, "a").unwrap();
    store.write(|tx| repo.insert(tx, &a, Timestamp(1))).unwrap();
    set_column(
        &store,
        "UPDATE objects SET kind = 'task' WHERE id = ?1",
        a.id,
    )
    .unwrap();
    let got = store.read(|tx| repo.get(tx, a.id));
    assert!(matches!(got, Err(StorageError::Corrupt { .. })), "{got:?}");
}

/// Reads run in a transaction that is always rolled back.
#[test]
fn reads_never_commit() {
    let dir = tempfile::tempdir().unwrap();
    let store = open(&dir).unwrap();
    store
        .read(|tx| {
            tx.execute(
                "INSERT INTO objects (id, kind, version, schema_version, privacy_class,
                     deletion_unit_id, created_at_ms, updated_at_ms, body)
                 VALUES ('x', 'k', 1, 1, 'normal', NULL, 1, 1, '{}')",
                [],
            )?;
            Ok::<_, StorageError>(())
        })
        .unwrap();
    let n: i64 = store
        .read(|tx| {
            Ok::<_, StorageError>(tx.query_row("SELECT COUNT(*) FROM objects", [], |r| r.get(0))?)
        })
        .unwrap();
    assert_eq!(n, 0, "a write inside read() was committed");
}

/// Corruption errors never echo unbounded stored text.
#[test]
fn corrupt_row_errors_are_bounded() {
    let dir = tempfile::tempdir().unwrap();
    let store = open(&dir).unwrap();
    let repo = SqliteRepository::<Thing>::new();
    let ids = FixtureIds::sequential(T0);
    let a = thing(&ids, "a").unwrap();
    store.write(|tx| repo.insert(tx, &a, Timestamp(1))).unwrap();
    let huge = "k".repeat(1 << 20);
    store
        .write(|tx| {
            tx.execute(
                "UPDATE objects SET kind = ?1 WHERE id = ?2",
                (huge.as_str(), a.id.to_string()),
            )?;
            Ok::<_, StorageError>(())
        })
        .unwrap();
    let err = store.read(|tx| repo.get(tx, a.id)).unwrap_err();
    let shown = err.to_string();
    assert!(shown.len() < 1024, "error message is {} bytes", shown.len());
}

/// Transactional State Evolution (Architecture §43; E1): the read-back step
/// bites. When what the store holds after a write differs from what was
/// meant to be written (a trigger rewrites the row here, standing for a
/// layer below that changes data), the write fails, the whole transaction
/// rolls back, and the last known-good state stays authoritative.
#[test]
fn a_write_that_does_not_read_back_rolls_back() -> TestResult {
    let dir = tempfile::tempdir()?;
    let store = open(&dir)?;
    let repo = SqliteRepository::<Thing>::new();
    let ids = FixtureIds::sequential(T0);
    let kept = thing(&ids, "kept")?;
    store.write(|tx| repo.insert(tx, &kept, T0_TIME))?;
    store.write(|tx| {
        tx.execute_batch(
            "CREATE TRIGGER rewrite_insert AFTER INSERT ON objects BEGIN \
               UPDATE objects SET body = json_set(body, '$.name', 'rewritten') \
               WHERE id = NEW.id; END; \
             CREATE TRIGGER rewrite_update AFTER UPDATE OF version ON objects \
               WHEN json_extract(NEW.body, '$.name') <> 'rewritten' BEGIN \
               UPDATE objects SET body = json_set(body, '$.name', 'rewritten') \
               WHERE id = NEW.id; END;",
        )
        .map_err(StorageError::from)
    })?;
    let read_back_failed = |got: &Result<Versioned<Thing>, StorageError>| matches!(got, Err(StorageError::Corrupt { reason, .. }) if reason.contains("read-back"));

    // an insert that does not read back is not promoted
    let fresh = thing(&ids, "fresh")?;
    let got = store.write(|tx| repo.insert(tx, &fresh, T0_TIME));
    assert!(read_back_failed(&got), "{got:?}");
    assert!(store.read(|tx| repo.get(tx, fresh.id))?.is_none());

    // nor is an update: the last known-good version stays
    let changed = Thing {
        name: "changed".into(),
        ..kept.clone()
    };
    let got = store.write(|tx| repo.update(tx, &changed, 1, T0_TIME));
    assert!(read_back_failed(&got), "{got:?}");
    let now = current(&store, &repo, kept.id)?;
    assert_eq!((now.version, now.value), (1, kept));
    Ok(())
}

/// The read-back compares the version as well as the body: a row whose
/// body reads back as written but whose version moved underneath (a
/// trigger bumps it here) is not promoted either.
#[test]
fn a_write_whose_version_does_not_read_back_rolls_back() -> TestResult {
    let dir = tempfile::tempdir()?;
    let store = open(&dir)?;
    let repo = SqliteRepository::<Thing>::new();
    let ids = FixtureIds::sequential(T0);
    let kept = thing(&ids, "kept")?;
    store.write(|tx| repo.insert(tx, &kept, T0_TIME))?;
    store.write(|tx| {
        tx.execute_batch(
            "CREATE TRIGGER bump_insert AFTER INSERT ON objects BEGIN \
               UPDATE objects SET version = version + 1 WHERE id = NEW.id; END; \
             CREATE TRIGGER bump_update AFTER UPDATE OF body ON objects \
               WHEN NEW.version = OLD.version + 1 BEGIN \
               UPDATE objects SET version = version + 1 WHERE id = NEW.id; END;",
        )
        .map_err(StorageError::from)
    })?;
    let version_differs = |got: &Result<Versioned<Thing>, StorageError>| matches!(got, Err(StorageError::Corrupt { reason, .. }) if reason.contains("read-back version"));

    let fresh = thing(&ids, "fresh")?;
    let got = store.write(|tx| repo.insert(tx, &fresh, T0_TIME));
    assert!(version_differs(&got), "{got:?}");
    assert!(store.read(|tx| repo.get(tx, fresh.id))?.is_none());

    let changed = Thing {
        name: "changed".into(),
        ..kept.clone()
    };
    let got = store.write(|tx| repo.update(tx, &changed, 1, T0_TIME));
    assert!(version_differs(&got), "{got:?}");
    let now = current(&store, &repo, kept.id)?;
    assert_eq!((now.version, now.value), (1, kept));
    Ok(())
}

/// A transaction lists what it wrote through repositories, in order, with
/// each object's body before (none for an insert) and after; a refused
/// write is not listed, and every transaction starts empty. The kernel
/// checks its candidate against exactly this before committing (E1,
/// ADR 0005).
#[test]
fn a_transaction_lists_what_it_wrote() -> TestResult {
    let dir = tempfile::tempdir()?;
    let store = open(&dir)?;
    let repo = SqliteRepository::<Thing>::new();
    let ids = FixtureIds::sequential(T0);
    let a = thing(&ids, "a")?;
    let b = thing(&ids, "b")?;
    store.write(|tx| repo.insert(tx, &a, T0_TIME))?;
    let renamed = Thing {
        name: "a2".into(),
        ..a.clone()
    };
    let written = store.write(|tx| -> Result<Vec<Written>, StorageError> {
        assert!(tx.written().is_empty(), "a transaction starts empty");
        repo.insert(tx, &b, T0_TIME)?;
        repo.update(tx, &renamed, 1, T0_TIME)?;
        let refused = repo.update(tx, &renamed, 1, T0_TIME);
        assert!(matches!(refused, Err(StorageError::VersionConflict { .. })));
        Ok(tx.written())
    })?;
    assert_eq!(written.len(), 2, "{written:?}");
    assert_eq!(written[0].kind, "project");
    assert_eq!(written[0].id, b.id.to_string());
    assert!(written[0].before.is_none());
    assert_eq!(serde_json::from_str::<Thing>(&written[0].after)?, b);
    assert_eq!(written[1].id, a.id.to_string());
    let before = written[1]
        .before
        .as_deref()
        .ok_or("an update lists its before")?;
    assert_eq!(serde_json::from_str::<Thing>(before)?, a);
    assert_eq!(serde_json::from_str::<Thing>(&written[1].after)?, renamed);
    assert!(store.read(|tx| Ok::<_, StorageError>(tx.written().is_empty()))?);
    Ok(())
}

/// Only a constraint violation on insert means the row already exists; any
/// other failure (here SQLite's integer overflow, raised by a trigger) is
/// reported as what it is, never as `AlreadyExists`.
#[test]
fn an_insert_that_fails_otherwise_is_not_reported_as_already_existing() -> TestResult {
    let dir = tempfile::tempdir()?;
    let store = open(&dir)?;
    let repo = SqliteRepository::<Thing>::new();
    let ids = FixtureIds::sequential(T0);
    store.write(|tx| {
        tx.execute_batch(
            "CREATE TRIGGER overflow BEFORE INSERT ON objects \
             BEGIN SELECT abs(-9223372036854775808); END;",
        )
        .map_err(StorageError::from)
    })?;
    let x = thing(&ids, "x")?;
    let got = store.write(|tx| repo.insert(tx, &x, T0_TIME));
    let e = got.err().ok_or("the insert should fail")?;
    assert!(!matches!(e, StorageError::AlreadyExists { .. }), "{e:?}");
    Ok(())
}

/// An object whose content is forgotten with its deletion unit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Held {
    id: ProjectId,
    unit: DeletionUnitId,
}

impl Entity for Held {
    type Id = ProjectId;
    const KIND: &'static str = "held";
    const SCHEMA: SchemaVersion = SchemaVersion::V1;
    fn id(&self) -> ProjectId {
        self.id
    }
    fn privacy_class(&self) -> PrivacyClass {
        PrivacyClass::Private
    }
    fn deletion_unit(&self) -> Option<DeletionUnitId> {
        Some(self.unit)
    }
}

/// How many objects of any kind belong to a deletion unit, counted through
/// its index: the kernel refuses a new task whose unit is not its own,
/// since forgetting one object must not forget another's content (E1,
/// verifier round 4, N5).
#[test]
fn a_deletion_unit_counts_its_objects_through_its_index() -> TestResult {
    let dir = tempfile::tempdir()?;
    let store = open(&dir)?;
    let ids = FixtureIds::sequential(T0);
    let (shared, single, empty) = (
        DeletionUnitId::generate(&ids)?,
        DeletionUnitId::generate(&ids)?,
        DeletionUnitId::generate(&ids)?,
    );
    let held = SqliteRepository::<Held>::new();
    let things = SqliteRepository::<Thing>::new();
    let objects = [
        Held {
            id: ProjectId::generate(&ids)?,
            unit: shared,
        },
        Held {
            id: ProjectId::generate(&ids)?,
            unit: shared,
        },
        Held {
            id: ProjectId::generate(&ids)?,
            unit: single,
        },
    ];
    let unitless = thing(&ids, "no unit")?;
    store.write(|tx| {
        for o in &objects {
            held.insert(tx, o, T0_TIME)?;
        }
        things.insert(tx, &unitless, T0_TIME).map(|_| ())
    })?;
    let counts = store.read(|tx| {
        Ok::<_, StorageError>((
            deletion_unit_size(tx, shared)?,
            deletion_unit_size(tx, single)?,
            deletion_unit_size(tx, empty)?,
        ))
    })?;
    assert_eq!(counts, (2, 1, 0));
    let plan = store.read(|tx| {
        let mut stmt = tx.prepare(
            "EXPLAIN QUERY PLAN SELECT count(*) FROM objects WHERE deletion_unit_id = ?1",
        )?;
        let rows = stmt.query_map([shared.to_string()], |r| r.get::<_, String>(3))?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(StorageError::from)
    })?;
    assert!(
        plan.iter()
            .any(|step| step.contains("INDEX objects_by_deletion_unit")),
        "{plan:?}"
    );
    Ok(())
}
