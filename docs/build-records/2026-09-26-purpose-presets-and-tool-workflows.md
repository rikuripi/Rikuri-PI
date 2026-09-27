# Build Record — Purpose presets and governed tool workflows

| Field | Value |
|---|---|
| Date | 2026-09-26 |
| Change | Make purpose-driven configurable capability bundles, tool choice and understandable results explicit in existing product documents |
| Author | Codex |
| Branch | main; no commit/push requested |
| Status | COMPLETE — documentation verified, not implementation or owner approval |
| Requirements | New PRD §9.22; Architecture §21A.3 / §25; Collaborative Intelligence §6 |
| Stage gate | None changed; Foundation-0 remains existing evidence scaffold only |

## Plan and characterization

Owner requested documenting this conversation's purpose bundles, customization/reset and plugin/MCP choices. Current documents already contain CognitiveRecipe, capability descriptors, Connector Gateway and progressive disclosure, but lack explicit preset layering/reset semantics and the complete user-facing tool/method workflow.

Preserve prior uncommitted work: four doc edits and decision-procedures Build Record from the preceding task, plus unrelated untracked Rikuri Pi.code-workspace. Planned edits: doc/README.md, PRD v1.2, Architecture v1.7, Collaborative Intelligence v0.1, and this new record. No other files or historic records modified.

Success: clear requirements for presets and overrides; isolated/joint context; permission-safe reset and plugin selection; exact versus statistical versus semantic tools; evidence-gap assistance; accessible explanations with expert depth; capability evaluation and limitations. Validate local links/anchors, unique IDs, version/stage references, whitespace and independent review.

## Risk, caller and wiring

Documentation only; no deployed tool, plugin, data flow or application integration. Preserve canonical authority, permissions, Local Only, cost gates and separated approval/verification. No protected policy changes. Named commercial tools are illustrative candidates, not advertised integrations. Method selection does not authorize execution or external collection.

Wiring: PRD §9.22 → Architecture capability registry / CognitiveRecipe / Connector Gateway → Collaborative Intelligence §6.4–6.7. Existing product components retain their stage order; integrated decision-recipe research depends on product Stage 4 evidence and Stage 6 governance, with Stage 10 productization. Foundation S00.04–S00.08, S02.02/S02.07, S04.08/S04.14/S04.15 and T00.12/T02.16/T04.18 stay unchanged.

## Disposition

Runtime, migrations, unit tests for new behavior, recovery drills, module contracts and implemented help/RAG: N/A, documentation only. Stage 00 full-gate/smoke are absent and Not yet proven. Future implementation owner: Henry/product owner; admission at the existing relevant product stages, integrated recipes at Stage 10 after Stage 4/6 prerequisites, with the negative/evaluation cases specified in the docs. No new foundation services, gate closures or schedules.

## Verification and close-out

- PASS: local Markdown path/heading validator resolved 24 links/anchors across four modified docs and this record; PRE-001–009 are unique.
- PASS: `git diff --check` (exit 0).
- PASS: `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s .claude/hooks -p 'test_*.py'` (16 tests, exit 0); refusal/lock messages are expected fixture output.
- PASS: inspected additions and final status; existing decision/VPS additions, previous Build Record and unrelated workspace file preserved. Source version filenames and existing stage/S/T references remain unchanged.
- Scrap Sweep: no repository temporary probes, generated files or replaced code; no deletions required.
- Independent verifier `/root/verify_docs`: PASS. Independently confirmed 24 local links/anchors, 16 guard tests, whitespace, unique requirements, stage scope and fix-lock not engaged. Bugs / Security & boundaries / Compliance: no Important or Nit findings. GitHub CI not observed; review is evidence, not owner approval.
- No staging, commit, push, tool installation, external collection or execution performed.

## Publication authorization

After verification, the owner explicitly requested “commit and push” on September 26, 2026. The earlier no-commit/no-push notes describe the state before this authorization. Publish both related documentation increments together; preserve the unrelated workspace file. The containing Git commit is the durable revision identifier.
