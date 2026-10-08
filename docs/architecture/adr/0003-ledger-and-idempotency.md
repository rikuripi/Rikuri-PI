# ADR-0003: Referenced-content append-only ledger and idempotency

Status: Accepted (2026-10-08, Henry Chen, product owner; proposed 2026-10-05)
Date: 2026-10-05
Owners: Henry Chen (product owner, accepts); Stage 00 implementer
Related requirements/tests: Stage Plan S00.06, S00.10, T00.05–T00.08, T00.12; Architecture §4 invariants 10–11, §8, §41.1; PRD CORE-002, CORE-004, CORE-007

## Context / problem

Rikuri must answer *what happened* durably, never repeat a consequential effect after a crash or a retry, and still be able to forget content later. A ledger that embeds content cannot honor forgetting; a ledger outside the state transaction can disagree with state after a crash.

## Decision drivers

- One transaction boundary for state, event and idempotency record (Architecture §8).
- A retry after any crash point returns the original outcome and changes nothing.
- The same key with a different request must be refused, not re-applied.
- Events carry references, not content (invariant 11).
- History cannot be edited by ordinary code paths.

## Options considered

1. **Ledger tables in the canonical SQLite database** (chosen): the state change, the event and the idempotency record commit or roll back together.
2. **A separate ledger database or log file**: needs a two-phase protocol between two stores to stay consistent across a crash; rejected for Foundation-0.
3. **Hash-chained events now**: adds tamper evidence against deliberate edits by someone with database access. That threat is outside Foundation-0's trust assumptions (the owner account is trusted, ADR 0004). Kept as a follow-up when cross-process or cross-device trust boundaries arrive (Architecture §27.11).

## Evidence / prototype

`crates/rikuri-ledger`. T00.05 contract suite (`tests/ledger_contract.rs`): append and query in order; consequential events refused without a key; replay returns the stored response; a changed request or command under the same key is a conflict; raw-SQL UPDATE and DELETE refused on both tables; content-shaped metadata refused; missing `caused_by` and dangling idempotency references refused; a failed transaction appends nothing; replay visits every event in order; a property test over committed and rolled-back batches.

## Decision

- **Tables** `events` and `idempotency` in the canonical database, owned by the `ledger` migration component. Triggers abort any UPDATE, DELETE or REPLACE (including `INSERT OR REPLACE` and upserts) on either table, on any connection; the store also turns on `recursive_triggers`. Schema changes such as `DROP TABLE` are outside this protection. *Corrected 2026-10-08: the triggers first covered UPDATE and DELETE only, and REPLACE rewrote rows ([record](../../build-records/2026-10-08-storage-ledger-verifier-defects.md)).*
- **Order:** `seq` (SQLite AUTOINCREMENT) is the authoritative order. Event IDs are ULIDs (ADR 0001) and are not used for ordering.
- **Event fields** follow Architecture §8: `event_id`, `schema_version`, `recorded_at`, `event_type` (dotted lowercase), `status` (`succeeded | failed | rejected`), `actor_id`, optional `project_id`, `task_id`, `run_id`, `subject_ids`, `input_refs`, `result_refs` (typed IDs), `security_decision_id`, `cost_record_id`, `idempotency_key`, `caused_by`, `metadata`.
- **Referenced content only:** metadata holds small scalars or lists of them; strings at most 200 characters; at most 4096 bytes in total (enforced in code and by the table). Anything larger is content and is referenced by ID.
- **Idempotency:** every event whose risk class is consequential (anything but `observe_read_only`) requires a caller-chosen key (8–128 characters). The key, command, BLAKE3 hash of the command and its canonical parameters, the event ID and the response are stored in the same transaction as the effect. A repeat with the same command and hash returns the stored response and does nothing; a different command or hash under the key is a conflict. A response must be an object of small facts (IDs, versions, statuses), validated like event metadata and capped at 4096 bytes, because idempotency rows cannot be deleted and content there could never be forgotten. The command name is 1–64 bytes of `a–z 0–9 . _ -`.
- **Replay helpers:** `replay` feeds every event to a fold in ledger order, so the kernel can rebuild or check derived facts from history (used by the health check, S00.07).

## Consequences

- Forgetting (Stage 01) deletes content and keys, never events; an event can outlive what it references, so replay must distinguish a governed tombstone from a dangling reference (T00.12 and Stage 01 forget).
- Triggers stop accidents and ordinary code paths, not a determined owner with the database open; that is consistent with ADR 0004. Within that scope, raw SQL can still insert an event with an explicit `seq`, including one earlier than existing events, and a row at `seq ≤ 0` (which only raw SQL can create) is not covered by the insert guard on a connection without `recursive_triggers`; ordering integrity against deliberate raw SQL is out of scope under the same assumption. The insert guard reads an omitted `seq` as -1, which SQLite documents as undefined; if that changed, ordinary appends would be refused (fail closed) and the ledger tests would catch it.
- The idempotency table grows with every consequential command. Retention/compaction policy is a later decision; Not yet proven at scale.

## Security / privacy impact

Events and idempotency responses carry no content, so audit history does not become a copy of private data. The metadata limits are a tripwire for accidental content embedding, not a classifier; reviewers still check what callers put in metadata.

## Data / migration impact

`ledger` v1. Event rows carry their own schema version for later evolution.

## Rollback or reversal path

Before user data exists, change freely. Afterwards, ledger changes are additive migrations; existing rows are never rewritten.

## Follow-up tests

- T00.06 property: no duplicate consequential effect across randomized commands with injected crashes.
- T00.07 crash points 1–3 and 6 through the real daemon; T00.08 the same matrix with idempotency disabled must fail.
- T00.12 task/run/event identity and verification links across crash and replay.
