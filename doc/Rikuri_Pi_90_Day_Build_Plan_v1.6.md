# Rikuri Pi — 90-Day Build Plan v1.6

> **HISTORICAL — superseded September 24, 2026.** Use the *Foundation-0 Re-baselined Stage Plan v2.0* and ADR 0000 for current scope, order, gate ownership, and timing. The dates and unchecked boxes below record the former 13-week baseline; they are not current assignments or acceptance criteria.

**Scope:** Stage 0 / Foundation-0, including the minimum Pi/project-trust/OS seams required to avoid later architectural rework  
**Start:** week of 14 September 2026  
**Target:** 13 December 2026 — Foundation-0 evidence complete and Architecture v1.6 frozen from implementation results  
**Primary reference platform:** Ubuntu LTS  
**Continuous secondary target:** macOS core/build/test  
**Operating standard:** Rikuri Pi Engineering Standard v1.1  
**Companion docs:** PRD v1.1; Architecture Spec v1.6 (Word baseline); Stage 0 Skeleton v1.2; Threat Model v1.2; Test/Replay Strategy v1.2; Engineering Templates v1.1

> Build core outward. Wire every module into a real vertical slice. Prove it works, fails safely, reports health, and leaves evidence before moving to the next dependent feature.

This was the **former ordered checklist**. Its unchecked boxes are historical; use the current Stage Plan for live dispositions and acceptance. `DEFERRED` means the item is moved to a new explicit ordinal/stage with reason, dependency impact, owner, reactivation/acceptance condition, and the downstream checklist is reordered in the same change.

---

## 1. Non-negotiable operating rules

- [ ] One builder, approximately four focused days/week; agents may implement, but deterministic gates certify.
- [ ] Ubuntu is the primary development/reference platform; macOS core remains continuously tested.
- [ ] Modular monolith first; one trusted `rikurid` owns canonical mutations.
- [ ] CLI/core before rich UI; no spatial workspace work until its predecessor gates are green.
- [ ] Pi is the primary/default governed harness substrate for local/open models; no parallel home-grown full harness.
- [ ] Qwen Code/DeepSeek Harness are references/benchmarks; native harness adapters require measured justification.
- [ ] Evolution for existing solutions; revolution for core strength: prefer proven mechanisms for routine infrastructure after versioned, security, and failure-path evaluation; reserve new engineering for Rikuri-specific intelligence and trust.
- [ ] Read != execute: new projects/repositories are untrusted for execution until explicit scoped promotion.
- [ ] Core CI never requires a live frontier model.
- [ ] Red required gate stops dependent feature work.
- [ ] Every stage closes with a real **wiring/vertical-slice** demonstration, not isolated module tests.
- [ ] Fallback engagement is observable; repeated fallback means Degraded.
- [ ] Every significant defect becomes a tripwire.
- [ ] Authoritative state changes follow preserve -> validate -> candidate -> validate -> durable write -> read-back verify -> atomic promotion -> archive, with last-known-good rollback.
- [ ] Every cross-component handoff has a durable ID/ACK; stale, truncated, or missing state is an explicit typed failure. Health reflects real outcomes, not process liveness.
- [ ] Build Records start during planning; Stage Gates update during work.
- [ ] Docs/RAG/runbooks change with behavior.
- [ ] Scrap Sweep every close-out.

---

## 2. Foundation-0 live gate board

| Gate | Status | Target | Evidence |
|---|---|---|---|
| Bootstrap | [ ] NOT STARTED | Day 0 | |
| W1 Crash/continuity | [ ] NOT STARTED | Week 1 | |
| F1 Calm fresh-install Home | [ ] NOT STARTED | Week 10 | |
| F2 Real imports + provenance + idempotency | [ ] NOT STARTED | Week 5 | |
| F3 Idea capture/retrieval/project attach | [ ] NOT STARTED | Week 10 | |
| F4 Cloud/local handoff + Local Only containment | [ ] NOT STARTED | Week 8 | |
| F5 Hard $1 budget reservation stop | [ ] NOT STARTED | Week 8 | |
| F6 Rebuild + backup/wipe/restore + forget | [ ] NOT STARTED | Weeks 11–12 | |
| F7 Prompt-injection authority containment | [ ] NOT STARTED | Week 9 | |
| F8 Engineering Standard/evidence-first completion | [ ] NOT STARTED | Week 2 onward; final Week 13 | |
| F9 Untrusted-project / descendant-process containment | [ ] NOT STARTED | Week 9 | |
| F10 Pi + Harness Profile local coding vertical slice | [ ] NOT STARTED | Week 7 | |

A gate becomes COMPLETE only when linked evidence exists.

---

# 3. Phase 0 — Bootstrap gate (before feature work)

**Stage Gate:** `00-bootstrap.md`

### Plan
- [ ] Copy Engineering Standard v1.1 into canonical repo path.
- [ ] Install Build Record/Stage Gate/Module Contract/Incident/Traceability templates v1.1.
- [ ] Create all Foundation Stage Gate records from this plan.
- [ ] Create live master checklist with stable item IDs/ordinals and DEFERRED/RESEQUENCED fields.
- [ ] Record Ubuntu primary + macOS continuous target decision in ADR.

### Build
- [ ] Initialize protected-main Git workflow.
- [ ] Rust workspace, lockfiles, dependency policy, secrets policy.
- [ ] CI: format/build/clippy/unit + secret scan + ratchets; no `continue-on-error` for required gates.
- [ ] Pre-commit basic checks + secret scan.
- [ ] `scripts/quality/full-gate.sh`, `smoke.sh`, `scrap-sweep.sh`, traceability/checklist validators.
- [ ] Add residue checks for skipped tests, `.bak/.old`, debug markers, committed secrets.
- [ ] Required docs/evidence directories.

### Verify / wire
- [ ] Red-test drill blocks CI.
- [ ] Secret drill blocks a known fixture without exposing the secret.
- [ ] Clean Ubuntu checkout runs documented full gate.
- [ ] macOS core build/test job runs.
- [ ] Checklist validator rejects an unchecked required item without disposition.
- [ ] Checklist validator rejects a RESEQUENCED item missing new stage/ordinal/impact.

### Close out
- [ ] Build Record + Stage Gate evidence complete.
- [ ] Scrap Sweep + docs current + full gate green.

---

# 4. Ordered weekly build sequence

## Week 1 — Domain, storage, ledger, crash continuity

**Stage Gate:** `01-kernel-ledger.md`

### Build
- [ ] `rikuri-domain`: stable IDs, privacy/risk, provenance, instruction authority, deletion unit, schema versions, project-trust state.
- [ ] SQLite bootstrap/migrations + repository traits.
- [ ] Append-only referenced-content ledger + idempotency.
- [ ] Provenance fields link actor, parent/correlation event, policy version, capability-token reference, concrete target, decision/result, and evidence hash.
- [ ] Versioned agent-facing JSON envelope with typed success/error states, stable error codes, correlation/provenance IDs, and documented CLI exit codes.
- [ ] `rikurid` + CLI: `init`, dummy task, ledger/state/health.
- [ ] Freeze a headless Runtime API: canonical task/orchestration/policy state remains in `rikurid`; terminal, desktop, mobile, browser, voice, and remote surfaces are attachable clients.
- [ ] `rikuri-testkit`: fake model/tool, crash injection, deterministic IDs/clock.
- [ ] Module Contracts for created crates.

### Verify / wire
- [ ] Transaction/repository/ledger contract tests.
- [ ] Property: no duplicate consequential effect across randomized transitions.
- [ ] Kill `rikurid` mid-task; restart/reconstruct state.
- [ ] Controlled regression with idempotency disabled fails W1.
- [ ] CLI -> kernel -> repository -> ledger is exercised as one real path.
- [ ] CLI and in-process adapters pass the same normalized JSON/error contract tests.

### Exit
- [ ] W1 green; ADRs stable IDs/storage/ledger/threat assumptions accepted or explicitly RESEQUENCED.

**Do not start:** model APIs, UI, vectors, plugin/workspace systems.

---

## Week 2 — Security kernel, project trust, OS Capability contract

**Stage Gate:** `02-security-platform.md`

### Build
- [ ] Threat Model v1.2 + trust-boundary docs enter repo before implementation.
- [ ] `rikuri-security`: privacy admission, capability decision, risk classes, trusted approval schema foundation.
- [ ] ProjectTrust contract: `untrusted_read`, scoped execute promotion, revoke/review-required.
- [ ] Descendant-process policy contract: parent approval is not descendant authority.
- [ ] `rikuri-platform`: normalized OS Capability API.
- [ ] Ubuntu adapter/envelope skeleton; macOS adapter compiles with explicit weaker assurance metadata.
- [ ] AISecurity reuse assessment: lift/adapt only proven components.
- [ ] Convert the September 2026 AISecurity protection matrix into a versioned automated regression corpus; scope every protection claim to a named threat path and recent evidence.

### Verify / wire
- [ ] Untrusted project can be opened/read/indexed without project executable side effect.
- [ ] Malicious `.git/config/core.fsmonitor` fixture does not execute during read-only inspection.
- [ ] OS Capability fake + Ubuntu adapter share contract tests.
- [ ] Denied operation produces deterministic security decision/evidence.
- [ ] Material blocked operations emit a user-visible alert; silent blocking fails the gate.
- [ ] Supported native/local fixtures cover daemon interruption, flags/xattrs/ACLs, copy/export/exfiltration, encrypted-copy/key semantics, and bulk sequential reads.
- [ ] Security health becomes Degraded if required enforcement probe is absent.

### Exit
- [ ] Security/platform contracts are real callers in CLI/test path; no decorative interfaces.

---

## Week 3 — Canonical content, crypto-forget, importer contract

**Stage Gate:** `03-content-import-contract.md`

### Build
- [ ] Encrypted content store/key registry with per-deletion-unit DEK design.
- [ ] Canonical object families for project/source/conversation/message/span/idea/decision/task/artifact/relation/execution/cost/security/version/tombstone.
- [ ] Importer contract: source -> validated candidates/provenance; parser cannot write canonical DB.
- [ ] All-or-nothing import transaction.
- [ ] Golden importer fixture framework.

### Verify / wire
- [ ] Synthetic import twice -> stable identity/no duplicates.
- [ ] Forget one object -> plaintext cannot materialize.
- [ ] Malformed/truncated import -> zero partial canonical commit.
- [ ] Inject a failure before each commit/promotion step; old authoritative data remains intact and retryable. Read back and verify candidate bytes and meaning before promotion.
- [ ] Import path passes through security/source authority tagging and real repository transaction.

---

## Weeks 4–5 — Real sources, project registry, retrieval baseline

**Stage Gate:** `04-import-project-retrieval.md`

### Build
- [ ] ChatGPT importer.
- [ ] Claude importer.
- [ ] Markdown/Obsidian snapshot importer.
- [ ] Git/project manifest/content importer in **read/untrusted** mode.
- [ ] Project Registry + Resource Resolver with authoritative location and trust state.
- [ ] Source/external IDs, parser version, hashes, origin, instruction authority.
- [ ] Generated Markdown projection; user vault imported as versioned source.
- [ ] FTS5 mandatory baseline + `VectorIndex` contract.
- [ ] `KnowledgeService` boundary with read-only-first tools for authorized space enumeration, search, cited source reconstruction/read, and bounded question answering.
- [ ] Freeze at least 20 real known-answer retrieval questions before vector tuning.

### Verify / wire
- [ ] Shared Importer contract for every importer.
- [ ] Idempotent re-import and version-change fixtures.
- [ ] Search -> provenance -> open original/live resource works.
- [ ] Delete/rebuild all derived indexes -> same canonical state; rerun corpus.
- [ ] Git/project import cannot silently promote execution trust.
- [ ] F2 green.
- [ ] Harness answers the retrieval corpus through scoped Knowledge Service tools without direct database/vector-index authority.
- [ ] Knowledge Context Compiler rejects or marks stale/superseded/conflicting evidence, preserves source and valid/observed time, and revalidates volatile or consequential claims at use time; a low-risk stable fact does not trigger needless checks.

**Defer unless evidence demands:** graph DB, reranker, fancy chunking, semantic graph UI.

---

## Week 6 — Context Compiler + privacy intent modes

**Stage Gate:** `05-context-privacy.md`

### Build
- [ ] Context Package includes provenance, authority, privacy, source span/content ref.
- [ ] Auto / Private / Local Only(Vault) project/workspace policy semantics.
- [ ] Imported/retrieved/model/tool content defaults to no instruction authority.
- [ ] Cloud admission rejects Local Only/Secret before serialization/materialization.
- [ ] Local sanitization/minimal-reproduction interface for optional frontier reasoning.

### Verify / wire
- [ ] Golden context tests.
- [ ] Property: content cannot self-promote authority.
- [ ] Local Only cloud package construction denied before network object exists.
- [ ] Project privacy persists across task/handoff and overrides routing eligibility.

---

## Week 7 — Pi primary harness + local coding vertical slice

**Stage Gate:** `06-pi-harness-profile.md`

### Build
- [ ] `HarnessAdapter` contract and `harness-pi` adapter.
- [ ] Harness Profile schema: family/task/runtime/hardware/privacy/project overlays.
- [ ] Runtime compatibility record separate from agent/profile compatibility.
- [ ] One local runtime adapter (Ollama/llama.cpp/LM Studio/vLLM through chosen supported interface).
- [ ] Minimal coding profile: read/search/edit/diff/test; bounded terminal; verify result.
- [ ] Capability providers advertise tools but gain no authority until policy issues a task-scoped capability token.
- [ ] Freeze a minimal Task Space contract for isolated browser/terminal/editor/files/credentials/network/log/audit ownership and explicit human/agent handoff; rich UI remains deferred.
- [ ] One synthetic family profile + second compatible profile fixture to prove no kernel model-name branching.
- [ ] Capability metadata covers input/output modalities, tools, structured output, streaming, runtime/hardware fit, and privacy eligibility; missing modality metadata is explicit rather than silently downgraded.

### Verify / wire — **F10**
- [ ] Open synthetic project read-only through Rikuri.
- [ ] Explicitly promote scoped execution trust.
- [ ] Pi inspects, edits, runs declared test, and returns diff/evidence through Rikuri tools.
- [ ] Project/local-model process cannot write canonical DB directly.
- [ ] Swap profile fixture without core harness change or project-state loss.
- [ ] Attach a synthetic vision-specialist fixture to a text/coding lead; prove separate role identity, context, permissions, cost, and evidence without requiring the lead model to accept images.
- [ ] Disable profile support deliberately -> compatibility status becomes Experimental/Unavailable, not silent failure.
- [ ] F10 green.

**Do not build:** separate Qwen/DeepSeek full-harness integration. Inspect their source only to inform profile tests.

---

## Week 8 — Execution Registry, budget, cloud adapter, cloud/local handoff

**Stage Gate:** `07-router-budget-handoff.md`

### Build
- [ ] Minimal dynamic model/access/runtime/harness/economics registry; model names remain data.
- [ ] Integer microdollar accounting; reserve -> call -> reconcile -> release.
- [ ] One cloud/broker adapter.
- [ ] Router selects execution profile from capability/privacy/access/budget, not model-name branch.
- [ ] Freeze a composite execution-plan schema: lead role plus optional coding/vision/voice/retrieval/embedding/verification specialists, each with scoped context, tools, budget, privacy, fallback, and handoff rules.
- [ ] Cloud-to-local/Pi handoff uses Rikuri-owned state.

### Verify / wire
- [ ] Budget property tests across retry/reserve sequences.
- [ ] Insufficient reservation -> zero network call. **F5**
- [ ] Second synthetic model/access/price record adds by data only.
- [ ] Cloud -> local handoff continues task without restatement.
- [ ] A two-role synthetic plan routes by declared modality and preserves independent accounting/provenance; a lost modality declaration fails visibly.
- [ ] Local Only refuses cloud and proceeds only in proven local envelope. **F4**
- [ ] Historical price evidence remains stable after current price changes.

---

## Week 9 — Adversarial execution, GitSpawn-class containment, approvals

**Stage Gate:** `08-project-exec-injection-approval.md`

### Build
- [ ] Trusted Approval Renderer from schema/normalized arguments.
- [ ] AISecurity detectors as advisory defense-in-depth only.
- [ ] Deterministic authority checks plus optional semantic intent-risk and session-sequence anomaly signals; no LLM is the root allow/deny authority.
- [ ] Security fixture corpus: prompt injection + Git/project execution surfaces.
- [ ] Inspect/track `.git/config`, hooks, package lifecycle/build scripts, Make/Cargo/Python hooks, Docker/devcontainer/MCP/workspace configs.
- [ ] Trust invalidation/review rule when relevant executable metadata changes.

### Verify / wire — **F7/F9**
- [ ] Malicious narration cannot change deterministic consequence text.
- [ ] Imported malicious instructions remain searchable with zero authority. **F7**
- [ ] Untrusted `git status` path cannot execute configured helper outside policy.
- [ ] After scoped trust promotion, malicious descendant attempts filesystem/network/secret escape and is denied.
- [ ] Parent-command approval does not expand descendant authority.
- [ ] Provenance reconstruction links request -> grant/denial -> tool call -> result -> alert with stable event IDs.
- [ ] Bulk reads followed by staging/compression/egress and repeated denied escalation trigger the configured session response.
- [ ] F9 green.

---

## Week 10 — Smallest useful desktop + Foundation coding view

**Precondition:** [ ] W1 [ ] F2 [ ] F4 [ ] F5 [ ] F7 [ ] F9 [ ] F10 green. If not, **RESEQUENCE UI**, do not weaken gates.

**Stage Gate:** `09-home-capture-coding-view.md`

### Build
- [ ] Tauri-class one-window client through versioned local IPC only.
- [ ] Home: prompt, minimal launcher, recent work, health/needs-attention.
- [ ] Text Idea Inbox + Search/Project views.
- [ ] Minimal coding view may embed Monaco for the already-proven Pi vertical slice; no Sterling window manager yet.
- [ ] Approval card from trusted renderer.
- [ ] Settings: execution endpoint/key reference/local runtime/privacy default.
- [ ] `en` + pseudo-locale; Unicode/CJK data round-trip.

### Verify / wire
- [ ] UI smoke checks content/behavior markers, not only window creation.
- [ ] Home -> project -> untrusted trust state -> Pi coding task -> diff/test evidence works through real IPC.
- [ ] F1 + F3 green.

**Still defer:** floating/tiled/detached windows, LSP/DAP richness, plugin API UI, multi-monitor, Project Graph, full Compare Mode.

---

## Weeks 11–12 — Recovery, rebuild, forgetting, replacement-machine proof

**Stage Gate:** `10-recovery-restore.md`

### Build
- [ ] Consistent SQLite + encrypted content + key/recovery metadata + source manifest + app/schema/doc version + integrity manifest.
- [ ] `backup`, `restore --verify`, `forget <id>`.
- [ ] Managed restore excludes usable keys for forgotten deletion units.
- [ ] One real migration through backup -> upgrade -> restore.
- [ ] Retrieval corpus CI-gated.

### Verify / wire — **F6**
- [ ] Restore onto clean Ubuntu environment from docs alone.
- [ ] Delete derived indexes; rebuild.
- [ ] Forget -> wipe -> restore -> object cannot materialize/search/reappear.
- [ ] Core backup/restore test also runs on macOS where platform-neutral.
- [ ] F6 green.

---

## Week 13 — System integration freeze + next-stage handoff

**Stage Gate:** `11-foundation-freeze.md`

### Integrate
- [ ] Clean machine: install -> import -> retrieve -> open project untrusted -> privacy mode -> promote scoped trust -> Pi edit/test -> budget/cloud/local handoff -> backup/restore/forget.
- [ ] Every protected module has Module Contract and real caller.
- [ ] Health board shows last-success/degraded state for required components.
- [ ] Threat -> control -> test -> evidence matrix current.
- [ ] No unexplained skipped tests, fallback, or debt increase.
- [ ] Every deferred item has reason + dependency impact + new stage/ordinal + owner + trigger/acceptance condition.
- [ ] Master Build Sequence is reordered to match actual Foundation evidence.
- [ ] Architecture v1.6 freeze ADR records what changed from plan.
- [ ] F8 green.

### Release/freeze gate
- [ ] W1 + F1–F10 green from clean checkout.
- [ ] Full gate, Scrap Sweep, docs/help current.
- [ ] All Build Records COMPLETE/ABANDONED with disposition.
- [ ] All Foundation Stage Gates complete.

---

# 5. Explicitly deferred from the 90 days — with ordered reschedule

These are **not forgotten**. Their sequence is the initial post-Foundation order and is updated whenever a dependency moves.

| New ordinal | Item | Why deferred | Reactivation condition |
|---|---|---|---|
| S1.1 | Full Monaco developer platform: LSP/DAP/Git/terminal/test/container services | Foundation proves only minimal coding view first | F1–F10 green; Pi vertical slice stable |
| S1.2 | Rich Pi coding profiles/subagents/review loops | Avoid tuning before base harness/security evidence | F10 stable + real dogfood corpus |
| S2.1 | Sterling freeform/tiled/detached/multi-monitor Window system | UI breadth must not mask core failures | Stage 1 coding dogfood gate green |
| S2.2 | Rikuri Plugin API + Window plugins + selective `monaco-vscode-api` | Needs stable Workspace/security capability model | Stage 2 window/security contracts green |
| S3.1 | Tailscale-backed Rikuri Link remote client | Remote access after local node is trustworthy | Foundation recovery/security + Stage 1 core client stable |
| S3.2 | Native Rikuri direct/rendezvous/E2E relay | Avoid networking platform before remote UX is proven | Tailscale Link dogfood evidence |
| S3.3 | Endpoint-secured opaque relay: application-layer E2EE, pairing, rotation, revocation, replay defense, recovery, self-host option | Relay must not become a trusted content endpoint | Link threat model and direct/Tailscale path proven |
| S4.1 | Provider-neutral Fast Decision Model shadow experiment; Jev is the initial replaceable candidate | Routing advice must be calibrated on Rikuri work before it can influence automation | Registry, performance ledger, labeled outcomes, and deterministic budget/security evidence available |
| S4.2 | Full Model Lab/discovery/hardware fit/learned Auto | Need real execution evidence first | Pi coding corpus + registry stable; only validated decision signals admitted |
| S4.3 | Qwen/DeepSeek/native harness exception adapters | Default is Pi; exception needs evidence | Benchmark shows material benefit |
| S4.4 | Unsloth-class local fine-tuning/quantization/inference provider | Hardware-specific optimization stays behind Local Model Service | Registry/evals stable; compare llama.cpp/Ollama/vLLM/MLX without Kernel coupling |
| S4.5 | Specialist-model orchestration for vision, voice, retrieval, embedding, coding, and verification | Composite routing needs single-agent evidence first | Registry modality metadata and deterministic composite-plan tests green |
| S5.1 | Browser/MCP/connectors/email/calendar/travel/shopping | Not core dependency | Link/workspace/security mature |
| S5.2 | Connector Gateway for native APIs, MCP, direct adapters, and Zapier-class hosted fabrics | Breadth must not grant authority | Capability-token, audit, and scoped credential contracts proven |
| S5.3 | Task Spaces with isolated browser profiles and explicit human/agent handoff | Rich isolation UX follows core contracts | Workspace/security contracts green; compare structured snapshots with screenshot control |
| S5.4 | Progressive-disclosure diagram skills and semantic/rendering separation | Useful but not a trusted-core dependency | Skill loader and artifact contracts stable |
| S6.1 | Governed Workflow IR + deterministic durable runtime for agent-proposed automations | Agent/provider sessions cannot safely own long-lived state, waits, retries, or authority | Capability/policy, credential-scope, privacy, budget, approval, versioning, ledger, and recovery contracts proven |
| S6.2 | Pin and examine Paperclip control-plane reference: heartbeat/event dispatch, atomic checkout, adapters, persistent runs, team templates, approvals, cost/health records; produce per-mechanism reuse/adapt/build ADRs | Cannot claim an external system is reliable or a Rikuri security boundary from its docs | F10 and Workflow IR gates mature; representative contention/restart/budget/denial experiments pass |
| S6.3 | Intent-led multi-agent team assembly, project lifecycle coordination, coordinator/evaluator/Strategy Council/Project Graph | Single-worker and control-plane evidence precede teams | S6.2 ADRs and bounded, verified one-worker substrate green |
| S7 | Personal Mesh/hub/CRDT/persistent responsibilities | Replication after recovery and Link | restore/replacement + Link proven |
| S8 | Rich domain/Live Companion/media | Common primitives first | Workspace/knowledge/permissions stable |
| S9 | Windows strong parity/business tenancy | OS abstraction and personal product first | Ubuntu+macOS API stable; personal stages proven |

---

# 6. Stop-the-line / resequencing rules

| Signal | Required response |
|---|---|
| CI/red-test does not actually block | STOP; fix gate before code. |
| Module passes unit tests but has no real caller | NOT COMPLETE; wire it before next dependent feature. |
| Pi/model profile needs model-name branch in kernel | STOP; move behavior to registry/profile/adapter. |
| UI/client becomes owner of task or agent-loop state | STOP; move canonical state and orchestration back into the headless runtime contract. |
| Modality metadata is absent or lost during discovery | mark profile incomplete/unavailable; never silently route vision/audio work as text-only. |
| Relay can read session content or grant node authority | STOP Link release; restore endpoint E2EE and independent authorization boundary. |
| Untrusted project can execute during inspection | STOP LINE; security architecture failure. |
| Descendant escapes parent envelope | STOP LINE; do not trust parent executable name. |
| AISecurity claim exceeds named control/test evidence | mark Degraded/Critical as appropriate; narrow the claim and add the missing regression. |
| Allowed actions form a dangerous sequence | pause, alert, preserve evidence, and require policy-directed review. |
| Local Only cannot prove external network denied | profile unavailable; no silent downgrade. |
| Budget max cannot be bounded | hard-dollar mode unavailable for that access path. |
| Probabilistic evaluator becomes authority | STOP; Jev or any successor remains advisory. Deterministic Budget Governor, AISecurity/policy, tests, and human approvals retain authority. |
| MCP/provider workflow becomes canonical orchestration state | STOP; translate to versioned Rikuri Workflow IR and require policy/security/budget/approval validation before deployment. |
| Retrieval misses real known-answer corpus | spend schedule here before UI breadth. |
| UI starts before preconditions | RESEQUENCE UI and every dependent checklist item. |
| Backup restores forgotten content | STOP RELEASE. |
| Repeated fallback hides dead primary | mark Degraded, fix primary, add tripwire. |
| Item postponed | record disposition + reason + impact + new ordinal/stage + owner + trigger; reorder dependents now. |

---

# 7. Day-90 Definition of Done

- [ ] W1 + F1–F10 pass with durable evidence.
- [ ] Clean Ubuntu checkout builds/tests/starts from docs alone; macOS core is green.
- [ ] Required CI/red-test/secret/checklist/ratchet gates proven.
- [ ] Real imported history searchable with provenance/regression corpus.
- [ ] Untrusted project read/index cannot execute project-controlled code.
- [ ] Explicit project trust promotion + descendant containment proven in Ubuntu envelope.
- [ ] Local model/fake deterministic model completes representative coding task through Pi + Harness Profile.
- [ ] Runtime compatibility and agent compatibility are separately recorded.
- [ ] Headless runtime continues a task across client detach/reattach without losing canonical state or duplicating effects.
- [ ] Capability/modality metadata and a synthetic lead-plus-specialist execution plan pass routing, privacy, accounting, and provenance tests.
- [ ] Cloud/local handoff + Auto/Private/Local Only semantics proven.
- [ ] Hard $1 reservation stop proven.
- [ ] Prompt-injection authority containment proven.
- [ ] Backup -> wipe -> restore and forget-after-restore proven.
- [ ] Every protected-core module is wired, has a Module Contract, health semantics, and tests.
- [ ] A missing handoff ACK, stale/truncated project tail, or detached implementation cannot produce a green status; end-to-end proof links user action to verified durable outcome.
- [ ] Threat/control/test/evidence traceability current.
- [ ] AISecurity regression matrix runs for supported paths; no prior BLOCKED cell silently becomes ALLOWED and material blocks are visible.
- [ ] Agent-facing Foundation commands use versioned machine-stable JSON envelopes and typed errors.
- [ ] Governed harness knowledge access uses scoped Knowledge Service tools, not direct storage/index authority.
- [ ] No unexplained skipped tests/fallbacks/debt/orphans.
- [ ] Every deferred item is explicitly resequenced in the Master Build Sequence.
- [ ] Stage 6 Paperclip evaluation is recorded as deferred research, with a pinned version and acceptance experiments; no Paperclip integration is represented as Foundation-0 delivery.
- [ ] Architecture/docs describe what was actually built.
