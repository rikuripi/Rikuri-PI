use thiserror::Error;

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
}
