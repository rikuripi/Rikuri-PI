# Build Record — PR #7 re-verification defects: lint escapes, wrapped suppressions, ignored files

| Field | Value |
|---|---|
| Change | Close the gate gaps the second review of PR #7 reproduced: lint policy for every workspace member, build flags that cap lints, rustfmt-wrapped suppressions, force-added ignored files, and a redaction assertion that could not fail |
| Date | 2026-10-05 |
| Author / agent session | Claude Code session for Henry Chen |
| Branch | `stage-00/bootstrap` |
| Commits | fill at close-out |
| Stage Gate | [Stage 00](../stage-gates/foundation0/00-bootstrap-kernel.md), Bootstrap gate |
| Related ADR / requirement | S00.02, S00.03, T00.02, T00.11; [first review-fixes record](2026-10-05-s00-gate-review-fixes.md) |
| Status | IN PROGRESS |

**Risk note (protected paths):** changes `scripts/quality/*`; F1 and F2 let the required gate pass code that breaks the workspace lint policy.

## 1. Plan

- [x] One-sentence statement: make the crate policy, contract check, ratchet and Scrap Sweep judge everything their records claim, and replace a redaction assertion that could not fail.
- [x] Blast radius: gate scripts and README; no product code.
- [x] Current behavior characterized: the verifier's reproductions (second PR #7 report) and the failing tests below.
- [x] **I will know this works when** every new regression test fails on the commit that adds it and passes after the fix with the test files unchanged (`git diff <tests commit> <fix commit> -- scripts/quality/tests` empty), the strengthened redaction test fails against a scan without `--redact`, and the full gate passes locally and in CI.
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
