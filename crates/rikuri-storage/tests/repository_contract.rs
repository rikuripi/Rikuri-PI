//! T00.04 Repository contract suite: versioned put/get, optimistic
//! conflicts, transaction rollback, corruption detection. Written against
//! the `Repository` trait so every implementation runs the same suite.

use std::sync::atomic::{AtomicU64, Ordering};

use proptest::prelude::*;
use rikuri_domain::{Clock, IdError, IdSource, PrivacyClass, ProjectId, SchemaVersion, Timestamp};
use rikuri_storage::{Entity, Repository, SqliteRepository, StorageError, Store, Versioned};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

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

struct Seq(AtomicU64);
impl IdSource for Seq {
    fn next_ulid(&self) -> Result<Ulid, IdError> {
        Ok(Ulid::from_parts(
            1_790_000_000_000,
            u128::from(self.0.fetch_add(1, Ordering::SeqCst)),
        ))
    }
}

struct Fixed;
impl Clock for Fixed {
    fn now(&self) -> Timestamp {
        Timestamp(1_790_000_000_000)
    }
}

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn open(dir: &tempfile::TempDir) -> TestResult<Store> {
    Ok(Store::open(
        &dir.path().join("state/rikuri.db"),
        Box::new(Fixed),
        &[],
    )?)
}

fn thing(ids: &Seq, name: &str) -> TestResult<Thing> {
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
    let ids = Seq(AtomicU64::new(1));
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
    let ids = Seq(AtomicU64::new(1));
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
    let ids = Seq(AtomicU64::new(1));
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
            let s = Seq(AtomicU64::new(1));
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
