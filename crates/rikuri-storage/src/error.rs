use rikuri_domain::bounded;
use thiserror::Error;

/// Whether an error returned from a transaction means the store itself
/// failed, as opposed to the caller refusing the change. Store failures
/// (SQLite, I/O, a poisoned lock, re-entry, a corrupt row) count against the
/// store's health; refusals (not found, already exists, version conflict,
/// validation) do not. Every error type used with `Store::read`/`write`
/// says which it is, so health can neither hide a failure nor report a
/// refusal as one.
pub trait StoreFailure {
    /// `Some(bounded description)` for a store failure, `None` for a refusal.
    fn store_failure(&self) -> Option<String>;
}

impl StoreFailure for StorageError {
    fn store_failure(&self) -> Option<String> {
        match self {
            StorageError::NotFound { .. }
            | StorageError::AlreadyExists { .. }
            | StorageError::VersionConflict { .. } => None,
            other => Some(bounded(&other.to_string())),
        }
    }
}

/// Every way a storage operation can fail. Callers map these to stable API
/// error codes; nothing here panics on bad data.
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("could not prepare {path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("{kind} {id} not found")]
    NotFound { kind: &'static str, id: String },

    #[error("{kind} {id} already exists")]
    AlreadyExists { kind: &'static str, id: String },

    #[error("{kind} {id} is at version {actual}, not the expected version {expected}")]
    VersionConflict {
        kind: &'static str,
        id: String,
        expected: u64,
        actual: u64,
    },

    #[error("stored {kind} {id} is invalid: {reason}")]
    Corrupt {
        kind: &'static str,
        id: String,
        reason: String,
    },

    #[error(
        "{component} schema is at version {found}, newer than this build supports ({supported}); refusing to open"
    )]
    SchemaTooNew {
        component: &'static str,
        found: u32,
        supported: u32,
    },

    #[error(
        "{component} migration {version} ({name}) was applied with a different definition (stored checksum {stored}, this build {expected}); refusing to open"
    )]
    MigrationMismatch {
        component: &'static str,
        version: u32,
        name: &'static str,
        stored: String,
        expected: String,
    },

    #[error(
        "{component} migration {version} ({name}) failed; the whole upgrade was rolled back and the database is unchanged: {source}"
    )]
    MigrationFailed {
        component: &'static str,
        version: u32,
        name: &'static str,
        #[source]
        source: rusqlite::Error,
    },

    #[error("invalid migration list for {component}: {reason}")]
    InvalidMigrations {
        component: &'static str,
        reason: String,
    },

    #[error("the store's writer lock is poisoned; restart the daemon")]
    Poisoned,

    #[error(
        "a store call was made from inside one of this store's own transactions, which would deadlock; use the transaction (`Tx`) passed to the closure instead"
    )]
    Reentrant,

    #[error(
        "{path} is already open by another store (another rikurid?); a database has one writer"
    )]
    InUse { path: String },
}
