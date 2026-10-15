# Provider-neutral worker adapter and evidence handoff — draft v0.1

Status: **design draft; real-worker seam unimplemented** under owner-accepted ADR0006. An [isolated simulator](ADAPTER_SIMULATOR.md) now exercises a subset. This document specifies the real-worker seam; it does not change TaskContract v1's schemas, Stage00 API or Orchard execution. The [shared contract](README.md) and maintained architecture remain authoritative. Full Ubuntu gate and real adapter qualification remain pending.

## Authority and ownership

Rikuri's controller owns run identity, admission, leases, current revocation, budget reservation, cancellation, terminal status and canonical promotion. The adapter translates admitted intent/profile into a supported runtime and reports observations. It cannot issue grants, choose independent verdicts, expand budgets or write canonical state. The independent gate runner owns test criteria and verified evidence. Orchard presents those records; its Apply requires separate exact-diff consent.

Pi remains the primary/default production harness. Native Codex/Claude are explicit opaque exception adapters with tested capability contracts. Provider access/session identity is not a Rikuri task/run ID or authority. These operations are abstract interface names, not registered Stage00 RPC commands.

## Proposed operations

| Operation | Trusted input | Returned observation | Required behavior |
| --- | --- | --- | --- |
| describe | Pinned adapter/runtime/profile versions | Supported operations and tested capability claims | Each claim links to dated evidence; unsupported or unknown is explicit |
| prepare | Validated contract, immutable source snapshot, trusted policy/grant handle and reserved budget handle | Prepared handle bound to contract digest, source revision, profile and envelope | Resolve current authority outside task text; validate all constraints; no model turn |
| start | Prepared handle, current admission, controller-owned run/lease and one-use attempt token | Started acknowledgement and runtime session reference | Start once only after current admission and budget reservation; persist ack before reporting started |
| observe | Authorized run reference and bounded cursor | Sequenced bounded observations and visible gaps | Source-label worker claims and tool facts; no observation promotes success |
| cancel | Authenticated controller cancellation token | Requested/acknowledged/stopped or unknown | Cancel actual descendants under envelope; do not claim stopped on request alone |
| collect | Stopped run and trusted immutable snapshot capture | Candidate artifact manifest, execution observations and usage evidence | Trusted capture checks paths, hashes, sizes, modes and protected files; worker manifest is only a claim |
| release | Controller-approved retention disposition | Resource-release receipt | No automatic deletion of artifacts, recovery copies or owner data |

Handles are trusted service references, not permission-bearing fields in submitted JSON. Authentication, expiration, current revocation, leases and budget reservation are pending service contracts; the fixture's runner-authored grant dictionary is not their implementation. Production schema/version registration is a separate task. Unknown operations/fields fail closed.

prepare may launch only an explicitly admitted diagnostic process when necessary to prove capabilities; it must not create a model turn or run project-controlled startup code merely to inspect a repository. A native server bootstrap that loads config/hooks/tools requires its own trusted-host audit, not a claim that filesystem restrictions alone make it safe.

A contract's limits remain hard controller/envelope ceilings. Unknown cost refuses paid work; opaque provider-internal activity is bounded only where proven and its blind spots remain visible. Preparation refuses requirements it cannot enforce. Prompts and profile text never substitute for enforcement.

## Minimum bindings and observation discipline

Every prepared/run record binds project/task/run IDs, intent/workspace/source references, exact canonical contract digest, source revision, immutable execution-profile version, adapter/runtime/model identities (unknown revision visibly unknown), trusted grant/policy version, budget reservation and envelope evidence. Replay uses the same bound identity, not provider-generated text.

Observations carry schema version, run reference, producer identity, sequence/cursor, event kind, observed time and provenance/coverage. Their payload is bounded, redacted under policy and stored by content reference; sensitive text is not embedded in the append-only ledger. Worker summaries and native protocol notifications are untrusted observations. Trusted capture/test services create their own receipts; a schema-valid observation is not authenticated proof.

Privacy uses current source restrictions. Secret material never enters model context. Local Only never reaches cloud serialization and requires a proven no-network path for its actual runtime, including descendants. Private cloud context requires current admission. The Ubuntu synthetic fixture's root-created namespaces qualify only those inspected tests; they do not qualify native workers or model-network routes.

## Independent evidence handoff

1. Controller validates the contract, resolves referenced state, checks current trust/revocation and reserves known costs before start.
2. Adapter reports execution stopped/completed/failed; that is not canonical DONE. Uncertain stop leaves the run unverified and its resources quarantined from reuse/application.
3. Trusted capture freezes candidate bytes and computes its own artifact manifest. Refuse unsupported paths, symlinks, drift, size overflow or protected-test changes. Partial capture remains explicitly incomplete; preserve previous source.
4. A separately controlled gate runner reads that immutable candidate and the approved criteria/check-registry versions. Required checks/test fixtures cannot be modified by the worker. Evidence binds contract digest, exact candidate hash, criterion/check ID and version, verifier identity, actual observation and pass/fail/coverage.
5. Controller authenticates receipt producers, verifies exact coverage, current admission and source freshness, then decides task terminal status through governed transactions. Hash matching establishes integrity, not correctness or authority.
6. Deliver verified candidate and human-readable evidence. Application remains not_applied until separate reviewed-hash consent, current source recheck, durable recovery preparation and an actual apply receipt.

TaskResult v1 is still a limited fixture format. It carries criterion IDs, verifier/artifact bindings and separate outcomes, but does not yet represent check versions, authenticated producer proofs, cancelled-in-flight execution, applied/recovered states, durable leases or event cursors. Those require separately versioned schemas; do not add unknown fields to v1 or fabricate existing fields.

## Terminal decisions and failures

| Observation | Controller decision |
| --- | --- |
| Execution completed, every approved criterion and mandatory gate independently passed, current authority valid | DONE; candidate may still be not_applied |
| Missing, stale, mismatched or unauthenticated evidence | UNVERIFIED; never inherit worker DONE |
| Mandatory check failed, no bounded repair remains | BLOCKED with failed evidence preserved |
| Missing approval or consequential choice beyond grant | NEEDS_OWNER_DECISION; dependent work cannot start |
| Authenticated cancellation and actual stop confirmed | CANCELLED; record any partial output and incomplete acceptance |
| Cancel requested but stop not confirmed | Nonterminal cancellation pending/unknown; no false CANCELLED or safe-cleanup claim |
| Runtime capability mismatch, revoked grant, hard budget/deadline exhausted | Refuse/stop; BLOCKED or UNVERIFIED according to observed stop/evidence; no automatic retries |
| Source drift before verification/application | Hold candidate; NEEDS_OWNER_DECISION or UNVERIFIED; never overwrite newer source |

These are design semantics, not additional TaskResult enum values. For the current fixture, in-flight cancellation/stop-unknown cannot be faithfully encoded and is unsupported. Retry creates an explicitly admitted new attempt/run within contract limits; it is not an automatic follow-on task.

## Replay, crash and handoff rules

Persist controller dispatch intent/attempt token before effects. Bind preparation/start to contract digest and snapshot; identical replay returns existing status, conflicting identity refuses. Lost start acknowledgement cannot trigger a second provider turn: query the admitted runtime or preserve UNKNOWN until reconciliation. An adapter without dependable query/idempotent dispatch cannot claim crash-safe start.

Restart reconciles durable controller ownership, current revocation/budget and actual runtime stop before resuming, rerunning checks or releasing workspaces. A provider session is continuation context, not authority. Model/profile change needs a newly versioned admitted profile/context packet; task intent/acceptance and canonical history remain Rikuri-owned. Handoff requires acknowledgements and gap/coverage evidence. No in-memory cache proves restart or distributed lease correctness.

## Compatibility with current code

| Existing source | Current behavior | Required seam |
| --- | --- | --- |
| Orchard CodexAdapter.run | Disposable source, native preflight, one turn, generic tests, summary returned | Structured prepared/started/stopped observations and trusted candidate capture; independent task-specific gate outside worker assessment |
| Orchard reviews | Separate exact-diff consent, hash checks, retained recovery | Continue separate application authority; never upgrade old DONE to canonical acceptance |
| TaskContract fixture | External fake grants, in-memory replay and paired synthetic functions | Qualified real policy/grants, authentic receipts, durable lifecycle and actual generated-artifact verification |
| Stage00 task.start/task.complete | Versioned task/run and outcome versus verification values | Future governed controller integration and receipt bindings; no direct adapter invocation of task.complete with asserted passed |

Historical Orchard DONE remains execution completed plus generic passing tests; acceptance stays UNVERIFIED unless separately evidenced. No import writes prototype SQLite rows directly into Rikuri canonical storage.

## First implementation package to propose

Implement only the provider-neutral interface types and a deterministic adapter simulator outside production callers, with typed refused/unsupported/unknown outcomes and fake durable dispatch acknowledgement. Prove exact-contract admission, snapshot integrity, independent evidence coverage, duplicate/lost-ack refusal, failed and missing gates, cancellation uncertainty, source drift and finite stopping. No paid/native turn or production state mutation.

Then separately qualify one real runtime's prepare/start/stop/collect path in a disposable environment with enforceable scope and cost/time limits. A genuine task-specific verifier must evaluate actual output; the paired mean fixture is insufficient for that claim. Run current Orchard regressions before changing its adapter. Admission of production services waits for their existing stage gates and policy prerequisites.

## Design checks and status

The success path and failure table above trace the responsibilities to one Rikuri owner and preserve execution/acceptance/application separation. Source inspection found no existing end-to-end implementation of this seam. There are no new acceptance passes or runtime promises from publishing this design. The full native IDE remains in the current optional developer-pack sequence; Foundation00–05, creative-studio direction and existing gates remain unchanged.
