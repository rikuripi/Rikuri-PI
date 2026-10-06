# Build Record — S00.04 `rikuri-domain`: stable IDs and kernel vocabulary

| Field | Value |
|---|---|
| Change | First workspace crate: stable typed IDs (ADR 0001, Proposed), privacy classes and execution intent, provenance with instruction authority, risk classes, project trust, deletion units and content references, schema versions |
| Date | 2026-10-05 |
| Author / agent session | Claude Code session for Henry Chen |
| Branch | `stage-00/bootstrap` |
| Commits | fill at close-out |
| Stage Gate | [Stage 00](../stage-gates/foundation0/00-bootstrap-kernel.md) |
| Related ADR / requirement | S00.04, S00.09 (contract), S00.10 (ADR 0001); Architecture §4, §8, §19.1, §21.3, §27.6, §45.1; PRD §10.1, SEC-003, SEC-023 |
| Status | IN PROGRESS |

## 1. Plan

- [x] One-sentence statement of change: add `crates/rikuri-domain`, the pure-type crate every kernel crate will use, with the vocabulary S00.04 lists and nothing else.
- [x] Blast radius: new crate only; no canonical data exists yet. Touches the authority model (instruction authority), privacy classification and project trust as *types*; no decisions are made here. Nothing irreversible.
- [x] Current behavior characterized: no Rust code existed (`git ls-files '*.rs'` empty on `main` at 95eaaac).
- [x] Verification defined up front — **I will know this works when** every enum's wire name, `Display`, `FromStr` and serde agree and match the Architecture §19.1 / PRD §10.1 spellings; IDs have exactly one accepted text form and parsing arbitrary text never panics (property test); provenance cannot be constructed or deserialized with authority above what its origin allows (`trusted_policy` never; `current_user` only for `user_live`); derived privacy never falls below its inputs; clippy (with `unwrap`/`expect`/`dbg`/`todo`/print denied outside tests) and the full gate pass.
- [x] Architecture-impact check:
  - [x] invariant / ADR — ADR 0001 Proposed (stable ID format); invariants 2, 7, 11, 30, 31 encoded as types
  - [x] API / schema / event format — wire names fixed now; storage and ledger adopt them
  - [x] canonical vs derived data — N/A, no data
  - [x] privacy / instruction authority / security — authority derivation and strictest-class combination
  - [x] deletion / backup / restore — `ContentRef` + `DeletionUnitId` so events never embed content
  - [x] budget / cost — N/A (`CostRecordId` only)
  - [x] nondeterministic AI / replay — `IdSource` trait so tests use deterministic IDs
  - [x] documentation / help / RAG — Module Contract README, ADR 0001, rustdoc
  - [x] module ownership / dependency direction — no Rikuri dependencies (Architecture §45.1)
- [x] Relevant threat/control/test: content claiming authority (F7 class) → origin-derived authority, refusal on deserialization; derived-data downgrade (SEC-023) → `combine`. Tests named in the Module Contract.
- [x] Existing solution review: `ulid` 3.0.0 (MIT, maintained, dylanhart/ulid-rs) for encoding/generation; `serde` 1.0.229, `thiserror` 2.0.21 (MIT/Apache-2.0). Alternatives in ADR 0001 (UUIDv7 via `uuid`). Custom code is limited to the prefix/typing layer, which no crate provides with Rikuri's canonical-only rule.
- [x] Authoritative state change? No.

## 2. Build

- [x] Smallest coherent implementation completed: `src/{id,privacy,authority,risk,trust,deletion,schema,wire}.rs`, `tests/id_properties.rs`.
- [x] Inputs crossing trust boundaries validated: every `FromStr`/serde path validates (closed enums, canonical IDs, provenance authority, schema version ≥ 1).
- [x] New code is typed and cohesive; largest file `id.rs` ≈ 330 lines.
- [x] Old/replaced implementation removed: N/A, new.
- [x] Module Contract: [`crates/rikuri-domain/README.md`](../../crates/rikuri-domain/README.md).
- [x] Security question: touches instruction authority and privacy classification as types. Allowed path: `user_live` → `current_user`; denied: every other origin, and any stored record claiming more. Tested both ways (`authority::tests`).

## 3. Debug — N/A (not a defect)

## 4. Verify

- [x] Unit tests including failure paths: 22 unit tests (see evidence log).
- [ ] Shared contract suite — N/A: no replaceable adapter in this crate; `IdSource`'s fixture implementation is tested when `rikuri-testkit` exists (S00.08).
- [x] Invariant/property test: `tests/id_properties.rs`, 4 properties (total canonical parsing over arbitrary text; issued IDs round-trip only as their kind; text order equals ID order; every registered prefix parses).
- [ ] Replay/golden fixture — N/A until events exist (S00.06).
- [ ] Running-system probe — N/A: a library with no runtime. Its real callers are S00.05–S00.07.
- [ ] End-to-end action trace — Not yet proven; arrives with T00.09.
- [x] Recovery/fallback — N/A.
- [x] Regression guard proven to fail against broken behavior: see evidence log (authority derivation deliberately broken in a scratch copy → test fails).
- [x] Cost/performance — N/A.

## 5. Close Out

- [ ] Full required gate green — local `RESULT: PASS (11 steps)`; CI run recorded in the bootstrap Build Record.
- [ ] Scrap Sweep complete; deleted artifacts listed:
- [x] No unexplained skipped/disabled tests or new unowned debt: ratchet baseline 0/0/0.
- [x] Documentation updated: Module Contract, ADR 0001.
- [x] Traceability: threat/control/test links in the Module Contract.
- [ ] Stage Gate Record updated with this change/evidence.
- [ ] Commit message explains WHY; Build Record committed with change.

## Skipped / Deferred / Resequenced

| Item | Disposition | Reason | Dependency impact | Owner | New stage/ordinal | Reactivation / acceptance condition |
|---|---|---|---|---|---|---|
| Real caller for `rikuri-domain` types | RESEQUENCED within Stage 00 | A pure-type crate is used by the storage, ledger and daemon crates that follow | S00.04 is not complete until a real caller exists; S00.05–S00.07 must use these types rather than redefine them | Stage 00 implementer | S00.05–S00.07 | `rikurid` stores and emits these IDs/enums through the T00.09 vertical slice |
| ADR 0001 acceptance | Owner decision pending | The Stage 00 exit requires ADR 0001–0004 accepted | None before data exists; reversal path in the ADR | Henry Chen | Stage 00 exit | Owner marks ADR 0001 Accepted |

Execution-evidence fields (run/outcome/profile identity): N/A, no execution path.

## Evidence log

```text
filled at close-out
```
