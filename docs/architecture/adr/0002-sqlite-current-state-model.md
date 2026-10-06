# ADR-0002: SQLite current-state model

Status: Proposed
Date: 2026-10-05
Owners: Henry Chen (product owner, accepts); Stage 00 implementer
Related requirements/tests: Stage Plan S00.05, S00.10, T00.04, E1; Architecture §8, §21–22, §43, §46 ("Canonical personal DB: LOCKED Foundation-0 — SQLite/WAL/single canonical writer"); PRD CORE-006

## Context / problem

The kernel needs a canonical store for current state that survives a killed process with no partial writes, refuses lost updates between concurrent writers, and can be migrated without leaving a half-upgraded store. The Architecture already locks SQLite with WAL and a single writer; this ADR fixes how state is laid out, versioned and migrated.

## Decision drivers

- Committed means durable; uncommitted leaves no trace (W1).
- Two writers working from the same base must not both win.
- Migrations are versioned, transactional, tested from prior versions and refuse what they do not understand (Architecture §43).
- Each data type keeps one authority; derived data stays out (Architecture §22).
- No schema churn before entities exist: Stage 00 has only kernel entities (tasks, runs).

## Options considered

1. **rusqlite with bundled SQLite** (chosen): synchronous, small, pins the SQLite version into the build, so macOS and Ubuntu run the same engine.
2. **sqlx**: async and compile-time-checked queries, but needs an async runtime and a database at build time; nothing in Stage 00 is async.
3. **A typed table per entity from day one**: precise, but premature while only kernel entities exist. Kept as the promotion path when a field must be queried or indexed.
4. **Event-sourced state** (rebuild "now" from events): rejected by Architecture §8. The ledger records history; tables hold current truth.

## Evidence / prototype

`crates/rikuri-storage`. T00.04 contract suite (`tests/repository_contract.rs`): versioned put/get, `AlreadyExists`, `VersionConflict` with the actual version, `NotFound`, rollback of a failed transaction, tamper detection, and a model-based property test over random insert/update sequences. Migration tests (`tests/migrations.rs`): fresh apply, idempotent reopen, upgrade preserving data, a failing migration leaving the last good version, refusal of a newer-build database and of a changed applied migration, refusal of malformed lists before any table is created, WAL and `synchronous=FULL` confirmed.

## Decision

- **One database** per Personal Intelligence at `<home>/state/rikuri.db`; the directory is created owner-only (0700). rusqlite 0.40.2 with the bundled SQLite 3.53.2 (libsqlite3-sys 0.38.2, MIT).
- **Pragmas at open:** `journal_mode=WAL` (verified, else refuse), `synchronous=FULL` (a commit survives power loss, not only a process crash), `foreign_keys=ON`, 5 s busy timeout, `quick_check` must report `ok`.
- **One writer:** `rikurid` owns the only connection. Writes run in IMMEDIATE transactions through `Store::write`; reads in a snapshot transaction through `Store::read`. Commit happens only if the closure returns `Ok`; any error rolls back.
- **Object registry:** a common `objects` table (`id`, `kind`, `version`, `schema_version`, `privacy_class`, `deletion_unit_id`, `created_at_ms`, `updated_at_ms`, `body` as JSON) holds current state for every typed entity (Architecture §21.1). `privacy_class` is constrained to the four classes. A field moves into a typed table when it must be queried or indexed; that is a migration, not a redesign.
- **Optimistic versioning:** `Repository<T>::update` names the version it read and changes the row only if it is still at that version, else `VersionConflict { expected, actual }`.
- **Read-back:** every insert and update reads its row back inside the transaction and compares it with what was meant to be written before the commit (Transactional State Evolution: durable write → read back → atomic promote at commit).
- **Migrations:** each component that owns tables registers a numbered list (`storage`, `ledger`, …). Lists are validated before anything is written. Each step runs in its own transaction with its bookkeeping row and a BLAKE3 checksum of its definition. Opening refuses a database at a newer version than the build knows, or one whose applied migration differs from this build's definition.
- **Body schema:** each entity declares its body schema version. Reading a different version is refused as `Corrupt` until a migration converts it.

## Consequences

- JSON bodies are not efficiently queryable; acceptable while lookups are by ID. Stage 01 adds typed tables and FTS5 where search needs them.
- A single connection serializes reads with writes. Fine for one owner; revisit with measured contention.
- `synchronous=FULL` costs write latency; measured cost is not yet recorded (Not yet proven).
- E1 (simplest implementation of Transactional State Evolution) is answered for single-transaction kernel changes by SQLite's atomic commit plus read-back; the crash matrix (T00.07) is the evidence. Multi-store transformations (backup/restore, Stage 01) still need explicit candidate/promotion.

## Security / privacy impact

The database file and its directory are owner-only. `privacy_class` is enforced by the schema. Content encryption and per-deletion-unit keys arrive in Stage 01 (ADR 0005); until then the store holds only kernel task data, never imported personal content. Tamper detection (ID/body mismatch, unknown schema version) refuses rather than trusts a row.

## Data / migration impact

First schema: `storage` v1 (object registry), `ledger` v1 (ADR 0003). No user data exists yet.

## Rollback or reversal path

Before Stage 01 data exists, change freely. Afterwards, a different engine or layout is a versioned migration through backup → upgrade → restore (T01.12), with the old database kept until the new one verifies.

## Follow-up tests

- T00.07 kernel crash matrix through the real daemon; T00.06 property across injected crashes.
- Measure `synchronous=FULL` write latency on the reference installation.
