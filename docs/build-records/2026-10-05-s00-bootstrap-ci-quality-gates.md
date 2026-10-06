# Build Record — S00.01–S00.03 bootstrap: workspace, CI and quality gates

| Field | Value |
|---|---|
| Change | Rust workspace with pinned toolchain and lockfile, dependency policy, secret-scan configuration, pre-commit hook, full-gate/ratchet/Module-Contract/Scrap-Sweep scripts, CI workflow, setup documentation, and the red-test and secret drills |
| Date | 2026-10-05 |
| Author / agent session | Claude Code session for Henry Chen |
| Branch | `stage-00/bootstrap` |
| Commits | fill at close-out |
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
- [x] `scripts/quality/full-gate.sh` (11 steps, stops at the first failure, prints `RESULT: PASS|FAIL (<step>)`), `ratchet.sh` + `ratchet-baseline.json` (ignored tests, lint suppressions, TODO-class markers; rises fail), `module-contracts.sh` (T00.11), `scrap-sweep.sh` (fails on tracked backup/editor files, `dbg!`, reasonless `#[ignore]`, conflict markers; lists untracked files and stashes), `hooks/pre-commit` (staged secret scan; fmt and `cargo check` when Rust changes).
- [x] `.github/workflows/ci.yml`: "Full gate (Ubuntu)" on PRs and pushes to `main`; "macOS core (build · unit · contract)" on pushes to `main` only (owner decision 2026-10-05). `permissions: contents: read`; checkout without persisted credentials.
- [x] README > Running the full gate; AGENTS.md repository-maturity paragraphs updated.
- [ ] `scripts/quality/smoke.sh` — see Deferred: it needs `rikurid`.
- [x] Security question: touches no auth/money/execution/private-data path. The gate itself protects against secrets entering history (allowed: ordinary commits pass; denied: planted key fails, T00.02).

## 3. Debug — N/A

## 4. Verify

- [x] Unit tests: the gate runs the workspace tests (rikuri-domain: 22 unit + 4 property).
- [ ] Shared contract suite — N/A for tooling.
- [ ] Running-system probe — the "running system" for this change is CI itself: see the CI runs in the evidence log.
- [ ] T00.01 red-test drill — evidence log.
- [ ] T00.02 secret drill — evidence log.
- [ ] T00.03 clean checkout from README — evidence log.
- [ ] T00.11 contract check fails on a broken contract — evidence log.
- [ ] Regression guard: each drill above is a deliberate known violation the gate must catch.

## 5. Close Out

- [ ] Full required gate green locally and in CI.
- [ ] Scrap Sweep complete; deleted artifacts listed.
- [x] No unexplained skipped tests or new unowned debt (baseline 0/0/0).
- [x] Documentation updated in the same change.
- [ ] Stage Gate Record updated with this change/evidence.
- [ ] Commit message explains WHY; Build Record committed with change.

## Skipped / Deferred / Resequenced

| Item | Disposition | Reason | Dependency impact | Owner | New stage/ordinal | Reactivation / acceptance condition |
|---|---|---|---|---|---|---|
| `scripts/quality/smoke.sh` (part of S00.03) | RESEQUENCED within Stage 00 | Smoke must check a content marker in `state.show` (T00.10), which needs `rikurid`; a smoke script with nothing to probe would only check liveness | S00.03 stays open until it lands; T00.10 unchanged | Stage 00 implementer | With S00.07 | `smoke.sh` starts `rikurid`, runs `state.show`, and fails without the expected content marker |
| Branch protection on `main` (part of S00.01; T00.01 "blocks") | BLOCKED on owner action | GitHub Free refuses protection on this private repository; the owner chose GitHub Pro (kickoff Build Record) and holds admin | Bootstrap gate and T00.01's blocking half stay Not yet proven; kernel code does not merge to `main` until protection exists | Henry Chen | Stage 00, before kernel code merges | Protection on `main` requires the "Full gate (Ubuntu)" check and a PR, disallows force-push and deletion; a red drill PR reports not mergeable |

## Evidence log

```text
filled as runs complete
```
