# Build Record — Risk tiers: set how far a change goes by what it can break

| Field | Value |
|---|---|
| Change | Add two risk tiers (routine, high-risk) that set the depth of tests, sibling sweeps, class tripwires and independent review, and give the review a finish line, so a session stops when its change is good enough for its risk instead of hunting. |
| Date | 2026-10-13 |
| Author / agent session | Claude Code (Opus 5.5) for Henry Chen |
| Branch | `docs/risk-tiers`, worktree `/home/junc/claude-wip/risk-tiers/rpi`, from `origin/main` `2e15f53` (the main checkout stays on `docs/bounded-agent-standard`, untouched) |
| Commits | `81f141a`, the change (owner, 2026-10-13: "Yes, commit push and open PR"); a second commit records this row, the status and the PR |
| Stage Gate | N/A: working rules; no Stage 00 item, interface or gate status changes |
| Related ADR / requirement | Owner, 2026-10-13, in the session that made this change: BUILD-PROCEDURE "too strict … every scaffolding seems to last forever … every little bug fix goes to hunt"; "Rikuri-PI is the one that seems to go into an endless hunt/loop"; approved the risk-tier proposal ("Ok, yes"). `AGENTS.md` §1, §5; the Engineering Standard's "October 13, 2026 — bounded agent operation" clarification; the owner's stop rule (2026-10-10, E1 record line 12) |
| Status | DONE locally; pushed; [PR #37](https://github.com/rikuripi/Rikuri-PI/pull/37) open; CI Not yet proven until a run is observed; the owner merges |

**Risk note (protected paths):** `AGENTS.md`, `REVIEW.md`, `.claude/agents/verifier.md` and the Engineering Standard are protected paths. Guard, settings, CI and policy files are not touched. The change loosens review in seven places and tightens it in one; all eight are listed under "Owner choices" below, and each reverts by deleting the added text. The most consequential loosening is choice 2: a new Important finding seen in a follow-up pass goes to the owner instead of being fixed in the task.

## Task Contract

1. **Outcome:** A routine change finishes in one pass; a high-risk change keeps its full evidence but reaches a defined finish line; a review stops failing on wording and on nits.
2. **Scope:** a dated addendum "October 13, 2026 — risk tiers" in `doc/Rikuri_Pi_Engineering_Standard_v1.2.md` (additions only); `REVIEW.md` (claim severity, test gaps, nits, the follow-up pass); `.claude/agents/verifier.md` (tier, mutation run for high-risk Rust code, verdict and follow-up rules); one clause in `AGENTS.md` §1 item 2 naming the tier. Tier of this change: **high-risk** (it changes the review and verification rules). **Not included:** the guard, its policy, settings, CI, the Stage Plan, the Test Strategy, the Templates, the node repository (its BUILD-PROCEDURE.md follows as a separate task once its open pull request lands).
3. **Done when:** every local link and anchor in the changed files resolves; `git diff --check` is clean; the guard tests pass inside the outer safety layer (`.claude/` changes); no conflict with the Test Strategy or the Templates; one verifier pass, or its follow-up, with no Important finding open.
4. **Authority:** Read, Edit, Write and Bash for checks in this worktree; no commit, push, PR or merge without the owner.
5. **Limits:** one task; two repair cycles; one verifier pass plus at most one follow-up. No time or token budget is configured; consumption is reported.

## 1. Plan (characterization)

- [x] **Where the loops come from (evidence on `main`).**
  - Crash matrix (`2026-10-08-stage-00-crash-matrix.md`): verifier rounds 1–5 failed with "no runtime defect" (lines 43–52), each round finding further conditions that no test isolated. cargo-mutants was installed after round 4 because "each found the next hand-missed condition" (line 29).
  - E1 (`2026-10-09-s00-11-e1-tse-spike.md`): rounds 2–4 failed on real Important defects in the protected check, which let planted wrong candidates through (for example D8, another task's privacy class rewritten unseen; lines 126–170). Rounds 5 and 6 failed "at Minor only"; round 7 ended E1 under the owner's stop rule (line 12). cargo-mutants already ran in rounds 3–4 with every compiling mutant caught but the one deferred `Store::health` fallback (lines 212, 218), so those rounds failed on missing rules, which mutation testing cannot find.
  - W1 placement: 2 rounds. The node repository's bootstrap: 18 rounds.
  - So two patterns were left unbounded: hand-found untested conditions, one per round, and failing on Minor findings. Real defects in protected code, E1's rounds 2–4, are what review is for, and this change keeps them Important.
- [x] **Nothing requires the extra tests.** Mutation testing is in no gate, CI job, Test Strategy or Template (`git grep -i mutant` over `scripts`, `.github` and the three documents: no requirement).
- [x] **What the rules said.**
  - Engineering Standard: Law 3 "walked mechanically, every time, regardless of how small"; Law 5 "encode every lesson as a gate"; Debug "fix the CLASS … sweep" (already bounded to the task by the bounded-operation clarification).
  - `AGENTS.md` §5.A scales the Build Record by risk but nothing else.
  - `REVIEW.md`: "says something exists, works, or passed without linked evidence is **Important**" (wider than its own Important list); severities are Important and Nit only.
  - `verifier.md`: no rule on nits or on what a follow-up may raise.
- [x] **The fix-lock is not the Rikuri-PI problem now.**
  - `.claude/state/fix-lock.log` (by `engaged_since`): 10 engagements on 2026-10-08, 9 on 10-10, none after. The lock is opt-in since 2026-10-10.
  - Release asks in `guard-relaxed.log`: 19 on 10-08 (14 from Rikuri-PI, 5 from `/home/junc/rikuri/rikurinode`); 3 since 10-11.
  - No change needed here.
- [x] **Tool check.** `cargo mutants --version` → `cargo-mutants 27.1.0`; `--in-diff <IN_DIFF>`: "Include only mutants in code touched by this diff". The established settings are in the crash-matrix and W1 records: short `TMPDIR`, `--test-workspace=true`, `--timeout 600`, `--output` in scratch. The `crash-injection` feature belongs to `rikurid` (`apps/rikurid/Cargo.toml`; CI: `-p rikurid --features crash-injection`).
- [x] **I will know this works when** the "Done when" line holds, and a reader of the four files can say, for any change, which tier it is, what its review checks, and when the review is over.

## 2. Build

- [x] `doc/Rikuri_Pi_Engineering_Standard_v1.2.md`: "October 13, 2026 — risk tiers" after the bounded-operation clarification. Additions only. It covers:
  - the two tiers and how to pick one, kept distinct from the guard's protected paths;
  - a table of what each tier gets;
  - the review's finish line (Important fails a pass, nits never do; claim severity narrowed);
  - test adequacy: a mutation run for high-risk Rust code, with what it cannot find;
  - the follow-up pass, which checks corrections, sends a new Important finding to the owner and starts no third round;
  - Laws 3 and 5 read with the tiers;
  - controls adequate when they cover what the repository uses.
- [x] `REVIEW.md`:
  - the tier sets the depth;
  - "Claims match evidence" narrowed to gate or stage results, security, data integrity and completion;
  - two test gaps named as Important;
  - a review fails only on an Important finding;
  - surviving mutants count as one finding in the nit cap;
  - a new section, "The follow-up pass".
- [x] `.claude/agents/verifier.md`:
  - read the tier; verify at the high-risk depth when it is missing or wrong;
  - a new step 5: one mutation run on the changed lines for high-risk Rust code, with the established settings and the unviable-list check;
  - mutation testing does not find missing rules; no hand hunting for further untested conditions;
  - follow-up and severity rules;
  - `VERDICT: PASS` means no Important finding open.
- [x] `AGENTS.md` §1 item 2: the Task Contract's Scope names the risk tier, linked to the addendum. This is the only edit to the owner's text and reverts in one line.

## 3. Debug

Repair cycle 1 of 2, after verifier round 1 (FAIL; findings below). Every finding was fixed in one cycle:

- **I1, Important:** the follow-up's finish line was undefined.
  - Cause: the first draft used "Critical" and "Minor", which `REVIEW.md` does not define, and its PASS rule contradicted its backlog rule for a new Important finding.
  - Fixed: one scale, `REVIEW.md`'s Important or nit, in all four files. A new Important finding in a follow-up is a FAIL and goes to the owner (`NEEDS_OWNER_DECISION`), never to a third round.
- **I2, Important:** claims about what mutation testing fixes were contradicted by the records.
  - Fixed: the addendum's premise now says E1's rounds 2–4 failed on real defects and only rounds 5–6 on Minor findings.
  - Mutation runs are credited only with the crash-matrix pattern; the addendum and the verifier say they do not find missing rules.
  - The verifier's "every untested condition" is now "the untested conditions the tool can reach".
- **Nit N1, loosenings not all exposed:** every loosening is now listed under "Owner choices". The routine fix-lock cell now says opt-in, as `AGENTS.md` §4 does.
- **Nit N2, routine review disagreed between files:** routine code gets one verifier pass. Routine documentation gets the §5.D checks and the owner's pull-request review. Either tier may have one follow-up (`AGENTS.md` §5.D).
- **Nit N3, the table contradicted itself on regression tests:** every fixed defect gets its regression test in both tiers. The tiered row is the class tripwire beyond it, and the incident tripwire applies to both tiers.
- **Nit N4, the mutation step was unworkable as written:**
  - now limited to Rust;
  - carries the established settings, with the feature qualified as `rikurid/crash-injection`;
  - checks the unviable list;
  - other-language high-risk code is judged by its own tests and "Done when".
- **Nit N5, factual errors:** the fix-lock counts are corrected above. The adequacy rule is cited from the Stage 00 gate record (line 95), where it was already recorded.
- **Other nits:**
  - the nit cap against "report every survivor" is settled: survivors count as one finding;
  - the tier is renamed **high-risk**, so it is not confused with the guard's "protected path";
  - the high-risk review cell now limits the mutation run to Rust code.

## 4. Verify

Tier: high-risk (review and verification rules). This is a documentation and `.claude/` change, so the local gate is the `AGENTS.md` §5.D checks plus the guard tests. No mutation run applies, because no code changed. CI's full gate runs on the pull request.

- [x] **Verifier round 1:** `VERDICT: FAIL`, with two Important findings (I1, I2) and nine nits. The gate was clean (`git diff --check`; 20/20 links; guard tests `Ran 107 tests in 8.762s`, OK, exit 0, inside safe-run, whose `safe-run.sh` matched the node repo's at `a064741` byte for byte). Dispositions are in Debug.
- [x] After repair cycle 1: links and anchors 21 checked, 0 unresolved; `git diff --check` clean; guard tests inside safe-run `Ran 107 tests in 8.882s`, OK, exit 0. **Pass**.
- [x] **Verifier follow-up (the one allowed): `VERDICT: PASS`.** It found I1, I2, N1–N5 and the four other nits corrected, and reran the gate: `git diff --check` exit 0; links 21/0; guard tests `Ran 107 tests in 8.892s`, OK, exit 0, inside the same `safe-run.sh`. It left three nits, fixed in place with no further round (`REVIEW.md`, as changed here):
  - "every compiling mutant caught" now names the one deferred `Store::health` fallback that survived (E1 lines 212, 218);
  - "a protected check" for E1 now reads "the candidate check it built", so it is not confused with the tier;
  - the nit cap now says survivors rated nits count as one nit, and each survivor rated Important is its own finding.
- [x] Real entry point: N/A. These are documentation and agent-instruction changes, with no executable behaviour. The rules take effect in sessions started after they reach `main`.

## 5. Close out

- [x] Scrap sweep: the worktree holds only the five files in Scope. The checkers and test logs are in this session's scratchpad, not the repository.
- [x] Repair cycles used: 1 of 2. Review rounds used: 2 of 2 (FAIL, then PASS on the follow-up).
- [x] Committed `81f141a`, pushed `docs/risk-tiers`, opened [PR #37](https://github.com/rikuripi/Rikuri-PI/pull/37) at the owner's request (2026-10-13).

## Owner choices this change exposes

Each one reverts by deleting the added text.

1. **Nits never fail a pass** (the owner's 2026-10-10 stop rule, written down). Cost: nits can merge as listed rather than fixed.
2. **A follow-up pass does not search for new findings.** A new Important finding it sees goes to the owner, not to a third round. Cost: the owner gets those decisions; a defect a third round would have found may wait for its own task.
3. **High-risk Rust code gets a mutation run in the first verifier pass** (the one tightening). Cost: minutes to tens of minutes per high-risk change, inside the safety layer; E1's whole-crate runs needed a 38 GB scratch, and `--in-diff` limits the run to changed lines. Benefit: the untested conditions the tool can reach surface together, instead of one per round as in the crash matrix. It finds no missing rules.
4. **Routine documentation changes get no verifier subagent**, only the §5.D checks and the owner's pull-request review. Cost: wording errors are found later.
5. **Claim severity narrowed** to gate or stage results, security, data integrity and completion. Cost: other overstatements are nits.
6. **Routine class sweep limited to the module touched.** Cost: a sibling elsewhere waits until it is seen.
7. **Routine class tripwire only for a defect seen twice** (the regression test is always kept). Cost: a first routine defect gets no class-wide check.
8. **A missing test in routine code is Important only when "Done when" depends on it.** Cost: thinner tests for low-risk code.

## Backlog (seen, not pursued)

- `docs/stage-gates/foundation0/00-bootstrap-kernel.md` line 95 still gives "the fix-lock becomes the default again" as a reactivation condition, while `AGENTS.md` (2026-10-13) makes the lock permanently opt-in. This was already disclosed as an owner decision in PR #36's record.
- `AGENTS.md` §5.A's list of changes that need a full Build Record omits the controls (CI, guard, `REVIEW.md`, the verifier), which the addendum puts in the high-risk tier. The addendum governs; aligning §5.A is a one-clause owner edit.
- The node repository's BUILD-PROCEDURE.md and REVIEW.md: the same tiers, as their own task.

## Results and limitations

**DONE locally.** All "Done when" checks pass, and the verifier's follow-up is PASS with no Important finding open.

Limitations:
- CI (`claude-guard.yml`, the full gate) is Not yet proven until the pull request runs.
- The new rules have not yet been used on a real change. Whether they shorten review without letting defects through is Not yet proven until a few routine and high-risk changes have run under them.
- Sessions already running keep the old text until they restart.
