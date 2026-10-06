//! The append-only event ledger (Stage Plan S00.06, ADR 0003).
//!
//! The ledger answers *what happened*; the object registry answers *what is
//! true now* (Architecture §8). A command that changes canonical state
//! updates the registry, appends one event and, for a consequential
//! command, records its idempotency key, all in one store transaction.
//!
//! Events reference content by ID and never embed it, so the ledger can stay
//! append-only while content remains forgettable (Architecture §4.11).
//! SQLite triggers refuse UPDATE and DELETE on events and idempotency rows,
//! so append-only holds even for code that bypasses this crate.

mod event;
mod idempotency;

pub use event::{
    Event, EventStatus, EventType, LedgerError, MAX_METADATA_BYTES, MAX_METADATA_STRING, NewEvent,
    append, by_task, get, replay, tail,
};
pub use idempotency::{
    IdempotencyCheck, IdempotencyKey, IdempotencyRecord, check, record, request_hash,
};

use rikuri_storage::{Migration, Migrations};

/// The ledger's tables. Registered with `Store::open`.
pub const MIGRATIONS: Migrations = Migrations {
    component: "ledger",
    list: &[Migration {
        version: 1,
        name: "events and idempotency",
        sql: "
CREATE TABLE events (
    seq                  INTEGER PRIMARY KEY AUTOINCREMENT,
    event_id             TEXT    NOT NULL UNIQUE,
    schema_version       INTEGER NOT NULL CHECK (schema_version >= 1),
    recorded_at_ms       INTEGER NOT NULL,
    event_type           TEXT    NOT NULL,
    status               TEXT    NOT NULL CHECK (status IN ('succeeded', 'failed', 'rejected')),
    actor_id             TEXT    NOT NULL,
    project_id           TEXT,
    task_id              TEXT,
    run_id               TEXT,
    subject_ids          TEXT    NOT NULL CHECK (json_valid(subject_ids) AND json_type(subject_ids) = 'array'),
    input_refs           TEXT    NOT NULL CHECK (json_valid(input_refs) AND json_type(input_refs) = 'array'),
    result_refs          TEXT    NOT NULL CHECK (json_valid(result_refs) AND json_type(result_refs) = 'array'),
    security_decision_id TEXT,
    cost_record_id       TEXT,
    idempotency_key      TEXT,
    caused_by_event_id   TEXT REFERENCES events (event_id),
    metadata             TEXT    NOT NULL
        CHECK (json_valid(metadata) AND json_type(metadata) = 'object' AND length(metadata) <= 4096)
) STRICT;
CREATE INDEX events_by_task ON events (task_id, seq);
CREATE TRIGGER events_no_update BEFORE UPDATE ON events
    BEGIN SELECT RAISE(ABORT, 'ledger events are append-only'); END;
CREATE TRIGGER events_no_delete BEFORE DELETE ON events
    BEGIN SELECT RAISE(ABORT, 'ledger events are append-only'); END;

CREATE TABLE idempotency (
    idempotency_key TEXT    PRIMARY KEY NOT NULL,
    command         TEXT    NOT NULL,
    request_hash    TEXT    NOT NULL,
    event_id        TEXT    NOT NULL REFERENCES events (event_id),
    response        TEXT    NOT NULL CHECK (json_valid(response) AND length(response) <= 4096),
    recorded_at_ms  INTEGER NOT NULL
) STRICT;
CREATE TRIGGER idempotency_no_update BEFORE UPDATE ON idempotency
    BEGIN SELECT RAISE(ABORT, 'idempotency records are append-only'); END;
CREATE TRIGGER idempotency_no_delete BEFORE DELETE ON idempotency
    BEGIN SELECT RAISE(ABORT, 'idempotency records are append-only'); END;
",
    }],
};
