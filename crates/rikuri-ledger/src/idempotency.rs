//! Idempotency records (Architecture §8, §41.1).
//!
//! A consequential command carries a caller-chosen key. The first time the
//! key is seen, the command runs and its response is stored with the key in
//! the same transaction as its effects. A retry with the same key and the
//! same request gets the stored response and changes nothing; the same key
//! with a different request is refused, never silently re-applied.
//!
//! Records can never be deleted, so a stored response holds only small
//! facts (IDs, versions, codes), validated like event metadata; content
//! stays behind a reference where it remains forgettable.

use rikuri_domain::{EventId, Timestamp, bounded};
use rikuri_storage::Tx;
use rikuri_storage::rusqlite::{self, ErrorCode, OptionalExtension, params};
use serde::Serialize;
use serde_json::Value;

use crate::LedgerError;
use crate::event::validate_metadata;

/// Longest command name accepted, in bytes.
pub const MAX_COMMAND_BYTES: usize = 64;

/// A command name is 1–64 bytes of `a–z 0–9 . _ -`, e.g. `task.create`.
fn validate_command(command: &str) -> Result<(), LedgerError> {
    let ok = !command.is_empty()
        && command.len() <= MAX_COMMAND_BYTES
        && command.bytes().all(|b| {
            b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'.' | b'_' | b'-')
        });
    if ok {
        Ok(())
    } else {
        Err(LedgerError::Invalid(format!(
            "idempotency command \"{}\" must be 1-{MAX_COMMAND_BYTES} bytes of a-z 0-9 . _ -",
            bounded(command)
        )))
    }
}

/// A caller-chosen idempotency key: 8–128 characters from
/// `A–Z a–z 0–9 . _ : -`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    pub fn new(key: &str) -> Result<IdempotencyKey, LedgerError> {
        let ok_len = (8..=128).contains(&key.len());
        let ok_chars = key
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b':' | b'-'));
        if ok_len && ok_chars {
            Ok(IdempotencyKey(key.to_owned()))
        } else {
            Err(LedgerError::Invalid(
                "idempotency key must be 8–128 characters from A–Z a–z 0–9 . _ : -".into(),
            ))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Fingerprint of a command and its normalized parameters. `serde_json`
/// maps keep keys sorted, so equal requests hash equally regardless of the
/// order a client sent fields in.
pub fn request_hash(command: &str, params: &Value) -> String {
    let mut h = blake3::Hasher::new();
    h.update(command.as_bytes());
    h.update(&[0]);
    h.update(params.to_string().as_bytes());
    h.finalize().to_hex().to_string()
}

/// A stored idempotency record.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct IdempotencyRecord {
    pub key: String,
    pub command: String,
    pub request_hash: String,
    pub event_id: EventId,
    pub response: Value,
    pub recorded_at: Timestamp,
}

/// What a key means for a request about to run.
#[derive(Debug, Clone, PartialEq)]
pub enum IdempotencyCheck {
    /// Never seen: run the command.
    Fresh,
    /// Seen with this same request: return the stored response, do nothing.
    Replay(IdempotencyRecord),
    /// Seen with a different request: refuse.
    Conflict(IdempotencyRecord),
}

/// Looks up `key` for `command` with `hash`.
pub fn check(
    tx: &Tx<'_>,
    key: &IdempotencyKey,
    command: &str,
    hash: &str,
) -> Result<IdempotencyCheck, LedgerError> {
    let row = tx
        .query_row(
            "SELECT idempotency_key, command, request_hash, event_id, response, recorded_at_ms
               FROM idempotency WHERE idempotency_key = ?1",
            params![key.as_str()],
            |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                    r.get::<_, String>(3)?,
                    r.get::<_, String>(4)?,
                    r.get::<_, i64>(5)?,
                ))
            },
        )
        .optional()?;
    let Some((key, stored_command, request_hash, event_id, response, at)) = row else {
        return Ok(IdempotencyCheck::Fresh);
    };
    let record = IdempotencyRecord {
        event_id: event_id
            .parse()
            .map_err(|e| LedgerError::Corrupt(format!("idempotency {key}: {e}")))?,
        response: serde_json::from_str(&response)
            .map_err(|e| LedgerError::Corrupt(format!("idempotency {key}: {e}")))?,
        recorded_at: Timestamp(at),
        command: stored_command,
        request_hash,
        key,
    };
    if record.command == command && record.request_hash == hash {
        Ok(IdempotencyCheck::Replay(record))
    } else {
        Ok(IdempotencyCheck::Conflict(record))
    }
}

/// A mark of the newest idempotency record (none when there is none yet),
/// for `records_after`: its rowid. Records are append-only, so every record
/// written later has a larger one. The kernel takes it when a command
/// begins and checks the records the command wrote (E1, ADR 0005).
pub fn record_mark(tx: &Tx<'_>) -> Result<Option<i64>, LedgerError> {
    Ok(tx.query_row("SELECT max(rowid) FROM idempotency", [], |r| r.get(0))?)
}

/// The idempotency records written after `mark` (every record when `None`),
/// oldest first, read through the rowid.
pub fn records_after(
    tx: &Tx<'_>,
    mark: Option<i64>,
) -> Result<Vec<IdempotencyRecord>, LedgerError> {
    let mut stmt = tx.prepare(
        "SELECT idempotency_key, command, request_hash, event_id, response, recorded_at_ms
           FROM idempotency WHERE rowid > ?1 ORDER BY rowid",
    )?;
    let rows = stmt.query_map(params![mark.unwrap_or(0)], |r| {
        Ok((
            r.get::<_, String>(0)?,
            r.get::<_, String>(1)?,
            r.get::<_, String>(2)?,
            r.get::<_, String>(3)?,
            r.get::<_, String>(4)?,
            r.get::<_, i64>(5)?,
        ))
    })?;
    rows.map(|row| {
        let (key, command, request_hash, event_id, response, at) = row?;
        Ok(IdempotencyRecord {
            event_id: event_id
                .parse()
                .map_err(|e| LedgerError::Corrupt(format!("idempotency {key}: {e}")))?,
            response: serde_json::from_str(&response)
                .map_err(|e| LedgerError::Corrupt(format!("idempotency {key}: {e}")))?,
            recorded_at: Timestamp(at),
            command,
            request_hash,
            key,
        })
    })
    .collect()
}

/// Records `key` with the response to return on replay, in the same
/// transaction as the command's effects and its event. The command must be a
/// short name and the response an object of small facts (see the module
/// documentation).
pub fn record(
    tx: &Tx<'_>,
    key: &IdempotencyKey,
    command: &str,
    hash: &str,
    event_id: EventId,
    response: &Value,
    now: Timestamp,
) -> Result<(), LedgerError> {
    validate_command(command)?;
    let Value::Object(fields) = response else {
        return Err(LedgerError::Invalid(
            "an idempotency response must be an object of small facts".into(),
        ));
    };
    let response = validate_metadata(fields)?;
    let result = tx.execute(
        "INSERT INTO idempotency (idempotency_key, command, request_hash, event_id, response, recorded_at_ms)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![key.as_str(), command, hash, event_id.to_string(), response, now.millis()],
    );
    match result {
        Ok(_) => Ok(()),
        Err(rusqlite::Error::SqliteFailure(e, msg)) if e.code == ErrorCode::ConstraintViolation => {
            let detail = msg.unwrap_or_default();
            if detail.contains("FOREIGN KEY") {
                Err(LedgerError::MissingReference {
                    field: "event_id",
                    id: event_id.to_string(),
                })
            } else if detail.contains("UNIQUE")
                || detail.contains("PRIMARY KEY")
                || detail.contains("append-only")
            {
                // the schema's append-only guard refuses an insert on an
                // existing key before the primary-key check sees it
                Err(LedgerError::IdempotencyKeyTaken(key.as_str().to_owned()))
            } else {
                Err(LedgerError::Invalid(format!(
                    "idempotency record refused: {detail}"
                )))
            }
        }
        Err(e) => Err(e.into()),
    }
}
