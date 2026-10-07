# Build Record — The pre-commit hook must not skip its checks on a large staged change

| Field | Value |
|---|---|
| Change | `scripts/quality/hooks/pre-commit` decides whether Rust files are staged without a pipe into `grep -q`, so a large staged change can no longer make it skip `cargo fmt --check` and `cargo check` silently |
| Date | 2026-10-07 |
| Author / agent session | Claude Code session for Henry Chen |
| Branch | `fix/pre-commit-sigpipe` |
| Commits | fill at close-out |
| Stage Gate | [Stage 00](../stage-gates/foundation0/00-bootstrap-kernel.md), follow-up from the pre-merge verification of PR #7 |
| Related ADR / requirement | S00.03 (hooks); AGENTS.md law 1 (nothing important fails silently) |
| Status | IN PROGRESS |

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
