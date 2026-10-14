# Build Record — Trim the agent rules to what changes behaviour

| Field | Value |
|---|---|
| Change | Rewrite `AGENTS.md` as a short rule set (priorities, hard limits, the Task Contract, done and stop, just-enough verification), and cut the Engineering Standard's process sections to what an agent would not do unprompted, with the two 2026-10-13 dated clarifications folded into the body. |
| Date | 2026-10-13 |
| Author / agent session | Claude Code (Opus 5.5) for Henry Chen |
| Branch | `docs/trim-agent-rules`, worktree `/home/junc/claude-wip/trim/rpi`, from `origin/main` `b556200` (PR #37 merged) |
| Commits | `2f8e4f3`, the change (owner, 2026-10-14: "Commit, push and open PR"); a second commit records this row |
| Stage Gate | N/A: working rules; no Stage 00 item, interface or gate status changes |
| Related ADR / requirement | Owner, 2026-10-13: "draft the trimmed AGENTS.md … be clear about priorities, your risk limits, when to stop, and what 'done' means. Just enough test and verification, prevent it from infinite checks/loops … modify the procedure to what's reasonable and delete things that don't contribute any more benefit. Another agent doing an extra check would work also." |
| Status | DONE locally; [PR #38](https://github.com/rikuripi/Rikuri-PI/pull/38) open; CI Not yet proven until a run is observed; the owner merges |

**Risk note (protected paths):** `AGENTS.md`, `REVIEW.md`, `.claude/agents/verifier.md`, `.gitignore`, `scripts/quality/scrap-sweep.sh` (one comment) and the Engineering Standard are protected paths. No guard, policy, settings, CI or executable line changes. What is removed is restatement of default agent behaviour, narrative case studies and duplicated text (listed under Build). The first draft also dropped two safety rules; verifier round 1 caught them and they are restored (Debug).

## Task Contract

1. **Outcome:** an agent reads a short `AGENTS.md` that says what matters, what it must never do, how far to go and when to stop; the Engineering Standard's process sections say only what each step must contain, by risk tier.
2. **Scope:** `AGENTS.md` (rewrite); `doc/Rikuri_Pi_Engineering_Standard_v1.2.md` from "2. Standard Work" to the end of "Working with AI Assistants" (rewrite; Purpose, the Foundation-0 profile, security assurance and the release gate unchanged); citation fixes in `REVIEW.md`, `.claude/agents/verifier.md`, `.gitignore`, `scripts/quality/scrap-sweep.sh` and `scripts/quality/full-gate.sh` (comments), and `doc/README.md` (link text); the last two were added by repair cycle 1. Tier: **high-risk** (review and verification rules). **Not included:** guard, policy, settings, CI, Stage Plan, Templates, Test Strategy, ADRs; the node repository (drafts handed to a node session).
3. **Done when:** every hard limit, invariant, gate rule and owner decision in the old texts is kept or its removal is listed with a reason; every citation of a section or heading in the repository resolves; links and anchors resolve; `git diff --check` clean; guard tests pass inside the outer safety layer; one verifier pass (plus at most one follow-up) with no Important finding open.
4. **Authority:** edit the files in Scope in this worktree; no commit, push or PR without the owner.
5. **Limits:** two repair cycles; one verifier pass plus one follow-up.

## 1. Plan

- [x] Citations that must keep resolving (`git grep`, outside dated records): `AGENTS.md#claude-guard-and-fix-lock` (Architecture spec, PRD, guard-config `_doc`); "AGENTS.md §…" in `REVIEW.md` and `verifier.md` (renumbered here); "Law 2" (guard-config); "Phase 0" (`ratchet.sh`, pre-commit hook); "Scrap Sweep" / "Scrap Taxonomy" (scripts, Templates, Master Build Sequence, Test Strategy, Architecture, README); "Close Out" and "Future-learning evidence rule" (`REVIEW.md`, Templates, Architecture); "AGENTS.md > Deterministic gates" in `.gitignore` and `claude-guard.yml` (already stale since PR #36; `.gitignore` repointed, the CI comment left, as PR #36 chose).
- [x] I will know this works when the "Done when" line holds.

## 2. Build

- [x] **`AGENTS.md`**, rewritten: 130 → 111 lines. Its sections:
  1. priorities;
  2. hard limits;
  3. the Task Contract;
  4. done and stopping;
  5. verification, enough and once;
  6. agents, branches and the owner's time, including the "Claude guard and fix-lock" heading the Architecture spec and PRD link to;
  7. what the code must preserve;
  8. where things are.
  - **Kept in substance:** every hard limit, the Task Contract, the limits, the three outcomes, the end-of-task report, the invariants, the guard and fix-lock facts and owner decisions, the authority table, the stage notes, platforms, release claims, the experiment rule, critical-security escalation, and budget reporting.
  - **Newly written in:** the live-node limit and the safe-run rule. Before this, they were only in the verifier and the Build Records.
  - **Dropped as restatement of what an agent does by default:**
    - run `git status` and read the code first;
    - don't rely on a frozen inventory;
    - characterize before claiming a defect;
    - read only the relevant documents (kept as one sentence).
  - **Condensed:** the old §5 A–E and §6 into §5 and §6. The detail moved to the Standard's Per-Change Loop.
- [x] **Engineering Standard**, process block from "2. Standard Work" to the end of "Working with AI Assistants" rewritten: 357 → 245 lines for the whole document.
  - Purpose, the Foundation-0 operating profile, security assurance and the release gate are unchanged.
  - Every heading other files cite is kept: the Five Laws (Law 2), Phase 0, the Per-Change Loop with CLOSE OUT, the Scrap Taxonomy (Scrap Sweep), the Future-learning evidence rule, the Periodic Loops and the Incident Procedure.
  - The two 2026-10-13 dated clarifications are folded into "2. How changes are built" and "Risk tiers"; their text stays in Git history.
  - **Dropped:**
    - case studies;
    - Phase 0's examples;
    - the Scrap Taxonomy's narrative;
    - the Periodic Loops' detail beyond one line each;
    - the AI-assistant bullets that duplicate `AGENTS.md`.
  - **Kept as one-liners:** typed code, deploy equals build, no dependent work while a wiring gate is open, the structured-evidence list, and the wire, boundary and health tests.
- [x] **Citations:** `REVIEW.md` (§5, §7 "What the code must preserve", §3, the `#risk-tiers` anchor), `verifier.md` (§5, §6, "Risk tiers"), `.gitignore` (→ "Claude guard and fix-lock") and `scrap-sweep.sh`'s comment (→ Engineering Standard > Scrap Taxonomy).

## 3. Debug

Repair cycle 1 of 2, after verifier round 1 (FAIL; two Important findings and eight nits):

- **Important A, experiment safety dropped.** The old §6 required:
  - least-privilege access;
  - network denied by default unless a named destination is authorized;
  - isolation checked rather than trusted;
  - inspecting source, provenance, license, maintenance, dependencies and side effects before adoption.

  All four are restored in `AGENTS.md` §2, "Untrusted content".
- **Important B, deletion protection narrowed.** The old §2 forbade deleting or resetting any worktree, branch, stash or untracked file without authorization, and §5.E kept user data.
  - Restored in `AGENTS.md` §2, "Other work and user data": anything this task did not create; age or an unknown purpose is not deletion authority.
  - The Scrap Taxonomy now removes only stray files this task made. Its keep list includes user data, and it says an untracked file the task did not create is reported, not deleted. That is what `scrap-sweep.sh`'s comment cites.
- **Nits fixed:**
  - the line counts (`AGENTS.md` 111, the Standard 245, after the repairs);
  - `full-gate.sh`'s comment ("AGENTS.md, law 2" → "Engineering Standard, Law 2");
  - `doc/README.md`'s link text ("Agent Rules");
  - the Stage 00 completion rule and the time-boxed E1, E2 and probe, restored in §8;
  - "the owner's stop rule of 2026-10-10" and "a wording finding gets its fix, not a new gate", restored in "Risk tiers";
  - "revision" in recorded results, and budget renewal only by a newly authorized follow-up task, restored in §5;
  - "a small implementation choice does not need an ADR", restored in §8.
- **Nit left as backlog:** `verifier.md` step 2 triggers fallback and recovery for any change, while the Standard limits that to high-risk. This mismatch predates this change.

## 4. Verify

- [x] Links and anchors in the changed files plus the Architecture spec, the PRD, `doc/README.md` and `README.md`: 151 checked, 0 unresolved. **Pass**.
- [x] `git diff --check`: clean. **Pass**.
- [x] Stale section citations (`git grep` outside dated records): after repair, only the comment in `.github/workflows/claude-guard.yml` ("Deterministic gates") remains, left as PR #36 chose. **Pass**.
- [x] Verifier round 1: `VERDICT: FAIL` (Important A and B; dispositions in Debug). Its gate run: links 151/0, `git diff --check` clean, guard tests 107 OK. **Fail, repaired**.
- [x] Guard tests inside the outer safety layer: `Ran 107 tests in 8.751s`, OK, exit 0. **Pass**.

- [x] **Verifier follow-up (the one allowed): `VERDICT: PASS`.**
  - Both Important findings fixed, and seven of the eight nits.
  - Gate rerun: `git diff --check` exit 0; `bash -n` on both scripts exit 0; links 151/0; guard tests `Ran 107 tests in 8.713s`, OK.
  - Its one new nit, that Scope should name the files repair cycle 1 touched, is fixed in place.

## 5. Close out

- [x] Scrap sweep: the worktree holds only the files in Scope; the checkers and logs are in the session scratchpad.
- [x] Repair cycles: 1 of 2. Review rounds: 2 of 2.
- [x] Committed `2f8e4f3`, pushed, opened [PR #38](https://github.com/rikuripi/Rikuri-PI/pull/38).

## Results and limitations

**DONE locally.**

Limitations:
- CI is Not yet proven until the PR runs.
- Whether the shorter rules keep sessions careful while ending the loops is Not yet proven until a few tasks have run under them.
- Sessions already running keep the old text until they restart.

Backlog:
- `verifier.md` step 2 triggers fallback and recovery for every change, while the Standard limits that to high-risk.
- The `claude-guard.yml` comment cites "Deterministic gates".
- The node repository gets the same trim from a node session: `~/claude-wip/trim-node/HANDOFF.md`, after node PR #9 merges.
