# Rikuri Pi × Rikuri Orchard — Architecture Integration Plan

> Repository reconciliation — October 14, 2026. This is a retained **proposal**, not an adopted decision or authorization to run its work packages. The live Foundation-0 Stage Plan is v2.1, Engineering Standard v1.2, and Orchard's executable baseline is v0.3. References below to v2.0/v1.1/O0 describe the source proposal's earlier baseline. The adopted September 30 creative-studio sequencing and current Master Build Sequence control over the proposal's IDE-first Stage 1/Stage 2 schedule. Contract alignment starts now as design work; production convergence waits for applicable evidence gates. No Foundation-0 promise changes.


**Date:** 2026-10-14\
**Status:** Proposed reconciliation, not yet merged into the Rikuri repository\
**Scope:** Rikuri Pi PRD v1.2; Architecture v1.7; Engineering Standard v1.1; ADR 0000; Foundation-0 Stage Plan v2.0; Rikuri Orchard First Seed v0.1\
**Decision companion:** `ADR_PROPOSED_Rikuri_Orchard_as_Software_Domain.md`

## Executive decision

**One trusted platform; one Project Lifecycle Kernel; one authoritative ProjectState; many domain experiences.** Rikuri Orchard is the software engineering / software factory *domain experience* of Rikuri Pi. It is not a second sovereign orchestration platform. The Monaco-centric native developer workspace is a *shared work surface* used by Orchard, not a second IDE product with its own task state or security authority.

**Immediate action:** freeze the proposed interface contracts in an ADR and module ownership matrix now. **Implementation convergence:** only when the appropriate Foundation-0 gates have passed. Existing Orchard v0.1 stays a disposable local prototype and cannot be represented as production agent execution/security.

The product target is an intent-driven environment: the human may say “just make it,” may co-create, or may prescribe exact applications, models, and layouts. The user defines outcome and accepts consequential choices. Rikuri selects and operates tools/models according to permission, measurable suitability, and relevant user preferences. User-visible creative work, not operation of apps and prompts, is the center of the experience.

## Baselines and nonnegotiable scope

- **PRD v1.2** and **Architecture v1.7** are the located product/architecture baselines. PRD v1.2 exists in the Library; earlier uncertainty about the missing v1.2 is resolved for the Library copy.
- **ADR 0000** and **Foundation-0 Stage Plan v2.0** control the six-stage Foundation-0 gates; old 90-Day Plan v1.6 is historical. Foundation-0 targets June 2027 with contingency; *acceptance evidence rather than time closes gates.*
- Foundation-0 Ubuntu is the only complete security/execution reference; macOS receives platform-neutral core CI. Never claim an Orchard macOS app, local browser UI, Tauri build, Docker container, or a Git worktree is a proven execution sandbox.
- Stage 1 after Foundation-0 adds Monaco/LSP/DAP/Git/test integration and Rikuri coding dogfood. Stage 2 adds richer Sterling-style windows, saved conceptual layouts, and multi-monitor support.
- Security enforces all worker operations outside the model. Agent-generated text, `AGENTS.md`, and checklists do not grant authorization or constitute verification.
- **No silent loss, stale-tail substitution, or false-green status.** State changes are transactional, with ledger, stable IDs, acknowledgement, idempotency and recoverable last-known-good state.

## Architecture / single ownership map

| Responsibility | Authoritative owner | Orchard view or specialization | Forbidden duplication |
| --- | --- | --- | --- |
| User intent / project state | Rikuri Project Registry + Project Lifecycle Kernel | Seed Box → software Intent/Project; Field Map → project graph | A second production Orchard project DB or lifecycle state machine |
| Canonical objects / events / migrations | `rikurid` / domain, storage, ledger | Reads/projections via versioned APIs; evidence links | Direct model or Orchard writes into Rikuri canonical SQLite |
| Policy / secrets / budgets / approvals | AISecurity, trusted approval renderer, budget governor | Orchard Gate presents exact approval decision | Agent decides or expands its own permissions |
| Agents / orchestration / task ownership | Rikuri scheduler, run/lease control plane, model/harness adapters | Orchard Crew shows work and assignments | Separate permanent task queues, worker identity, job leases |
| Knowledge / provenance / personal learning | Rikuri Knowledge Context Compiler + evidence records | Gardener’s Journal shows scoped lessons | Parallel agent-specific memories or unverified facts treated as truth |
| Execution capability | Rikuri tool/connector registry, OS capability adapters | Tool Shed catalog / verified integrations | UI executing arbitrary shell/code behind policy |
| Isolated execution | Rikuri OS execution envelope | Greenhouse is a view of *tested* isolation capabilities | A branch, worktree, container, or prompt mislabeled secure isolation |
| Quality / completion evidence | Rikuri Quality Gate Engine + build/stage record | Harvest Board and Weather Report | “Agent says done,” or UI manually changed to DONE |
| Editor, terminal, diff, tests | Shared developer capability layer | Monaco as an Orchard window; terminal/test/diff peers | Orchard-specific Monaco fork, duplicate IDE services |
| Workspace layout | Rikuri Workspace API | Orchard software-workspace presets | Durable project content stored only in desktop geometry |
| Application/model selection | Shared capability registry + intent planner + model profiles | Software tools first; later all domains | Static per-provider master prompts and app-specific state authorities |

The same core later serves writing, research, film production, product design, and other domains via profiles. Each domain specializes artifacts, tools, quality criteria, and views—not the Project Lifecycle Kernel.

## The canonical workflow

`Frame (capture desired outcome)` → `Shape (scope, choices, examples)` → `Plan (small dependencies, task contract)` → `Execute (authorized agent/app)` → `Verify (deterministic independent evidence)` → `Deliver (reviewable artifact)` → `Learn (provenance-backed observations)`.

**Every execution** receives stable `project_id`, `intent_id`, `task_id`, `run_id`, source revision, authorized workspace/path scope, `execution_profile_id` (model+agent harness+runtime+version), budget, approvals, acceptance criteria, evidence destination, and explicit stop conditions. An attempt can end `DONE`, `BLOCKED`, `NEEDS_OWNER_DECISION`, `CANCELLED`, or `UNVERIFIED`; views may decorate, but never override those states. A `DONE` state is not assigned merely because a test command exited zero—it requires task-specific acceptance evidence and all applicable mandatory gates.

## Module-by-module build and proof matrix

**Module C0 — Contracts and ADR, immediately (documentation, no trusted execution added).** Define Project/Intent/Task/Run/Workspace/Artifact/Evidence IDs; state ownership; task contract schema; adapter and policy boundaries; model profile schema; completion and stopping semantics; errors. **Proof:** trace every Orchard concept to exactly one Rikuri owner and document one end-to-end sequence with failure handling. No second system-of-record created.

**Module C1 — Orchard O0 prototype characterization, now.** Preserve standalone Python/HTML/CSS/SQLite Seed Box as a learning artifact; keep a versioned status truth table. Four provided Python tests passed in the assistant's separate review runtime on 2026-10-14; the user's environment remains separately verified by the user. **Proof:** create seed, restart, reread; invalid transition rejected; UI makes no claim of worker execution. Do not silently migrate its SQLite database into Rikuri canonical storage.

**Module C2 — Foundation-0 Stage 00 `rikurid` core, by active baseline.** IDs, SQLite current state, append ledger, idempotency, CLI, failure injection, CI, macOS core build, Ubuntu sandbox feasibility probe. Orchard may use mock schemas/test fixtures but must not enlarge S00 gate set. **Proof:** restart/crash and red-test drills with durable receipts; no duplicate effects.

**Module C3 — Orchard O1 bounded Codex experiment, isolated from Rikuri release claims.** One orchestrator, one worker, one approved disposable test repo, finite scope, no auto worktrees/PR/merge, no secrets, a configured external execution boundary, and one provider-specific Codex adapter behind a provider-neutral task contract. One targeted repair attempt by default; one required independent gate with actual test/inspection evidence; no automatic next task. **Proof:** seed → one work order → actual edit → recorded test/diff → independent gate → DONE or BLOCKED; deliberately failing requirement stays BLOCKED; replay does not silently repeat effects. Explicitly label anything not verified (including containment) as UNVERIFIED. Before invoking external execution, test the available containment; do not claim strong isolation from a prompt.

**Module C4 — Foundation-0 Stages 01–03: owner state, context, security.** Import/provenance and managed restore/forget; policy/context budget preflight; untrusted repo read/execute boundary; Ubuntu envelope and descendant restrictions. Only switch Orchard to these services once their relevant stage gates actually pass. **Proof:** stable source trace; safe refusal; real network/process/restore tests. Prototype SQLite remains an explicitly separate demonstration store until a tested migration/importer exists.

**Module C5 — Foundation-0 Stage 04: real single worker on Pi.** Reuse the **same** Task Contract and Tool/Model Adapter contract proven in O1; add Pi-backed local coding slice, runtime-vs-harness profile separation, model-family profile swap, cloud→local handoff. **Proof:** one authorized read/edit/test/diff task, profile swap with no kernel change, handoff ACK, budgets and security gates, evidence receipt. Codex remains an external/native-harness adapter where justified, not a competing core.

**Module C6 — Foundation-0 Stage 05: simple Home and idea loop.** Home is a client of real IPC; idea capture/search/project association works through Rikuri APIs. Orchard appears as an optional experimental software-project presentation if it can use proven contracts without weakening gating. **Proof:** full owner demo from clean checkout. No requirement for the full Orchard UI or rich spatial windows in Foundation-0.

**Module C7 — Post-Foundation Stage 1: unified software developer experience.** Monaco window, LSP, DAP when useful, Git, terminal/PTy, test/build, diff/review, project trust indicators. Orchard surfaces Seed Box/Field Map/Harvest Board as views over shared ProjectState and evidence. **Proof:** normal Rikuri change can be selected/planned/executed by an authorized worker, reviewed via Monaco/diff/tests, and recorded in a Build Record entirely through the coherent platform.

**Module C8 — Post-Foundation Stage 2: spatial workspaces.** Freeform/docked panels, saved conceptual layouts, optional detached windows/multi-monitor subject to OS permissions; user/agent control arbitration. **Proof:** coding or media workspace can be saved/restored without changing project/agent authoritative state; missing monitor/panel gives explicit recoverable warning.

**Module C9 — Later experiments: multiple workers, app selection, Model Lab and intent-led creation.** Parallel task leasing, actual lease/cancellation/recovery testing; app/agent-native API catalog; verified application selection; per-model effective-dated prompt guidance; small empirical model regression suite; user-chosen Outcome/Collaborative/Directed control. **Proof:** model/profile swaps change compiled instructions but not intent or acceptance; app integrations are tested per OS/version; recommendations demonstrate actual user benefit and resource economy. Wider scope requires explicit ADR/gates—not retroactively added to Foundation-0.

## Orchard vocabulary mapped to shared primitives

| Orchard presentation | Rikuri meaning |
| --- | --- |
| Seed Box | Intent/Idea Inbox and authorized promotion to Project |
| Field Map | Project graph, dependencies, stage gates, owner choices |
| Orchard Crew | Agent/task view over worker/run/lease records |
| Tool Shed | Model/harness/app/tool/skill capability registry |
| Greenhouse | Tested task execution environment, not merely a worktree |
| Orchard Gate | Trusted human approval and policy decision |
| Weather Report | Real health, resource/budget, failure and last-success telemetry |
| Harvest Board | Verified deliverable and linked evidence/release record |
| Gardener's Journal | Learning candidates with provenance, scope, confidence, experiments |

Naming is a UI concern; durable API names and truth states remain technical and unambiguous. Keep the calm professional interface with restrained animated accents and reduced-motion compatibility.

## Resolve `AGENTS.md` and changing models at the correct layers

**Four distinct sources of instruction must not be collapsed into one growing prompt:**

1. **Non-negotiable policy / engineering invariants:** enforce in Rikuri policies, permissions and executable CI gates, not via agent promises.
2. **Repository-specific durable facts:** keep `AGENTS.md` short; file paths, safe local commands, architectural boundaries, where to find more detailed guidance. Remove generic legacy “verify again and again” instructions unless measured necessary.
3. **Task-specific contract:** outcome, allowed edits, acceptance evidence, budgets and explicit terminal conditions. Generated per job and retained as authoritative state.
4. **Execution-profile guidance:** effective-dated model + agent harness + runtime version capabilities, current provider recommendations, tuned autonomy, context/prompt conventions, and measured exceptions. Compile instructions without double-injecting generic verification rules.

**Critical distinction:** the worker may perform its own checks, but the **system** determines when the required independent gate and acceptance evidence pass. Stop after pass or bounded failure; never create an infinite self-review loop. Maintain provider-profile history with evaluation and rollback. A new Claude or Codex release triggers comparison and a small eval suite before changing defaults. User instructions remain model-neutral.

**Immediate `AGENTS.md` revision recommendation:** split security/task authority out of the contributor prompt, keep the durable repository orientation concise, link relevant Engineering Standard and gate commands, and do not ask models to do duplicative generic self-checks. The Orchard starter `AGENTS.md` should continue identifying itself as a standalone prototype, not replace Rikuri Pi repository policies.

## The first work package to authorize

**WP-ORCHARD-CONTRACT-01 — Converge interfaces without merging software**

- Inputs: located source docs plus `Rikuri_Orchard_First_Seed_v0.1.zip`.
- Change: adopt proposed ADR (after owner review), publish an ownership/contract matrix and a minimum versioned `TaskContract` schema, map UI Seed statuses to canonical concepts, shorten prototype `AGENTS.md` to durable facts, and correct Orchard roadmap so integration contracts begin at O1 instead of being postponed to O5.
- Allowed scope: docs/spec/tests for schemas; no production Rikuri state mutation, no agent spawning, no network install, no stage-gate change.
- Verification: schema tests for valid/missing/unauthorized/terminal states; one independent review of ownership against PRD, architecture and ADR 0000; run original four O0 tests; trace `Seed → Plan → Run → Verified Deliverable` in a fake adapter with blocking failure and explicit stop.
- Termination: contract and ADR reviewed, affected docs aligned, tests and review recorded; stop. Otherwise return BLOCKED/NEEDS_OWNER_DECISION with evidence. Do not automatically begin O1 Codex execution or future modules.

**Then WP-ORCHARD-CODEX-01** (distinct owner authorization): one tiny genuine coding task through Codex under bounded, demonstrably isolated execution; outcomes/receipts as described in C3.

## Documentation reconciliation checklist

1. **New ADR**: record Orchard-as-domain-experience, state ownership, Monaco role, prompt/model profiles and O1 experimental boundary; reference ADR 0000 instead of superseding it.
2. **PRD v1.2 successor/addendum**: human-imagination-first UX, intent-to-app/model matching, three control modes, shared command engine for Rikuri-developed capabilities, requirements for independent completion evidence and user override. Product priority is not a Foundation-0 promise.
3. **Architecture v1.7 successor/addendum**: update capability registry/planner, execution profiles, lifecycle kernel, shared commands, editor/workspace, Project Quality Manager and stage seams. Preserve existing §28, §32, §33, §49 and §53 concepts; do not create a duplicate system.
4. **Engineering Standard v1.1 successor/addendum**: instruction-layer separation, deterministic gates, model-profile regression testing, evidence-based stop states, task contracts and incident→tripwire learning.
5. **Orchard prototype docs**: revise O1 to begin with adapter contract compatible with Rikuri; revise O5 from “merge control planes” to “retire/migrate prototype infrastructure once mature Rikuri services exist.”
6. **Foundation-0 Stage Plan v2.0**: retain all gates, dependencies and timing; include only explicit non-scope cross-reference or a formally approved change. Do not insert rich Orchard/IDE/app discovery into Stage 00.
7. **Git and provenance**: one ADR-reviewed change set, linked Build Record, exact source-version references, failed-tests evidence, and explicit list of deferred features.

## Open decisions to resolve by evidence, not guesswork

- Will Orchard UI ultimately share the Tauri/Workspace client or remain a replaceable browser surface? Decide after comparing portability and UI integration costs.
- Which production adapter is suitable for native Codex/Claude sessions? Pin supported versions, permissions and cancellation semantics and run adapter contract tests.
- What importer/migration maps Orchard O0 `seed.id` to `idea_id` and then `project_id` without losing source identity, versions or provenance? Do not copy prototype rows directly into canonical DB.
- Which concrete environment meets the Ubuntu Stage 03 execution envelope under default system protections? Until verified, isolation claims remain NOT YET PROVEN.
- Which per-model verification prompts are net beneficial? Decide through model-version-specific measurements rather than generic `AGENTS.md` mandates.

## Summary rule

**Build one small, demonstrably useful vertical slice at a time.** Each slice must show real user value, one accountable owner per durable capability, enforceable trust boundaries, proof of actual end-to-end wiring, a comprehensible receipt, and a stop condition. The project grows by adding verified capabilities to one kernel—not by multiplying clever but disconnected systems.

---

## November 17, 2026 — RIEMS consumer boundary (owner-approved design; not implemented)

This historical proposal remains retained context. [RIEMS ADR 0007](../docs/architecture/adr/0007-riems-outcome-driven-execution.md) assigns outcome-oriented strategy choice to Rikuri's shared control plane. The [Orchard companion](Rikuri_Pi_Orchard_RIEMS_Integration_2026-11-17.md) describes software-development specialization; it does not admit a production Orchard scheduler, new approval/budget state, multi-agent crew or migration. Orchard v0.3 remains a bounded one-Codex-worker prototype with separate review/apply/recovery evidence. Product Stages 1/4/6/7 respectively measure single-worker baselines, routing alternatives, justified team selection and trusted mesh placement. No Foundation-0 gate, Orchard milestone or automatic workflow changes.
