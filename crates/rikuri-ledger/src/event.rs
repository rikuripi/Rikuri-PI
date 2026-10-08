//! Events: append, read, replay.

use rikuri_domain::{
    ActorId, AnyId, CostRecordId, EventId, IdError, ProjectId, RiskClass, RunId, SchemaVersion,
    SecurityDecisionId, TaskId, Timestamp, bounded, wire_enum,
};
use rikuri_storage::rusqlite::{self, OptionalExtension, params};
use rikuri_storage::{StorageError, Tx};
use serde::Serialize;
use serde_json::{Map, Value};
use thiserror::Error;

use crate::idempotency::IdempotencyKey;

/// The event record's schema version.
pub const EVENT_SCHEMA: SchemaVersion = SchemaVersion::V1;
/// Upper bound on serialized metadata (also enforced by the table).
pub const MAX_METADATA_BYTES: usize = 4096;
/// Upper bound, in characters, on any one metadata string or key. Metadata
/// carries small facts (counts, codes, names of things); content lives
/// behind a reference.
pub const MAX_METADATA_STRING: usize = 200;

/// Every way an append or read can fail.
#[derive(Debug, Error)]
pub enum LedgerError {
    #[error(transparent)]
    Storage(#[from] StorageError),
    #[error("invalid event: {0}")]
    Invalid(String),
    #[error("event {event_type} is consequential and needs an idempotency key")]
    IdempotencyKeyRequired { event_type: String },
    #[error("{field} references {id}, which does not exist")]
    MissingReference { field: &'static str, id: String },
    #[error("event {0} already exists")]
    DuplicateEvent(String),
    #[error("idempotency key {0} is already recorded")]
    IdempotencyKeyTaken(String),
    #[error("stored ledger row is invalid: {0}")]
    Corrupt(String),
}

impl From<rusqlite::Error> for LedgerError {
    fn from(e: rusqlite::Error) -> Self {
        LedgerError::Storage(StorageError::Sqlite(e))
    }
}

wire_enum! {
    /// The outcome of the action an event records.
    pub enum EventStatus {
        Succeeded => "succeeded",
        Failed => "failed",
        Rejected => "rejected",
    }
}

/// A dotted lowercase event name such as `task.created`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct EventType(String);

impl EventType {
    pub fn new(name: &str) -> Result<EventType, LedgerError> {
        let parts: Vec<&str> = name.split('.').collect();
        let word_ok = |w: &str| {
            w.starts_with(|c: char| c.is_ascii_lowercase())
                && w.bytes().all(|b| b.is_ascii_lowercase() || b == b'_')
        };
        if name.len() > 64 || parts.len() < 2 || !parts.iter().all(|w| word_ok(w)) {
            return Err(LedgerError::Invalid(format!(
                "event type \"{}\" must be dotted lowercase words, e.g. task.created",
                bounded(name)
            )));
        }
        Ok(EventType(name.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// An event to append. The caller supplies the ID (from an `IdSource`) so
/// a retried command can be traced, and states the operation's risk so the
/// ledger can require an idempotency key for consequential effects.
#[derive(Debug, Clone)]
pub struct NewEvent {
    pub event_id: EventId,
    pub event_type: EventType,
    pub status: EventStatus,
    pub risk: RiskClass,
    pub actor_id: ActorId,
    pub project_id: Option<ProjectId>,
    pub task_id: Option<TaskId>,
    pub run_id: Option<RunId>,
    pub subject_ids: Vec<AnyId>,
    pub input_refs: Vec<AnyId>,
    pub result_refs: Vec<AnyId>,
    pub security_decision_id: Option<SecurityDecisionId>,
    pub cost_record_id: Option<CostRecordId>,
    pub idempotency_key: Option<IdempotencyKey>,
    pub caused_by: Option<EventId>,
    pub metadata: Map<String, Value>,
}

/// A stored event. `seq` is the ledger's authoritative order.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Event {
    pub seq: u64,
    pub event_id: EventId,
    pub schema_version: SchemaVersion,
    pub recorded_at: Timestamp,
    pub event_type: String,
    pub status: EventStatus,
    pub actor_id: ActorId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<ProjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<TaskId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<RunId>,
    pub subject_ids: Vec<AnyId>,
    pub input_refs: Vec<AnyId>,
    pub result_refs: Vec<AnyId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_decision_id: Option<SecurityDecisionId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_record_id: Option<CostRecordId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caused_by: Option<EventId>,
    pub metadata: Map<String, Value>,
}

fn is_small_scalar(v: &Value) -> bool {
    match v {
        Value::Null | Value::Bool(_) | Value::Number(_) => true,
        Value::String(s) => s.chars().count() <= MAX_METADATA_STRING,
        _ => false,
    }
}

/// Validates a small-facts object (event metadata, idempotency responses):
/// keys and strings of at most [`MAX_METADATA_STRING`] characters, values
/// that are small scalars or lists of them, at most [`MAX_METADATA_BYTES`]
/// serialized. Returns the serialized form.
pub(crate) fn validate_metadata(m: &Map<String, Value>) -> Result<String, LedgerError> {
    for (k, v) in m {
        if k.chars().count() > MAX_METADATA_STRING {
            return Err(LedgerError::Invalid(format!(
                "metadata key \"{}\" is longer than {MAX_METADATA_STRING} characters",
                bounded(k)
            )));
        }
        let ok = match v {
            Value::Array(items) => items.iter().all(is_small_scalar),
            other => is_small_scalar(other),
        };
        if !ok {
            return Err(LedgerError::Invalid(format!(
                "metadata \"{}\" must be a small scalar or a list of them; \
                 reference content by ID instead of embedding it",
                bounded(k)
            )));
        }
    }
    let text = serde_json::to_string(m).map_err(|e| LedgerError::Invalid(e.to_string()))?;
    if text.len() > MAX_METADATA_BYTES {
        return Err(LedgerError::Invalid(format!(
            "metadata is {} bytes; the limit is {MAX_METADATA_BYTES}",
            text.len()
        )));
    }
    Ok(text)
}

fn ids_json(ids: &[AnyId]) -> Result<String, LedgerError> {
    serde_json::to_string(ids).map_err(|e| LedgerError::Invalid(e.to_string()))
}

/// Appends `e` inside the caller's transaction. Validates that a
/// consequential event carries an idempotency key, that metadata holds no
/// content, and that `caused_by` names an existing event.
pub fn append(tx: &Tx<'_>, e: &NewEvent, now: Timestamp) -> Result<Event, LedgerError> {
    if e.risk.is_consequential() && e.idempotency_key.is_none() {
        return Err(LedgerError::IdempotencyKeyRequired {
            event_type: e.event_type.as_str().to_owned(),
        });
    }
    let metadata = validate_metadata(&e.metadata)?;
    if let Some(parent) = e.caused_by
        && get(tx, parent)?.is_none()
    {
        return Err(LedgerError::MissingReference {
            field: "caused_by",
            id: parent.to_string(),
        });
    }
    if get(tx, e.event_id)?.is_some() {
        return Err(LedgerError::DuplicateEvent(e.event_id.to_string()));
    }
    tx.execute(
        "INSERT INTO events (event_id, schema_version, recorded_at_ms, event_type, status,
            actor_id, project_id, task_id, run_id, subject_ids, input_refs, result_refs,
            security_decision_id, cost_record_id, idempotency_key, caused_by_event_id, metadata)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
        params![
            e.event_id.to_string(),
            EVENT_SCHEMA.get(),
            now.millis(),
            e.event_type.as_str(),
            e.status.as_str(),
            e.actor_id.to_string(),
            e.project_id.map(|i| i.to_string()),
            e.task_id.map(|i| i.to_string()),
            e.run_id.map(|i| i.to_string()),
            ids_json(&e.subject_ids)?,
            ids_json(&e.input_refs)?,
            ids_json(&e.result_refs)?,
            e.security_decision_id.map(|i| i.to_string()),
            e.cost_record_id.map(|i| i.to_string()),
            e.idempotency_key.as_ref().map(|k| k.as_str().to_owned()),
            e.caused_by.map(|i| i.to_string()),
            metadata,
        ],
    )?;
    get(tx, e.event_id)?
        .ok_or_else(|| LedgerError::Corrupt("appended event missing on read-back".into()))
}

const SELECT: &str = "SELECT seq, event_id, schema_version, recorded_at_ms, event_type, status,
    actor_id, project_id, task_id, run_id, subject_ids, input_refs, result_refs,
    security_decision_id, cost_record_id, idempotency_key, caused_by_event_id, metadata FROM events";

struct RawEvent {
    seq: i64,
    event_id: String,
    schema_version: i64,
    recorded_at_ms: i64,
    event_type: String,
    status: String,
    actor_id: String,
    project_id: Option<String>,
    task_id: Option<String>,
    run_id: Option<String>,
    subject_ids: String,
    input_refs: String,
    result_refs: String,
    security_decision_id: Option<String>,
    cost_record_id: Option<String>,
    idempotency_key: Option<String>,
    caused_by: Option<String>,
    metadata: String,
}

fn raw(r: &rusqlite::Row<'_>) -> rusqlite::Result<RawEvent> {
    Ok(RawEvent {
        seq: r.get(0)?,
        event_id: r.get(1)?,
        schema_version: r.get(2)?,
        recorded_at_ms: r.get(3)?,
        event_type: r.get(4)?,
        status: r.get(5)?,
        actor_id: r.get(6)?,
        project_id: r.get(7)?,
        task_id: r.get(8)?,
        run_id: r.get(9)?,
        subject_ids: r.get(10)?,
        input_refs: r.get(11)?,
        result_refs: r.get(12)?,
        security_decision_id: r.get(13)?,
        cost_record_id: r.get(14)?,
        idempotency_key: r.get(15)?,
        caused_by: r.get(16)?,
        metadata: r.get(17)?,
    })
}

fn opt_id<T: std::str::FromStr<Err = IdError>>(v: Option<&str>) -> Result<Option<T>, IdError> {
    v.map(str::parse).transpose()
}

fn corrupt(seq: i64, what: &str, e: impl std::fmt::Display) -> LedgerError {
    LedgerError::Corrupt(format!("seq {seq}: {what}: {e}"))
}

fn ids(seq: i64, text: &str, what: &str) -> Result<Vec<AnyId>, LedgerError> {
    serde_json::from_str(text).map_err(|e| corrupt(seq, what, e))
}

fn decode(r: RawEvent) -> Result<Event, LedgerError> {
    let seq = r.seq;
    let bad = |what: &'static str| move |e: IdError| corrupt(seq, what, e);
    Ok(Event {
        seq: u64::try_from(seq).map_err(|e| corrupt(seq, "seq", e))?,
        event_id: r.event_id.parse().map_err(bad("event_id"))?,
        schema_version: u32::try_from(r.schema_version)
            .ok()
            .and_then(SchemaVersion::new)
            .ok_or_else(|| corrupt(seq, "schema_version", r.schema_version))?,
        recorded_at: Timestamp(r.recorded_at_ms),
        status: r.status.parse().map_err(|e| corrupt(seq, "status", e))?,
        actor_id: r.actor_id.parse().map_err(bad("actor_id"))?,
        project_id: opt_id(r.project_id.as_deref()).map_err(bad("project_id"))?,
        task_id: opt_id(r.task_id.as_deref()).map_err(bad("task_id"))?,
        run_id: opt_id(r.run_id.as_deref()).map_err(bad("run_id"))?,
        subject_ids: ids(seq, &r.subject_ids, "subject_ids")?,
        input_refs: ids(seq, &r.input_refs, "input_refs")?,
        result_refs: ids(seq, &r.result_refs, "result_refs")?,
        security_decision_id: opt_id(r.security_decision_id.as_deref())
            .map_err(bad("security_decision_id"))?,
        cost_record_id: opt_id(r.cost_record_id.as_deref()).map_err(bad("cost_record_id"))?,
        caused_by: opt_id(r.caused_by.as_deref()).map_err(bad("caused_by"))?,
        metadata: serde_json::from_str(&r.metadata).map_err(|e| corrupt(seq, "metadata", e))?,
        idempotency_key: r.idempotency_key,
        event_type: r.event_type,
    })
}

/// The event with this ID, if any.
pub fn get(tx: &Tx<'_>, id: EventId) -> Result<Option<Event>, LedgerError> {
    tx.query_row(
        &format!("{SELECT} WHERE event_id = ?1"),
        params![id.to_string()],
        raw,
    )
    .optional()?
    .map(decode)
    .transpose()
}

/// The last `limit` events, oldest first.
pub fn tail(tx: &Tx<'_>, limit: u32) -> Result<Vec<Event>, LedgerError> {
    let mut stmt = tx.prepare(&format!(
        "SELECT * FROM ({SELECT} ORDER BY seq DESC LIMIT ?1) ORDER BY seq ASC"
    ))?;
    let rows = stmt.query_map(params![limit], raw)?;
    rows.map(|r| decode(r?)).collect()
}

/// Every event of one task, in ledger order.
pub fn by_task(tx: &Tx<'_>, task: TaskId) -> Result<Vec<Event>, LedgerError> {
    let mut stmt = tx.prepare(&format!("{SELECT} WHERE task_id = ?1 ORDER BY seq"))?;
    let rows = stmt.query_map(params![task.to_string()], raw)?;
    rows.map(|r| decode(r?)).collect()
}

/// Feeds every event to `f` in ledger order; returns how many were seen.
/// The basis for rebuilding or checking state from history.
pub fn replay<E: From<LedgerError>>(
    tx: &Tx<'_>,
    mut f: impl FnMut(&Event) -> Result<(), E>,
) -> Result<u64, E> {
    let mut stmt = tx
        .prepare(&format!("{SELECT} ORDER BY seq"))
        .map_err(LedgerError::from)?;
    let rows = stmt.query_map([], raw).map_err(LedgerError::from)?;
    let mut n = 0;
    for r in rows {
        let event = decode(r.map_err(LedgerError::from)?)?;
        f(&event)?;
        n += 1;
    }
    Ok(n)
}
