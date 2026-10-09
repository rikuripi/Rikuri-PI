# Build Record — W1: pin where each crash point sits

| Field | Value |
|---|---|
| Change | Make each Stage 00 crash point prove its own position: the hook is told what the command has done so far, and the in-process and real-process matrices assert it per point |
| Date | 2026-10-09 |
| Author / agent session | Claude Code (Opus 5.5) for Henry Chen |
| Branch | `stage-00/w1-crash-point-placement`, from `main` at `f8c4e95` (after PR #21) |
| Commits | this PR's commits on the branch |
| Stage Gate | [Stage 00](../stage-gates/foundation0/00-bootstrap-kernel.md); W1; T00.07; closes the [crash matrix record](2026-10-08-stage-00-crash-matrix.md)'s deferral "Crash-point positions (W03–W05)" |
| Related ADR / requirement | Test Strategy §5 (crash points 1, 2, 3, 6); Stage Plan T00.07; ADR 0003 |
| Status | COMPLETE: verifier round 2 PASS; full gate PASS; CI [run 37957881673](https://github.com/rikuripi/Rikuri-PI/actions/runs/37957881673) on PR #22 at `d524c67` |

## 1. Plan

- [x] One-sentence statement: a crash point that moves (point 2 before the plan or after the event append, point 1 into the transaction, point 3 before the commit) must turn a test red, as recovery at each point is only proven where the point actually sits.
- [x] Owner request 2026-10-09: "Contt. with W1" (the W1 row names this tripwire as pending).
- [x] Current behaviour characterized: verifier round 6 of PR #21 moved point 2 before `plan` (W03) and after the event append (W04), and point 1 into the transaction (W05); every test stayed green, because points 1 and 2 both expect "no change" and the hook sees only the point's name.
- [x] **I will know this works when:**
  - the hook receives a `Progress` (transaction open, state written, events appended, idempotency recorded, committed) read from facts when the point is reached (after verifier round 1: flags set on lines next to the steps let a point move without crossing them), not from flags the kernel sets by hand;
  - an in-process test records the progress at every point of every consequential command and matches Test Strategy §5: point 1 nothing yet; point 2 transaction open and state written, no event appended; points 3 and 6 committed with events and the idempotency record (no record with idempotency off);
  - the real daemon's stop marker carries the progress and the crash matrix asserts it in every cell;
  - mutants W03, W04, W05 (PR #21 verifier) and a point-3-before-commit mutant are caught;
  - the full gate passes and the independent verifier finds no unresolved Important finding.
- [x] Architecture impact: `CrashHook::at` gains a `Progress` argument (crate-internal test seam; production uses `NoCrash`, which ignores it). No canonical data, transaction or wire change. The stop marker line gains fields after its existing prefix.
- [x] Authoritative state change: none.

## 2. Build

- [x] `crash::Progress` (five fields; `Display` as `tx=… state=… events=… idem=… committed=…`); `CrashHook::at(point, command, progress)`. `Kernel::consequential` computes each point's progress from facts at the call: `transaction_open` from the store's new `Store::in_transaction()`; `state_written` from SQLite's `total_changes()` against its value when the transaction began; `events_appended` from the ledger's newest sequence number (`rikuri_ledger::tail(tx, 1)`) against its value when the transaction began (after verifier round 2; first a counter kept by hand); `idempotency_recorded` from the result of writing the record; `committed` once `store.write` has returned. A replay reports `committed` only. (First version: flags set by hand next to each step; verifier round 1 showed literal moves that cross no flag line survive.)
- [x] `rikuri-storage`: `Store::in_transaction()`, true while this thread is inside one of the store's transactions (the guard `refuse_reentry` already used); test in `tests/reentrancy.rs`; Module Contract updated.
- [x] `HangAtPoint`'s stop marker prints the progress after its existing prefix.
- [x] Tests: `apps/rikurid/tests/crash_point_placement.rs` (every point of every consequential command, idempotency on and off, against Test Strategy §5 written out in the test; a replay reports no write); `crash_matrix.rs` checks the marker's progress in every cell and in T00.08; T00.06's hook takes the new argument.
- [x] `FailAtPoint` removed: nothing used it (T00.06 has its own hook), and cargo-mutants showed every condition in it untested. S00.08's testkit owns test injectors. Reversible: it is a few lines in git history.
- [x] Docs: `rikurid` and `rikuri-storage` Module Contracts; crash matrix record (deferral DONE, `FailAtPoint` note, cargo-mutants filter correction). The Stage Gate Record and the W1 row follow CI on the PR.

## 3. Debug

- [x] Before: PR #21's verifier moved point 2 before `plan` (W03), after the event append (W04) and point 1 into the transaction (W05); every test stayed green (crash matrix record, round 6).
- [x] After, first version: 13 placement and progress mutants caught.
- [x] Independent verifier round 1 (FAIL): **I-1** the verifier's literal moves survived: point 1 to the first line inside the write closure (W05, the hook still got the outer default progress), point 2 to just after the append loop but before the `events_appended` line (W04), and point 2 moved before `plan` together with its flag line (W03). My mutants had all crossed a flag line. Minor: the Module Contract still said placement was deferred and had no entry for the new test; a stale `FailAtPoint` mention in the crash matrix record; the record's cargo-mutants command lacked `--timeout` (with the default 20 s, four feature-pass mutants time out at the matrix's own 20 s deadline); `Progress::fmt` is compiled by default (only its callers are feature-only); a doc comment left above the wrong function.
- [x] Responses: progress is read from facts at each call (above), so moving a call moves the facts with it; 12 literal and fact mutants against the new code, all caught (evidence log), including the verifier's three literal moves. The Minors are corrected. Root-cause class: **a tripwire that checks the kernel's own bookkeeping instead of the state it describes**; the sibling sweep found none elsewhere in the W1 tests (the outcome checks read the store and ledger after a real restart).

## 4. Verify

- [x] Placement tests pass in process and against the real daemon (evidence log).
- [x] Mutants, first version (flags set by hand): 13 of 13 caught; verifier round 1 then showed literal moves that cross no flag line survive.
- [x] cargo-mutants 27.1.0 on `crash.rs` and the kernel's W1-path functions, in two passes because `HangAtPoint` and the marker exist only with `crash-injection`: default features over the workspace (19: 8 caught, 3 unviable, 8 "missed": `HangAtPoint`, which a default build does not compile, and `Progress::fmt`, which it compiles but only feature-only code calls) and `--features crash-injection` with the crash matrix (9: 8 caught, 1 unviable). Union: **0 missed**. Unviable (4): `&&`→`||` in three `if let` chains, and `Ok(Default::default())` for `HangAtPoint`, which has no `Default`. No quota errors in either run's logs.
- [x] Full gate: `RESULT: PASS (15 steps)`.
- [x] cargo-mutants rerun on the fact-based code (`--timeout 600`, `-j 1`): default pass over `crash.rs` and the kernel W1-path functions (the `in Store::in_transaction` filter matched none of that function's mutants; the corrected run below covers it, and `Store::health` appears because cargo-mutants 27.1.0 does not apply `--re` to "delete field" mutants): 30 tested, 18 caught, 3 unviable, 9 missed: the 8 feature-only `HangAtPoint`/`Progress::fmt` mutants, which the feature pass catches (9 tested, 8 caught, 1 unviable), and one in storage (below).
- [x] Filter defect found and corrected: `--re 'in Kernel::(…)'` skips cargo-mutants' whole-function replacements, which it names `replace Kernel::health -> …` without "in". That narrowed PR #21's kernel bar too (correction noted in the crash matrix record and the Module Contract). Run with the corrected filter: `Kernel::handle`, `consequential`, `check_consistency`, `quarantined` (×3), `health` and `Store::in_transaction` (×2): 12 caught, 2 unviable (`Response` and `Applied` have no `Default`), 1 missed: `Store::health`'s poisoned-lock fallback without its `last_error` (deferred below).
- [x] Independent verifier round 2: **PASS**, no Critical or Important finding; I-1 resolved (its literal M1, M3, M4 and new moves caught), counts reproduced, production cost of the facts measured at 0.16% of a committed write in release. Minor: (m1) `events_appended` still came from a hand counter, so a point placed between an append and the counter survived (N3, N4); (m2) wording that still described the rejected design (crash.rs, README, the placement test header); (m3) the record overstated the `in_transaction` coverage of one run and did not explain why `Store::health` appeared; (m4) a stale "13 of 13" in §4; (m5) the crash matrix record's DONE row ahead of this record's close-out. Observation: `in_transaction` reflects the store's guard, not SQLite's autocommit state; a raw `COMMIT` through `Tx`'s deref (recorded in the storage contract as not yet refused) would make them disagree.
- [x] Responses, round 2: (m1) `events_appended` is read from the ledger (above); N3, N4 and the earlier moves rerun, 13 of 13 caught; cargo-mutants on `Kernel::consequential` (both naming forms): 16 tested, 14 caught, 2 unviable, 0 missed. (m2)–(m4) corrected. (m5) settled by this record's close-out. The autocommit observation is noted; it rides on the existing storage deferral for raw transaction control.

## 5. Close Out

- [ ] Gate, Scrap Sweep, docs, commits and PR when the owner asks.

## Skipped / Deferred / Resequenced

| Item | Disposition | Reason | Dependency impact | Owner | New stage/ordinal | Reactivation / acceptance condition |
|---|---|---|---|---|---|---|
| `Store::health` poisoned-lock fallback: its `last_error` is unasserted (cargo-mutants) | DEFERRED | Reachable only by a panic while holding the store's health lock, so only an inline test in `rikuri-storage` can poison it; outside W1. With the field gone, health still is not green (no success recorded), only the reason is lost | none for W1 | Stage 00 implementer | The next `rikuri-storage` change, or S00.09's contract review | An inline test poisons the lock and asserts the snapshot's `last_error` |
| Points 3 and 6 coincide | Kept (recorded in the crash matrix record) | Nothing runs between the commit and the response in the kernel, so their progress is the same | none | Stage 00 implementer | When a response path sits between them | Their progress differs and a test pins it |

## Evidence log

```text
$ cargo test -p rikurid --test crash_point_placement --test t00_06_no_duplicate_effects → 2 + 4 passed
$ cargo test -p rikurid --features crash-injection --test crash_matrix --target-dir target/crash-injection → 6 passed
# mutants (scratch copy), each against rikurid and rikuri-cli tests and crash_matrix:
P1 point 2 before plan (W03)            CAUGHT placement test + T00.07 marker check
P2 point 2 after the event append (W04) CAUGHT placement test + T00.07
P3 point 1 inside the transaction (W05) CAUGHT placement test, replay test + T00.07
P4 point 3 before the commit            CAUGHT placement, replay, T00.06 every-point + T00.07
P5 point 6 before the commit            CAUGHT placement, replay, T00.06 every-point + T00.07
F1–F6 a progress flag never set, or wrong after the commit   CAUGHT (all six)
D1 marker swaps events/idem             CAUGHT T00.08 (idempotency off tells them apart)
D2 marker prints no progress            CAUGHT T00.07, T00.08
# first cargo-mutants attempt on crash.rs + kernel W1 path: /tmp quota exceeded (exit 1), discarded;
#   its "missed" list was mostly feature-gated HangAtPoint (no feature in a default pass) and FailAtPoint (no caller)
# FailAtPoint removed (no caller); scratch caches cleared; rerun one job at a time:
$ T=$(mktemp -d /tmp/cm.XXXX); TMPDIR=$T cargo mutants --file apps/rikurid/src/crash.rs --file apps/rikurid/src/kernel.rs \
    --re '^apps/rikurid/src/crash\.rs|in Kernel::(consequential|check_consistency|quarantined|health|handle)\b' \
    --test-workspace=true -j 1 --timeout 600 --output <scratch>/cm5a; rm -rf -- "$T"
19 mutants tested in 2m: 8 missed, 8 caught, 3 unviable      (missed: Progress::fmt and HangAtPoint, feature-only)
$ T=$(mktemp -d /tmp/cm.XXXX); TMPDIR=$T cargo mutants --file apps/rikurid/src/crash.rs --re '^apps/rikurid/src/crash\.rs' \
    --features crash-injection --test-package rikurid -j 1 --timeout 600 --output <scratch>/cm5b -- --test crash_matrix; rm -rf -- "$T"
9 mutants tested in 2m: 8 caught, 1 unviable                 (catches every one of pass A's 8 except the uncompilable Default)
$ scripts/quality/full-gate.sh → RESULT: PASS (15 steps)

# verifier round 1 → FAIL (I-1: literal W05/W04 and W03-with-flag survive). Fix: progress from facts.
$ cargo test -p rikuri-storage --test reentrancy → 5 passed (+ in_transaction_is_true_only_inside_a_transaction)
$ cargo test -p rikurid --test crash_point_placement --test t00_06_no_duplicate_effects --test consistency_replay → 2, 4, 48 passed
$ cargo test -p rikurid --features crash-injection --test crash_matrix --target-dir target/crash-injection → 6 passed
# mutants against the new code (moves generated from the source text, so they are true moves):
L1 point 1 to the first line inside the write closure (W05 literal)  CAUGHT placement, replay + T00.07
L2 point 2 right after the append loop (W04 literal)                 CAUGHT placement + T00.07
L3 point 2 and its counter before plan (W03 with its line)            CAUGHT placement + T00.07
L4 point 2 after the first append                                     CAUGHT placement + T00.07
L5/L6 point 3/6 inside the transaction                                CAUGHT placement, T00.06 + T00.07
L7 point 1 after the commit                                           CAUGHT placement, T00.06 + T00.07
F1 counter never incremented; F2 change count >=; F3 recorded false;
F4 transaction never reported open; F5 committed not set              CAUGHT (all five)
$ T=$(mktemp -d /tmp/cm.XXXX); TMPDIR=$T cargo mutants --file apps/rikurid/src/crash.rs --file apps/rikurid/src/kernel.rs \
    --file crates/rikuri-storage/src/store.rs \
    --re '^apps/rikurid/src/crash\.rs|in Kernel::(consequential|check_consistency|quarantined|health|handle)\b|in Store::in_transaction' \
    --test-workspace=true -j 1 --timeout 600 --output <scratch>/cm6a; rm -rf -- "$T"
30 mutants tested in 2m: 9 missed, 18 caught, 3 unviable     (8 feature-only + Store::health fallback)
$ … --features crash-injection --test-package rikurid -j 1 --timeout 600 … -- --test crash_matrix   (crash.rs)
9 mutants tested in 2m: 8 caught, 1 unviable
# the filter skipped whole-function replacements; corrected:
$ … --re 'replace Kernel::(handle|consequential|check_consistency|quarantined|health) |replace Store::in_transaction ' …
15 mutants tested in 65s: 1 missed, 12 caught, 2 unviable    (missed: Store::health poisoned-lock fallback's last_error)

# verifier round 2 → PASS; Minor m1: hand counter for events_appended (N3, N4 survive). Fix: ledger's newest seq.
$ cargo test (placement 2, T00.06 4, replay 48, health 5; crash_matrix 6) → all passed
N4 point 2 after the append loop; N3 after the first append; L3 before plan; L7 point 1 after commit;
L1 point 1 in the transaction; L5/L6 points 3/6 in the transaction; F1 events >=; F2 events never;
F3 changes >=; F4 recorded false; F5 tx never open; F6 committed false            13 of 13 CAUGHT
$ … cargo mutants --file apps/rikurid/src/kernel.rs --re 'Kernel::consequential' --test-workspace=true -j 1 --timeout 600 …
16 mutants tested in 61s: 14 caught, 2 unviable                (0 missed)
```
