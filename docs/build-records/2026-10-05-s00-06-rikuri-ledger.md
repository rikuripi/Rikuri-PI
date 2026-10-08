# Build Record — S00.06 `rikuri-ledger`: append-only events and idempotency

| Field | Value |
|---|---|
| Change | Append-only referenced-content event ledger, idempotency records and replay helpers in the canonical database |
| Date | 2026-10-05 |
| Author / agent session | Claude Code session for Henry Chen |
| Branch | `stage-00/storage-ledger`: replayed on 2026-10-08 from the local `stage-00/kernel` branch onto `main` at `c5f0060`, which had gained 41 commits (the PR #7/#11/#13/#17 gate fixes); clean cherry-pick, no conflicts. The old branch and its worktree are kept as a backup |
| Commits | `298e9d6` on the original branch, replayed as `9de5061`; defect fixes `ab41b21` and the following commit ([defect record](2026-10-08-storage-ledger-verifier-defects.md)) |
| Stage Gate | [Stage 00](../stage-gates/foundation0/00-bootstrap-kernel.md) |
| Related ADR / requirement | S00.06, T00.05, S00.10 ([ADR 0003](../architecture/adr/0003-ledger-and-idempotency.md), Accepted 2026-10-08); Architecture §8, §41.1; PRD CORE-002, CORE-004, CORE-007 |
| Status | IN PROGRESS |

## 1. Plan

- [x] One-sentence statement: record what happened, append-only and content-free, and make every consequential command replay-safe by key, inside the same transaction as its state change.
- [x] Blast radius: new crate and tables in the canonical database; audit integrity. No money, execution or network path.
- [x] Current behavior characterized: no ledger existed.
- [x] **I will know this works when** T00.05 passes (append and query in order; consequential events refused without a key; same key + same request replays the stored response; changed request or command conflicts; key cannot be recorded twice; raw UPDATE/DELETE refused on both tables; content-shaped metadata refused; missing references refused; failed transactions append nothing; replay visits all in order; property: exactly the committed events, in order); and deliberately broken key requirement, metadata check, conflict check, triggers and tail ordering each fail a test.
- [x] Architecture impact: ADR 0003 (Proposed); events reference content by ID (deletion compatibility); one transaction with state (Architecture §8).
- [x] Threat/control/test: in the Module Contract.
- [x] Existing solution review: SQLite tables in the canonical database instead of a separate log (no cross-store commit protocol needed); BLAKE3 for request fingerprints. Alternatives in ADR 0003.
- [x] Authoritative state change? Appends only, inside the caller's transaction.

## 2. Build

- [x] `src/event.rs`, `src/idempotency.rs`, `src/lib.rs` (migration); `tests/ledger_contract.rs`.
- [x] Inputs validated: event type, key format, metadata, references; stored rows decoded defensively.
- [x] Module Contract: [`crates/rikuri-ledger/README.md`](../../crates/rikuri-ledger/README.md).
- [x] Security question: audit integrity and content leakage. Allowed: valid appends. Denied: history edits, keyless consequential events, content in metadata, dangling references. Tested both ways.

## 3. Debug

Building the ledger exposed that `AnyId` could not be deserialized from runtime text; fixed in `rikuri-domain` under the [review-fixes record](2026-10-05-s00-gate-review-fixes.md) with a regression test committed first.

## 4. Verify

- [x] T00.05 contract suite: 9 tests including the property.
- [x] Regression guard: five mutants, each failing (evidence log).
- [ ] T00.06/T00.07/T00.12 through `rikurid` (S00.07–S00.08).

## 5. Close Out

- [x] Full required gate green on the branch: Pass, 13 steps, after the replay onto current `main` (evidence log).
- [x] Commit; Stage Gate Record updated (2026-10-08, owner's request).

## Skipped / Deferred / Resequenced

| Item | Disposition | Reason | Dependency impact | Owner | New stage/ordinal | Reactivation / acceptance condition |
|---|---|---|---|---|---|---|
| Real caller | RESEQUENCED within Stage 00 | `rikurid` issues every event | S00.06 incomplete until then | Stage 00 implementer | S00.07 | T00.09 slice appends through the ledger |
| Idempotency retention | DEFERRED | Volume is negligible at Stage 00 | None on correctness | Stage 00 implementer | Stage 01 (with backup/forget) | Retention rule written and tested before records are pruned |

Execution-evidence fields: N/A here; the event shape carries task/run/event IDs, status, references and cost/security slots for T00.12/T02.16.

## Evidence log

```text
$ cargo test -p rikuri-ledger → ledger_contract: 9 passed
$ cargo clippy -p rikuri-ledger --all-targets -- -D warnings → clean

# Mutants (scratch copy)
consequential events without a key accepted → consequential_events_need_an_idempotency_key FAILED
metadata not validated                      → events_cannot_embed_content FAILED
changed request replays                     → idempotency_replays_the_same_request_and_refuses_a_changed_one FAILED
append-only triggers missing                → ledger_rows_are_append_only_even_for_raw_sql FAILED
tail returns newest first                   → append_then_query_in_order, ledger_holds_exactly_the_committed_events FAILED
# 2026-10-08, replayed onto main c5f0060 (branch stage-00/storage-ledger)
$ scripts/quality/full-gate.sh → RESULT: PASS (13 steps), exit 0
  guard 91 tests OK; fmt, build, clippy -D warnings, test, cargo deny, secrets, ratchet (0/0/0) PASS
  CONTRACT checked 3 crate(s); POLICY checked 3 crate(s); SCRAP clean
  tests: rikuri-ledger ledger_contract 9 passed; rikuri-storage migrations 7, repository_contract 5 passed
```
