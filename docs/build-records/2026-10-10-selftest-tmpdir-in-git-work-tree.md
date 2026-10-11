# Build Record — Gate self-tests pass with TMPDIR inside a git work tree

| Field | Value |
|---|---|
| Change | `policy-bypasses.sh` builds its fixture outside every git work tree wherever `TMPDIR` points: git looks for a repository no higher than the fixture, and the test stops by name if the fixture is still inside one. A new gate self-test reruns every other self-test with `TMPDIR` in an ignored directory of a git work tree |
| Date | 2026-10-10 (close-out 2026-10-11) |
| Author / agent session | Claude Code (Opus 5.5) for Henry Chen, a side session started from the E1 session |
| Branch | worktree `claude/angry-wilson-8c8cfd`, from `main` at `ee30d43` (after PR #27) |
| Commits | `d3ad41a` (the new self-test alone, red), `4b80ce9` (fixture fix), close-out (README and this record); made after the owner chose "Create PR" |
| Stage Gate | [Stage 00](../stage-gates/foundation0/00-bootstrap-kernel.md), gate hygiene: S00.03 (gate scripts); no Stage Gate item changes |
| Related ADR / requirement | AGENTS.md laws 1, 4 and 5 (nothing fails silently; detect absence of success; defects become tripwires) |
| Status | COMPLETE locally (verifier round 1 PASS; all 15 gate steps PASS, in two runs); CI Not yet proven |

**Risk note (protected paths):** `scripts/quality/*` is the gate. The change adds one self-test and isolates one self-test's fixture. It removes no case, and no check the gate applies to the product changes (`crate-policy.sh` and `full-gate.sh` are untouched). The gate still has 15 steps; the `gate-selftests` step takes about 30 s longer (measured: the other self-tests 25 s, the new one 29 s).

## 1. Plan

- [x] One-sentence statement: the gate's self-tests give the same result with `TMPDIR` inside a git work tree as outside one, and the gate fails when one does not.
- [x] Defect (found by a verifier on 2026-10-10, with `TMPDIR` under `/home/junc/Rikuri-PI/target/`): `policy-bypasses.sh` failed two cases, `BYPASS FAIL: a build script that sets CLIPPY_ARGS is refused (expected fail, got pass)` and the same for `RUSTC_BOOTSTRAP`. With the default `TMPDIR` the step passes. It failed closed (the gate went red; no policy hole).
- [x] Current behaviour characterized (evidence log): reproduced with `TMPDIR` under this worktree's ignored `target/` and under an untracked directory of a scratch work tree, the same two lines both times (57 ok, rc 1); with the default `TMPDIR`, 59 ok, rc 0.
- [x] Root cause: check 4 of `crate-policy.sh` (round 3, 2026-10-05) judges only tracked member sources when `git -C <root> rev-parse --is-inside-work-tree` succeeds, because tracked files are what CI sees. The fixture was a bare `mktemp -d`. With `TMPDIR` inside a repository, git's discovery walks up from the fixture to the enclosing work tree (from `/tmp/wt/tmp/fixture`, `--show-toplevel` is `/tmp/wt`). `git ls-files` then lists none of the fixture's files, so no build script is judged and both refusals pass. The fixture's last two cases were unaffected because round 3 runs `git init` in the fixture itself. Cargo's side was checked too. The fixture's root manifest is a `[workspace]`, so Cargo looks no higher for a workspace. Cargo does read config from parent directories, but neither the repository nor `~/.cargo` has one here (checked by presence), and the `outer` case tests a parent config on purpose.
- [x] Defect class: a self-test whose result depends on what surrounds `TMPDIR` (here, repository discovery).
- [x] Sibling sweep: all 14 self-tests run with the default `TMPDIR` and with `TMPDIR` in an ignored directory of a git work tree. Only `policy-bypasses.sh` differed. The others create their own repository with `git init` or do not consult git. None left anything in the `TMPDIR`.
- [x] `crate-policy.sh` is not changed: judging only tracked sources in a work tree is its intended round-3 behaviour, and the gate always runs it on the repository's top level.
- [x] Options for isolating the fixture: (a) `GIT_CEILING_DIRECTORIES`, git's documented boundary for repository discovery; (b) `git init` at the start, which would leave crate-policy's not-a-repository path (every source judged) untested, since the cases before round 3 are what cover it; (c) a fixed directory outside `TMPDIR`, but no path is guaranteed to be outside a repository. Chosen: (a), plus an explicit check of the precondition.
- [x] **I will know this works when:** the new self-test fails before the fix, naming `policy-bypasses.sh` and its two cases, both with the default `TMPDIR` (as the gate runs it) and nested in a work tree (as the verifier ran it); it passes after the fix; `policy-bypasses.sh` still passes 59 of 59 with the default `TMPDIR`; the fixture stops by name if it ends up inside a work tree anyway; the new self-test fails, rather than passing vacuously, if its scratch directory is not really an ignored directory of a work tree, or (added after verifier round 1) if a rerun is not given that directory as its `TMPDIR`; the full gate passes.

## 2. Build

- [x] `scripts/quality/tests/policy-bypasses.sh`: exports `GIT_CEILING_DIRECTORIES` as the fixture's parent, so git looks for a repository no higher than the fixture. If the fixture is still inside a work tree, it stops with `BYPASS FAIL: the fixture … is inside a git work tree; the cases before round 3 need it outside one`. That happens, for example, when a caller exports `GIT_DIR`, which the ceiling does not stop. The cases themselves are unchanged.
- [x] New `scripts/quality/tests/tmpdir-in-git-work-tree.sh`: makes a scratch git work tree with an ignored `target/`, and fails with "nothing was checked" unless git sees `target/tmp` as an ignored directory inside it. It then sets its own inherited `TMPDIR` to a directory that does not exist (added after verifier round 1), so a rerun not given the work tree's `TMPDIR` fails at its `mktemp -d` rather than passing as it always did. It reruns every other self-test with `TMPDIR` in the work tree and fails, naming each self-test that fails (with its output) or saying it found none.
- [x] README: the gate step list says one self-test reruns the others with `TMPDIR` inside a git work tree, so a self-test that passes only outside one fails the gate.
- [x] AGENTS.md, Module Contracts, the Stage Gate Record: N/A (no crate responsibility, interface or gate item changes).

## 3. Debug

- [x] Problem reproduced before the fix: yes (evidence log).
- [x] State-changing remediation: none. Every run used scratch directories inside the outer safety layer's private `/tmp`, or `target/selftest-tmp` in this worktree, which was removed after each run (empty each time).

## 4. Verify

All runs were inside the outer safety layer: user, mount, PID and network namespaces; `/run` masked; private `/tmp`; refusing stand-ins for docker, sudo, systemctl, lncli and the like first on `PATH`; `DOCKER_HOST` at a missing socket. `lnd` started at `2026-10-09T20:20:27.903167252Z` before and after every run.

- [x] New self-test red before the fix; result: the final version, run in a committed copy of `HEAD` (`git archive`), fails with `TMPDIR-IN-GIT FAIL: policy-bypasses.sh fails with TMPDIR inside a git work tree` and the two `BYPASS FAIL` lines above, while the 13 other self-tests pass, rc 1. This holds with the default `TMPDIR` and with `TMPDIR` nested in an ignored `target/` of a work tree, and nothing is left there. (The first version, before the round-1 change, was red the same way in this worktree.)
- [x] Green after the fix; result: the new self-test passes 14 of 14 with the default `TMPDIR` and nested, and leaves nothing behind. `policy-bypasses.sh` passes 59 of 59 with the default `TMPDIR`, with `TMPDIR` a symlink into a work tree, with a trailing slash, and with `TMPDIR` the top of a work tree. With `GIT_DIR` exported (the verifier also tried it with `GIT_WORK_TREE`), it stops by name (rc 1).
- [x] Mutants of the new self-test (scratch copies, not committed; default `TMPDIR`):
  - M1, the rerun without `TMPDIR="$inner"`: caught; all 14 reruns fail.
  - M2, the same with no poisoned `TMPDIR`: passes 14 of 14, which shows the poisoned `TMPDIR` is what catches M1. Verifier round 1 found this gap before the change.
  - M3, no ignore rule, and M4, no `git init`: each caught as `… is not an ignored directory of a git work tree; nothing was checked`.
  - With `TMPDIR` already inside another work tree, M4 passes. That pass is legitimate, not vacuous: the reruns are still inside a work tree.
- [x] Full gate, run 1 (before the round-1 change); result: `scripts/quality/full-gate.sh` passed its first nine steps: tools; gate self-tests (15 files, with `policy-bypasses.sh` 59 of 59 and the new self-test's 14 reruns ok); guard; format; build; clippy; test (177 passed, `RESIDUE none`); smoke; crash matrix (6 passed). It stopped at `dependencies`, where `cargo deny` could not resolve `github.com` without a network. The remaining six steps were then run one by one with the gate's exact commands, in the same layer but sharing the host network (only the resolver directory under `/run` exposed). All six PASS: dependencies (`advisories ok, bans ok, licenses ok, sources ok`), secrets (132 commits, no leaks), ratchet (0/0/0), contracts (7 crates), crate policy (7 crates), and scrap, which listed this change's two new files as untracked before commit. Nothing was left in the private `/tmp`.
- [x] Full gate, run 2 (after the round-1 change, same layers); result: identical. Steps 1–9 PASS (15 self-test files, 14 reruns ok, 59 BYPASS ok; test 177 passed with `RESIDUE none`; crash matrix 6 passed). `dependencies` again could not resolve `github.com`, then the six remaining steps PASS with the host network shared. Nothing was left in the private `/tmp`. One uninterrupted 15-step run is CI's to show.
- [x] Independent verifier round 1 (**PASS**, on the uncommitted tree; same layers, `lnd` start time unchanged):
  - Passed: the guard tests (91); gate steps 1–9 in one run and the other six one by one; links and S00.03; red in a committed copy of `HEAD` (13 ok, the two cases named, both `TMPDIR`s); green, 14 of 14 both ways; the `policy-bypasses.sh` edge cases; the `GIT_DIR` refusal; the M3 and M4 mutants.
  - Diff matches the Plan: exactly the four named files. The only test-code change from red is the one additive `policy-bypasses.sh` hunk, with its reason above.
  - Findings, none a Fail: (1) "183 passed" was the test step plus the crash matrix (177 + 6); (2) a mutant dropping `TMPDIR="$inner"` from the rerun survived, since the precondition proves the directory is right, not that the reruns use it; (3) the README's "so none depends on where `TMPDIR` points" claimed more than the check proves; (4) "each caught" held for the default `TMPDIR` only; (5) the added time was not measured.
  - Not yet proven: an uninterrupted 15-step run, and CI.
- [x] Responses, round 1:
  - (1) Corrected (177 + 6).
  - (2) Closed test-first: the check now poisons its inherited `TMPDIR` (Build section). M1 is caught and M2 shows why; the final version is red against `HEAD` as before.
  - (3) README and this record's statement reworded to what the check proves.
  - (4) Recorded under Mutants.
  - (5) Measured: about 30 s.
  - The changes after round 1 are confined to the new self-test (two lines and a comment), README wording and this record, each shown above. No second verifier round was run.
- [ ] CI: Not yet proven (this PR's `Full gate (Ubuntu)` run). The macOS job runs no gate self-tests.

## Close-out (2026-10-11)

- [x] Commits: `d3ad41a` holds `tmpdir-in-git-work-tree.sh` alone. Run from that commit's tree (a committed copy, safety layer), it fails naming `policy-bypasses.sh` and its two cases (13 reruns ok, rc 1). `4b80ce9` adds only the fixture hunk (`git diff d3ad41a 4b80ce9`: `policy-bypasses.sh`, 11 insertions), and from its tree the check passes 14 of 14. The close-out commit adds README and this record.
- [x] Scrap: nothing task-created is left in the repository. The outer safety layer and run logs are in the session's scratchpad, outside the repository. `target/selftest-tmp` was removed after each run; the verifier removed its `target/verify/`. The scrap sweep lists only this change's two new files, which get committed.
- [x] Docs: README gate step list.

## Limitations

- The new self-test exercises one surrounding: an ignored directory of a work tree (the observed one; an untracked directory failed the same way in the reproduction). Other things that could surround `TMPDIR` are not exercised, for example a parent Cargo config.
- The poisoned `TMPDIR` catches a rerun that is not given the work tree's `TMPDIR` only through tools that refuse a missing `TMPDIR`. `mktemp` refuses, and every self-test makes its fixture with `mktemp -d`. Python's `tempfile` silently falls back to `/tmp`.
- A caller that exports `GIT_DIR` or `GIT_WORK_TREE` overrides repository discovery for every git-using self-test. `policy-bypasses.sh` now stops by name in that case; the other self-tests do not check for it. No gate caller exports them (the pre-commit hook runs no self-tests).
- Observation, not changed here: when every member source is untracked, `crate-policy.sh` judges no source for check 4 and still prints only `POLICY checked 1 crate(s)`, which is how this defect looked like a pass. Reporting how many sources check 4 judged would make an empty judgement visible.

## Evidence log

```text
# Before the fix: policy-bypasses.sh alone
A default TMPDIR                                    rc=0  59 ok
B TMPDIR=<worktree>/target/selftest-tmp (ignored)   rc=1  57 ok
  BYPASS FAIL: a build script that sets CLIPPY_ARGS is refused (expected fail, got pass)
    POLICY checked 1 crate(s)
  BYPASS FAIL: a build script that sets RUSTC_BOOTSTRAP is refused (expected fail, got pass)
    POLICY checked 1 crate(s)
C TMPDIR=/tmp/wt/tmp (untracked, scratch work tree) rc=1  57 ok, the same two lines

# Mechanism
/tmp/plain/fixture: inside-work-tree=fatal: not a git repository (… up to mount point /)
/tmp/wt/tmp/fixture: inside-work-tree=true toplevel=/tmp/wt
  with GIT_CEILING_DIRECTORIES=/tmp/wt/tmp: fatal: not a git repository

# Sibling sweep: every self-test, rc with default TMPDIR | in an ignored dir of a work tree
crate-checks 0|0  policy-bypasses 0|1  pre-commit-large-change 0|0  ratchet-and-paths 0|0
ratchet-lexing 0|0  ratchet-wrapped 0|0  redaction-and-ignored-files 0|0
scrap-sweep-excludes 0|0  secret-scan-merge 0|0  secret-scan-scope 0|0
secret-scan-target-history 0|0  smoke-checks-content 0|0  temp-residue 0|0
workspace-policy 0|0   (nothing left in the TMPDIR)

# Final check, red against HEAD (committed copy), default and nested TMPDIR
rc=1  TMPDIR-IN-GIT FAIL: policy-bypasses.sh fails with TMPDIR inside a git work tree
        BYPASS FAIL: a build script that sets CLIPPY_ARGS is refused (expected fail, got pass)
        BYPASS FAIL: a build script that sets RUSTC_BOOTSTRAP is refused (expected fail, got pass)

# After the fix
check, default TMPDIR                rc=0 ok=14 (29 s; the other self-tests take 25 s)
check, nested TMPDIR                 rc=0 ok=14, left 0
policy-bypasses, default             rc=0 ok=59
  symlinked TMPDIR / trailing slash / TMPDIR = work-tree top: rc=0 ok=59 each
  GIT_DIR exported                   rc=1  BYPASS FAIL: the fixture /tmp/tmp.tDV47zJToL is inside a git work tree; …
M1 rerun without TMPDIR="$inner"     rc=1  14 × TMPDIR-IN-GIT FAIL
M2 M1 without the poisoned TMPDIR    rc=0  ok=14 (the gap verifier round 1 found)
M3 no .gitignore / M4 no git init    rc=1  TMPDIR-IN-GIT FAIL: … is not an ignored directory of a git work tree; nothing was checked
```
