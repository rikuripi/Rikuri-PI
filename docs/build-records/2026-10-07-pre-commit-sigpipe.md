# Build Record — The pre-commit hook must not skip its checks on a large staged change

| Field | Value |
|---|---|
| Change | `scripts/quality/hooks/pre-commit` decides whether Rust files are staged without a pipe into `grep -q`, so a large staged change can no longer make it skip `cargo fmt --check` and `cargo check` silently |
| Date | 2026-10-07 |
| Author / agent session | Claude Code session for Henry Chen |
| Branch | `fix/pre-commit-sigpipe` |
| Commits | `417c5ed` (regression test and sibling rewrites, red), `e7af53a` (fix), `93c6222` (pathspec-variable cases, red), `fa98886` (fix), close-out; PR #17 |
| Stage Gate | [Stage 00](../stage-gates/foundation0/00-bootstrap-kernel.md), follow-up from the pre-merge verification of PR #7 |
| Related ADR / requirement | S00.03 (hooks); AGENTS.md law 1 (nothing important fails silently) |
| Status | COMPLETE (2026-10-07; verifier PASS at `fa98886`; merged with PR #17) |

**Risk note (protected path):** `scripts/quality/*` is the gate. The defect let the pre-commit gate pass a change it should have checked, without a message.

## 1. Plan

- [x] One-sentence statement: the hook's "are Rust files staged?" test must give the same answer for any size of staged change.
- [x] Defect (pre-merge verification of PR #7, 2026-10-06; reproduced 3/3 by the verifier with 318 KB of staged names): `git diff --cached --name-only | grep -qE '…'` under `set -o pipefail`. `grep -q` exits at its first match; when the name list is larger than the pipe buffer, `git diff` dies of SIGPIPE (status 141), the pipeline fails, and the `if` skips formatting and type checks with no message. CI still runs the full gate, so the defect costs local feedback, not the required gate.
- [x] Fix: ask git for the matching paths with a pathspec (`git diff --cached --name-only -- '*.rs' '*Cargo.toml' '*Cargo.lock' '*rust-toolchain.toml'`) and test the result for emptiness; no pipe. Git's default pathspec `*` also matches `/`, so `*.rs` covers every directory, as the old regex did.
- [x] Sibling sweep: no other non-test script pipes into `grep -q`. Two gate tests do (`printf "$out" | grep -q …` in `redaction-and-ignored-files.sh` and `secret-scan-merge.sh`). Two of those assert "the key is not printed" and would pass if `printf` died of SIGPIPE: they fail open, though today's scan output is far below the pipe buffer. All four become here-strings in the tests commit; no assertion changes.
- [x] **I will know this works when** a new regression test, committed red before the fix, shows the hook refusing a misformatted Rust file both alone (control) and among thousands of other staged files; the fix leaves the tests unchanged; the full gate passes locally and in CI.

## Evidence log

```text
# Failing before the fix (tests written; hook unchanged), 2026-10-07 — three runs, same result
$ scripts/quality/tests/pre-commit-large-change.sh
HOOK ok: a misformatted Rust file staged alone is refused
HOOK FAIL: a misformatted Rust file among 4,000 staged files is refused (the hook passed a misformatted Rust file)
exit=1   (3 of 3 runs)

# Sibling tests rewritten to here-strings (same assertions)
redaction-and-ignored-files.sh: REDACT ok x2, IGNORED ok · secret-scan-merge.sh: MERGE ok x3
against a scratch copy of the scan without --redact (not committed): REDACT FAIL x2, MERGE FAIL x2 — leaks still caught

# After the fix (fix-lock engaged after 417c5ed; tests unchanged since then)
$ scripts/quality/tests/pre-commit-large-change.sh        (3 runs)
HOOK ok: a misformatted Rust file staged alone is refused
HOOK ok: a misformatted Rust file among 4,000 staged files is refused
exit=0   (3 of 3 runs)
scratch repo, only notes.txt staged → hook exit 0 (no Rust staged, cargo not run)
```

## Verification round 1 (verifier, 2026-10-07): environment variables

The verifier could not run the tests: with the fix-lock engaged, the guard refuses writing any path containing `tests/`, even a `/tmp` export, and it stopped there. Reading the change, it found that git's pathspec environment variables alter what the new pathspec matches: `GIT_LITERAL_PATHSPECS=1` makes `*.rs` literal (nothing matches) and `GIT_GLOB_PATHSPECS=1` stops `*` matching `/` (nested files missed), so the hook could again skip its checks silently. Also noted, both acceptable: a rename `a.rs` → `a.txt` now runs cargo (more correct: removing a Rust file can break the build), and names git prints quoted are now caught.

**Owner decision (2026-10-07):** release the lock, add test cases first, fix, and release again before verification so the verifier can run the tests.

```text
# New cases (only a nested Rust file staged, so no top-level Cargo.toml triggers the checks), against the first fix
HOOK ok: a staged nested Rust file is refused
HOOK FAIL: … with GIT_LITERAL_PATHSPECS=1 set (the hook passed a misformatted Rust file)
HOOK FAIL: … with GIT_GLOB_PATHSPECS=1 set (the hook passed a misformatted Rust file)
HOOK FAIL: … with GIT_NOGLOB_PATHSPECS=1 set (the hook passed a misformatted Rust file)

# After clearing the pathspec variables for the one git call (fix-lock engaged after 93c6222; tests unchanged since)
$ scripts/quality/tests/pre-commit-large-change.sh        (2 runs)
HOOK ok ×6 (alone · among 4,000 files · nested · GIT_LITERAL_PATHSPECS · GIT_GLOB_PATHSPECS · GIT_NOGLOB_PATHSPECS), exit=0
$ scripts/quality/full-gate.sh → RESULT: PASS (13 steps)
```

The fix-lock was released again after this commit, by the owner's decision, so the verifier can run the tests from a `/tmp` export; that the tests did not change during the fix is shown by `git diff 93c6222 <fix commit> -- scripts/quality/tests` (empty).

## Close-out (2026-10-07)

- [x] Verifier PASS at `fa98886` (fix-lock not engaged during its run; its `/tmp` exports checked with `cmp` and removed):
  - 417c5ed's test: red against main's hook (4,000-file case), green against `e7af53a` and the head. 93c6222's test: red against `e7af53a` (the three variable cases), green at the head. Tests unchanged through each fix.
  - File selection, 25 scenarios through both selections: the same answer as main's regex, or broader (a rename `.rs` → `.txt`; names git quotes, such as `ä.rs`). A change staging only `README.md` still skips cargo.
  - The here-string tests still fail on a scan without `--redact` and without `--verbose`.
  - CI run 37573859282 at `fa98886`: `RESULT: PASS (13 steps)`; its own local gate in a `/tmp` clone: `RESULT: PASS (13 steps)`.
- Known, not changed here (both predate this record; CI's full gate covers them):
  - Neither the old nor the new selection triggers on a `rust-toolchain` file without extension or on `.cargo/config.toml`, though both affect the build.
  - When cargo fails for a reason other than formatting (for example a crate that lost its only source file in a rename), the hook still says "formatting differs".
  - No case for `GIT_ICASE_PATHSPECS`: it only widens matching, so it cannot cause a skip; the hook clears it anyway.
- [x] Stage 00 follow-up marked resolved. Scrap: nothing task-created left in the repository.
