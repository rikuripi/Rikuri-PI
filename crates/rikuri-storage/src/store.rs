//! The store: one SQLite connection, one writer, explicit transactions.

use std::cell::RefCell;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use rikuri_domain::{Clock, Timestamp};
use rusqlite::{Connection, TransactionBehavior};
use serde::Serialize;

use crate::migrate::{self, Migration, Migrations};
use crate::{StorageError, StoreFailure};

/// The storage component's own tables: the common object registry
/// (Architecture §21.1) holding every typed entity's current state.
pub(crate) const STORAGE_MIGRATIONS: Migrations = Migrations {
    component: "storage",
    list: &[
        Migration {
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
        },
        Migration {
            version: 2,
            name: "objects counted by deletion unit",
            // the kernel refuses a new task whose deletion unit is not its own
            // (E1, ADR 0005); counting through this index keeps that check
            // from scanning every object
            sql: "CREATE INDEX objects_by_deletion_unit ON objects (deletion_unit_id);",
        },
    ],
};

/// A transaction. Dereferences to the connection, so components that own
/// tables run their SQL inside the same transaction as everyone else.
pub struct Tx<'a> {
    inner: rusqlite::Transaction<'a>,
    written: std::cell::RefCell<Vec<Written>>,
}

/// One object a transaction wrote through a repository, with its stored
/// body before (none for an insert) and after. The kernel validates its
/// candidate against exactly these before committing (Transactional State
/// Evolution's "validate candidate" step; E1, ADR 0005).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Written {
    pub kind: &'static str,
    pub id: String,
    pub before: Option<String>,
    pub after: String,
}

impl Tx<'_> {
    /// Every object this transaction wrote through a repository, in order.
    /// Writes a repository refused are not listed.
    pub fn written(&self) -> Vec<Written> {
        self.written.borrow().clone()
    }

    pub(crate) fn note_write(&self, w: Written) {
        self.written.borrow_mut().push(w);
    }

    /// The newest applied migration version of `component`, read inside this
    /// transaction. Use this, not `Store::schema_version`, from inside a
    /// `read` or `write` closure.
    pub fn schema_version(&self, component: &str) -> Result<Option<u32>, StorageError> {
        migrate::applied_version(self, component)
    }
}

/// Distinguishes stores so a re-entrant call can be recognized.
static NEXT_STORE_ID: AtomicU64 = AtomicU64::new(1);

thread_local! {
    /// The stores whose transactions this thread is inside, outermost first.
    /// Stores may nest (A's transaction calls B), but none may appear twice.
    static ACTIVE_STORES: RefCell<Vec<u64>> = const { RefCell::new(Vec::new()) };
}

/// Marks this thread as inside `id`'s transaction until dropped, including
/// when the transaction closure panics.
struct ActiveGuard;

impl ActiveGuard {
    fn enter(id: u64) -> ActiveGuard {
        ACTIVE_STORES.with(|a| a.borrow_mut().push(id));
        ActiveGuard
    }
}

impl Drop for ActiveGuard {
    fn drop(&mut self) {
        ACTIVE_STORES.with(|a| {
            a.borrow_mut().pop();
        });
    }
}

impl Deref for Tx<'_> {
    type Target = Connection;

    fn deref(&self) -> &Connection {
        &self.inner
    }
}

/// Health of the store, reported on absence of success as well as on
/// failure (Architecture §53.7). The top-level fields summarize every
/// transaction: `last_success` is the last committed write or completed
/// read, and a store that has never succeeded is not healthy. `reads` and
/// `writes` keep the two apart, so a successful read can never hide a
/// failing write. Only store failures (SQLite, I/O, a poisoned lock,
/// re-entry, a corrupt row) count as failures; a transaction the caller
/// rolled back with its own refusal (not found, conflict, validation) is
/// counted in `refused` and says nothing against the store.
#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
pub struct StoreHealth {
    pub last_attempt: Option<Timestamp>,
    pub last_success: Option<Timestamp>,
    pub last_error: Option<String>,
    pub consecutive_failures: u64,
    pub work_completed: u64,
    pub reads: OpHealth,
    pub writes: OpHealth,
    pub refused: u64,
}

/// Health evidence for one kind of transaction (reads or writes).
#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
pub struct OpHealth {
    pub last_attempt: Option<Timestamp>,
    pub last_success: Option<Timestamp>,
    pub last_error: Option<String>,
    pub consecutive_failures: u64,
    pub succeeded: u64,
}

impl OpHealth {
    fn succeeded_at(&mut self, at: Timestamp) {
        self.last_attempt = Some(at);
        self.last_success = Some(at);
        self.consecutive_failures = 0;
        self.succeeded += 1;
    }

    fn failed_at(&mut self, at: Timestamp, error: &str) {
        self.last_attempt = Some(at);
        self.last_error = Some(error.to_owned());
        self.consecutive_failures += 1;
    }
}

/// How a transaction ended, for the store's health.
enum Outcome {
    Succeeded,
    Failed(String),
    Refused,
}

/// The canonical SQLite store.
pub struct Store {
    id: u64,
    conn: Mutex<Connection>,
    /// Held for the store's lifetime: one store, and so one writer, per
    /// database file, also across processes.
    _writer_lock: std::fs::File,
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
        // the lock first: a refused open creates nothing
        let writer_lock = lock_writer(path)?;
        ensure_private_file(path, "database file")?;
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
            id: NEXT_STORE_ID.fetch_add(1, Ordering::Relaxed),
            conn: Mutex::new(conn),
            path: path.to_path_buf(),
            clock,
            health: Mutex::new(StoreHealth::default()),
            _writer_lock: writer_lock,
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
    /// wrote is visible unless the commit succeeded. Calling this store
    /// again from inside `f` (`read`, `write`, `schema_version`) is refused
    /// with `Reentrant`; use the `Tx` instead.
    pub fn write<T, E>(&self, f: impl FnOnce(&Tx<'_>) -> Result<T, E>) -> Result<T, E>
    where
        E: From<StorageError> + StoreFailure,
    {
        self.run(TransactionBehavior::Immediate, f)
    }

    /// Runs `f` in a read transaction: one consistent snapshot. Rolled back
    /// at the end; reads cannot change state.
    pub fn read<T, E>(&self, f: impl FnOnce(&Tx<'_>) -> Result<T, E>) -> Result<T, E>
    where
        E: From<StorageError> + StoreFailure,
    {
        self.run(TransactionBehavior::Deferred, f)
    }

    fn run<T, E>(
        &self,
        behavior: TransactionBehavior,
        f: impl FnOnce(&Tx<'_>) -> Result<T, E>,
    ) -> Result<T, E>
    where
        E: From<StorageError> + StoreFailure,
    {
        let started = self.clock.now();
        let result = (|| {
            self.refuse_reentry()?;
            let mut conn = self.conn.lock().map_err(|_| StorageError::Poisoned)?;
            let _active = ActiveGuard::enter(self.id);
            let inner = conn
                .transaction_with_behavior(behavior)
                .map_err(StorageError::from)?;
            let tx = Tx {
                inner,
                written: std::cell::RefCell::new(vec![]),
            };
            let value = f(&tx)?;
            match behavior {
                TransactionBehavior::Deferred => tx.inner.rollback().map_err(StorageError::from)?,
                _ => tx.inner.commit().map_err(StorageError::from)?,
            }
            Ok::<T, E>(value)
        })();
        let outcome = match &result {
            Ok(_) => Outcome::Succeeded,
            Err(e) => e.store_failure().map_or(Outcome::Refused, Outcome::Failed),
        };
        self.record(
            started,
            matches!(behavior, TransactionBehavior::Deferred),
            outcome,
        );
        result
    }

    fn record(&self, at: Timestamp, read: bool, outcome: Outcome) {
        let Ok(mut h) = self.health.lock() else {
            return;
        };
        match outcome {
            Outcome::Succeeded => {
                h.last_attempt = Some(at);
                h.last_success = Some(at);
                h.consecutive_failures = 0;
                h.work_completed += 1;
                if read {
                    h.reads.succeeded_at(at);
                } else {
                    h.writes.succeeded_at(at);
                }
            }
            Outcome::Failed(error) => {
                h.last_attempt = Some(at);
                h.consecutive_failures += 1;
                if read {
                    h.reads.failed_at(at, &error);
                } else {
                    h.writes.failed_at(at, &error);
                }
                h.last_error = Some(error);
            }
            // the caller's decision; the store itself worked
            Outcome::Refused => h.refused += 1,
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

    /// The newest applied migration version of `component`. From inside a
    /// transaction use `Tx::schema_version`; calling this there is refused.
    pub fn schema_version(&self, component: &str) -> Result<Option<u32>, StorageError> {
        self.refuse_reentry()?;
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

impl Store {
    /// True while this thread is inside one of this store's transactions (a
    /// read or a write, from before `BEGIN` until after its commit or
    /// rollback). A caller can prove where it stands relative to its
    /// transaction from the store's own state, not its own bookkeeping
    /// (`rikurid`'s crash points, W1).
    pub fn in_transaction(&self) -> bool {
        ACTIVE_STORES.with(|a| a.borrow().contains(&self.id))
    }

    /// The connection lock is not reentrant: taking it again on a thread that
    /// already holds it would hang forever. Refuse instead, visibly.
    fn refuse_reentry(&self) -> Result<(), StorageError> {
        if self.in_transaction() {
            Err(StorageError::Reentrant)
        } else {
            Ok(())
        }
    }
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

/// Takes the database's writer lock (`<database>.lock`, owner-only), held
/// for the store's lifetime. A second store on the same database, in this
/// process or another, is refused instead of becoming a second writer.
fn lock_writer(path: &Path) -> Result<std::fs::File, StorageError> {
    let mut name = path.as_os_str().to_owned();
    name.push(".lock");
    let lock_path = PathBuf::from(name);
    ensure_private_file(&lock_path, "lock file")?;
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&lock_path)
        .map_err(|e| io_error(&lock_path, e))?;
    match file.try_lock() {
        Ok(()) => Ok(file),
        Err(std::fs::TryLockError::WouldBlock) => Err(StorageError::InUse {
            path: path.display().to_string(),
        }),
        Err(std::fs::TryLockError::Error(e)) => Err(io_error(&lock_path, e)),
    }
}

/// Creates a file owner-only before anything opens it (the database before
/// SQLite does, which then gives its WAL and shared-memory files the same
/// permissions), or checks an existing one.
fn ensure_private_file(path: &Path, what: &str) -> Result<(), StorageError> {
    if path.exists() {
        #[cfg(unix)]
        refuse_if_shared(path, what)?;
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
            refuse_if_shared(path, what)?;
            Ok(())
        }
        Err(e) => Err(io_error(path, e)),
    }
}
