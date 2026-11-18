# Documentation index

These are maintained revisions of the supplied project documents. Their original filenames and version labels are preserved. Superseded versions are retained in Git history rather than the current checkout.

Coding agents start with the [Agent Rules](../AGENTS.md). Use the Engineering Templates below for Build Records rather than maintaining a second checklist.

## Mission

[Mission Charter v1.0](Rikuri_Pi_Mission_Charter_v1.0.md) — why Rikuri exists: expand what a person can imagine, understand, create and accomplish, free of any one model, OS or ecosystem. It holds the decision test for competing approaches (§12). It is rationale, not requirements: the PRD and Architecture below govern, and the live Stage Plan controls Foundation-0.

## Current Foundation-0 baseline

| Document | Role |
| --- | --- |
| [Foundation-0 Re-baselined Stage Plan v2.1](Rikuri_Pi_Foundation0_Rebaselined_Stage_Plan_v2.1.md) | **Live** six-stage sequence, item IDs, acceptance tests, experiments, and gate board. Start here. |
| [ADR 0000 — Scope Reconciliation](Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md) | Adopted decision separating the Foundation-0 commitments from the broader product and historical 90-day plan. |
| [PRD v1.2](Rikuri_Pi_PRD_v1.2.md) | Product vision and requirements, including later releases. A product-level P0 is not automatically due in Foundation-0. |
| [Architecture Spec v1.7](Rikuri_Pi_Architecture_Spec_v1.7.md) | Architecture baseline; the live stage plan controls Foundation-0 sequence and evidence. |

The September 25 update set has been reconciled in-repository for version, scope, roadmap, and acceptance consistency. Source version filenames are retained; Git history records the supplied originals and subsequent corrections. ADR 0000 retains its original decision with a dated addendum for the current baseline. See [the reconciliation record](reviews/2026-09-25-documentation-consistency.md) for changes and validation.

Foundation-0 stages use two digits (00–05). Product stages use unpadded numbers (0–10); product Stage 0 contains all six Foundation-0 stages. Product Stage 4 is Model Lab, not Foundation-0 Stage 04 (Pi/local coding). Writer phases are a separate domain roadmap.

## Security direction and evidence

The September 28 **Option C** decision makes personal-data protection the highest product responsibility. Rikuri owns permissions; trusted AISecurity policy evaluation and qualified enforcement adapters apply them. OpenShell is the first Ubuntu candidate. [PRD SEC-020–024](Rikuri_Pi_PRD_v1.2.md#10-security-and-trust-requirements), [Architecture §27.12–27.13](Rikuri_Pi_Architecture_Spec_v1.7.md#2712-option-c-policy-ownership-and-enforcement-adapters), the [ADR addendum](Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md#september-28-2026--option-c-and-personal-data-protection) and [Test Strategy §8.2](Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md#82-option-c-qualification-and-personal-data-acceptance) define authority, fail-closed operation, private-data lifecycle and admission evidence.

- [Personal-data threat model](../docs/security/THREAT_MODEL.md): assets, trusted boundaries, exclusions and existing-stage test mapping; a companion to the authoritative baseline, not another roadmap.
- [OpenShell v0.1.2 containment receipt](../docs/security/evidence/2026-09-28-openshell-v0.1.2/README.md): ten synthetic smoke outcomes on this host; full qualification and Rikuri integration remain unproven.
- [Security documentation Build Record](../docs/build-records/2026-09-28-option-c-security-and-personal-data.md): changes, review and limitations.

No Foundation gate is marked complete. Native macOS remains experimental; Sentry is an optional research reference, with no required hardware layer.

The [September 28 review follow-ups](Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md#september-28-2026--option-c-review-follow-ups-and-foundation-0-credential-scope) disable OpenShell's own approval channel for workers, keep container-engine access away from Rikuri's processes, require E2 to decide where the Pi harness runs with all model calls through `rikurid`, keep Foundation-0 workers credential-free with no permitted destination (owner choice; Stage 03 dates unchanged), and freeze the baseline. New ideas go to the [parking lot](../docs/PARKING_LOT.md). Stage progress is tracked in [Stage Gate Records](../docs/stage-gates/foundation0/00-bootstrap-kernel.md), which link the live plan. See the [Build Record](../docs/build-records/2026-09-28-option-c-review-followups.md).

## Research and future architecture

| Document | Role |
| --- | --- |
| [Collaborative Intelligence / Inquiry & Discovery Architecture v0.1](Rikuri_Pi_Collaborative_Intelligence_Architecture_v0.1.md) | Long-term product North Star and post-Foundation research baseline, including evidence-based decision procedures, ACH safeguards, and business/fundraising evidence; not a Foundation-0 implementation commitment. |
| [Writer / Creative Authoring Architecture v1.4](Rikuri_Pi_Writer_Creative_Authoring_Architecture_v1.4.md) | Human-first creative authoring architecture and roadmap. |

The September 26 decision-support clarification links PRD §9.21 to Architecture §21A and Collaborative Intelligence §8. Architecture §38.5 describes optional VPS placement and privacy admission. Existing Foundation-0 evidence hooks accommodate future work; no new implementation gate or assurance claim is added. See [the change record](../docs/build-records/2026-09-26-decision-procedures-and-vps-boundaries.md).

Purpose-driven customizable presets and plugin/MCP tool choices are specified in PRD §9.22, Architecture §21A.3/§25.7, and [Collaborative Intelligence §6.4–6.7](Rikuri_Pi_Collaborative_Intelligence_Architecture_v0.1.md#64-purpose-driven-capability-presets), including safe reset, method selection, evidence-gap guidance and novice/expert explanations. See [the preset change record](../docs/build-records/2026-09-26-purpose-presets-and-tool-workflows.md).

The September 27 context-delivery contract specifies mandatory knowledge discovery and tool wiring on model/session changes, immediate retrieval on topic changes or intersections, versioned task packets, source freshness/coverage and measurable delivery evidence. [Architecture §19.5–19.10](Rikuri_Pi_Architecture_Spec_v1.7.md#195-context-delivery-lifecycle) owns the contract, [PRD §9.2](Rikuri_Pi_PRD_v1.2.md#92-context-and-memory-engine) owns MEM-007–012, and [Test Strategy §6.2](Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md#62-context-delivery-and-continuity) maps the acceptance cases. This is unimplemented design; existing Foundation gates and later connector/media stages are preserved. See [the context-delivery change record](../docs/build-records/2026-09-27-context-delivery-and-model-continuity.md).

The September 27 scoped-autonomy design adds understandable development/staging/live presets, expiring task grants, unattended readiness, independent progress during approval waits, and defect-scoped regression evidence. [Architecture §16.5–16.9](Rikuri_Pi_Architecture_Spec_v1.7.md#165-scoped-autonomy-and-user-presets), [PRD security requirements SEC-013–019](Rikuri_Pi_PRD_v1.2.md#10-security-and-trust-requirements) and [future acceptance cases](Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md#14-post-foundation-autonomy-and-repair-acceptance) distinguish development, verification and live activation. Automatic repair-cycle completion is prospective and requires policy reconciliation; current owner-only fix-lock release, the shared guard and verifier rules remain unchanged. The user's current external project is deferred. See [the autonomy change record](../docs/build-records/2026-09-27-scoped-autonomy-and-repair-cycles.md).

## Engineering and acceptance references

The September 28 adopted clarification preserves versioned user notes and edited generated Markdown before regeneration, with flags and safe refusal; [Architecture §20.3.1](Rikuri_Pi_Architecture_Spec_v1.7.md#2031-revision-history-and-edited-projections) and PRD KNOW-005/011 retain two one-way flows. [Architecture §20.6](Rikuri_Pi_Architecture_Spec_v1.7.md#206-capture-classification-and-conversational-evidence) adds staged capture, correctable classification and conversational source cards. [§38.6](Rikuri_Pi_Architecture_Spec_v1.7.md#386-shared-humanai-project-conversations) defines shared human–AI conversations with audience/history controls, authenticated LAN/remote access, and Tailscale first followed by native encrypted direct/relay with Tailscale fallback. Minimum team controls move from product Stage 9 to [Stage 6 S6.C1–C4](Rikuri_Pi_Master_Build_Sequence_v1.1.md#stage-6--multi-agent-governance--project-graph), after transport, source and governance prerequisites; enterprise administration remains Stage 9. Foundation S01.08/T01.16 only clarifies projection preservation, with no gate/status change. [Test Strategy §6.3](Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md#63-document-revisions-and-projection-preservation) and [§15](Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md#15-post-foundation-capture-and-shared-conversation-acceptance) define unproven acceptance; the [ADR addendum](Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md#september-28-2026--document-history-and-shared-conversation-clarification) and [Build Record](../docs/build-records/2026-09-28-knowledge-capture-and-team-conversations.md) record scope and verification.

| Document | Role |
| --- | --- |
| [Engineering Standard v1.2](Rikuri_Pi_Engineering_Standard_v1.2.md) | Per-change procedure, bootstrap practices, and release discipline. |
| [Engineering Templates v1.1](Rikuri_Pi_Engineering_Templates_v1.1.md) | Build Records, Stage Gate Records, Module Contracts, traceability, debt, and incident templates. |
| [Foundation-0 Test, Replay & Acceptance Strategy v1.2](Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md) | Test layers, replay fixtures, crash matrix, and evidence model supporting the live gates. |
| [Foundation-0 ADR Starter Pack v1.0](Rikuri_Pi_Foundation0_ADR_Starter_Pack_v1.0.md) | ADR template and initial decision queue; use the live plan for current stage timing and ordinals. |

## Sequence context

| Document | Role |
| --- | --- |
| [Master Build Sequence v1.1](Rikuri_Pi_Master_Build_Sequence_v1.1.md) | Dependency order for Foundation-0 and later stages; points to the live plan for Foundation-0 detail. |
| [90-Day Build Plan v1.6](Rikuri_Pi_90_Day_Build_Plan_v1.6.md) | **Historical** 13-week plan. Retained for rationale and traceability; it is not a second live gate board. |

Foundation-0 gates are not complete merely because the documents are present. Record Pass, Fail, or Not yet proven with reproducible evidence as each stage is implemented.

## September 30 creative-studio direction

The lead product is now a modular creative studio powered by the trusted Personal Intelligence substrate. [PRD STUDIO-001–010](Rikuri_Pi_PRD_v1.2.md#27-september-30-creative-studio-requirements) specifies authorship, scenario packets, Pi collaborators, disciplined reflection, source coverage, portability and production handoff. [Writer](Rikuri_Pi_Writer_Creative_Authoring_Architecture_v1.4.md) preserves its detailed creative architecture; [Collaborative Intelligence](Rikuri_Pi_Collaborative_Intelligence_Architecture_v0.1.md) retains its full reflection/abstraction design and adds a systematic cross-disciplinary pipeline.

[Master Build Sequence](Rikuri_Pi_Master_Build_Sequence_v1.1.md) brings essential Writer work to Stage 2 S2.C1–C4 and moves full native IDE depth to the optional Stage 8 pack; its September 30 dispositions supersede the earlier IDE-first/Writer-at-8 summary. Foundation 00–05 is unchanged. Requested critique differs from Stage 10 learned reflection; evidence/governance prerequisites remain. [ADR clarification](Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md#september-30-2026--creative-studio-direction-and-sequencing), [future acceptance §16](Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md#16-post-foundation-creative-studio-acceptance) and [Build Record](../docs/build-records/2026-09-30-creative-studio-direction.md) document the decision. All runtime capability remains unproven.

## October 1 security draft review

The supplied Security Architecture v1.0 has been [compared with the maintained security design](reviews/2026-10-01-security-draft-review.md). [PRD §10](Rikuri_Pi_PRD_v1.2.md#10-security-and-trust-requirements) folds the draft's refinements into SEC-020–024 and adds SEC-028–031, [Architecture §27.14](Rikuri_Pi_Architecture_Spec_v1.7.md#2714-october-1-security-contract-reconciliation) reconciles vocabulary and boundaries, and [Test Strategy §17](Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md#17-security-draft-addition-acceptance) defines future acceptance. The draft is not adopted as a second roadmap; Foundation gates and creative-studio direction remain unchanged. Secret exclusion, Local Only assurance, movement permission and revocation semantics control over looser draft wording. [Build Record](../docs/build-records/2026-10-01-security-draft-review.md) records validation; runtime assurance remains unproven.

## October 1 Writer's IDE specification

[Writer IDE Specification v0.1](Rikuri_Pi_Writer_IDE_Spec_v0.1.md) refines the [Writer architecture](Rikuri_Pi_Writer_Creative_Authoring_Architecture_v1.4.md) and PRD §28 with WIDE-001–012: stable sentence/paragraph/story-unit identity, versioned work, human authorship/AI craft roles, contextual questions, optional lessons/flashcards, scholarly references and explicit collaboration terms. [ADR clarification](Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md#october-1-2026--writers-ide-and-creative-ai-role) reconciles earlier generation wording; [Test Strategy §18](Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md#18-writer-ide-acceptance-refinement) links future acceptance. Stage 2 essentials and later 6/8/10 dependencies remain; runtime capability is Not yet proven. [Build Record](../docs/build-records/2026-10-01-writer-ide-spec.md).

## October 2 harness intent and evidence refinement

[Architecture §11.1–11.3](Rikuri_Pi_Architecture_Spec_v1.7.md#111-preserved-intent-and-clarification-contract) defines preserved intent, material clarification, measurement/RAG routing, bounded execution and controlled profile evolution. [PRD HAR-001–004](Rikuri_Pi_PRD_v1.2.md#harness-intent-and-evidence-refinement--october-2-2026), [future acceptance §19](Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md#19-harness-intent-and-evidence-contract-acceptance), and the [ADR clarification](Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md#october-2-2026--harness-intent-and-evidence-clarification) preserve Pi-primary execution and the existing stage boundaries. The owner's small-model/node experience is design rationale, not runtime proof. [Build Record](../docs/build-records/2026-10-02-harness-intent-evidence.md); all runtime behavior remains Not yet proven.

## October 8 mission charter

The owner adopted the [Mission Charter](Rikuri_Pi_Mission_Charter_v1.0.md), a corrected form of the supplied Addendum A. Most of it restates existing promises. The new ones are PRD principles P32–P34, [PRD §29 MISSION-001–006](Rikuri_Pi_PRD_v1.2.md#29-october-8-mission-charter-requirements), three §22 measures, [Architecture §26.5](Rikuri_Pi_Architecture_Spec_v1.7.md#265-platform-claim-states-architectures-and-vendor-ai-layers) (four platform claim states, ARM64 targets, vendor AI layers as adapters rather than homes) and [Test Strategy §20](Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md#20-mission-charter-acceptance). Learned adaptation to the user stays at product Stage 10, and new features went to the [parking lot](../docs/PARKING_LOT.md), whose exit procedure now records a decision-test answer. Foundation-0 stages, gates, S/T/E IDs and dates are unchanged. See the [review](reviews/2026-10-08-mission-addendum-review.md), the [ADR clarification](Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md#october-8-2026--mission-charter-and-platform-independence) and the [Build Record](../docs/build-records/2026-10-08-mission-charter.md); runtime behavior remains Not yet proven.

## October 14 Orchard architectural direction

[ADR 0006 — owner accepted, merge pending](../docs/architecture/adr/0006-proposed-orchard-software-domain.md) and the [annotated integration plan](Rikuri_Pi_Orchard_Integration_Plan_2026-10-14.md) describe Orchard as the software-project domain experience over Rikuri-owned state, policy, scheduling, knowledge and quality services. Contracts can be designed before implementation convergence; prototype stores remain separate until a validated importer exists. Monaco remains a shared workspace surface.

The owner accepted ADR 0006’s architectural direction after review and clarification; normal merge remains pending. The annotated integration plan retains source proposal context. No implementation admission is claimed. See the [acceptance record](../docs/build-records/2026-10-14-orchard-direction-acceptance.md). Stage Plan v2.1, Engineering Standard v1.2 and the creative-studio Master Build Sequence remain authoritative. The supplied older Architecture v1.7 snapshot was compared and the maintained version preserved. See the [integration Build Record](../docs/build-records/2026-10-14-orchard-documentation-integration.md) for source hashes, conflicts and deferred work.

## Orchard TaskContract v1 draft

The [versioned contract and fixture](../docs/architecture/contracts/task-contract-v1/README.md) define provider-neutral requests and separate execution/acceptance/application outcomes. The closed JSON shapes and offline fixture checker are experimental design scaffolding, not a production adapter or permission authority. See the [contract Build Record](../docs/build-records/2026-10-14-orchard-task-contract-v1.md) for test/review evidence and limitations. Foundation-0 gates and sequencing are unchanged.

## November 17 RIEMS outcome-driven execution direction

Owner-approved future direction: [ADR 0007](../docs/architecture/adr/0007-riems-outcome-driven-execution.md) defines the RIEMS control-plane boundary; [Orchard RIEMS integration](Rikuri_Pi_Orchard_RIEMS_Integration_2026-11-17.md) defines the software-domain consumer. See [PRD §9.23](Rikuri_Pi_PRD_v1.2.md#923-riems--outcome-driven-execution-management-post-foundation), [Architecture §12.4](Rikuri_Pi_Architecture_Spec_v1.7.md#124-riems-strategy-selection-boundary-future-not-implemented), the [Master Build Sequence](Rikuri_Pi_Master_Build_Sequence_v1.1.md), and the [Build Record](../docs/build-records/2026-11-17-riems-documentation-reconciliation.md). **Design only:** no RIEMS optimizer exists yet; Foundation-0 Stage Plan v2.1, gate authority and Orchard one-worker defaults do not change.
