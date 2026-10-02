# Rikuri Pi — Master Build Sequence & Integration Gate Map

> Repository consistency revision: September 25, 2026. Source version retained; see [reconciliation record](reviews/2026-09-25-documentation-consistency.md).

**Version:** 1.1 | September 25, 2026  
**Purpose:** The ordered build path after the six-stage Foundation-0 plan, including the post-platform Collaborative Intelligence research/productization track. It is dependency-driven rather than calendar-driven. Every module is built, tested, wired, observed, documented, and stage-gated before dependent capability expands.

> **Core rule:** Build the boring trusted substrate first. Add one vertical capability at a time. Never allow a shiny downstream feature to hide a missing upstream guarantee.

Foundation-0 stages use two digits (00–05). Product stages use unpadded numbers (0–10); product Stage 0 contains all six Foundation-0 stages. Product Stage 4 is Model Lab, not Foundation-0 Stage 04 (Pi/local coding). Writer phases are a separate domain roadmap.

## A. Universal per-module loop

Every module/function/feature uses this same sequence:

1. **Plan** — one-sentence outcome, predecessor/dependent items, blast radius, trust/data/budget impact, exact acceptance and wiring point.
2. **Characterize** — prove current behavior before changing it.
3. **Contract** — define stable interface, ownership, failure/degraded semantics, health signal, and module contract.
4. **Build smallest coherent increment.**
5. **Unit/failure tests.**
6. **Shared contract/invariant/property tests.**
7. **Wire it** to the real caller/entry point.
8. **Integration/vertical-slice test** through production-style adapters.
9. **Adversarial/failure/recovery/security test** where applicable.
10. **Observe** last-success, fallback, logs/events, performance/cost evidence.
11. **Update docs/RAG/runbook/help in the same change.**
12. **Scrap Sweep** and remove old/scaffolding/orphans.
13. **Full gate.**
14. **Stage Gate update** with evidence. Only then may dependent work begin.

## B. Deferral / resequencing protocol

When an item cannot or should not be done in its current position, do **not** leave an unchecked box behind.

Record:

```yaml
item_id: ...
disposition: DEFERRED | RESEQUENCED | BLOCKED | REMOVED | SUPERSEDED
reason: ...
evidence_or_constraint: ...
dependency_impact: ...
old_stage_ordinal: ...
new_stage_ordinal: ...
owner: ...
reactivation_trigger: ...
acceptance_condition: ...
decision_record: ...
```

Then reorder every dependent item in the live checklist in the same change. If the move changes an architecture boundary or promised release scope, update the ADR/Architecture/PRD as appropriate.

---

# Stage 0 — Foundation-0: trusted core

**Live sequence:** Foundation-0 Re-baselined Stage Plan v2.1 and ADR 0000. The earlier 90-Day Build Plan v1.6 is historical. Six gates are estimated from October 2026 to June 2027 with a four-to-eight-week contingency reserve. Dates cannot close unproven gates. Ubuntu is the full execution/security reference; platform-neutral macOS core build/unit/contract tests run continuously, with native macOS envelope and desktop assurance experimental.

1. **Stage 00 — Bootstrap and kernel:** CI/red drill, Engineering Standard/Build Records, IDs/storage/ledger, first transactional crash points, Pi driveability and harness-placement spike, and early Ubuntu execution-backend admission probe (OpenShell first candidate).
2. **Stage 01 — Ownership:** real ChatGPT and Markdown/Obsidian import with provenance and lexical retrieval, project resolver, encryption, deletion keys, managed backup/restore. Restore both the pre-forget and post-forget snapshots with current durable revocation state; safely refuse/quarantine without it.
3. **Stage 02 — Trust and money:** minimal Knowledge Context Compiler with ten expected answer/abstain outcomes, Auto/Private/Local Only admission, hard budget reservation, trusted approvals, and one synthetic non-sensitive cloud access path. F4-lite policy refusal is an intermediate checkpoint.
4. **Stage 03 — Execution boundary:** Ubuntu filesystem/network envelope, ProjectTrust, untrusted read without execution, descendant containment. Prove the F4 envelope prerequisite; full cloud/local handoff waits for a local runtime.
5. **Stage 04 — Pi and local coding:** Pi Harness Profile, local runtime, scoped KnowledgeService tools, verified synthetic coding slice, second profile swap, and full cloud-to-local continuity with Local Only in the proven envelope. F4 and F10 close here.
6. **Stage 05 — Home and integrated release:** one-window English Home, text Idea capture/search/attach, pseudo-locale/CJK checks, complete eight-point crash matrix, F1/F3/F8 and clean-machine W1/F1–F10 evidence review.

**Foundation-0 exit:** all W1/F1–F10 gates, the Stage 02 F4-lite checkpoint, and owner-runnable demos have linked Pass/Fail/Not yet proven receipts. The separate Engineering Standard public-exposure release gate does not silently apply to this internal milestone. Every handoff has an ID and ACK; stale/truncated state and process-alive-only health cannot satisfy a gate. The Stage Plan is authoritative for item ordinals and test details.

---

# Stage 1 — Capture continuity and supporting execution

**Why next:** Preserve creative sources and prove supporting execution before expanding the studio. Foundation-0 Stage 04's coding proof remains a security/reference slice, not an obligation to build a full IDE first.

1. **S1.01 Claude importer** after F2: provenance, idempotency, supported-export coverage and visible gaps.
2. **S1.02 read-only Git/project manifest importer** after F9: untrusted inspection without execution.
3. **S1.03 capture continuity:** supported text imports and source revision receipts; correctable classification/topic retrieval under Architecture §20.6. Semantic/vector experiments require the existing ≥40-question corpus and FTS failures; originals remain retrievable without them.
4. **S1.04 supporting execution and handoff:** reuse the admitted Foundation coding/tool path for one real Rikuri change, showing diff, verification, durable result and an external coding-tool handoff. A full Monaco/LSP/DAP/terminal product is not a predecessor of creative authoring.

Compare established capture and tool adapters before adoption with pinned version/license, authority/locality, lifecycle cost and allowed/denied evidence. No dependency is selected by this plan.

Before any Stage 1 or later worker receives a credential or a permitted network destination (for example dependency fetch in the Stage 8 developer pack, formerly Stage 1 items 5/8), that path must pass the Option C broker and permitted-destination qualification deferred from Foundation-0 ([Stage Plan deferral table](Rikuri_Pi_Foundation0_Rebaselined_Stage_Plan_v2.1.md#deferral-and-resequencing-table); Test Strategy §8.2). Until then it stays unavailable.

**Stage 1 gate:** supported imported sources survive restart with accurate coverage and provenance; inspection does not execute; one admitted development change and external handoff retain verification and project continuity. Unsupported provider history is visibly incomplete. Full native IDE dogfood is deferred to Stage 8's optional developer pack.

---

# Stage 2 — Creative Studio essentials and Sterling Workspace

Build order:

1. Window Registry + stable Window/Workspace/Link IDs.
2. Freeform move/resize/focus/min/max/close.
3. Grid/preset arranger that never permanently constrains resizing.
4. Context Link.
5. Agent Link.
6. Prompt Link/broadcast with isolated initial reasoning.
7. Saved conceptual layouts.
8. Status roll-up: Working / Blocked / Done / Idle, project-level needs-attention.
9. Detach/native windows.
10. Multi-monitor/desktop-space restoration.
11. Agent-callable Workspace API under policy.
12. Rikuri Plugin API: editor/agent/tool/window extensions with explicit capabilities.
13. Plugin lifecycle: provenance/pin/quarantine/verify/update/rollback/remove.
14. **S2.C1 — durable creative objects:** fragments, scenes, character/world notes, relationships, source cards and author intent over existing governed storage; browse/search/export/restart without a model.
15. **S2.C2 — connected authoring views:** manuscript, scene/outline board, character/world bible and multiple story branches on the same canvas; selection and edits remain project-consistent.
16. **S2.C3 — requested exploration:** create, compare, choose and selectively merge alternatives; ordinary critique and supplied-evidence abstraction retain provenance and author-controlled canon. No longitudinal inference or autonomous reflection.
17. **S2.C4 — resume and production handoff:** recover the current work, open questions and alternatives; export a traceable creative brief/spec to an external coding or production tool.
18. **S2.C5 — local plugin contract prototype:** explicit capabilities and entitlement boundary; host adapter cannot own canonical data. No public distribution or paid checkout is implied.

Item 4 includes evidence cards referencing existing source spans/artifacts and visible freshness/coverage. After the existing policy-backed workspace operations are available, add projection-edit comparison/recovery and explicit revision proposals under Architecture §20.3.1. Rich media extraction waits for Stage 8; multi-human membership waits for Stage 6. Source cards and recovery UI never grant canonical write authority.

**Stage 2 gate:** a creator starts with unorganized fragments, develops two alternative scene paths, links characters/world/source evidence, chooses or merges with visible conflicts, resumes after restart and exports a coherent production brief. Chat, authoring, sources, candidates and logs coexist, link, detach and restore. Rejected alternatives remain available under retention policy; AI cannot silently change canon. S2.C1–C5 satisfy STUDIO acceptance in Test Strategy §16. Rich media extraction, team governance, unattended research and learned reflection wait for their own gates.

---

# Stage 3 — Rikuri Link: “phone home”

Build order:

1. Node identity/presence/state (`online`, `sleeping`, `offline`).
2. Tailscale connector abstraction.
3. Authenticated remote command channel to `rikurid`; no public port forwarding.
4. Thin client chat/task/status/approval.
5. File/photo/share-to-node.
6. Voice instruction path.
7. Reconnect/idempotent command delivery and durable job continuity.
8. Full Workspace streaming/control for desktop-class remote clients where appropriate.
9. Multi-node `Run on: Auto` selection baseline.
10. Optional wake/gateway capability where hardware permits.
11. Native Rikuri direct-connection prototype.
12. Rendezvous/discovery service.
13. E2E encrypted relay fallback.
14. Keep Tailscale connector as backup even after native Link is primary.

Items 1–4 must distinguish human/device/worker identity and use the same authenticated application permissions for LAN and outside clients. Tailscale is first; native direct/relay paths remain unavailable until their encryption, identity, replay and revocation contracts pass. Route switches cannot weaken authority or transport guarantees. The node is an admitted plaintext processing endpoint; untrusted relays are not. This establishes transport for Stage 6 collaboration, not multi-human membership by itself.

**Stage 3 gate:** leave the home network, issue/continue a task on the Ubuntu node, approve a consequential step, disconnect/reconnect, and recover the same durable job without exposing plaintext to a relay or opening a router port.

---

# Stage 4 — Model Lab, harness intelligence, routing & economics

Build order:

1. Provider/local model discovery adapters.
2. Runtime compatibility registry.
3. Harness/Profile compatibility registry.
4. Exact execution identity: model revision/quantization/runtime/profile/settings/hardware.
5. Hardware-fit screening.
6. Task eval suites: coding/tool/RAG/reasoning/language/etc.
7. Open-harness source-change analyzer and benchmark comparator.
8. Profile recommendation/tuning under rollback.
9. Provider price/access/quota ingestion with provenance/confidence.
10. Local-cost estimate policy.
11. Performance Ledger dashboards.
12. Auto router using verified evidence only after sufficient observations.
13. Native-harness exception routing when measured material advantage exists.
14. Background probation/revalidation daemon with thermal/battery/idle policy.
15. Bounded composite execution (lead plus specialists), including a vision-specialist fixture, under per-role capability, privacy, verification, and shared budget limits; no bypass of later multi-agent governance.

**Stage 4 gate:** adding a new compatible model normally requires registry/profile data and evaluation, not core code; Auto can explain why it selected model+runtime+harness+machine+privacy path.

**Collaborative Intelligence prerequisite:** Model onboarding/calibration must preserve versioned prompt/frame/context profile identity and exposed reasoning/sampling settings in benchmark evidence so later cognitive recipes can compare “same model, better operation” rather than relying on model-name rankings. Seed values, when supported, are reproducibility/variance metadata rather than durable quality labels.

---

# Stage 5 — Creative research, connectors and distribution adapters

Order: browser worker isolation -> connector capability model -> MCP/connectivity adapters -> creative-source capture, files/cloud storage and research -> Attention Firewall and bounded proactive knowledge-gap suggestions -> external studio/plugin distribution adapter -> optional email/calendar/travel packs -> richer voice. Every connector gets least-authority, approval, audit, and revoke tests. MCP remains a capability/connectivity layer; it does not own Rikuri workflow state, authority, memory, or audit truth.

At files/cloud storage, wire common Save/share capture for supported source types with durable-save and extraction/search health. After source permission/freshness reconciliation and Attention Firewall controls, add personal conversational contributions (request/mention, quiet suggestions, explicitly enabled automatic mode) with budget, relevance/novelty, duplicate suppression and source cards. Shared multi-human posting waits for Stage 6's audience controls; richer media extraction waits for Stage 8. Failed classification must leave the original recoverable and baseline search usable.

**Gate:** common assistant request works from natural language without exposing MCP/provider plumbing and without expanding authority silently.

---

# Stage 6 — Multi-agent governance + Project Graph

Before implementing a fleet, pin and evaluate Paperclip as a reference: event-driven heartbeats, atomic checkout and leases, adapter lifecycle, durable run/session state, goal ancestry, templates, approvals, budgeting, and audit. Exercise contention, replay, failed handoff, cancellation, budget exhaustion, and recovery. For each mechanism write an ADR with version/license/security review, comparison to other mature components, measured result, and a reuse/adapt/build decision. The company org chart is a domain profile, not the universal ProjectState schema. This evaluation is S6.2 and is deferred outside Foundation-0.

Order: typed/versioned Rikuri Workflow IR -> schema/policy/security validation -> human review gates -> durable DAG/checkpoints/waits/retries/cancellation/recovery -> immutable deployment versions + event history -> Paperclip reference evaluation/ADRs -> Project Lifecycle Kernel (Frame/Shape/Plan/Execute/Verify/Deliver/Learn) -> intent-led team proposal with scope/cost/access/review -> coordinator/COO role -> specialists -> evaluator -> team budgets -> parallel isolation -> Strategy Council -> scenario packets -> Project Graph -> integration/review workflows -> coordinator scorecard.

Agents propose or repair workflows; deterministic infrastructure owns accepted execution. Each workflow records proposer/provenance, exact approved version, capabilities and credential scopes, data classes, outbound destinations, budget envelope, triggers, approvals, modifications, and outcomes. Governance is implemented and tested before autonomous workflow deployment.

After the existing workflow/policy/governance substrate is verified, implement the minimum shared conversation in this order:

1. **S6.C1 — membership and scoped authority:** authenticated human/device/worker identities; expiring invitations/acceptance; owner/member/guest capabilities; explicit history grants and removal. Requires Stage 3 transport; no implicit shell or approval rights.
2. **S6.C2 — conversation and shared objects:** durable messages and attachment/source references, authenticated authorship, revision ancestry, task ownership/conflict handling and idempotent acknowledgements through Rikuri services. Requires S6.C1; use one authoritative node, no Stage 7 replication dependency.
3. **S6.C3 — audience-safe agent contributions:** apply Stage 5 capture/retrieval/attention controls to shared posts and history; current audience and source checks before commit/delivery/read, invalidation of pending posts and contexts on membership changes. Requires S6.C2 and source reconciliation; private contexts are not implicitly joined.
4. **S6.C4 — LAN/remote recovery and security slice:** two humans (one LAN, one outside) and an agent exchange a file and cited note; reconnect without duplicate effects; revoke a member/device during retrieval; refuse unauthorized history/source disclosure and injected tool instructions. Requires S6.C1–C3; test every enabled transport, leaving native paths unavailable until independently proven.

**September 28 resequencing:** minimum project membership, sharing and conversation audit move from product Stage 9 (business/team controls, formerly unnumbered) to Stage 6 S6.C1–C4. Reason: meaningful human–AI collaboration requires them before enterprise deployment. Dependency impact: Stage 6 now includes Stage 3 transport and Stage 5 contribution controls plus its own governance before multi-human activation; Stage 9 extends this verified base. Owner: product owner and Stage 6 implementer. Reactivation: predecessor gates green and admitted-platform security available. Acceptance: S6.C4 and [Test Strategy §15](Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md#15-post-foundation-capture-and-shared-conversation-acceptance) pass with receipts. [ADR dated clarification](Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md#september-28-2026--document-history-and-shared-conversation-clarification) records the decision. No Foundation scope or gate change; re-estimate Stage 6 from this added work rather than retaining an unsupported schedule.

**Gate:** a plain-language request can assemble one worker or a justified team without manual hiring or runtime configuration. Atomic ownership, acknowledgements, authoritative ProjectState, and transactional commit are shown by integration tests. A long-running workflow survives agent exit, context compaction, restart, model replacement, durable waits, cancellation, and branch/rejoin without duplicate effects or lost state; unauthorized modification is rejected and visible. Multi-agent work then demonstrably improves selected tasks over one-agent baseline without losing budget/security/evidence control or spamming the user.

The gate also includes S6.C4's shared human–AI conversation, with both authorized collaboration and denied disclosure/action paths proven. Neither a working owner-to-node Link demo nor a team of agents alone closes the multi-human requirement.

---

# Stage 7 — Personal Mesh + persistent responsibilities

Order: second-device identity -> encrypted event/version replication -> conflict semantics -> optional hub -> resource catalog across nodes -> selective sharing scopes -> durable responsibilities -> digest/attention sync -> CRDT only where simultaneous text editing proves necessary.

**Gate:** replacement/offline/reconnect/concurrent-update scenarios converge without silent loss; backup remains independent from sync.

---

# Stage 8 — Rich domains and Live Companion

Deep world/character systems, scientific scenario tools, game/animation specification workflows, editable visual assets, marketing and other optional domains extend Stage 2 essentials. Each is a projection over common project/window/idea/knowledge/security primitives; no independent “mini-brain.”

The optional developer pack resumes the former Stage 1 items: Monaco/file/diff; terminal/PTY; Git/worktree/trust UI; LSP; test/build tasks; formatter/linter; symbol context; containers; DAP; richer coding profiles; harness comparisons; full native dogfood. Preserve that internal order and platform admission gates. Selective `monaco-vscode-api` compatibility is evaluated here only for demonstrated gaps. It is not a prerequisite for creative briefs or external coding handoff.

Extend the common capture/classification/retrieval path with image/OCR, audio/transcript and video/frame provenance. Evaluate remembered-content queries, decisive visual evidence, failed extraction and user corrections. Shared contributions inherit Stage 6 audience/history controls. Meeting capture remains explicit and visible; group voice/video conferencing requires its own capability/transport assessment and is not implied by captured audio or Link voice instructions.

---

# Stage 9 — Windows parity + business/team readiness

Implement Windows OS Capability/envelope parity after Ubuntu/macOS contracts are stable, then organization identity, tenancy, role/policy admin, shared memory, centralized audit, enterprise deployment and connectors.

Minimum project membership/sharing/conversation audit now belongs to Stage 6 S6.C1–C4. Stage 9 extends it with organization-wide administration and assurance; basic team conversation does not depend on Windows delivery.

---

# Stage 10 — Collaborative Intelligence / Inquiry & Discovery

This is the long-term differentiating layer. It is deliberately downstream of trustworthy Personal Intelligence, Model Lab evidence, and multi-agent governance. Research prototypes may begin earlier after product Stage 4 Model Lab evidence and product Stage 6 governance are green; none may become an authority over canonical state or protected policy simply because a prototype performs well.

Build/research order:

1. ExperienceEpisode projection over existing canonical events/outcomes.
2. Cheap deterministic/statistical pattern detection and anomaly/change triggers.
3. Selective reflection service; local/cheap models first, escalation by budget.
4. Candidate abstraction objects with evidence, counterexamples, rivals, scope and lifecycle.
5. CognitiveRecipe registry covering frame/prompt/context/model/runtime/harness/tools/reasoning/sampling/verification.
6. Champion/challenger evaluation and bounded Curiosity/Optionality Budget.
7. Model onboarding/calibration that learns task-specific operation rather than static ranking.
8. Lightbulb lifecycle with dormant/resurface/usefulness feedback and delayed credit.
9. Cognitive Ensembles: independent generation -> cross-critique -> evidence tournament -> synthesis, with methodological diversity preferred over agent count.
10. Inquiry & Discovery incubation sessions for explicit questions, contradictions, recurring patterns and watched domains.
11. Domain verifiers/labs: software tests, mathematics/proof, research/source/statistics, writing fresh-eye/closure labs, and others as warranted.
12. Quiet Guidance layer: evidence-backed suggestions about collaboration/user workflow with inspectability, correction, dismissal, privacy and no covert manipulation.
13. Only after strong offline evidence: contextual-bandit / RL-style adaptive recipe selection under hard budget, rollback and exploration floors.
14. Dogfood the system on Rikuri development before broad user-facing autonomy.

**Stage 10 gate:** On predeclared benchmark/user-work sets, Collaborative Intelligence demonstrates repeatable uplift or useful discovery versus simpler baselines while staying within cost/attention/privacy envelopes. Every surfaced heuristic/Lightbulb is traceable to evidence; counterexamples and stale conditions can downgrade it; candidate discovery cannot silently mutate canonical truth or protected policy; and “no useful insight” remains an acceptable outcome.

---

# C. Master Stage Gate checklist

A stage may be marked COMPLETE only when all are true:

- [ ] Every required predecessor complete or explicitly superseded by ADR.
- [ ] Every new interface has at least one real implementation and caller.
- [ ] Shared contract tests green.
- [ ] Failure/security/recovery tests green.
- [ ] Real running vertical slice green.
- [ ] Health/last-success/fallback observability verified.
- [ ] User-visible behavior matches docs/help.
- [ ] Migration/rollback/recovery path proven where state changed.
- [ ] Performance/cost evidence recorded where routing decisions rely on it.
- [ ] Scrap Sweep/clean tree green.
- [ ] No unexplained skipped/disabled tests.
- [ ] Every deferred item has been resequenced, not merely noted.
- [ ] Full gate green.
- [ ] Stage Gate evidence links complete.

**Only then does the next dependent stage/function become active.**

## September 30, 2026 — creative-studio resequencing

This is the live post-Foundation order, superseding the earlier full-IDE-first and all-Writer-at-Stage-8 ordering. Foundation-0 00–05, W1/F1–F10, Bootstrap/F4-lite and T00.12/T02.16/T04.18 are unchanged. No dates, gates or implementations become green through this decision.

| Item / disposition | Old position | New position | Reason / dependency impact | Owner | Reactivation / acceptance |
| --- | --- | --- | --- | --- | --- |
| Essential Writer foundation/MVP, RESEQUENCED | Product 8, unnumbered Writer pack | Product 2, S2.C1–C4 | Creator value follows trusted core and Stage 1 capture; advanced domains are not prerequisites | Product owner / Stage 2 implementer | Stage 0/1 gates; Test Strategy §16 essentials slice |
| Full developer platform, DEFERRED | Product 1, items 1–12 | Product 8, optional developer pack, same internal order | Full IDE is not the lead product; Stage 1 S1.04 retains admitted coding verification and handoff | Product owner / developer-pack implementer | Demonstrated creator/developer demand, predecessor and platform gates; full native feature-to-commit dogfood receipt |
| Advanced single-worker repair controller, RESEQUENCED | Product 1 coding lifecycle under SEC-013–019 | Product 8 developer pack after test-runner/trust contracts | Rich repair depends on deferred developer services; Stage 1 retains existing grants and verified supporting execution | Product owner / developer-pack implementer | Explicit policy/ADR reconciliation, independent verification and Test Strategy §14 single-worker acceptance; no owner-only fix-lock change |
| Monaco compatibility, RESEQUENCED | Product 2, item 14 | Product 8 developer pack after editor contract | No editor dependency for first studio | Product owner / developer-pack implementer | Measured compatibility gap; capability/security/update/rollback evidence |
| Scientific/media/game/marketing depth, RETAINED | Product 8 rich domains | Product 8 domain packs | Progressive depth; no universal simulator or design-tool clone promised | Product owner / domain implementer | Domain contract and allowed/denied verification; creator-approved handoff |
| Learned reflection/abstraction, RETAINED | Product 10, research after 4 and 6 | Same | Requested critique differs from learning across episodes; instrument early, infer late | Product owner / CI implementer | Predeclared baseline comparison, counterexamples, revocation, budget and quiet-guidance evidence |

Stage 5 distribution/research, Stage 6 governed parallel and overnight Pi work, and Stage 7 persistent responsibilities reuse their existing authority prerequisites. Initial Stage 2 requested comparisons are sequential; they cannot bypass later fleet governance. Writer phases map to this sequence, rather than becoming a second gate board. See ADR 0000 September 30 clarification.
