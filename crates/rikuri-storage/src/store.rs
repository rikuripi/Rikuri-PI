//! The store: one SQLite connection, one writer, explicit transactions.

use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Duration;

use rikuri_domain::{Clock, Timestamp};
use rusqlite::{Connection, TransactionBehavior};
use serde::Serialize;

use crate::StorageError;
use crate::migrate::{self, Migration, Migrations};

/// The storage component's own tables: the common object registry
/// (Architecture §21.1) holding every typed entity's current state.
pub(crate) const STORAGE_MIGRATIONS: Migrations = Migrations {
    component: "storage",
    list: &[Migration {
        version: 1,
        name: "object registry",
        sql: "
CREATE TABLE objects (
    id               TEXT    PRIMARY KEY NOT NULL,
    kind             TEXT    NOT NULL,
    version          INTEGER NOT NULL CHECK (version >= 1),
    schema_version   INTEGER NOT NULL CHECK (schema_version >= 1),
    privacy_class    TEXT    NOT NULL
        CHECK (privacy_class IN ('normal', 'private', 'local_only', 'secret')),
    deletion_unit_id TEXT,
    created_at_ms    INTEGER NOT NULL,
    updated_at_ms    INTEGER NOT NULL,
    body             TEXT    NOT NULL CHECK (json_valid(body))
) STRICT;
CREATE INDEX objects_by_kind ON objects (kind, id);
",
    }],
};

/// A transaction. Dereferences to the connection, so components that own
/// tables run their SQL inside the same transaction as everyone else.
pub struct Tx<'a> {
    inner: rusqlite::Transaction<'a>,
}

impl Deref for Tx<'_> {
    type Target = Connection;

    fn deref(&self) -> &Connection {
        &self.inner
    }
}

/// Health of the store, reported on absence of success as well as on
/// failure (Architecture §53.7). `last_success` is the last committed write
/// or completed read; a store that has never succeeded is not healthy.
#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
pub struct StoreHealth {
    pub last_attempt: Option<Timestamp>,
    pub last_success: Option<Timestamp>,
    pub last_error: Option<String>,
    pub consecutive_failures: u64,
    pub work_completed: u64,
}

/// The canonical SQLite store.
pub struct Store {
    conn: Mutex<Connection>,
    path: PathBuf,
    clock: Box<dyn Clock>,
    health: Mutex<StoreHealth>,
}

impl std::fmt::Debug for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Store")
            .field("path", &self.path)
            .finish_non_exhaustive()
    }
}

impl Store {
    /// Opens (creating if needed) the database at `path`, checks it, applies
    /// the storage component's migrations and then each of `components`.
    pub fn open(
        path: &Path,
        clock: Box<dyn Clock>,
        components: &[&Migrations],
    ) -> Result<Store, StorageError> {
        if let Some(dir) = path.parent() {
            create_private_dir(dir)?;
        }
        let mut conn = Connection::open(path)?;
        conn.busy_timeout(Duration::from_secs(5))?;
        let mode: String = conn.query_row("PRAGMA journal_mode = WAL", [], |r| r.get(0))?;
        if !mode.eq_ignore_ascii_case("wal") {
            return Err(StorageError::Corrupt {
                kind: "database",
                id: path.display().to_string(),
                reason: format!("journal mode is {mode}, WAL required"),
            });
        }
        // FULL: a committed transaction survives power loss, not only a
        // process crash. The kernel's promise is "committed means durable".
        conn.pragma_update(None, "synchronous", "FULL")?;
        conn.pragma_update(None, "foreign_keys", true)?;
        let check: String = conn.query_row("PRAGMA quick_check", [], |r| r.get(0))?;
        if check != "ok" {
            return Err(StorageError::Corrupt {
                kind: "database",
                id: path.display().to_string(),
                reason: format!("quick_check: {check}"),
            });
        }

        let mut all: Vec<&Migrations> = vec![&STORAGE_MIGRATIONS];
        all.extend_from_slice(components);
        migrate::apply_all(&mut conn, &all, clock.now())?;

        Ok(Store {
            conn: Mutex::new(conn),
            path: path.to_path_buf(),
            clock,
            health: Mutex::new(StoreHealth::default()),
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The current time from the store's clock.
    pub fn now(&self) -> Timestamp {
        self.clock.now()
    }

    /// Runs `f` in an IMMEDIATE (writer) transaction. Commits if `f` returns
    /// `Ok`; rolls back if it returns `Err` or the commit fails. Nothing `f`
    /// wrote is visible unless the commit succeeded.
    pub fn write<T, E>(&self, f: impl FnOnce(&Tx<'_>) -> Result<T, E>) -> Result<T, E>
    where
        E: From<StorageError>,
    {
        self.run(TransactionBehavior::Immediate, f)
    }

    /// Runs `f` in a read transaction: one consistent snapshot. Rolled back
    /// at the end; reads cannot change state.
    pub fn read<T, E>(&self, f: impl FnOnce(&Tx<'_>) -> Result<T, E>) -> Result<T, E>
    where
        E: From<StorageError>,
    {
        self.run(TransactionBehavior::Deferred, f)
    }

    fn run<T, E>(
        &self,
        behavior: TransactionBehavior,
        f: impl FnOnce(&Tx<'_>) -> Result<T, E>,
    ) -> Result<T, E>
    where
        E: From<StorageError>,
    {
        let started = self.clock.now();
        let result = (|| {
            let mut conn = self.conn.lock().map_err(|_| StorageError::Poisoned)?;
            let inner = conn
                .transaction_with_behavior(behavior)
                .map_err(StorageError::from)?;
            let tx = Tx { inner };
            let value = f(&tx)?;
            match behavior {
                TransactionBehavior::Deferred => tx.inner.rollback().map_err(StorageError::from)?,
                _ => tx.inner.commit().map_err(StorageError::from)?,
            }
            Ok::<T, E>(value)
        })();
        self.record(started, result.is_ok());
        result
    }

    fn record(&self, at: Timestamp, ok: bool) {
        if let Ok(mut h) = self.health.lock() {
            h.last_attempt = Some(at);
            if ok {
                h.last_success = Some(at);
                h.consecutive_failures = 0;
                h.work_completed += 1;
            } else {
                h.consecutive_failures += 1;
                h.last_error = Some("transaction failed or was rolled back".into());
            }
        }
    }

    /// A snapshot of the store's health evidence.
    pub fn health(&self) -> StoreHealth {
        self.health
            .lock()
            .map(|h| h.clone())
            .unwrap_or(StoreHealth {
                last_error: Some("health lock poisoned".into()),
                ..StoreHealth::default()
            })
    }

    /// The newest applied migration version of `component`.
    pub fn schema_version(&self, component: &str) -> Result<Option<u32>, StorageError> {
        let conn = self.conn.lock().map_err(|_| StorageError::Poisoned)?;
        migrate::applied_version(&conn, component)
    }
}

/// Creates the data directory readable only by the owner.
fn create_private_dir(dir: &Path) -> Result<(), StorageError> {
    if dir.as_os_str().is_empty() || dir.exists() {
        return Ok(());
    }
    let mut builder = std::fs::DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder.create(dir).map_err(|source| StorageError::Io {
        path: dir.display().to_string(),
        source,
    })
}
