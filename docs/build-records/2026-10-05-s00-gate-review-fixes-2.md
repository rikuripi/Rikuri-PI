# Build Record — PR #7 re-verification defects: lint escapes, wrapped suppressions, ignored files

| Field | Value |
|---|---|
| Change | Close the gate gaps the second review of PR #7 reproduced: lint policy for every workspace member, build flags that cap lints, rustfmt-wrapped suppressions, force-added ignored files, and a redaction assertion that could not fail |
| Date | 2026-10-05 |
| Author / agent session | Claude Code session for Henry Chen |
| Branch | `stage-00/bootstrap` |
| Commits | `dab40d6` (regression tests, red), `cd4e7fb` (fixture amendments, owner-approved), `c5bee23` (regression tests for the verifier's bypasses, red), `ca06400` (round-2 regression tests, red), `309ebb6` (round-3 regression tests, red), `5e8148a` (fix) |
| Stage Gate | [Stage 00](../stage-gates/foundation0/00-bootstrap-kernel.md), Bootstrap gate |
| Related ADR / requirement | S00.02, S00.03, T00.02, T00.11; [first review-fixes record](2026-10-05-s00-gate-review-fixes.md) |
| Status | COMPLETE (2026-10-05; CI run 37414127507 green. Session `05c5e1d0…`'s fix-lock released by the owner 2026-10-06T16:29:04) |

**Risk note (protected paths):** changes `scripts/quality/*` and `.gitleaks.toml`; F1 and F2 let the required gate pass code that breaks the workspace lint policy.

## 1. Plan

- [x] One-sentence statement: make the crate policy, contract check, ratchet and Scrap Sweep judge everything their records claim, and replace a redaction assertion that could not fail.
- [x] Blast radius: gate scripts and README; no product code.
- [x] Current behavior characterized: the verifier's reproductions (second PR #7 report) and the failing tests below.
- [x] **I will know this works when** every new regression test fails on the commit that adds it and passes after the fix with the test files unchanged (`git diff <tests commit> <fix commit> -- scripts/quality/tests` empty), the strengthened redaction test fails against a scan without `--redact`, and the full gate passes locally and in CI. *Amended 2026-10-05 (owner decision, Debug below): the tests commit is `cd4e7fb`, which corrects fixtures `dab40d6` got wrong; the red/green evidence is rerun for it. Amended again after verification rounds 1–3 (owner decisions): the tests commit is `309ebb6`, so `git diff 309ebb6 <fix commit> -- scripts/quality/tests` must be empty.*
- [x] Existing solution review: workspace members come from `cargo metadata --no-deps`, Cargo's own answer, instead of directory globs.

## 3. Debug

| # | Defect (found by the second review) | Root-cause class | Tripwire (committed before the fix) |
|---|---|---|---|
| F1 | A workspace member outside `crates/` and `apps/` escapes the lint policy and the contract check; Architecture §45 plans members under `adapters/` | The checks enumerated directories they assumed, not what Cargo builds | `tests/workspace-policy.sh` (two cases) |
| F2 | `.cargo/config.toml` or `RUSTFLAGS` with `--cap-lints allow` (or `-A …`) silences every lint while all checks pass | Lint configuration can be overridden outside the manifests the policy reads | `tests/workspace-policy.sh` (three cases) |
| F3 | A rustfmt-wrapped `#[cfg_attr(…, allow(…))]` (the form the format step requires) is not counted by the ratchet; same for a wrapped `#[ignore]` | Line-based regex over multi-line attributes | `tests/ratchet-wrapped.sh` |
| F4 | `secret-scan-merge.sh` checked a slice of the key gitleaks never prints even unredacted, so "the key is not printed" could not fail | Assertion did not target what the tool actually prints | `tests/redaction-and-ignored-files.sh` checks every key line; shown failing against a scan without `--redact` |
| F5 | A file force-added under the ignored top-level `target/` passes the secret scan (allowlisted) and the Scrap Sweep | The sweep did not look for tracked files that `.gitignore` excludes | Same test, second part |

Not changed here, recorded:
- **Guard audit trail (verifier):** the Stage Plan edit in ea109b0 was made by inline Python in a shell command, which the guard does not detect as a protected-path write, so `.claude/state/guard-relaxed.log` has no entry for it. The guard is a tripwire, not a wall (AGENTS.md). Working rule from now on: edit protected paths with the Edit/Write tools, which the guard sees and logs.
- `UnknownVariant`'s fields are public, so a struct literal could bypass the bound (today only `::new` builds it); a bounded error cut can split an escape sequence (cosmetic). Kept as known limitations of S00.04.

## Evidence log

```text
# Failing before the fix (tests written; nothing fixed)
$ scripts/quality/tests/workspace-policy.sh
WORKSPACE ok: a clean workspace passes the crate policy
WORKSPACE ok: a clean workspace passes the contract check
WORKSPACE FAIL: a member outside crates/ and apps/ without workspace lints is refused (expected fail, got pass)
WORKSPACE FAIL: a member outside crates/ and apps/ without a contract is refused (expected fail, got pass)
WORKSPACE FAIL: a .cargo/config.toml that caps lints is refused (expected fail, got pass)
WORKSPACE FAIL: RUSTFLAGS that cap lints are refused (expected fail, got pass)
WORKSPACE FAIL: RUSTFLAGS that allow a lint are refused (expected fail, got pass)
exit=1
$ scripts/quality/tests/ratchet-wrapped.sh
WRAPPED FAIL: wrapped suppressions and ignores did not raise the ratchet (lint_suppressions 0, ignored_tests 0)
exit=1
$ scripts/quality/tests/redaction-and-ignored-files.sh
REDACT ok: no line of the key appears in the scan output
REDACT ok: the finding is shown with the secret redacted
IGNORED FAIL: a force-added ignored file passed the Scrap Sweep
exit=1
# The strengthened redaction assertion against a copy of the scan without --redact
REDACT FAIL: part of the key appears in the scan output
REDACT FAIL: no redacted finding shown
```

### Handoff (2026-10-05 19:20)

The session that began this record (`14531436…`, fix-lock 19:16:48) stopped mid-fix at 19:17:53 with the fix uncommitted. Session `05c5e1d0…` resumed it, engaged its own fix-lock on this record at 19:23:52 (the first lock binds only its own, now ended, session), and changed no test file.

### Fix

- F1/F2: `workspace-members.sh` (new) lists members with `cargo metadata --no-deps`; `crate-policy.sh` and `module-contracts.sh` judge those members; `crate-policy.sh` refuses `--cap-lints`, `-A`/`--allow` in Cargo config files (root, parents, `CARGO_HOME`) and in `RUSTFLAGS`-style variables. The helper was created without its executable bit, so every case of `workspace-policy.sh` failed with `Permission denied`; set (`chmod +x`). It fails with a one-line `MEMBERS` message, not a traceback, when Cargo cannot load the workspace.
- F3: `ratchet.sh` counts attributes over whole files (wrapped `cfg_attr(…allow…)` and `#[ignore]`). F5: `scrap-sweep.sh` fails on tracked files `.gitignore` excludes. F4: no gate change; the tests carry the stronger assertion (below).
- README: gate step list updated for the above.

### Test-fixture defects found while verifying the fix

| # | Defect | Class |
|---|---|---|
| T1 | `tests/workspace-policy.sh`: the "clean" fixture lists `members = ["crates/*", "tools/*"]` with no `tools/` member. Cargo refuses that workspace (`failed to load manifest for workspace member …/tools/*`; `cargo build` refuses it too, cargo 1.98.1), so the two "clean passes" cases and the three flag cases failed before the policy ran | Fixtures modelled the old directory-glob implementation, not a workspace Cargo can load |
| T2 | `tests/crate-checks.sh` (first review round, green before this fix): no root `Cargo.toml` and no crate targets, so Cargo found no workspace. Its two pass cases failed; its fail cases passed for the wrong reason (Cargo error, not the policy) | Same |
| T3 | `tests/secret-scan-merge.sh` still held the F4 assertion that could not fail: it passed against a scan without `--redact` (shown below) | Assertion did not target what gitleaks prints |

**Owner decision (2026-10-05, in session):** amend the fixtures (no assertion changes) and strengthen the merge test's key check in the same commit; this session's fix-lock was released for that and re-engaged before the fix was verified. Commit `cd4e7fb`: `workspace-policy.sh` gains a compliant `tools/ok` member; `crate-checks.sh` gains a root workspace manifest, `edition` and `src/lib.rs`; `secret-scan-merge.sh` checks every line of the key.

```text
# Amended tests (cd4e7fb) against each version of the scripts (scratch copies; 2026-10-05 19:28-19:30)
workspace-policy.sh  vs pre-fix (dab40d6)   2 ok, 5 FAIL (outside-member x2, config cap, RUSTFLAGS x2)
workspace-policy.sh  vs fix                 7 ok
crate-checks.sh      vs first-round pre-fix (5908a8c)
                                            FAIL opt-in accepted (crate-policy.sh absent); FAIL "## Healthiness" counted
crate-checks.sh      vs dab40d6 and vs fix  5 ok
secret-scan-merge.sh original (HEAD) vs a scan without --redact
                                            MERGE ok: the key is not printed   <- could not fail (T3)
secret-scan-merge.sh amended vs a scan without --redact
                                            MERGE FAIL: the scan printed part of the key
secret-scan-merge.sh amended vs dab40d6 and vs fix
                                            3 ok

# Fix, real tree (fix-lock engaged; tests as committed in cd4e7fb)
$ scripts/quality/full-gate.sh
── GATE tools … scrap: PASS (13 of 13)
RESULT: PASS (13 steps)
```

### Verification round 1: verifier FAIL, and the fix

The `verifier` subagent (2026-10-05, ~19:35–19:56) passed the gate (the literal red runs and CI were Not yet proven) but defeated the fix in four classes:

| # | Finding | Class | Tripwire (`c5bee23`, committed before the fix) |
|---|---|---|---|
| V1 | **Regression:** the whole-file ratchet scanner ended a `cfg_attr` at a `)` inside a string or comment, so `#[cfg_attr(all(), doc = ")", allow(clippy::unwrap_used))]` counted 0 where the line-based ratchet counted 1 (fmt- and clippy-clean). Also never counted: `#[cfg_attr(…, ignore)]`, `#[/* c */ ignore]` | Text matching that does not know Rust comments and literals | `tests/ratchet-lexing.sh` (7 cases, including comment/string text that must not count) |
| V2 | A crate in `exclude` used as a path dependency compiles outside the lint policy, contract check and `clippy --workspace` | Policy judged members, not every local crate Cargo builds | `tests/policy-bypasses.sh` (F1 part) |
| V3 | Cargo config switches clippy off without a lint flag: `[alias] clippy = …`, `build.rustc-wrapper` (verifier); `CARGO_BUILD_RUSTDOCFLAGS` and `CARGO_ENCODED_RUSTDOCFLAGS` unchecked (verifier, by reading the code). Found by this session while writing the tests: `[env] CLIPPY_ARGS` with `force` (probe: clippy passed an `unwrap()` it refuses) and the environment forms `RUSTC_WRAPPER`, `CARGO_ALIAS_CLIPPY` | Policy checked lint flags, not other ways to replace what clippy runs | `tests/policy-bypasses.sh` (F2 part; legitimate alias, `[env]`, empty `RUSTC_WRAPPER` and member path dependencies stay green) |
| V4 | A key force-added under `target/` and deleted in the next commit: the sweep sees only HEAD, and the scan's `^target/` allowlist exempted it in all of history | An allowlist for a path that is never committed legitimately | `tests/secret-scan-target-history.sh` |

Owner decision (2026-10-05, in session): fix all four, tests first. Fix:

- `ratchet.sh`: blanks comments (nested block comments too) and the contents of string, raw-string, byte and char literals before matching (lifetimes left alone); a `cfg_attr` holding `ignore` counts as an ignored test.
- `workspace-members.sh --outside` lists members' path dependencies that are not members (`cargo metadata --no-deps`: offline, never writes `Cargo.lock`). A first version used full resolution; it wrote `Cargo.lock` into the fixture and then failed `--locked`, so it was replaced. `crate-policy.sh` refuses those, path `[patch]`/`[replace]` entries to non-members (manifest and config), config `paths` overrides, `[alias]` for the commands the gate runs, `build.rustc`/`rustc-wrapper`/`rustc-workspace-wrapper`/`rustdoc`, `[env]` clippy/rustc variables, and the environment forms (`RUSTC*`, `RUSTDOC`, `CARGO_BUILD_RUSTC*`, `CLIPPY_ARGS`, `CARGO_ALIAS_<gate command>`, `CARGO_BUILD_RUSTDOCFLAGS`, `CARGO_ENCODED_RUSTDOCFLAGS`). A failed member listing no longer stops the script before the configuration checks name the cause.
- `.gitleaks.toml`: the `^target/` allowlist removed (both scans read only tracked content; target/ is ignored).
- `scrap-sweep.sh`: the force-added check reads only the repository's `.gitignore` files, so a personal excludes file cannot change the result (verifier observation).
- README: gate description corrected (the verifier found three phrases that overstated it) and a note to run the gate without `RUSTC_WRAPPER`.

```text
# c5bee23 tests against the round-1 fix (before this fix), 2026-10-05 19:57
ratchet-lexing.sh              6 FAIL (3 hidden allow, cfg_attr ignore, commented ignore, comment/string text counted 2/5), 1 ok
policy-bypasses.sh             7 FAIL (excluded path dep, alias, rustc-wrapper, [env] CLIPPY_ARGS, RUSTC_WRAPPER,
                               CARGO_ALIAS_CLIPPY, CARGO_BUILD_RUSTDOCFLAGS), 5 ok (clean and legitimate cases)
secret-scan-target-history.sh  TARGET FAIL: a key committed under target/ and later deleted passed the scan

# After the fix (fix-lock engaged since c5bee23; tests unchanged), 2026-10-05 20:03
crate-checks 5 ok · policy-bypasses 12 ok · ratchet-and-paths 2 ok · ratchet-lexing 7 ok · ratchet-wrapped 1 ok
redaction-and-ignored-files 3 ok · secret-scan-merge 3 ok · secret-scan-scope 2 ok · secret-scan-target-history 1 ok
workspace-policy 7 ok   (all exit 0)
Ad hoc (no committed test): manifest [patch] to vendor/foo refused; [patch] to a member passes;
config [patch] and `paths` override refused; no Cargo.lock written
$ scripts/quality/full-gate.sh → RESULT: PASS (13 steps); secrets: 58 commits scanned, no allowlist of our own
  (corrected after round 2: gitleaks' built-in path allowlist still applies)
```

### Verification round 2: verifier FAIL, threat model, and the fix

The verifier (~20:06–20:21) confirmed every round-1 bypass caught and the full gate green, then found more in the same classes. R1–R4 were each shown to let the gate pass code it refuses (clippy exit 0 on an `unwrap()`, or `cargo test` exit 0 on a failing test); R5 and R7 hide suppressions from the ratchet; R6 hides a secret; R8 is documentation:

| # | Finding | Tripwire (`ca06400`, committed before the fix) |
|---|---|---|
| R1 | rustflags `@tools/lints.txt` (an argument file holding `--cap-lints allow`), also via `RUSTFLAGS` | `tests/policy-bypasses.sh` |
| R2 | rustflags `--force-warn clippy::…`: `-D warnings` does not promote force-warned lints | same |
| R3 | A member `build.rs` printing `cargo::rustc-env=CLIPPY_ARGS=--cap-lints=allow…` (no `println!`, fmt- and clippy-clean) | same (`RUSTC_BOOTSTRAP` too; an unrelated variable stays green) |
| R4 | A target `runner = "true"` makes a failing test pass (sibling class: the test step) | same (config and `CARGO_TARGET_<triple>_RUNNER`) |
| R5 | Raw-identifier attribute names `#[r#allow(…)]`, `#![r#allow]`, `#[r#expect]`, `#[r#cfg_attr(…allow…)]`, `#[r#ignore]` counted 0 | `tests/ratchet-lexing.sh` |
| R6 | gitleaks' built-in allowlist (`useDefault`) skips some paths (`package-lock.json`, `go.sum`, `*.svg`, `node_modules/`), contradicting "no path allowlist" | Accepted gap (below) |
| R7 | Suppressions in non-`.rs` files compiled through `#[path]` or `include!` are not counted (the old ratchet had the same gap) | Accepted gap (below) |
| R8 | Six record/README inaccuracies, and refusals with no committed test (more than the "Owed tests" item listed) | Corrected here; the same commit pins the hand-checked refusals named in round 2 in `tests/policy-bypasses.sh` (seven round-1 forms were missed and pinned in round 3), and the sweep's `.gitignore`-only rule in `tests/scrap-sweep-excludes.sh` |

**Owner decision (2026-10-05, in session): threat model and stopping rule.** The gate is a tripwire for accidental or careless weakening and matches the common forms. Deliberate subversion by someone with commit access is for review and branch protection. Close R1–R5 and R8 tests-first, accept R6 and R7 as recorded gaps, and run a final verifier round for regressions and accuracy only, not new bypass hunting.

Fix (fix-lock engaged since `ca06400`; tests unchanged):

- `crate-policy.sh`: rustflags starting with `@` or carrying `--force-warn` are refused; a target `runner` in Cargo config or `CARGO_TARGET_*_RUNNER` is refused; member `.rs` sources are searched for `rustc-env=` assignments to clippy/rustc variables and for clippy's argument separator. The threat model is stated in its header.
- `ratchet.sh`: attribute names may be raw identifiers.
- `.gitleaks.toml`: comment corrected; the built-in allowlist recorded as an accepted gap.
- README: gate description corrected (rustflags, raw identifiers, `.rs` only) and the threat model stated with a link to the gaps below.

```text
# ca06400 tests against the code before this fix (the state verification round 2 examined), 2026-10-05 20:23
policy-bypasses.sh   8 FAIL (@argfile x2, --force-warn x2, build.rs CLIPPY_ARGS and RUSTC_BOOTSTRAP,
                     config runner, CARGO_TARGET_..._RUNNER), 40 ok (pinned refusals and legitimate cases)
ratchet-lexing.sh    1 FAIL (raw-identifier names: expected 1/4, got 0/0), 7 ok
scrap-sweep-excludes.sh  ok on the fix; FAIL against a copy with --exclude-standard (the round-1 sweep)

# After the fix, 2026-10-05 20:25
11 self-tests, 81 cases, all ok (policy-bypasses 48, ratchet-lexing 8, workspace-policy 7, crate-checks 5, …)
$ git diff ca06400 -- scripts/quality/tests → (empty)
$ scripts/quality/full-gate.sh → RESULT: PASS (13 steps); secrets: 59 commits scanned
```

### Verification round 3 (regressions and accuracy only): verifier FAIL, and the fix

The verifier (~20:27–21:24) confirmed R1–R5 and every round-1 bypass caught, the gate green, the ratchet's counts on a 37,152-file corpus of real Rust (std, stdarch, the cargo registry) unchanged, and the policy fast (0.48 s on a member holding the whole std source). It failed the change on false positives in the new member-source check, which AGENTS.md treats as a defect:

| # | Finding | Tripwire (`309ebb6`, committed before the fix) |
|---|---|---|
| L7 | A build script setting `cargo::rustc-env=RUSTC_VERSION=…`, the usual way to embed the toolchain version, was refused: the pattern took any `RUSTC*` name as a compiler setting | `tests/policy-bypasses.sh`: must pass |
| L8 | A doc comment quoting `rustc-env=RUSTFLAGS=…` was refused | Not a test: with the narrowed names it no longer matches; a comment quoting an exact `CLIPPY_*`/`RUSTC_BOOTSTRAP` assignment is still refused (recorded below) |
| L9 | Untracked `.rs` files in a member directory were judged, so local and CI results could differ | `tests/policy-bypasses.sh`: untracked is not judged, tracked is refused |
| A1–A6 | Wording: the "every other refusal" claim (seven round-1 forms still had no committed test), the round-2 evidence heading, the round-2 intro, "TODO markers in `.rs` files" (the ratchet counts code and config), the sweep comment's secret-scan claim (not under gitleaks' built-in allowlist), and the check described as covering "build scripts" | Corrected here; `309ebb6` pins the seven forms and an unreadable config |

Fix (fix-lock engaged since `309ebb6`; tests unchanged): the check matches only names clippy or rustc read from their own environment (`CLIPPY_*`, `RUSTC_BOOTSTRAP`; `RUSTFLAGS`-style names are read by Cargo) and reads tracked files when the root is a git work tree (every file otherwise, as in the test fixtures); the flag message says "compiler flags" since it covers rustdocflags; README, sweep comment and this record corrected.

```text
# 309ebb6 tests against the code before this fix, 2026-10-05 21:25
policy-bypasses.sh  2 FAIL (RUSTC_VERSION build script refused; untracked build script judged), 57 ok

# After the fix, 2026-10-05 21:27
11 self-tests, 92 cases, all ok (policy-bypasses 59)
$ git diff 309ebb6 -- scripts/quality/tests → (empty)
$ scripts/quality/full-gate.sh → RESULT: PASS (13 steps); secrets: 60 commits scanned
$ git diff --check → clean
```

**Delta check of the round-3 fix (verifier, ~21:30): PASS.** L7–L9 fixed (scratch git fixtures); R3, the single-colon `cargo:rustc-env=CLIPPY_ARGS` form and `RUSTC_BOOTSTRAP` still refused; narrowing confirmed end to end (a build script setting `RUSTFLAGS`, `CARGO_ENCODED_RUSTFLAGS` or `RUSTC_WRAPPER` through `rustc-env` leaves clippy failing, so dropping those names loses nothing); A1–A6 fixed; 25 links resolve; scrap clean. Not yet proven: CI (fix uncommitted at the time), and the literal "2 FAIL, 57 ok" pre-fix run (the fix-lock forbids copying test files).

### Close-out

```text
$ git diff 309ebb6 5e8148a -- scripts/quality/tests → 0 lines (tests unchanged by the fix)
$ git push origin stage-00/bootstrap → ea109b0..5e8148a (PR #7, draft)
CI run 37414127507 (head 5e8148a), 2026-10-06T04:31Z:
  Full gate (Ubuntu): success — GATE tools … scrap: PASS, 62 commits scanned, RESULT: PASS (13 steps)
  macOS core: skipped (runs on main only, by design; Stage 00 gate record)
```

Plan criterion met: every regression test failed on the commit that added it (or, for pinning tests, passed on the code it pins) and passes on `5e8148a` with the test files unchanged since `309ebb6`; the strengthened redaction checks fail against a scan without `--redact`; the full gate passes locally and in CI. Changes reviewed by the `verifier` subagent over three rounds and a delta check (final verdict PASS).

## Limitations and follow-ups

- **Guard gaps (shared `guard.py`, owned by the owner's full-tier sync; not changed here).** (a) Under this session's fix-lock, a scratch `cp` into a `…/scripts/quality/tests/` path built from a loop variable was not detected; a repeat with a plain path was denied at 19:26. It wrote outside the repository and no repository test changed before the owner's release. (b) False positives: inline Python that writes only this record was denied because the record's text names test files (edits went through the Edit tool instead, which the guard checks by target); the verifier's read-only inline Python was denied because it named a third-party path containing `tests/` (it read those files with the Read tool instead).
- **Stale fix-lock:** `.claude/state/fix-locks/14531436-….json` belongs to the session that stopped at 19:17:53. It binds only that session; releasing it is the owner's (`python3 .claude/hooks/guard.py fix-lock off --session 14531436-57fa-467d-9a83-90e449ff11f2`).
- **Threat model (owner, 2026-10-05):** the gate catches accidental or careless weakening in its common forms; deliberate subversion by someone with commit access is for review and branch protection (blocked until GitHub Pro, see the Stage 00 gate record). Within that, these gaps are accepted and known:
  - The crate policy reads the Cargo config files Cargo reads from the workspace root; `cargo --config` on the command line is outside it (the gate's commands pass none). The gate trusts the toolchain on `PATH` (a planted `cargo-clippy`).
  - The `rustc-env` check is a text search over tracked member `.rs` files, comments included: a variable name assembled at run time (`concat!("CLIPPY", "_ARGS")`) is not seen, and a comment quoting an exact `CLIPPY_*` or `RUSTC_BOOTSTRAP` assignment is refused (fail-closed; rename the text). Outside a git work tree (test fixtures) it reads every file and skips directories named `target` or `.git`. A member whose manifest sets `build = "../elsewhere/build.rs"` keeps its build script outside the member directory, where the check does not look (noted by the verifier, not tested).
  - The ratchet does not see attributes a macro generates (`macro_rules!` passing `allow(…)` as a `meta`), or suppressions in non-`.rs` files compiled through `#[path]` or `include!` (R7).
  - The Scrap Sweep's "skipped tests without a reason" check matches only the literal `#[ignore]`. The ratchet counts the other forms (`cfg_attr(…, ignore)`, commented, raw-identifier), so they cannot be added without a reviewed baseline change.
  - gitleaks' built-in path allowlist stays (R6). Replacing it means maintaining a copy of the default ruleset; reconsider if such paths ever appear in this repository.
- The "Owed tests" item from round 1 is closed: `ca06400` pins the path-patch refusals and the refusals named in round 2; `309ebb6` pins the seven round-1 forms it missed and an unreadable config. Still unpinned, by choice: "no workspace members found" (Cargo itself refuses most such workspaces first).
- Developers using sccache must unset `RUSTC_WRAPPER` for gate runs (README).
