# Build Record — Loaded `Provenance` never carries instruction authority

| Field | Value |
|---|---|
| Change | A `Provenance` deserialized from bytes always has `instruction_authority: none`; only `Provenance::new(Origin::UserLive, …)`, called by the kernel at the authenticated command boundary, carries `current_user` |
| Date | 2026-10-07 |
| Author / agent session | Claude Code session for Henry Chen |
| Branch | `fix/provenance-loaded-authority` |
| Commits | fill at close-out |
| Stage Gate | [Stage 00](../stage-gates/foundation0/00-bootstrap-kernel.md), S00.04 |
| Related ADR / requirement | S00.04; Architecture §19.1 (LOCKED), §27.4, §4 invariant 7; PRD SEC-003; [S00.04 record](2026-10-05-s00-04-rikuri-domain.md) deferral "`Provenance` derives its authority from its own `origin` field" |
| Status | IN PROGRESS |

**Risk note (protected core: instruction authority).** This changes how `rikuri-domain` grants instruction authority, a security boundary. Expanded risk and negative-test evidence below.

## 1. Plan

- [x] One-sentence statement: deserialization of `Provenance` keeps the recorded origin but always yields `instruction_authority: none`, so stored or supplied bytes can never act as the current user's command.
- [x] Defect (found by the pre-merge verification of PR #7, 2026-10-06): `{"origin":"user_live","instruction_authority":"current_user",…}` deserializes with `CurrentUser` authority. Architecture §19.1 is LOCKED: "imported/retrieved/historical content has `instruction_authority: none` … even text previously written by the user. Only the current authenticated user command and trusted Rikuri policy create normal instruction authority." A record read back from storage is historical by definition.
- [x] Exposure: latent. Nothing on `main` deserializes `Provenance`; the committed `stage-00/kernel` work (storage, ledger) does not use it (`git grep` on that branch, 2026-10-07).
- [x] Owner decision (2026-10-07, in session), of three options put: **loaded records get no authority** (chosen); two separate types (`RecordedProvenance` without authority, `Provenance` without `Deserialize`); a Module Contract rule only (not enforced).
- [x] Blast radius: `crates/rikuri-domain/src/authority.rs` (deserialization path and docs), the crate's Module Contract README; no caller exists yet.
- [x] **I will know this works when** new regression tests, committed red before the fix, show that every consistent stored record loads with authority `none` (including `user_live`/`current_user`) and that a live command's provenance does not keep its authority through serialize → deserialize; the existing refusal of inconsistent claims (`stored_records_cannot_raise_authority`) still passes; the tests are unchanged by the fix; the full gate passes locally and in CI.

## Wire semantics (verifier on PR #13, 2026-10-07)

The verifier passed PR #13 at `a75e8a0` (no load path yields authority: 18 origin × authority combinations, nested types, `serde_json::Value`, `toml`) but found that a loaded record, authority `none` in memory, serialized as `"instruction_authority":"current_user"` (the capture-time value chosen in §2 below). Any reader of the bytes other than this type's own `Deserialize` (ledger tools, the S00.07 JSON envelope, UIs, exports, S02.02 context packets) would see historical content labelled `current_user`, against §19.1 on the wire. Not exploitable yet: nothing serializes `Provenance`.

**Owner decision (2026-10-07, in session):** write the real authority. Serialization writes the value's actual authority (a loaded record `none`, a live command `current_user`); deserialization accepts the authority the origin implies or `none`, still refuses any raise, then grants `none`. The fix-lock was released to update one over-strict expectation in `stored_records_cannot_raise_authority` (it refused `user_live`/`none`, a lowered authority) and to add `the_wire_carries_the_real_authority`; re-engaged after the tests commit.

```text
# Failing before the wire fix (tests updated; code as at a75e8a0)
test authority::tests::the_wire_carries_the_real_authority ... FAILED   left: "current_user"  right: "none"
test authority::tests::stored_records_cannot_raise_authority ... FAILED  {"origin":"user_live","instruction_authority":"none",…} refused
test result: FAILED. 5 passed; 2 failed
```

## Risk and negative evidence

- Allowed: `Provenance::new(Origin::UserLive, …)` still yields `CurrentUser` (live boundary); every other origin `none`.
- Denied: any bytes, including a faithful serialization of a live command, load with `none`; a record whose authority disagrees with its origin is still refused, so tampering stays visible rather than silently downgraded.
- Consequence by design: a live `Provenance` no longer round-trips equal through storage (authority drops to `none`). Audit keeps the recorded origin and the authority field as written.
- Not covered: `Provenance::new` is public, so trusted in-process code can still create live authority; keeping that call at the authenticated boundary is the kernel's job (S00.07 and later), recorded in the Module Contract.

## Evidence log

```text
# Failing before the fix (tests written; nothing fixed), 2026-10-07
$ cargo test -p rikuri-domain --lib authority::
test authority::tests::a_live_command_loses_its_authority_through_storage ... FAILED   left: CurrentUser  right: None
test authority::tests::loaded_records_never_carry_authority ... FAILED                 left: CurrentUser  right: None
test authority::tests::stored_records_cannot_raise_authority ... ok   (inconsistent claims refused, unchanged)
test result: FAILED. 4 passed; 2 failed
```

## 2. Build (fix-lock engaged after `c77c8a9`)

- `TryFrom<ProvenanceRecord>` validates as before (the recorded authority must be the one the origin implies, so a tampered record is refused) and then grants `none`.
- *Superseded by the wire fix below:* `From<Provenance> for ProvenanceRecord` first wrote the authority *at capture*, `origin.content_authority()`, so that a loaded `user_live` record would not save as `user_live`/`none`, which the then-strict validation refused. The verifier showed this labels historical content `current_user` on the wire.
- Wire fix (after `9304937`, fix-lock re-engaged): serialization writes the value's real authority (`p.instruction_authority`); validation accepts the authority the origin implies or `none` and refuses any raise; a loaded record still gets `none`. Bytes for live-constructed values are unchanged.
- Doc comments and the Module Contract (`crates/rikuri-domain/README.md`): authority never comes from bytes; decide with `instruction_authority()`, never from `origin()`; `Provenance::new(Origin::UserLive, …)` only at the authenticated boundary; threat/control/test line added.
- The [S00.04 record](2026-10-05-s00-04-rikuri-domain.md)'s deferral row and the Stage 00 follow-up bullet are marked resolved.

```text
# After the fix (tests unchanged since c77c8a9: the diff touches docs comments, TryFrom and From only)
$ cargo test -p rikuri-domain
authority::tests: a_live_command_loses_its_authority_through_storage ok · loaded_records_never_carry_authority ok
                  stored_records_cannot_raise_authority ok · only_a_live_user_command_has_authority ok · 2 more ok
test result: ok. 26 passed (unit) · 4 passed (properties)

# Scratch consumer crate (path dependency, pinned toolchain; not committed)
stored once : {"origin":"user_live","instruction_authority":"current_user","privacy_class":"normal"}
stored twice: (same bytes: true) · loaded authority: none / none · loaded twice equal: true
imported/current_user        -> refused: provenance with origin imported cannot carry instruction authority current_user
user_live/none               -> refused (not written by this code; a hand-edited record)
user_live/trusted_policy     -> refused
```

```text
# After the wire fix (tests unchanged since 9304937: the diff touches doc comments, TryFrom and From only)
$ cargo test -p rikuri-domain
authority::tests: the_wire_carries_the_real_authority ok · stored_records_cannot_raise_authority ok
                  loaded_records_never_carry_authority ok · a_live_command_loses_its_authority_through_storage ok · 3 more ok
test result: ok. 27 passed (unit) · 4 passed (properties)
```
