# Build Record — sequence check and security-draft recheck after Option C

**Date:** October 2, 2026 · **Risk:** documentation only (brief record; touches protected planning and security documents at the owner's request) · **Branch:** `claude/project-thread-881i3t` into `codex/creative-studio-security-drafts`

## Scope

The owner asked three things after merging `main` into the drafts branch: whether the September 30 stage reshuffle still makes sense after the September 28 revisions, to revise the Stage Plan to follow the best sequence, and to recheck the October 1 security review. Foundation-0 stages 00–05, S/T/E IDs, gate states and dates are out of scope and unchanged.

## Plan and changes

- **Sequence check.** The reshuffle holds and Option C strengthens it: the old Stage 1 developer platform would have been the first credential- or network-permitted worker. Three refinements were found and recorded in the Master Build Sequence October 2 table:
  - S1.04 needs dependencies prepared outside the worker.
  - Stage 2 is reordered so S2.C1–C4 follow the basic window and Link items and S2.C5 precedes the general Plugin API (S2.C IDs unchanged).
  - Stage 6 pulls forward only worktree isolation for parallel code changes.
- **Stage Plan v2.1.** The post-Foundation summary, mermaid labels and pacing were updated, along with the S05.07 fallback (now Stage 8 developer pack), the Claude/Git importer rows and the expected broker stage (Stage 5 or Stage 8).
- **Security recheck.** An independent read-only pass compared the October 1 review, SEC-025–032, Architecture §27.14 and Test Strategy §17 with Option C. It found 6 important and 9 minor issues. The dispositions are applied in PRD §10, Architecture §27.14, Test Strategy §17 and the threat model's acceptance cells, and a dated recheck section was added to the review.
- **ADR 0000.** An October 2 dated clarification records both outcomes under the baseline freeze.

## Verification

- `python3 -m unittest discover -s .claude/hooks -p 'test_*.py'`: Pass (45 tests).
- Local link and anchor check over changed Markdown: Pass. `git diff --check`: Pass. Stage Plan S/T ID set identical before and after; the only checklist-line change is the S05.07 fallback text.
- Runtime tests: N/A, no executable behavior changes.

## Limitations

The recheck is a document comparison; no backend, broker or revocation behavior was exercised. Deeper editorial consolidation of SEC-025–032 into SEC-020–024 is optional and not done. Pacing estimates for Stages 1 and 2 are marked for re-estimation, not re-estimated.

## Disposition

Delivered as a PR into the drafts branch. The drafts branch is not merged to `main`.
