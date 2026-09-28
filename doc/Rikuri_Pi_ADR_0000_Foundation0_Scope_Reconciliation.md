# ADR 0000 — Foundation-0 scope and acceptance reconciliation

**Date:** September 24, 2026  
**Status:** Adopted planning baseline; implementation claims remain unproven until their gates pass  
**Applies to:** PRD v1.1, Architecture v1.6, Engineering Standard v1.1, Foundation-0 Re-baselined Stage Plan v2.0  
**Owner:** Rikuri Pi product owner

## Context

The PRD describes the full product and a broader Foundation Release. The architecture and earlier 90-Day Build Plan used F1–F10 for a 13-week slice. The owner has chosen a six-stage Foundation-0 plan with time to test and observe real results. Treating every product-level P0 or the broader Foundation Release checklist as a June 2027 promise would make the plan inconsistent and push technical risks late into the build.

## Decision

The **Foundation-0 Re-baselined Stage Plan v2.0** is the live gate schedule. Six stages run sequentially from October 2026 toward a June 2027 planning target; reserve four to eight weeks of contingency into July–August. Dates are estimates. A required gate stays red until its repeatable demonstration and linked technical evidence pass. The PRD retains the larger product vision and product-level P0 priority; the broader Foundation Release is later than Foundation-0.

| Claim or dependency | Foundation-0 commitment | Later product commitment / boundary |
| --- | --- | --- |
| W1 continuity | Kernel crash points at Stage 00; full eight real cross-component crash points at final release | Further platform and multi-node fault modes get their own gates |
| F2 import | Real ChatGPT and Markdown/Obsidian import with provenance and idempotency in Stage 01 | Claude and read-only Git/project manifest importers early Stage 1 |
| F3 idea | Text capture, lexical retrieval, provenance, project attach in Stage 05 | Voice capture and meaning-based/semantic retrieval later |
| F4 privacy and handoff | Stage 02 policy refusal before HTTP construction; Stage 03 Ubuntu no-network envelope; full cloud-to-local continuity closes with the Stage 04 local runtime | More providers, native macOS containment and remote paths require separate evidence |
| F5/F7 money and authority | Hard budget preflight and prompt-injection authority containment in Stage 02 | Richer routing and access paths remain gated by adapter-specific contracts |
| F6 forgetting | Stage 01 managed backup B and *pre-forget* backup A restoration use current durable revocation state; A without trustworthy current state safely refuses/quarantines | Unmanaged exports and external copies cannot be erased or revoked by Rikuri |
| F8 engineering | One brief Build Record per coherent change, expanded for high-risk work; real caller, Module Contract, health and tests for each protected crate; linked Stage Gate evidence | The full Project Quality Manager product arrives later |
| F9/F10 execution | Stage 03 untrusted project/descendant containment; Stage 04 Pi local coding slice and profile swap | Rich Monaco/LSP/DAP dogfood and multi-agent orchestration later |
| Knowledge Context Compiler | Minimal Stage 02 authority/time/conflict/privacy compiler meets ten expected answer-or-abstain outcomes; neither unsafe acceptance nor blanket abstention passes | Richer distillation, ranking, entity resolution, multilingual search and risk models later |
| Platforms and UI | Ubuntu is full security/execution reference. macOS platform-neutral core CI remains continuous; native envelope experimental. Stage 05 one-window English Home, externalized strings, pseudo-locale/CJK smoke | Native macOS protection and desktop parity, voice Home, seven-language localization and rich windows later |

Each stage ends with an owner-runnable demonstration and a plain-language **Pass / Fail / Not yet proven** receipt. Tests, failure drills, and appropriately independent adversarial review back claims the owner cannot verify by reading code. An agent’s completion claim is insufficient. The public-exposure release gate in the Engineering Standard remains separate and includes its human external security audit.

## Early risk decisions

1. Stage 00 probes bubblewrap namespace creation on the pinned Ubuntu image under its default AppArmor settings. If unavailable, choose and test a safe alternative through an ADR; do not disable system-wide restrictions to make a test pass.
2. E1–E6 are time-boxed experiments. E3 cannot turn a weaker forget/restore promise into a green F6. E4 cannot green-light an always-abstain knowledge compiler. E5 cannot report network denial from configuration alone.
3. Only one tested OpenAI-compatible cloud access path is in the first slice, using synthetic non-sensitive fixtures at Stage 02. Provider gateways need capability-specific testing; a shared wire format alone does not certify production behavior.
4. Speculative additions go to the parking lot. A real change to a promise or security boundary requires an ADR, revised PRD and architecture text, current acceptance tests, and updated runbooks in the same change. This record does not freeze documentation against corrections.

## Consequences and open implementation questions

- The earlier 90-Day Build Plan v1.6 is historical, not a parallel gate board. The Master Build Sequence now points to the live plan; future stages are re-estimated from evidence at each gate.
- The exact durable revocation storage and recovery-key protocol, viable Ubuntu envelope backend, and cloud/local handoff state schema are implementation decisions to prove through E3, E5, and Stage 04. Until then their guarantees are requirements, not implemented facts.
- If an experiment fails, keep the dependent gate red and record an ADR-backed redesign or explicit owner decision on any weaker product promise. Do not silently reduce an acceptance test to match elapsed time.
- No actual build or security gate has passed merely because these planning documents agree.

## September 25, 2026 — documentation reconciliation addendum

The original decision and version scope above are retained as history. The current planning baseline is PRD v1.2, Architecture v1.7, Engineering Standard v1.2, Stage Plan v2.1, and Master Build Sequence v1.1, with Collaborative Intelligence v0.1 and Writer v1.4 as later-stage references. Stage Plan v2.1 supersedes v2.0 as the live gate board. The six-stage scope, gate identities, platform boundaries, and evidence requirements above remain in force.

The Collaborative Intelligence addition makes existing execution evidence explicit: stable IDs, versioned execution/profile identity where exposed, typed events, verification/outcome links, cost, retries, human intervention/feedback, provenance, and privacy. T00.12, T02.16, and T04.18 verify these requirements on existing vertical paths; they do not introduce a seventh stage or claim a gate has passed. No Foundation-0 personality inference, autonomous reflection, learned routing, Lightbulb generation, incubation scheduler, or protected-policy mutation is added. Product Stage 4 evidence and Stage 6 governance precede research prototypes; product Stage 10 owns productization. Writer’s own foundation/MVP remains post-Foundation domain work.

Acceptance alignment includes managed pre-forget restore with current revocation and refusal without it, ten expected answer-or-abstain knowledge cases, the complete eight-point release crash matrix, and the unchanged W1/F1–F10 plus Bootstrap/F4-lite gate board. Implementation remains unproven.

## September 28, 2026 — document history and shared conversation clarification

The owner adopted preservation and flagging of edited projections and requested the preceding capture and human–AI collaboration proposal be included in planning. Earlier decisions above remain historical records. The maintained baseline remains PRD v1.2, Architecture v1.7, Stage Plan v2.1 and Master Build Sequence v1.1.

The two one-way Markdown flows remain controlling. PRD KNOW-005 and its former bidirectional-editing open question are reconciled to Architecture §20.3: user-authored notes import as versioned evidence; generated output never merges back automatically. S01.08 and T01.16 specify durable preservation/read-back, recovery references and visible flags before regeneration, including refusal on preservation failure or unresolved racing edits. Rich recovery/review UI and separately authorized proposals follow product Stage 2. Preserved revisions inherit privacy, retention, forget and current-revocation restore constraints. This refines the existing ownership contract, without changing Foundation gate identities or adding a collaboration service to Foundation.

Capture/classification/contribution design reuses the existing source/artifact service, Knowledge Service, Context Compiler and Attention Firewall. Descriptive classifications remain derived and correctable; source text and agent output cannot authorize actions or promote accepted decisions. Product Stage 1 admits text/semantic experiments under existing evaluation prerequisites; Stage 2 adds source cards; Stage 5 adds capture connectors and bounded personal contributions; Stage 8 adds richer media. Discovery/reflection retains product Stage 4 evidence and Stage 6 governance prerequisites and Stage 10 productization.

Minimum project membership, sharing and conversation audit are **RESEQUENCED** from the formerly unnumbered product Stage 9 business/team scope to Stage 6 S6.C1–C4. The purpose is useful collaboration between multiple humans and agents before enterprise deployment. The [Master Build Sequence Stage 6](Rikuri_Pi_Master_Build_Sequence_v1.1.md#stage-6--multi-agent-governance--project-graph) records owner, predecessor gates, order, acceptance and scheduling impact. Stage 9 retains tenancy, organization administration and Windows assurance; Stage 7 replication is optional for the initial single-authority-node conversation.

Rikuri Link remains Tailscale first, then proven native direct/rendezvous/end-to-end encrypted relay with Tailscale fallback. LAN and remote clients share authenticated application authorization; transport access confers no project/tool rights. The owner-operated agent node is an authorized plaintext processing endpoint; relay opacity does not imply secrecy from that endpoint or permission to send context to cloud models. Local Only's proven no-network guarantee is unchanged. Stage 6 must prove audience/history checks, replay-safe delivery and revocation with two humans on LAN/remote clients and an agent before shared conversations are admitted.

The [Build Record](../docs/build-records/2026-09-28-knowledge-capture-and-team-conversations.md) tracks document verification. All runtime behavior remains Not yet proven; no implementation gate, platform assurance, connector installation, deployment or unattended responsibility is authorized by this planning decision.
