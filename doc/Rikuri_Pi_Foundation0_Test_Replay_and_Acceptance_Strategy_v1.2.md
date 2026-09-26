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
4. Run Pi with a local model/fake deterministic model through the Ubuntu envelope.
5. Edit one file, run the declared test command, capture diff and verification evidence.
6. Attempt malicious child-process escape; assert denied.
7. Swap to a second model-family/profile fixture; same core execution path and canonical state remain valid.
8. Mutate executable project metadata and prove trust re-review/invalidation policy fires.

The test distinguishes runtime support from Harness Profile compatibility. A checkpoint may be runtime-compatible but agent-incompatible and must not be shown as `Optimized`.

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
- Foundation threat -> control -> test traceability is current;
- F8 engineering-assurance demonstration passes.
- F9 untrusted-project/GitSpawn-class containment demonstration passes.
- F10 Pi + Rikuri Harness Profile local coding vertical slice passes.
