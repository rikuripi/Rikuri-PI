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
    ///
    /// On Unix the data directory and database are owner-only: a missing
    /// directory is created 0700 and a missing database 0600 (SQLite gives
    /// its `-wal` and `-shm` files the database's permissions), and an
    /// existing directory or database that group or others can access is
    /// refused rather than used or silently re-permissioned. Other platforms
    /// do not enforce this yet (Windows support arrives at product Stage 9).
    pub fn open(
        path: &Path,
        clock: Box<dyn Clock>,
        components: &[&Migrations],
    ) -> Result<Store, StorageError> {
        let dir = match path.parent() {
            Some(d) if !d.as_os_str().is_empty() => d,
            _ => Path::new("."),
        };
        ensure_private_dir(dir)?;
        ensure_private_file(path)?;
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
        // Defense in depth for append-only tables: REPLACE then fires DELETE
        // triggers on this connection. The tables' own BEFORE INSERT guards
        // hold on every connection.
        conn.pragma_update(None, "recursive_triggers", true)?;
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

fn io_error(path: &Path, source: std::io::Error) -> StorageError {
    StorageError::Io {
        path: path.display().to_string(),
        source,
    }
}

/// Refuses `path` if group or others have any access to it (Unix).
#[cfg(unix)]
fn refuse_if_shared(path: &Path, what: &str) -> Result<(), StorageError> {
    use std::os::unix::fs::PermissionsExt;
    let mode = std::fs::metadata(path)
        .map_err(|e| io_error(path, e))?
        .permissions()
        .mode()
        & 0o777;
    if mode & 0o077 != 0 {
        return Err(io_error(
            path,
            std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                format!(
                    "the {what} has mode {mode:o}, so group or others can access it; \
                     Rikuri requires owner-only (chmod {})",
                    if what == "data directory" {
                        "700"
                    } else {
                        "600"
                    }
                ),
            ),
        ));
    }
    Ok(())
}

/// Creates the data directory owner-only, or checks an existing one. The
/// mode is checked after creating too: a recursive create succeeds when the
/// directory appeared in the meantime, possibly with a looser mode.
fn ensure_private_dir(dir: &Path) -> Result<(), StorageError> {
    if !dir.exists() {
        let mut builder = std::fs::DirBuilder::new();
        builder.recursive(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }
        builder.create(dir).map_err(|e| io_error(dir, e))?;
    }
    #[cfg(unix)]
    refuse_if_shared(dir, "data directory")?;
    Ok(())
}

/// Creates the database file owner-only before SQLite opens it, or checks
/// an existing one. SQLite gives the WAL and shared-memory files the same
/// permissions as the database.
fn ensure_private_file(path: &Path) -> Result<(), StorageError> {
    if path.exists() {
        #[cfg(unix)]
        refuse_if_shared(path, "database file")?;
        return Ok(());
    }
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    match options.open(path) {
        Ok(_) => Ok(()),
        // created between the check and the open: check what is there
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            #[cfg(unix)]
            refuse_if_shared(path, "database file")?;
            Ok(())
        }
        Err(e) => Err(io_error(path, e)),
    }
}
