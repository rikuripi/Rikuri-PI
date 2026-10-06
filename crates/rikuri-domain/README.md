# rikuri-domain — Module Contract

Template C, [Engineering Templates v1.1](../../doc/Rikuri_Pi_Engineering_Templates_v1.1.md#template-c--module-contract). Stage Plan item S00.04.

## Purpose

The kernel's shared vocabulary: stable typed IDs, data privacy classes and execution intent, provenance with instruction authority, risk classes, project execution trust, deletion units with content references, and schema versions. Every other crate speaks these types, so a value means the same thing in storage, the ledger, the CLI envelope and later stages.

## Owns

- The ID format and the registry of ID prefixes ([ADR 0001](../../docs/architecture/adr/0001-stable-id-format.md)); `IdSource` as the only way to issue IDs.
- The wire names of `PrivacyClass`, `ExecutionIntent`, `Origin`, `InstructionAuthority`, `RiskClass` and `ProjectTrust`.
- The rule that content authority derives from origin: only `user_live` content carries `current_user`; no `Provenance` value can hold `trusted_policy`, and deserialization refuses a raised authority.
- The rule that authority never comes from bytes (Architecture §19.1, LOCKED): a deserialized `Provenance` is historical and always has `instruction_authority: none`, even a faithful record of a live command. Only `Provenance::new(Origin::UserLive, …)` carries `current_user`; callers must make that call only at the authenticated command boundary, and must decide authority with `instruction_authority()`, never from `origin()`. The serialized `instruction_authority` is the value's real authority, so a loaded record is written back as `none` and readers of the bytes never see historical content labelled `current_user`; loading accepts the authority the origin implies or `none`, and refuses any raise.
- The privacy combination rule: derived material takes the strictest input class; unclassified content is `local_only`.
- `ContentRef`: events and records point at content through a content ID and its deletion unit, never by embedding it.

## Must not own

- I/O of any kind: storage, files, network, processes. The only system reads are the wall clock in `SystemClock` and `SystemIdSource`.
- Policy decisions (admission, approval, budget, envelope assurance) — `rikuri-security`, `rikuri-budget` and later crates decide; this crate only names the values.
- Transition rules for project trust (Stage 03) and declassification (a separately authorized transformation, later stages).
- Any model, provider or runtime name.

## Trust level / boundary

Trusted kernel code, linked into `rikurid`. It parses untrusted text (IDs and enum values from stored records, the CLI envelope and imports) and must reject anything outside the closed sets without panicking.

## Canonical data touched

None directly. It defines the shapes other crates store.

## Public contracts / interfaces

- `ProjectId`, `TaskId`, `RunId`, `EventId`, `ActorId`, `DeletionUnitId`, `ContentId`, `SourceId`, `SourceSpanId`, `SecurityDecisionId`, `CostRecordId`, `AnyId`; text form `<prefix>_<ULID>`, one canonical form per ID.
- `IdSource` (trait), `SystemIdSource` (monotonic wall-clock ULIDs). `IdSource` and `Clock` are the two seams through which tests make IDs and times deterministic.
- `PrivacyClass`, `ExecutionIntent`, `Origin`, `InstructionAuthority`, `RiskClass`, `ProjectTrust`: `as_str`, `FromStr`, `Display`, serde, all from one table.
- `Provenance`, `ContentRef`, `SchemaVersion`.
- `Clock` (trait), `SystemClock`, `Timestamp` (UTC milliseconds).

## Allowed dependencies

`serde`, `thiserror`, `ulid`. Test-only: `proptest`, `serde_json`.

## Forbidden dependencies

Every other Rikuri crate (domain stays at the bottom of the dependency graph, Architecture §45.1); database, HTTP, async runtime, process or OS crates.

## Failure and degraded behavior

Parsing returns typed errors (`IdError`, `UnknownVariant`, `ProvenanceError`, `ZeroSchemaVersion`) and never panics; property tests cover arbitrary input. `SystemIdSource` returns `IdError::SourceUnavailable` if its lock is poisoned or a millisecond's ULID space is exhausted, rather than issuing a duplicate or nil ID.

## Health / last-success signal

N/A at runtime: a pure library with no state, work queue or background activity. Its health is its gate: the unit and property tests in the full gate. Components that issue IDs report ID-source failures through their own health.

```text
state: N/A (pure library)
last_attempt / last_success / last_error / consecutive_failures / work_completed: N/A
```

## Architecture / ADR links

- Invariants: Architecture §4 invariants 2 (stable identity), 7 (zero instruction authority for retrieved content), 11 (deletable content referenced by ID), 30 (privacy intent), 31 (read never implies execute); §8 (referenced-content events); §19.1 (provenance fields).
- ADRs: [ADR 0001 — Stable ID format](../../docs/architecture/adr/0001-stable-id-format.md).

## Threat / control links

- Threat: imported or model text claims instruction authority. Control: authority derived from origin; raised authority refused on deserialization. Test: `authority::tests::stored_records_cannot_raise_authority`.
- Threat: stored or supplied bytes, including a record of a past live command, act as the current user's command. Control: deserialization always grants `none`. Tests: `authority::tests::loaded_records_never_carry_authority`, `authority::tests::a_live_command_loses_its_authority_through_storage`, `authority::tests::the_wire_carries_the_real_authority`.
- Threat: derived data loses its source restriction. Control: `PrivacyClass::combine` takes the strictest class. Test: `privacy::tests::combining_never_lowers_restriction`.
- Threat: two text forms of one ID fork an idempotency key or reference. Control: canonical-only parsing. Test: `tests/id_properties.rs`.

## Required tests

- [x] unit
- [x] failure paths (malformed, wrong-kind, non-canonical, nil, unknown variants, raised authority)
- [ ] shared contract suite — N/A: no replaceable implementation behind a trait here except `IdSource`, whose deterministic fixture arrives with `rikuri-testkit` (S00.08)
- [x] invariant/property tests (`tests/id_properties.rs`)
- [ ] replay/golden fixtures — N/A until events exist (S00.06)
- [ ] integration/smoke — the real callers are `rikuri-storage`, `rikuri-ledger` and `rikurid` (S00.05–S00.07); this crate is not complete until they use it

## Documentation / help entry points

This file; rustdoc on each module.

## Known limitations / explicit debt

- No caller yet. Wiring point: `rikuri-storage` and `rikuri-ledger` in Stage 00; the Stage 00 gate cannot close while any type here lacks a real caller (Stage Plan stop-the-line: "module passes unit tests but has no real caller → NOT COMPLETE").
- `SystemIdSource` is monotonic within one process only. Cross-process ordering relies on wall-clock time; IDs stay unique (80 random bits) but not strictly ordered across processes.
