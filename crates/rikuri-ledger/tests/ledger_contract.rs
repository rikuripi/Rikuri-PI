//! T00.05 Ledger contract suite: append, query, idempotency (replay returns
//! the stored response; a changed request is refused), append-only
//! enforcement, referenced-content limits, and transactional atomicity.

use std::sync::atomic::{AtomicU64, Ordering};

use proptest::prelude::*;
use rikuri_domain::{
    ActorId, AnyId, Clock, EventId, IdError, IdSource, RiskClass, TaskId, Timestamp,
};
use rikuri_ledger::{
    EventStatus, EventType, IdempotencyCheck, IdempotencyKey, LedgerError, MAX_METADATA_STRING,
    NewEvent,
};
use rikuri_storage::{StorageError, Store};
use serde_json::{Map, Value, json};
use ulid::Ulid;

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

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

struct Fixture {
    _dir: tempfile::TempDir,
    store: Store,
    ids: Seq,
    actor: ActorId,
}

fn fixture() -> TestResult<Fixture> {
    let dir = tempfile::tempdir()?;
    // in a subdirectory the store creates owner-only; the temporary
    // directory itself follows the umask and may be shared
    let store = Store::open(
        &dir.path().join("state/rikuri.db"),
        Box::new(Fixed),
        &[&rikuri_ledger::MIGRATIONS],
    )?;
    let ids = Seq(AtomicU64::new(1));
    let actor = ActorId::generate(&ids)?;
    Ok(Fixture {
        _dir: dir,
        store,
        ids,
        actor,
    })
}

fn event(f: &Fixture, kind: &str, risk: RiskClass, key: Option<&str>) -> TestResult<NewEvent> {
    Ok(NewEvent {
        event_id: EventId::generate(&f.ids)?,
        event_type: EventType::new(kind)?,
        status: EventStatus::Succeeded,
        risk,
        actor_id: f.actor,
        project_id: None,
        task_id: None,
        run_id: None,
        subject_ids: vec![],
        input_refs: vec![],
        result_refs: vec![],
        security_decision_id: None,
        cost_record_id: None,
        idempotency_key: key.map(IdempotencyKey::new).transpose()?,
        caused_by: None,
        metadata: Map::new(),
    })
}

fn now() -> Timestamp {
    Timestamp(1_790_000_000_000)
}

#[test]
fn append_then_query_in_order() {
    let f = fixture().unwrap();
    let task = TaskId::generate(&f.ids).unwrap();
    let mut first = event(&f, "task.created", RiskClass::KnownRepair, Some("key-0001")).unwrap();
    first.task_id = Some(task);
    first.subject_ids = vec![AnyId::from(task)];
    first.metadata.insert("steps".into(), json!(0));
    let mut second = event(&f, "task.stepped", RiskClass::KnownRepair, Some("key-0002")).unwrap();
    second.task_id = Some(task);
    second.caused_by = Some(first.event_id);
    let observed = event(&f, "system.observed", RiskClass::ObserveReadOnly, None).unwrap();

    let stored = f
        .store
        .write(|tx| {
            let a = rikuri_ledger::append(tx, &first, now())?;
            let b = rikuri_ledger::append(tx, &second, now())?;
            let c = rikuri_ledger::append(tx, &observed, now())?;
            Ok::<_, LedgerError>((a, b, c))
        })
        .unwrap();
    assert!(stored.0.seq < stored.1.seq && stored.1.seq < stored.2.seq);
    assert_eq!(stored.0.subject_ids, vec![AnyId::from(task)]);
    assert_eq!(stored.1.caused_by, Some(first.event_id));

    let tail = f.store.read(|tx| rikuri_ledger::tail(tx, 2)).unwrap();
    let tail_ids: Vec<_> = tail.iter().map(|e| e.event_id).collect();
    assert_eq!(tail_ids, vec![second.event_id, observed.event_id]);

    let by_task = f.store.read(|tx| rikuri_ledger::by_task(tx, task)).unwrap();
    assert_eq!(by_task.len(), 2);
    assert_eq!(by_task[0].metadata.get("steps"), Some(&json!(0)));

    let got = f
        .store
        .read(|tx| rikuri_ledger::get(tx, observed.event_id))
        .unwrap()
        .unwrap();
    assert_eq!(got, stored.2);
}

#[test]
fn consequential_events_need_an_idempotency_key() {
    let f = fixture().unwrap();
    let e = event(&f, "task.created", RiskClass::NovelModification, None).unwrap();
    let r = f.store.write(|tx| rikuri_ledger::append(tx, &e, now()));
    assert!(
        matches!(r, Err(LedgerError::IdempotencyKeyRequired { .. })),
        "{r:?}"
    );
}

#[test]
fn idempotency_replays_the_same_request_and_refuses_a_changed_one() {
    let f = fixture().unwrap();
    let key = IdempotencyKey::new("retry-key-1").unwrap();
    let params = json!({"title": "x", "b": 1});
    let reordered: Value = serde_json::from_str(r#"{"b":1,"title":"x"}"#).unwrap();
    let hash = rikuri_ledger::request_hash("task.create", &params);
    assert_eq!(hash, rikuri_ledger::request_hash("task.create", &reordered));

    let e = event(
        &f,
        "task.created",
        RiskClass::KnownRepair,
        Some("retry-key-1"),
    )
    .unwrap();
    let response = json!({"task_id": "t", "version": 1});
    f.store
        .write(|tx| {
            assert_eq!(
                rikuri_ledger::check(tx, &key, "task.create", &hash)?,
                IdempotencyCheck::Fresh
            );
            rikuri_ledger::append(tx, &e, now())?;
            rikuri_ledger::record(tx, &key, "task.create", &hash, e.event_id, &response, now())
        })
        .unwrap();

    let replay = f
        .store
        .read(|tx| rikuri_ledger::check(tx, &key, "task.create", &hash))
        .unwrap();
    match replay {
        IdempotencyCheck::Replay(r) => {
            assert_eq!((r.event_id, r.response), (e.event_id, response.clone()));
        }
        other => panic!("expected replay, got {other:?}"),
    }

    let changed = rikuri_ledger::request_hash("task.create", &json!({"title": "y"}));
    let conflict = f
        .store
        .read(|tx| rikuri_ledger::check(tx, &key, "task.create", &changed))
        .unwrap();
    assert!(
        matches!(conflict, IdempotencyCheck::Conflict(_)),
        "{conflict:?}"
    );
    let other_command = f
        .store
        .read(|tx| rikuri_ledger::check(tx, &key, "task.step", &hash))
        .unwrap();
    assert!(matches!(other_command, IdempotencyCheck::Conflict(_)));

    // the key cannot be recorded twice
    let again = f.store.write(|tx| {
        rikuri_ledger::record(tx, &key, "task.create", &hash, e.event_id, &response, now())
    });
    assert!(
        matches!(again, Err(LedgerError::IdempotencyKeyTaken(_))),
        "{again:?}"
    );
}

#[test]
fn ledger_rows_are_append_only_even_for_raw_sql() {
    let f = fixture().unwrap();
    let e = event(
        &f,
        "task.created",
        RiskClass::KnownRepair,
        Some("append-only-1"),
    )
    .unwrap();
    let key = IdempotencyKey::new("append-only-1").unwrap();
    f.store
        .write(|tx| {
            rikuri_ledger::append(tx, &e, now())?;
            rikuri_ledger::record(tx, &key, "c", "h", e.event_id, &json!({}), now())
        })
        .unwrap();
    for sql in [
        "UPDATE events SET status = 'failed'",
        "DELETE FROM events",
        "UPDATE idempotency SET response = '{}'",
        "DELETE FROM idempotency",
    ] {
        let r = f.store.write(|tx| {
            tx.execute(sql, [])?;
            Ok::<_, StorageError>(())
        });
        assert!(r.is_err(), "{sql} was allowed");
    }
    let still = f.store.read(|tx| rikuri_ledger::tail(tx, 10)).unwrap();
    assert_eq!(still.len(), 1);
    assert_eq!(still[0].status, EventStatus::Succeeded);
}

#[test]
fn events_cannot_embed_content() {
    let f = fixture().unwrap();
    let cases: Vec<(&str, Value)> = vec![
        ("long text", json!("x".repeat(MAX_METADATA_STRING + 1))),
        ("nested object", json!({"body": "secret"})),
        ("list of objects", json!([{"a": 1}])),
    ];
    for (why, value) in cases {
        let mut e = event(&f, "task.created", RiskClass::ObserveReadOnly, None).unwrap();
        e.metadata.insert("payload".into(), value);
        let r = f.store.write(|tx| rikuri_ledger::append(tx, &e, now()));
        assert!(matches!(r, Err(LedgerError::Invalid(_))), "{why}: {r:?}");
    }
    let mut ok = event(&f, "task.created", RiskClass::ObserveReadOnly, None).unwrap();
    ok.metadata.insert("count".into(), json!(3));
    ok.metadata.insert("codes".into(), json!(["a", "b"]));
    assert!(
        f.store
            .write(|tx| rikuri_ledger::append(tx, &ok, now()))
            .is_ok()
    );
}

#[test]
fn references_and_names_are_validated() {
    let f = fixture().unwrap();
    let mut orphan = event(&f, "task.stepped", RiskClass::ObserveReadOnly, None).unwrap();
    orphan.caused_by = Some(EventId::generate(&f.ids).unwrap());
    let r = f
        .store
        .write(|tx| rikuri_ledger::append(tx, &orphan, now()));
    assert!(
        matches!(
            r,
            Err(LedgerError::MissingReference {
                field: "caused_by",
                ..
            })
        ),
        "{r:?}"
    );

    let e = event(&f, "task.created", RiskClass::ObserveReadOnly, None).unwrap();
    let dup = f.store.write(|tx| {
        rikuri_ledger::append(tx, &e, now())?;
        rikuri_ledger::append(tx, &e, now())
    });
    assert!(
        matches!(dup, Err(LedgerError::DuplicateEvent(_))),
        "{dup:?}"
    );

    let unknown = EventId::generate(&f.ids).unwrap();
    let key = IdempotencyKey::new("dangling-1").unwrap();
    let r = f
        .store
        .write(|tx| rikuri_ledger::record(tx, &key, "c", "h", unknown, &json!({}), now()));
    assert!(
        matches!(r, Err(LedgerError::MissingReference { .. })),
        "{r:?}"
    );

    for bad in ["Task.created", "task", "task..x", "task.created!", ""] {
        assert!(EventType::new(bad).is_err(), "{bad:?} accepted");
    }
    for bad in ["short", "has space here", &"k".repeat(129)] {
        assert!(IdempotencyKey::new(bad).is_err(), "{bad:?} accepted");
    }
}

#[test]
fn a_failed_transaction_appends_nothing() {
    let f = fixture().unwrap();
    let e = event(&f, "task.created", RiskClass::ObserveReadOnly, None).unwrap();
    let r: Result<(), LedgerError> = f.store.write(|tx| {
        rikuri_ledger::append(tx, &e, now())?;
        Err(LedgerError::Invalid("caller failed after appending".into()))
    });
    assert!(r.is_err());
    assert!(
        f.store
            .read(|tx| rikuri_ledger::tail(tx, 10))
            .unwrap()
            .is_empty()
    );
}

#[test]
fn replay_visits_every_event_in_order() {
    let f = fixture().unwrap();
    let mut appended = vec![];
    for i in 0..5 {
        let e = event(&f, "system.observed", RiskClass::ObserveReadOnly, None).unwrap();
        appended.push(e.event_id);
        f.store
            .write(|tx| rikuri_ledger::append(tx, &e, Timestamp(i)))
            .unwrap();
    }
    let mut seen = vec![];
    let n = f
        .store
        .read(|tx| {
            rikuri_ledger::replay(tx, |e| {
                seen.push(e.event_id);
                Ok::<_, LedgerError>(())
            })
        })
        .unwrap();
    assert_eq!(n, 5);
    assert_eq!(seen, appended);
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]

    /// Across random batches, some committed and some rolled back, the
    /// ledger holds exactly the committed events, in commit order, with
    /// strictly increasing sequence numbers.
    #[test]
    fn ledger_holds_exactly_the_committed_events(batches in proptest::collection::vec((1usize..4, any::<bool>()), 1..12)) {
        let f = fixture().unwrap();
        let mut expected = vec![];
        for (size, commit) in batches {
            let events: Vec<NewEvent> = (0..size)
                .map(|_| event(&f, "system.observed", RiskClass::ObserveReadOnly, None).unwrap())
                .collect();
            let r: Result<(), LedgerError> = f.store.write(|tx| {
                for e in &events {
                    rikuri_ledger::append(tx, e, now())?;
                }
                if commit { Ok(()) } else { Err(LedgerError::Invalid("rollback".into())) }
            });
            prop_assert_eq!(r.is_ok(), commit);
            if commit {
                expected.extend(events.iter().map(|e| e.event_id));
            }
        }
        let all = f.store.read(|tx| rikuri_ledger::tail(tx, 1000)).unwrap();
        let ids: Vec<_> = all.iter().map(|e| e.event_id).collect();
        prop_assert_eq!(ids, expected);
        prop_assert!(all.windows(2).all(|w| w[0].seq < w[1].seq));
    }
}

// ---- 2026-10-08 verifier defects (docs/build-records/2026-10-08-storage-ledger-verifier-defects.md)

/// Append-only must hold against every SQL form that rewrites a row, not
/// only UPDATE and DELETE: REPLACE deletes the old row without firing DELETE
/// triggers unless `recursive_triggers` is on.
#[test]
fn ledger_rows_refuse_replace_too() {
    let f = fixture().unwrap();
    let e = event(
        &f,
        "task.created",
        RiskClass::KnownRepair,
        Some("replace-1"),
    )
    .unwrap();
    let key = IdempotencyKey::new("replace-1").unwrap();
    f.store
        .write(|tx| {
            rikuri_ledger::append(tx, &e, now())?;
            rikuri_ledger::record(tx, &key, "c", "h", e.event_id, &json!({}), now())
        })
        .unwrap();
    for sql in [
        // same seq and event_id, rewritten fields
        "INSERT OR REPLACE INTO events SELECT seq, event_id, schema_version, recorded_at_ms,
            'task.deleted', 'failed', actor_id, project_id, task_id, run_id, subject_ids,
            input_refs, result_refs, security_decision_id, cost_record_id, idempotency_key,
            caused_by_event_id, '{\"rewritten\":true}' FROM events",
        // new seq, same event_id
        "REPLACE INTO events (event_id, schema_version, recorded_at_ms, event_type, status,
            actor_id, subject_ids, input_refs, result_refs, metadata)
         SELECT event_id, schema_version, recorded_at_ms, 'task.deleted', 'failed', actor_id,
            '[]', '[]', '[]', '{}' FROM events",
        "INSERT OR REPLACE INTO idempotency SELECT idempotency_key, command, request_hash,
            event_id, '{\"task_id\":\"forged\"}', recorded_at_ms FROM idempotency",
        "INSERT INTO idempotency SELECT idempotency_key, command, request_hash, event_id, '{}',
            recorded_at_ms FROM idempotency WHERE true
         ON CONFLICT (idempotency_key) DO UPDATE SET response = '{\"task_id\":\"forged\"}'",
    ] {
        let r = f.store.write(|tx| {
            tx.execute(sql, [])?;
            Ok::<_, StorageError>(())
        });
        assert!(r.is_err(), "allowed: {sql}");
    }
    let still = f.store.read(|tx| rikuri_ledger::tail(tx, 10)).unwrap();
    assert_eq!(still.len(), 1);
    assert_eq!(still[0].event_type.as_str(), "task.created");
    assert_eq!(still[0].status, EventStatus::Succeeded);
    assert!(still[0].metadata.is_empty());
    let replay = f
        .store
        .read(|tx| rikuri_ledger::check(tx, &key, "c", "h"))
        .unwrap();
    match replay {
        IdempotencyCheck::Replay(r) => assert_eq!(r.response, json!({})),
        other => panic!("expected the original record to replay, got {other:?}"),
    }
}

/// An error never echoes unbounded caller input (the PR #7 defect class).
#[test]
fn event_type_errors_are_bounded() {
    let huge = "a".repeat(1 << 20);
    let err = EventType::new(&huge).unwrap_err();
    assert!(
        err.to_string().len() < 512,
        "error message is {} bytes",
        err.to_string().len()
    );
}

/// Metadata keys obey the same length limit as metadata strings.
#[test]
fn metadata_keys_are_bounded() {
    let f = fixture().unwrap();
    let mut e = event(&f, "task.noted", RiskClass::ObserveReadOnly, None).unwrap();
    e.metadata
        .insert("k".repeat(MAX_METADATA_STRING + 1), json!(1));
    let r = f.store.write(|tx| rikuri_ledger::append(tx, &e, now()));
    match r {
        Err(LedgerError::Invalid(msg)) => assert!(msg.len() < 512, "{} bytes", msg.len()),
        other => panic!("long key accepted or wrong error: {other:?}"),
    }
}

/// The idempotency command is a short name and the response holds only
/// small scalars or ID lists: idempotency rows cannot be deleted, so content
/// stored there could never be forgotten.
#[test]
fn idempotency_command_and_response_are_validated() {
    let f = fixture().unwrap();
    let e = event(
        &f,
        "task.created",
        RiskClass::KnownRepair,
        Some("validate-1"),
    )
    .unwrap();
    f.store
        .write(|tx| rikuri_ledger::append(tx, &e, now()))
        .unwrap();
    let key = IdempotencyKey::new("validate-1").unwrap();
    let long_command = "c".repeat(65);
    let bad: [(&str, &str, Value); 6] = [
        ("empty command", "", json!({})),
        ("long command", &long_command, json!({})),
        ("spaced command", "task create", json!({})),
        (
            "nested response",
            "task.create",
            json!({"body": {"text": "secret"}}),
        ),
        (
            "long string response",
            "task.create",
            json!({"note": "x".repeat(MAX_METADATA_STRING + 1)}),
        ),
        ("non-object response", "task.create", json!("plain text")),
    ];
    for (why, command, response) in bad {
        let r = f.store.write(|tx| {
            rikuri_ledger::record(tx, &key, command, "h", e.event_id, &response, now())
        });
        assert!(matches!(r, Err(LedgerError::Invalid(_))), "{why}: {r:?}");
    }
    let ok = f.store.write(|tx| {
        rikuri_ledger::record(
            tx,
            &key,
            "task.create",
            "h",
            e.event_id,
            &json!({"task_id": "t", "version": 1}),
            now(),
        )
    });
    assert!(ok.is_ok(), "{ok:?}");
}

/// Append-only holds on any connection, not only the store's: the schema's
/// insert guards refuse REPLACE on a raw connection where
/// `recursive_triggers` is off (so DELETE triggers do not fire for REPLACE).
#[test]
fn ledger_rows_refuse_replace_on_a_raw_connection() {
    let f = fixture().unwrap();
    let e = event(
        &f,
        "task.created",
        RiskClass::KnownRepair,
        Some("raw-conn-1"),
    )
    .unwrap();
    let key = IdempotencyKey::new("raw-conn-1").unwrap();
    f.store
        .write(|tx| {
            rikuri_ledger::append(tx, &e, now())?;
            rikuri_ledger::record(tx, &key, "c", "h", e.event_id, &json!({}), now())
        })
        .unwrap();

    let raw = rikuri_storage::rusqlite::Connection::open(f.store.path()).unwrap();
    // Neither the pragma nor foreign keys may be what refuses the rewrite.
    raw.pragma_update(None, "foreign_keys", false).unwrap();
    let (recursive, foreign): (i64, i64) = raw
        .query_row(
            "SELECT * FROM pragma_recursive_triggers, pragma_foreign_keys",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    assert_eq!(
        (recursive, foreign),
        (0, 0),
        "the raw connection must rely on neither recursive_triggers nor foreign keys"
    );
    for sql in [
        // explicit seq of an existing event, new event_id
        "INSERT OR REPLACE INTO events (seq, event_id, schema_version, recorded_at_ms, event_type,
            status, actor_id, subject_ids, input_refs, result_refs, metadata)
         SELECT seq, 'forged-event', schema_version, recorded_at_ms, 'task.deleted', 'failed',
            actor_id, '[]', '[]', '[]', '{}' FROM events",
        // existing event_id, seq omitted
        "REPLACE INTO events (event_id, schema_version, recorded_at_ms, event_type, status,
            actor_id, subject_ids, input_refs, result_refs, metadata)
         SELECT event_id, schema_version, recorded_at_ms, 'task.deleted', 'failed', actor_id,
            '[]', '[]', '[]', '{}' FROM events",
        "INSERT OR REPLACE INTO idempotency SELECT idempotency_key, command, request_hash,
            event_id, '{\"task_id\":\"forged\"}', recorded_at_ms FROM idempotency",
    ] {
        assert!(
            raw.execute(sql, []).is_err(),
            "allowed on a raw connection: {sql}"
        );
    }
    drop(raw);

    let still = f.store.read(|tx| rikuri_ledger::tail(tx, 10)).unwrap();
    assert_eq!(still.len(), 1);
    assert_eq!(still[0].event_type.as_str(), "task.created");
    let replay = f
        .store
        .read(|tx| rikuri_ledger::check(tx, &key, "c", "h"))
        .unwrap();
    match replay {
        IdempotencyCheck::Replay(r) => assert_eq!(r.response, json!({})),
        other => panic!("expected the original record to replay, got {other:?}"),
    }
}
