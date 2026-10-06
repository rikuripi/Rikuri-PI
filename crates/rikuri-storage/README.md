# rikuri-storage — Module Contract

Template C, [Engineering Templates v1.1](../../doc/Rikuri_Pi_Engineering_Templates_v1.1.md#template-c--module-contract). Stage Plan item S00.05.

## Purpose

The canonical store: one SQLite database holding current state, written by one writer in explicit transactions, migrated by versioned checksummed steps, and read and written through `Repository<T>` with optimistic versioning ([ADR 0002](../../docs/architecture/adr/0002-sqlite-current-state-model.md)).

## Owns

- Opening the database: owner-only data directory, WAL (verified), `synchronous=FULL`, foreign keys, busy timeout, `quick_check`.
- The transaction boundary: `Store::write` (IMMEDIATE; commit only on `Ok`, rollback on any error) and `Store::read` (snapshot, always rolled back).
- The migration runner and its bookkeeping (`schema_migrations`): validation of every component's list before writing, one transaction per step, checksums, refusal of newer or divergent schemas.
- The `objects` registry table and `Repository<T>` / `SqliteRepository<T>`: insert at version 1, update only from the expected version, read-back verification inside the transaction, tamper detection on read.
- The store's health evidence (`StoreHealth`).

## Must not own

- What events or idempotency records mean, or their tables (`rikuri-ledger` registers its own migrations).
- Business rules of any entity (tasks, runs, projects): those live with the kernel that defines the entity.
- Policy, privacy admission or budget decisions; content encryption and keys (Stage 01).
- Any second writer: only `rikurid` opens the store for writing.

## Trust level / boundary

Trusted kernel code inside `rikurid`. Treats stored rows as possibly damaged: a row whose body names another ID, whose schema version is unknown, or whose kind differs is reported `Corrupt`, never trusted.

## Canonical data touched

`objects` (all typed entities' current state) and `schema_migrations`. Other components' tables are created through their registered migrations.

## Public contracts / interfaces

- `Store::open(path, clock, &[&Migrations])`, `Store::write`, `Store::read`, `Store::health`, `Store::schema_version`, `Store::now`.
- `Migration`, `Migrations`; `Entity` (kind, schema version, ID, privacy class, deletion unit); `Versioned<T>`; `Repository<T>` (`get`, `insert`, `update`, `list`); `SqliteRepository<T>`; `StorageError`.
- `Tx` dereferences to the SQLite connection so components that own tables write in the same transaction.

## Allowed dependencies

`rikuri-domain`, `rusqlite` (bundled SQLite), `serde`, `serde_json`, `thiserror`, `blake3`. Test-only: `proptest`, `tempfile`, `ulid`.

## Forbidden dependencies

`rikuri-ledger`, `rikurid`, any network, async runtime or provider crate.

## Failure and degraded behavior

Every failure is a typed `StorageError`; nothing panics on bad data. A failed transaction leaves no trace. A failed migration leaves the previous version intact and retryable. A database from a newer build, or with a changed applied migration, is refused at open rather than half-read. A poisoned writer lock is reported as `Poisoned` (restart the daemon).

## Health / last-success signal

`Store::health()` reports, for transactions:

```text
state:                (derived by rikurid: no success yet or recent failures → not healthy)
last_attempt:         start time of the last transaction
last_success:         start time of the last committed write or completed read
last_error:           last failure description
consecutive_failures: failures since the last success
work_completed:       successful transactions since open
```

A store that has done nothing has no `last_success` and must not be reported healthy (Architecture §53.7).

## Architecture / ADR links

- Invariants: Architecture §4 invariants 2–4, §8 (one transaction boundary), §21–22, §43 (Transactional State Evolution).
- ADRs: [ADR 0002](../../docs/architecture/adr/0002-sqlite-current-state-model.md); [ADR 0001](../../docs/architecture/adr/0001-stable-id-format.md) for IDs.

## Threat / control links

- Threat: a crash mid-change leaves partial state. Control: one transaction per change, commit only on success, WAL + `synchronous=FULL`. Tests: `committed_state_survives_reopen`, rollback cases in the contract suite; kill -9 matrix T00.07.
- Threat: two writers both commit on the same base (lost update). Control: version-checked update. Test: contract suite and `versions_follow_the_model`.
- Threat: a damaged or tampered row is trusted. Control: ID/kind/schema checks on read, schema CHECK constraints. Test: `tampered_rows_are_reported_not_trusted`.
- Threat: an upgrade half-applies or an old build misreads a newer store. Control: per-step transactions, checksums, version refusal. Tests: migration suite.

## Required tests

- [x] unit (via the integration suites; the crate is small)
- [x] failure paths (conflict, not found, duplicate, rollback, tamper, failed/newer/divergent migrations)
- [x] shared contract suite — T00.04 `tests/repository_contract.rs`, generic over `Repository<T>`
- [x] invariant/property tests — model-based `versions_follow_the_model`
- [ ] replay/golden fixtures — N/A (no nondeterministic path)
- [ ] integration/smoke — through `rikurid` (S00.07)

## Documentation / help entry points

This file; ADR 0002; rustdoc.

## Known limitations / explicit debt

- One connection serializes reads with writes; acceptable for one owner, revisit with measured contention.
- `synchronous=FULL` write latency not yet measured on the reference installation.
- Real caller arrives with `rikurid` (S00.07); until then this crate is not complete.
