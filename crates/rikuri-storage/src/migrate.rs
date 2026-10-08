//! Versioned, transactional, checksummed schema migrations
//! (Architecture §43). Each component that owns tables (storage itself, the
//! ledger) registers its own numbered list. One open upgrades every
//! component in a single transaction, so a failure anywhere leaves the whole
//! previous schema intact and retryable. Opening refuses a database written
//! by a newer build, by a different definition of an applied migration, or
//! with a gap in its recorded versions.

use rikuri_domain::Timestamp;
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};

use crate::StorageError;

/// One forward step of a component's schema.
#[derive(Debug, Clone, Copy)]
pub struct Migration {
    /// 1, 2, 3, … with no gaps.
    pub version: u32,
    pub name: &'static str,
    pub sql: &'static str,
}

/// A component's complete, ordered migration list.
#[derive(Debug, Clone, Copy)]
pub struct Migrations {
    /// Stable lowercase name, e.g. `storage` or `ledger`.
    pub component: &'static str,
    pub list: &'static [Migration],
}

const BOOKKEEPING: &str = "
CREATE TABLE IF NOT EXISTS schema_migrations (
    component     TEXT    NOT NULL,
    version       INTEGER NOT NULL CHECK (version >= 1),
    name          TEXT    NOT NULL,
    checksum      TEXT    NOT NULL,
    applied_at_ms INTEGER NOT NULL,
    PRIMARY KEY (component, version)
) STRICT;
";

/// Content checksum of a migration's definition.
fn checksum(m: &Migration) -> String {
    let mut h = blake3::Hasher::new();
    h.update(m.name.as_bytes());
    h.update(&[0]);
    h.update(m.sql.as_bytes());
    h.finalize().to_hex().to_string()
}

impl Migrations {
    fn validate(&self) -> Result<(), StorageError> {
        let invalid = |reason: String| StorageError::InvalidMigrations {
            component: self.component,
            reason,
        };
        if self.component.is_empty()
            || !self
                .component
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b == b'_')
        {
            return Err(invalid("component name must be lowercase letters".into()));
        }
        if self.list.is_empty() {
            return Err(invalid("no migrations".into()));
        }
        for (i, m) in self.list.iter().enumerate() {
            let expected =
                u32::try_from(i + 1).map_err(|_| invalid("too many migrations".into()))?;
            if m.version != expected {
                return Err(invalid(format!(
                    "versions must run 1, 2, 3, … without gaps; position {} has version {}",
                    i + 1,
                    m.version
                )));
            }
        }
        Ok(())
    }

    /// The newest version this build knows.
    pub fn latest(&self) -> u32 {
        self.list.last().map_or(0, |m| m.version)
    }
}

/// Upgrades the database all-or-nothing (Architecture §43). Validates every
/// component's list without touching the database; then, in one IMMEDIATE
/// transaction, checks every component's recorded history (contiguous,
/// not newer than this build, unchanged definitions) before applying any
/// pending step, applies all pending steps of all components, and commits
/// once. Any refusal or failure rolls the whole upgrade back, so the
/// previous build still opens. Migration SQL therefore must be
/// transactional: no `BEGIN`/`COMMIT`, `VACUUM` or `journal_mode` changes.
pub(crate) fn apply_all(
    conn: &mut Connection,
    components: &[&Migrations],
    now: Timestamp,
) -> Result<(), StorageError> {
    let mut seen = std::collections::HashSet::new();
    for c in components {
        c.validate()?;
        if !seen.insert(c.component) {
            return Err(StorageError::InvalidMigrations {
                component: c.component,
                reason: "component registered twice".into(),
            });
        }
    }
    let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
    tx.execute_batch(BOOKKEEPING)?;
    let mut current = Vec::with_capacity(components.len());
    for c in components {
        current.push(check_component(&tx, c)?);
    }
    for (c, applied) in components.iter().zip(current) {
        for m in c.list.iter().filter(|m| m.version > applied) {
            let failed = |source| StorageError::MigrationFailed {
                component: c.component,
                version: m.version,
                name: m.name,
                source,
            };
            tx.execute_batch(m.sql).map_err(failed)?;
            tx.execute(
                "INSERT INTO schema_migrations (component, version, name, checksum, applied_at_ms)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![c.component, m.version, m.name, checksum(m), now.millis()],
            )
            .map_err(failed)?;
        }
    }
    tx.commit()?;
    Ok(())
}

/// Checks `c`'s recorded history and returns its current version. Writes
/// nothing.
fn check_component(conn: &Connection, c: &Migrations) -> Result<u32, StorageError> {
    let applied: Vec<(u32, String)> = {
        let mut stmt = conn.prepare(
            "SELECT version, checksum FROM schema_migrations WHERE component = ?1 ORDER BY version",
        )?;
        let rows = stmt.query_map(params![c.component], |r| Ok((r.get(0)?, r.get(1)?)))?;
        rows.collect::<Result<_, _>>()?
    };

    for (i, (version, _)) in applied.iter().enumerate() {
        if usize::try_from(*version).ok() != Some(i + 1) {
            return Err(StorageError::InvalidMigrations {
                component: c.component,
                reason: format!(
                    "database records {} versions with version {version} at position {}; \
                     they must run 1, 2, 3, … without gaps",
                    applied.len(),
                    i + 1
                ),
            });
        }
    }
    if let Some(&(found, _)) = applied.last()
        && found > c.latest()
    {
        return Err(StorageError::SchemaTooNew {
            component: c.component,
            found,
            supported: c.latest(),
        });
    }
    for (version, stored) in &applied {
        let m = c
            .list
            .iter()
            .find(|m| m.version == *version)
            .ok_or_else(|| StorageError::InvalidMigrations {
                component: c.component,
                reason: format!("database records unknown version {version}"),
            })?;
        let expected = checksum(m);
        if *stored != expected {
            return Err(StorageError::MigrationMismatch {
                component: c.component,
                version: *version,
                name: m.name,
                stored: stored.clone(),
                expected,
            });
        }
    }
    Ok(applied.last().map_or(0, |&(v, _)| v))
}

/// The newest applied version of `component`, if any.
pub(crate) fn applied_version(
    conn: &Connection,
    component: &str,
) -> Result<Option<u32>, StorageError> {
    Ok(conn
        .query_row(
            "SELECT MAX(version) FROM schema_migrations WHERE component = ?1",
            params![component],
            |r| r.get::<_, Option<u32>>(0),
        )
        .optional()?
        .flatten())
}
