# rikuri-ledger — Module Contract

Template C, [Engineering Templates v1.1](../../doc/Rikuri_Pi_Engineering_Templates_v1.1.md#template-c--module-contract). Stage Plan item S00.06.

## Purpose

The append-only record of what happened, and the idempotency records that make a retried or crash-replayed consequential command return its original outcome instead of applying twice ([ADR 0003](../../docs/architecture/adr/0003-ledger-and-idempotency.md)).

## Owns

- The `events` and `idempotency` tables (its own `ledger` migrations) and their append-only triggers: UPDATE, DELETE and REPLACE (including `INSERT OR REPLACE` and upserts) are refused by the schema on any connection. Schema changes such as `DROP TABLE` are outside this protection; only migrations change the schema.
- Event validation: dotted lowercase type, a key for every consequential event, metadata of small scalars only (keys and strings ≤ 200 characters, ≤ 4096 bytes), existing `caused_by`, no duplicate event ID.
- Idempotency: key format, command name (1–64 bytes of `a–z 0–9 . _ -`), request fingerprint (BLAKE3 of command + canonical parameters), `Fresh` / `Replay` / `Conflict` check, recording in the caller's transaction. A stored response is an object of small facts validated like metadata, because idempotency rows can never be deleted and content there could never be forgotten.
- Reads: `get`, `tail` (oldest first), `by_task`, `replay` (every event in `seq` order).

## Must not own

- Current state (the object registry in `rikuri-storage`) or any entity's business rules.
- The transaction boundary: every call runs inside the caller's `Store::write`/`read` so state, event and idempotency commit together.
- Content: events hold references (typed IDs), never content.
- Which commands are consequential or what their events mean: the kernel (`rikurid`) decides and passes the risk class.

## Trust level / boundary

Trusted kernel code inside `rikurid`. Stored rows are decoded defensively: an unparsable ID, status or JSON column is reported as `Corrupt`.

## Canonical data touched

`events`, `idempotency`.

## Public contracts / interfaces

`MIGRATIONS`; `NewEvent`, `Event`, `EventType`, `EventStatus`, `LedgerError` (implements `rikuri_storage::StoreFailure`: a storage failure or corrupt row counts against the store's health, a refused event does not); `append`, `get`, `tail`, `by_task`, `replay`; `IdempotencyKey`, `request_hash`, `check` → `IdempotencyCheck`, `record`, `IdempotencyRecord`; limits `MAX_METADATA_BYTES`, `MAX_METADATA_STRING`.

## Allowed dependencies

`rikuri-domain`, `rikuri-storage`, `serde`, `serde_json`, `thiserror`, `blake3`. Test-only: `proptest`, `tempfile`, `rikuri-testkit`.

## Forbidden dependencies

`rikurid`, any network, async or provider crate.

## Failure and degraded behavior

Typed `LedgerError`; an invalid event is refused before anything is written, and the caller's transaction then rolls back with it. Raw SQL that tries to UPDATE, DELETE or REPLACE history is aborted by triggers. Text quoted in an error is bounded (`rikuri_domain::bounded`).

## Health / last-success signal

N/A as a library (no state of its own between calls). `rikurid` reports ledger health from its startup and on-demand replay check (state agrees with history) and from the store's transaction evidence.

```text
state / last_attempt / last_success / last_error / consecutive_failures / work_completed:
  reported by rikurid's ledger-consistency component (S00.07)
```

## Architecture / ADR links

- Invariants: Architecture §4 invariants 10–11, §8, §41.1.
- ADRs: [ADR 0003](../../docs/architecture/adr/0003-ledger-and-idempotency.md); ADR 0001 (IDs), ADR 0002 (store).

## Threat / control links

- Threat: a retry or crash replay repeats a consequential effect. Control: key required for consequential events; replay returns the stored response; changed request refused. Tests: T00.05 idempotency cases; kill -9 matrix T00.07; property T00.06.
- Threat: history edited by ordinary code. Control: schema triggers for UPDATE, DELETE and REPLACE, plus `recursive_triggers` on the store's connection. Tests: `ledger_rows_are_append_only_even_for_raw_sql`, `ledger_rows_refuse_replace_too`.
- Threat: private content copied into the audit trail or an idempotency record. Control: metadata and response limits; references only. Tests: `events_cannot_embed_content`, `metadata_keys_are_bounded`, `idempotency_command_and_response_are_validated`.
- Threat: hostile input inflates errors and logs. Control: bounded error text. Test: `event_type_errors_are_bounded`.

## Required tests

- [x] unit/failure (validation, conflicts, references)
- [x] shared contract suite — T00.05 `tests/ledger_contract.rs`
- [x] invariant/property — committed vs rolled-back batches
- [ ] replay/golden fixtures — T00.12 through `rikurid`
- [ ] integration/smoke — through `rikurid` (S00.07)

## Documentation / help entry points

This file; ADR 0003; rustdoc.

## Known limitations / explicit debt

- Triggers stop accidents, not the trusted owner with the database open (ADR 0004).
- Idempotency records are kept indefinitely; a retention policy is a later decision.
- Real caller arrives with `rikurid` (S00.07).
