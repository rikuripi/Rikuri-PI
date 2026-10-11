# E1 evidence: rerunnable harnesses

Evidence for [the E1 Build Record](../../2026-10-09-s00-11-e1-tse-spike.md) and [ADR 0005](../../../architecture/adr/0005-e1-transactional-state-evolution-for-kernel-transactions.md). Neither patch is product code: each adds environment switches and a test harness to a scratch checkout, so a reviewer can rerun the design comparison and the planted defects instead of trusting the recorded numbers.

| File | Applies to | What it adds |
|---|---|---|
| `spike-on-48bb7ee.patch` | `48bb7ee` (`main` before E1) | `RIKURI_E1_SYNC` (the store's `synchronous` level); `RIKURI_E1_DEFECT` (a completion that leaves the task pointing at its run); `RIKURI_E1_PRECOMMIT` (`1`: the full replay before the commit; `scoped`: the first, task-scoped check, later replaced); `tests/e1_spike.rs` with experiments 1–3 and design B, the candidate/promotion prototype for `task.create` |
| `write-set-defects-and-cost.patch` | the commit that adds this file (the landed write-set check) | `RIKURI_E1_D1` to `D8`, `V1` to `V11` except `V10`, `N1`, `N5`, `N6`, `N8`, `N9`, `X2`, `X3`, `X4`, `R1`, `R3`, and `S1` in the ledger (planted defects: D4–D8 are verifier round 2's, V1–V11 round 3's, N round 4's, X round 5's, R and S round 6's); `RIKURI_E1_NOCHECK` (skip the candidate check, the answer check and the idempotency-record checks, for a baseline); `tests/e1_defects.rs` with the defects run and the long-task cost |
| `drive-daemon.py`, `drive-scenarios.sh` | binaries built from a copy with the patch above | the N, X, R and S defects through the real `rikurid` and `rikuri` CLI, each on a fresh scratch home, with and without the check (written by verifier round 4, adopted and extended here) |

Apply a patch with `patch -p1 < <patch>` from the scratch copy's root. `git apply` also works in a copy that is its own git repository, but inside another work tree (for example a copy under this repository's `target/`) it applies nothing and still exits 0 (verifier round 3).

Timed runs need `E1_DISK`, a directory on a real disk: on a tmpfs `fsync` is free and the numbers mean nothing. The recorded runs used the reference installation (Ubuntu 26.04.1 LTS, kernel 7.0.0-38, ext4 on LVM) with `E1_DISK` under the repository's ignored `target/`.

## Running the spike (design comparison)

From a scratch checkout of `48bb7ee` (not your working tree):

```bash
patch -p1 < /path/to/Rikuri-PI/docs/build-records/evidence/2026-10-09-s00-11-e1/spike-on-48bb7ee.patch
```

Experiment 1, a planted defect with no check, the full replay and the scoped check before the commit:

```bash
RIKURI_E1_DEFECT=1 cargo test --release -p rikurid --test e1_spike exp1_planted_defect -- --nocapture
```

```bash
RIKURI_E1_DEFECT=1 RIKURI_E1_PRECOMMIT=1 cargo test --release -p rikurid --test e1_spike exp1_planted_defect -- --nocapture
```

```bash
RIKURI_E1_DEFECT=1 RIKURI_E1_PRECOMMIT=scoped cargo test --release -p rikurid --test e1_spike exp1_planted_defect -- --nocapture
```

Expected: without a check the completion is promoted, the replay finds two problems and writes are quarantined; with either check it is refused ("candidate refused before promotion"), the task stays running, and the next write succeeds.

Experiment 2, cost per create as the ledger grows (`E1_N` tasks first; leave `RIKURI_E1_PRECOMMIT` unset, or set it to `1` or `scoped`):

```bash
E1_DISK=$PWD/target/e1-bench E1_N=3000 RIKURI_E1_PRECOMMIT=1 cargo test --release -p rikurid --test e1_spike exp2_precommit_cost -- --nocapture
```

Experiment 3, design B at its crash points, then latency for designs A and B (`RIKURI_E1_SYNC=FULL` or `NORMAL`):

```bash
cargo test --release -p rikurid --test e1_spike exp3_b_crash_points -- --nocapture
```

```bash
E1_DISK=$PWD/target/e1-bench RIKURI_E1_SYNC=FULL E1_M=300 cargo test --release -p rikurid --test e1_spike exp3_a_latency -- --nocapture
```

```bash
E1_DISK=$PWD/target/e1-bench RIKURI_E1_SYNC=FULL E1_M=300 cargo test --release -p rikurid --test e1_spike exp3_b_latency -- --nocapture
```

fsyncs per create: run the built test binary (`target/release/deps/e1_spike-*`) under `strace -f -c -e trace=fsync,fdatasync` with `E1_M=100` and then `E1_M=200`, and take the difference.

## Running the write-set check's defects and cost

On a scratch copy of the tree at the commit that adds this file:

```bash
patch -p1 < /path/to/Rikuri-PI/docs/build-records/evidence/2026-10-09-s00-11-e1/write-set-defects-and-cost.patch
```

```bash
RIKURI_E1_D2=1 cargo test --release -p rikurid --test e1_defects defects -- --nocapture
```

The run creates task `a` and a private task `b`, starts `a`, tries to start it again (a correct kernel refuses this as `invalid_state`), steps it and completes it. Run it with no switch (control), then with each of:

| Switch | Planted defect |
|---|---|
| `RIKURI_E1_D1` | `task.complete` leaves the task's current run set |
| `RIKURI_E1_D2` | `task.start` also inserts a run no event names |
| `RIKURI_E1_D3` | `task.step` also bumps another task's step count |
| `RIKURI_E1_D4` | `task.start` skips its "already running" refusal |
| `RIKURI_E1_D5` | `task.step` gives the running run a verification link |
| `RIKURI_E1_D6` | `task.step` is applied twice, with two `task.stepped` events under one key |
| `RIKURI_E1_D7` | `task.step` appends an event about another task directly, outside its plan |
| `RIKURI_E1_D8` | `task.step` rewrites task `b`'s privacy class and title, with no event |
| `RIKURI_E1_V1` | `task.step` rewrites its own task's title and privacy class |
| `RIKURI_E1_V2` | `task.step` replaces its own task's deletion unit |
| `RIKURI_E1_V3` | `task.step` moves its own task to a new project |
| `RIKURI_E1_V4` | `task.start` gives the new run an attempt one too high |
| `RIKURI_E1_V5` | `task.start` leaves the task's last run stale |
| `RIKURI_E1_V6` | `task.complete` leaves the task `created` |
| `RIKURI_E1_V7` | `task.step`'s event is marked failed and records a wrong step count |
| `RIKURI_E1_V8` | `task.step` changes its run's attempt |
| `RIKURI_E1_V9` | `task.step` also creates a hidden task, with its own `task.created` under the step's key |
| `RIKURI_E1_V11` | `task.step` is applied twice, the twin event under another key |

V10 (raw SQL around the repository) and V12 (fidelity to the request) have no switch: they are outside the check's design (E1 record).

Expected: the control passes every command (the second start refused as `invalid_state`); each defect's command is refused before promotion, and afterwards the replay finds no problem, nothing is quarantined, the next create succeeds and task `b` is unchanged. Add `RIKURI_E1_NOCHECK=1` to see what happens without the check: D4 and V5 quarantine the daemon; D8 rewrites task `b`, and V1, V2, V3 and V6 change task `a`, with the replay noticing none of them.

This machine class may run other live services: run the harness inside a sandbox with no network and no access to `/run` (the E1 runs used a user, mount, network and PID namespace with `/run` masked).

The N, X, R and S defects need requests or answers the harness never sends or inspects (a step naming another task's run, a retry under one key, the answer itself), so they are driven through the real daemon and CLI. In the patched copy, build the binaries, then run the scenarios with `<bin-dir>` set to that build's `release` directory:

```bash
cargo build --release -p rikurid -p rikuri-cli --bins
```

```bash
docs/build-records/evidence/2026-10-09-s00-11-e1/drive-scenarios.sh <bin-dir> <scratch-dir>
```

| Switch | Planted defect (verifier round 4) |
|---|---|
| `RIKURI_E1_N1` | the kernel's check of the run a request names is skipped, so a step (N1) or completion (N2) of task a can name task b's running run |
| `RIKURI_E1_N5` | a new task reuses an existing task's deletion unit |
| `RIKURI_E1_N6` | the idempotency record is not written |
| `RIKURI_E1_N9` | a step's idempotency record is written under another command |
| `RIKURI_E1_N8` | `system.init` creates the Inbox trusted for execution |
| `RIKURI_E1_X4` | a create writes no idempotency record and also clears the kernel's note that it wrote one (verifier round 5) |
| `RIKURI_E1_X2` | a create's record stores another task's ID as its response |
| `RIKURI_E1_X3` | a create also writes a record under `<key>-x` |
| `RIKURI_E1_R1` | a create answers, and so records, another task's ID (verifier round 6) |
| `RIKURI_E1_R3` | `system.init` answers a project ID that is not the Inbox's |
| `RIKURI_E1_S1` | the ledger stamps appended events with schema version 7 |

Expected: each refused before promotion, nothing quarantined; the legitimate scenarios (`foreign-run`, `stored-response`, `extra-record`, `create-answer`, `init-answer`, `event-schema`, `init-trust` with no switch) pass. Without the check, R1's create answers task a for task "b secret", R3's init answers another project and S1 leaves events at version 7, all unseen; Without the check: N1 steps task a on b's run while health stays healthy; N2 quarantines the daemon; N5 leaves two tasks sharing a deletion unit, unseen; N6 and X4 duplicate the task on retry and quarantine; N9 makes the retry fail with `idempotency_conflict`, unseen; N8 leaves a trusted Inbox, unseen; X2's retry answers with another task, unseen; X3 refuses a later command under `<key>-x`, unseen.

Long-task cost, ms per `task.step` after `E1_N` steps on one task (add `RIKURI_E1_NOCHECK=1` for the baseline, or `E1_NO_INDEX=1` to drop the ledger's `events_by_task_type` index first):

```bash
E1_DISK=$PWD/target/e1-bench E1_N=6000 cargo test --release -p rikurid --test e1_defects long_task_cost -- --nocapture
```

## Recorded results

Reference installation, release builds, `synchronous=FULL` unless stated.

| Measure | Value |
|---|---|
| Experiment 1 | no check: promoted, 2 replay problems, quarantined; full and scoped: refused, last known-good kept, no quarantine (recovered harness rerun 2026-10-10: same) |
| ms per create, 100 / 1,000 / 3,000 tasks | no check 3.27 / 3.20 / 3.14; full replay 4.44 / 8.58 / 18.56; scoped 3.46 / 3.26 / 3.12 |
| Design B crash points (before stage, after stage, mid-promote, none) | each recovers to one task, one `task.created`, no staged candidate, replay clean (rerun 2026-10-10: same) |
| ms per create, 300 creates | A FULL 3.26 / 3.31, B FULL 6.15 / 6.15, A NORMAL 0.39 / 0.42, B NORMAL 0.36 / 0.38 (rerun 2026-10-10: A FULL 3.43, B FULL 6.22) |
| fsyncs per create | A 1.03, B 2.03 |
| D1, D2, D3 on the first write-set check (2026-10-10) | each refused before promotion; replay clean, no quarantine, next create ok; control passes |
| D4–D8 on the first write-set check (verifier round 2) | each **promoted**: in the verifier's runs D4–D7 then quarantined the daemon (with this harness D5 does not, since the later completion overwrites the planted link); D8 rewrites task `b` and nothing notices |
| X2–X4 on the round-5 check (verifier round 5) | each **promoted**: X4's retry duplicates the task and the daemon is then quarantined; X2's retry answers with another task; X3 later refuses a legitimate command with `idempotency_conflict`; X2 and X3 are never seen |
| X2–X4 on the round-6 check (2026-10-10, daemon driver) | each refused before promotion, nothing quarantined; the legitimate `stored-response` and `extra-record` scenarios pass |
| R1, R3, S1 on the round-6 check (verifier round 6) | each **promoted** and never seen, health healthy |
| every planted defect on the round-7 check (2026-10-10): the harness's 18, and N1 ×2, N2 ×2, N5, N6, N8, N9, X2, X3, X4, R1, R3, S1 through the daemon | each refused before promotion, nothing quarantined; every legitimate scenario passes |
| D1–D8 on the round-3 check (2026-10-10) | each refused before promotion; replay clean, no quarantine, next create ok, task `b` unchanged; control passes |
| V1–V11 on the round-3 check (verifier round 3) | each **promoted**; the replay sees none of V1–V3, V6 |
| N1, N2, N5, N6, N8, N9 on the round-4 check (verifier round 4) | N1, N2, N5, N9, N8 **promoted** (N1 and N5, N8, N9 unseen afterwards; N2 then quarantines); N6's retry promoted, then quarantined |
| D1–D8, V1–V9, V11 and N1, N2, N5, N6, N8, N9 on the round-5 check (2026-10-10) | each refused before promotion, nothing quarantined; the harness's control and the daemon's legitimate scenarios pass |
| D1–D8, V1–V9, V11 on the round-4 check with per-command specs (2026-10-10) | each refused before promotion; replay clean, no quarantine, next create ok, task `b` unchanged; control passes |
| ms per step on one long task, round-4 check with per-command specs, three runs each (2026-10-10) | after 1,000 steps: no check 3.47 / 3.86 / 4.09, check 3.52 / 3.98 / 3.68 (noise larger than the difference); after 6,000: no check 3.24 / 3.46 / 3.26, check 4.06 / 4.56 / 3.98 |
| ms per step on one long task, round-3 check, three runs each (2026-10-10) | after 1,000 steps: no check 3.24 / 3.18 / 3.36, check 3.46 / 3.68 / 3.48; after 6,000: no check 3.30 / 3.22 / 4.09, check 3.96 / 4.14 / 4.12 |
| ms per step after 20,000 steps, first write-set check, one run (verifier round 2) | no check 3.18, check 5.10: the index-only count grows about 0.09 ms per 1,000 events on the task |
| ms per step on one long task, first write-set check, three runs each (2026-10-10) | after 1,000 steps: no check 3.14 / 3.26 / 3.22, check 3.36 / 3.72 / 3.68, check without index 3.87 / 3.78 / 3.94; after 6,000: no check 3.39 / 3.48 / 3.22, check 3.92 / 3.97 / 4.06, check without index 5.99 / 5.80 / 5.36 |

## Code

What each design adds for kernel transactions, counted from these sources:

- **Design B** (candidate/promotion), prototyped for one command, `task.create`: a staging table and about 95 lines (`b_create` 85, `b_recover` 8, the table 2). It stages in one commit and promotes in a second, runs the idempotency check in both, and needs recovery at open to discard staged candidates. Its validation covered only the title and status; the other three commands would each need their own staging shape. Never built for them, because the comparison above showed no outcome B gives that one transaction does not, at twice the fsyncs.
- **Design A plus the write-set check** (landed): no change to the commit path; 1107 lines of code covering all four task commands, `system.init`, their answers and the idempotency record (`apps/rikurid/src/candidate.rs` 572, the per-command specs in `candidate/spec.rs` 535), plus the write list and the deletion-unit count and index in `rikuri-storage` (`Tx::written`, `deletion_unit_size`) and the ledger's two reads and index, with about 250 added non-blank lines of production code across storage, ledger and kernel (some of them moved, not new). Most of those lines are TSE's "validate candidate" step, which design B would need just as much: B's 95 lines buy only the staging, the second commit and the recovery at open, with almost no validation.
