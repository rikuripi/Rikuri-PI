# ADR-0002: SQLite current-state model

Status: Accepted (2026-10-08, Henry Chen, product owner; proposed 2026-10-05)
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

- **One database** per Personal Intelligence at `<home>/state/rikuri.db`; on Unix the directory is created owner-only (0700) and the database file 0600, and an existing directory or file that group or others can access is refused. rusqlite 0.40.2 with the bundled SQLite 3.53.2 (libsqlite3-sys 0.38.2, MIT).
- **Pragmas at open:** `journal_mode=WAL` (verified, else refuse), `synchronous=FULL` (a commit survives power loss, not only a process crash), `foreign_keys=ON`, `recursive_triggers=ON` (so REPLACE fires DELETE triggers on append-only tables), 5 s busy timeout, `quick_check` must report `ok`.
- **One writer:** `rikurid` owns the only connection. Writes run in IMMEDIATE transactions through `Store::write`; reads in a snapshot transaction through `Store::read`. Commit happens only if the closure returns `Ok`; any error rolls back.
- **Object registry:** a common `objects` table (`id`, `kind`, `version`, `schema_version`, `privacy_class`, `deletion_unit_id`, `created_at_ms`, `updated_at_ms`, `body` as JSON) holds current state for every typed entity (Architecture §21.1). `privacy_class` is constrained to the four classes. A field moves into a typed table when it must be queried or indexed; that is a migration, not a redesign.
- **Optimistic versioning:** `Repository<T>::update` names the version it read and changes the row only if it is still at that version, else `VersionConflict { expected, actual }`.
- **Read-back:** every insert and update reads its row back inside the transaction and compares it with what was meant to be written before the commit (Transactional State Evolution: candidate write → read back inside the transaction → durable write and atomic promote in one commit). The read-back confirms what the transaction holds; durability comes from the commit with WAL and `synchronous=FULL`. *Corrected 2026-10-08: this line first put the durable write before the read-back.*
- **Migrations:** each component that owns tables registers a numbered list (`storage`, `ledger`, …). Lists are validated before anything is written. The whole upgrade, across every component, runs in one transaction: every component's recorded history is checked first (contiguous versions, BLAKE3 checksums of applied definitions, not newer than this build), then all pending steps are applied with their bookkeeping rows, then one commit. A refusal or failure anywhere rolls everything back, so the previous build still opens. Opening refuses a database at a newer version than the build knows, one whose applied migration differs from this build's definition, or one with a gap in its recorded versions. Migration SQL must be transactional (no `BEGIN`/`COMMIT`, `VACUUM` or `journal_mode` changes). *Corrected 2026-10-08: this line first said each step ran in its own transaction, which let a failure in one component strand another already upgraded ([record](../../build-records/2026-10-08-storage-ledger-verifier-defects.md)).*
- **Body schema:** each entity declares its body schema version. Reading a different version is refused as `Corrupt` until a migration converts it.

## Consequences

- JSON bodies are not efficiently queryable; acceptable while lookups are by ID. Stage 01 adds typed tables and FTS5 where search needs them.
- A single connection serializes reads with writes. Fine for one owner; revisit with measured contention.
- `synchronous=FULL` costs write latency; measured cost is not yet recorded (Not yet proven). *Measured 2026-10-09 (E1, [ADR 0005](0005-e1-transactional-state-evolution-for-kernel-transactions.md)): about 3.3 ms per kernel command against 0.4 ms with `NORMAL` on the reference installation (one fsync per commit); kept.*
- E1 (simplest implementation of Transactional State Evolution) is answered for single-transaction kernel changes by SQLite's atomic commit plus read-back; the crash matrix (T00.07) is the evidence. Multi-store transformations (backup/restore, Stage 01) still need explicit candidate/promotion. *Clarified 2026-10-09 by [ADR 0005](0005-e1-transactional-state-evolution-for-kernel-transactions.md): E1 compared both designs; one transaction stays, and the kernel adds a candidate check scoped to what a command touches before the commit (TSE's "validate candidate" step, which read-back alone did not cover). Clarified 2026-10-10 by ADR 0005's clarification: the check now validates what each command wrote and appended, read from the transaction.*

## Security / privacy impact

The database file and its directory are owner-only on Unix, enforced at open (see Decision); other platforms do not enforce it yet (Windows arrives at product Stage 9). `privacy_class` is enforced by the schema and, with `deletion_unit_id`, checked against the body on every read. Content encryption and per-deletion-unit keys arrive in Stage 01 (ADR 0005); until then the store holds only kernel task data, never imported personal content. Tamper detection (ID/body mismatch, kind, unknown schema version, privacy or deletion-unit column disagreeing with the body) refuses rather than trusts a row.

## Data / migration impact

First schema: `storage` v1 (object registry), `ledger` v1 (ADR 0003). No user data exists yet.

Note 2026-10-10: `storage` v2 adds one index, `objects_by_deletion_unit` on `(deletion_unit_id)`, so the kernel's pre-commit check can count a deletion unit's objects without scanning the table ([ADR 0005](0005-e1-transactional-state-evolution-for-kernel-transactions.md), clarification 2026-10-10, N5). It is additive: no row is rewritten, and a version-1 store upgrades at open (`storage_migration_2_adds_the_deletion_unit_index_to_a_version_1_store`).

## Rollback or reversal path

Before Stage 01 data exists, change freely. Afterwards, a different engine or layout is a versioned migration through backup → upgrade → restore (T01.12), with the old database kept until the new one verifies.

## Follow-up tests

- T00.07 kernel crash matrix through the real daemon; T00.06 property across injected crashes.
- Measure `synchronous=FULL` write latency on the reference installation. *Done 2026-10-09 (ADR 0005).*
