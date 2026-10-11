# Build Record — Tests must leave nothing in the temporary directory

| Field | Value |
|---|---|
| Change | `cargo test --workspace` stops leaving `/tmp/.tmp*/state/` trees behind: `rikuri-storage`'s reentrancy tests wait for their worker threads to clean up, and the gate (and the macOS job) run the tests with a private, empty `TMPDIR` that must still be empty afterwards |
| Date | 2026-10-10 |
| Author / agent session | Claude Code (Opus 5.5) for Henry Chen, a side session started from the E1 session |
| Branch | `fix/test-temp-residue`, from `main` at `48bb7ee` (after PR #24) |
| Commits | `ca737b4` (tripwires, red), `5f50728` (fix), close-out (this record) |
| Stage Gate | [Stage 00](../stage-gates/foundation0/00-bootstrap-kernel.md), gate hygiene: S00.03 (gate scripts) and S00.02 (CI, the macOS job's test step); no Stage Gate item changes |
| Related ADR / requirement | AGENTS.md laws 1 and 5 (nothing fails silently; defects become tripwires), Scrap Sweep |
| Status | COMPLETE locally (verifier round 2 PASS; gate steps all PASS, in two runs); CI Not yet proven |

**Risk note (protected paths):** `scripts/quality/*` is the gate and `.github/workflows/ci.yml` is CI. The change adds a check to the gate's `test` step and to the macOS job's test step; it removes nothing.

## 1. Plan

- [x] One-sentence statement: a test run leaves nothing in the temporary directory, and the gate fails when one does.
- [x] Defect (found by a verifier on 2026-10-10): every `cargo test --workspace` run left new `/tmp/.tmpXXXXXX/state/rikuri.db.lock` trees; 104 such directories existed that day, dated 2026-10-08 to 2026-10-10 (160 when this session began; 161 in the 16:59 snapshot listed for the owner).
- [x] Current behaviour characterized (evidence log below):
  - The leaked trees have tempfile's default name and mode (`.tmpXXXXXX`, 0775 under umask 002) around the store's layout (`state/` 0700, files 0600), so they are `tempfile::tempdir()` directories with a `Store` in them, not directories a daemon recreated (the store creates missing directories 0700).
  - They come in every stage of a half-finished removal: complete (`rikuri.db` + lock; `a.db`/`b.db` with WAL), the lock alone, the database alone, an empty `state/`, an empty top directory. That is `remove_dir_all` cut off part way, which happens when the process exits while a thread is still dropping its `TempDir`.
  - Each test target was run alone, three rounds, with a private `/tmp`: only `rikuri-storage`'s `reentrancy` target left anything. Its tests open the store and the `TempDir` on a worker thread (so a deadlock fails the test instead of hanging it), and the worker sends its answer **before** it drops them. The test returns at once; when the last test returns, the harness exits the process and the worker's cleanup never finishes. 40 runs of that target alone left 39 directories, in all the shapes above.
- [x] Root-cause class: a test's scratch directory owned by a thread the test does not wait for, so its cleanup races the end of the process.
- [x] Sibling sweep (other threads and processes in tests):
  - `s00_07_defects`, `s00_07_round3`, `t00_09` and `cli_exit_codes` start serving threads (`start_daemon`) and request threads; their `TempDir`s belong to the test thread, which removes them before the test reports, and a thread still holding the store only keeps unlinked files open. Not leaking (0 in 3 rounds).
  - The crash matrix kills and waits for its child daemon in `Drop`, and its `TempDir`s (fixed prefix `rikuri-crash.` under `/tmp`) belong to the test thread. `smoke.sh` removes its `mktemp -d /tmp/rikuri-smoke.*` in an `EXIT` trap. Normal runs leave nothing (the private `/tmp` was empty after the full gate; verifier: 3 more rounds of smoke and the crash matrix). **Corrected after verifier round 1:** this line first said no `rikuri-crash.*` directories exist in `/tmp`; 8 do. The check that "showed" none was a zsh command that aborted on an unmatched glob (`/tmp/cm.*`) and printed nothing, misread as empty. The 8 trees (crash-matrix scratch homes with a socket and a store with WAL) date from 2026-10-09 00:54–00:55, in pairs about 21 s apart (20.8–21.1 s): consistent with a crash-matrix test process killed from outside before its `Drop` ran (mutation runs with timeouts were under way on the crash-matrix work that night; a guess, not reproduced). No in-process cleanup survives SIGKILL; they are added to the owner's list. No `rikuri-smoke.*` directories exist.
- [x] **I will know this works when:**
  - a regression test in `reentrancy.rs`, committed red before the fix, shows the case's scratch directory still exists when `on_a_thread` answers, and passes after it;
  - the gate's `test` step, which now runs `cargo test --workspace` through `scripts/quality/no-temp-residue.sh` (a fresh private `TMPDIR` that must be empty afterwards), fails on the unfixed tests whenever they leak, naming the leaked files, and passes after the fix (the leak is a race, so the residue check alone catches the old tests in some runs, not all; the regression test is the reliable tripwire at the source, the residue check the one for the class);
  - a gate self-test shows the wrapper fails on residue (including a hidden file and an unreadable directory) and when the command removes its `TMPDIR`, keeps a failing command's exit status, passes a command that cleans up, and removes its private directory every time;
  - the full gate passes; the independent verifier finds no unresolved Important finding.
- [x] Architecture impact: test code and the gate only. No runtime, canonical data, wire, privacy, money or recovery path changes.
- [x] Existing solution review: tempfile's `TempDir` already removes its directory on drop; the defect is that the drop never ran. No new dependency.
- [x] Authoritative state change: none.
- [x] Fix-lock: **not engaged.** The fix is in a test file (`crates/rikuri-storage/tests/reentrancy.rs` matches the lock's `*/tests/*`), so the lock would freeze the fix itself. In its place: the red commit holds the tripwires, and the fix commit changes only the helper code in `reentrancy.rs` (`git diff <red> <fix> -- scripts .github` is empty, and the regression test's body is unchanged).
- [x] Existing leaked directories are **not** deleted here: they are listed for the owner, who decides (close-out).
- [x] Owner decisions 2026-10-10: commit, push and open the PR once the verifier passes ("Yes, commit+push+PR (Recommended)"); keep the macOS CI change ("Keep it (Recommended)").

## 2. Build

- [x] `scripts/quality/no-temp-residue.sh <command…>`: runs the command with `TMPDIR` at a fresh `mktemp -d /tmp/rikuri-tests.XXXXXX` (under `/tmp`, not `$TMPDIR`, to keep test socket paths short on macOS), lists anything left in it (hidden files and unreadable directories included, first 200 entries), fails if anything is left, if the command removed the directory or replaced it with a symlink, or if the command failed (keeping the command's own status), and removes the directory (its cleanup never follows a link put in its place).
- [x] `full-gate.sh`: `step test scripts/quality/no-temp-residue.sh cargo test --workspace --locked`; the step count stays 15.
- [x] `ci.yml` macOS job: its test step runs through the same wrapper. That runner keeps its files between jobs, so leaks there would pile up unseen.
- [x] Gate self-test `scripts/quality/tests/temp-residue.sh` (10 cases; a command that removes its `TMPDIR` added after verifier round 1; the same exiting 5, and a `TMPDIR` replaced by a symlink, after round 2).
- [x] `reentrancy.rs`: regression test `a_case_leaves_no_scratch_directory_behind` (red); then `within_deadline` runs the work on a thread, sends the answer only after the work returned (its stores and scratch directories already dropped) and joins the thread; `on_a_thread` and a new `on_two_stores` use it, replacing the three hand-written thread-and-channel blocks. The nested-store tests keep their cases and assertions.
- [x] Docs: README (gate step list), this record. AGENTS.md, Module Contracts, the Stage Gate Record: N/A (no crate responsibility, interface or gate item changes).

## 3. Debug

- [x] Problem reproduced before the fix: yes (39 of 40 runs, evidence log).
- [x] State-changing remediation: none outside the repository. The leaked directories stay until the owner decides. One residue this session made itself was removed: a mutant of the wrapper (scratch copy, below) left `/tmp/tmp.J7PGMrBHjD/outer/locked` (mode 000) that the self-test's own cleanup could not remove; the self-test now makes its scratch writable before removing it, and later mutant runs kept their scratch in `target/`.

## 4. Verify

- [x] Regression test red on the old helper, green after; result: **red 19 of 20 runs** on the old helper (`/tmp/.tmpXXXXXX outlived the case`; a race, so the thread sometimes wins), **green 50 of 50** after the fix (the join makes it deterministic).
- [x] Gate `test` step red on the old tests (names the residue), green after; result: with the red commit's tree, the step failed through the regression test (exit 101). With the old test set alone (`-- --skip a_case_leaves_no_scratch_directory_behind`), the residue check failed 2 of 5 runs, naming `.tmpXXXXXX/state/rikuri.db` and `.lock`. After the fix: 5 of 5 runs pass, `RESIDUE none`; the `reentrancy` target alone, 40 runs: 0 directories left.
- [x] Gate self-test; result: 7 of 7 ok, as the user and as namespace root; after verifier round 1, the eighth case was red against the wrapper (`a command that removed its TMPDIR gave status 0`, output `RESIDUE none`), then 8 of 8 ok after the fix, as the user and as root. Against scratch-copy mutants of the wrapper (not committed), each caught: residue ignored (4 FAIL lines), `ls` instead of `find` (misses hidden entries, 5), private directory not removed (7), `TMPDIR` not set (12), files only (2), residue masking the command's status (1), the command's failure dropped (2).
- [x] Full gate (inside the outer safety layer: user, mount and network namespace, `/run` masked, private `/tmp`, refusing stand-ins for docker, sudo, systemctl, lncli and the like first on `PATH`, `DOCKER_HOST` at a missing socket; `lnd` start time recorded before and after); result: `scripts/quality/full-gate.sh` passed its first nine steps (tools, gate self-tests including the 7 new cases, guard, format, build, clippy, **test with `RESIDUE none`**, smoke, crash matrix) and stopped at `dependencies`: `cargo deny` could not fetch the advisory database without a network. The remaining six steps were then run one by one with the gate's exact commands in the same layer sharing the host network (DNS resolver directory re-exposed, `/run` otherwise masked): dependencies, secrets, ratchet, contracts, crate policy, scrap all PASS (the scrap sweep lists this change's three new files as untracked, before commit). `lnd` started at `2026-10-09T20:20:27.903167252Z` before and after both runs. One uninterrupted 15-step run is CI's to show.
- [x] Independent verifier round 1 (FAIL, on the record, not the code; every cargo and gate run in the safety layer, `lnd` start time unchanged): the fix, the regression test (red 20 of 20 on the old helper, green 0 of 250 failures after, including 8 parallel loops), the wrapper (7 of 7 as root and as uid 1000; 5 own mutants caught; rc 2 / 127 / 137 kept; 250 entries listed as 200) and gate steps 1–9 plus the six remaining steps all pass; the residue check caught main's `reentrancy.rs` in 3 of 5 full runs and 14 of 20 target runs; deadlock detection unchanged (with re-entry refusal switched off in a scratch copy, the same three tests fail at 10.0 s with the same messages under both helpers); all 29 test binaries 10 times through the wrapper, 0 residue. **Important:** the sibling-sweep claim "no `rikuri-crash.*` directories exist in `/tmp`" was false (8 exist), so the owner's list missed them. Nits: (1) a command that deletes its own `TMPDIR` passes the wrapper; (2) the acceptance line read as if the residue check always catches the race; (3) the Stage Gate field named only S00.03 though `ci.yml` is S00.02; (4) pre-existing on `main`: a case that fails to set up or panics is reported as "no answer … (deadlock)". Not yet proven: the commits, one uninterrupted 15-step run, CI.
- [x] Responses, round 1: the claim corrected (sibling sweep above) and the 8 trees added to the owner's list; (1) fixed tests-first, the eighth self-test case red, then green; (2) acceptance line reworded; (3) S00.02 added; (4) left as it is, recorded under limitations (it predates this change and every such case still fails the test).
- [x] Gate rerun after the round-1 changes, same layers; result: steps 1–9 PASS (self-tests 8 of 8; `test` with `RESIDUE none`), `dependencies` again unable to fetch without a network, then dependencies (`advisories ok, bans ok, licenses ok, sources ok`), secrets, ratchet, contracts, crate policy and scrap PASS with the host network shared. `lnd` start time unchanged (`2026-10-09T20:20:27.903167252Z`).
- [x] Independent verifier round 2 (**PASS**, on the uncommitted tree; same layers, `lnd` start time unchanged): the round-1 Important finding and nits 1–3 resolved, nit 4 accurately recorded; the eighth self-test case red against a reconstructed round-1 wrapper, 8 of 8 on the current one as root and as uid 1000 without capabilities; gate steps 1–9 in one run (183 tests passed, `RESIDUE none`) and the other six one by one, all PASS; `reentrancy` 20 runs through the wrapper, 0 residue. New nits: (1) the exit status on the removed-`TMPDIR` path was untested (a mutant forcing `rc=1` there survived); (2) neither header comment mentioned that case; (3) if a command replaced its `TMPDIR` with a symlink, the cleanup's `chmod -R` followed it and changed the target's modes; (4) "exactly 21 s apart" holds only to the second (20.84–21.06 s); (5) the owner's list is a snapshot, and four newer `.tmp*` trees exist (17:08, 17:11, 17:20, 18:37), made by another checkout without the fix. Outside this change: 207 `rikuri-test-data-*` directories and one `rikuri-test-refuse-bin-*` in `/tmp`, named by no tracked file here (probably another repository).
- [x] Responses, round 2: (1)–(3) tests first: two cases (removed and exiting 5; replaced by a symlink whose target is mode 500) red against the round-2 wrapper (status 1 where 5 was due, and the old message; the symlink case passed with the target turned 700), then the wrapper refuses a replaced `TMPDIR` too and its cleanup skips `chmod` on a link: 10 of 10 as the user and as root; the surviving mutant (forced `rc=1`) and a cleanup that follows the link are both caught now; both header comments updated. (4) Reworded. (5) The list was refreshed at close-out; cleanup should go by pattern, not by the list (it grows while unfixed checkouts run `cargo test`). The other repository's directories are reported to the owner only.
- [x] Gate rerun after the round-2 changes, same layers; result: steps 1–9 PASS (self-tests 10 of 10; `test` with `RESIDUE none`), `dependencies` unable to fetch without a network, then the six remaining steps PASS with the host network shared. `lnd` start time unchanged. The changes after round 2 are confined to the wrapper, its self-test and this record, each shown above; no third verifier round was run.
- [ ] CI: Not yet proven (this PR's `Full gate (Ubuntu)` run; the macOS step after merge to `main`).

## Close-out (2026-10-10)

- [x] Commits: `ca737b4` holds the tripwires and `reentrancy.rs` with the regression test but the old helper; at that commit the regression test failed 10 of 10 runs. `5f50728` changes only `reentrancy.rs` (`git diff ca737b4 5f50728 -- scripts .github` is empty; the regression test's body is identical in both).
- [x] Scrap: nothing task-created left in the repository. The outer safety layer, logs and saved file states live in the ignored `target/safety/`; the scrap sweep lists no untracked files after the commits.
- [x] Leaked directories: not deleted. The owner's list (166 `/tmp/.tmp*` trees and 8 `/tmp/rikuri-crash.*` trees, about 4.8 MB, snapshot 2026-10-10 19:21) is in the session notes, outside the repository; cleanup should go by pattern, since checkouts without this fix keep adding `.tmp*` trees.
- [x] Docs: README gate step list; AGENTS.md, Module Contracts and the Stage Gate Record N/A (no crate responsibility, interface or gate item changes).

## Limitations

- The residue check sees what honours `TMPDIR`. Scratch made at a fixed path is outside it: the crash matrix's `/tmp/rikuri-crash.*` homes and smoke's `/tmp/rikuri-smoke.*`. Both clean up on every normal run; the 8 crash-matrix trees came from a process killed from outside, which no in-process cleanup survives.
- The residue check catches a racy leak only in the runs where it happens; the regression test covers this defect's source on every run.
- Pre-existing (from `main`): in `reentrancy.rs`, a case that fails to set up or panics fails with the message "no answer within 10s (deadlock)". The test still fails; only the message misleads.

## Evidence log

```text
# Leaked directories in /tmp, 2026-10-10 (before this change), grouped by what state/ holds
 83 rikuri.db rikuri.db.lock          16 rikuri.db.lock          8 rikuri.db
 25 (empty or no state/)               4 rikuri.db rikuri.db-wal rikuri.db.lock
  3 rikuri.db rikuri.db-shm rikuri.db-wal      1 rikuri.db rikuri.db-shm rikuri.db-wal rikuri.db.lock
  7 a.db a.db-shm a.db-wal a.db.lock  2 a.db a.db-shm a.db-wal   1 a.db a.db.lock
  5 b.db b.db-shm b.db-wal b.db.lock  2 b.db b.db-shm b.db-wal   3 b.db b.db.lock
top directories drwxrwxr-x (tempfile default under umask 002); state/ drwx------; files -rw-------

# Each test target alone, 3 rounds, private /tmp (safety layer)
only leak: rep=3 rikuri-storage test reentrancy → .tmpMAYXcT/state/{rikuri.db,rikuri.db.lock}
(crash_matrix is skipped without its crash-injection feature, as in `cargo test --workspace`)

# reentrancy target alone, 40 runs, private /tmp: 39 directories left
 31 state/rikuri.db state/rikuri.db.lock      5 state/rikuri.db.lock
  1 state/rikuri.db-wal state/rikuri.db state/rikuri.db.lock
  1 state/ (empty)      1 (empty top directory)
```
