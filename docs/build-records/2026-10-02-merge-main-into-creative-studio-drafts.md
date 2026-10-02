# Build Record — merge main into the creative-studio and security drafts

**Date:** October 2, 2026 · **Risk:** documentation only (brief record) · **Branch:** `codex/creative-studio-security-drafts` brought up to date with `main` on `claude/project-thread-881i3t`

## Scope

The drafts branch (September 30 creative-studio direction, October 1 security draft review and Writer IDE spec, October 2 harness intent) was written on a `main` that predated the September 28 Option C, review follow-up and reference-installation addenda and the guard sync. This change merges `main` into it so the drafts can be reviewed against current documents. No decision is adopted or reversed here.

## Plan and conflict resolution

| File | Resolution |
| --- | --- |
| ADR 0000 | Kept every addendum from both sides, in date order: September 28 (three, from `main`), then September 30, October 1 and October 2 (from the drafts). |
| Architecture §27.1 | Took `main`: the threat model now exists at `docs/security/THREAT_MODEL.md`. |
| Architecture §27.12 | Both sides added a §27.12. `main`'s Option C section keeps §27.12/§27.13, which many documents link. The drafts' "October 1 security contract reconciliation" becomes §27.14; its stale sentence calling the threat model absent now links the maintained file. Links in PRD §10, Test Strategy §17, the doc index, the security review and its Build Record point to §27.14. |
| Master Build Sequence, Stage 1 | Took the drafts' resequenced Stage 1 and gate. Carried `main`'s Option C broker paragraph, which named the old Stage 1 items 5/8, as a rule for Stage 1 and later workers, pointing at the Stage 8 developer pack. `main`'s classification and Desktop Commander sentences were already folded into the drafts' S1.03 and adapter-comparison text. |

## Open items found, not resolved here (owner decisions)

1. **Requirement ID collision.** `main` defines SEC-020–024 (Option C personal-data protection) and the drafts define SEC-020–027 (security draft additions) with different meanings. Both tables now appear in PRD §10. They need renumbering or consolidation before merge.
2. **Baseline freeze.** The September 28 follow-up addendum froze the planning baseline and routed new ideas to `docs/PARKING_LOT.md`. The drafts resequence product Stages 1, 2, 5 and 8 through dated addenda; the owner should confirm that is intended under the freeze.
3. **Stage Plan drift.** Stage Plan v2.1 (protected) still describes product Stage 1 as the developer platform and Stage 2 as Monaco/terminal (handoff table), and S05.07 defers Monaco "to Stage 1". Under the drafts' sequence that work is Stage 8.
4. **Security review predates Option C.** The October 1 review compared the draft against documents without Option C, the threat model or the OpenShell receipt. Its dispositions should be rechecked against Architecture §27.12–27.13 and Test Strategy §8.2.

## Verification

- `python3 -m unittest discover -s .claude/hooks -p 'test_*.py'`: Pass (45 tests).
- Local link and anchor check over all changed Markdown: Pass (0 broken).
- `git diff --check`: Pass. No conflict markers remain.
- Runtime tests: N/A, no executable behavior changes.

## Disposition

Merged for review in a draft PR. Not merged to `main`; the four open items above wait on the owner.
