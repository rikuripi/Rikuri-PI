# Documentation consistency reconciliation — September 25, 2026

**Status:** Documentation review complete; implementation and security gates remain unproven.
**Scope:** All current planning, architecture, engineering, and acceptance documents in `doc/`. The historical 90-Day Build Plan is retained unchanged.
**Baseline reviewed:** Git commit `b684677`; user requested cross-document review and revision.

## Findings and changes

| Finding | Reconciliation |
| --- | --- |
| Current documents named Stage Plan v2.0; PRD/architecture footers named old versions | Point current references to v2.1 and correct footers, duplicate PRD introduction, stage-plan date, and architecture contents |
| Original ADR 0000 covered earlier versions | Preserve the original adopted decision; append a dated current-baseline and evidence-scaffold reconciliation |
| Foundation and product stage numbers could be confused | Define 00–05 versus product 0–10; explicitly require product Stage 4 evidence and Stage 6 governance for Collaborative Intelligence research |
| Architecture roadmap ended at Stage 9 | Align Stage 9 Windows parity and add Stage 10 with the PRD/master roadmap prerequisites and evidence gate |
| Writer “Foundation” could imply Foundation-0 scope | Map Writer to product Stage 8; clarify domain phases, later learned-intelligence dependencies, and governed canonical promotion |
| New scaffold obligations lacked named acceptance tests | Add T00.12, T02.16, T04.18 within existing stages and map them to existing build items; clarify unknown provider fields and privacy-governed evidence |
| Research document could require incubation scheduling now | Keep extension-friendly budget/cancellation contracts; defer the scheduler and autonomous learning |
| Test strategy omitted managed pre-forget recovery and knowledge compiler outcomes | Add current revocation/refusal case, ten expected answer-or-abstain fixtures, full release crash matrix, platform boundary and evidence-contract tests |
| Test strategy retained deferred importers and a missing image | Label Claude/Git fixtures for product Stage 1; replace missing image with readable test-layer text |
| ADR queue still used 13-week timing | Map to current stages, require backup ADR 0014 before Stage 01 F6, and include UI ADR 0013 in Stage 05 |
| Composite execution referenced nonexistent S4.5 | Add explicit product Stage 4 build item 15 and update deferral references |
| Deferral templates lacked fields required by their validator | Include disposition, dependency impact, owner, new stage/ordinal, and reactivation/acceptance condition |
| Engineering Standard named a nonexistent canonical path | Identify the current file and label the future implementation path as a planned move |

## Validation and limits

- Checked local Markdown links across README and every documentation file.
- Checked that Stage Plan build/test IDs are unique, contiguous within each stage, and that referenced build/test IDs resolve.
- Checked current version references, roadmap stage coverage, and preserved historical references.
- Checked the live gate board retains 13 rows and every row remains NOT STARTED.
- Checked `git diff --check`; reviewed the change diff.
- No executable implementation exists in this repository; no runtime, security, model-quality, or external research claim was validated by this documentation pass. The added tests are specifications, not passing test results.

Source version filenames remain stable. Git records the supplied originals and these revisions. Accepted ADR history is preserved; this reconciliation does not certify implementation or alter the six-stage Foundation-0 scope.
