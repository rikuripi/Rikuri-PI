# Rikuri Pi — Master Build Sequence & Integration Gate Map

**Version:** 1.0 | September 16, 2026  
**Purpose:** The ordered build path after the six-stage Foundation-0 plan. It is dependency-driven rather than calendar-driven. Every module is built, tested, wired, observed, documented, and stage-gated before dependent capability expands.

> **Core rule:** Build the boring trusted substrate first. Add one vertical capability at a time. Never allow a shiny downstream feature to hide a missing upstream guarantee.

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

**Live sequence:** Foundation-0 Re-baselined Stage Plan v2.0 and ADR 0000. The earlier 90-Day Build Plan v1.6 is historical. Six gates are estimated from October 2026 to June 2027 with a four-to-eight-week contingency reserve. Dates cannot close unproven gates. Ubuntu is the full execution/security reference; platform-neutral macOS core build/unit/contract tests run continuously, with native macOS envelope and desktop assurance experimental.

1. **Stage 00 — Bootstrap and kernel:** CI/red drill, Engineering Standard/Build Records, IDs/storage/ledger, first transactional crash points, Pi driveability spike, and early Ubuntu namespace/AppArmor admission probe.
2. **Stage 01 — Ownership:** real ChatGPT and Markdown/Obsidian import with provenance and lexical retrieval, project resolver, encryption, deletion keys, managed backup/restore. Restore both the pre-forget and post-forget snapshots with current durable revocation state; safely refuse/quarantine without it.
3. **Stage 02 — Trust and money:** minimal Knowledge Context Compiler with ten expected answer/abstain outcomes, Auto/Private/Local Only admission, hard budget reservation, trusted approvals, and one synthetic non-sensitive cloud access path. F4-lite policy refusal is an intermediate checkpoint.
4. **Stage 03 — Execution boundary:** Ubuntu filesystem/network envelope, ProjectTrust, untrusted read without execution, descendant containment. Prove the F4 envelope prerequisite; full cloud/local handoff waits for a local runtime.
5. **Stage 04 — Pi and local coding:** Pi Harness Profile, local runtime, scoped KnowledgeService tools, verified synthetic coding slice, second profile swap, and full cloud-to-local continuity with Local Only in the proven envelope. F4 and F10 close here.
6. **Stage 05 — Home and integrated release:** one-window English Home, text Idea capture/search/attach, pseudo-locale/CJK checks, complete eight-point crash matrix, F1/F3/F8 and clean-machine W1/F1–F10 evidence review.

**Foundation-0 exit:** all W1/F1–F10 gates, the Stage 02 F4-lite checkpoint, and owner-runnable demos have linked Pass/Fail/Not yet proven receipts. The separate Engineering Standard public-exposure release gate does not silently apply to this internal milestone. Every handoff has an ID and ACK; stale/truncated state and process-alive-only health cannot satisfy a gate. The Stage Plan is authoritative for item ordinals and test details.

---

# Stage 1 — Pi coding dogfood + native developer platform

**Why next:** Rikuri should become useful for building itself before expanding into many unrelated assistant features.

Before item 1, add **S1.01 Claude importer** after F2 and **S1.02 read-only Git/project manifest importer** after F9; run the importer and untrusted-project contract suites. Then build in this order:

1. Monaco Window and file/diff model.
2. Terminal/PTY service.
3. Git service, worktree/branch isolation, project trust UI.
4. LSP client/service and first languages used by Rikuri (Rust/TypeScript first).
5. Test/build/task runner.
6. Formatter/linter integration.
7. Repository/symbol search and code-aware context.
8. Docker/container/dev-tool capabilities.
9. DAP/debug baseline where it provides material value.
10. Pi coding profiles: small-model vs larger-model tool exposure, planner/reviewer, context/compaction, verification cadence.
11. Open-harness analysis tool: diff Qwen Code/DeepSeek Harness behavior and test useful techniques against profiles.
12. Dogfood: complete multiple real Rikuri changes entirely through the Rikuri coding workflow and Build Procedure.

**Stage 1 gate:** a real feature can be planned -> coded -> diffed -> tested -> full-gated -> documented -> committed with evidence through Rikuri. External IDE is optional, not required.

---

# Stage 2 — Sterling Workspace + native plugin surface

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
14. Selective `monaco-vscode-api` compatibility only for demonstrated gaps.

**Stage 2 gate:** chat + Monaco + terminal + tests + agent + logs can coexist, link, detach, restore, and remain project-consistent after restart.

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

**Stage 4 gate:** adding a new compatible model normally requires registry/profile data and evaluation, not core code; Auto can explain why it selected model+runtime+harness+machine+privacy path.

---

# Stage 5 — Browser, connectors, assistant capability packs

Order: browser worker isolation -> connector capability model -> MCP/connectivity adapters -> email -> calendar/contacts -> files/cloud storage -> research -> travel/shopping -> routines/Attention Firewall -> richer voice. Every connector gets least-authority, approval, audit, and revoke tests. MCP remains a capability/connectivity layer; it does not own Rikuri workflow state, authority, memory, or audit truth.

**Gate:** common assistant request works from natural language without exposing MCP/provider plumbing and without expanding authority silently.

---

# Stage 6 — Multi-agent governance + Project Graph

Before implementing a fleet, pin and evaluate Paperclip as a reference: event-driven heartbeats, atomic checkout and leases, adapter lifecycle, durable run/session state, goal ancestry, templates, approvals, budgeting, and audit. Exercise contention, replay, failed handoff, cancellation, budget exhaustion, and recovery. For each mechanism write an ADR with version/license/security review, comparison to other mature components, measured result, and a reuse/adapt/build decision. The company org chart is a domain profile, not the universal ProjectState schema. This evaluation is S6.2 and is deferred outside Foundation-0.

Order: typed/versioned Rikuri Workflow IR -> schema/policy/security validation -> human review gates -> durable DAG/checkpoints/waits/retries/cancellation/recovery -> immutable deployment versions + event history -> Paperclip reference evaluation/ADRs -> Project Lifecycle Kernel (Frame/Shape/Plan/Execute/Verify/Deliver/Learn) -> intent-led team proposal with scope/cost/access/review -> coordinator/COO role -> specialists -> evaluator -> team budgets -> parallel isolation -> Strategy Council -> scenario packets -> Project Graph -> integration/review workflows -> coordinator scorecard.

Agents propose or repair workflows; deterministic infrastructure owns accepted execution. Each workflow records proposer/provenance, exact approved version, capabilities and credential scopes, data classes, outbound destinations, budget envelope, triggers, approvals, modifications, and outcomes. Governance is implemented and tested before autonomous workflow deployment.

**Gate:** a plain-language request can assemble one worker or a justified team without manual hiring or runtime configuration. Atomic ownership, acknowledgements, authoritative ProjectState, and transactional commit are shown by integration tests. A long-running workflow survives agent exit, context compaction, restart, model replacement, durable waits, cancellation, and branch/rejoin without duplicate effects or lost state; unauthorized modification is rejected and visible. Multi-agent work then demonstrably improves selected tasks over one-agent baseline without losing budget/security/evidence control or spamming the user.

---

# Stage 7 — Personal Mesh + persistent responsibilities

Order: second-device identity -> encrypted event/version replication -> conflict semantics -> optional hub -> resource catalog across nodes -> selective sharing scopes -> durable responsibilities -> digest/attention sync -> CRDT only where simultaneous text editing proves necessary.

**Gate:** replacement/offline/reconnect/concurrent-update scenarios converge without silent loss; backup remains independent from sync.

---

# Stage 8 — Rich domains and Live Companion

Writing, study, research, meeting/lecture, media, trading/other domain packs. Each is a projection over common project/window/idea/knowledge/security primitives; no independent “mini-brain.”

---

# Stage 9 — Windows parity + business/team readiness

Implement Windows OS Capability/envelope parity after Ubuntu/macOS contracts are stable, then organization identity, tenancy, role/policy admin, shared memory, centralized audit, enterprise deployment and connectors.

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
