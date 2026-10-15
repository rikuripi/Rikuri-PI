# ADR 0006 — Rikuri Orchard as the Software Project Domain Experience

**Date:** 2026-10-14\
**Status:** OWNER-ACCEPTED — architectural direction accepted October 14, 2026; normal merge pending; implementation admission evidence remains Not yet proven\
**Owners:** Rikuri Pi product owner / software architecture\
**Supersedes:** None. Preserves ADR 0000 and Foundation-0 Stage Plan v2.1.

## Owner acceptance — October 14, 2026

After the independent review and the three approved clarifications, the owner accepted this architectural direction. Acceptance establishes Orchard's domain role and Rikuri ownership; it authorizes no native experiment, production migration or stage-gate change. See the [acceptance Build Record](../../build-records/2026-10-14-orchard-direction-acceptance.md). Normal merge remains pending. The retained filename records the proposal's origin.

## Repository reconciliation — October 14, 2026

The current Stage Plan v2.1 and Engineering Standard v1.2 supersede the source proposal's v2.0/v1.1 references. Orchard is now v0.3 with a bounded Codex worker and separate review/apply/recovery workflow; O0 describes its historical starter. Current recorded evidence and limitations control, rather than the historical claim of no execution. The creative-studio direction and current Master Build Sequence retain authority over post-Foundation scheduling; this proposal does not restore IDE-first delivery. The maintained Architecture v1.7 includes later dated clarifications absent from the downloaded copy and is preserved.

See the [annotated integration plan](../../../doc/Rikuri_Pi_Orchard_Integration_Plan_2026-10-14.md) and [Build Record](../../build-records/2026-10-14-orchard-documentation-integration.md). Required evidence below remains a prerequisite, not evidence supplied by this documentation import.

## Context

Rikuri Pi PRD v1.2 and Architecture v1.7 already specify a Project Lifecycle Kernel (Frame → Shape → Plan → Execute → Verify → Deliver → Learn), one authoritative ProjectState, Pi-first harness architecture, AISecurity enforcement, a Monaco-centric developer capability layer, Sterling-style Workspaces, and evidence-first engineering. Orchard v0.3 retains its local prototype SQLite store and adds a bounded Codex worker with separate review/apply/recovery. First Seed v0.1 and its O0–O5 roadmap are historical context; postponing shared contracts until O5 would risk duplicate control planes. The current M0–M6 roadmap and recorded v0.3 evidence govern prototype implementation. The owner's observed Claude/`AGENTS.md` experience also shows that generic legacy self-verification instructions can be counterproductive as models and agent harnesses change.

## Decision

1. **One architecture:** Orchard is Rikuri Pi's software-project domain profile and user-facing factory experience. It never owns a second production ProjectState, trust authority, scheduler, ledger, knowledge store, quality engine or model router.
2. **Contract now, implementation later:** Specify common IDs, task/approval/verification contracts and owner boundaries immediately. The historical O1 milestone describes a bounded experimental harness adapter with isolation requiring actual evidence; it does not become a production integration or security claim before matching Rikuri stage gates pass. Reinterpret O5 as final UI/migration convergence, not first contract integration.
3. **One workbench:** Monaco is a first-class Rikuri Workspace window backed by common LSP/DAP/terminal/test/Git/diff services. Orchard invokes that surface; it does not fork an IDE.
4. **Shared operation semantics:** For capabilities Rikuri authors, user-visible state-changing operations have a common domain command engine exposed to approved GUI, CLI and agent/API interfaces, with security and evidence enforcement. External applications use tested native APIs/scripts/CLI/MCP or limited UI adapters; not all are forced into that design.
5. **Intent-led user experience:** Support Outcome ("just make it"), Collaborative ("help me"), and Directed ("my exact tools and workflow") interaction modes as choices per task rather than permanent skill labels. The user's intended result and acceptance criteria are model/app independent.
6. **Model-aware, bounded execution:** Maintain dated and tested model + native-agent-harness + runtime profiles and compile efficient per-task prompts from authoritative intent; keep `AGENTS.md` focused on project facts and nonredundant context. Keep required deterministic gates outside model self-assessment; stop at success, explicit blocker or retry/budget limit.
7. **Evidence and safety:** Never infer DONE from an agent's claim or a happy-path UI update. Durable receipts verify tool behavior, permissions, output semantics, and policy. No surprises in worktree/PR/dependency/deployment operations. Preserve transactional recovery, idempotency, privacy labels and no-silent-failure invariant.
8. **Foundation-0 scope protected:** This ADR adds future capability ownership and experimental seams, but does **not** increase Foundation-0's W1/F1–F10 gates, platform assurances, June 2027 target or completion definition. Changes to any protected promise require separate approved ADR/test revisions.

## Alternatives considered

- **Separate Orchard autonomous platform until O5:** faster freedom in the prototype, but duplicate databases/permissions/schedulers and costly later integration. Reject as production target; allow only bounded throwaway experiments.
- **Make Orchard its own IDE with Monaco as its chassis:** simplifies a software-only UX but conflicts with Rikuri's domain-neutral spatial Workspace and editor-as-window contract. Reject.
- **Use a single global AGENTS.md to standardize all models:** easy to start, but fragile under changing provider guidance, tool capability and model behavior. Reject as execution architecture.
- **Expand Foundation-0 to ship Orchard:** attractive visual progress but risks weakening the critical security and persistence gates. Reject without a separately evidenced rebaseline.

## Consequences

- Shared APIs and canonical ownership reduce duplicated engineering and support future writing, research and creative domain experiences.
- Initial contracts must be designed more carefully, though implementations can stay deliberately thin.
- The current Orchard prototype SQLite store and HTML remain valuable, but importing data into Rikuri requires explicit validated migration with provenance and rollback.
- Codex/Claude experiments are labeled by proven capabilities; one isolated successful test never establishes broad sandbox, model or OS security assurance.
- No visual status may claim stronger completion than underlying evidence. Owner sees simple outcomes and a drill-down into technical receipts.

## Architectural adoption prerequisites

- Review the ownership matrix against maintained PRD v1.2, Architecture v1.7 (including dated clarifications), ADR 0000, Foundation-0 Stage Plan v2.1 and Engineering Standard v1.2. Preserve the current Master Build Sequence and creative-studio direction.
- Owner accepts Orchard as the software-project domain over shared Rikuri services, with the completion distinction below and unchanged Foundation-0 commitments; record acceptance and merge through normal change control.

Architectural adoption establishes ownership and direction. It does not authorize execution, import prototype state or attest to implemented contracts. Schema tests, adapter traces and a live spike are implementation-admission evidence, not prerequisites to deciding this direction.

## Evidence before implementation admission

- Provider-neutral TaskContract with stable IDs, allowed scope, capabilities, acceptance, independent gate, stop and terminal states; schema/negative tests.
- One fake-adapter trace from intent to verified artifact and one denial/failure trace, with no false DONE.
- Run the current relevant Orchard v0.3 regression suite, including retained Seed Box tests, worker boundaries and review/apply/recovery checks. Four historical O0 tests alone are insufficient. No existing Rikuri protected gate changes without documented ADR reconciliation.
- Separately authorize a bounded native-adapter experiment demonstrating real changes, task-specific acceptance, evidence and finite termination, with tested containment for the selected environment; no automatic follow-on work. The historical O1 label does not authorize a new run.

Existing v0.3 verification records provide limited evidence of a real heading task, native checks and authored review/apply fixtures. They may support matching claims after checking source/runtime versions and applicability; they do not prove a provider-neutral contract, arbitrary task acceptance or production integration. Record missing evidence as Not yet proven.

## Completion and historical status mapping

Execution, acceptance and application are separate outcomes. Existing Orchard `DONE` receipts mean a completed worker turn and passing project-test gates, as limited by its README and verification record. Preserve those original receipts and their provenance; map them to **execution completed**, not canonical verified task completion. Historical task acceptance remains **UNVERIFIED** unless separately evidenced; application is recorded only from an actual apply receipt.

Canonical task `DONE` requires task-specific acceptance evidence and every applicable mandatory gate to pass. A verified outcome may still be unapplied; an applied artifact does not prove acceptance. Missing evidence remains UNVERIFIED, and a failed mandatory check prevents canonical DONE. Any importer must test this mapping and cannot retroactively strengthen historical claims.

## Rollback

If integration contracts are proven inadequate, supersede or withdraw this direction through owner-reviewed change control, retain the current Orchard prototype and its existing data/evidence as a stand-alone nonproduction baseline, revert only new schema/docs changes via reviewed commit, preserve previous Rikuri trusted state and active Foundation-0 gate plan, then run a revised comparative spike before re-proposal.
