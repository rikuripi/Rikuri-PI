# Build Record — S00.08 `rikuri-testkit`

| Field | Value |
|---|---|
| Change | One shared test kit (`FixtureClock`, `FixtureIds`, `CrashInjector`) replacing the fixtures each test file defined for itself, with the crash-point vocabulary moved to `rikuri-domain` so the kit and the kernel share one copy of it |
| Date | 2026-10-09 |
| Author / agent session | Claude Code (Opus 5.5) for Henry Chen |
| Branch | `stage-00/s00-08-testkit`, from `main` at `b471c0e` (after PR #22) |
| Commits | this PR's commits on the branch |
| Stage Gate | [Stage 00](../stage-gates/foundation0/00-bootstrap-kernel.md); S00.08; S00.09 (testkit Module Contract); closes the [crash matrix record](2026-10-08-stage-00-crash-matrix.md)'s row "S00.08 `CrashInjector`" |
| Related ADR / requirement | Stage Plan S00.08 (as resequenced 2026-10-05: `FakeModelAdapter` → S02.07, `FakeTool` → S04.01); AGENTS.md (real callers, no unexplained alternate implementation) |
| Status | COMPLETE locally (verifier round 1 PASS; full gate PASS); CI Not yet proven |

## 1. Plan

- [x] One-sentence statement: tests across the workspace get their clocks, IDs and crash hooks from one crate, `rikuri-testkit`, and the duplicated per-file fixtures go away.
- [x] Owner decisions 2026-10-09: "Contt. with W1" led here (W1 deferred its `CrashInjector` to S00.08); the crash-point vocabulary (`CrashPoint`, `Crashed`, `Progress`, `CrashHook`) lives in `rikuri-domain`, beside the other injection seams `Clock` and `IdSource` ("rikuri-domain (Recommended)"). The reason first given was that a testkit depending on `rikurid` would compile a second, mismatched copy of the trait in `rikurid`'s tests. Verifier round 1 showed that holds only for a crate's own unit tests (`rikurid` has none); integration tests get one copy. The move stands on layering: the kit depends on the vocabulary, never on the code it tests, and storage's and the ledger's suites do not pull in the daemon. The owner, told of the corrected premise, confirmed the decision on 2026-10-09 ("Keep it in domain").
- [x] Current behaviour characterized: 13 test files define their own fixtures: fixed clocks (8), sequential IDs at a fixed time (3), a stepping clock (1), a switchable failing ID source (1), and crash hooks (`ArmedOnce`, `Recorder`). All sequential sources start at 1.
- [x] **I will know this works when:**
  - `rikuri-domain` exports the crash vocabulary and `rikurid::crash` re-exports it, so existing callers compile unchanged; `NoCrash` and `HangAtPoint` stay in `rikurid`;
  - `rikuri-testkit` provides `FixtureClock` (fixed or stepping), `FixtureIds` (sequential at a fixed time, switchable to failing) and `CrashInjector` (arms one point once, optionally for one command; records every point it sees with its progress), each with its own tests;
  - every per-file fixture is replaced, except `rikuri-domain`'s three (recorded below with their real reasons);
  - the testkit has a Module Contract; the domain and `rikurid` contracts are updated; the full gate passes; mutants of the kit are caught; the independent verifier finds no unresolved Important finding.
- [x] Architecture impact: a fifth library crate (test-only: no runtime crate depends on it except as a dev-dependency); domain gains a test-seam module. No canonical data, wire or behaviour change.
- [x] Authoritative state change: none.

## 2. Build

- [x] `rikuri_domain::crash` holds `CrashPoint`, `Crashed`, `Progress` (with its marker `Display`) and `CrashHook`, re-exported at the domain root; `rikurid::crash` re-exports them and keeps `NoCrash` and `HangAtPoint`. Callers compile unchanged.
- [x] `crates/rikuri-testkit` (workspace dependency; depends on `rikuri-domain`, `ulid`): `FixtureClock::{fixed, stepping}`, `FixtureIds::{sequential, failure_switch}`, `CrashInjector::{new, arm, arm_for, seen, clear_seen}`, `T0`; `tests/fixtures.rs` (5 tests); Module Contract `README.md`.
- [x] Callers migrated, fixtures removed: `rikuri-storage` (6 suites), `rikuri-ledger` (1), `rikurid` (`t00_06` `ArmedOnce`, `crash_point_placement` `Recorder`, `health_components` stepping clock, `s00_07_round3` switchable IDs), `rikuri` CLI T00.09 (sequential IDs and fixed clock). `ulid` dropped from four dev-dependency lists that no longer use it.
- [x] Docs: Module Contracts (testkit new; domain, rikurid, storage, ledger, CLI updated), AGENTS.md (five library crates), crash matrix record (S00.08 row DONE).

## 3. Debug

- [x] cargo-mutants' feature pass cannot reach `Progress::fmt` once it lives in `rikuri-domain` (it builds the mutated file's package, which has no `crash-injection` feature). The marker's text form is a contract between daemon and harness, so `rikuri-domain` now pins it directly (`tests/crash_progress.rs`), which also catches that mutant in the default pass.
- [x] The kit's suite did not pin that the injector records the point it crashes at (the doc promises every point reached); an assertion was added before the hand mutants ran.
- [x] Independent verifier round 1: **PASS**, no Critical or Important finding (behaviour preserved file by file, same test and assert counts as `main`; 13 caught, 9 unviable, 0 missed; 14 of 16 own hand mutants and 8 of 8 caller mutants caught). Minor: (1) the stated reason for the domain exception and for the "must move" premise was wrong (two copies arise only in a crate's own unit tests), and two unit-test fixtures in domain's sources were not listed; (2) ADR 0001's follow-up ("`FixtureIds` passes the same ID properties") had no disposition; (3) the domain contract ticked integration/smoke while still saying "No caller yet"; (4) re-arming was not pinned (mutant K16 survived), and the poisoned-lock wording described an unreachable path; (5) nothing mechanically kept runtime crates off the kit. Nits: no protected-path risk note for AGENTS.md; the kit's README cited T00.07/T00.08 as callers; the Stage Gate line "S00.08 not built yet" was not in close-out; `T0` vs the repeated `Timestamp` literal; the domain Purpose and Cargo description did not mention the seams.
- [x] Responses: (1) reasons corrected in this record, both contracts, and to the owner; (2) `fixture_ids_keep_the_adr_0001_id_properties` (1,000 IDs) and a dated note on ADR 0001's follow-up; (3) the limitation rewritten; (4) `arming_again_replaces_a_pending_point` (K16 now caught), wording made accurate; (5) crate policy rule 5 refuses `rikuri-testkit` as any member's normal or build dependency, renamed or target-specific, with three gate self-tests (accepted as a dev-dependency, refused as a runtime dependency, refused renamed as a build dependency). Nits: risk note below, citations fixed, close-out item added, `T0_TIME` replaces the literal in three suites, domain Purpose and description updated.

## 4. Verify

- [x] Kit tests: 5 passed; workspace: every suite passed, 0 failed; crash matrix (crash-injection): 6 passed.
- [x] cargo-mutants 27.1.0 (`-j 1`, `--timeout 600`, short TMPDIR) on `rikuri-testkit/src/lib.rs` and `rikuri-domain/src/crash.rs` over the workspace: 22 tested, 12 caught, 9 unviable, 1 missed (`Progress::fmt`, only feature code called it); after `tests/crash_progress.rs`: `crash.rs` 1 of 1 caught. Unviable (9): `Default::default()` for types without `Default`, and `MutexGuard` constructors that do not exist.
- [x] Hand mutants of the kit's semantics (never disarms, `arm_for` ignores the command, the crashing point not recorded, stepping ignores its step, the failure switch not shared, IDs from 0, `is_armed` false, `clear_seen` keeps): 8 of 8 caught.
- [ ] Full gate; independent verifier.

## 5. Close Out

- [x] Full gate after the round-1 responses: `RESULT: PASS (15 steps)` (contracts and crate policy check 7 crates; the new self-tests run in the gate's self-test step). Scrap Sweep: untracked files are this change's new files and the owner's `rikurinode.code-workspace`; no mutants.out, no proptest-regressions, scratch outside the repository.
- [ ] Commits and PR (owner, 2026-10-09: "Commit and PR, I'll merge"; the owner merges); after CI: tick S00.08 and the "Module Contracts for … testkit" engineering gate in the Stage Gate Record, and replace its line "S00.08 `rikuri-testkit` is not built yet".
- [x] Protected-path risk note (REVIEW.md): `AGENTS.md` (the repository paragraph names five library crates), `scripts/quality/crate-policy.sh` (rule 5) and `scripts/quality/tests/crate-checks.sh` (its self-tests), and ADR 0001 (a dated note on a follow-up, the decision unchanged). Risk: a policy rule that refuses legitimate manifests, or a self-test that cannot fail. Mitigation: the rule reads only `[dependencies]`/`[build-dependencies]` (and target tables), the real workspace passes it, and its self-tests include both an accepted and two refused manifests; no existing rule or step changed.

## Skipped / Deferred / Resequenced

| Item | Disposition | Reason | Dependency impact | Owner | New stage/ordinal | Reactivation / acceptance condition |
|---|---|---|---|---|---|---|
| `FakeModelAdapter`, `FakeTool` | RESEQUENCED (owner, 2026-10-05) | Their interfaces do not exist yet | none for Stage 00 | Henry Chen | S02.07, S04.01 | Each fake passes its interface's contract suite |
| `rikuri-domain`'s three local fixtures | Kept, explained | `tests/id_properties.rs` injects arbitrary ULIDs from proptest, which `FixtureIds` does not produce; the unit-test fixtures in `src/id.rs` and `src/deletion.rs` cannot use the kit, because a crate's own unit tests would compile a second copy of domain through it (verifier round 1 proved both: an integration test using the kit compiled and passed, a unit test failed with "multiple different versions of crate rikuri_domain") | none | Stage 00 implementer | — | — |

## Evidence log

```text
$ cargo test -p rikuri-testkit → 5 passed
$ cargo test --workspace --no-fail-fast → every result ok, 0 failed
$ cargo test -p rikurid --features crash-injection --test crash_matrix --target-dir target/crash-injection → 6 passed
$ T=$(mktemp -d /tmp/cm.XXXX); TMPDIR=$T cargo mutants --file crates/rikuri-testkit/src/lib.rs \
    --file crates/rikuri-domain/src/crash.rs --test-workspace=true -j 1 --timeout 600 --output <scratch>; rm -rf -- "$T"
22 mutants tested in 56s: 1 missed, 12 caught, 9 unviable     (missed: Progress::fmt, feature-only caller)
# feature pass on domain/crash.rs refused: "the package 'rikuri-domain' does not contain this feature"
# + crates/rikuri-domain/tests/crash_progress.rs (marker text form, wire names) → 2 passed
$ … cargo mutants --file crates/rikuri-domain/src/crash.rs --test-workspace=true …
1 mutant tested in 17s: 1 caught
# hand mutants of the kit (scratch copy, against rikuri-testkit and rikurid tests): 8 of 8 CAUGHT
#   (H3 and H8 first written with compile errors in the mutant itself; rewritten and rerun)
```
