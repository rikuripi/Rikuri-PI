# ADR-0001: Stable ID format

Status: Proposed
Date: 2026-10-05
Owners: Henry Chen (product owner, accepts); Stage 00 implementer
Related requirements/tests: Stage Plan S00.04, S00.10, T00.12; Architecture §4 invariant 2, §8, §22, §46 ("IDs: PROVISIONAL — ULID unless ADR changes"); PRD §19.2; Writer IDE §3

## Context / problem

Every durable entity needs an identity that survives renames, moves, re-indexing, model changes, restore and, later, replication between the owner's devices (Stage 7). The Stage Plan says to adopt ULID or UUIDv7, with no custom format. Events, idempotency keys and references are compared as text in SQLite and in the JSON envelope, so one ID must have exactly one text form.

## Decision drivers

- Unique without coordination between processes or devices.
- Sortable by creation time, so ledger order and index locality are cheap.
- Readable in logs and receipts; the kind is visible without a lookup.
- Deterministic in tests (fixture IDs) without a test-only code path in the kernel.
- A maintained, permissively licensed implementation.

## Options considered

1. **ULID, typed prefix** (`task_01JA…`): 128 bits (48-bit millisecond time, 80 random), 26-character Crockford base32, sortable as text. Crate `ulid` 3.0.0, MIT.
2. **UUIDv7, typed prefix** (`task_0192…-…`): same bit budget (RFC 9562), 36-character hyphenated hex, wider standard tooling. Crate `uuid`, MIT/Apache-2.0.
3. **Per-table integer keys**: rejected. Not unique across tables, devices or restores; leaks counts; collides on replication.
4. **Content hashes as identity**: rejected for entities. An entity changes while keeping its identity; hashes remain for content integrity (Stage 01, BLAKE3).

## Evidence / prototype

`rikuri-domain` implements option 1 (`crates/rikuri-domain/src/id.rs`). Tests: canonical round-trip, wrong-kind refusal, non-canonical and malformed refusal, nil refusal, prefix registry uniqueness, monotonic generation over 10,000 IDs, and property tests over arbitrary text and every 128-bit value (`tests/id_properties.rs`), including "text order equals ID order".

## Decision

Option 1, matching the Architecture's provisional choice:

- Text form `<prefix>_<ULID>`: a registered lowercase ASCII-letter prefix, an underscore, 26 uppercase Crockford base32 characters. Parsing accepts only that canonical form: lowercase ULIDs, unknown prefixes and the nil ULID are refused, so one ID never has two spellings.
- Each kind is its own Rust type (`TaskId`, `EventId`, …). `AnyId` carries the prefix for fields that may reference several kinds.
- Registered prefixes in Stage 00: `project`, `task`, `run`, `evt`, `actor`, `dunit`, `content`, `source`, `span`, `secdec`, `cost`. Adding a kind is additive; a prefix is never renamed or reused.
- IDs are issued only through `IdSource`. Production uses `SystemIdSource` (process-monotonic). Tests use a deterministic source (`rikuri-testkit` `FixtureIds`, S00.08).
- IDs are opaque. Nothing parses meaning from the timestamp beyond ordering. IDs from imported sources are stored as attributes of a Rikuri entity, never used as its identity.
- Storage: SQLite `TEXT` in canonical form.

UUIDv7 would also satisfy the drivers. ULID wins on readability (26 versus 36 characters, case-insensitive alphabet) and because the Architecture already names it. The two are bit-compatible, so a later move is a representation change, not a re-identification.

## Consequences

- Receipts, logs and the CLI show IDs whose kind is obvious.
- Ordering across processes follows wall-clock time. Two processes in the same millisecond produce unique IDs whose relative order is arbitrary. The ledger's own sequence, not ID order, is authoritative for event order (ADR 0003).
- The 48-bit timestamp reveals creation time to anyone who sees an ID. Acceptable: IDs are not secrets, and exported or shared IDs carry the same time already visible in their records.

## Security / privacy impact

IDs carry no content, path, title or user data, only a creation time. Strict parsing closes a class of reference-forking bugs where one entity has two spellings, for example in idempotency keys. Parsers never panic on hostile input (property-tested).

## Data / migration impact

None yet; no data exists. Future kinds append to the prefix registry.

## Rollback or reversal path

Before Stage 01 data exists, change the format in `rikuri-domain` alone. Afterwards, ULID ↔ UUIDv7 is a lossless 128-bit re-encoding, done as a versioned schema migration under Transactional State Evolution (Architecture §43).

## Follow-up tests

- `rikuri-testkit` `FixtureIds` passes the same ID properties (S00.08).
- T00.12: task/run/event IDs survive crash and replay unchanged.
