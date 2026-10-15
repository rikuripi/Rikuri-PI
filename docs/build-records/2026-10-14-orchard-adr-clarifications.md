# ADR 0006 clarification — 2026-10-14

## Task Contract

- **Outcome:** Make architectural adoption, implementation admission and completion semantics unambiguous.
- **Scope:** Routine documentation-only correction of the three review findings in ADR 0006; no adoption, runtime changes, migration, schema implementation or stage-gate changes.
- **Done when:** Adoption and implementation evidence are separate; maintained baselines replace obsolete operative references; historical Orchard DONE cannot become verified canonical acceptance; local links and whitespace checks pass.
- **Authority:** Owner approved the three recommended clarifications in chat. Edit this record and ADR 0006 only; no network, worker execution, commit, push or PR. Routine documentation-only change does not require another independent review round.
- **Limits:** One task; at most two repair cycles/two review rounds; no paid services.

## Review evidence

The prior independent verifier found no Important issues and three nits: combined adoption/implementation prerequisites, stale operative baseline references, and missing historical-DONE mapping. This task implements those corrections only. The original supplied proposal remains traceable through the integration record's source hashes.

## Disposition

ADR remains PROPOSED. Owner adoption and normal merge are distinct from approval to clarify text. Existing schema, fake-adapter and execution evidence requirements remain implementation admission requirements; no evidence is fabricated or waived.

## Verification

- PASS: read-back checks of the three clarified sections, PROPOSED status and maintained operative baseline references.
- PASS: all local ADR link targets exist. New headings have no incoming anchor links.
- PASS: `git diff --check`.
- CI/runtime tests: not run; documentation-only correction.
- Base revision: `13b58bdcc9c8a0a0e8219fff615d2679315d8431`. Changes remain uncommitted.
- DONE: the three approved clarifications are recorded; no implementation, adoption or execution started.
