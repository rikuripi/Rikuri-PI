//! `Repository<T>`: current state of typed entities with optimistic
//! versioning (ADR 0002).
//!
//! Each write names the version it read; if the stored version differs the
//! write is refused with `VersionConflict`, so two writers can never both
//! commit on the same base. After every write the stored body is read back
//! inside the same transaction and compared with what was meant to be
//! written (Transactional State Evolution: durable write → read back and
//! verify → atomic promote at commit).

use std::fmt::Display;
use std::marker::PhantomData;
use std::str::FromStr;

use rikuri_domain::{DeletionUnitId, IdError, PrivacyClass, SchemaVersion, Timestamp, bounded};
use rusqlite::{ErrorCode, OptionalExtension, params};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::{StorageError, Tx};

/// A typed canonical entity stored in the object registry.
pub trait Entity: Serialize + DeserializeOwned {
    /// The entity's typed ID (from `rikuri-domain`).
    type Id: Copy + Display + FromStr<Err = IdError>;
    /// Registry kind; equal to the ID prefix.
    const KIND: &'static str;
    /// The body's schema version. Reading a different version is refused
    /// until a migration converts it.
    const SCHEMA: SchemaVersion;

    fn id(&self) -> Self::Id;
    fn privacy_class(&self) -> PrivacyClass;
    fn deletion_unit(&self) -> Option<DeletionUnitId> {
        None
    }
}

/// An entity with its registry metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Versioned<T> {
    pub value: T,
    pub version: u64,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

/// Versioned current-state access to one entity type.
pub trait Repository<T: Entity> {
    fn get(&self, tx: &Tx<'_>, id: T::Id) -> Result<Option<Versioned<T>>, StorageError>;
    /// Stores a new entity at version 1. Refuses an ID that exists.
    fn insert(&self, tx: &Tx<'_>, value: &T, now: Timestamp) -> Result<Versioned<T>, StorageError>;
    /// Replaces the entity if it is still at `expected_version`; the result
    /// is at `expected_version + 1`.
    fn update(
        &self,
        tx: &Tx<'_>,
        value: &T,
        expected_version: u64,
        now: Timestamp,
    ) -> Result<Versioned<T>, StorageError>;
    /// Up to `limit` entities of this kind, in ID (creation) order.
    fn list(&self, tx: &Tx<'_>, limit: u32) -> Result<Vec<Versioned<T>>, StorageError>;
}

/// The SQLite implementation over the `objects` registry table.
pub struct SqliteRepository<T> {
    _entity: PhantomData<fn() -> T>,
}

impl<T> Default for SqliteRepository<T> {
    fn default() -> Self {
        SqliteRepository {
            _entity: PhantomData,
        }
    }
}

impl<T> SqliteRepository<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

struct Row {
    id: String,
    kind: String,
    version: i64,
    schema_version: i64,
    privacy_class: String,
    deletion_unit_id: Option<String>,
    created_at_ms: i64,
    updated_at_ms: i64,
    body: String,
}

const SELECT: &str = "SELECT id, kind, version, schema_version, privacy_class, deletion_unit_id,
    created_at_ms, updated_at_ms, body FROM objects";

fn read_row(r: &rusqlite::Row<'_>) -> rusqlite::Result<Row> {
    Ok(Row {
        id: r.get(0)?,
        kind: r.get(1)?,
        version: r.get(2)?,
        schema_version: r.get(3)?,
        privacy_class: r.get(4)?,
        deletion_unit_id: r.get(5)?,
        created_at_ms: r.get(6)?,
        updated_at_ms: r.get(7)?,
        body: r.get(8)?,
    })
}

fn to_body<T: Entity>(value: &T) -> Result<String, StorageError> {
    serde_json::to_string(value).map_err(|e| StorageError::Corrupt {
        kind: T::KIND,
        id: value.id().to_string(),
        reason: format!("cannot serialize: {e}"),
    })
}

/// Decodes a stored row, trusting nothing in it: kind, body schema, ID,
/// privacy class and deletion unit must all agree with the body. Stored text
/// that appears in an error is bounded.
fn decode<T: Entity>(row: Row) -> Result<Versioned<T>, StorageError> {
    let corrupt = |reason: String| StorageError::Corrupt {
        kind: T::KIND,
        id: bounded(&row.id),
        reason,
    };
    if row.kind != T::KIND {
        return Err(corrupt(format!("row kind is \"{}\"", bounded(&row.kind))));
    }
    if row.schema_version != i64::from(T::SCHEMA.get()) {
        return Err(corrupt(format!(
            "body schema version {} is not {}; a migration is required",
            row.schema_version,
            T::SCHEMA.get()
        )));
    }
    let version = u64::try_from(row.version).map_err(|_| corrupt("negative version".into()))?;
    let value: T = serde_json::from_str(&row.body)
        .map_err(|e| corrupt(format!("body does not parse: {}", bounded(&e.to_string()))))?;
    if value.id().to_string() != row.id {
        return Err(corrupt(format!("body carries ID {}", value.id())));
    }
    let class = value.privacy_class();
    if row.privacy_class != class.as_str() {
        return Err(corrupt(format!(
            "privacy class column is \"{}\" but the body is {class}",
            bounded(&row.privacy_class)
        )));
    }
    let unit = value.deletion_unit().map(|u| u.to_string());
    if row.deletion_unit_id != unit {
        return Err(corrupt(format!(
            "deletion unit column is {} but the body's is {}",
            row.deletion_unit_id
                .as_deref()
                .map_or("none".into(), bounded),
            unit.as_deref().unwrap_or("none")
        )));
    }
    Ok(Versioned {
        value,
        version,
        created_at: Timestamp(row.created_at_ms),
        updated_at: Timestamp(row.updated_at_ms),
    })
}

/// Reads the row back inside the transaction and checks it holds exactly
/// what was written.
fn read_back<T: Entity>(
    tx: &Tx<'_>,
    id: &str,
    body: &str,
    version: u64,
) -> Result<Versioned<T>, StorageError> {
    let row = tx
        .query_row(&format!("{SELECT} WHERE id = ?1"), params![id], read_row)
        .optional()?
        .ok_or_else(|| StorageError::Corrupt {
            kind: T::KIND,
            id: id.to_owned(),
            reason: "row missing on read-back".into(),
        })?;
    if row.body != body {
        return Err(StorageError::Corrupt {
            kind: T::KIND,
            id: id.to_owned(),
            reason: "read-back body differs from what was written".into(),
        });
    }
    let stored = decode::<T>(row)?;
    if stored.version != version {
        return Err(StorageError::Corrupt {
            kind: T::KIND,
            id: id.to_owned(),
            reason: format!("read-back version {} is not {version}", stored.version),
        });
    }
    Ok(stored)
}

impl<T: Entity> Repository<T> for SqliteRepository<T> {
    fn get(&self, tx: &Tx<'_>, id: T::Id) -> Result<Option<Versioned<T>>, StorageError> {
        tx.query_row(
            &format!("{SELECT} WHERE id = ?1"),
            params![id.to_string()],
            read_row,
        )
        .optional()?
        .map(decode)
        .transpose()
    }

    fn insert(&self, tx: &Tx<'_>, value: &T, now: Timestamp) -> Result<Versioned<T>, StorageError> {
        let id = value.id().to_string();
        let body = to_body(value)?;
        let inserted = tx.execute(
            "INSERT INTO objects (id, kind, version, schema_version, privacy_class,
                                  deletion_unit_id, created_at_ms, updated_at_ms, body)
             VALUES (?1, ?2, 1, ?3, ?4, ?5, ?6, ?6, ?7)",
            params![
                id,
                T::KIND,
                T::SCHEMA.get(),
                value.privacy_class().as_str(),
                value.deletion_unit().map(|d| d.to_string()),
                now.millis(),
                body
            ],
        );
        match inserted {
            Ok(_) => {}
            Err(rusqlite::Error::SqliteFailure(e, _))
                if e.code == ErrorCode::ConstraintViolation =>
            {
                return Err(StorageError::AlreadyExists { kind: T::KIND, id });
            }
            Err(e) => return Err(e.into()),
        }
        read_back(tx, &id, &body, 1)
    }

    fn update(
        &self,
        tx: &Tx<'_>,
        value: &T,
        expected_version: u64,
        now: Timestamp,
    ) -> Result<Versioned<T>, StorageError> {
        let id = value.id().to_string();
        let body = to_body(value)?;
        let expected =
            i64::try_from(expected_version).map_err(|_| StorageError::VersionConflict {
                kind: T::KIND,
                id: id.clone(),
                expected: expected_version,
                actual: 0,
            })?;
        let changed = tx.execute(
            "UPDATE objects
                SET version = version + 1, schema_version = ?1, privacy_class = ?2,
                    deletion_unit_id = ?3, updated_at_ms = ?4, body = ?5
              WHERE id = ?6 AND kind = ?7 AND version = ?8",
            params![
                T::SCHEMA.get(),
                value.privacy_class().as_str(),
                value.deletion_unit().map(|d| d.to_string()),
                now.millis(),
                body,
                id,
                T::KIND,
                expected
            ],
        )?;
        if changed == 1 {
            return read_back(tx, &id, &body, expected_version + 1);
        }
        let actual: Option<i64> = tx
            .query_row(
                "SELECT version FROM objects WHERE id = ?1 AND kind = ?2",
                params![id, T::KIND],
                |r| r.get(0),
            )
            .optional()?;
        match actual {
            None => Err(StorageError::NotFound { kind: T::KIND, id }),
            Some(actual) => Err(StorageError::VersionConflict {
                kind: T::KIND,
                id,
                expected: expected_version,
                actual: u64::try_from(actual).unwrap_or(0),
            }),
        }
    }

    fn list(&self, tx: &Tx<'_>, limit: u32) -> Result<Vec<Versioned<T>>, StorageError> {
        let mut stmt = tx.prepare(&format!("{SELECT} WHERE kind = ?1 ORDER BY id LIMIT ?2"))?;
        let rows = stmt.query_map(params![T::KIND, limit], read_row)?;
        rows.map(|r| decode(r?)).collect()
    }
}
