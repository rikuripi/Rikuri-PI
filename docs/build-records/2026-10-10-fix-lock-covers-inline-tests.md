# Build Record — The fix-lock covers Rust tests written inside a source file

| Field | Value |
|---|---|
| Change | `.claude/hooks/guard-config.json`: add the `inline_tests` rule (as in AISecurity and rikurinode) and worked examples, so that while a fix-lock is engaged the guard refuses changes to a `.rs` file's `#[cfg(test)]` tests, not only to files named by `test_globs` |
| Date | 2026-10-10 |
| Author / agent session | Claude Code session for Henry Chen (side session started from the Stage 00 session) |
| Branch | `claude/brave-kowalevski-a0588b` (worktree) |
| Commits | `1cd71be` (rule, examples, this record), close-out commit; PR #31 |
| Stage Gate | N/A: guard policy only; no Stage 00 item, interface or gate status changes |
| Related ADR / requirement | AGENTS.md "Deterministic gates" ("Tests and baselines become read-only"); lesson gate rikurinode `docs/build-records/2026-09-28-fix-lock-covers-inline-tests.md` |
| Status | COMPLETE locally (verifier round 1: NOT YET PROVEN, only for one uninterrupted gate run, which is CI's); PR opened at the owner's request 2026-10-11; CI and the owner's merge Not yet proven |

**Risk note (protected path):** `.claude/hooks/guard-config.json` is the guard's policy. The change only tightens it: it adds denies that apply while a fix-lock is engaged and changes nothing when no lock is held. `guard.py`, `guard_base.py` and `test_guard.py` are not touched (they are shared unchanged with the other full-tier repositories; `cmp` confirms they are identical today).

## 1. Plan

- [x] One-sentence statement of change: while a fix-lock is engaged, a session can no longer change the tests inside a Rust source file (from its first `#[cfg(test)]` marker to the end, with the attributes and comments directly above it), add or change code that behaves differently under test, or write any `.rs` file from the shell; production code in the same file stays editable.
- [x] Policy context (`main` moved during this work): PR #27 ([record](2026-10-10-fix-lock-optional-while-scaffolding.md)) made the fix-lock off by default while the code is scaffolding, with red-before-fix evidence and verifier step 4 standing in for it; a session may still engage it for a risky fix, and it becomes the default again when the Stage 00 gate closes. That record notes this very gap ("the lock's own `test_globs` never covered inline tests either"). This change makes an engaged lock, now optional and later the default, cover what AGENTS.md calls test code inside `.rs` files; it does not change when the lock is used.
- [x] Blast radius: Claude Code sessions in this repository, only while they hold a fix-lock. No product code, canonical data, money, execution or privacy path. Not irreversible: removing the two keys restores today's behaviour.
- [x] Current behavior characterized before changing; evidence: the hook, given real PreToolUse events with a real fix-lock engaged, denies a write to a `tests/` file but allows every change to `crates/rikuri-domain/src/time.rs`'s inline tests (Evidence log, "Before"). `guard.py fix-lock on` nonetheless prints "test files, inline tests and baselines are now read-only": the shared guard has supported `inline_tests` since 2026-09-30 (`5f81ced`), but the rule is per-repository config, and Rikuri-PI had no Rust code when the lesson gate added it to the other repositories. The first Rust code arrived on 2026-10-05 (`586d878`, S00.04) without the rule. Today 11 of the 53 tracked `.rs` files keep their tests inline, none of them covered.
- [x] Verification defined up front — "I will know this works when": (1) new worked examples in `guard-config.json`, written before the rule, fail against today's config and pass with the rule, without being changed in between; (2) the same hook-level dry run shows every inline-test change DENIED after, the production-code edit in the same file still ALLOWED, the `tests/` control still DENIED and a session without a lock still ALLOWED; (3) the guard tests and `scripts/quality/full-gate.sh` pass.
- [x] Architecture-impact check completed:
  - [x] invariant / ADR: none changed; brings the repository up to its own AGENTS.md rule.
  - [x] API / schema / event format: N/A.
  - [x] canonical vs derived data: N/A.
  - [x] privacy / instruction authority / security: tightens the AI-session tripwire; nothing loosened.
  - [x] deletion / backup / restore: N/A.
  - [x] budget / cost: N/A.
  - [x] nondeterministic AI / replay: N/A.
  - [x] documentation / help / RAG: the policy documents itself in `_inline_tests_doc`; AGENTS.md already says tests become read-only under the lock (no change needed).
  - [x] module ownership / dependency direction: N/A.
- [x] Relevant threat/control/test identified: threat is a session turning a failing inline test green by editing the test instead of the code (weakening an assertion, `#[cfg(any())]` above the module, `cfg!(test)` in production code, an assert-macro shadow, or a shell write the guard cannot diff). Control: the `inline_tests` rule. Tests: the worked examples, replayed by `test_guard.py`, plus `test_guard.py`'s generic inline-test cases (already present, run against a synthetic config).
- [x] Existing solution review: reuse the rule already proven in AISecurity and rikurinode, copied unchanged (the two repositories' `inline_tests` values are identical; checked with Python `==`). No custom code.
- [x] Authoritative state change? N/A (no Rikuri state).

**Behaviour change to expose (owner's choice, reversible):** with the rule, a session holding a fix-lock can no longer write *any* `.rs` file from the shell (`sed -i`, `git checkout -- <file>.rs`, interpreter code that writes a `.rs` path), and its Edit `old_string`s on `.rs` files must match the file exactly; it uses Edit/Write instead. That is how AISecurity and rikurinode behave today. It applies only to the lock holder; other sessions and lock-free work are unaffected. Still allowed under the lock (the hook is a tripwire, not a wall; probed below and by the verifier): `git stash`, and the commands listed as an accepted limitation under Skipped / Deferred. The worked examples quote `crates/rikuri-domain/src/time.rs` exactly, so a change to that file's quoted lines means updating them (said in `_inline_tests_doc` too): the production-edit example then turns the guard tests red, and the weakened-assertion example keeps passing but on the exact-match refusal instead of the rule. The fix-lock in this worktree's ignored state (`053e1595…`, the S00.11 E1 session's, copied in when the worktree was made) binds only that session, and that session's own checkout picks up this config only when its branch merges `main`. Undo: delete the `inline_tests` key.

## 2. Build

- [x] Smallest coherent implementation completed: two keys added to `guard-config.json` (`_inline_tests_doc`, adapted; `inline_tests`, byte-identical to AISecurity's line, sha256 `17dc6747…`, and equal as parsed JSON to rikurinode's) plus 13 worked examples. A first insertion through the Edit tool turned the rule's `\u200b`…`\ufeff` escapes into literal invisible characters (same parsed value, unreviewable bytes); the line was replaced with AISecurity's exact ASCII bytes, and the file has no invisible or format characters (checked: Unicode categories Cf/Zs/Zl/Zp/Cc beyond ASCII, none).
- [x] Inputs crossing trust boundaries validated: `load_config` already validates `inline_tests` (shape and regex compile) and the guard answers ASK on a bad config.
- [x] New code is typed and cohesive; large-file warning/justification handled: config only.
- [x] Old/replaced implementation removed in the same change: nothing replaced.
- [x] Module Contract updated if responsibility/interface changed: N/A (no crate changes).
- [x] Security question answered: no auth, money, execution or private data touched. The worked examples prove both paths on a real file (`crates/rikuri-domain/src/time.rs`): denied under the lock (weakened assertion, `#[cfg(any())]` attached above the marker, `cfg!(test)` added to production code, `sed -i`, `cd … && sed -i`, a python heredoc append; and for a new file, an appended `#[cfg(test)]` module, `#[cfg(not(test))]`, an `assert_eq` macro shadow) and allowed (the same test edit without the lock, `sed -i` without the lock, a production-code edit in the same file under the lock, a new file without tests under the lock). `guard_example_only.rs` does not exist; the guard reads a missing file as empty, as in AISecurity's examples.

## 3. Debug

- [x] Problem reproduced before fix; evidence: Evidence log, "Before".
- [x] Root-cause **class** named: per-repository guard policy drifts behind the repository's stack. The shared guard learned a protection, but each repository opts in through its own config, and nothing checks a repository's config again when it adopts a new language.
- [x] Sibling cases swept: AISecurity (50 `.rs`, 29 with inline tests) and rikurinode (222 `.rs`, 133 with inline tests) have the rule; Iceman and rikuripi/rikurinode track no `.rs` files today (N/A until they do). Within Rikuri-PI, `test_globs` covers `tests/*`, `*_test.rs`, `fixtures/*` and baselines, but **not the test-only crate `crates/rikuri-testkit/`** (FixtureClock, FixtureIds, CrashInjector; a dev-dependency only of its four consumers): its `src/lib.rs` has no `#[cfg(test)]` marker and matches no test glob, so an Edit to it under the lock is allowed (verifier round 1). Covering it is a further policy choice with a cost (a fix that needs a testkit change would need the owner to release the lock), so it is put to the owner, not made here (Skipped / Deferred).
- [x] State-changing remediation justified by evidence: config tightening only.

## 4. Verify

- [x] Unit tests added/updated, including failure paths; result: 13 worked examples (9 deny, 4 allow), each replayed under both profiles by `test_guard.ConfigExamples`; `python3 -m unittest discover -s .claude/hooks -p 'test_*.py'`: `Ran 91 tests … OK` (Evidence log).
- [x] Shared contract suite run if boundary/adapter changed: N/A.
- [x] Invariant/property test: N/A.
- [x] Replay/golden fixture: N/A.
- [x] Running-system probe observed (hook-level dry run); result: a scratch copy of the guard (this repository's `guard.py`/`guard_base.py`, the config before or after, `time.rs`) with a real per-session fix-lock taken through `guard.py fix-lock on`, fed PreToolUse JSON on stdin as Claude Code does. Before: the `tests/` control DENY, all four inline-test changes ALLOW. After: the control and all four DENY with the inline-test reasons; the production-code edit and a lock-free session's test edit ALLOW (Evidence log).
- [x] End-to-end trace: N/A (no product path).
- [x] Recovery/fallback path: N/A.
- [x] Regression guard proven to fail against the old behavior: the examples were added first, without the rule: `FAILED (failures=18)`, exactly the 9 deny examples × 2 profiles, each `AssertionError: 'allow' != 'deny'`; every other test passed. The rule was then added with the examples unchanged (sha256 of the examples list `81077750…` before and after) and the suite passed. A fix-lock does not apply here: the examples and the rule live in one file, and the lock cannot make half a file read-only; the unchanged-examples hash stands in for it.
- [x] Cost/performance: N/A.
- [x] Independent verifier round 1 (**NOT YET PROVEN**, only for one uninterrupted 15-step gate run, which is CI's; every run in the safety layer, `lnd` start time unchanged): red→green reproduced from `git archive HEAD` (`FAILED (failures=18)`, all `'allow' != 'deny'`, then `OK`; examples hash reproduced); hook dry run with its own per-session lock: 0 mismatches on the acceptance events, 22 deny cases all allowed on the old config, and further cases denied after (MultiEdit, whole-file Write, deleting the module, a new `cfg(any(test, …))` helper, `cfg_attr(test, …)`, `#[cfg(any())]` across a blank line and a comment, a `debug_assert` shadow, `replace_all`, `git checkout HEAD~1 --`, `cp`, `perl -pi`, `>>`, a `*.rs` glob, `python3 -c`) while production-only edits stayed allowed; shared files unchanged (`cmp` against all four other full-tier repositories); rule line byte-identical; no invisible characters; record facts and counts confirmed. Findings: (1) the testkit crate is not covered (sibling sweep corrected above; owner choice below); (2) more commands still pass under the lock than the record listed (limitation row extended); (3) the examples quote `time.rs` exactly, which the record did not say (now in the record and in `_inline_tests_doc`); (4) the gate box overstated a two-part run as green (box reopened, deferred row added); the `secrets` step scans history only, so both changed files were scanned directly: `gitleaks dir`: no leaks found.
- [x] Responses, round 1: (1)–(4) as above. After the `_inline_tests_doc` sentence was added: guard tests `Ran 91 tests … OK`, examples hash unchanged, rule line hash unchanged, `git diff --check` clean, `gitleaks dir` on both files: no leaks found. The other gate steps do not read the guard config, so they were not rerun.

## 5. Close Out

- [ ] Full required gate green — every step PASS, but across two runs; one uninterrupted run is Not yet proven (CI on the PR). `scripts/quality/full-gate.sh` in the safety layer passed its first nine steps (tools, gate self-tests, guard, format, build, clippy, test with 176 passed / 0 failed, smoke, crash matrix) and stopped at `dependencies`, which cannot fetch the RustSec advisory database without a network (`Could not resolve host: github.com`). The remaining six steps were then run with the gate's exact commands in the same layer sharing the host network (only the DNS resolver directory re-exposed under `/run`): dependencies, secrets, ratchet, contracts, crate policy and scrap all PASS. `lnd` started at `2026-10-09T20:20:27.903167252Z` before and after both runs. One uninterrupted 15-step run is CI's to show.
- [x] Scrap Sweep complete; deleted artifacts listed: the builder's and the verifier's scratch copies of the guard under the ignored `target/safety/` (`dryrun-before/`, `dryrun-after/`, `verifier/{red,green,hook-before,hook-after,drift-prod,drift-test}/`; reproducible with `target/safety/dryrun.py`). Kept there as evidence receipts: the safety layer, the dry-run harness, the before/after configs and every log. Nothing task-created is tracked except the config change and this record; the other session's lock file in `.claude/state/fix-locks/` is untouched.
- [x] No unexplained skipped/disabled tests or new unowned debt: none skipped; each item below has an owner and a condition.
- [x] Documentation updated in the same change: `_inline_tests_doc` in the config; AGENTS.md, README N/A (already accurate).
- [x] Traceability: N/A (no security or architecture promise of the product changes).
- [x] Stage Gate Record: N/A (no gate item affected).
- [x] Commit message explains WHY; Build Record committed with change. The owner chose "Create PR" for this session on 2026-10-11 (protected path: the PR is the owner's review; merging is the owner's).

## Skipped / Deferred / Resequenced — visible, never silent

| Item | Disposition | Reason | Dependency impact | Owner | New stage/ordinal | Reactivation / acceptance condition |
|---|---|---|---|---|---|---|
| Class tripwire: a repository that tracks Rust inline tests must have an `inline_tests` rule; `fix-lock on` must not print "inline tests … read-only" when its config has no such rule | DEFERRED (proposed follow-up) | Both belong in the shared `guard.py`/`test_guard.py`, which change only in rikurinode and are then copied unchanged to every full-tier repository (owner's rule); out of this task's scope. Today this repository's examples catch removal of its own rule | None for Stage 00 work. Iceman and rikuripi/rikurinode stay exposed only if they adopt Rust before it lands | Henry Chen | Next shared-guard change | A generic `test_guard.py` case fails when tracked `.rs` files contain `#[cfg(test)]` and no `inline_tests` glob covers them, shown red against a config without the rule |
| One uninterrupted 15-step `full-gate.sh` run | Not yet proven | This machine runs the gate without a network for safety, and `cargo deny` needs one; all 15 steps passed across two runs | Blocks calling the gate green; nothing downstream | Henry Chen | This PR | `Full gate (Ubuntu)` passes on the PR, up to date with `main` |
| `crates/rikuri-testkit/` is test-only code the fix-lock does not cover | Owner choice (proposed, not made) | Same class as this change (AGENTS.md since PR #27 counts `rikuri-testkit` as test code), but it widens what a lock freezes (a fix needing a testkit change would need the lock released); out of this task's brief | None today | Henry Chen | Next guard-policy change | If chosen: `crates/rikuri-testkit/*` in `test_globs` with a deny-under-lock example, red first |
| Commands that still rewrite inline tests under the lock: `cargo fmt`, `rustfmt`, `cargo clippy --fix`, `find … -exec sed -i`, `git apply`, `patch -p1 <`, `sed -i "$F"` with the path in a variable | Accepted limitation (recorded) | The hook is a tripwire, not a sandbox (AGENTS.md); the same holds in AISecurity and rikurinode. Review of the diff and the owner remain the backstop | None | Henry Chen | — | Revisit if a locked session is seen changing tests this way |

## Evidence log

Every test, gate and dry run below ran inside an outer safety layer (user, mount, PID and network namespaces; `/run` masked by a tmpfs; private `/tmp`; refusing stand-ins for docker, sudo, systemctl, lncli and the like first on `PATH`, checked inside; `DOCKER_HOST` at a missing socket). Layer, harness and logs live in the ignored `target/safety/`.

```text
# Shared files unchanged and identical across repositories (cmp), 2026-10-10
guard.py, guard_base.py, test_guard.py: identical to AISecurity's and rikurinode's (rikuri/rikurinode)

# Before — hook dry run, config at HEAD 48bb7ee, real per-session fix-lock engaged
$ guard.py fix-lock on docs/build-records/2026-10-10-fix-lock-covers-inline-tests.md   (CLAUDE_CODE_SESSION_ID=dryrun-holder)
fix-lock ENGAGED for docs/build-records/2026-10-10-fix-lock-covers-inline-tests.md (this session) — test files, inline tests and baselines are now read-only
[before] control: a tests/ file (test_globs)                            exit 0  DENY — fix-lock is engaged: crates/rikuri-domain/tests/clock.rs is a test/baseline. …
[before] weaken an assertion in time.rs's #[cfg(test)] module           exit 0  ALLOW (no output)
[before] switch the tests off: #[cfg(any())] attached above the marker  exit 0  ALLOW (no output)
[before] production code acts differently under test: cfg!(test)        exit 0  ALLOW (no output)
[before] shell write to the file: sed -i                                exit 0  ALLOW (no output)
[before] production-code edit in the same file (the fix)                exit 0  ALLOW (no output)
[before] same test edit from a session that holds no lock               exit 0  ALLOW (no output)

# Red — 13 examples added, rule not yet added
97 examples, sha256 81077750acf9e45d33f51e03fb9899884bc1bffe0f78b8604e19b02b35e46005
Ran 91 tests in 8.573s
FAILED (failures=18)            # 18 × "AssertionError: 'allow' != 'deny'" = 9 deny examples × {stable, development}

# Rule added (examples untouched)
97 examples, sha256 81077750acf9e45d33f51e03fb9899884bc1bffe0f78b8604e19b02b35e46005
inline_tests line sha256: AISecurity 17dc6747…a1c3 = Rikuri-PI 17dc6747…a1c3; parsed == rikurinode: True
load_config OK

# Green
$ python3 -m unittest discover -s .claude/hooks -p 'test_*.py'
Ran 91 tests in 8.956s
OK

# After — same dry run, new config
[after] control: a tests/ file (test_globs)                            exit 0  DENY — fix-lock is engaged: crates/rikuri-domain/tests/clock.rs is a test/baseline. …
[after] weaken an assertion in time.rs's #[cfg(test)] module           exit 0  DENY — fix-lock is engaged: this changes the tests inside crates/rikuri-domain/src/time.rs (its first inline-test marker to the end of the file, …
[after] switch the tests off: #[cfg(any())] attached above the marker  exit 0  DENY — fix-lock is engaged: this changes the tests inside crates/rikuri-domain/src/time.rs …
[after] production code acts differently under test: cfg!(test)        exit 0  DENY — fix-lock is engaged: this adds, removes or changes code in crates/rikuri-domain/src/time.rs that acts differently under test …
[after] shell write to the file: sed -i                                exit 0  DENY — fix-lock is engaged: command writes crates/rikuri-domain/src/time.rs, a file type that keeps tests inside the code (inline_tests). …
[after] production-code edit in the same file (the fix)                exit 0  ALLOW (no output)
[after] same test edit from a session that holds no lock               exit 0  ALLOW (no output)

# Probe, new config, lock held (guard.decide, stable profile)
cargo fmt --all                                    allow
cargo test -p rikuri-domain                        allow
cargo clippy --fix --allow-dirty -p rikuri-domain  allow
rustfmt crates/rikuri-domain/src/time.rs           allow
cat crates/rikuri-domain/src/time.rs               allow
git checkout -- crates/rikuri-domain/src/time.rs   deny
git stash                                          allow
Edit with a curly-quote (loose-match) old_string   deny
Edit of a .md record                               allow

# Full gate, run 1 (no network)
lnd before gate: 2026-10-09T20:20:27.903167252Z
── GATE tools: PASS · gate-selftests: PASS · guard: PASS · format: PASS · build: PASS · clippy: PASS
── GATE test: PASS            (176 passed, 0 failed)
SMOKE PASS: init, task create, state.show content, ledger and log without content, health, restart
── GATE smoke: PASS · crash-matrix: PASS
── GATE dependencies: cargo deny --locked check
[ERROR] failed to fetch advisory database https://github.com/RustSec/advisory-db … Could not resolve host: github.com
RESULT: FAIL (dependencies)
lnd after gate: 2026-10-09T20:20:27.903167252Z

# Full gate, steps 10–15 with the gate's commands (host network, /run masked except DNS)
advisories ok, bans ok, licenses ok, sources ok
── GATE dependencies: PASS · secrets: PASS (no leaks found) · ratchet: PASS · contracts: PASS · crate-policy: PASS
SCRAP untracked files (need a disposition; not failed):
  docs/build-records/2026-10-10-fix-lock-covers-inline-tests.md
── GATE scrap: PASS
lnd before/after: 2026-10-09T20:20:27.903167252Z
```
