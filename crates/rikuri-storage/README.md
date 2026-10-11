# rikuri-storage — Module Contract

Template C, [Engineering Templates v1.1](../../doc/Rikuri_Pi_Engineering_Templates_v1.1.md#template-c--module-contract). Stage Plan item S00.05.

## Purpose

The canonical store: one SQLite database holding current state, written by one writer in explicit transactions, migrated by versioned checksummed steps, and read and written through `Repository<T>` with optimistic versioning ([ADR 0002](../../docs/architecture/adr/0002-sqlite-current-state-model.md)).

## Owns

- Opening the database: owner-only data directory and database file on Unix (created 0700/0600; an existing directory or database that group or others can access is refused, never re-permissioned; SQLite gives `-wal`/`-shm` the database's mode; not yet enforced on other platforms), WAL (verified), `synchronous=FULL`, foreign keys, `recursive_triggers`, busy timeout, `quick_check`.
- The transaction boundary: `Store::write` (IMMEDIATE; commit only on `Ok`, rollback on any error) and `Store::read` (snapshot, always rolled back). A `Store` call made from inside one of its own transactions (`read`, `write`, `schema_version`) is refused with `Reentrant` instead of deadlocking on the connection lock; code inside a transaction uses the `Tx` (`Tx::schema_version`). Different stores may nest (A's transaction calls B); re-entering any store already in the nest is refused.
- One writer per database: `Store::open` takes an exclusive lock on `<database>.lock` (owner-only) for the store's lifetime, so a second store on the same file, in this process or another, gets `InUse`. The lock is taken before the database file is created, so an open refused at the lock (held elsewhere, or a lock file others can access) creates no database file; the data directory and the lock file itself may already have been created.
- The migration runner and its bookkeeping (`schema_migrations`): validation of every component's list before writing; then one transaction for the whole upgrade, which checks every component's recorded history (contiguous versions, checksums, not newer than this build) before applying any step, so a refusal or failure anywhere changes nothing.
- The `objects` registry table and `Repository<T>` / `SqliteRepository<T>`: insert at version 1, update only from the expected version, read-back inside the transaction (it confirms what the transaction holds, not durability, which comes from the commit), tamper detection on read.
- The store's health evidence (`StoreHealth`), and the rule (`StoreFailure`) by which an error from a transaction counts as a store failure (SQLite, I/O, a poisoned lock, re-entry, a corrupt row) or as the caller refusing the change (not found, already exists, version conflict, validation).

## Must not own

- What events or idempotency records mean, or their tables (`rikuri-ledger` registers its own migrations).
- Business rules of any entity (tasks, runs, projects): those live with the kernel that defines the entity.
- Policy, privacy admission or budget decisions; content encryption and keys (Stage 01).
- Any second writer: only `rikurid` opens the store for writing.

## Trust level / boundary

Trusted kernel code inside `rikurid`. Treats stored rows as possibly damaged: a row whose body names another ID, whose schema version is unknown, whose kind differs, or whose privacy class or deletion unit column disagrees with the body is reported `Corrupt`, never trusted. Stored text quoted in an error is bounded (`rikuri_domain::bounded`).

## Canonical data touched

`objects` (all typed entities' current state) and `schema_migrations`. Other components' tables are created through their registered migrations. Storage migrations: version 1 (the object registry), version 2 (the `objects_by_deletion_unit` index, E1).

## Public contracts / interfaces

- `Store::open(path, clock, &[&Migrations])`, `Store::write`, `Store::read`, `Store::health`, `Store::schema_version`, `Store::now`, `Store::path`, `Store::in_transaction` (whether this thread is inside one of this store's transactions; `rikurid`'s crash points use it, W1).
- `pub use rusqlite`, so components that own tables use the same SQLite build.
- `Tx::schema_version`, for reading a component's version inside a transaction.
- `Tx::written` → `Vec<Written>` (`kind`, `id`, `before`, `after` bodies): every object the repository inserted or updated in this transaction, recorded after its read-back, in order. `rikurid`'s pre-commit candidate check validates exactly this list (ADR 0005, clarification 2026-10-10). Raw SQL through the `Tx` is not listed.
- `deletion_unit_size(tx, unit)`: how many objects of any kind belong to a deletion unit, through the `objects_by_deletion_unit` index; `rikurid` refuses a new task whose unit is not its own (E1).
- `Migration`, `Migrations`; `Entity` (kind, schema version, ID, privacy class, deletion unit); `Versioned<T>`; `Repository<T>` (`get`, `insert`, `update`, `list`); `SqliteRepository<T>`; `StorageError` (including `Reentrant` and `InUse`); `StoreFailure`, which every error type used with `read`/`write` implements; `StoreHealth`, `OpHealth`.
- `Tx` dereferences to the SQLite connection so components that own tables write in the same transaction. Raw transaction control through it (`COMMIT`, `BEGIN`) is not yet refused: deferred to S00.07 ([record](../../docs/build-records/2026-10-08-storage-ledger-verifier-defects.md)).

## Allowed dependencies

`rikuri-domain`, `rusqlite` (bundled SQLite), `serde`, `serde_json`, `thiserror`, `blake3`. Test-only: `proptest`, `tempfile`, `rikuri-testkit`.

## Forbidden dependencies

`rikuri-ledger`, `rikurid`, any network, async runtime or provider crate.

## Failure and degraded behavior

Every failure is a typed `StorageError`; nothing panics on bad data. A failed transaction leaves no trace. A failed or refused upgrade changes nothing, in any component, so the previous build still opens and the upgrade can be retried. A database from a newer build, with a changed applied migration or with a gap in its recorded versions is refused at open rather than half-read. A shared data directory or database file is refused at open (`Io`, permission denied, with the `chmod` to fix it). A poisoned writer lock is reported as `Poisoned` (restart the daemon).

## Health / last-success signal

`Store::health()` reports, for transactions:

```text
state:                (derived by rikurid: no success yet or recent failures → not healthy)
last_attempt:         start time of the last transaction that succeeded or failed
last_success:         start time of the last committed write or completed read
last_error:           last store failure, bounded
consecutive_failures: store failures since the last success, any kind
work_completed:       successful transactions since open
reads, writes:        the same evidence for each kind separately, so a successful read
                      cannot hide a failing write
refused:              transactions the caller rolled back with its own refusal (not failures)
```

A store that has done nothing has no `last_success` and must not be reported healthy (Architecture §53.7).

## Architecture / ADR links

- Invariants: Architecture §4 invariants 2–4, §8 (one transaction boundary), §21–22, §43 (Transactional State Evolution).
- ADRs: [ADR 0002](../../docs/architecture/adr/0002-sqlite-current-state-model.md); [ADR 0001](../../docs/architecture/adr/0001-stable-id-format.md) for IDs.

## Threat / control links

- Threat: a crash mid-change leaves partial state. Control: one transaction per change, commit only on success, WAL + `synchronous=FULL`. Tests: `committed_state_survives_reopen`, rollback cases in the contract suite; kill -9 matrix T00.07.
- Threat: two writers both commit on the same base (lost update). Control: version-checked update. Test: contract suite and `versions_follow_the_model`.
- Threat: a damaged or tampered row is trusted. Control: ID/kind/schema/privacy/deletion-unit checks on read, schema CHECK constraints. Tests: `tampered_rows_are_reported_not_trusted`, `rows_of_another_kind_are_refused`, `privacy_and_deletion_columns_must_match_the_body`, `corrupt_row_errors_are_bounded`.
- Threat: an upgrade half-applies or an old build misreads a newer store. Control: one transaction for the whole upgrade, checks before any step, checksums, contiguity, version refusal. Tests: migration suite, including `failed_multi_component_upgrade_changes_nothing`, `refusal_in_one_component_upgrades_no_other`, `recorded_versions_must_be_contiguous`.
- Threat: another local user reads or replaces the database. Control: owner-only directory and files on Unix, shared ones refused. Tests: `database_and_directory_are_owner_only`, `shared_directory_or_database_is_refused`.
- Threat: a read changes state. Control: `Store::read` always rolls back. Test: `reads_never_commit`.
- Threat: a caller re-enters the store inside its own transaction and the process hangs silently. Control: a per-thread stack of the stores whose transactions are open; re-entry into any of them returns `Reentrant`, and the stack is popped even if a transaction closure panics. Tests: `store_calls_inside_its_own_transaction_are_refused_not_deadlocked`, `sequential_store_calls_still_work`, `nested_stores_are_allowed_but_reentry_anywhere_in_the_nest_is_refused`, `reentry_after_an_inner_store_returns_is_still_refused`.
- Threat: two writers on one database. Control: the writer lock. Test: `a_second_store_on_the_same_database_is_refused`.
- Threat: health hides failing writes behind successful reads, or reports refusals as failures. Control: separate read and write evidence; `StoreFailure` classification. Tests: `reads_do_not_hide_write_failures_and_refusals_are_not_failures`, `storage_errors_say_whether_the_store_failed` (and the ledger's `ledger_errors_say_whether_the_store_failed`); through `rikurid`, `store_health_reports_failed_writes` and `refused_requests_do_not_mark_the_store_failed`.
- Threat: a refused open leaves files behind, or the lock file is readable by others. Control: lock first, owner-only lock file. Tests: `a_second_open_reports_in_use`, `the_lock_file_is_owner_only`.

## Required tests

- [x] unit (via the integration suites; the crate is small)
- [x] failure paths (conflict, not found, duplicate, rollback, tamper, failed/newer/divergent/gapped migrations, multi-component upgrade failure, shared files), a write whose body or version does not read back as written rolls back with the last known-good row kept (`a_write_that_does_not_read_back_rolls_back`, `a_write_whose_version_does_not_read_back_rolls_back`, E1; each fails alone when its comparison is removed), and an insert that fails for any reason other than a constraint violation is not reported as `AlreadyExists` (`an_insert_that_fails_otherwise_is_not_reported_as_already_existing`)
- [x] shared contract suite — T00.04 `tests/repository_contract.rs`, generic over `Repository<T>`
- [x] a transaction lists each insert and update with the body it replaced, in order; a refused update is not listed, and every transaction starts with an empty list — `a_transaction_lists_what_it_wrote` (E1)
- [x] a deletion unit's objects are counted, of any kind, through the `objects_by_deletion_unit` index (`EXPLAIN QUERY PLAN`) — `a_deletion_unit_counts_its_objects_through_its_index`; storage migration 2 upgrades a version-1 store with its objects intact — `storage_migration_2_adds_the_deletion_unit_index_to_a_version_1_store` (E1). The two tests that pin storage's schema version now expect 2 (owner-approved with the migration).
- [x] invariant/property tests — model-based `versions_follow_the_model`
- [ ] replay/golden fixtures — N/A (no nondeterministic path)
- [x] re-entry refused, never deadlocked, and `in_transaction` true only inside a read or write — `tests/reentrancy.rs`
- [x] integration/smoke — through `rikurid` (S00.07: T00.09 and the smoke, PR #20)

## Documentation / help entry points

This file; ADR 0002; rustdoc.

## Known limitations / explicit debt

- One connection serializes reads with writes; acceptable for one owner, revisit with measured contention.
- `synchronous=FULL` costs about 3.3 ms per kernel command on the reference installation (one fsync per commit), against about 0.4 ms with `NORMAL`; kept, because `NORMAL` can lose the last commits on power loss (E1, [ADR 0005](../../docs/architecture/adr/0005-e1-transactional-state-evolution-for-kernel-transactions.md)). Power loss itself is not tested (Not yet proven).
- Real caller arrives with `rikurid` (S00.07); until then this crate is not complete.
