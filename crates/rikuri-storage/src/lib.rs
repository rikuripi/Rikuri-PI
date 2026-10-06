//! Canonical store for the Rikuri kernel (Stage Plan S00.05, ADR 0002).
//!
//! SQLite in WAL mode with one writer. Every canonical change happens inside
//! a [`Store::write`] transaction; a component that needs tables registers
//! [`Migrations`] and the store applies them, checksummed, at open. Typed
//! entities live behind [`Repository`], whose updates are optimistic: an
//! update names the version it read and fails if another write got there
//! first. See `README.md` for the Module Contract.

mod error;
mod migrate;
mod repo;
mod store;

pub use error::StorageError;
pub use migrate::{Migration, Migrations};
pub use repo::{Entity, Repository, SqliteRepository, Versioned};
pub use store::{Store, StoreHealth, Tx};

/// Re-exported so components that own tables (the ledger) write SQL against
/// the same connection type the store uses.
pub use rusqlite;
