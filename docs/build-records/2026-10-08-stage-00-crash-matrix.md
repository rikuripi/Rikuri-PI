# Build Record — Stage 00 crash matrix (W1): T00.06, T00.07, T00.08, T00.12

| Field | Value |
|---|---|
| Change | Prove `rikurid` survives a crash at every kernel transaction point with rollback or deterministic recovery and no duplicate effect (W1), prove the check bites, and keep IDs and links stable across crash and replay |
| Date | 2026-10-08 |
| Author / agent session | Claude Code (Opus 5.5) for Henry Chen |
| Branch | `stage-00/crash-matrix`, from `main` at `aef4094` (after PR #20) |
| Commits | this PR's commits on `stage-00/crash-matrix` (no other branch carries this work) |
| Stage Gate | [Stage 00](../stage-gates/foundation0/00-bootstrap-kernel.md); W1; T00.06, T00.07, T00.08, T00.12; closes S00.06's open item; carries the S00.07 deferral "consistency replay does not compare completions with run state" |
| Related ADR / requirement | ADR 0002, ADR 0003 (Accepted); Test Strategy §5 (crash points); Architecture §8 (one transaction), §43; [S00.07 record](2026-10-08-s00-07-rikurid.md) |
| Status | COMPLETE: verifier round 6 PASS; CI [run 37885121628](https://github.com/rikuripi/Rikuri-PI/actions/runs/37885121628) on PR #21 at `c0bd3be` (`GATE crash-matrix: PASS`, `RESULT: PASS (15 steps)`). The macOS job runs the matrix after the merge to `main` |

## 1. Plan

- [x] One-sentence statement: turn the crash points and consistency replay that S00.07 built into automated proof: a real `kill -9` at points 1, 2, 3 and 6 leaves either no change or the whole change and the retry with the same key gives exactly one effect; random command sequences with injected crashes never duplicate an effect; with idempotency off the same checks fail; and task, run and event IDs and links survive crash and replay.
- [x] Owner request 2026-10-08: "Merge #20 when green. Then go to next" (the crash matrix was the proposed next item).
- [x] Blast radius: tests, a test harness and a gate step; one oracle fix in `rikurid`'s consistency replay (health). No production behaviour change other than the replay reporting more problems (which, with the owner's quarantine decision, can quarantine a daemon whose state drifted).
- [x] Current behavior characterized: the crash points, `FailAtPoint`, `HangAtPoint` and `RIKURI_TEST_DISABLE_IDEMPOTENCY` exist (S00.07); the verifier exercised points 1/2/3/6 by hand with a real SIGKILL in rounds 1–3. There is no automated matrix, no property test, no T00.08 regression and no T00.12 test. The consistency replay checks run → event links but not event → run: a run shown as running while the ledger holds its completion, or a changed outcome, passes (red tests below).
- [x] **I will know this works when:**
  - **(B)** the consistency replay reports ledger events about a run that state does not link, and an outcome or verification that differs from the ledger, with red-first tests;
  - **(C) T00.06:** a proptest over random command sequences with in-process crashes at random points and retries with the same key finds no duplicate effect and a clean replay, and the same oracle fails on a fixed sequence with idempotency off;
  - **(D) T00.07:** a real `rikurid` (`crash-injection` build) stopped at each point by `RIKURI_CRASH_AT` and killed with SIGKILL, then restarted unarmed (the same binary without `RIKURI_CRASH_AT`; the verifier also recovered with the production binary), gives one task after the retry (fresh at points 1/2, replayed at 3/6) with a clean replay;
  - **(D) T00.08:** the same harness with idempotency off produces a duplicate that the W1 oracle reports;
  - **(D) T00.12:** across crash and replay of a full task lifecycle the task, run and event IDs in replayed responses are stable and resolve in state and the ledger, run links point at the right events, and a missing reference is refused;
  - the matrix runs as a gate step, mutants of the protections are caught, the full gate passes, and the independent verifier finds no unresolved Important finding.
- [x] Architecture-impact check: canonical data and transactions unchanged; the replay (health) gets stricter; a new gate step compiles and runs the `crash-injection` feature's real-process harness; test builds only.
- [x] Existing solution review: proptest (already a workspace dependency) for T00.06; std process control for SIGKILL (`Child::kill` sends SIGKILL on Unix).
- [x] Mutation testing tool (owner decision 2026-10-08, after four verifier rounds each found the next hand-missed condition: "Install cargo-mutants"): `cargo-mutants` 27.1.0, MIT, <https://github.com/sourcefrog/cargo-mutants>, installed with `cargo install cargo-mutants --version 27.1.0 --locked` into the owner's `~/.cargo/bin`. Trust boundary: a developer tool that builds and tests mutated copies of this repository in a temporary directory; it is not a dependency of any crate and not a gate step (adding it to the gate or CI is a separate owner decision). Failure behaviour: a mutant it cannot build is reported unviable, never counted as caught. Used here on `apps/rikurid/src/health.rs` (the W1 oracle) and the kernel's W1-path functions. Procedure: a short `TMPDIR` (socket paths), `--output` outside the repository, and on every run compare the unviable list with the 7 recorded below: resource exhaustion (a full /tmp) makes viable mutants look unviable, which a "0 missed" summary would hide (verifier round 6 saw exactly that).
- [x] Authoritative state change: none.

## 2. Build

- [x] (A) S00.07 close-out records: S00.04, S00.05 and S00.07 Build Records COMPLETE after PR #20; Stage Gate Record ticks and evidence rows T00.04, T00.05, T00.09, T00.10.
- [x] (B) Consistency replay checks event → run as well as run → event (`health::check_run_events`).
- [x] (C) T00.06 `apps/rikurid/tests/t00_06_no_duplicate_effects.rs`: a test `CrashHook` armed once per command (`ArmedOnce`); a proptest (64 cases, 1–29 commands) over create (four keys, two titles, so replays and conflicts occur), start, step and complete with a crash at a random point or none, the client's retry with the same key after a simulated crash, and random duplicate delivery of a request that already succeeded (it must be a replay with the same result); oracle: a clean consistency replay, one task per create key that succeeded, one `task.created` per task. Plus a deterministic test of every point of every command (fresh retry before commit, replay after, clean replay after each), and the twin that fails with idempotency off.
- [x] (D) `apps/rikurid/tests/crash_matrix.rs` (`[[test]]` with `required-features = ["crash-injection"]`): spawns the real `rikurid` binary, waits for `RIKURI_CRASH_POINT <point> <command>` on stderr, kills it with SIGKILL (`Child::kill`), restarts it unarmed on the same home and retries. T00.07 is 20 cells (4 points × `system.init`, create, start, step, complete; init added after verifier round 1 found it missing), cycling through four outcome/verification endings; T00.08 runs the same crash with `RIKURI_TEST_DISABLE_IDEMPOTENCY`; T00.12 checks IDs and links in every cell and missing references separately. Gate step `crash-matrix` after `smoke` (15 steps); the macOS core CI job runs the same test (see the owner choice below). Docs: README (step list, macOS job), AGENTS.md (repository paragraph, Stage 00 verification), the `rikurid` Module Contract (replay in both directions, required tests, limitations), `full-gate.sh` comment.

## 3. Debug

- [x] (B) reproduced by red tests (evidence log).
- [x] Root-cause class: **a checker that only follows the pointers it is given.** The replay validated each run's links into the ledger, so ledger facts that state forgot (an unlinked completion) or contradicted (an outcome) were invisible. Sibling sweep: tasks (the replay counts created/stepped/started events per task in both directions already) and runs (this fix).
- [x] Independent verifier round 1 (FAIL, no runtime defect): **I1** three `check_run_events` branches (per-run step count, events for a run that does not exist, an unlinked `run.started`) had no test that fails without them; **I2** `system.init` was never crashed although the docs said "every consequential command". Minor: **M1** task → run pointers unchecked (the same root-cause class, inside state), and the owner never compared with `system.initialized`; **M2** R7 caught by T00.08 in only 4 runs of 6; **M3** stale S00.04/S00.05 text; **M4** T00.12 gaps (missing task on step/complete, only success endings); **M5** points 3 and 6 coincide; **M6** "normal daemon" wording; **M7** test-binary count.
- [x] Responses: I1 tests added (green at once, because the code was right; their bite is shown by mutants after the lock release). I2: init added to both matrices. M1 and the owner check fixed tests-first (red run in the evidence log; fix-lock engaged; `health::check_task_runs`, `health::check_owner`). M4: missing-task refusals and four endings added; the harness speaks through `rikuri_api::client`, the CLI's own transport, and T00.09 proves the binary gives the same envelopes. M2, M5, M6, M7: wording, and limitations below. M3: corrected.
- [x] Sibling sweep, second pass: state ↔ ledger for tasks (counts), runs (links both ways, outcome, verification, steps per run) and the owner (`system.initialized`); state ↔ state for task ↔ run (current run, last run, status) and run → task. Idempotency records ↔ events are the ledger's own contract (T00.05).
- [x] Independent verifier round 2 (FAIL, no runtime defect; no false problem on any legitimate history through the production CLI: uninitialized, multi-run, never started, 1,004 tasks, homes written by kernels `0519357` and `aef4094`): **I-R2-1** six branches of `check_task_runs`/`check_owner` (a finished task with a current run; a last run of another task; runs but no last run; a current run that is not running; the owner's project missing; owners ≠ one per `system.initialized`) could be deleted with every test green, the same class as I1, because each earlier test tampered in a way another branch also saw. Minor: **m1** a 22-space run inside the owner message (a Python line continuation swallowed the Rust one); **m2** "normal daemon" left in a harness comment; **m3** the test-binary count explained wrongly; **m4** the crash-matrix step left the `crash-injection` build as `target/debug/rikurid`, and an unarmed `crash-injection` build reported "production configuration", contradicting the Module Contract and Cargo.toml ("reports itself Degraded"); **m5** S00.04/S00.05 deferral rows still RESEQUENCED; **m6** the T00.08 gate row credited it with all 8 mutants.
- [x] Responses: I-R2-1: one dedicated test per branch (8, including two branches the verifier found redundant), each proven by a mutant that only that test catches (O1–O8). m4 is a defect against the contract: red test `an_unarmed_crash_injection_build_is_not_reported_as_production` first, fix-lock engaged, then `ComponentHealth::build` takes `test_build` (`cfg!(feature = "crash-injection")`) and reports "crash-injection test build (no crash point armed)" as Degraded; and the gate step and the macOS CI step build in `target/crash-injection`, so the production binary is never replaced. m1, m2, m3, m5, m6 corrected. Lesson for the root-cause class: a check per branch needs a test whose tamper only that branch can see.
- [x] Independent verifier round 3 (FAIL, no runtime defect; legitimate histories, refusals, SIGKILL recovery by the production binary and the `test_build` change all correct): **I-R3-1** three comparisons still had no isolating test: the completion and verification links (`check_run_events`; the old test's tamper also changed the outcome, whose message names `run.completed` too) and the owner's actor ID (`check_owner`; the test changed only the project). Minor: **m-R3-1** a *created* task with a current run, and one owner with two initializations, untested inside tested arms; **m-R3-2** the documented manual command lacked `--target-dir`; **m-R3-3** "28 test binaries" left in the evidence log; **m-R3-4** the CI header listed 12 of 15 steps, and the macOS job name does not mention the matrix.
- [x] Responses, round 3. The tests had been chasing one finding at a time, so this round widened coverage beyond the named findings (still hand-enumerated; round 4 found more). Five isolating tests for I-R3-1 and m-R3-1: a second completion with the same outcome and a second verification with the same result (appended through the ledger API, as a regressed kernel would), an owner replaced by another actor, a created task with a current run, and a second initialization. Then the S00.07-era checks that had never been mutated: 15 more mutants found 13 survivors (per-task created and started counts, ledger events for a missing task, and every `check_run` condition). Since the event → run direction, other checks also see most of their states. Eleven tests, mostly built on a "phantom" run (a run object with no history of its own), give each condition a state only it can see, or assert on its own message. One test had to be corrected: it rewrote every run's link, not only the phantom's, so the R2a mutant survived until it targeted the phantom alone. Final pass: all 49 mutants (the verifier's 34 plus these 15) run against the final tests (evidence log). m-R3-2, m-R3-3: corrected. m-R3-4: the header lists all 15 steps; the job keeps its name `macOS core (build · unit · contract)` so its check history stays continuous, and the header says it also runs the matrix.
- [x] Independent verifier round 4 (FAIL, no runtime defect; `health.rs` and `kernel.rs` unchanged since round 3; legitimate histories clean through the production CLI): **I-R4-1** `check_run`'s completion and verification link checks never tested alone (every link test bent the start link); **I-R4-2** the "check could not run" path untested: with the replay error swallowed, an undecodable ledger row read as healthy, a Law 1 silent pass; **I-R4-3** the Module Contract and this record claimed the whole oracle. Minor: **m-R4-1** the other side of six tested conditions (a task never created, a dropped outcome or verification, a running run with only a completion link, a completed run without its verification link, a verification caused by the start); **m-R4-2** the build component's precedence (idempotency off must outrank a test build). Informational: three direction halves seen only through events missing an ID.
- [x] Responses, round 4: eleven tests, one per finding, including a ledger row the check cannot decode (inserted past the API, as corruption would be), which must report "could not run" and quarantine; the two precedence tests (a crash-injection daemon without idempotency is Critical; an in-process kernel without idempotency is Critical); and the three informational halves, using events a regressed kernel writes without their task or run ID. All 71 mutants caught. Claims corrected to say what is shown: a hand-enumerated set, which can still miss a condition.
- [x] Independent verifier round 5 (FAIL, no runtime defect; cargo-mutants rerun reproduced 85 caught / 0 missed / 4 unviable, the unviable genuinely uncompilable): **I-R5-1** `check_run`'s completion and verification links had no missing / another-run / another-task case of their own (only the start link did), while the Module Contract said each was tested; scenario run by the verifier: a second run reusing run 1's `run.completed` passes silently once that check is removed. **I-R5-2** the kernel's sibling of the "could not run" path (`Kernel::check_consistency` mapping a read error to a failed report) was untested: swallowed, a store whose task no longer decodes stays writable. Minor: the success path of `ComponentHealth::record` (reset, `last_success`, `work_completed`); the `crash_armed` branch and its order against `test_build`; the cargo-mutants claim did not state its default-feature scope, and the logged command lacked `--output` and the temp-directory cleanup; `health_components.rs` coupled its counts to one store read per `system.health`; no protected-path risk note (REVIEW.md); reads judged before writes when both fail (H05).
- [x] Responses, round 5: per-link tests for completion and verification (missing, another run's, about another task); `a_check_that_cannot_run_quarantines_writes`; `a_successful_check_after_a_failure_resets_the_evidence` (a stepping clock, so this success is distinguishable from an older one); `an_armed_crash_injection_build_says_it_is_armed`; the thresholds and the write-over-read precedence pinned directly through `ComponentHealth::from_store`, and the end-to-end store test loosened to "unhealthy, then Critical"; the risk note below; the cargo-mutants scope and command corrected. The mechanical bar is widened to the kernel functions on the W1 path (`consequential`, `check_consistency`, `quarantined`, `health`, `handle`).
- [x] Independent verifier round 6 (PASS). Minor: (1) the owner read's "could not run" path untested (W16); (2) crash-point positions not pinned (W03–W05: moving point 2 before the plan or after the event append, or point 1 into the transaction, keeps every outcome test green); (3) the duplicate-effect check pinned only for `task.created` (W14a–e; a consistent double-apply, counted twice by state and ledger alike, is seen only by it); (4) the Stage Gate tick "All stage modules wired" overclaimed while S00.08 testkit does not exist; (5) record wording (cargo-mutants scope) and a missing warning that a full /tmp makes viable mutants unviable. Informational: run events carrying no run ID and linked by no run are invisible to the run checks (two regressions at once).
- [x] Responses, round 6: (1) `an_owner_the_check_cannot_read_is_reported`; (3) `a_duplicate_of_any_event_type_under_one_key_is_reported` (every event type of a full lifecycle, with a step; a first version without a step left W14b alive); (4) the tick removed and scoped in the Stage Gate Record; (5) corrected in §1. (2) and the informational case are deferred below with their reactivation conditions.
- [x] File size: `apps/rikurid/tests/consistency_replay.rs` is about 1,330 lines, over AGENTS.md's 1,200, and is kept as one file on purpose: it holds one test per condition of the W1 oracle, sharing fixture helpers (tamper, phantom runs, stray events). Splitting it would need a shared `tests/common` module, which each test crate uses only in part; the resulting `dead_code` warnings fail clippy, and silencing them would add lint suppressions that the debt ratchet counts.
- [x] Protected-path risk note (REVIEW.md): this change edits `.github/workflows/ci.yml` (a macOS step running the crash matrix in its own target directory; the header comment), `scripts/quality/full-gate.sh` (a 15th step, `crash-matrix`, in `target/crash-injection`; a comment), `AGENTS.md` (the repository paragraph and Stage 00 verification text) and the Stage Gate Record (evidence rows, S00.04/S00.05/S00.07 ticks, S00.06 note). Risk: a gate step that could pass vacuously, or a CI change that weakens a check. Mitigation: the step runs the real-process matrix, whose cells and mutants are in this record; no existing step, check or `continue-on-error` setting is removed or loosened; the Stage Gate Record claims CI only where runs exist and says "Not yet proven" for this change.
- [x] Harness defect found on the first real-process run: T00.08 timed out waiting for the restarted daemon's "duplicate effect" warning, because the daemon logs it before "ready" and `Daemon::start` had consumed it. Fixed in the test (the harness keeps its startup lines); the daemon was right.

## 4. Verify

- [x] Red, then green, for B (evidence log).
- [x] T00.06, T00.07, T00.08, T00.12 pass locally (evidence log) and in CI: [run 37885121628](https://github.com/rikuripi/Rikuri-PI/actions/runs/37885121628) on PR #21 at `c0bd3be` (log: `Running tests/crash_matrix.rs (target/crash-injection/...)`, `t00_07_kill_9_at_every_point_of_every_command_then_one_effect ... ok`, 6 passed, `GATE crash-matrix: PASS`, `RESULT: PASS (15 steps)`).
- [x] Mutants, round 1: 4 against T00.06 and the replay tests, 8 against the real-process matrix; all caught (evidence log). R7's catch by T00.08 is racy (verifier: 4 runs in 6); T00.07 catches it every time.
- [x] Mutants, round 2 (the I1 branches and the new checks), after the owner released the fix-lock (2026-10-08): 10 of 10 caught (evidence log).
- [x] Full gate: `RESULT: PASS (15 steps)`, in round 1 and again with the oracle fix.
- [x] Mutants, round 3 (one per branch of the new checks, before the second lock): 8 of 8 caught, each by its own test (evidence log).
- [x] Independent verifier round 1: FAIL (see Debug).
- [x] Independent verifier round 2: FAIL (see Debug).
- [x] Mutants, round 4 (after verifier round 3): the verifier's 34 and 15 more on the S00.07-era checks, 49 of 49 caught (evidence log). Not the whole oracle: verifier round 4 found 9 more conditions.
- [x] Mutants, round 5 (after verifier round 4): all 71 hand-enumerated mutants (49 + the round-4 verifier's 22) caught by the final tests (evidence log).
- [x] Independent verifier round 3: FAIL (see Debug).
- [x] Independent verifier round 4: FAIL (see Debug).
- [x] Mutants, round 6, mechanical: `cargo mutants --file apps/rikurid/src/health.rs --test-workspace=true` (27.1.0): first run 80 caught, 5 missed, 4 unviable; the 5 were in the S00.07-era health evidence (the consistency component's failure count, reads failing while writes succeed, the Degraded/Critical threshold at 3), none in the W1 oracle. New `apps/rikurid/tests/health_components.rs` (2 tests); rerun: **85 caught, 0 missed, 4 unviable** (unviable: three `Default::default()` replacements for a type without `Default`, and `&&`→`||` inside an `if let` chain, which does not compile). With the 71 hand mutants (which split conditions more finely than the tool's operators), all caught. Scope: the tool runs default features, so `crash_matrix` is not in its pass, and it generates no mutant for the `test_build` branch; those feature-only branches are covered by hand mutants that `crash_matrix` catches.
- [x] Independent verifier round 5: FAIL (see Debug).
- [x] Mutants, round 7: cargo-mutants on `health.rs` and the kernel's W1-path functions: **96 tested, 89 caught, 0 missed, 7 unviable** (three `Default::default()` replacements for a type without `Default`; four `&&`→`||` inside `if let` chains, which do not compile). All 132 hand mutants (71 + the round-5 verifier's 61, including the feature-only branches that only `crash_matrix` catches) caught.
- [x] Independent verifier round 6: **PASS**, no Critical or Important finding; cargo-mutants rerun 96 / 89 caught / 0 missed / 7 unviable (all genuinely uncompilable); all round-5 findings resolved; production binaries correct on legitimate use, refusal and SIGKILL recovery. Five Minor findings, dispositions in Debug.
- [x] After round 6 (tests and records only): the six mutants the Minors named that tests could catch (W14a–e, W16) are caught by the two new tests; full gate rerun (evidence log). No further verifier round: the condition the owner set (no Important finding, gate green) was met at round 6.

## 5. Close Out

- [x] Full required gate green locally: `RESULT: PASS (15 steps)` after the last change. CI: Not yet proven until the PR runs.
- [x] Scrap Sweep: the untracked files are this change's five new files and the owner's `rikurinode.code-workspace` (left alone); no `mutants.out`, no proptest-regressions, no `/tmp/cm.*` or `/tmp/rikuri-crash.*` left; `target/crash-injection` lies inside the ignored `target/`. Mutants, probes and logs stay in the session scratchpad, outside the repository.
- [x] Documentation: `rikurid` Module Contract, README (gate steps, macOS job), AGENTS.md (gate changed), Stage Gate Record (evidence rows, S00.06 note). Help/RAG: N/A (no user-facing command changed).
- [x] Commits and PR: owner approval 2026-10-08, "Yes, commit, push, PR", conditional on a verifier round with no Important finding and a green gate (met at round 6). No merge without the owner's word.

## Owner choices exposed

- **The crash matrix also runs in the macOS core CI job** (`.github/workflows/ci.yml`). Why: SQLite recovery after SIGKILL is platform-neutral core behaviour, and AGENTS.md requires macOS core validation. Cost: about a second of test time plus the `crash-injection` build of `rikurid` on the owner's Mac after each merge. Reversible: delete the step. The macOS job is not a required check, so PRs never wait for it.
- **The gate gained a step** (`crash-matrix`, 15 steps). Cost: under a second of run time, plus a separate build in `target/crash-injection` (the first build there compiles the dependencies again, about a minute; later runs are incremental). The separate directory keeps the test build from replacing the production `target/debug/rikurid` (verifier round 2). Reversible: remove the step; the test then never runs, since `cargo test --workspace` skips a target with required features.

## Skipped / Deferred / Resequenced

| Item | Disposition | Reason | Dependency impact | Owner | New stage/ordinal | Reactivation / acceptance condition |
|---|---|---|---|---|---|---|
| Crash points 4, 5, 7, 8 (tool, budget, backup) | Not in Stage 00 | Those systems do not exist yet (Stage Plan T00.07) | Their stages add cells to `crash_matrix.rs`; all eight rerun at release | Stage implementer of each system | When each system lands (Stage Plan T00.07); full matrix (points 1–8) at Stage 05 and release (E1 register, release checklist) | Each point's cell in the matrix passes and bites |
| W1 row on the Stage Plan gate board | DONE 2026-10-09 (owner: "Update W1") | Updated to COMPLETE for the Stage 00 points with [run 37885121628](https://github.com/rikuripi/Rikuri-PI/actions/runs/37885121628) as its receipt | Stage 00 exit | Henry (protected document) | This PR | The board's Evidence cell links the run |
| Points 3 and 6 coincide in the kernel | Kept; recorded | Nothing runs between the commit and the response inside `Kernel::consequential` (S00.07 design), so the 8 point-6 cells repeat the point-3 ones | Point 6 differs once a response path sits between them (the socket write, a UI acknowledgement) | Stage 00 implementer | When such a path is added | A point-6 cell that fails where point 3 passes, or a recorded reason they stay together |
| T00.12 "deletion-governed references" | Partly proven | Stage 00 has no deletion path; the matrix checks that the task title never reaches the ledger, and each task carries its own deletion unit | Forget/deletion tests own the rest | Stage implementer of forget | When forget lands (Stage Plan) | A forget test proves the payload goes and only references stay |
| Crash-point positions (W03–W05) | DEFERRED | The outcome tests prove recovery at each hook as placed, but points 1 and 2 both expect "no change", so moving point 2 before `plan` or after the event append, or point 1 into the transaction, keeps them green. Pinning placement needs the hook to see the open transaction or a kernel step trace | W1's outcome claims stand for the hooks where they are; a refactor that moved a hook would go unnoticed until this lands | Stage 00 implementer | S00.08 (`CrashInjector` in `rikuri-testkit`) | A test fails when point 2 moves before the plan or after the event append, or point 1 moves inside the transaction |
| S00.08 `CrashInjector` | Stand-in recorded | The crash matrix uses `rikurid::crash` (`CrashHook`, `FailAtPoint`, `HangAtPoint`, and the tests' `ArmedOnce`). A testkit cannot own a hook that `rikurid` defines unless `rikurid` depends on the testkit, so the dependency direction is S00.08's decision | S00.08 and the Stage 00 testkit Module Contract | Stage 00 implementer | S00.08 | The S00.08 record decides (re-export, wrap, or move the trait) and the crash matrix uses the result |
| Run events without a run ID | Accepted limitation | `run.completed`/`run.verified` that carry no run ID and are linked by no run are invisible to the run checks; reaching it takes two regressions at once (the event without its ID, and no run linking it) | none for Stage 00 | Stage 00 implementer | Reconsider when another component writes run events (Stage 02+) | A check for run-typed events without a run ID, if another producer appears |
| Random proptest seeds | Kept (consistent with the earlier deferral in the storage/ledger defect record) | A failing seed prints its minimal case; proptest records it under `tests/` | none | Stage 00 implementer | Unchanged | A recorded counterexample is committed as a regression |

## Evidence log

```text
# (B) red run before the fix (apps/rikurid/tests/consistency_replay.rs); clippy clean
FAILED completion_events_that_state_does_not_show_are_reported (an unlinked completion went unnoticed: [])
FAILED an_outcome_that_differs_from_the_ledger_is_reported      (a changed outcome went unnoticed: [])
ok     a_consistent_lifecycle_has_no_problems                     (legitimate use)
# fix-lock engaged; fix: health::check_run_events (ledger run events → state links, outcome,
# verification, steps per run)
$ cargo test --workspace --locked --no-fail-fast → 28 result lines ok (test binaries plus doc-test runs), 0 failures; consistency_replay 3/3
  (T00.09 steps a run and asserts healthy, so task.stepped events carry their run ID: no false problems)

# (C) T00.06, in process
$ cargo test -p rikurid --test t00_06_no_duplicate_effects
ok crashes_and_retries_never_duplicate_an_effect (proptest, 64 cases)
ok every_point_of_every_command_recovers_with_one_effect (4 points × 4 commands)
ok without_idempotency_the_oracle_reports_the_duplicate
# mutants (scratch copy, src only), run against t00_06 + consistency_replay:
M1 replay skipped inside the write (re-applies)   CAUGHT (proptest + every-point test)
M2 point 2 fires after the commit                 CAUGHT (every-point test; the proptest alone missed it)
M3 oracle drops the duplicate-effect check        CAUGHT (twin)
M4 oracle drops the event → run checks            CAUGHT (consistency_replay ×2)

# (D) real process; first run: T00.08 FAILED (harness consumed the startup warning; see Debug), fixed in the test
$ cargo test -p rikurid --features crash-injection --test crash_matrix
ok t00_07_kill_9_at_every_point_of_every_command_then_one_effect (16 cells, 32 daemon starts)
ok t00_08_without_idempotency_the_same_crash_duplicates_and_w1_fails
ok t00_12_missing_references_are_refused_without_an_effect
test result: ok. 3 passed; finished in 0.33s
# mutants (scratch copy), run against crash_matrix:
R1 replay skipped (re-applies)                    CAUGHT T00.07
R2 point 2 fires after commit                     CAUGHT T00.07
R3 RIKURI_TEST_DISABLE_IDEMPOTENCY ignored        CAUGHT T00.08
R4 oracle drops the duplicate-effect check        CAUGHT T00.08
R5 system.health does not rerun the replay        CAUGHT T00.08
R6 a missing run accepted (current run used)      CAUGHT T00.12
R7 HangAtPoint does not stop the daemon           CAUGHT T00.07 (every run); T00.08 too, but racily (verifier: 4 runs in 6)
R8 completion links the wrong verification event  CAUGHT T00.07

# full gate
$ scripts/quality/full-gate.sh → RESULT: PASS (15 steps); GATE crash-matrix: PASS

# verifier round 1 → FAIL (I1, I2; Minor M1–M7). All new tests written before the lock:
# consistency_replay +8 (I1 ×3, task/run pointers ×3, owner ×1, a legitimate multi-run history ×1);
# crash_matrix: init cells (20 in all), four endings, missing task on step/complete;
# the T00.06 every-point test includes system.init. clippy clean with and without crash-injection.
$ cargo test -p rikurid --test consistency_replay        (red run, before the fix)
FAILED an_owner_that_disagrees_with_its_initialization_is_reported   no problem mentions ["system.initialized"]: []
FAILED a_task_pointing_at_a_missing_run_is_reported                  no problem mentions ["current run", <id>, "does not exist"]: []
FAILED a_running_run_its_task_does_not_point_at_is_reported          no problem mentions ["is running", "current run"]: []
FAILED a_task_status_that_contradicts_its_current_run_is_reported    no problem mentions ["is running but has no current run"]: []
ok     the three I1 tests, legitimate_multi_run_histories_have_no_problems and the first three   (7 passed; 4 failed)
$ cargo test -p rikurid --features crash-injection --test crash_matrix → 3 passed (20 cells; init recovers correctly)
$ python3 .claude/hooks/guard.py fix-lock on docs/build-records/2026-10-08-stage-00-crash-matrix.md
# fix: health::check_task_runs (task ↔ run pointers, status) and health::check_owner (owner, project ↔ system.initialized)
$ cargo test -p rikurid --test consistency_replay → 11 passed
$ cargo test --workspace --no-fail-fast → every binary ok, 0 failed (29 result lines: 24 test binaries plus 5 doc-test runs; T00.06 is the +1 over the 28 above, since cargo test --workspace never builds crash_matrix, which needs its feature; M7, corrected after round 2)
$ cargo test -p rikurid --features crash-injection --test crash_matrix → 3 passed
$ scripts/quality/full-gate.sh → RESULT: PASS (15 steps)
# owner released the fix-lock ("Yes, release it"); round-2 mutants (scratch copy), run against
# consistency_replay + t00_06 + crash_matrix:
V4 per-run step count dropped              CAUGHT a_run_step_count_that_differs_from_the_ledger_is_reported
V5 events for a missing run ignored        CAUGHT run_events_for_a_run_that_does_not_exist_are_reported
V8 run.started link not compared           CAUGHT an_unlinked_run_started_event_is_reported
N1 running task without a current run      CAUGHT a_task_status_that_contradicts_its_current_run_is_reported
N2 missing current run ignored             CAUGHT a_task_pointing_at_a_missing_run_is_reported
N3 missing last run ignored                CAUGHT a_task_pointing_at_a_missing_run_is_reported
N4 orphaned running run ignored            CAUGHT a_running_run_its_task_does_not_point_at_is_reported
N5 owner's project not compared            CAUGHT an_owner_that_disagrees_with_its_initialization_is_reported
N6 init event without an owner ignored     CAUGHT an_owner_that_disagrees_with_its_initialization_is_reported
N7 check_owner not called                  CAUGHT an_owner_that_disagrees_with_its_initialization_is_reported

# verifier round 2 → FAIL (I-R2-1; Minor m1–m6). New tests, all before the second lock:
# consistency_replay +8, one per branch; crash_matrix +1 (build health); m2 comment fixed.
$ cargo test -p rikurid --test consistency_replay → 19 passed (the branches were right; bite shown below)
$ cargo test -p rikurid --features crash-injection --test crash_matrix   (red run, m4)
FAILED an_unarmed_crash_injection_build_is_not_reported_as_production
  {"component":"build","detail":"production configuration","state":"healthy",...}
ok     t00_07 (20 cells), t00_08, t00_12
# round-3 mutants (scratch copy, lock still released), each disabling one branch:
O1 finished task with a current run allowed    CAUGHT a_finished_task_with_a_current_run_is_reported
O2 last run of another task allowed            CAUGHT a_last_run_of_another_task_is_reported
O3 runs without a last run allowed             CAUGHT a_task_with_runs_but_no_last_run_is_reported
O4 current run of another task allowed         CAUGHT a_current_run_of_another_task_is_reported
O5 current run that is not running allowed     CAUGHT a_current_run_that_is_not_running_is_reported
O6 missing owner project allowed               CAUGHT a_missing_owner_project_is_reported
O7 owners ≠ one per init event allowed         CAUGHT owners_without_exactly_one_initialization_are_reported
O8 run of a missing task allowed               CAUGHT a_run_of_a_missing_task_is_reported
$ python3 .claude/hooks/guard.py fix-lock on docs/build-records/2026-10-08-stage-00-crash-matrix.md
# fix: ComponentHealth::build(test_build = cfg!(feature = "crash-injection"), ...); m1 message;
# gate step and macOS CI step build in target/crash-injection
$ cargo test -p rikurid --features crash-injection --test crash_matrix --target-dir target/crash-injection → 4 passed
$ scripts/quality/full-gate.sh → RESULT: PASS (15 steps)
# after the gate, target/debug/rikurid is the production build:
$ rikuri system health → [('build', 'healthy', 'production configuration')]

# verifier round 3 → FAIL (I-R3-1; Minor m-R3-1..4). Test-only responses, no fix-lock needed.
$ cargo test -p rikurid --test consistency_replay → 24 passed (+5: second completion, second verification,
  owner replaced, created task with a current run, second initialization)
# 15 mutants of the S00.07-era checks (scratch copy) against those tests: 13 SURVIVED
#   (C1 created count, C3 started count, C4 ledger task missing, R1–R6 check_run conditions; C2 and C5 caught)
$ cargo test -p rikurid --test consistency_replay → 35 passed (+11 isolating tests, phantom runs)
# R2a still survived: its test rewrote every run's start link, so the original run produced the
# expected message through the run mismatch; corrected to target the phantom alone → R2a CAUGHT
# final pass, all 49 mutants (verifier's 34 + these 15), each run against rikurid and rikuri-cli tests
# and crash_matrix (crash-injection):
caught 49 / survived 0
$ scripts/quality/full-gate.sh → RESULT: PASS (15 steps)

# verifier round 4 → FAIL (I-R4-1..3; Minor m-R4-1, m-R4-2). Test-only responses.
$ cargo test -p rikurid --test consistency_replay --test t00_06_no_duplicate_effects → 43 + 4 passed
$ cargo test -p rikurid --features crash-injection --test crash_matrix --target-dir target/crash-injection → 5 passed
# 71 mutants (49 + the round-4 verifier's 21 + its V29), each against rikurid and rikuri-cli tests and crash_matrix:
caught 68 / survived 3   (V03, V05, V13: the direction halves the verifier called redundant)
# + a_step_or_start_missing_one_id_is_reported (events without their task or run ID); first run failed on a
#   test bug (the second case reused the first home's task ID), fixed → 44 passed
V03, V05, V13 → CAUGHT a_step_or_start_missing_one_id_is_reported      (71 of 71)

# owner: "Install cargo-mutants"; cargo install cargo-mutants --version 27.1.0 --locked
# (TMPDIR must be short: the copies' test sockets otherwise exceed the 103-byte limit, which the
#  daemon rightly refuses; the first attempt's baseline failed on exactly that)
$ T=$(mktemp -d /tmp/cm.XXXX); TMPDIR=$T cargo mutants --file apps/rikurid/src/health.rs --test-workspace=true -j 3 \
    --output <scratch dir>; rm -rf -- "$T"      (--output keeps mutants.out out of the repository)
89 mutants tested in 2m: 5 missed, 80 caught, 4 unviable
MISSED health.rs:68:39  replace += with *= in ComponentHealth::record   (failure count never asserted)
MISSED health.rs:80:68  replace > with >= in ComponentHealth::from_store (reads failing alone never tested)
MISSED health.rs:87:22  replace < with == / > / <= in ComponentHealth::from_store (threshold at 3 untested)
# + apps/rikurid/tests/health_components.rs: consecutive failures 0 → 1 → 2 (Critical); failing reads
#   give store 1 Degraded → 2 Degraded → 3 Critical, "read transaction(s) failed"
$ cargo mutants … (same command)
89 mutants tested in 2m: 85 caught, 4 unviable          (0 missed, 0 timeouts)

# verifier round 5 → FAIL (I-R5-1, I-R5-2; minors). Test-only responses:
# consistency_replay +2 (per-link completion and verification: missing, another run's, another task's) → 46 passed
# health_components: thresholds pinned via from_store, success-path reset with a stepping clock, could-not-run
#   quarantine, end-to-end store test loosened → 5 passed; crash_matrix +1 (armed build says so) → 6 passed
$ T=$(mktemp -d /tmp/cm.XXXX); TMPDIR=$T cargo mutants --file apps/rikurid/src/health.rs \
    --file apps/rikurid/src/kernel.rs \
    --re '^apps/rikurid/src/health\.rs|in Kernel::(consequential|check_consistency|quarantined|health|handle)\b' \
    --test-workspace=true -j 3 --output <scratch dir>; rm -rf -- "$T"
96 mutants tested in 2m: 89 caught, 7 unviable          (0 missed, 0 timeouts)
# hand mutants: 71 earlier + the round-5 verifier's 61, each against rikurid and rikuri-cli tests and crash_matrix
hand: caught 132 / survived 0

# verifier round 6 → PASS (no Critical/Important); five Minors. Test-only responses:
# + a_duplicate_of_any_event_type_under_one_key_is_reported, an_owner_the_check_cannot_read_is_reported → 48 passed
#   (first version of the duplicate test had no step, so W14b survived; added a step and task.stepped to the required types)
vr6_W14a..e (duplicate check narrowed per type), vr6_W16 (owner read error swallowed) → 6 of 6 CAUGHT
```
