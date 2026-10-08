# Rikuri Pi — Foundation-0 Test, Replay & Acceptance Strategy

> Repository consistency revision: September 25, 2026. Source version retained; see [reconciliation record](reviews/2026-09-25-documentation-consistency.md).

**Version:** 1.2 Draft  
**Applies to:** Foundation-0 Re-baselined Stage Plan v2.1 / Architecture v1.7 / PRD v1.2 / ADR 0000 and its September 25 addendum

---

## 1. Principle

Rikuri's protected behavior must be testable without asking a nondeterministic model to “behave the same way again.” Live AI is an optional integration dependency; deterministic replay is the core test dependency.

---

## 2. Test layers

Test layers, from narrow to integrated: unit/failure tests → shared contracts and invariants → deterministic replay → adapter/security/recovery probes → owner-visible vertical slices. The Stage Plan v2.1 assigns each layer to stages 00–05; product stages 1–10 are later work.


A core PR must pass without an API key. A green test result is necessary but not sufficient: important changes also require an observed running-system probe, Build Record evidence, documentation update, and the applicable Stage Gate state.

---

## 3. `rikuri-testkit`

Start reusable test components in Stage 00 (S00.08); add billing/network/backup/handoff probes when their owning stages introduce those components:

- `FakeModelAdapter`: scripted turns, tool requests, errors, usage/tokens;
- `FakeProviderBilling`: maximum-cost estimates and actual reconciliation fixtures;
- `FakeTool`: deterministic schema/risk/result/verification;
- `CrashInjector`: abort before/after transaction boundaries and tool commits;
- `ReplayTrace`: recorded model/tool sequence reapplied through the same kernel path;
- `FixtureClock`: stable timestamps where needed;
- `FixtureIds`: deterministic IDs in test mode;
- `NetworkProbe`: proves denied/allowed envelope behavior;
- `BackupFixture`: deterministic small state/content tree.
- `HandoffProbe`: records issued/acknowledged IDs and injects missing, duplicate, stale, and truncated handoffs.
- `StatePromotionProbe`: faults every preserve/validate/candidate/write/read-back/promote/archive boundary and compares last known-good state.

The fake path and live path must share kernel, policy, ledger, budget, and verification code.

---


## 3.1 Shared contract suites

Replaceable implementations must prove compatibility by running the same contract suite. Initial contracts include:

- `Importer`: inspect/parse/validate/idempotency/error atomicity;
- `Repository`: optimistic/versioned access, transaction behavior, migration compatibility;
- `ModelAdapter`: identity, normalized request/response, cancellation, usage/error normalization, budget preflight;
- `VectorIndex`: upsert/delete/query/rebuild without canonical mutation;
- `ExecutionEnvelope`: declared assurance matches observed filesystem/network behavior;
- `BackupStore`: snapshot/restore/verify integrity contract.
- `PiHarnessAdapter`: normalized session/tool/result/cancellation/evidence behavior through Pi.
- `HarnessProfile`: versioned family/task/runtime overlay validation and compatibility metadata.
- `OSCapabilityAdapter`: normalized capability semantics across Ubuntu/macOS implementations.
- `ProjectTrust`: read/inspect versus execute promotion, invalidation, and descendant containment.

A new implementation is not accepted because it has a similar method signature; it must pass the contract suite.

## 3.2 Invariant/property tests

Use property/state-machine testing for claims that should hold across many operation sequences. Foundation invariants include:

- `spent + reserved <= hard ceiling` for every admitted paid request;
- identical re-import does not increase canonical object count;
- event/idempotency semantics never duplicate a committed consequential effect across crash/replay;
- Local Only never enters a cloud-bound Context Package;
- imported/retrieved/model/tool content never self-promotes `instruction_authority`;
- deleting/rebuilding derived indexes cannot mutate canonical state;
- forgotten content cannot be materialized after active key destruction;
- untrusted-project read/index operations cannot cause project-controlled execution;
- project child processes cannot exceed the parent execution envelope;
- adding a compatible model/profile does not require a model-name branch in the protected kernel.
- a failed or incomplete authoritative transformation never promotes the candidate or retires the last known-good state;
- a missing handoff ACK, stale tail, or detached caller never produces a green health or completion event.

Randomized tests are not a substitute for targeted examples; they supplement the golden fixtures.

## 3.3 Red-test drill and gate verification

During repository bootstrap, intentionally introduce a failing test and confirm required CI blocks the change. Remove it and confirm green. The same principle applies when new gate classes are added: prove the gate catches a known violation at least once.

## 4. Golden fixtures

### Import

- minimum and representative ChatGPT export;
- changed ChatGPT schema fixture;
- Claude export (early product Stage 1, after F2);
- Markdown/Obsidian links/frontmatter/Unicode;
- Git manifest with moves/renames (early product Stage 1, after F9);
- duplicate/re-import fixtures;
- malformed/truncated source.

### Context

Golden Context Packages prove:

- source-span provenance survives retrieval;
- imported/history content has `instruction_authority:none`;
- Local Only is excluded from cloud packages;
- Secret is excluded from all model packages;
- package is stable enough for replay but versioned when schema changes.
- a superseded or contradicted remembered claim is marked as evidence, revalidated against current authority when risk warrants, or excluded with a reason;
- stable low-risk evidence does not cause needless network or model verification.

### Security

- malicious `.git/config` / `core.fsmonitor` helper fixture;
- Git hook/helper fixture;
- npm/package lifecycle and build-script fixture;
- Make/Cargo/Python build-hook fixture;
- Docker/devcontainer/MCP/workspace config fixture;
- descendant process attempts filesystem/network/secret escape;
- explicit trust-promotion and trust-invalidation cases.

Injection corpus also includes direct override text, role-hijacking language, exfiltration requests, encoded payload cues, delimiter injection, misleading natural-language explanations, and benign lookalikes for false-positive checks.

### Approval

Given normalized tool + arguments + risk metadata, expected approval consequence output is golden/structural and cannot change based on model narration.

---

## 5. Crash/replay matrix

Inject crashes at minimum:

1. before canonical transaction;
2. after state write but before event append attempt (transaction must roll back);
3. after transaction commit;
4. before tool side effect;
5. after tool side effect but before result event;
6. after idempotency commit but before UI response;
7. during budget reservation/reconciliation;
8. during backup snapshot creation.

Expected outcome is either clean rollback or deterministic recovery without duplicate consequential effects.

---

## 6. Retrieval evaluation

The retrieval benchmark is a product-quality gate.

Each case:

```yaml
id: rq_...
question: ...
expected_sources: [...]
expected_objects: [...]
required_fact: optional
notes: why this case matters
```

Start with at least 20 questions from real imported history. Track exact-hit/source-recall and a small manually reviewed relevance score. Do not hide poor retrieval under an LLM-generated answer.

Every real dogfood failure that matters becomes a permanent regression case.

---

### 6.1 Minimal Knowledge Context Compiler

Stage 02 T02.15/E4 uses ten fixtures with predeclared answer-or-abstain outcomes covering supersession, conflict, current authority, and uncertainty. Required supported answers must be returned; consequential or unresolved conflicts abstain with a reason. Zero unsafe acceptance and no blanket-abstention pass. This is distinct from the Stage 01 retrieval corpus floor.

### 6.2 Context delivery and continuity

Acceptance follows the [context delivery lifecycle](Rikuri_Pi_Architecture_Spec_v1.7.md#195-context-delivery-lifecycle) and its [stage boundaries and evidence](Rikuri_Pi_Architecture_Spec_v1.7.md#1910-stage-boundaries-and-evidence). This September 27 clarification refines fixtures for existing context, handoff and scoped retrieval contracts; it adds no stage or gate, does not reduce §6.1's ten cases, and does not bring later connectors or autonomous topic inference into Foundation-0. **All runtime outcomes below are Not yet proven.**

Use scripted models, fixture IDs/clocks, frozen source revisions and explicit event order. Each case declares the authorized source set, required evidence, expected terminal retrieval status, allowed next action and forbidden effects. Assert the real lifecycle → policy → Knowledge Service → compiler → destination-adapter path; a prompt saying that memory exists, a model's assertion that it searched, or an ACK without a working tool binding cannot pass. Live-model trials separately assess whether supplied evidence helps the task; live prose is not the deterministic ordering or permission oracle.

| Fixture scenario | Required evidence and refusal | Owning implementation / acceptance |
| --- | --- | --- |
| New session or model/profile switch | Under equivalent authorization, the destination receives the versioned knowledge tool manifest, scoped bindings, current task packet and retrieval receipt without a user memory reminder. A known-answer lookup through the advertised tool succeeds; prior decisions and unfinished work survive. Privacy changes recalculate the allowed evidence. | S02.02 / T02.01; S04.14–S04.15 / T04.01, T04.06, T04.16–T04.17 |
| Missing bootstrap, capability or readiness ACK | Separately remove the trusted bootstrap, remove the binding, use a stale manifest/ACK, and withhold readiness. Each produces a typed blocked/unavailable/degraded outcome; no dependent model action starts and health never reports ready merely because another part of the prompt was sent. Retrieved text cannot impersonate a trusted bootstrap/manifest or grant capability. | S04.14–S04.15 / T04.01, T04.13, T04.17 |
| Resume or compaction boundary | Reconstruct the synthetic continuation from Rikuri task state, revalidate knowledge access, attach the current packet and bind the destination's tools again. A truncated summary cannot silently discard an unresolved blocker or substitute for source evidence. | S04.15 / T04.13, T04.16; full long-running workflow orchestration remains product Stage 6 |
| Explicit topic change or crossing | A declared topic/project change or explicit cross-topic request dispatches a scoped search in that turn before dependent reasoning/action proceeds; no wait for a user memory reminder or background timer. Assert a terminal retrieval outcome is handled before continuation. A crossing does not grant access to another project or disclose its restricted names/metadata. | S02.02 / T02.01; S04.14 / T04.01, T04.17; richer topic/semantic retrieval is evaluated in product Stage 1, cross-source assistant intersections in Stage 5, and multi-agent/Project Graph integration in Stage 6 |
| Substantive turn classified unchanged | Script an unchanged-topic signal and a model that never requests memory while the new input has relevant evidence outside the current packet. The runtime still dispatches its lightweight scoped lookup and handles its result before a dependent answer/action. Stop/cancel remains immediate and does not wait for retrieval. | S04.14 / T04.01, T04.17; the scripted signal does not require a Foundation semantic classifier |
| Out-of-order retrieval | Delay an earlier search until after a newer topic generation or policy revision. Its results cannot replace the current packet or unblock the newer turn; cancellation, expiry and bounded retry have explicit terminal outcomes. | S02.02 / T02.01; S04.14 / T04.01, T04.17 |
| Empty, unavailable or incomplete source | Script zero matches, missing permission, offline source, partial coverage and deadline expiry separately. Packet and user-visible status distinguish them; unavailable/unsearched sources cannot be described as searched with no results. Required missing evidence blocks the dependent decision or causes explicit abstention. | S02.02 / T02.01, T02.15; S04.14 / T04.17; live email/calendar/notes connectors remain product Stage 5 |
| Source change, revocation or forget | Change a supporting revision or revoke access while retrieval is pending and after packet delivery. Stale pending results, cached packets and prior conversation material cannot be readmitted to a new invocation; renew the session when retained provider state cannot be safely excluded. Prove authorized unaffected sources still work. Apply existing forget/restore limits; do not claim erasure of external copies. | S02.02 / T02.01, T02.15; S04.14–S04.15 / T04.16–T04.17; existing §9 tests remain required |
| Code conflicts with session narrative | A session says a fix passed, but the synthetic checkout/test evidence disagrees. Cite the current file revision and observed result, mark the conflict and avoid promoting narrated completion. | S02.02 / T02.15; S04.14–S04.15 / T04.16–T04.18; real Git/Claude imports remain product Stage 1 |
| Multimodal evidence | A transcript omits a decisive diagram or frame. Retrieve the original image region or timestamped video/audio span with provenance, or disclose that the modality was not inspected. Transcript-only coverage cannot pass a fixture requiring visual evidence. | Later product Stage 8 source/media integration; no Foundation multimodal gate |
| End-to-end evidence chain | Follow trigger/query → source revisions and retrieval outcome → package/version → readiness ACK → actual invocation/profile → verification/outcome. Replay does not duplicate outcomes; sensitive queries, source titles and payloads do not leak through telemetry. | S02.02 / T02.01, T02.16; S04.14–S04.15 / T04.18 |

Measure timeliness and coverage against the architecture's finite per-turn time, token, query, retry and cost budgets. Expiry produces visible partial/unavailable status; latency targets never waive authority, freshness or required-evidence checks. Record:

- capture-to-searchable lag from an authorized source update/capture to its first queryable revision, including failed updates and last-success age;
- p50/p95 trigger-to-context-delivery latency, separating warm/cold and local/remote paths and counting timed-out or failed attempts separately rather than omitting them;
- missing-critical-evidence rate against predeclared required source/fact sets, plus required-answer success so blanket abstention cannot appear comprehensive;
- stale-evidence acceptance and permission leakage, both with a required count of zero in their deterministic safety fixtures;
- searches, tokens, retries and cost per completed task, alongside verification success and visible incomplete-coverage outcomes.

Freeze and report corpus/fixture revision, index and connector versions, model/profile and context-policy versions, hardware, concurrency, cache conditions and sample counts. Use fixture clocks for deterministic ordering and controlled fault deadlines; measure performance separately with a monotonic clock on the declared benchmark environment. The architecture's initial latency targets are provisional experiments until accepted with measured evidence in the owning implementation stage. Neither a fast incomplete answer nor a successful live-model demonstration closes these contracts by itself.

### 6.3 Document revisions and projection preservation

September 28, 2026 — acceptance for KNOW-005/011 and [Architecture §20.3.1](Rikuri_Pi_Architecture_Spec_v1.7.md#2031-revision-history-and-edited-projections). **Runtime outcomes are Not yet proven.** Foundation S01.03/S01.05/S01.08 and T01.16 cover the minimal import/projection preservation path; richer comparison and proposal UI follows product Stage 2. These cases refine existing source ownership, recovery and forget contracts; they add no Foundation gate.

Use frozen source/base revisions, deterministic IDs and fault injection through the actual importer and projection publication path. Preserve and compare exact bytes/hashes and canonical record identities, not a model's description of a recovered edit.

| Case | Required evidence |
|---|---|
| User-authored note revisions | Import A, edit to B, import B twice. A and B remain addressable captured versions with ancestry/provenance; repeated B creates no duplicate source version. Unknown author stays unknown; uncaptured external history is not claimed |
| Unchanged generated output | Rebuild from recorded canonical versions with the expected generator identity; ordinary generation works without false edit warnings or canonical mutation |
| Edited generated output | Modify a generated file, request regeneration and verify durable preservation/read-back of edited bytes and base references, visible flag and recoverable diff before any replacement; no automatic canonical write |
| Preservation failure / crash / retry | Fail the write/read-back, interrupt before and after preservation and publication, then retry. Keep edited content and last good generation recoverable; no false clean/synchronized state, duplicate revision effects or unpreserved deletion |
| Missing base / unknown file / concurrent edit | Remove the manifest/base or create an unknown file; edit again during regeneration. Detect conflict and preserve current content. Prove exclusive/version-conditional replacement or publish separately; a check followed by an unguarded overwrite fails |
| Privacy, forget and restore | Protected preserved bytes/base/diffs inherit encryption, scope and deletion dependencies. Forget removes managed recoverability; a pre-forget managed restore uses current revocation and refuses/quarantines without it. History is not an exemption from T01.09/T01.10 |
| Later review / promotion | Product Stage 2: retain a recovered edit, export it as an explicitly authorized user-owned note, or propose a canonical revision. A valid authorized proposal succeeds with ancestry; stale bases/permissions require resolution and untrusted file text cannot authorize promotion |

### Foundation projection release evidence

The existing generated-projection rebuild criterion requires S01.08/T01.16 evidence: untouched generated content is rebuildable; externally edited content is preserved or destructive regeneration is refused. Explicit authorized forget follows its deletion contract and must not recreate the forgotten bytes as recovery history.

## 7. Budget tests

Required:

- reservation fits -> call allowed -> reconcile actual -> release remainder;
- reservation exceeds remaining -> no network call;
- retry attempts consume/reserve within same parent envelope;
- provider returns missing usage -> state marked unknown/estimated according to policy;
- price catalog stale/unknown -> hard-dollar profile fails safely if maximum cannot be bounded;
- integer arithmetic prevents floating-point drift.

---

## 8. Egress/envelope tests

Reference Ubuntu tests:

- Local Only test file is readable inside allowed mount;
- worker cannot read path outside envelope;
- worker cannot connect to external IP/DNS/HTTP when `external_network=Denied`;
- no-network assurance failure marks profile unavailable;
- cloud adapter rejects Local Only before HTTP request object is emitted.

macOS experimental tests report actual assurance level rather than forcing Linux parity.

---

## 8.1 Pi/harness/profile and project-trust integration tests

Required Foundation vertical slice:

1. Open synthetic coding project in `Untrusted/Read` state.
2. Search/read through Rikuri/Pi; assert no executable project surface runs.
3. Request execution; render explicit trust promotion showing scope and consequences.
4. Run Pi with a local model/fake deterministic model through the Ubuntu envelope, with Pi placed as E2 decided (inside the envelope, or with its built-in tools and extension loading disabled). Pi's model calls go only through the `rikurid`-owned endpoint; Pi holds no provider key, and the Local Only worker's network policy stays empty.
5. Edit one file, run the declared test command, capture diff and verification evidence.
6. Attempt malicious child-process escape; assert denied. Repeat the hostile inputs against the Pi process itself: no read outside the grant, no network, no disabled built-in tool or extension, no model call except through `rikurid`.
7. Swap to a second model-family/profile fixture; same core execution path and canonical state remain valid.
8. Mutate executable project metadata and prove trust re-review/invalidation policy fires.

The test distinguishes runtime support from Harness Profile compatibility. A checkpoint may be runtime-compatible but agent-incompatible and must not be shown as `Optimized`.

## 8.2 Option C qualification and personal-data acceptance

September 28, 2026 clarification: this table refines the existing Stage 03 contract/failure/security suite and maps related storage/context/release obligations. It does not close a gate or bring later product surfaces into Foundation-0. Apply SEC-020–024 before admitting each access path. Use synthetic personal/financial/media/professional data and credential canaries; never use the owner's real files, accounts or secrets as attack fixtures.

| Boundary / owning tests | Required evidence before admission |
| --- | --- |
| Backend admission — S00.13, S03.02, T03.01/05 | Exact release, license/advisory review, platform/kernel/driver/LSM, immutable production image digests, full effective policy (including providers/defaults), required host privilege/services, grant epoch/expiry, probe version/date and evidence freshness. Record the account running the backend controller (dedicated service account or rootless engine), container-engine socket ownership, and that the accounts for `rikurid`/Pi/model and tool servers are outside root-equivalent groups (`docker`, `lxd`). Backend-native approval is pinned manual at gateway scope and verified at launch. Measure startup, resource/latency cost and operational recovery. Pinning alone is insufficient. |
| Least file access — T03.02/05/09/10 | Allowed read/write and independent read-back succeed; forbidden read/write/delete, read-only mutation, symlink/path tricks and child-process escapes fail. No broad home/media/credential mounts, host process access or container-engine/admin socket access. |
| Private outbound data — T03.03/04/14/16 and Stage 02 F4-lite | Local Only/Secret refusal before external request construction; in-envelope TCP/UDP/DNS/IPv4/IPv6, HTTP/proxy, redirects, loopback/host-service and alternate egress probes as applicable. An unreachable fixture is not a pass: prove a positive network control. Separately test an admitted allowed destination with permitted data, then private-data disclosure to that same destination; Foundation-0 admits no worker destination, so that pair applies from the first later stage that enables one. The local model is reached only through the `rikurid` endpoint; probe that a Local Only worker cannot reach a cloud model through it and that Private material follows PRD §10.1 admission; a loopback or host-service allowance used instead of the socket route counts as a permitted destination; and probe the supervisor's reach to host loopback services. |
| Broker authority — T03.05/09/16 for enabled credential paths (none in Foundation-0; applies from the first later stage that enables one) | Synthetic secret stays outside worker environment/arguments/files/proc access, model context, error/response headers and logs. Allowed scoped operation succeeds; wrong worker, destination, port/path/operation, redirects, credential echo, refresh/replay and expired/revoked use fail. Provider binding cannot expand the grant. Unsupported protocols remain unavailable. |
| Loss and recovery — T03.04/12/15 | Interrupt controller, relay, broker and supervisor separately during active I/O. Exercise grant expiry, revoked permissions, policy updates, failed renewals, stale receipts and rollback/restart. New work refuses; ongoing authority is fenced within the bound fixed by S03.11. Descendants stop, stale credentials do not work, and health/audit/alerts reflect actual state. Test the stop path without model cooperation. |
| Untrusted input — T03.06–11 and F7/F9 | A project, imported note, model or tool output cannot run metadata hooks during inspection, change policy/grants, replace verification or obtain a broader backend. A worker's backend-native policy proposal (OpenShell `policy.local` `POST /v1/proposals`) is refused or leaves the effective policy hash unchanged, and no auto-drafted proposal is approved outside Rikuri. Privilege/capability/resource limits and process-tree escape attempts remain bounded. |
| Data lifecycle — Stages 01/02, F6, T02.16, T03.16 and feature-owning product stages | Classification, audience and retention carry into derived text/media metadata, summaries, embeddings, prompts, logs, temporary files and backups for each implemented path. Canary absence is checked at serialized requests and actual sinks. Forget/restore consults current revocation; mutation/racing-write/failure tests preserve valid originals and verify rollback/read-back. Later media/database paths need their own fixtures before activation. |
| Telemetry and service egress — T03.16 | Optional vendor/product telemetry starts disabled; capture worker and controller/gateway/supervisor/broker outbound activity under normal operation and failures. A worker no-network result alone is insufficient. Enabled diagnostic export requires scoped consent, minimized payloads and secret exclusion. |
| User trust and upgrades — T03.12/13/16 and Stage 05 release | Denial and loss of protection are understandable, linked to the real event, and cannot appear Healthy. Relevant backend/kernel/image/provider/configuration updates invalidate affected qualification; rerun mandatory suites and rollback without resurrecting authority. Requalify at the applicable tier of the Engineering Standard's [two-tier rule](Rikuri_Pi_Engineering_Standard_v1.2.md#security-assurance-and-upstream-maintenance): this full table is tier 2, and a change not listed as tier 1 is tier 2. Profiles stay unavailable until the applicable tier passes; receipts record the running kernel (`uname -r`). |

For every denial case, distinguish enforced denial from a timeout, missing file, broken fixture or unsupported test. Use baseline/positive controls and independently observe side effects. Policy proofs record their modeled scope and unsupported fields; they do not substitute for runtime tests. Publish measured revocation times and permitted-task success/failure, not only attack counts. Resource exhaustion and failures are exercised in bounded disposable environments.

The [OpenShell v0.1.2 receipt](../docs/security/evidence/2026-09-28-openshell-v0.1.2/README.md) supplies ten earlier smoke outcomes, including a successful ordinary-container baseline. It does not cover this qualification table: permission revocation, credential brokering, controller failure, DNS/IPv6, permitted OpenShell network access, control-plane egress and the real Rikuri caller remain unproven. Do not count a disabled/unimplemented surface as tested protection. Independent review and release gates remain mandatory.

September 28 review follow-up ([ADR addendum](Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md#september-28-2026--option-c-review-follow-ups-and-foundation-0-credential-scope)): the receipt's run also auto-drafted one network proposal (its later status was not captured), showed an unexplained provider-environment settings change with no providers attached, and denied network twice over (container `network_mode: none` plus OpenShell policy). Qualification must explain the settings change and test the configuration Rikuri will actually ship, including the `rikurid` model endpoint path.

## 9. Forget/restore tests

Sequence:

```text
create objects
-> backup A
-> forget object X
-> purge/rebuild indexes
-> backup B
-> wipe
-> restore B
```

Assert X cannot be materialized and is absent from projections/indexes/search. Tombstone/audit fact may remain. Add a documented test for what happens if the user explicitly restores an unmanaged old export/backup; Rikuri cannot revoke external copies.

---

### 9.1 Managed pre-forget restore

In addition to restoring post-forget backup B, restore pre-forget managed backup A using trustworthy durable current revocation state. X must remain unavailable in materialization, projections, indexes, and search. Repeat A restoration without trustworthy current revocation state: restore must refuse or quarantine, never silently resurrect X. This is mandatory T01.10/F6 and reruns at release. Unmanaged exports remain outside this guarantee.

## 10. i18n tests

Foundation-0 ships English only but tests:

- no shell component hardcodes visible English outside resource files;
- pseudo-locale expands strings and exposes clipping;
- Unicode/CJK sample content round-trips through imports, DB, FTS, projection, and UI even before translated interface strings exist;
- core errors use stable codes + structured fields, not English-only logic.

---


## 11. Engineering-assurance tests

Quality-process claims are also testable:

- Build Record template validator: every unchecked required item has an explicit skipped/deferred/resequenced disposition with reason, dependency impact, new stage/ordinal, owner, and reactivation/acceptance condition;
- master/stage checklist ordering validator flags a resequenced item whose dependent downstream items were not moved;
- Stage Gate validator: COMPLETE requires linked acceptance evidence;
- module-contract check for protected-core crates;
- docs/RAG source touched or explicitly N/A when behavior/API/help contract changes;
- debt ratchet cannot increase silently;
- forbidden residue checks catch skipped tests, debug residue, obvious backup/swap files, and selected policy violations;
- traceability check flags Foundation security promises with no named control or no test;
- smoke test verifies distinct content/behavior markers, not only process liveness;
- last-success health fixture proves absence-of-success becomes Degraded.

Automated checks should be conservative about orphan deletion. They may identify candidates; ambiguous code is reviewed before removal.

## 12. Evidence model

A checked verification item should point to a durable receipt such as:

- CI run/result summary;
- test count and command;
- replay trace ID;
- smoke/probe output;
- benchmark result;
- screenshot/artifact when visual behavior matters;
- Build Record;
- ADR;
- Stage Gate Record;
- incident/tripwire record.

Agent prose such as “tests pass” or “looks fixed” is not itself evidence.

### 12.1 Collaborative Intelligence evidence contracts

T00.12 proves stable identity and event/outcome references across replay. T02.16 proves execution/profile identity, explicit unknowns, verification separate from completion, cost/retry/intervention and delayed feedback links, and privacy-minimized telemetry. T04.18 proves the same chain through Pi, profile swap and cloud/local handoff, without duplicate outcomes or candidate self-promotion. Use deterministic fixtures; removing a required reference or allowing sensitive payloads into ungoverned telemetry must fail. These tests capture evidence; they do not require inference, learned routing, incubation, or Lightbulbs.

## 13. Release gate

A Foundation-0 build is not releasable/dogfood-ready unless:

- Bootstrap, W1, F1–F10, and the F4-lite checkpoint pass with linked receipts;
- the complete eight-point cross-component crash matrix passes at release;
- platform-neutral macOS core build/unit/contract tests pass; native macOS containment remains experimental;
- T00.12, T02.16, and T04.18 evidence-contract tests pass;
- all migrations from supported test versions pass;
- retrieval baseline does not regress below the frozen threshold;
- no security fixture can acquire authority;
- hard-budget tests show zero calls beyond reservation ceiling;
- backup/wipe/restore verification passes;
- generated projection can be deleted and rebuilt;
- docs and ADR index are current;
- required Build Records are COMPLETE/ABANDONED with explicit disposition;
- Stage Gate Records match actual acceptance evidence;
- no unexplained skipped tests or debt-ratchet increase exists;
- Module Contracts exist for protected-core crates;
- Scrap Sweep/clean-tree review is complete;
- Foundation threat -> control -> test traceability is current, including the [personal-data threat model](../docs/security/THREAT_MODEL.md);
- every enabled sensitive path has current Option C qualification evidence; unsupported/stale paths are unavailable, known boundary failures block release, and public protection claims match the admitted platform/configuration;
- F8 engineering-assurance demonstration passes.
- F9 untrusted-project/GitSpawn-class containment demonstration passes.
- F10 Pi + Rikuri Harness Profile local coding vertical slice passes.

## 14. Post-Foundation autonomy and repair acceptance

September 27, 2026 — future acceptance design for PRD SEC-013–019 and [Architecture §16.5–16.9](Rikuri_Pi_Architecture_Spec_v1.7.md#165-scoped-autonomy-and-user-presets), including [defect-scoped regression evidence](Rikuri_Pi_Architecture_Spec_v1.7.md#168-defect-scoped-regression-evidence). **All runtime outcomes here are Not yet proven.** This section adds no Foundation gate and grants no current agent new authority. The current AGENTS owner-only fix-lock release, shared guard and report-only verifier remain unchanged. Under September 30 resequencing, existing scoped supporting execution remains Stage 1; the advanced single-worker repair lifecycle belongs to the optional Stage 8 developer pack after test/trust contracts and explicit policy/ADR reconciliation; richer UI follows Stage 2 and durable branching/waits/multi-agent behavior follows Stage 6. These dependencies cannot be bypassed by labeling a run unattended.

Use declared task/environment/grant fixtures, deterministic clocks/IDs, scripted workers, frozen evidence/candidate revisions and fault injection through the real policy → envelope → task/approval → verification/controller path. A model saying it has permission or that tests pass is not an oracle. Prove containment through the actual supported tools and descendant paths, including tools other than shell. Run only safe scratch fixtures; the user's live node/project is not a test target.

| Case | Required positive evidence and negative/failure check |
|---|---|
| Routine development | One grant permits repeated edits, debugging, collected tests/builds, approved project dependency installation, owned development-process restart and declared disposable-fixture reset without per-command prompts. Same interpreter outside granted scope is denied; syntax alone cannot confer authority |
| Real isolation | Development workers cannot access undeclared live mounts, credentials, service endpoints, host/global settings or external destinations. Unavailable/stale enforcement makes the dependent mode unavailable; no unrestricted-host fallback |
| Readiness and UI | Safe preflight exposes a missing tool/cache right, expiring credential, budget/quota limit, repository lock and external harness/organization block with governing rule and understandable remedy. An immediate required blocker cannot produce Ready. Once/task/project choices expose effective scope and expiry without hand-editing policy files; novices can distinguish them in usability trials |
| Candidate versus activation | Prepare and verify a consequential candidate in isolation while live activation remains pending. The development grant cannot perform the live effect or mutate active policy. Candidate/target/precondition changes after approval invalidate its application; tests alone do not authorize deployment |
| Independent progress | With one operation waiting, an authorized independent task makes verified progress without interfering with the blocked state. A dependent operation, shared-state conflict, required red gate or environment-wide enforcement failure blocks the affected work. No invented tasks, denial retries through another tool or silent partial-completion claim |
| Scope of evidence seal | The original failing regression, fixtures, expectations and controlled runner references remain intact while ordinary new/unrelated tests evolve. Collection counts and executed test identities prove the sealed test ran; the initial failure must represent the intended defect |
| False-green attempts | Mutate discovery/runner configuration, disable assertions, skip the regression, replace a fixture/baseline, fabricate a green result or alter verifier policy. None completes the cycle. Re-run the preserved regression and relevant surrounding checks against the exact candidate with independently recorded evidence |
| Incorrect original test | Detect a malformed/wrong test before sealing when possible. After sealing, amendment_required retains original evidence and prepares a separate correction; the builder cannot accept its own amendment. Authorized independent work continues while required review of the affected cycle is pending |
| Automatic cycle completion | Under a separately admitted development policy/grant, a trusted controller completes a verified cycle without repeated owner-release prompts, retaining evidence. Missing, failed, stale or worker-authored approval evidence cannot pass. Completion does not deploy, waive gates or operate today's fix-lock |
| Expiry, revoke and handoff | Expire/revoke the grant before a request, during work and before activation; replacement models, child workers and resumed sessions cannot restore authority from old context. Safe checkpoint/shutdown follows its own bounded policy; scope cannot grow through retries or profile changes |
| Trust and new assets | Changed executable metadata triggers S03.07/T03.11 re-review/invalidation. Any future automatic revalidation must independently prove the unchanged containment boundary. New services, secrets or financial authority require fresh admission; an unchanged project name/preset is insufficient |
| Service versus custody | A synthetic node/API fixture with no exposed financial authority permits admitted development/query work while live restart, durable data deletion, credential changes and fund actions follow their distinct grants. A zero-balance label cannot authorize signing or waive service/data protection |
| Crash, duplicate and later decision | Interrupt after evidence sealing, verification and pending approval; recover last known-good state with exact candidate/grant identities. Duplicate completion/approval events cannot close another defect or duplicate a live effect. Changed candidate after verification requires re-verification |
| End-of-run accountability | Deadline/idle waiting is bounded; no indefinite paid polling. Digest separates verified changes, failed gates, pending decisions and remaining work, identifies the real blocking layer, and reports budget use. A waiting/failed task cannot be marked complete because its time budget ended |

At product Stage 8’s optional developer pack, exercise the advanced single-worker portions; Stage 6 must add actual branch/wait/rejoin and multi-agent fixtures before claiming that behavior. Preserve the existing hard-budget, privacy, project-trust and no-self-approval suites. Separately evaluate policy/UX with non-expert users: can they predict what will happen, revoke a grant and understand a block without interpreting shell syntax?

Record per-case policy/grant/envelope, candidate/evidence/runner/verifier versions, allowed and denied effects, actual tool/verification receipts, recovery outcome and constraint source. Measure time to verified progress, avoidable approval interruptions, false allows/denials on a labeled corpus, readiness misses, unresolved wait time, correction/review effort and cost. Zero prompts is not a success metric by itself. Adopt thresholds at the owning implementation stage with observed evidence; documentation, synthetic script output or a single successful demo does not prove safe unattended operation.

## 15. Post-Foundation capture and shared conversation acceptance

September 28, 2026 — future acceptance for KNOW-008–010, COLLAB-001–006 and [Architecture §20.6](Rikuri_Pi_Architecture_Spec_v1.7.md#206-capture-classification-and-conversational-evidence)/[§38.6](Rikuri_Pi_Architecture_Spec_v1.7.md#386-shared-humanai-project-conversations). **All runtime results are Not yet proven.** Product Stage 1 evaluates text classification/semantic changes after its existing retrieval prerequisites; Stage 2 source cards; Stage 3 transport; Stage 5 connectors/personal contributions; Stage 6 S6.C1–C4 shared conversations; Stage 8 richer media. Enterprise Stage 9 extends this base. No team service or semantic/media classifier is added to Foundation.

Use isolated participant/source fixtures, deterministic clocks/IDs, declared source ACLs and membership/history versions, scripted classifier/agent outputs and real supported transport/adapter entry points. Include positive and negative cases; successful mocks or a working owner remote-control path alone cannot close the collaboration gate. Do not use private live conversations as adversarial fixtures.

| Case | Required positive evidence and denial/failure behavior |
|---|---|
| Capture and extraction | Explicit Save/share creates one durable, scoped source/artifact with provenance; duplicate retry is idempotent. Inject failed/pending extraction and index lag; original stays recoverable, status reports the actual gap, and no saved/searchable acknowledgement precedes success. Merely viewing content creates no unauthorized capture |
| Classification and correction | Valid descriptive metadata improves known-item retrieval; a wrong label can be corrected and model reprocessing retains the correction. Reject malformed candidate output and invented source links. No classifier tools can execute, publish or accept a decision; its failure cannot hide the original from baseline search |
| Useful contributions and silence | Retrieve a predeclared relevant prior note and deliver a cited card with reason and current source status. Deduplicate repeated triggers, coalesce bursts and cancel late results after a topic switch. Low-relevance/duplicate material stays quiet; mention-only mode never activates unsolicited posting. Grant expiry/revocation stops automatic contributions |
| Audience boundary | Agent can privately read source X but one conversation recipient cannot. Shared post, title, thumbnail, citation, summary and history do not reveal X. Allowed source Y can be shared successfully. Summarization, a link-only card and another agent's context cannot bypass source restrictions |
| Membership and history changes | Add a guest with an explicit limited history grant, remove a member, revoke a device, and change source ACLs while retrieval/post delivery is pending. Newcomer receives only admitted history; former access cannot authorize queued posts, subscriptions, cached attachment reads or stale model context |
| Instruction and action authority | Retrieved note/file contains instructions to run commands or invite a person. Those bytes remain evidence; no tool or membership effect occurs. Authorized humans can issue scoped tasks; mentions/guest posts cannot approve actions, expand scope or override another task owner. Conflicting instructions are surfaced through ownership/approval rules |
| LAN and remote vertical slice | Two separate human principals, one on LAN and one outside, join an agent on one authoritative node, share a file and cited note, and inspect the same project state. Execute over the initial Tailscale path; native direct/relay/fallback each needs equivalent evidence before being offered |
| Transport and endpoint claims | Verify endpoint authentication, ciphertext-only relay handling and tamper/replay rejection on each enabled path. Simulate direct failure and route switch; no insecure downgrade or identity/scope change. The node's plaintext processing is declared. Cloud-worker calls require independent destination admission; Local Only cannot leave through LAN, tunnel, relay or cloud context construction |
| Disconnect, retry and outage | Disconnect before/after message commit and acknowledgement; repeat event/operation IDs. Resume without duplicate posts or side effects; pending differs from delivered. Node failure remains unavailable rather than silently moving canonical data/context to an unapproved server |
| Media at Stage 8 | Recover a screenshot from remembered visual content with the actual image region/source and annotations; test a diagram absent from its transcript. Failed/unsupported OCR, audio or video remains partial. Meeting capture is visibly activated and stoppable; text/file success proves no live group-call capability |

Record policy/source/membership/history revisions, destination and transport identity, minimized retrieval/contribution IDs, captured/received/committed/delivered evidence, retries, source age, useful-card latency, cost, duplicate/false interruption counts and revocation outcomes. Evaluate usefulness and known-item recall against lexical/on-request baselines with a frozen corpus; require both legitimate sharing and zero unauthorized disclosure in the boundary fixtures. Adopt explicit measured relevance/latency/attention thresholds at the owning stage, not arbitrary confidence labels.

Owner: product owner and owning-stage implementer. S6.C4 closes only with the actual LAN/outside human–AI slice and negative/recovery receipts after S6.C1–C3 and prior gates. Protocol/library assessment records pinned version/license, threat model, lifecycle/integration cost, metadata leakage, replay/key rotation/revocation and lost-device behavior. Unavailable native paths remain unavailable; external human security review remains part of the separate public-exposure gate. Already disclosed external plaintext cannot be recalled by membership revocation.

# 16. Post-Foundation creative-studio acceptance

These are planned contracts for PRD STUDIO-001–010, not tests run or extra Foundation-0 gates. Implement deterministic core fixtures without API credentials; supported live adapters need separate conformance receipts. Existing privacy, budgets, authority, revocation/recovery and context delivery suites remain applicable.

| Requirements | Legitimate vertical slice | Rejection / failure evidence | Delivery |
| --- | --- | --- | --- |
| STUDIO-001, 009 | Fragment-first creator uses linked text/scene/world views; restart and model change preserve objects; portable export and baseline search work without paid service | Closing a view cannot delete the work; cancellation cannot block existing data; unsaved or failed capture cannot show saved | Stage 2 S2.C1–C2, C4 |
| STUDIO-002, 003 | Fork two distinct scenarios from one version; compare assumptions/consequences; choose or selectively merge with conflict review and retained rejected candidates | Stale parent, unresolved unapproved character-knowledge conflicts, partial save, injected source instructions and unapproved canon change refuse promotion visibly; recovery preserves last good version | Stage 2 S2.C3; parallel/overnight Stage 6 |
| STUDIO-004 | Requested reframing cites supplied evidence and rivals; later learned rule shows supports, negative cases, falsifier and revalidation | Summary cannot replace original; duplicate evidence cannot inflate confidence; contradicted/stale/unsupported heuristic downgraded; no insight allowed | Requested Stage 2; learned Stage 10 after 4/6 |
| STUDIO-005 | Domain fixture separates scientific baseline, explicit parameters, intentional fictional departure and modeled consequences | Unsupported fact, wrong units, impossible parameter/domain and tool failure cannot be reported verified; creator can knowingly retain fictional departure | Stage 8 |
| STUDIO-006 | Import/share a supported source revision with timestamp, provenance, extraction status and coverage; baseline retrieval survives adapter failure | Missing/offscreen/deleted/unsupported history and failed OCR remain incomplete; denied permissions prevent capture; revocation invalidates derived retrieval | Stage 1/5/8 by source type; Foundation imports unchanged |
| STUDIO-007 | Scoped relevant-discipline/source suggestion carries evidence and budget; suppress duplicates, cancel/expire cleanly | Local Only refuses search before HTTP; sensitive query without egress grant refused; irrelevant suggestions do not create tasks; expiry cannot continue research | Stage 5; learned incubation Stage 10 |
| STUDIO-008 | Same Pi role/identity resumes via a replacement admitted model with visible run history and current grants | Model/worker replacement cannot inherit expired rights; delegation and unattended work wait for governance; removal/revocation stops pending effects | Identity Stage 2; governance Stage 6; responsibilities Stage 7 |
| STUDIO-009, 010 | Scoped plugin opens local work and exports a versioned brief with rights, decisions, assets and unresolved issues | Host input cannot bypass canonical transactions; denied disclosure cannot leak via plugin/search; export is not permission to code, publish or spend | Prototype/brief Stage 2; distribution Stage 5; richer specs Stage 8 |

Evaluate creator usefulness versus simpler chat/document baselines with predeclared tasks and human feedback. Track retrieval/resume errors, unresolved contradictions, lost alternatives, cost/attention and source coverage, not only generated output volume. Advanced reflection additionally meets the existing Stage 10 comparative evidence gate. Public exposure still requires Engineering Standard release evidence and external human security review.

# 17. Security draft addition acceptance

October 1, 2026: future acceptance for the October 1 security-draft refinements (folded into SEC-020–024 on October 3) and SEC-028–031, with Architecture §27.14. These do not add Foundation stages or claim runtime tests exist. October 2 recheck: where a row overlaps [§8.2](#82-option-c-qualification-and-personal-data-acceptance), §8.2 governs backend qualification from Stage 03 onward and this table adds only the listed non-backend fixtures; each row names its overlapping §8.2 boundary. Fake and live adapters must share policy, ledger, budget and validation paths. Each enabled control needs authorized, denied, bypass and failed-enforcement evidence; no detector output or configuration file is a security oracle.

| Requirements | Allowed / denied / bypass / failure fixture | Delivery and dependency |
| --- | --- | --- |
| SEC-021 (folded SEC-025) | Valid scoped actor action succeeds; authenticated but unauthorized actor denied; forged role/tool manifest and child escalation denied; required policy/enforcement unavailable refuses visibly; a worker's backend-native policy proposal leaves effective policy unchanged (T03.05) | §8.2 Untrusted input. Existing S02.01/S02.04/S02.08–09 and S03.05/S03.08 unchanged; multi-actor delegation product 6 |
| SEC-020, SEC-023 (folded SEC-026) | Authorized scoped share succeeds; read-only grant denies export; copied/derived/embedded material and archive-upload sequence retain source restrictions; unknown/conflicting mapping or failed classification cannot downgrade | Existing S02.02–04/S02.11 and S03.10 unchanged; connector movement product 5, sharing 6, rich media 8; no universal host DLP promise; §8.2 Data lifecycle |
| SEC-022 (folded SEC-027) | Authorized broker operation succeeds without raw secret exposure; wrong service/task, expiry or revocation denied; redirect/error/log/cache paths cannot reveal secret; absent broker or upstream revocation stops profiles requiring that guarantee | §8.2 Broker authority. Foundation-0: no worker credential (ADR 0000 September 28 decision 4), rows N/A; broker qualification before the first credential-bearing worker, expected Stage 5 connectors or the Stage 8 developer pack |
| SEC-028 | Fresh eligible account/endpoint policy permits permitted context; stale/unknown required assurance, provider switch and Local Only/Secret all deny; adapter bypass cannot construct forbidden HTTP; registry failure refuses dependent routing | Existing S02.03/S02.07/F4-lite unchanged; richer provider registry product 4/5 |
| SEC-029 | Granted Link/application operation works; enrolled but unauthorized peer denied; raw-IP/DNS/proxy/redirect/alternate-interface and open-session expiry/revocation tested; controller loss isolates within the adapter's measured revocation bound | §8.2 Private outbound data and Loss and recovery. Existing S03.08 no-network proof unchanged; Link product 3, scoped network delegation 6, mesh 7; attestation/enterprise 9 |
| SEC-030 | Trusted scoped approval and configured step-up authenticate intended actor; wrong/stale approval, model-written consequences and biometric-only override denied; no recent probe cannot show protected; unavailable auth/enforcement refuses dependent high-impact action | Existing S02.08 and S03.02 assurance; richer UX product 2/5/6, hardware-auth enterprise 9 |
| SEC-031 | Valid candidate durably writes/read-backs and activates under verified enforcement; unauthorized/stale policy update denied; crash/partial external-rule update remains restrictive; rollback/old backup cannot restore revoked rights | §8.2 Loss and recovery and User trust and upgrades. Foundation transactional/revocation/envelope gates unchanged; each later mutable security adapter before activation |
| SEC-024 (folded SEC-032) | Evidence correctly links identity/policy/decision/outcome and accessible redacted receipt; secret telemetry and unauthorized reads denied; tampered receipt/stale backend/update detected; interrupted component cannot stay false-green | §8.2 Telemetry and service egress and User trust and upgrades; revalidation at the Engineering Standard tier. Existing T00.12/T02.16/T04.18 and T03.16 unchanged; all later adapters before advertisement; learned anomalies remain gated research |

The raw AISecurity matrix, native platform probes and live external policy behavior must be obtained and rerun at their owning gates; descriptions in imported drafts are not test receipts. Runtime and public-release assurance remain Not yet proven.

# 18. Writer IDE acceptance refinement

October 1, 2026: [Writer IDE Specification v0.1 §10](Rikuri_Pi_Writer_IDE_Spec_v0.1.md#10-acceptance-and-delivery-dispositions) defines A01–A11 for WIDE-001–012 refining STUDIO-001–010. §16 scenario generation means non-prose structural alternatives in creative mode; human canon approval does not authorize agent manuscript composition. Allowed short teaching illustrations and denied scene/serial ghostwriting must both be exercised. Business generation is a separately admitted later mode.

Owning stages: Stage 2 S2.C1–C4 identity/anchors, versioning/recovery, text questions, chosen lessons/cards, basic references and portable solo work; Stage 6 S6.C1–C4 multi-human/agent terms and rights/action/processing boundaries; Stage 8 admitted voice, richer scholarly publishing and domain depth. Tests must include stale references/proposals, split/merge/copy lineage, subtree restoration, current revocation, consent and no-cloud fallback, regeneration failures and export omissions. Core fixtures are deterministic without credentials; live adapters need separate receipts. No additional Foundation gates or runtime passing claims. All cases remain Not yet proven.

# 19. Harness intent and evidence contract acceptance

October 2, 2026 — future acceptance for HAR-001–004 and Architecture §11.1–11.3. These are specified fixtures, not implemented or passed tests; Foundation gates remain unchanged. Use deterministic scripted adapters for transitions and exact-profile task evaluations for model behavior; fake/live paths share authority, budget, ledger and verification contracts.

| Requirement | Allowed and adverse evidence | Delivery / predecessor |
| --- | --- | --- |
| HAR-001 | Ambiguous “update” asks assessment versus applied edits; answered target persists through switch/compaction; minor reversible preference proceeds visibly; factual unknown uses inspection. Conflicting constraints reopen clarification; pending independent work proceeds while consequential edits wait; silence and source-injected answers cannot grant rights | Product 2 task surfaces after Foundation context/authority contracts; multi-agent propagation product 6 |
| HAR-002 | Current node-state question uses fresh target-bound measurements; source explanation uses relevant RAG; mixed question joins both. Reject fabricated readings, wrong units/target, stale snapshots, irrelevant retrieved citations, contradictory sources, missing tools and citation-only claims. Show supported partial answer or visible refusal, while supported known-answer cases succeed; always-abstain fails | Minimal context evidence uses existing T02.15–16/T04.18 without expanding them; general classification product 2, connector/instrument adapters product 5, exact small-model comparisons product 4 |
| HAR-003 | Declared verification admits a durable supported result; failure enters bounded repair within grant; cancellation, revoked grant, exhausted budget/retries and unavailable verifier stop dependent actions visibly. Research support and creative creator judgment cannot be replaced by coding-test success or model agreement | Foundation single-role slice unchanged; product 2 task lifecycle; advanced developer repair product 8 under §14 policy prerequisites |
| HAR-004 | Adapter retains intent/constraints and records exact revisions; evaluated compatible profile admitted by authorized controller and can roll back. Detect dropped constraints, stale evidence, unsupported adapter capability, regression and source-derived policy change; candidate cannot self-promote or modify security/budget | Foundation profile swap T04.02/T04.18 unchanged; measured tuning product 4; learned recipe candidates product 10 after 4/6 |

Track required-evidence recall/support, wrong or unsupported completion, justified clarification, missed material ambiguity, user correction/interruptions, verified task usefulness, retries, latency and cost. Declare task sets and comparison baseline in advance; sparse observations and self-reported model confidence cannot establish reliability. Local Only and Secret boundaries apply to evaluation, query text, receipts and fallbacks. Instrumentation installation/execution needs separate admitted authority; this change does not operate the owner's Bitcoin node.

# 20. Mission charter acceptance

October 8, 2026: future acceptance for PRD MISSION-001–006, P32–P34 and Architecture §26.5, from the owner's [Mission Charter](Rikuri_Pi_Mission_Charter_v1.0.md). These are specified fixtures, not implemented or passed tests. Foundation gates, S/T/E IDs and dates are unchanged. Core fixtures are deterministic and need no credentials; fake and live adapters share the same admission, privacy, budget and verification paths.

| Requirement | Allowed and adverse evidence | Delivery / predecessor |
| --- | --- | --- |
| MISSION-001 | Each entry leaving the parking lot, and each post-Foundation re-estimation, records a decision-test answer. Adverse: an entry promoted with no recorded answer; a decision-test answer used to justify skipping a gate or weakening a security boundary | Parking-lot "Leaving the lot" procedure, first at the review held with S05.10 (a process step outside S05.10's acceptance); each post-Foundation re-estimation |
| MISSION-002 | A Stage 2 report shows work completed by AI separately from lessons the user chose to save and outcomes the user declared. Adverse: a capability or success figure computed from tokens, agent count, tool calls or generated volume; before Stage 10, any capability or competence figure inferred from the user's history; a lesson or practice record created without the user's choice; encouragement text not grounded in the user's actual progress | Stage 2 with WIDE-006/008 cases; Stage 10 research for measured capability growth |
| MISSION-003 | A capability verified on one OS/architecture shows Verified there with versions, date and receipt, and Implemented or Architecture-supported elsewhere, with its security assurance level reported alongside. Adverse: Verified set by a compile-only or launch-only CI job; Verified carried over to an OS or architecture version it was not observed on; an Unavailable capability shown without its reason; an availability state presented as, or implying, a security assurance level | Documentation review of new or revised claims from this change; runtime capability reports from product Stage 2 (no Foundation S05.02/S05.08 scope) |
| MISSION-004 | A vendor runtime adapter answers an admitted task through the normal router, budget and ledger; uninstalling it leaves projects and knowledge intact and marks the capability degraded; a vendor runtime that executes inside an admitted network-Denied envelope serves a Local Only task for that execution. Adverse: a vendor layer writes canonical state or holds the only copy of any project data; Local Only work served by a vendor runtime outside a network-Denied envelope for that execution, or after a runtime or OS update without requalification; Local Only-derived data uploaded once the runtime leaves the envelope or its network returns; Private or Local Only data sent to a layer whose memory, history or personalization retention is not disabled and verified (SEC-028), or a vendor-retained copy of Private or Local Only data surviving a managed forget; a built-in assistant acting outside Rikuri's grants (§13.5, §27.3); a vendor runtime bypassing admission, budget reservation or provenance; loss of the runtime reported as healthy | Product Stage 4 Model Lab adapters; Windows-specific adapters Stage 9; Local Only proof reuses the Stage 03 no-network envelope tests (T03.03/T03.04) |
| MISSION-005 | The same canonical project exported on one OS/architecture imports and continues on another verified pair. Adverse: an architecture-specific assumption in a kernel contract or canonical format; an ARM64 or Windows claim above Implemented without its own receipt | Stage 7 second-device fixtures across the pairs verified by then (Not yet proven while fewer than two pairs are verified); Windows pairs Stage 9; ARM64 pairs when sequenced through the parking lot |
| MISSION-006 | In Stage 2, creative-mode advice and teaching (WIDE-006) and text contextual questions (WIDE-007) behave as the user set them across a session and a model change, and reflection happens when the user asks (STUDIO-004). Adverse before Stage 10: Rikuri changes mode or level of help based on competence learned from the user's history; a stalled-project or recurring-failure observation learned from the user's history without a request; any learned adaptation that cannot be inspected, corrected or revoked. Deterministic status signals such as Idle, Blocked or needs-attention are not learned adaptation and are not adverse | Stage 2 creative-mode advice, text questions and requested reflection (WIDE-006/007, STUDIO-004); voice Ask and Dictate Stage 8 (WIDE-007); general mode selector via the parking lot; learned adaptation Stage 10 after Stage 4 evidence and Stage 6 governance |

The PRD §22 measures for capability expansion, independent understanding and creative realization need a declared method, baseline and opt-in evidence before any result is reported. No measure about the person may come from hidden profiling.
