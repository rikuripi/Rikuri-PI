# Build Record — S00.01–S00.03 bootstrap: workspace, CI and quality gates

| Field | Value |
|---|---|
| Change | Rust workspace with pinned toolchain and lockfile, dependency policy, secret-scan configuration, pre-commit hook, full-gate/ratchet/Module-Contract/Scrap-Sweep scripts, CI workflow, setup documentation, and the red-test and secret drills |
| Date | 2026-10-05 |
| Author / agent session | Claude Code session for Henry Chen |
| Branch | `stage-00/bootstrap` |
| Commits | 586d878 (workspace + first crate), f56532b (gates), 62272c4 (secret-scan scope fix), 5908a8c (drill evidence); merges 56d3d95 and 7b80a59 bring in the kickoff branch and `main`. Review defects: [separate record](2026-10-05-s00-gate-review-fixes.md) |
| Stage Gate | [Stage 00](../stage-gates/foundation0/00-bootstrap-kernel.md), Bootstrap gate |
| Related ADR / requirement | S00.01, S00.02, S00.03, T00.01, T00.02, T00.03, T00.11; Engineering Standard Phase 0; Test Strategy §3.3 |
| Status | IN PROGRESS |

**Risk note (protected paths, REVIEW.md):** this change creates `deny.toml`, `.gitleaks.toml`, `scripts/quality/*` and `.github/workflows/ci.yml`, and edits `AGENTS.md`, all protected. These files *are* the gate: a weak gate passes bad work silently. The risks are a step that cannot fail (mitigated by the drills below), a tool version that drifts between local and CI (mitigated by pinned versions checked by the gate itself), and an allowlist that hides a real secret (none added beyond `target/`).

## 1. Plan

- [x] One-sentence statement: make one script, `scripts/quality/full-gate.sh`, the repository's required gate, run it identically locally and in CI, and prove it blocks a failing test and a planted secret.
- [x] Blast radius: repository tooling and CI only. No canonical data, no auth/money/execution path. Branch protection on `main` (a GitHub setting) is the owner's action.
- [x] Current behavior characterized: `main` at 95eaaac has only the guard workflow (`.github/workflows/claude-guard.yml`, path-filtered to `.claude/**`); no Rust, no CI for code, no branch protection (`GET …/branches/main/protection` → 404; rulesets → 403 "Upgrade to GitHub Pro"). An empty Cargo workspace does not build (checked in scratch: missing member directories are an error), so the bootstrap includes its first real crate (`rikuri-domain`, separate Build Record).
- [x] Verification defined up front — **I will know this works when:**
  1. the full gate passes locally and in CI on the same commit, and each step can fail (each runs `set -e` with no `continue-on-error`);
  2. **T00.01:** a PR with a planted failing test turns CI red; with the test removed it turns green; once branch protection exists, GitHub reports the red PR as not mergeable;
  3. **T00.02:** a PR with a planted fake private key turns CI red, and the CI log shows the finding redacted, with none of the key's characters;
  4. **T00.03 (Ubuntu half):** a fresh clone in a scratch directory, following only README > Running the full gate, ends with `RESULT: PASS`; the macOS half is the CI macOS job's first green run on `main`;
  5. **T00.11:** the Module Contract check fails when a crate's contract is missing a required section.
- [x] Architecture-impact check: invariant/ADR — Engineering Standard Phase 0 implemented, no ADR change; API/schema — N/A; documentation — README, AGENTS.md repository-maturity text; others N/A.
- [x] Relevant threat/control/test: secrets committed to history → pre-commit and CI gitleaks scans (T00.02); vulnerable or unlicensed dependency → cargo-deny; a gate that silently passes → drills; debt creeping in → ratchet.
- [x] Existing solution review: gitleaks 8.30.1 (MIT) and cargo-deny 0.20.2 (MIT/Apache-2.0) are the Stage Plan's adopt choices, installed from release archives verified against published SHA-256 digests. GitHub Actions pinned by commit SHA: `actions/checkout` v7.0.1 (3d3c42e5…), `Swatinem/rust-cache` v2.9.2 (6323deb1…). A plain git hook under `scripts/quality/hooks/` instead of the `pre-commit` framework, which is not installed on the reference machine and would add a Python tool for two commands; the hook directory is already covered by the guard's `scripts/quality/*` protection. The scripts follow patterns proven in the owner's rikurinode repository (pinned checksum-verified gitleaks, pinned toolchain); they stay small scripts, not a quality product (Stage Plan deferral row "Checklist / Build Record / traceability validators").
- [x] Authoritative state change? No.

## 2. Build

- [x] Workspace: `Cargo.toml` (resolver 3, edition 2024, `publish = false`, workspace lints: `unsafe_code` forbidden; `dbg_macro`, `todo`, `unimplemented`, `unwrap_used`, `expect_used`, `print_stdout`, `print_stderr` denied; `clippy.toml` allows unwrap/expect in tests only), `Cargo.lock` committed, `rust-toolchain.toml` pinned to 1.98.1 with rustfmt and clippy.
- [x] `deny.toml`: RustSec vulnerabilities, yanked, unmaintained and unsound crates fail; permissive license allowlist; crates.io only; no wildcard versions.
- [x] `.gitleaks.toml`: default ruleset, `target/` excluded, nothing else allowlisted.
- [x] `scripts/quality/full-gate.sh` (12 steps after the Debug fix, stops at the first failure, prints `RESULT: PASS|FAIL (<step>)`), `secret-scan.sh` (HEAD-history scan shared with its self-test), `tests/secret-scan-scope.sh` (the gate's regression test), `ratchet.sh` + `ratchet-baseline.json` (ignored tests, lint suppressions, TODO-class markers; rises fail), `module-contracts.sh` (T00.11), `scrap-sweep.sh` (fails on tracked backup/editor files, `dbg!`, reasonless `#[ignore]`, conflict markers; lists untracked files and stashes), `hooks/pre-commit` (staged secret scan; fmt and `cargo check` when Rust changes).
- [x] `.github/workflows/ci.yml`: "Full gate (Ubuntu)" on PRs and pushes to `main`; "macOS core (build · unit · contract)" on pushes to `main` only (owner decision 2026-10-05). `permissions: contents: read`; checkout without persisted credentials.
- [x] README > Running the full gate; AGENTS.md repository-maturity paragraphs updated.
- [ ] `scripts/quality/smoke.sh` — see Deferred: it needs `rikurid`.
- [x] Security question: touches no auth/money/execution/private-data path. The gate itself protects against secrets entering history (allowed: ordinary commits pass; denied: planted key fails, T00.02).

## 3. Debug — secret scan read every ref, not the change under test

- [x] Problem reproduced before fix. T00.01's "removed → green" run (37397462829, PR #8 at 31078aa) went red at `GATE secrets` ("leaks found: 1") although that branch never held a key. Its checkout (`fetch-depth: 0`) fetched every branch, including `drill/t00-02-secret`, and scanned 40 commits where PR #7's earlier run scanned 38. Reproduced locally on `stage-00/bootstrap`: `gitleaks git .` scanned 41 commits (remote-tracking refs included) and found 1 leak; with `--log-opts="--full-history HEAD"`, 39 commits and none.
- [x] Root-cause **class**: a gate whose input is ambient state (every ref the clone happens to hold) instead of the change under test. Without `--log-opts`, gitleaks runs `git log --all`. Effects: an unrelated PR fails because of another branch, and local results depend on which remote branches were fetched.
- [x] Sibling cases swept: the pre-commit hook scans only the staged diff; the ratchet and Scrap Sweep read tracked files (`git grep`, `git ls-files`); the guard tests are self-contained; the Module Contract check reads crate directories, which in CI are a clean checkout and locally are reported by the Scrap Sweep if untracked. No other instance. The kickoff record's `--all -m` scan was a deliberate whole-repository audit, not a gate.
- [x] Tripwire, written and seen failing before the fix: `scripts/quality/tests/secret-scan-scope.sh` builds a throwaway repository with a clean `main` and a side branch holding a run-time-generated fake key. It requires a clean HEAD to pass while the other branch holds the key, and the side branch to fail. Against the old scan it printed `SCOPE FAIL: clean HEAD failed because another branch holds a secret` (exit 1). The fix-lock was then engaged on this record (2026-10-05 18:09 −0700). The fix changed only gate code: `scripts/quality/secret-scan.sh` (one shared scan command for the gate and its test) gained `--log-opts="--full-history HEAD"`, and the full gate runs `scripts/quality/tests/*.sh` as its first step after tool checks.
- [x] Fix-lock release, 2026-10-05 18:24 −0700. Approval: the owner answered "Release the lock (Recommended)" to the session's question, which named the change (only how the test builds its fake key; assertions unchanged). That approval lives in the conversation, not in a file. The guard itself did not ask: under the `development` profile it allows and logs this action (`.claude/state/guard-relaxed.log`, 18:24:07), and the release records this session as `released_by`. Limitation found by review: the test was first committed only after the release, so git cannot show that only the fixture line changed; `guard-relaxed.log` shows a single Edit to it at 18:24:13, consistent with the claim but not proof. Later defect batches commit their failing tests before the fix ([review-fixes record](2026-10-05-s00-gate-review-fixes.md)). Committing the fix was then refused by the pre-commit secret scan, correctly: the test's source contained a literal PEM header and footer. With the owner's approval the lock was released and only the fixture line changed, so the source builds the armor string at run time (`"RSA " + "PRIVATE KEY"`). The two SCOPE assertions are unchanged. Rechecked: against the fixed scan it passes; against the old scan (the `--log-opts` line removed, in a scratch copy) it still prints `SCOPE FAIL: clean HEAD failed because another branch holds a secret`, exit 1; the staged source scans clean.
- [x] Remediation justified by evidence: the CI log's fetched-branch list and commit count, plus the local reproduction, point to scan scope. The fix narrows only what is scanned to the commit under test. Secrets on unmerged branches pushed to GitHub are still exposures; a whole-repository audit scan (all refs) is a separate periodic check, not this per-change gate.

## 4. Verify

- [x] Unit tests: the gate runs the workspace tests (rikuri-domain: 22 unit + 4 property).
- [ ] Shared contract suite — N/A for tooling.
- [ ] Running-system probe — the "running system" for this change is CI itself: see the CI runs in the evidence log.
- [x] T00.01 red-test drill — red Pass, removed → green Pass (evidence log). "Blocks" Pass (2026-10-06, after the owner enabled protection): drill PR #10's red run left GitHub reporting it BLOCKED, and the same PR with the test removed reported CLEAN; closed unmerged (evidence log).
- [x] T00.02 secret drill — Pass for the Stage Plan wording: the planted key blocks CI (run 37397273842 failed) and the 534-line log holds none of the key (evidence log). Criterion 3's "shows the finding redacted" was **not** shown by that run, which predates `--verbose` and printed no finding at all; the redacted finding is proven by the gate self-tests (`REDACT ok`, `MERGE ok`, green in CI run 37549907918), not by re-running the drill (corrected 2026-10-06 after the pre-merge verification).
- [x] T00.03 clean checkout from README (Ubuntu half) — Pass at f56532b (11 steps, evidence log) and, by the independent verifier, at 5908a8c (12 steps). The builder's own rerun at 5908a8c is void: its harness rewrote `/tmp/` paths but missed `-C /tmp`, so `install` looked in the wrong place; the README was not at fault, but its fixed `/tmp` paths are fragile (follow-up: use a `mktemp -d` directory). macOS half: Pass, 2026-10-07 — the first green macOS run on `main` (run 37567919632, on the owner's Mac; [runner record](2026-10-06-macos-self-hosted-runner.md)).
- [x] T00.11 contract check fails on a broken contract — Pass (evidence log).
- [x] Regression guard: every gate class was shown failing on a known violation (evidence log), and the scope defect has its own self-test.

## 5. Close Out

- [x] Full required gate green locally and in CI at 5908a8c: local `RESULT: PASS (12 steps)`; PR #7's own run 37399244521 `RESULT: PASS (12 steps)`, 42 commits scanned. PR #7 also had a red run, 37397893651 at 56d3d95 (`RESULT: FAIL (secrets)`, 41 commits scanned): the same scope defect as §3, before its fix. The review defects that followed are fixed under their own record.
- [x] Scrap Sweep complete; deleted artifacts listed: drill PRs #8/#9 closed unmerged; local drill branches deleted; `/tmp` residue of the T00.03 run removed by name; scratch clones live only in the session scratchpad. Remote branches `drill/t00-01-red-test` and `drill/t00-02-secret` remain because the guard reserves deleting remote branches for the owner (owner action). The planted block also stays reachable through GitHub's `refs/pull/9/head`; it is random bytes, not a key.
- [x] No unexplained skipped tests or new unowned debt (baseline 0/0/0).
- [x] Documentation updated in the same change.
- [x] Stage Gate Record updated with this change/evidence.
- [x] Commit messages explain why; this record is committed with the change.

## Skipped / Deferred / Resequenced

| Item | Disposition | Reason | Dependency impact | Owner | New stage/ordinal | Reactivation / acceptance condition |
|---|---|---|---|---|---|---|
| `scripts/quality/smoke.sh` (part of S00.03) | RESEQUENCED within Stage 00 | Smoke must check a content marker in `state.show` (T00.10), which needs `rikurid`; a smoke script with nothing to probe would only check liveness | S00.03 stays open until it lands; T00.10 unchanged | Stage 00 implementer | With S00.07 | `smoke.sh` starts `rikurid`, runs `state.show`, and fails without the expected content marker |
| Branch protection on `main` (part of S00.01; T00.01 "blocks") | RESOLVED 2026-10-06: the owner upgraded `rikuripi` to GitHub Pro and created ruleset 24616501 "Protect main"; acceptance condition met (evidence log, T00.01 blocks). Was: BLOCKED on owner action | GitHub Free refuses protection on this private repository; the owner chose GitHub Pro (kickoff Build Record) and holds admin | Bootstrap gate and T00.01's blocking half stay Not yet proven; kernel code does not merge to `main` until protection exists | Henry Chen | Stage 00, before kernel code merges | Protection on `main` requires the "Full gate (Ubuntu)" check and a PR, disallows force-push and deletion; a red drill PR reports not mergeable |

## Evidence log

```text
# Local full gate, before the Debug fix (f56532b)
$ scripts/quality/full-gate.sh            → RESULT: PASS (11 steps)

# CI, PR #7 (stage-00/bootstrap at f56532b), run 37397124138, 2026-10-06 01:02–01:03Z
Full gate (Ubuntu): success — all 11 GATE steps PASS, RESULT: PASS (11 steps)
macOS core: skipped (pull_request; runs on pushes to main by design)

# Pre-commit hook on real commits
586d878: staged ~70.9 KB scanned, no leaks; fmt + check passed
f56532b: staged ~23.6 KB scanned, no leaks

# T00.01 red half — PR #8 (drill/t00-01-red-test at 3da7d98), run 37397240985
── GATE clippy: PASS
test t00_01_red_test_drill_must_fail ... FAILED
assertion `left == right` failed: T00.01 red-test drill: CI must go red
── GATE test: FAIL
RESULT: FAIL (test)                       → conclusion: failure        Pass (red as required)

# T00.01 green half, first attempt — PR #8 at 31078aa (test removed), run 37397462829
── GATE test: PASS ... ── GATE secrets: FAIL, "leaks found: 1", 40 commits scanned
RESULT: FAIL (secrets)                    → defect, see §3 Debug

# T00.02 — PR #9 (drill/t00-02-secret at 8e13ee9), run 37397273842
local commit without --no-verify: hook "leaks found: 1", commit refused (exit 1)
committed with --no-verify (deliberate bypass), pushed
── GATE test: PASS ── GATE dependencies: PASS
WRN leaks found: 1
── GATE secrets: FAIL
RESULT: FAIL (secrets)                    → conclusion: failure        Pass (red as required)
CI log 534 lines; 3 key-body canary fragments (24 chars each) found in log: 0
positive control: same 3 fragments found in the planted file: 3; PEM header lines in log: 0
                                                                     Pass (key not printed)

# T00.03 Ubuntu half — fresh clone of stage-00/bootstrap, empty HOME (env -i), README only
step 1: rustup.rs installer, no default toolchain
step 2: README commands verbatim → /tmp/gitleaks.tgz: OK, /tmp/cargo-deny.tgz: OK
step 4: rustc 1.98.1 (48a229cea 2026-09-01) installed from rust-toolchain.toml
RESULT: PASS (11 steps), exit=0                                      Pass
(the README commands' /tmp residue from this run was removed by name afterwards)

# T00.11 and gate-class negative controls (scratch clone; nothing committed)
module-contracts: intact exit 0; "## Must not own" renamed exit 1; README missing exit 1
format: extra misformatted fn → exit 1 (Diff in …/lib.rs)
clippy: unwrap() in library code → exit 101 (used `unwrap()` on an `Option` value)
cargo deny: ulid = "*" → exit 2 (error[wildcard])
ratchet: one #[ignore] test added → exit 1 (RATCHET ignored_tests: 1 > baseline 0)
scrap: tracked notes.bak + reasonless #[ignore] → exit 1 (both reported)

# Debug: scope defect reproduced, tripwire failing first, fix verified
$ gitleaks git . (old scope, local)                  → 41 commits scanned, leaks found: 1
$ scripts/quality/tests/secret-scan-scope.sh (old)   → SCOPE FAIL: clean HEAD failed because another branch holds a secret; exit=1
$ guard.py fix-lock on <this record>                 → ENGAGED 2026-10-05T18:09:25-0700
(fix: secret-scan.sh --log-opts="--full-history HEAD"; full gate runs gate self-tests)
$ scripts/quality/tests/secret-scan-scope.sh (fixed) → SCOPE ok ×2; exit=0
$ scripts/quality/secret-scan.sh                     → 39 commits scanned, no leaks found
$ scripts/quality/secret-scan.sh <worktree of origin/drill/t00-02-secret> → leaks found: 1 (still caught)
$ scripts/quality/full-gate.sh                       → RESULT: PASS (12 steps)
$ guard.py fix-lock off (owner approved in conversation) → released 2026-10-05T18:24:08-0700
(test fixture only: PEM armor built at run time; assertions unchanged)
test vs fixed scan → SCOPE ok ×2, exit 0; vs old scan (scratch copy) → SCOPE FAIL …, exit 1
staged source of the test → gitleaks: no leaks found

# T00.01 green half — PR #8 at 811581c (test removed + fixed scan), run 37399044114
(drill/t00-02-secret, with its planted key, still on GitHub during this run)
── GATE gate-selftests: PASS (SCOPE ok ×2)
── GATE test: PASS
41 commits scanned → ── GATE secrets: PASS
RESULT: PASS (12 steps)                   → conclusion: success        Pass (green as required)
```

### T00.01 "blocks" (2026-10-06, after the owner enabled protection)

The owner upgraded `rikuripi` to GitHub Pro and created ruleset 24616501 "Protect main" (2026-10-06 16:45), replacing ruleset 24550762, which targeted no branch. Owner choices: branches must be up to date before merging; no required approvals (the kickoff decision that the agent merges once CI and the `verifier` are green stands); no bypass. The guard-policy workflow is not a required check because it is path-filtered (the full gate runs the same guard tests); macOS stays post-merge (kickoff decision).

```text
$ gh api repos/rikuripi/Rikuri-PI/rules/branches/main      (ruleset 24616501, enforcement active, target ~DEFAULT_BRANCH)
deletion · non_fast_forward · pull_request (required_approving_review_count 0)
required_status_checks: "Full gate (Ubuntu)" (integration_id 15368, GitHub Actions), strict_required_status_checks_policy true
current_user_can_bypass (agent account hchengit): never

# Red: drill PR #10 (drill/t00-01-blocks at 665e316: stage-00/bootstrap + crates/rikuri-domain/tests/red_drill.rs), run 37548496990
test t00_01_blocks_drill_must_fail ... FAILED
── GATE test: FAIL → RESULT: FAIL (test)
$ gh pr view 10 --json mergeable,mergeStateStatus,reviewDecision
mergeable MERGEABLE (no conflicts) · reviewDecision "" (none required) · mergeStateStatus BLOCKED     Pass (red PR cannot merge)

# Control: PR #10 at 1c424eb (planted test removed), run 37548667310
RESULT: PASS (13 steps) → mergeStateStatus CLEAN     (the block came from the red required check alone)

$ gh pr close 10 --delete-branch    → closed unmerged (mergedAt null); remote branch deleted; origin/main unchanged at e1c6d11
```

No merge or push to `main` was attempted: GitHub's own merge state is the evidence, and a failed refusal would have put a failing test on `main`. The pre-commit hook also refused the first drill commit for formatting (`pre-commit: formatting differs`), so the drill failed at the test step as intended.
