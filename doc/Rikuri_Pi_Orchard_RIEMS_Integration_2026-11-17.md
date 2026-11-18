# Rikuri Orchard — RIEMS integration boundary

**Date:** November 17, 2026\
**Status:** Owner-approved future direction; design only, not implemented\
**Parent authority:** [Rikuri Pi ADR 0007](../docs/architecture/adr/0007-riems-outcome-driven-execution.md) proposed for repository adoption; implementation remains unproven.

## What changes, and what does not

RIEMS (Rikuri Intelligent Execution Management System) selects the **least complex adequate strategy for a verified outcome**, rather than optimizing agent count. Orchard is a software-project domain profile consuming Rikuri's RIEMS decisions, not an independent production scheduler, approval service, budget ledger, model router, vault or canonical ProjectState. RIEMS is layered over Rikuri's Capability Planner and existing execution profiles; the semantic scheduler controls dependency readiness, and the durable asynchronous runtime controls checkpoint, wait, resume, cancellation and recovery.

The current Orchard v0.3 remains a *bounded one-Codex-worker prototype*. There is **no** claim that RIEMS, parallel workers, optimization, machine scheduling or a durable production coordinator is implemented. This design change neither grants new delegated authority nor adds an Orchard milestone, task, spending authorization or automatic workflow.

## Proposed adoption gates

1. **Single-worker baseline first:** For representative coding requests capture objective, source revision, scoped permissions, time/cost estimates, applied result, task-specific acceptance, tests, rework and true time to verified outcome. Do not treat a successful worker turn as task acceptance or a reviewed patch as already applied.
2. **Choice of method:** Compare deterministic tooling, a capable single coding agent, and limited independent specialists only where a genuinely parallel dependency graph permits. Record expected coordination/integration cost and a strong baseline. A fixed 3-, 8-, or 20-agent crew is never a product success metric.
3. **Critical path and barriers:** Decide where frontend, backend, tests and integration can safely overlap. Block dependent or conflicting changes; wait for revision-specific evidence. Waiting agents should yield their active compute slots once the shared durable runtime exists.
4. **Adaptive decisions:** When the deadline or resource environment changes, diagnose the bottleneck before reallocating agents or models. Do not silently skip security/quality/approval gates. The owner can direct an eligible method or stop the job.
5. **Resource placement:** Shared compute via Rikuri Link/Mesh must admit machine, privacy, cost, identity, project and user-isolation policies. No implied pooled VRAM/RAM or default use of financial/Bitcoin node resources.
6. **Verification:** An RIEMS plan is successful only when independent acceptance gates show a useful deliverable, with elapsed time, total spend, failures and rework recorded in Rikuri-owned evidence. User-facing status remains truthful and reversible.

## Sequence and evidence

- **Orchard M2/M3:** Keep the existing review/apply contract, policy boundaries, reliability tests and one-worker defaults. Collect compatible evidence; do not launch a parallel team.
- **Rikuri product Stage 4:** Compare model/tool routing and measured single-worker strategies; learned selection remains gated.
- **Rikuri product Stage 6:** Qualify outcome-driven team selection, dependency-aware orchestration, and strategy replanning against deterministic/single-agent baselines.
- **Rikuri product Stage 7:** Qualify trusted mesh placement, fairness and privacy isolation separately.

**Failure gate:** A strategy that costs more, delays verification, increases conflicts, weakens privacy or cannot meet explicit acceptance must be rejected or returned to the user as a transparent tradeoff, not presented as a smarter workforce.

The authoritative execution state, permissions and gate evidence remain in Rikuri-controlled systems as they become available. Orchard's current SQLite/task state remains isolated prototype data until a reviewed, tested migration path is admitted.
