# Build Record — PR #7 review defects: merge-commit secrets, lint opt-in, ID deserialization

| Field | Value |
|---|---|
| Change | Fix the defects the independent review of PR #7 and the start of the ledger found in the Stage 00 gate and in `rikuri-domain`, each with a regression test committed before the fix |
| Date | 2026-10-05 |
| Author / agent session | Claude Code session for Henry Chen |
| Branch | `stage-00/bootstrap` |
| Commits | fill at close-out |
| Stage Gate | [Stage 00](../stage-gates/foundation0/00-bootstrap-kernel.md), Bootstrap gate and S00.04 |
| Related ADR / requirement | S00.02, S00.03, S00.04, T00.02, T00.11; ADR 0001; [bootstrap record](2026-10-05-s00-bootstrap-ci-quality-gates.md); [domain record](2026-10-05-s00-04-rikuri-domain.md) |
| Status | IN PROGRESS |

**Risk note (protected paths):** changes `scripts/quality/*` and `.gitleaks.toml`, the gate itself. Two of the defects let the required gate pass work it must block.

## 1. Plan

- [x] One-sentence statement: close every gap the review reproduced, so the gate refuses what its records say it refuses, and make `AnyId` readable from stored data.
- [x] Blast radius: gate scripts, gitleaks configuration, `rikuri-domain` ID parsing. No canonical data exists.
- [x] Current behavior characterized: by the verifier's reproductions (PR #7 report) and by the failing tests below.
- [x] **I will know this works when** every regression test below fails on the commit that adds it (before the fix) and passes after it with no change to the test files (`git diff <tests commit> <fix commit> -- scripts/quality/tests crates/rikuri-domain/src/id.rs` shows only non-test lines), the full gate passes locally and in CI, and the previously demonstrated gate failures still fail.
- [x] Architecture impact: gate behavior only; ADR 0001 unchanged (parsing is stricter in what it echoes, not in what it accepts).
- [x] Existing solution review: N/A; fixes to existing scripts.

## 3. Debug

| # | Defect (found by) | Reproduction | Root-cause class | Tripwire (committed before the fix) |
|---|---|---|---|---|
| 1 | Secret written in a merge commit passes the scan (verifier) | Throwaway repo, key added while resolving a conflict: key in HEAD's tree, `secret-scan.sh` exit 0 | Scan reads `git log -p`, which shows no diff for merge commits unless `-m` is given; the scope fix narrowed refs but kept this blind spot | `scripts/quality/tests/secret-scan-merge.sh`: `MERGE FAIL: a key introduced in a merge commit passed the scan` |
| 2 | Scan prints no finding at all (verifier) | Same fixture: output is only `leaks found: 1` | No `--verbose`; the T00.02 acceptance line says "shows the finding redacted" | Same test: `MERGE FAIL: the finding is not shown (expected a REDACTED finding)` |
| 3 | A crate without `[lints] workspace = true` escapes the lint policy (verifier) | Crate using `unsafe`, `unwrap()`, `println!` passed all 12 steps | Cargo applies workspace lints only to crates that opt in; nothing checked the opt-in | `scripts/quality/tests/crate-checks.sh`: no `crate-policy.sh` exists |
| 4 | `## Healthiness` counts as `## Health` (verifier) | Contract check matched heading prefixes | `grep "^## Health"` without an end anchor | Same test: `CRATES FAIL: '## Healthiness' does not count as '## Health'` |
| 5 | Ratchet misses `#[cfg_attr(…, allow(…))]` (verifier) | Suppression regex only matched `#[allow(` / `#![allow(` | Regex too literal | `scripts/quality/tests/ratchet-and-paths.sh`: `PATHS FAIL: a cfg_attr allow() suppression did not raise the ratchet` |
| 6 | `target/` allowlisted at any depth (verifier) | Key in `docs/target/notes.txt` passed | Allowlist `(^\|/)target/` instead of the top-level build directory | Same test: `PATHS FAIL: a key under docs/target/ passed the scan` |
| 7 | `AnyId` deserializes only from `'static` text (builder, writing the ledger) | Reading a ledger row: `error[E0597]: … does not live long enough` | `AnyId` holds its prefix as `&'static str`, and serde's derive then demands `'de: 'static` | `rikuri-domain` `id::tests::any_id_deserializes_from_runtime_text` (compile failure, E0597) |
| 8 | Rejected IDs echo unbounded input (verifier) | 1 MiB input → 1 MiB error | Errors copied the whole untrusted string | `id::tests::errors_do_not_echo_unbounded_input`: `error is 1048616 bytes` |

Not fixed here, recorded: the ratchet's `|| true` would count a `git grep` error as zero (no test; fixed by checking `git grep`'s exit status, 1 = no match, ≥2 = failure).

Sibling sweep: other scans of history (`--all` audits in the kickoff record) used `-m`; the pre-commit hook scans the staged diff, which includes conflict resolutions, so it is not affected by #1. Other places that parse untrusted text into errors: `UnknownVariant` (closed enums; same bounded echo applied), `ProvenanceError` (enum values only).

## Evidence log

```text
# Failing before the fix (tests written, nothing fixed yet)
$ scripts/quality/tests/secret-scan-merge.sh
MERGE FAIL: a key introduced in a merge commit passed the scan
MERGE ok: the key is not printed
MERGE FAIL: the finding is not shown (expected a REDACTED finding)
exit=1
$ scripts/quality/tests/crate-checks.sh
CRATES FAIL: a crate that opts into workspace lints is accepted (expected pass, got fail)
  …/scripts/quality/crate-policy.sh: No such file or directory
CRATES FAIL: '## Healthiness' does not count as '## Health' (expected fail, got pass)
exit=1
$ scripts/quality/tests/ratchet-and-paths.sh
PATHS FAIL: a cfg_attr allow() suppression did not raise the ratchet
PATHS FAIL: a key under docs/target/ passed the scan
exit=1
$ cargo test -p rikuri-domain --lib id::
error[E0597]: `text` does not live long enough
error[E0597]: `bad` does not live long enough
$ (scratch copy without the E0597 test) cargo test -p rikuri-domain --lib errors_do_not_echo
id::tests::errors_do_not_echo_unbounded_input --- FAILED
error is 1048616 bytes
```
