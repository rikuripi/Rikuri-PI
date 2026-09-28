# Documentation index

These are maintained revisions of the supplied project documents. Their original filenames and version labels are preserved. Superseded versions are retained in Git history rather than the current checkout.

Coding agents start with [Agent Standard Work](../AGENTS.md). Use the Engineering Templates below for Build Records rather than maintaining a second checklist.

## Current Foundation-0 baseline

| Document | Role |
| --- | --- |
| [Foundation-0 Re-baselined Stage Plan v2.1](Rikuri_Pi_Foundation0_Rebaselined_Stage_Plan_v2.1.md) | **Live** six-stage sequence, item IDs, acceptance tests, experiments, and gate board. Start here. |
| [ADR 0000 — Scope Reconciliation](Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md) | Adopted decision separating the Foundation-0 commitments from the broader product and historical 90-day plan. |
| [PRD v1.2](Rikuri_Pi_PRD_v1.2.md) | Product vision and requirements, including later releases. A product-level P0 is not automatically due in Foundation-0. |
| [Architecture Spec v1.7](Rikuri_Pi_Architecture_Spec_v1.7.md) | Architecture baseline; the live stage plan controls Foundation-0 sequence and evidence. |

The September 25 update set has been reconciled in-repository for version, scope, roadmap, and acceptance consistency. Source version filenames are retained; Git history records the supplied originals and subsequent corrections. ADR 0000 retains its original decision with a dated addendum for the current baseline. See [the reconciliation record](reviews/2026-09-25-documentation-consistency.md) for changes and validation.

Foundation-0 stages use two digits (00–05). Product stages use unpadded numbers (0–10); product Stage 0 contains all six Foundation-0 stages. Product Stage 4 is Model Lab, not Foundation-0 Stage 04 (Pi/local coding). Writer phases are a separate domain roadmap.

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
