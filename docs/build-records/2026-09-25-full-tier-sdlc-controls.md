# Build Record — Full-tier AI-session controls

| Field | Value |
|---|---|
| Change | Add the owner's full-tier AI-session controls: guard hook, policy, verifier, review instructions, guard CI |
| Date | 2026-09-25 |
| Author / agent session | Claude Code |
| Branch | main (uncommitted; owner commits) |
| Commits | This change (committed separately from the CLAUDE.md entry-point change) |
| Stage Gate | N/A — agent tooling; no Foundation-0 gate opened or closed |
| Related ADR / requirement | Engineering Standard v1.2 (protected main, secret scan, "point every AI session at this file"); AGENTS.md Five working laws |
| Status | COMPLETE — verifier PASS (round 3); first CI run NOT YET PROVEN at commit time (see deferral table) |

## 1. Plan

- [x] One-sentence statement of change: Claude Code sessions in this repository get the same deterministic gates as the owner's other full-tier repositories (rikurinode, Iceman, AISecurity), with a policy written for Rikuri Pi.
- [x] Owner intent: on 2026-09-25 the owner chose "Full tier" for Rikuri-PI ("Ok go ahead") when asked which tier the repository is. The owner's standing rule for a full-tier bootstrap: copy `guard.py` and `test_guard.py` unchanged from rikurinode; write a repository-specific `guard-config.json` with worked examples, `settings.json`, `REVIEW.md`, and `.claude/agents/verifier.md`; wire the guard tests into CI.
- [x] Blast radius: new `.claude/` files, `REVIEW.md`, `.github/workflows/claude-guard.yml`, `.gitignore`, and the maturity and gate text in `AGENTS.md` and `README.md`. No application, canonical data, money, or execution path exists yet. Every later Claude Code session in the checkout is affected: tool calls matching the policy are refused or need the owner's approval.
- [x] Current behavior characterized: Claude Code sessions here have no hook, so a session can read secret files, force-push, and edit AGENTS.md or the gate documents without the owner. `CLAUDE.md` imports `AGENTS.md` (see `2026-09-25-claude-md-entry-point.md`). No CI exists. `guard.py` and `test_guard.py` are byte-identical across the three existing repositories' working trees (`cmp`), including an uncommitted, additive `fix_lock_preflight` change from another session that is present in all three.
- [x] Verification defined up front — "I will know this works when": (1) the guard tests pass on Python 3.9 and 3.12 in this checkout, replaying every worked example; (2) `guard.py` and `test_guard.py` are `cmp`-identical to rikurinode's; (3) a fresh Claude Code session started in this checkout is refused a read of `.env` by the hook; (4) an independent verifier reports on the change.
- [x] Architecture-impact check completed:
  - [x] invariant / ADR — no architecture change; enforces existing laws (Law 2 stop the line, secret handling, protected main) for one agent client.
  - [x] API / schema / event format — N/A.
  - [x] canonical vs derived data — N/A.
  - [x] privacy / instruction authority / security — Claude Code's Read and Grep tools are refused secret files and common Bash readers are caught; the guard is a tripwire, not a sandbox (see Limitations).
  - [x] deletion / backup / restore — N/A.
  - [x] budget / cost — N/A.
  - [x] nondeterministic AI / replay — N/A.
  - [x] documentation / help / RAG — AGENTS.md and README updated; no help or RAG surface exists yet.
  - [x] module ownership / dependency direction — N/A.
- [x] Relevant threat/control/test identified: an agent session that reads a secret into its transcript, rewrites remote history, weakens a gate (procedure, CI, branch protection, secret scan, stage-gate record) or edits a test to make a fix pass. Controls: `guard.py` decisions, replayed by `test_guard.py` from the worked examples.
- [x] Existing solution review: Claude Code's PreToolUse hook mechanism; the owner's shared `guard.py` (Python standard library only, no dependencies) already running in three repositories. No custom mechanism is written here.
- [x] Authoritative state change? N/A — no canonical state.

## 2. Build

- [x] Smallest coherent implementation completed: `.claude/hooks/{guard.py,test_guard.py}` and `.claude/settings.json` copied unchanged from rikurinode; Rikuri Pi policy in `.claude/hooks/guard-config.json` (78 worked examples after the verifier rounds below); `.claude/agents/verifier.md`; `REVIEW.md`; `.github/workflows/claude-guard.yml` (Python 3.9 + 3.12); `.gitignore` keeps `.claude/` local except the shared gates; `AGENTS.md` gains a "Deterministic gates" section and updated maturity/verification text; `README.md` maturity sentence updated.
- [x] Inputs crossing trust boundaries validated: the hook reads Claude Code's PreToolUse JSON; malformed input or config returns ASK, never a silent allow (existing `FailureModes` tests).
- [x] New code is typed and cohesive: no new code; the shared guard is 380 lines.
- [x] Old/replaced implementation removed: N/A, nothing replaced.
- [x] Module Contract: N/A, no crate exists.
- [x] Security question answered: secret reads (deny), force-push and history rewrite (deny / ask), protected paths and GitHub gate changes (ask), test edits under fix-lock (deny). Allowed and denied paths are both proven by the worked examples and the live probes below.

Deliberate deviation from the owner's bootstrap rule: `BUILD-PROCEDURE.md` and rikurinode's template are **not** copied. AGENTS.md is already the adaptation of that procedure ("Adapted from the supplied Build Procedure and Build Checklist") and says it "does not create a competing roadmap"; Template A in the Engineering Templates is this repository's build-record template. A second procedure file would be the competing source AGENTS.md forbids.

Protected set for this repository (the owner said "commit and push" after reviewing this list on 2026-09-25, taken as acceptance; reversible by editing `protected_globs`): the procedure (AGENTS.md, CLAUDE.md, REVIEW.md, Engineering Standard and its planned `docs/engineering/` home, Engineering Templates, Test and Acceptance Strategy), the gates (guard, Claude settings including `settings.local.json`, verifier, CI workflows, `scripts/quality/*`, `deny.toml`, `.gitleaks.toml`, `.pre-commit-config.yaml`, `.gitignore`), the records that decide scope and gate status (ADRs including the planned `docs/architecture/adr/`, the Stage Plan, `docs/stage-gates/*`), and the directories holding them, so `rm -r` of a whole directory also asks. The Architecture Spec, PRD, ADR Starter Pack, and build records stay editable. Cost: every Stage Plan checkbox or gate-board update asks the owner; that is intended, since documentation cannot close a gate.

## 3. Debug — N/A, not a defect

## 4. Verify

- [x] Unit tests: `test_guard.py` replays all 78 worked examples plus the shared generic and failure-mode cases. Python 3.12.3: PASS (16 tests). Python 3.9.25: PASS (16 tests).
- [x] Shared contract suite — N/A.
- [x] Invariant/property test — N/A.
- [x] Replay/golden fixture — N/A.
- [x] Running-system probe: PASS. On a scratch copy of the checkout, a fresh `claude -p` session's `Read .env` was refused with the guard's secret-file reason. A second session with `--permission-mode acceptEdits` could not edit `AGENTS.md` (builder-run; not independently reproduced). The hook returned `ask`, which a non-interactive session cannot approve. `AGENTS.md` in the copy was byte-identical afterwards. The verifier repeated the `.env` probe in the real checkout: PASS.
- [x] End-to-end traced — N/A.
- [x] Recovery/fallback path — N/A.
- [x] Regression guard proven to fail: PASS. A scratch copy with `AGENTS.md` removed from `protected_globs` and `*.kdbx` removed from `secret_globs` failed 2 tests (`'allow' != 'deny'`, `'allow' != 'ask'`).
- [x] Cost/performance — N/A.
- [x] Independent verifier (`.claude/agents/verifier.md` instructions, separate agent): first run `VERDICT: FAIL`, because a session could switch the hook off by writing `.claude/settings.local.json`, a silent allow in a git-ignored file. Its other findings: secret reads the policy missed (`*.env.*`, `*.local.*`, `hosts.yml`, `.git-credentials`, `gh auth token`), GitHub writes through `gh api` other than protection (GraphQL mutations, lowercase methods, actions/merge/refs endpoints), `.gitignore` edits, directory deletes (`rm -rf .claude/hooks`, `rm -rf .claude/state`, `git rm -r .github`), the unprotected Test and Acceptance Strategy and planned ADR/Standard homes, false positives (read-only GraphQL, `*baseline*` catching build records), and overstated claims in AGENTS.md, REVIEW.md and this record. The builder reproduced four of the findings before fixing them. The policy findings were fixed with worked examples and the claims corrected. The exception is directory-based secrets (`secrets/…`), which the shared guard cannot express; they are listed under Limitations. Second run: `VERDICT: FAIL`. All round-1 findings were handled, but common forms of the new `gh` patterns got through: `gh auth status -t`, GraphQL bodies from a file, `--input`, or a shell variable, and `-fKEY=VAL`. Fixed with worked examples, together with the cheap evasion forms (a repeated `-X`, a header containing `graphql`). The baseline test globs were narrowed to data files, so build records no longer match. Remaining forms are listed under Limitations.

## 5. Close Out

- [x] Full required gate green: the documentation-maturity gate (links, `git diff --check`) and the guard tests pass. The Stage 00 gates do not exist yet.
- [x] Scrap Sweep complete: scratch copies were used for the probes and removed. The test runs' `__pycache__` was removed. No temporary files remain in the checkout.
- [x] No unexplained skipped/disabled tests or new unowned debt (see table).
- [x] Documentation updated in the same change: AGENTS.md (Deterministic gates section, maturity paragraph, verification line), README.md, REVIEW.md. No help or RAG surface exists yet.
- [x] Traceability — N/A.
- [x] Stage Gate Record — N/A.
- [x] Commit message explains WHY; Build Record committed with change, separately from `2026-09-25-claude-md-entry-point.md` (owner asked for commit and push on 2026-09-25).

## Limitations

- **Tripwire, not a sandbox.** The shared `guard.py` does not catch `python3 -c "open('.env').read()"`, shell globs (`cat .e*`), `rg -uu` over the tree, case variants (`Read .ENV` opens `.env` on a case-insensitive macOS filesystem), the Grep tool's `glob` parameter, git aliases, or directories named in `.gitignore` (such as `secrets/`, since policy globs match basenames). Fixing these means changing the shared `guard.py` in every full-tier repository, so they are recorded rather than fixed here.
- **Deliberately obfuscated commands** get past the policy's regexes, for example a keyword split across quotes (`query='mut''ation …'`). This is what review and the owner are for.
- **Changed directory, then relative path**: `cd .claude/state && rm fix-lock` releases the fix-lock without asking, because the shared guard does not carry `cd` into later segments.
- **Policy false positives accepted for coverage**: every `.gitignore` asks, including nested ones. A `rm -r` of any directory named `doc` asks. `rm -rf .claude/hooks/__pycache__` asks (`find … -exec rm` does not). A `hosts.yml` anywhere is treated as a secret, like the gh token file. Any `gh api` write, including a PR comment, asks. A read-only GraphQL query that uses variables (`query($owner:String!)`, gh's documented form) asks, because of the `$` rule.
- **Shared-guard false positives**: `git push origin --delete <branch>` is denied. `grep -e <word>` treats a secret-looking word such as `credentials` as a file. `git check-ignore .env` and `git log --grep credentials` ask.
- **Claude Code only.** Codex and other agents get the same rules by instruction, not enforcement.
- **User-level settings** (`~/.claude/settings.json`) are outside the repository and cannot be protected by this policy.
- GitHub Actions are pinned by tag (`@v4`, `@v5`), as in the sibling repositories. The same mutable-tag supply-chain risk applies to all of them.

## Skipped / Deferred / Resequenced — visible, never silent

| Item | Disposition | Reason | Dependency impact | Owner | New stage/ordinal | Reactivation / acceptance condition |
|---|---|---|---|---|---|---|
| `claude-guard.yml` observed green on GitHub | NOT YET PROVEN | Never run: `.github/` is not on origin/main; the owner commits and pushes | Until then only local runs prove the tests; Python 3.9 on `ubuntu-latest` is unproven | Henry | first push after this change | Both matrix jobs (3.9, 3.12) green on that push |
| Protected set confirmed | DONE | Owner said "commit and push" after reviewing the list (2026-09-25) | None; the set can be narrowed by editing `protected_globs` | Henry | — | — |
| `.claude/settings.local.json` gap in rikurinode, Iceman, AISecurity | DEFERRED | Other repositories' protected policy files; rikurinode's has another session's uncommitted edits | Those repositories' hooks can be disabled by a silent write | Henry | next change to each repository's guard policy | Each policy protects `.claude/settings.local.json` with a worked example |
| `guard.py` docstring lists only three repositories | DEFERRED | The shared file carries another session's uncommitted `fix_lock_preflight` change in all three repositories; editing it now mixes two changes | Documentation only | Henry | next `guard.py` sync | Docstring names all full-tier repositories; `cmp` identical across all four |

## Evidence log

```text
cmp guard.py / test_guard.py / settings.json vs rikurinode, Iceman, AISecurity working trees: identical (9/9)
python3 (3.12.3) -m unittest discover -s .claude/hooks -p 'test_*.py'   -> Ran 16 tests ... OK   (67 examples)
python3.9 (3.9.25, uv)   same                                            -> Ran 16 tests ... OK
weakened-policy scratch copy                                             -> FAILED (failures=2)
claude -p (scratch copy) Read .env -> ".../.env is a secret file — its contents must not enter the transcript; ask the owner for the one value you need"
claude -p --permission-mode acceptEdits (scratch copy) Edit AGENTS.md -> blocked; cmp AGENTS.md unchanged
guard.py direct: Edit AGENTS.md -> {"permissionDecision": "ask", ...}; Edit doc/Rikuri_Pi_PRD_v1.2.md -> silent allow
verifier round 1: VERDICT: FAIL (settings.local.json silent allow) + policy gaps -> fixed, 44 -> 67 examples
verifier round 2: VERDICT: FAIL (gh auth status -t, graphql @file/--input/$VAR, -fKEY=VAL) -> fixed, 67 -> 78 examples; py3.12 + py3.9 Ran 16 tests OK
verifier round 3: VERDICT: PASS (all round-2 items ask or are in Limitations; routine gh/git/cargo/build-record actions allow; 3.12 + 3.9 OK; cmp 9/9; one nit, the $-variable GraphQL false positive, added to Limitations)
```
