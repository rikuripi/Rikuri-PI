# Build Record — Shared guard: a tripwire for Rust inline tests that no fix-lock rule covers (sync)

| Field | Value |
|---|---|
| Change | `.claude/hooks/guard.py` and `test_guard.py` replaced by the shared change: the guard tests fail while a tracked `.rs` file holds `#[cfg(test)]` tests that no `inline_tests` rule covers, and `fix-lock on` says only what it makes read-only, naming the files whose inline tests it does not |
| Date | 2026-10-11 |
| Author / agent session | Claude Code (Opus 5.5) for Henry Chen; side session `musing-khayyam-31764d`, started from a Rikuri-PI session |
| Branch | `claude/musing-khayyam-31764d` (worktree), from `main` at `be979e3` |
| Commits | one commit on this branch, this record included (owner, 2026-10-11: "Commit in rikuripi/rikurinode, and one sync commit per repository"); its PR waits for PR #31 |
| Stage Gate | N/A: working rules; no Stage 00 item, interface or gate status changes |
| Related ADR / requirement | AGENTS.md > Deterministic gates; the owner's rule that `guard.py`, `guard_base.py` and `test_guard.py` stay identical across the Full-tier repositories; the deferred class tripwire in [2026-10-10-fix-lock-covers-inline-tests.md](2026-10-10-fix-lock-covers-inline-tests.md) (PR #31) |
| Status | COMPLETE locally — verified in a clean clone and by the origin's verifier; committed; the PR, full gate and CI wait for PR #31 |

**Risk note (protected paths):** `.claude/hooks/guard.py` and `test_guard.py` are the AI-session guard. The hook's verdicts do not change; `fix-lock on` gains a warning and loses a claim that was false here. `guard_base.py` and `guard-config.json` are unchanged by this record.

## 1. Plan

- [x] One-sentence statement: this repository gets the shared guard's class tripwire for Rust inline tests, unchanged from the origin.
- [x] Why: on 2026-10-10 this repository was found with 11 tracked `.rs` files keeping `#[cfg(test)]` tests and no `inline_tests` rule, so a fix-lock left them editable while `fix-lock on` said "test files, inline tests and baselines are now read-only". The rule is PR #31; its record deferred the class tripwire to the next shared-guard change, owner Henry. This is that change.
- [x] Where it was built and verified: rikuripi/rikurinode, record `docs/build-records/2026-10-11-guard-inline-tests-tripwire.md` (tests first and seen red, 22 of 22 mutants red, the five repositories run against their own policies). Owner decisions, 2026-10-11: start in rikuripi/rikurinode; **this repository's rule (PR #31) lands first**; `fix-lock on` engages and names uncovered files; then "Unstack, commit now" (the line-continuation fix had not landed anywhere and was being redone, so this change lands first and that fix is re-applied on top) and "A branch in each repo".
- [x] Blast radius: Claude Code sessions in this repository (the guard tests in CI and the full gate; the `fix-lock on` message). No product code, canonical data, money, execution or privacy path. Reversible by restoring the two files.
- [x] **I will know this works when** the files here equal the origin's byte for byte (`cmp`), the guard suite is red on today's `main` on exactly the new tripwire and green with PR #31's rule, and CI's full gate passes on the pull request.

## 2. Build

- [x] `.claude/hooks/guard.py` (sha256 `c1d54b73…`) and `test_guard.py` (`08e00cb4…`): copied from rikuripi/rikurinode branch `fix/guard-inline-tests-tripwire`, which is that repository's `main` (`a443085`) plus this change; this repository's committed copies equal that base, so the diff here is this change only (267 lines added, 3 removed); `guard_base.py` already equal (`cmp`).

## 3. Debug

N/A here: reproduced, written red first and fixed in the origin. Root-cause class: per-repository guard policy drifts behind the repository's stack; nothing re-checked this repository's config when Stage 00 added Rust (2026-10-05).

## 4. Verify

All runs in a clean clone of `main` at `be979e3`, inside the outer test-safety layer (rikuripi/rikurinode `scripts/quality/safe-run.sh`), lnd's StartedAt unchanged. Receipts are in the origin's `docs/build-records/evidence/2026-10-11-guard-inline-tests-tripwire/`.

- [x] **Red on today's `main`** (its own config, no rule): 107 run, 1 failure on Python 3.14 and on 3.9.25 — `GenericPolicy.test_tracked_inline_tests_have_a_rule`, naming the 11 files (`crates/rikuri-api/src/command.rs`, `…/lib.rs`, `crates/rikuri-domain/src/{authority,deletion,id,privacy,risk,schema,time,trust,wire}.rs`) — `sync-Rikuri-PI-main.out`. This is the defect the change exists to catch.
- [x] **Green with PR #31's rule** (its `guard-config.json`, byte-identical to PR #31's head): 107 OK on Python 3.14 and on 3.9.25 — `sync-Rikuri-PI-with-rule.out`.
- [x] Real entry point: `python3 .claude/hooks/guard.py fix-lock on …` in that clone (`rpi-fix-lock-on.out`): the old guard says "test files, inline tests and baselines are now read-only" with no rule; the new one says "test files and baselines" and warns that the tests inside 11 tracked files are NOT read-only; with PR #31's rule it says "test files, inline tests and baselines" and warns nothing.
- [x] Recovery/fallback: N/A — none changes; a guard crash still asks.
- [x] The origin's independent verifier: round 1 FAIL — `fix-lock on` crashed on a tracked `.rs` file whose name is not UTF-8, and evidence carried trailing spaces; both fixed in round 2 with tests first (22 of 22 mutants red). Round 2 **PASS** on the stacked files, including this worktree's copy (`cmp`) and this repository's clone (red on `main` only on the tripwire, naming 11 files; 118 OK on 3.14 and 3.9.25 with PR #31's rule, its head `8e2ddc2`). Round 3 (these unstacked files): **PASS**, including this worktree's copy (`cmp`). It also noted that `origin/main` has since moved to `5bc2948`, where the tripwire finds 13 uncovered files (new `.rs` files since `be979e3`) and 0 with PR #31's rule: the red/green pair holds there too. Not yet proven: Python 3.12, CI. Its one note for this record: the link to `2026-10-10-fix-lock-covers-inline-tests.md` resolves only once PR #31 merges, which is the planned order.
- [ ] Full gate on this branch (`scripts/quality/full-gate.sh`): not run — it would be red until PR #31 is in `main`. Run after merging `main` once #31 has landed.
- [ ] CI: Not yet proven.

## 5. Close Out

- [x] Scrap Sweep: nothing created in this repository but this record; the clone lives in the session scratchpad.
- [x] Docs: `guard.py`'s header describes the tripwire. No document here quotes the old `fix-lock on` message; AGENTS.md ("makes tests and baselines read-only") stays true.
- [x] Commit: made on this branch at the owner's request (2026-10-11), the two files and this record. Then: PR #31 merges; this branch merges `main`, `cmp` against the origin again, runs the full gate and opens its PR. The line-continuation sync (`chore/guard-line-continuation`) is re-applied on top of this change afterwards.

## Skipped / Deferred / Resequenced — visible, never silent

| Item | Disposition | Reason | Dependency impact | Owner | New stage/ordinal | Reactivation / acceptance condition |
|---|---|---|---|---|---|---|
| PR, full gate and CI for this sync | Resequenced (owner, 2026-10-11) | On today's `main` the new test is red by design until PR #31's rule is in | None for Stage 00 work | Henry Chen | After PR #31 | Branch merged up to `main`; `cmp` equal to the origin; full gate and CI green |
| `#[cfg(any(test, …))]`-only files are not flagged | Accepted limitation (recorded in the origin) | Every rule in use covers those forms; no repository has such a file without a plain `#[cfg(test)]` | None | Henry Chen | — | Revisit if one appears |

## Evidence log

```
main be979e3, own config:   107 run, 1 failure (the new tripwire; 11 files), py3.14 and py3.9.25   sync-Rikuri-PI-main.out
main + PR #31's rule:       107 OK (py3.14), 107 OK (py3.9.25)                    sync-Rikuri-PI-with-rule.out
fix-lock on, old vs new:    see rpi-fix-lock-on.out
files:                      guard.py c1d54b73…, test_guard.py 08e00cb4…, guard_base.py cmp-equal
lnd StartedAt:              2026-10-09T20:20:27.903167252Z before and after every run
```
