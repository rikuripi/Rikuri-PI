# Build Record — PR #7 review defects: merge-commit secrets, lint opt-in, ID deserialization

| Field | Value |
|---|---|
| Change | Fix the defects the independent review of PR #7 and the start of the ledger found in the Stage 00 gate and in `rikuri-domain`, each with a regression test committed before the fix |
| Date | 2026-10-05 |
| Author / agent session | Claude Code session for Henry Chen |
| Branch | `stage-00/bootstrap` |
| Commits | `9b91f0e` (regression tests, red), `90fee2c` (fix), `ea109b0` (fix-lock release, S00.08 resequencing, guard gap) |
| Stage Gate | [Stage 00](../stage-gates/foundation0/00-bootstrap-kernel.md), Bootstrap gate and S00.04 |
| Related ADR / requirement | S00.02, S00.03, S00.04, T00.02, T00.11; ADR 0001; [bootstrap record](2026-10-05-s00-bootstrap-ci-quality-gates.md); [domain record](2026-10-05-s00-04-rikuri-domain.md) |
| Status | COMPLETE (closed out 2026-10-06; re-verification below) |

**Risk note (protected paths):** changes `scripts/quality/*` and `.gitleaks.toml`, the gate itself. Two of the defects let the required gate pass work it must block. `ea109b0` also edits the Stage Plan (S00.08 item wording, after the owner chose the resequencing) and the Stage 00 gate record, both protected; those edits record owner decisions and change no acceptance criterion.

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

## 2. Build (fix, under the fix-lock engaged 2026-10-05 after commit 9b91f0e)

- `scripts/quality/secret-scan.sh`: `--log-opts="--full-history -m HEAD"` (merge diffs scanned) and `--verbose` (each finding printed with its value redacted). The pre-commit hook also prints findings (`--verbose`).
- `.gitleaks.toml`: allowlist only `^target/`.
- `scripts/quality/crate-policy.sh` (new, full-gate step `crate-policy`): every `crates/*` and `apps/*` manifest must have `[lints]` exactly `workspace = true`, parsed with Python's `tomllib`.
- `scripts/quality/module-contracts.sh`: takes an optional root; a heading counts when its name ends at whitespace or the line end, so the template's `## Health / last-success signal` counts and `## Healthiness` does not. (A first attempt required the exact line and failed the real contract in the full gate; corrected before commit.)
- `scripts/quality/ratchet.sh`: takes an optional root; counts `#[cfg_attr(…, allow(…))]`; a `git grep` failure (exit ≥ 2) stops the ratchet instead of counting as zero.
- `rikuri-domain`: `AnyId` has a hand-written `Deserialize` that accepts borrowed or owned text; rejected input in `IdError` and `UnknownVariant` is escaped and cut at 64 bytes with its length noted (`wire::bounded`).
- README step list updated (13 steps).

## 4. Verify

- [x] Every regression test failed before the fix (evidence below) and passes after it.
- [x] No test changed during the fix: `git diff --stat 9b91f0e -- scripts/quality/tests` is empty; the `#[cfg(test)]` regions of `id.rs` and `wire.rs` are byte-identical to 9b91f0e.
- [x] Full gate: `RESULT: PASS (13 steps)` locally.
- [x] CI on the fix commit: run 37401144525 at 90fee2c, `RESULT: PASS (13 steps)`, 55 commits scanned with merge diffs.
- [x] Independent re-verification of PR #7. The second independent review re-checked these fixes and found further defects in the same classes; they were fixed tests-first under the [second review-fixes record](2026-10-05-s00-gate-review-fixes-2.md). The whole of PR #7 was then verified at head `e42901a` on 2026-10-06: PASS, no merge blockers (`verifier` subagent; follow-ups in the [Stage 00 record](../stage-gates/foundation0/00-bootstrap-kernel.md)).

## Fix-lock and owner decisions (2026-10-05)

- Lock engaged 18:44:33 −0700 after the red-test commit 9b91f0e; released 18:55:41 −0700. Approval: the owner answered "Release it now (Recommended)" in the session, after being told the tests were unchanged in 90fee2c and the local gate was green. The guard's `development` profile allows and logs the release without asking, so that answer, recorded here, is the approval.
- **Guard gap, recorded (owner choice: no guard change now).** The fix-lock checks paths inside the project directory only, so test files in a git worktree elsewhere (this session used one in its scratchpad for kernel work) are not protected while a lock is held. Working rule from now on: no test edits in any worktree while a fix-lock is held; the kernel worktree was idle during this lock. The guard is shared unchanged across the owner's full-tier repositories; a fix is the owner's later decision.
- **S00.08 resequencing (owner choice).** `FakeModelAdapter` moves to S02.07 and `FakeTool` to S04.01, each built with the interface it fakes; Stage 00's test kit is `FixtureIds`, `FixtureClock` and `CrashInjector`. Recorded in the [Stage 00 record](../stage-gates/foundation0/00-bootstrap-kernel.md) and on the Stage Plan items.

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

# After the fix (test files unchanged since 9b91f0e)
$ for t in scripts/quality/tests/*.sh; do $t; done
CRATES ok ×5 (opt-in accepted; missing [lints] refused; lint override refused; exact headings accepted; "## Healthiness" refused)
PATHS ok: a cfg_attr allow() suppression raises the ratchet
PATHS ok: a key under docs/target/ fails the scan
MERGE ok: a key introduced in a merge commit fails the scan
MERGE ok: the key is not printed
MERGE ok: the finding is shown, redacted
SCOPE ok ×2
all exit 0
$ cargo test -p rikuri-domain → 24 unit + 4 property passed
$ scripts/quality/full-gate.sh (first attempt) → GATE contracts: FAIL (exact-line heading rule rejected the template heading) — corrected
$ scripts/quality/full-gate.sh → RESULT: PASS (13 steps); secrets: 53 commits scanned (merge diffs included)
$ git diff --stat 9b91f0e -- scripts/quality/tests → (empty)
$ diff of the #[cfg(test)] regions of id.rs and wire.rs against 9b91f0e → identical
```
