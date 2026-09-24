# RIKURI PI

Personal Intelligence Platform  
Architecture Specification  
Version 1.6 \| Updated September 24, 2026

> **Simple entrance. Infinite workspace.**  
> **Models are replaceable. State is not.**  
> **Boring core. Experimental edges.**

| Field              | Value                                                                                                                                                                              |
|--------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Product            | Rikuri Pi                                                                                                                                                                          |
| Meaning            | Pi = Personal Intelligence                                                                                                                                                         |
| Document           | Architecture Specification v1.6                                                                                                                                                    |
| Companion document | Rikuri Pi Product Requirements Document v1.1                                                                                                                                       |
| Status             | Foundation-0 + staged platform architecture baseline incorporating Pi-first harnessing, native developer platform, project-execution trust, Ubuntu-first adapters, and Rikuri Link |
| Audience           | Product owner, developers, security reviewers, AI/agent engineers, future contributors                                                                                             |

## Foundation-0 reconciliation — September 24, 2026

ADR 0000 and the **Foundation-0 Re-baselined Stage Plan v2.0** control the Foundation-0 slice. This architecture preserves the larger design; the stage plan owns sequence and evidence. The six stages target June 2027 with four to eight weeks of contingency. Stage 00 proves kernel crash recovery and runs an early Ubuntu sandbox admission probe; all eight real cross-component crash points close at release. Stage 02 proves a minimal Knowledge Context Compiler with ten expected answer-or-abstain outcomes (an always-abstain implementation fails), policy refusal, and hard-budget preflight. Stage 03 proves Ubuntu envelope and project/descendant containment; the full cloud-to-local F4 gate waits for the local runtime in Stage 04. Stage 05 proves the one-window English Home and the integrated gates. Each stage has an owner-runnable demonstration backed by a linked evidence receipt.

Ubuntu is the only full Foundation-0 security/execution reference. The platform-neutral Rust core builds and runs unit/contract tests on macOS throughout; native macOS envelope and desktop parity remain experimental. One OpenAI-compatible cloud access path is tested with non-sensitive synthetic data, with other providers requiring their own capability contracts. Managed restore must consult durable current revocation state even for a backup created before forgetting; absent trustworthy revocation state it refuses/quarantines, rather than presenting a false safe restore. No unproven platform or provider capability is advertised as supported. Product P0 priorities and the later Foundation Release do not expand the Foundation-0 gate set.

# Contents

1.  Purpose and Scope

2.  Decision Status Legend

3.  Architectural North Star

4.  Non-Negotiable Architecture Invariants

5.  System Context

6.  Core Process Topology

7.  Process and Service Boundaries

8.  Event-Driven Core and Append-Only Ledger

9.  First-Run Architecture and Initial AI Selection

10. Capability Registry

11. Capability Planner

12. Execution Profile and Router

13. Model and Runtime Adapter Architecture

14. Model Lab

15. Agent Performance Model and Performance Ledger

16. Budget Governor and Autonomy Envelope

17. Workflow Engine and Unified Scheduler

18. Governance and Multi-Agent Organization

19. Context Compiler and Memory Architecture

20. Knowledge and Idea Fabric

21. Canonical Data and Storage Architecture

22. Source-of-Truth Rules

23. Search and Derived Index Architecture

24. Project Registry and Resource Resolver

25. Tool, Plugin, Connector, and Extension Architecture

26. Machine Intelligence and Privileged Execution

27. Security and Trust Architecture

28. Workspace and Window Architecture

29. Experience Surfaces

30. Interaction Profile, Personality, Voice, and Language

31. Themes and Accessibility

32. Coding Workspace Architecture

33. Domain Workspace Framework

34. Live Companion: Screen, Audio, Meeting, and Lecture Assistance

35. Visual and Artifact Rendering

36. Internationalization and Multilingual Intelligence

37. Documentation, Self-Help, and RAG Architecture

38. Personal Mesh and Multi-Device Architecture

39. Backup, Restore, and Time-Machine-Like Recovery

40. Observability, Proof of Work, and Health

41. Failure and Recovery Semantics

42. API Boundaries and Versioning

43. Schema Migration Policy

44. Update and Release Architecture

45. Repository and Module Organization

46. Technology Decision Register

47. Proven Interaction Patterns to Preserve

48. Development Compatibility Matrix

49. Implementation Stages

50. Architecture Acceptance Tests

51. Deferred Prototypes and Post-Foundation Decisions

52. Architecture Decision Record Process

53. Engineering Assurance and Project Quality Management

54. Closing Architecture Principle

Appendix A. Key Canonical Entities

Appendix B. Representative Workspace Tool Contract

Appendix C. Representative Performance Observation

Appendix D. Representative Budget Envelope Inheritance

Appendix E. Glossary

# 1. Purpose and Scope

This document translates the Rikuri Pi PRD into an implementable
architecture. It defines the system boundaries, stable contracts,
canonical state model, trust model, execution model, workspace model,
storage model, extension model, recovery model, and initial technology
choices needed to begin development without turning temporary
implementation choices into permanent constraints.

Rikuri Pi is intended to be a **user-owned Personal Intelligence
platform**, not a single assistant, chatbot, IDE, or model wrapper. Its
job is to preserve continuity and help users accomplish work across
models, applications, devices, projects, and time. The architecture must
serve a technical novice with almost no configuration while remaining
powerful enough for an expert to inspect, override, extend, and
automate.

The architecture therefore optimizes for five things simultaneously:

1.  **Usability:** the user begins with intent, not infrastructure.
2.  **Continuity:** knowledge, project state, decisions, and history
    survive model and device changes.
3.  **Replaceability:** fast-moving AI components sit behind stable
    Rikuri interfaces.
4.  **Trust:** autonomy is bounded by money, time, permissions, privacy,
    attention, and verification.
5.  **Evolvability:** the system can absorb new models, tools, runtimes,
    protocols, and UI modules without redesigning the user’s Personal
    Intelligence.
6.  **Professional outcomes:** the platform should make disciplined
    planning, verification, documentation, cleanup, and release
    readiness available to users who do not know how to manage those
    processes themselves.

This specification does not freeze every library. It intentionally
separates architectural invariants from swappable implementations.

## 1.1 Revision 1.6 focus

September 24 alignment: evolution for existing solutions; revolution for
core strength. Paperclip is a reference implementation for later agent
control-plane design, subject to a pinned version, license review,
representative execution, and failure-path evidence. Existing mechanisms
can be adopted or adapted behind Rikuri contracts; distinct investment
belongs in verified personal knowledge, adaptive harnessing, independent
security enforcement, state integrity, and intent-led team coordination.

Version 1.6 keeps the durable kernel, canonical-state, Engineering
Assurance, dynamic model/economics, and evidence-first principles of
v1.3 while incorporating the next set of concrete architectural
decisions:

1.  **Pi becomes the primary/default governed harness substrate for
    local/open models.** Rikuri builds its model-family profiles,
    security, skills, verification, evidence, and workspace behavior
    around Pi rather than maintaining a parallel full harness. Open
    native harnesses such as Qwen Code and DeepSeek Harness become
    reference implementations, benchmark competitors, and optional
    exception adapters.
2.  **Runtime and agent compatibility are separate registries.**
    llama.cpp/vLLM/Ollama/LM Studio answer whether a model can execute;
    Rikuri Harness Profiles answer how to use it reliably. The core
    harness should remain stable while family profiles/overrides change.
3.  **Projects are untrusted data until explicitly promoted for
    execution.** Read/index authority is separate from execute
    authority. Project-controlled descendants—including Git
    helpers/hooks and build/package scripts—remain inside the inherited
    capability envelope.
4.  **Ubuntu LTS is the primary Linux/reference implementation.** macOS
    remains a continuously tested Tier-1 target; Windows follows once
    the OS Capability API has survived both. Omarchy contributes
    patterns, not a base dependency.
5.  **Monaco is the embedded editor inside a Rikuri-native developer
    platform.** LSP, DAP, Git, terminal, tests, search,
    formatters/linters, Docker/dev tools, and plugin/window APIs are
    integrated directly. VS Code compatibility is selective and never
    defines the workbench.
6.  **Rikuri Link becomes the remote-control last mile.** Tailscale is
    the initial connector; a later Rikuri-native end-to-end encrypted
    direct/rendezvous/relay path becomes primary while Tailscale remains
    a fallback. Remote control is deliberately separate from Personal
    Mesh state replication.
7.  **Stage sequencing is wiring-gated.** A module is not complete
    because its unit tests pass; it must be connected to the real call
    path, exercised in the running system, fail safely, expose
    health/evidence, and pass the stage integration gate. Deferred work
    is explicitly resequenced rather than left as an unchecked orphan.

# 2. Decision Status Legend

Every significant design decision in this specification is labeled with
one of three statuses.

**LOCKED** means the decision is an architectural invariant. Reversing
it later would invalidate important assumptions or user promises and
therefore requires explicit architecture review.

**PROVISIONAL** means the choice is the preferred implementation for the
first build but is hidden behind a Rikuri-owned interface so it can be
replaced later.

**OPEN** means the requirement is known, but implementation should be
decided by prototype, benchmark, platform experiment, or later product
evidence.

Examples:

| Status      | Example                                                          |
|-------------|------------------------------------------------------------------|
| LOCKED      | Models do not own canonical Rikuri state.                        |
| LOCKED      | Vector indexes and embeddings are derived, rebuildable data.     |
| LOCKED      | All autonomous paid work operates inside a hard budget envelope. |
| PROVISIONAL | SQLite is the initial canonical personal-state database.         |
| PROVISIONAL | Tauri is the initial desktop-shell candidate.                    |
| PROVISIONAL | Monaco is the initial native Rikuri code editor component.       |
| OPEN        | Exact vector-index implementation after benchmark.               |
| OPEN        | Final Linux and Windows sandbox primitives.                      |

# 3. Architectural North Star

Rikuri should feel simpler as it becomes more capable. Complexity is
allowed underneath but must not leak into ordinary work unless it helps
the user.

The user should be able to say:

- “Help me finish this project.”
- “Save this idea.”
- “Ask three models and compare them.”
- “Run this overnight, but do not spend more than \$5.”
- “Show the code on the left, tests below it, and the browser on my
  second monitor.”
- “Take notes on this lecture.”
- “Find what we decided six months ago.”
- “Use only local models for these files.”

Rikuri should translate those intentions into planning, context
assembly, routing, execution, verification, state updates, and
appropriate presentation.

The platform’s fundamental design rule is:

> **Use the best available environment for the work while keeping
> identity, state, knowledge, permissions, budgets, and continuity under
> Rikuri control.**

# 4. Non-Negotiable Architecture Invariants

The following are **LOCKED** unless a later ADR explicitly revises the
architecture promise.

1.  **Rikuri owns continuity.** Model-provider chat state may be useful,
    but it cannot be required to resume a project.
2.  **Stable identity precedes storage location.** Projects, ideas,
    tasks, decisions, artifacts, conversations, agents, machines, and
    events receive stable IDs independent of filenames or paths.
3.  **One logical source of truth does not mean one physical database.**
    Each data type has one declared authority at a time; Rikuri knows
    which representation is canonical.
4.  **Canonical data is durable; derived data is rebuildable.**
    Embeddings, search indexes, graph projections, summaries,
    thumbnails, rankings, caches, and Markdown projections may be
    deleted and regenerated.
5.  **Policy is enforced at boundaries Rikuri can actually control.**
    Governed workers use Rikuri-owned context, tool, budget, and
    capability boundaries. Native runtimes are treated as opaque workers
    inside an execution envelope; Rikuri does not pretend to govern
    hidden internals it cannot observe or intercept.
6.  **Egress is enforced by construction, not by hoping to inspect every
    packet.** Local Only material may enter only execution
    configurations whose external-network capability is denied by a
    proven envelope. Cloud/API requests are serialized by trusted Rikuri
    adapters that reject Local Only content before request construction.
7.  **Retrieved and ingested content has zero instruction authority.**
    Web pages, imported chats, documents, email, OCR, transcripts,
    repository text, tool output, and prior messages are data. They
    cannot grant themselves tool, network, filesystem, spending, or
    approval authority.
8.  **Approval semantics come from trusted schemas.** Consequence,
    target, risk, rollback, and verification text shown to the user is
    generated from deterministic tool metadata and normalized arguments,
    never from the model’s own narration.
9.  **Autonomy is bounded.** Every autonomous job has an explicit or
    inherited envelope for authority, privacy, spending, time, retries,
    parallelism, attention, and verification.
10. **Outcome verification is distinct from tool completion.** Exit code
    0, a successful API call, or an agent statement is not proof of
    success.
11. **Sensitive content remains deletable.** Append-only audit history
    references content by stable ID rather than embedding it. Deletion
    units use cryptographic erasure where practical so “forget” can
    coexist with an append-only ledger.
12. **The workspace is programmable, but Foundation-0 does not depend on
    the spatial workspace.** Humans and agents will eventually share a
    structured Workspace API; the first prototype uses one simple
    desktop surface after the kernel works.
13. **User attention is a protected resource.** Subagents do not
    independently interrupt the user by default.
14. **The protected core cannot silently self-modify.** Constitution,
    security authority, budget enforcement, secret boundaries, identity,
    update authority, and production core code require controlled review
    and deployment.
15. **Open interfaces surround fast-moving components.** Models,
    providers, harnesses, embeddings, rendering libraries, editors,
    vector engines, and extension ecosystems are replaceable adapters.
16. **Documentation is part of the product build.** Help/RAG indexes are
    derived from versioned canonical documentation matching the
    installed version.
17. **Internationalization is architectural, not cosmetic.**
    Foundation-0 ships English only, but UI strings are externalized,
    locale-aware data is preserved, and the core never assumes
    English-only content.
18. **Benchmarks are priors; verified user work is evidence.** Early
    routing uses deterministic rules and sparse verified outcomes.
    Learned routing does not pretend confidence before sufficient
    observations exist.
19. **Completion is an evidence claim, not an agent statement.** A task,
    change, module, stage, or release is complete only when its declared
    gates and evidence are satisfied or an explicit visible exception is
    recorded.
20. **Nothing important fails silently.** Required components expose
    success/failure/degraded state and last-success evidence. A fallback
    that engages is observable and reviewable rather than quietly
    masking a broken primary path.
21. **Every defect class becomes a tripwire.** Significant defects end
    with a regression test, invariant/property check, monitor, refusal,
    ratchet, or other permanent control that makes recurrence impossible
    or loud.
22. **The repository remains intentional.** Replaced code, orphaned
    modules, stale probes, skipped tests, debug residue, and unmanaged
    generated artifacts are either justified, gated, scheduled as
    explicit debt, or removed.
23. **Novice users should not be forced to become project managers to
    obtain professional results.** Rikuri may automate planning,
    checklists, quality gates, documentation, and evidence collection,
    but consequential decisions, requirements, and tradeoffs remain
    visible to the user.
24. **Model names, access paths, prices, and quota rules are dynamic
    registry data, not core-code constants.** New model generations may
    appear at any time. Rikuri must be able to register, discover,
    deprecate, price, meter, evaluate, and route a newly available model
    without adding model-name branches or schema enums to the Kernel.

<!-- -->

24. **Privacy intent is user-facing; enforcement is technical.** Auto,
    Private, and Local Only/Vault are stable user concepts translated
    into destination, sanitization, provider, network, and
    materialization policy.
25. **Read authority never implies execute authority.** A
    project/repository remains untrusted for execution until an
    explicit, scoped, reversible trust decision is recorded; descendants
    inherit the same execution envelope.
26. **Pi is a replaceable substrate, not a canonical authority.** Rikuri
    owns project state, security, profiles, evidence, and user
    experience even though Pi is the primary local/open harness.
27. **Editor/workbench choice does not define Rikuri.** Monaco is a work
    surface inside Rikuri; VS Code/Cursor/native tools may coexist but
    do not own canonical project state.
28. **Remote access does not require cloud-hosted agents.** Rikuri
    should reach owner-controlled nodes securely from thin clients;
    cloud models/services are optional resources rather than mandatory
    homes for persistent agents and credentials.

# 5. System Context

Rikuri Pi system context

Rikuri is divided into six conceptual layers:

- **Experience Layer:** Quick Assist, Home, voice, workspace,
  dashboards, and domain workspaces.
- **Control Plane:** intent interpretation, context compilation,
  capability planning, workflow, routing, governance, evaluation, and
  budgets.
- **Model/Runtime Fabric:** frontier models, local models, provider
  brokers, native vendor runtimes, and Rikuri’s generic harness.
- **Tool/Capability Fabric:** files, browser, email, calendar, code,
  terminal, canvas, connectors, APIs, MCP-compatible tools, and GUI
  control.
- **Knowledge/State Fabric:** projects, ideas, decisions, conversations,
  events, sources, entities, relationships, and machine knowledge.
- **Security/Autonomy Boundary:** capabilities, privacy, secrets,
  privilege, spending, attention, and audit before effects reach
  machines, accounts, networks, or financial systems.

The user experiences one Rikuri. The internal system may employ many
models, workers, processes, machines, and storage engines.

# 6. Core Process Topology

## 6.1 The stable kernel

**LOCKED:** Rikuri has a small trusted core whose semantics remain
stable as AI technology changes.

Kernel responsibilities:

- Personal Intelligence identity and device identity
- stable object IDs and schema/version registry
- canonical state repositories and migrations
- append-only event ledger and idempotency records
- task/checkpoint state needed for crash recovery
- capability, privacy, risk, and instruction-authority types
- deterministic policy decisions and trusted approval rendering
- Budget Governor reservations and accounting
- context-package admission policy
- deletion/tombstone/key lifecycle metadata
- backup/restore metadata and integrity manifests
- security health and audit state

The Kernel does not contain model-specific prompting logic and does not
depend on any frontier provider.

## 6.2 Foundation-0: modular monolith

Foundation-0 deliberately begins as one trusted Rust daemon plus thin
clients rather than a fleet of services.

Foundation-0 process and trust topology

`rikurid` is the only Foundation-0 process allowed to mutate canonical
state, append authoritative security/audit events, authorize
consequential tools, allocate paid budget, unwrap Rikuri-managed content
keys, or construct cloud-bound model requests.

## 6.3 Fast-moving userland

Userland contains replaceable components such as model/provider
adapters, local inference runtimes, vector implementations, future
browser workers, domain skills, UI modules, coding components, voice
engines, and third-party extensions.

**LOCKED principle:** userland can change rapidly without changing
canonical Rikuri identity, provenance, deletion semantics, or security
promises.

# 7. Process and Service Boundaries

Foundation-0 prefers **modular code boundaries before process
boundaries**. A premature service topology would add failure modes
without improving the guarantees being prototyped.

### 7.1 Foundation-0 process topology

| Process           | Foundation-0 responsibility                                                                                         | Trust level      |
|-------------------|---------------------------------------------------------------------------------------------------------------------|------------------|
| `rikurid`         | canonical state, ledger, policy, context admission, import coordination, search coordination, budget, backup/delete | trusted          |
| `rikuri`          | thin CLI over local IPC                                                                                             | untrusted client |
| `rikuri-desktop`  | later single-window shell over the same IPC; no business logic                                                      | untrusted client |
| governed worker   | local model/tool process launched with an explicit execution envelope                                               | untrusted worker |
| external provider | receives only the request intentionally serialized by a trusted adapter                                             | external party   |

A crash in a client or worker must not corrupt canonical state. The
daemon serializes canonical mutations through explicit transactions.

### 7.2 Future extraction

Later releases may split indexer, browser worker, extension host, model
lab, sync, and orchestration into separate processes. Those extractions
must preserve the same versioned contracts; they are deployment choices,
not new data authorities.

### 7.3 Foundation trust rule

A worker never becomes trusted merely because Rikuri launched it.
Models, imported parsers, distillers, local inference servers, native
runtimes, extension hosts, browsers, and GUI workers are treated as
potentially fallible or adversarial relative to canonical state and
protected data.

### 7.4 Headless runtime and attachable clients

LOCKED architectural intent: Rikuri's trusted runtime, orchestration,
policy, task state, event ledger, model routing, and tool execution
remain independently usable without a desktop UI. Desktop, terminal,
browser, mobile, voice, and remote clients attach through versioned
contracts; no client owns canonical task state or the agent loop.

A rich operating environment may coordinate editors, browsers, computer
use, approvals, and specialist agents, but it must not fork or hide the
underlying runtime semantics. A task started from one client can be
observed, approved, resumed, or handed off from another according to
capability and identity policy.

# 8. Event-Driven Core and Append-Only Ledger

Every meaningful state transition emits a structured event. The ledger
answers **what happened**; explicit state tables answer **what is true
now**. Rikuri is not a pure event-sourced system.

A command that changes canonical state follows one transaction boundary:

    command
      -> validate current version + policy
      -> update canonical current state
      -> append event + idempotency record
      -> commit

Foundation event requirements:

    event_id: evt_...
    schema_version: 1
    timestamp: ...
    project_id: ...
    task_id: ...
    actor_id: ...
    event_type: ...
    subject_ids: [...]
    input_refs: [...]
    result_refs: [...]
    status: ...
    security_decision_id: ...
    cost_record_id: ...
    idempotency_key: ... # required for consequential operations
    metadata: {...}

**LOCKED:** events reference content by ID/version and never embed large
or sensitive source content. This permits an append-only audit trail
while allowing cryptographic forgetting of the underlying content.

**LOCKED:** crash recovery must reconstruct committed task/checkpoint
state from canonical versioned state plus the ledger without relying on
private model-session memory.

Foundation-0 includes a kill-and-restart gate: terminate the trusted
process during a dummy task, restart, and prove that already committed
consequential actions are not repeated.

# 9. First-Run Architecture and Initial AI Selection

The long-term first-run default remains **Auto**, meaning a Rikuri
policy rather than a privileged vendor. Foundation-0, however, avoids
building the hardware recommender and provider marketplace before the
core works.

## 9.1 Foundation-0 onboarding

A fresh installation opens Home immediately. No configuration wizard
blocks first launch. When the user first requests AI work, Rikuri asks
only for the missing execution capability:

    Choose how to run this task

    [ Connect an API / broker endpoint ]
    [ Use a local OpenAI-compatible runtime ]
    [ Not now ]

The exact credential is stored by the OS/key service or future secret
broker, not inside canonical prompt/history records.

## 9.2 Initial supported automation paths

Foundation-0 supports only paths Rikuri can reason about cleanly:

- `direct_api`
- `broker_api`
- `local_runtime`

A subscription-backed vendor-native runtime is **not** a foundational
automation path. It is optional later when the vendor explicitly
supports the needed integration and terms. Interactive consumer
subscriptions may remain separate user environments whose artifacts can
later be imported.

## 9.3 Local hardware

Foundation-0 may record basic machine facts needed to launch a
configured local runtime, but model discovery and a full hardware-fit
recommender are deferred. Rikuri must not promise that a particular
local model fits solely from nominal RAM.

## 9.4 Bundled models

**LOCKED:** the installer does not bundle a multi-gigabyte
general-purpose assistant model. Small utility models may be added later
behind capability and licensing checks.

## 9.5 Internationalization at first install

Foundation-0 ships English user-facing strings only, but all shell
strings are externalized and a pseudo-locale is exercised in CI. Source
content remains Unicode and language-tag capable from day one.

# 10. Capability Registry

Rikuri routes capabilities rather than hard-coded vendors.

Every model, runtime, tool, machine, extension, and worker publishes a
versioned capability descriptor.

Example model/runtime descriptor:

    id: execution-capability-...
    model: ...
    runtime: ...
    modalities:
      text: true
      vision: true
      audio_in: false
      audio_out: false
    agent_features:
      tools: true
      structured_output: true
      filesystem: true
      browser: false
      code_edit: true
    limits:
      context_tokens: ...
      max_output_tokens: ...
    privacy:
      locality: cloud
      retention_class: ...
    languages:
      en: measured
      es: measured
      zh-Hans: measured
      zh-Hant-TW: measured
    hardware:
      machine_id: null

Machine capability descriptors expose hardware, installed runtimes,
services, and available resources. Tool descriptors expose actions,
input/output schemas, risk class, permissions, and verification hooks.

**LOCKED:** adding a new model or tool should primarily be a matter of
registering an adapter plus capabilities, then evaluating it. Core
Rikuri logic must not require model-name-specific branching except where
a provider’s behavior genuinely requires it.

## 10.1 Dynamic Model Registry

**LOCKED:** model names are data. The Kernel, router, UI, tests, and
persistence schema must not contain an enum or branch that assumes
today’s model lineup is complete. Provider model identifiers are treated
as opaque external keys and mapped to stable Rikuri records.

A representative record is:

    model_record_id: mdl_...
    provider_id: provider_...
    provider_model_key: <opaque provider-supplied identifier>
    display_name: <provider/user-facing label>
    family_label: <optional mutable metadata>
    revision: <provider revision when exposed>
    lifecycle: available | deprecated | retired | unavailable
    capability_descriptor_id: cap_...
    discovered_by: manual | provider_api | registry_adapter | local_runtime
    first_seen_at: ...
    last_verified_at: ...
    metadata: {...}

`display_name` and `family_label` are presentation metadata, not
identity. Renaming a model, adding a new model family, or retiring an
old one does not require a canonical schema migration. Tests use
synthetic model identifiers rather than current commercial model names
except in adapter-specific fixtures.

A new model should normally enter Rikuri through:

    discover/register -> capability description -> access profiles -> economics metadata -> evaluation -> routing eligibility

No model receives architectural privilege merely because it is new,
popular, or bundled with a subscription.

## 10.2 Dynamic Access, Price, and Quota Registries

Model identity is separate from how the model is reached and how that
access is charged. A single model may have multiple access profiles, for
example a direct API, broker API, local runtime, or officially supported
native/subscription harness.

    access_profile_id: access_...
    model_record_id: mdl_...
    runtime_or_harness_id: ...
    access_path: direct_api | broker_api | local_runtime | vendor_native | subscription_native
    auth_method: ...
    provider_account_id: ...
    price_card_id: price_...        # optional
    quota_pool_ids: [quota_...]      # optional; shared or model-specific
    telemetry_capabilities:
      tokens: exact | estimated | unavailable
      cash_cost: exact | estimated | unavailable
      quota: provider_reported | inferred | unavailable

Price cards are versioned and effective-dated. They are not hard-coded
into routing logic. Pricing dimensions are extensible because providers
may charge differently for input, output, cache read/write, long
context, priority/fast service, tools, batch processing, regions, or
future units. Historical execution records reference the price/usage
evidence that applied when the work occurred rather than being
recomputed from today’s price.

Subscription quota pools are also data-driven. A provider may expose one
shared pool across several models, separate pools per model, rolling
windows, credits, or only an aggregate percentage. Rikuri stores only
what is actually observable and never invents token counts from a
percentage bar.

Economic observations carry provenance/confidence such as `exact`,
`provider_reported`, `estimated`, or `unknown`. Fixed monthly
subscription fees may later be amortized over observed verified work for
analysis, but that derived metric is not represented as the provider’s
API token price.

## 10.3 Specialist capability roles

Rikuri represents model roles independently rather than requiring one
selected model to support every modality. A task may compose a lead
reasoning/planning model with coding, vision, voice/speech, retrieval,
embedding, translation, or verification specialists. This resembles
mixture-of-experts at the platform level, but each specialist remains an
explicit execution participant with its own identity, permissions,
context package, cost, latency, and evidence.

Capability metadata is first-class and machine-readable: accepted input
and produced output modalities, tool use, structured output, image and
audio understanding/generation, streaming, local hardware/runtime
compatibility, privacy eligibility, and measured task performance.
Discovery that loses modality metadata must mark the profile incomplete
or unavailable rather than silently treating a vision-capable model as
text-only.

# 11. Capability Planner

The Capability Planner converts user intent into a task requirements
profile before non-trivial delegated work.

Inputs include:

- objective and expected deliverable
- domain and task class
- modalities
- context size and source locations
- privacy class
- consequence/risk class
- latency preference
- user budget
- language(s)
- required tools
- machine/project location
- available provider quota/subscription
- current resource pressure
- verification requirements

Output is a `TaskRequirements` record rather than a direct model choice.

This separation allows routing logic to improve without changing the
user-facing task model.

# 12. Execution Profile and Router

The long-term routing unit remains an **Execution Profile**:

    execution_profile_id: xp_...
    model_record_id: mdl_...
    model_revision: ...
    runtime_family: governed | opaque_native
    provider_id: ...
    access_profile_id: access_...
    access_path: direct_api | broker_api | local_runtime | vendor_native | subscription_native
    price_card_id: price_...        # optional snapshot/reference
    quota_pool_ids: [quota_...]      # optional
    machine_id: ...
    tools: [...]
    context_policy: ...
    security_envelope_id: ...
    budget_envelope_id: ...
    verification_profile_id: ...
    settings_profile_id: ...

The profile describes the system that produces the result, not merely
the model weights.

### 12.1 Foundation-0 router

There is no learned router in Foundation-0. Selection is deterministic
and auditable:

1.  reject profiles incompatible with the privacy/context admission
    policy;
2.  reject profiles whose execution envelope cannot provide the
    requested guarantee;
3.  reject profiles that cannot reserve the required budget;
4.  among remaining configured profiles, use an explicit user choice or
    a simple declared preference;
5.  escalate after verification failure only when policy and budget
    permit it.

Sparse observations are recorded but do not justify a statistical
ranking claim.

### 12.2 Future Auto

Later, Auto may select profiles using measured verified success,
quality, cost, latency, language competence, retries, human
intervention, resource use, and user preference. Public benchmarks are
priors; Rikuri’s own verified observations eventually dominate for the
user’s workload.

### 12.3 Composite execution plans

The router may produce a composite execution plan instead of a single
model choice. The plan names the lead role and optional specialists,
defines which context and tools each receives, and assigns budget,
privacy, verification, fallback, and handoff rules per role. Specialists
do not automatically see the full conversation, filesystem, credentials,
or each other's private context.

Initial composition is deterministic and policy-driven. Learned
orchestration remains later and must be evaluated against simpler
single-model execution for quality, latency, cost, privacy, and failure
recovery.

# 13. Model and Runtime Adapter Architecture

Rikuri separates model execution from agent behavior and treats each as
a replaceable compatibility layer.

## 13.1 Pi-backed governed workers

For local/open models and provider-neutral governed execution, **Pi is
the primary/default agent harness substrate**. Rikuri does not fork Pi’s
core unless an upstream limitation is proven and an ADR justifies the
maintenance cost. Rikuri owns the surrounding contracts:

    Rikuri task/context/security/budget
            -> Pi adapter/session
            -> model through selected runtime/provider
            -> Rikuri tool broker / OS capability / project-trust boundary
            -> verification/evidence/ledger

Pi may provide the agent loop, tool dispatch, skills/extensions, and
session mechanics. It does not own canonical Rikuri state, privacy
authority, project trust, budget authority, or completion status.

## 13.2 Harness Profiles

A **Rikuri Harness Profile** describes how a model family or checkpoint
works best with Pi without hard-coding that behavior into the kernel.
Profile dimensions may include:

- preferred system/tool guidance and tool-schema quirks;
- effective context/compaction strategy;
- planning/autonomy level;
- editing/diff strategy;
- compile/test/debug cadence;
- number/complexity of tools exposed;
- subagent availability/parallelism;
- structured-output/reasoning behavior;
- known failure modes and required verification;
- recommended task/runtime/hardware overlays.

Profiles compose: base task profile + model family + capability/size
override + runtime + hardware + privacy/project-trust + project-language
profile. A new related model first reuses a family profile; measured
failure creates a small versioned override.

## 13.3 Runtime compatibility versus agent compatibility

The compatibility registry records two independent states:

    Runtime: can llama.cpp/vLLM/Ollama/LM Studio execute this model correctly?
    Agent:   can Pi/Rikuri use it reliably for tools/context/planning/editing/verification?

User-facing status may collapse these to `Optimized`, `Compatible`,
`Experimental`, or `Unavailable`, while expert detail exposes the
separate evidence.

## 13.4 Native/open harness exception adapters

Qwen Code, DeepSeek Harness, Hermes, and future harnesses are normally
**reference code and benchmark comparators**, not the default runtime.
Because their source is open, Rikuri development tooling may inspect
prompt variants, tool descriptions, context/compaction logic, planning,
subagents, and model-specific branches; useful techniques are evaluated
and incorporated into Rikuri profiles/extensions where appropriate.

If a native harness materially outperforms Pi/Rikuri for a model/task
and can satisfy policy/observability requirements, Rikuri may route to
it through an adapter. That is an evidence-backed exception, not a
model-name rule.

## 13.5 Opaque frontier/native workers

Vendor-native frontier runtimes may own hidden context/tool loops.
Rikuri treats them as opaque workers inside an execution envelope and
governs only what it can actually control: task input, mounted
resources, credentials, network/process scope where enforceable,
artifacts/diffs, cost/quota evidence when exposed, and independent
verification.

## 13.6 Access paths and skills

Direct API, broker API, local runtime, officially supported
native/subscription paths, and remote-owner-node execution remain
access-path data. Skills/domain procedures remain model-independent and
never expand their own security authority.

# 14. Model Lab

The full Model Lab remains part of the target architecture but is
**DEFERRED from Foundation-0**.

Foundation-0 records enough execution identity to make later measurement
possible:

- model/provider identifier and revision when exposed
- access path
- runtime/harness version
- key settings
- machine identifier for local work
- task class
- verification result
- tokens/cost/latency when available

No automatic model acquisition, hardware recommender, parameter tuning,
probation, or background benchmarking is required during Foundation-0.

When Model Lab is added, every evaluated artifact remains identified by
exact revision/quantization/runtime/settings/hardware rather than model
name alone.

Model Lab consumes the Dynamic Model Registry rather than defining a
fixed set of models. Discovery adapters may add newly available cloud or
local models at runtime; capability evaluation and economics metadata
determine whether they become routing candidates. A model appearing or
disappearing from a provider catalog should be an ordinary registry
lifecycle event, not a Kernel release.

# 15. Agent Performance Model and Performance Ledger

The Performance Ledger starts as an **evidence log**, not a prediction
engine.

Every meaningful execution records, when available:

    observation_id: perf_...
    task_class: ...
    execution_profile_id: ...
    verification: pass | fail | unknown
    user_feedback: up | down | none
    model_record_id: mdl_...
    access_profile_id: access_...
    price_card_id: price_... | null
    actual_cost_microusd: ... | null
    cost_confidence: exact | provider_reported | estimated | unknown
    input_tokens: ... | null
    output_tokens: ... | null
    cached_tokens: ... | null
    quota_observation_before: ... | null
    quota_observation_after: ... | null
    quota_confidence: provider_reported | inferred | unknown
    model_calls: ...
    wall_time_ms: ...
    retries: ...
    human_interventions: ...

Foundation-0 ground truth is deliberately narrow:

- deterministic verification passed/failed; and
- explicit user confirmation/feedback when offered.

A profile with four observations must not be presented as “94%
reliable.” The UI and router must expose insufficient-data states.

Later, once evidence is adequate, Rikuri may estimate:

- probability of verified success
- cost per verified success
- time per verified success
- tokens per verified success
- quality per dollar/minute where quality has defensible labels
- best escalation policy for a task class

**LOCKED:** token price alone is never the optimization target.

# 16. Budget Governor and Autonomy Envelope

The Budget Governor is trusted deterministic code below the model. A
prompt such as “do not spend more than \$5” is not the enforcement
mechanism.

## 16.1 Budget envelope

A task may define money, time, model-call, retry, and future parallelism
ceilings. Child work inherits slices; spawning does not create new
budget.

## 16.2 Hard-dollar reservation

A hard money limit requires an atomic reservation before a paid provider
call:

Budget reservation flow

Currency accounting uses integer micro-units, never floating-point
money.

If a provider/access path cannot bound a request sufficiently to
preserve a hard-dollar promise, that profile cannot advertise a hard
dollar ceiling.

## 16.3 Thresholds and stop semantics

- soft warning: continue and record forecast;
- approval threshold: checkpoint and ask before expanding the envelope;
- hard threshold: block new paid calls deterministically.

Foundation-0 proves a \$1 hard-stop demo and records the reason in the
ledger. Predictive cost-to-completion and sophisticated loop detection
are later improvements.

## 16.4 Model Economics Catalog and Subscription Quota Registry

The Budget Governor does not know vendor model names or pricing
formulas. It asks a versioned economics interface for the currently
applicable charging/quota semantics of an `access_profile_id`.

For metered APIs, hard-dollar enforcement uses price-card-backed
maximum-cost reservation when a safe bound is available. For fixed-price
subscriptions, Rikuri separately records:

- fixed subscription fee and billing period when the user chooses to
  enter it;
- incremental cash cost of included work (normally zero unless
  credits/overage apply);
- provider-reported or inferred quota consumption and reset windows when
  observable;
- model/access attribution when the provider exposes it;
- wall time, verified outcome, and user feedback;
- amortized cost per observed token or verified task only as a
  retrospective analytic metric.

Quota scarcity may have a routing penalty even when marginal cash cost
is zero. If only an aggregate usage percentage is available, Rikuri
records that percentage and does not fabricate per-model token usage. If
a provider later exposes separate quota pools for different models, the
same schema represents them as separate or shared `quota_pool_id`
records without code changes.

**LOCKED:** adding a new model, changing a display name, changing a
price, or changing which models share a subscription quota pool must be
representable as registry/catalog data. It must not require
model-specific branches in the Budget Governor.

# 17. Workflow Engine and Unified Scheduler

The target Workflow Engine supports durable DAGs, dependencies, parallel
branches, retries, gates, schedules, checkpoints, cancellation, and
recovery. Foundation-0 implements only the minimum durable
task/checkpoint state needed to prove crash survival and handoff.

Foundation-0 states:

    queued -> running -> waiting_tool -> waiting_approval -> verifying -> completed
                                      \-> failed
                                      \-> cancelled

Every consequential action carries an idempotency key so restart/replay
cannot silently duplicate an already committed effect.

**DEFERRED:** multi-project scheduling, cron/event automation, parallel
agent branches, strategy councils, and Temporal-class infrastructure.

# 18. Governance and Multi-Agent Organization

The governance model remains valid: the user is the Principal and Rikuri
eventually manages replaceable coordinators, specialists, and
evaluators. Foundation-0 intentionally contains **no multi-agent
orchestration**.

The first build proves the deeper prerequisites first:

- explicit task contract
- one worker at a time
- bounded capability and budget envelope
- durable state and checkpoint
- verification
- handoff between execution profiles

Coordinator/COO routing, Strategy Council, evaluator roles, charters,
scorecards, and swarms are deferred until the single-worker substrate is
trustworthy.

## **18.1 Control plane reference and adoption boundary**

Paperclip documents event-driven heartbeats, atomic single-assignee task
checkout, adapter invocation, run/session records, budgets, approvals,
and team templates. Treat these as candidates rather than proof of
production reliability. Pin a release, inspect source and license, and
test contention, lease expiry, duplicate trigger, partial run, restart,
approval denial, budget stop, cancellation, and rollback. For each
mechanism compare reuse, adaptation, a mature generic component, and
custom implementation in an ADR.

A post-Foundation worker invocation carries project, task, worker,
trigger, run, and lease IDs; an authority and budget envelope; and an
execution profile. The control plane verifies authoritative ProjectState
and policy, atomically acquires work, compiles current policy-filtered
knowledge, invokes a replaceable adapter, acknowledges tool outcomes,
verifies the deliverable, and commits candidate state transactionally.
Missing acknowledgements and failed runs are visible and retryable.
Expired leases never authorize duplicate external effects; idempotency
and committed-outcome checks govern replay.

Rikuri organizes a project-scoped graph of goals, tasks, roles, review,
and authority. One worker is valid. A company org chart is an optional
profile. The user describes needed help; the proposed team shows role,
scope, cost, access, duration, verification, and approvals while hiding
adapter settings unless requested. The common Project Lifecycle Kernel
is Frame, Shape, Plan, Execute, Verify, Deliver, Learn with one Project
ID and authoritative ProjectState. Domain profiles specialize artifacts
and gates. Markdown is a portable projection, not the state database.

References:
https://github.com/paperclipai/paperclip/blob/master/docs/start/architecture.md
; https://docs.paperclip.ing/guides/projects-workflow/routines/ ;
https://docs.paperclip.ing/guides/org/team-catalog/ ;
https://docs.paperclip.ing/guides/day-to-day/costs/ .

# 19. Context Compiler and Memory Architecture

Knowledge Context Compiler invariant: memory, summaries, embeddings,
prior agent output, retrieved documents, and inferred links are evidence
candidates. At use time, compile policy-eligible context against current
authoritative sources, valid/observed time, supersession, provenance,
conflicts, relevance, and confidence. Verification frequency follows
volatility and consequence; unresolved conflicts remain visible and the
compiler may abstain.

The Context Compiler constructs the smallest useful context package that
is compatible with the destination execution profile.

Foundation inputs may include project state, task objective, relevant
decisions, retrieved source spans, recent events, and available tools.

### 19.1 Provenance and instruction authority are separate

Every context segment carries at least:

    source_id: ...
    source_span_id: ...
    origin: user_live | user_authored | imported | web | tool_output | model_output
    instruction_authority: current_user | trusted_policy | none
    privacy_class: normal | private | local_only | secret
    content_ref: ...

**LOCKED:** imported/retrieved/historical content has
`instruction_authority: none` even if its prose contains imperatives,
system-like delimiters, or text previously written by the user. Only the
current authenticated user command and trusted Rikuri policy create
normal instruction authority.

### 19.2 Privacy admission before assembly

The compiler applies privacy and destination policy before materializing
the final package. Local Only source spans cannot be assembled into a
package targeting an external provider or a worker whose envelope allows
external networking.

### 19.3 Secret class

Secret material is excluded from model context entirely. Future trusted
brokers may use a credential without revealing it to a model.

### 19.4 Untrusted-content framing

Model prompts clearly delimit retrieved material as data.
Prompt-injection detectors may annotate suspicious segments, but
security does not depend on perfect detection; deterministic
tool/capability policy remains authoritative.

# 20. Knowledge and Idea Fabric

The Knowledge and Idea Fabric is the first serious dogfood application
of the canonical data model.

## 20.1 Import first

Foundation-0 imports:

- ChatGPT export
- Claude export
- user Markdown/Obsidian vault
- Git repository manifest/content index

Every importer is designed as a pure transformation: source
bytes/snapshot in -\> canonical records plus provenance out. Importers
are idempotent and record parser version, source-system identity,
source/external IDs when available, content hash, and import timestamp.

Original source material is preserved untouched or referenced at its
authoritative external location according to source policy.

## 20.2 Capture first, structure later

Text and local voice capture become first-class Idea sources. Original
audio is authoritative source material; transcript and classification
are derived.

Idea remains distinct from Decision. Suggested lifecycle states remain
Captured, Exploring, Promising, Attached, Adopted, Implemented,
Rejected, and Superseded.

## 20.3 Two one-way Markdown pipes

Bidirectional Obsidian synchronization is removed from the architecture.

Two one-way Markdown flows

**Pipe A - Rikuri to Markdown:** canonical Rikuri knowledge -\>
generated Markdown projection. It is regenerable and never merged back.
Edits in the generated tree are detected and flagged.

**Pipe B - user notes to Rikuri:** a user-owned Markdown/Obsidian vault
-\> immutable/versioned imported source snapshots with provenance. The
external vault remains authoritative for those notes.

There is no three-way merge between canonical Rikuri knowledge and a
projection.

## 20.4 Distillation

Distillation output is derived Knowledge data with source-span
references. A distiller receives no side-effect tools by default; it
returns structured candidate output that trusted Rikuri validates before
storage.

## 20.5 Knowledge as a Service

LOCKED architectural intent: the agent harness does not own retrieval,
memory, or RAG. A Rikuri Knowledge Service owns the stable query
contract over canonical records, source files, SQL projections, derived
indexes, and the Obsidian-compatible projection. Agents and harnesses
receive narrowly scoped knowledge tools rather than direct storage
authority.

The initial tool surface should favor read-only operations: enumerate
authorized knowledge spaces, search, reconstruct or read a cited source,
and ask a bounded question with citations. Write, distill, link, delete,
export, or share operations require separate capabilities and policy
decisions. Derived vector stores remain disposable and rebuildable.

This boundary allows local and frontier models, Pi-backed workers,
native harnesses, and future multi-agent systems to use the same
knowledge service without each embedding a separate RAG stack.

# 21. Canonical Data and Storage Architecture

Foundation-0 uses a local-first canonical store:

    RikuriData/
    ├── state/
    │   └── rikuri.db
    ├── content/
    │   └── encrypted-blobs/
    ├── sources/
    │   ├── imports/
    │   └── manifests/
    ├── knowledge/
    │   └── projection/       # generated; rebuildable
    ├── indexes/              # rebuildable
    ├── docs/                 # version-matched canonical help docs
    └── backups/

SQLite stores canonical structured state and metadata. Large/sensitive
content is stored as encrypted content blobs or authoritative external
resources referenced by stable IDs.

### 21.1 Core object registry

A common registry carries identity and lifecycle fields; typed tables
hold domain-specific properties.

    objects
      id
      kind
      created_at
      updated_at
      privacy_class
      status
      deletion_unit_id
      ext_version
      ext_json

Typed tables include projects, ideas, decisions, conversations,
messages, sources, source spans, tasks, artifacts, relations, execution
runs, cost records, security decisions, content blobs, content keys,
object versions, and tombstones.

### 21.2 Strong core plus extension field

Stable/high-value fields are typed. A versioned JSON extension field is
a pressure-release valve for evolving metadata, not an excuse to avoid
schema design.

### 21.3 Per-object content keys and forgetting

Sensitive deletion units use distinct data-encryption keys (DEKs) under
the device/recovery key hierarchy. `forget(object)` destroys the active
DEK and writes a tombstone, removes all derived indexes/projections,
queues ciphertext for garbage collection, and prevents future managed
backups from retaining a usable key.

The append-only ledger may retain the fact that an object was forgotten
but not its deleted content.

Rikuri can guarantee forgetting only for stores/backups it manages;
independently exported copies remain outside its control.

# 22. Source-of-Truth Rules

Each object family declares exactly one authoritative representation at
a time.

Foundation-0 source rules:

| Data                                  | Authority                                                     |
|---------------------------------------|---------------------------------------------------------------|
| Rikuri projects/ideas/decisions/tasks | SQLite canonical state                                        |
| imported conversation content         | preserved original snapshot + canonical message/span identity |
| user Obsidian/Markdown notes          | external vault; Rikuri snapshot is imported evidence          |
| Rikuri Markdown knowledge projection  | derived output; never edited back                             |
| source code                           | live Git repository/file at current resolved revision         |
| embeddings/vector indexes             | derived and disposable                                        |
| search ranking                        | derived                                                       |
| event/audit history                   | append-only ledger referencing content IDs                    |
| deleted content                       | tombstone + destroyed key; content is no longer an authority  |

A rename, move, re-index, or change of model must not create a new
logical identity.

# 23. Search and Derived Index Architecture

Search is a core product problem, not infrastructure garnish.
Foundation-0 optimizes retrieval quality against a fixed real-history
evaluation set.

### 23.1 Baseline architecture

    query
      -> lexical FTS5 candidates
      -> optional VectorIndex candidates
      -> simple fusion + recency policy
      -> result with provenance
      -> open original

**PROVISIONAL:** SQLite FTS5 is required in Foundation-0.

**LOCKED interface:** semantic retrieval sits behind a `VectorIndex`
contract.

**EXPERIMENTAL:** sqlite-vec or another local implementation may be used
only if it passes the retrieval eval and remains disposable. Deleting
all vector/index data must not destroy knowledge.

### 23.2 Retrieval evaluation

The initial evaluation corpus uses at least twenty known-answer
questions drawn from the owner’s imported ChatGPT/Claude/notes/project
history. Each case records expected source/object IDs and optional
answer facts.

Every chunking/embedding/fusion change runs the same suite. New real
failures become new regression cases.

### 23.3 No smart-chunking rabbit hole

Foundation-0 begins with straightforward, provenance-preserving
chunks/spans. Rerankers, multilingual embedding optimization, graph
search, and sophisticated semantic segmentation are deferred until the
baseline is measurable.

# 24. Project Registry and Resource Resolver

The Project Registry is the catalog of what exists and where.

A project may have:

- canonical Rikuri project identity
- one or more local repository paths
- Git remote(s)
- documents on another device
- NAS/cloud assets
- related knowledge
- active tasks/workflows
- workspace layouts
- machines capable of executing it

The Resource Resolver maps semantic project/resource identity to a
current usable target.

Example:

    User: “Continue the KV-cache experiment.”
    -> project identity found
    -> authoritative repo currently on Ryzen
    -> current branch/HEAD queried
    -> coding worker routed to Ryzen or repo cloned with permission

# 25. Tool, Plugin, Connector, and Extension Architecture

Rikuri prefers **capabilities over marketplaces**. The product should
expose useful tools consistently without recreating the VS Code
extension ecosystem or allowing arbitrary plugins to inherit the user’s
full account authority.

Capability order remains:

    Rikuri structured capability / trusted native API
      -> standard protocol/service (LSP, DAP, Git, rclone-class tool, etc.)
      -> MCP/connector/plugin with declared capabilities
      -> browser/GUI worker
      -> raw shell as bounded fallback

### 25.1 Rikuri Plugin API

A plugin may contribute one or more of:

- Pi/Rikuri skill or agent extension;
- tool/service adapter;
- editor capability;
- model/runtime/provider adapter;
- importer/search/index adapter;
- a new Sterling-style Window type;
- menus/actions/settings/schema metadata;
- domain-pack behavior.

Plugins receive explicit capabilities through AISecurity/Rikuri
Security. They do not automatically inherit unrestricted filesystem,
network, secrets, canonical DB writes, shell, or other plugins’
authority.

### 25.2 Lifecycle

The target lifecycle is:

    discover -> inspect provenance/license/dependencies -> pin exact version/commit
             -> capability review -> install/quarantine -> activate
             -> health/verification -> update with rollback -> remove cleanly

A failed verification or security regression rolls back or disables the
plugin. Extension updates are distinct from protected-core updates.

### 25.3 VS Code/Monaco compatibility

Monaco does not imply universal VS Code-extension compatibility. Rikuri
may use selective compatibility components such as `monaco-vscode-api`
where they save work, but the preferred order is: integrate the
underlying open tool directly; otherwise build a native Rikuri
plugin/window; emulate a VS Code extension only when the value outweighs
API-compatibility/security maintenance.

MCP and external connector ecosystems remain supported integration
surfaces, not privileged trust zones.

### 25.4 Machine stable capability contracts

LOCKED architectural intent: every agent-facing command, tool, and
privileged capability exposes a versioned machine-stable contract. The
canonical response is a structured JSON envelope with typed
success/error states, stable error codes, correlation and provenance
identifiers, and defined process exit semantics where a CLI exists.
Human-readable rendering is a presentation layered above the contract.

Plugin installation never creates authority. A plugin or MCP server can
advertise a capability, but AISecurity/Rikuri Security grants the
runtime capability token. Filesystem, network, credentials, shell,
screen control, remote access, external applications, financial systems,
and physical devices are privileged capability providers and must remain
behind policy mediation.

### 25.5 Connector Gateway

Rikuri normalizes native APIs, MCP servers, hosted integration fabrics
such as Zapier-class services, and direct service adapters behind a
Connector Gateway. Discovery and authentication may be delegated, but
Rikuri exposes only the task-scoped subset approved by policy and
records the external action and result. Hosted breadth is not equivalent
to trusted breadth.

### 25.6 Progressive disclosure for skills and visual grammars

Skills should begin with a small routing manifest and load only the
instructions, grammar, examples, or validators required for the selected
task. Diagram generation should separate the semantic system
representation from its renderer so the same system graph can produce
architecture, sequence, security-boundary, documentation, and
interactive views. Diagram Design-class skill grammars are an
implementation candidate, not a core renderer dependency.

# 26. Machine Intelligence and Privileged Execution

Rikuri presents one normalized machine-capability model while using
platform adapters underneath.

### 26.1 OS Capability API

Representative capabilities include:

    package.query/install/remove
    service.status/start/stop/restart
    process.list/inspect/terminate
    network.status/ports
    filesystem.inspect/watch
    sandbox.create/run
    power.sleep/wake-if-supported
    window.list/move/resize-if-supported
    container.inspect/logs/run

Models and skills request the semantic capability; the adapter chooses
`apt/systemd/D-Bus` on Ubuntu, appropriate macOS APIs/launchd/Swift
helpers on macOS, and Windows APIs/PowerShell/services later. Raw shell
remains available only within explicit policy.

### 26.2 Platform order

- **Ubuntu LTS:** primary development and reference implementation for
  local agents, sandbox/envelope, Docker, local inference,
  Tailscale/Rikuri Link, and later CUDA.
- **macOS:** continuous platform-neutral core build/unit/contract tests; native execution envelope and desktop parity remain experimental through Foundation-0. Reuse appropriate AISecurity Rust/Swift components only when their guarantees pass platform-specific probes.
- **Windows:** implement after the OS Capability API survives Ubuntu +
  macOS; no Windows-specific assumption enters canonical kernel
  contracts.
- **Omarchy/other Linux:** optional enhanced adapters may use their
  agent-readable tooling, but Rikuri is not based on Arch/Hyprland.

### 26.3 Privilege brokerage

The long-term privilege broker grants narrow, time/scoped capabilities
rather than blanket passwordless sudo/admin. The approval describes
intent/consequence and produces an auditable capability token where the
platform permits it.

### 26.4 Machine health and incident memory

Machine operations use observe -\> diagnose -\> act -\> verify.
Successful novel repairs may become tested skills/runbooks.
Health/fallback state is visible and absence-of-success can mark a
component degraded.

# 27. Security and Trust Architecture

Security is built around **capability containment and trust
boundaries**, with AISecurity components reused as defense-in-depth
signals where they fit.

## 27.1 Foundation threat model

Foundation-0 assumes imported/web content, model output, local model
processes, future extensions, and external networks can be malicious or
misleading. Accidental model behavior and runaway loops are expected.
The OS kernel and the currently authenticated user account are trusted
computing-base assumptions; root/admin or hardware compromise is out of
Foundation-0 scope.

Protected assets include canonical state, Local Only data, Secret
material, credentials, user intent, budget, audit integrity,
backup/recovery keys, and future financial authority.

The detailed threat model is maintained as a companion document at
`docs/security/THREAT_MODEL.md`.

## 27.2 Egress by construction

Rikuri does not promise universal content inspection of every
process/network path.

Egress control by construction

Two deterministic controls form the Foundation-0 promise:

**Cloud/API path:** only trusted Rikuri provider adapters serialize
external model requests. Context admission rejects `local_only` and
`secret` spans before request construction.

**Local worker path:** Local Only content may be mounted only into an
execution envelope whose declared and tested external-network assurance
is `Denied`.

    Local Only source
          -> Context Compiler
          -> require network=Denied
          -> restricted local worker

If the platform cannot prove the required envelope, Rikuri refuses that
execution profile rather than silently weakening the privacy promise.

Linux is the reference strong-enforcement platform for Foundation-0.
macOS may support a no-network sandbox experiment, but host allowlisting
or universal interception is not claimed without stronger platform
capability. Windows strong-enforcement parity is deferred.

## 27.3 Governed versus opaque native workers

Governed workers can be controlled at context, tool, budget, and network
boundaries Rikuri owns. Native runtimes are opaque workers; policy
applies at the envelope boundary, not their hidden internal loop. A
vendor-native worker with unrestricted network therefore cannot receive
Local Only data.

## 27.4 Prompt-injection containment

Prompt-injection detection is a signal, not the root guarantee.

Hard rules:

- imported/retrieved content has `instruction_authority: none`;
- ingestion/distillation workers have no side-effect tools by default;
- tool calls are checked against the current Task Contract and
  capability envelope;
- retrieved text cannot expand tools, filesystem, network, budget, or
  privilege;
- approval descriptions are generated by trusted code from tool schema,
  normalized arguments, risk class, rollback, and verification metadata;
- model narration is visually separated from authoritative approval
  consequences.

An injection may confuse a model; it must not be able to manufacture
authority.

## 27.5 AISecurity reuse

Existing AISecurity code provides useful reusable components including
prompt-injection pattern/heuristic detection, sensitive-data detection,
privacy decisions for trusted outbound paths, command-policy rules,
intent-coherence signals, agent policy vocabulary, package/model
vetting, encryption/vault code, and audit logging.

Foundation-0 must not confuse those detectors with universal
enforcement. In the existing source, the privacy router is a decision
engine whose caller owns transport; non-shell intent actions largely
fall through to configured defaults; and the macOS sandbox profile can
deny all network when offline but cannot itself enforce DNS-host
allowlists for permitted network. These are valuable inputs to Rikuri
Security Core, not a complete boundary.

## 27.6 Project/repository execution trust

A project is untrusted **for execution** until explicitly promoted.
Merely opening, indexing, searching, diffing, or asking questions about
a repository is a read/data operation. It does not authorize
project-controlled execution.

Before trusted execution, Rikuri inspects or constrains execution
surfaces including:

- `.git/config`, `core.fsmonitor`, Git hooks and helpers;
- package lifecycle/install scripts;
- Make/CMake/task runners;
- Cargo build scripts/proc-macro/tooling paths where applicable;
- Python packaging/build hooks;
- npm/pnpm/yarn scripts;
- Dockerfiles/compose/devcontainer definitions;
- MCP/workspace/tool configuration;
- shell scripts, binaries, generated tools, and interpreter directives.

**Descendant-process rule:** approval of a parent command does not
implicitly approve arbitrary project-configured descendants. The
execution envelope constrains the process tree. A benign-looking
`git status` must not become an escape hatch through a configured
helper.

Trust elevation is explicit, scoped to a project/version/worktree or
declared surface, reversible, recorded in the security ledger, and
invalidated/reviewed when relevant executable metadata changes. Unknown
projects begin in inspect/read-only mode; if execution is needed before
trust promotion, it happens in the restricted Ubuntu reference envelope
without user secrets and with constrained network/filesystem/privilege.

## 27.7 Trusted approval renderer

Consequential approval cards are rendered from deterministic tool
metadata, not generated model prose. The user sees operation, concrete
target, consequence, risk, rollback availability, and verification plan.
Raw command/model rationale is secondary/advanced information.

## 27.8 Security health

Security modules report `Healthy`, `Degraded`, or `Critical` based on
required enforcement capability and recent successful checks. A liveness
HTTP response alone is not proof of protection. Capabilities that depend
on unavailable enforcement fail closed where the policy requires it.

## 27.9 Independently hardened enforcement boundary

LOCKED architectural intent: AISecurity is treated as promising reusable
security machinery, not as a proven universal security boundary. The
September 2026 protection matrix demonstrated strong pre-write blocking
for the tested remote-agent/FUSE path, including denial before a
writable descriptor was issued, but did not validate native macOS attack
paths, enforcement-daemon resistance, flags, extended attributes, ACL
manipulation, every exfiltration path, or every encrypted-copy/key
semantic.

Rikuri must engineer and validate the boundary independently across each
supported platform. Protection claims are scoped to named threat paths
and evidence. If the required control is unavailable or stale, the
dependent capability is Degraded or Critical and fails closed where the
policy requires it.

## 27.10 Layered authorization and session behavior

Security evaluation proceeds through deterministic request validation
and capability/authority checks, optional semantic intent/coherence risk
analysis before tool execution, actual execution within the envelope, a
durable provenance event, and session-level behavior/anomaly analysis.
Semantic models may raise risk or require review; they do not replace
deterministic allow/deny controls.

Behavioral policy evaluates sequences, not only individual operations.
Examples include bulk sequential reads, unusual compression followed by
an outbound connection, repeated denied requests, rapid privilege
expansion, or access patterns inconsistent with the Task Contract.
Thresholds and response policy are versioned and testable.

## 27.11 Security evidence and regression matrix

Every security-relevant request, approval, denial, capability grant,
tool call, outcome, alert, and verification emits a structured event
linking actor, task, parent event, policy version, capability token,
concrete target, decision, result, and evidence hash. Cryptographic
signing is a target for cross-process or cross-device trust boundaries;
Foundation-0 first guarantees tamper-evident integrity appropriate to
the local threat model.

The AISecurity protection matrix becomes a versioned automated
regression suite. Previously blocked operations becoming allowed,
blocked attacks becoming invisible, or required alerts failing are
build-breaking security regressions. The suite expands to native/local
paths, daemon interruption, flags/xattrs/ACLs, copy/export/exfiltration,
bulk reads, encrypted copies and key lifecycle, recovery, and
user-visible notification.

# 28. Workspace and Window Architecture

The Rikuri Workspace is a native **Sterling-style spatial workbench**,
not an IDE panel layout. A Window is a first-class view over
project/task/context state. Monaco is one Window type; chat, agent
status, terminal, diff, test, logs, browser, research, dashboards, and
future domain surfaces are peers.

Target capabilities:

- freely create/move/resize/minimize/maximize/close windows;
- optional equal grids and asymmetric presets without permanently
  locking tiles;
- Context, Agent, and Prompt link groups independent of physical layout;
- saved conceptual workspace layouts;
- detachable/native windows and multi-monitor/desktop/Space placement
  where supported;
- user-controlled visibility from one simple surface to dense
  multi-window operation;
- agent-callable Workspace API for layout/focus/window creation under
  policy;
- status aggregation inspired by proven multi-agent consoles: `Working`,
  `Blocked/Needs Attention`, `Done`, `Idle/Waiting`, rolled up to
  task/project level.

**Foundation-0 still does not require the rich spatial implementation.**
It freezes the Workspace object/window/link contracts and ships one
ordinary desktop surface only after command-line/core gates are green.
Stage 2 implements the actual Sterling window manager after the Pi
coding dogfood stage proves editor/agent integration.

Physical geometry remains device-specific; logical window identity, link
groups, project/task association, and saved conceptual layouts may later
synchronize.

## 28.1 Rikuri Task Spaces

A Task Space is the isolated working environment for one task or agent.
It may contain a dedicated browser context, terminal, editor, files,
credentials, network policy, tools, logs, budget, and audit trail. Human
and agent control can be transferred explicitly; automation that
requires agent control pauses while the human owns the space.

Task Spaces generalize the isolated browser-space pattern: an agent
works without commandeering the user's everyday browser or desktop.
Profiles expose only approved login state and storage. Structured page
snapshots and semantic references are preferred to continuous
screenshots when they are sufficient. Ego Lite-class designs are
references; Rikuri must retain its own policy boundary and
cross-platform abstraction.

# 29. Experience Surfaces

The target experience still includes Quick Assist, Home, full Workspace,
and specialized Dashboards. Foundation-0 ships only the **Home** surface
plus simple Capture, Search, Project, and task views.

The usability acceptance rule is stronger than “has a simple UI”:

> A fresh install reaches Home without a setup gauntlet. Missing
> model/runtime capability is requested just-in-time when the user first
> needs it.

The UI remains a client of `rikurid`; it owns presentation, not
canonical business logic.

# 30. Interaction Profile, Personality, Voice, and Language

User interaction behavior is distinct from governance/security.

An Interaction Profile may store:

- verbosity
- technical level
- tone/formality
- humor
- teaching style
- degree of challenge/skepticism
- preference for questions vs initiative
- default language
- voice selection and speaking rate
- preferred response structure

Users change it naturally:

> “Be more concise.”  
> “Teach me instead of just solving it.”  
> “Challenge my assumptions.”  
> “Use Spanish with me.”

Profiles can have workspace overrides without changing the user’s global
profile.

The Constitution remains separate and cannot be weakened by a
personality setting.

# 31. Themes and Accessibility

**P0 themes:** System, Light, Dark, High Contrast.

UI components use semantic tokens rather than hard-coded colors:

    surface.primary
    surface.secondary
    text.primary
    text.muted
    accent
    success
    warning
    critical
    focus

Later customization can include accent color, density, font scale, code
theme, window chrome, and extension theme packs.

Accessibility requirements include keyboard navigation, visible focus,
scalable text, screen-reader semantics, color-contrast testing, and
reduced-motion support.

# 32. Coding Workspace Architecture

Rikuri’s coding experience is built around **Monaco + Pi + Rikuri-native
developer services** inside the Sterling Workspace. It is not built on
VS Code or Cursor.

## 32.1 Monaco editor surface

Monaco supplies code editing, syntax/highlight surfaces, selections,
diagnostics presentation, multi-cursor, and diff/editor primitives. It
is a Window type, not the application chassis.

## 32.2 Native developer capability layer

Rikuri integrates mature open protocols/tools behind its own capability
contracts:

- Language Server Protocol for language intelligence;
- Debug Adapter Protocol where useful;
- Git/source-control service;
- terminal/PTY service (xterm.js-class presentation);
- test/build/task runners;
- formatter/linter integration;
- repository/symbol search;
- Docker/container/dev-environment tools;
- live diff/review and app/browser preview.

These capabilities are shared by the human UI and Pi/Rikuri agents where
policy permits. A VS Code extension is not required merely because the
underlying language server/tool is commonly distributed through VS Code.

## 32.3 Pi/Rikuri coding agent

The default local coding path is:

    Monaco / task request
      -> Rikuri project context + trust classification
      -> Pi + Rikuri Harness Profile
      -> read/search/edit/test/terminal tools through Rikuri policy
      -> diff + verification evidence
      -> Monaco/Test/Terminal/Agent windows

Coding profiles add repository exploration, planning, context
management, editing strategy, compile/test/debug loops, Git awareness,
review, and bounded subagents. Model-family behavior lives in
profiles/extensions rather than separate Qwen/DeepSeek full harnesses.

## 32.4 Plugin and extension policy

Rikuri does not promise universal VS Code Marketplace compatibility.
Selective `monaco-vscode-api`-class services may be used when valuable,
but native integration or a Rikuri plugin/window is preferred. All
plugin/tool capability flows through AISecurity.

## 32.5 External editor coexistence

VS Code, Cursor, vendor IDEs, and terminal workflows may coexist. A
later bridge/extension can expose Rikuri state or commands, but external
editors do not become the canonical workspace/state owner.

# 33. Domain Workspace Framework

Coding, Writing, Research, Study, Marketing, Product Development,
Business Planning, Trading, Travel, and future domains all use the same
Workspace primitives and canonical state.

A domain pack may define:

- window templates
- skills
- specialized entity schemas
- recommended tools
- workflow templates
- verification logic
- retrieval policies
- dashboards
- context menus/actions

Domain packs do not create isolated memory silos.

## 33.1 Writing example

Writing adds Story/Document Canon, characters, locations, scenes,
timeline, voice/style, continuity checks, revision history, comments,
tracked suggestions, and narrative refactoring.

The same Idea Inbox stores fragments from voice, mobile notes, photos,
conversations, or handwritten captures. An idea remains an idea until
adopted.

# 34. Live Companion: Screen, Audio, Meeting, and Lecture Assistance

Rikuri may assist with what the user is currently seeing or hearing when
the platform permits and the user explicitly activates capture.

Sources may include:

- selected screen/window
- system audio where OS permits
- microphone
- camera where explicitly chosen
- browser tab/media
- shared meeting/lecture transcript

Use cases include real-time notes, explanation, translation,
accessibility assistance, meeting decisions/tasks, lecture notes, and
contextual help.

**LOCKED privacy rule:** capture is explicit and visibly indicated.
Rikuri must show when it is watching/listening and provide immediate
stop controls.

Raw capture can be processed locally when privacy policy requires.
Call/meeting recording remains subject to platform capability and
applicable consent/legal requirements.

# 35. Visual and Artifact Rendering

Rikuri should remain productive even when the user does not own
Microsoft 365, Adobe Creative Cloud, or another commercial suite.

**PROVISIONAL open rendering stack:**

- Mermaid-class declarative diagrams
- Excalidraw/tldraw-class freeform canvas
- Vega-Lite/Plotly-class data charts
- SVG/HTML Canvas for custom graphics
- HTML/CSS/browser rendering for rich interactive artifacts
- ONLYOFFICE/LibreOffice integration for office documents
- FFmpeg-class media processing

Browser/webview rendering provides a portable baseline across macOS,
Linux, and Windows. Artifacts should retain an editable source
representation when practical and export to common formats such as PNG,
SVG, PDF, HTML, Markdown, DOCX/ODT, or media files.

# 36. Internationalization and Multilingual Intelligence

Internationalization remains a foundation-level architecture requirement
while product translation is deferred.

Foundation-0 requirements:

- English is the only shipped human translation;
- every user-facing shell string is externalized;
- `en-US`/English resources are versioned separately from UI components;
- a pseudo-locale such as `en-XA` is rendered in CI to expose hardcoded
  text and layout assumptions;
- all text/storage paths are Unicode-safe;
- locale/language tags use standard identifiers where applicable;
- dates, numbers, and future currencies use locale-aware formatters
  rather than manual formatting;
- source-content language is metadata separate from interface locale;
- search/index storage never requires English translation of the
  canonical original.

Later releases add Spanish, Simplified Chinese, Traditional Chinese
(Taiwan), Japanese, French, and German UI resources and language-aware
model evaluation/routing.

# 37. Documentation, Self-Help, and RAG Architecture

Documentation is canonical source code for the product and ships
version-matched with the application.

Foundation-0 repository:

    docs/
      architecture/
      architecture/adr/
      security/
      data/
      user/
      PARKING_LOT.md

Behavior-changing code changes update documentation in the same
review/merge. CI validates internal links and examples where practical.

RAG/search indexes over documentation are derived. The canonical help
source is Markdown/docs-as-code, not Chroma or another vector database.

A feature/tool can expose a stable help identifier so future “How do I
use this?” queries resolve to documentation matching the installed
Rikuri version.

# 38. Personal Mesh and Multi-Device Architecture

Rikuri separates three concerns that are often incorrectly collapsed
into “cloud”: **remote control**, **execution placement**, and **state
replication**.

## 38.1 Rikuri Link — remote control / last mile

A remote client on phone/tablet/laptop/another computer can securely
control an owner-operated Rikuri Node. The node keeps persistent agents,
files, credentials, memory, tools, local models, and project resources
when practical. The remote client sends intents/voice/files, receives
status/results, and handles approvals; larger clients may render the
full Workspace.

Initial implementation uses **Tailscale** as the practical secure
connector: private device identity, WireGuard-based encrypted
connectivity, NAT traversal, and no router port-forwarding requirement.
Rikuri UI hides VPN/IP/SSH concepts behind device presence and
`Run on: Auto`.

Long term, Rikuri adds its own end-to-end encrypted connection layer:

    Rikuri Client
      -> direct connection when possible
      -> small rendezvous/discovery service
      -> encrypted relay only when direct path fails
      -> Rikuri Node

The service handles discovery/connection/push, not plaintext agent work.
Tailscale remains a supported fallback connector after native Link
matures.

## 38.2 Node registry and execution placement

Rikuri represents machines as Nodes with online/sleeping/offline
presence, capability inventory, project/resource locations, and policy.
Later the router may choose a node by GPU/RAM/software/project
locality/energy/privacy. Remote wake/gateway support is optional and
capability-declared.

## 38.3 Personal Mesh replication

State replication remains later than single-machine correctness and
backup/restore. The append-only ledger and stable IDs provide a future
basis. CRDTs are reserved for genuinely concurrent editable text. Remote
Link can exist without replicating canonical state, and Mesh can later
use Link transport without becoming the same subsystem.

Priority order:

    single-machine correctness
     -> encrypted backup/restore
     -> Pi coding dogfood
     -> secure remote Link to owner node
     -> optional second-device canonical replication
     -> hub/mesh
     -> CRDT only when needed

## 38.4 Endpoint-secured Link relay

LOCKED architectural intent: a fallback Rikuri Link relay is an
untrusted transport, not a trusted session endpoint. Pairing establishes
endpoint device identity and authorization; application-layer end-to-end
encryption protects task content, approvals, files, audio, and session
state so the relay routes opaque envelopes and cannot read or alter them
undetected.

Direct or Tailscale-class private connectivity remains preferred when
practical. Relay metadata exposure, replay resistance, key rotation,
device revocation, short-lived pairing, lost-device recovery, and
self-hosted relay operation require explicit threat modeling and tests.
Relay compromise must not grant node authority.

# 39. Backup, Restore, and Time-Machine-Like Recovery

Backup/restore is part of Foundation-0 because it proves that Rikuri’s
state is actually owned by the user.

### 39.1 Foundation restore point

A consistent restore point contains or references:

- SQLite canonical snapshot
- encrypted content blobs needed by live objects
- current recovery/key metadata needed for those objects
- source/import manifests
- settings and configured execution profiles excluding raw secrets
- schema/application version
- hash/integrity manifest
- documentation version

Foundation-0 does **not** implement a custom restic-style deduplication
system. First prove correct encrypted snapshot, integrity verification,
wipe, and restore. Dedupe/chunking can be added behind the backup format
later.

### 39.2 Deletion-aware backups

Managed backup history must not quietly resurrect a forgotten object’s usable DEK, including a snapshot made **before** forgetting. Forgetting advances a durable revocation record that managed restore must validate against current trusted state before materializing any old snapshot. If current revocation state cannot be authenticated, restore safely refuses or quarantines affected sensitive material; it cannot report success and expose an old usable key. Later backup snapshots exclude the forgotten key. Retention/GC of encrypted orphan ciphertext is separate from key destruction. Exports and copies outside Rikuri-managed restore cannot be revoked by this guarantee.

### 39.3 Restore acceptance

    backup -> wipe application/state -> fresh install -> restore --verify

The recovered installation must preserve stable IDs, projects, ideas,
decisions, settings, and provenance.

# 40. Observability, Proof of Work, and Health

Observability exists to prove behavior, not to drown the user in logs.

Foundation-0 records:

- structured lifecycle events
- security/policy decisions
- budget reservations and reconciliation
- execution profile identity
- verification result
- token/cost/latency when available
- import parser/version/hash
- backup/restore verification

### 40.1 Nondeterministic-test strategy

Live frontier behavior is never required for core CI. Testing layers
are:

Foundation-0 test pyramid

`rikuri-testkit` provides fake model adapters, fake tools, deterministic
provider usage/cost fixtures, crash injection, and replay traces.

Golden fixtures cover importers, context packages, approval cards,
ledger replay, budget reservation, migrations, backup/restore, and
prompt-injection attempts.

### 40.2 Security health

Health reports enforcement state and latest successful verification for
required controls, not merely process liveness.

### 40.3 Provenance and user visible security events

The event ledger is also the reconstructable provenance graph for human,
agent, workflow, plugin, connector, and security activity. The UI
presents useful summaries and immediate alerts for material denials or
suspicious behavior while preserving the underlying evidence for audit
and debugging. Silent successful blocking is not sufficient for
consequential attacks.

# 41. Failure and Recovery Semantics

Foundation-0 failure semantics favor explicit degraded state and
recoverability.

### 41.1 Trusted daemon crash

Committed transactions survive. On restart, task/checkpoint state
reconstructs from canonical versioned state plus ledger. Consequential
actions with committed idempotency keys are not repeated.

### 41.2 Worker crash

Worker state is treated as disposable. The task returns to a recoverable
checkpoint or fails with evidence. Canonical state remains consistent.

### 41.3 Provider failure

No paid retry occurs outside the task’s retry/time/budget envelope.
Reserved budget is reconciled or released according to known usage.

### 41.4 Index corruption

Mark search degraded, preserve canonical state, rebuild indexes from
source/canonical records, rerun retrieval eval.

### 41.5 Budget exhaustion

Block new paid calls, checkpoint, and require explicit budget expansion
or a policy-permitted local continuation.

### 41.6 Security guarantee unavailable

If a task requires a guarantee such as `external_network=Denied` and no
current envelope backend can prove it, the task/profile is refused. UI
language must say the guarantee is unavailable rather than claiming
protection.

### 41.7 Import parser failure

The failed source remains untouched. No partial canonical import is
committed; parser/version/error is recorded for diagnosis.

# 42. API Boundaries and Versioning

Foundation-0 versioned Rikuri-owned contracts:

- Core/IPC API
- Repository traits
- Event Schema
- Knowledge/Provenance Schema
- Context Package Schema
- Model/Provider Adapter API
- Tool Schema
- Security/Envelope API
- Budget API
- Importer API
- VectorIndex API
- Backup Manifest Format
- Harness Adapter / Harness Profile Schema
- Runtime Compatibility Record
- Project Trust / Execution Trust Contract
- OS Capability API

Foundation freezes minimal semantic shapes for Workspace Window/Link
identity and Node identity without implementing rich behavior. Future
contracts add Workflow DAG, full Workspace API, Rikuri Plugin SDK,
Rikuri Link native transport, Personal Mesh replication protocol, and
domain SDKs.

Breaking changes require migration or parallel-version support according
to the contract’s stability class. Adapters declare compatible
API/schema ranges.

# 43. Schema Migration Policy

Transactional State Evolution: preserve/read old authoritative state -\>
validate inputs -\> transform complete candidate -\> validate candidate
-\> durably write -\> read back and verify integrity and semantics -\>
atomically promote -\> only then retire/archive old state. On any
pre-commit failure, the last known-good state remains authoritative and
the operation is retryable. Test each crash point and restoration.

Canonical schema changes must be:

- versioned
- transactional
- tested from supported prior versions
- backup-aware
- recoverable
- documented

The updater creates or verifies a restore point before high-risk
canonical migrations.

Migration failures must not leave the user with a half-upgraded Personal
Intelligence.

# 44. Update and Release Architecture

Rikuri distinguishes:

- Core application updates
- schema migrations
- extension updates
- model/runtime updates
- docs package updates
- derived-index rebuilds

Updates are staged, versioned, and health-checked. A
model/runtime/quantization change that may alter measured behavior
triggers selective Model Lab revalidation.

The protected core cannot be replaced by an agent simply because an
agent generated a patch.

# 45. Repository and Module Organization

Foundation-0 remains a modular monolith, but v1.4 adds only the
**implemented** abstractions needed to prevent later rework. No empty
future subsystem is created merely because it appears below.

    rikuri/
    ├── apps/
    │   ├── rikurid/
    │   ├── rikuri-cli/
    │   └── desktop/                     # minimal shell after core gates
    ├── crates/
    │   ├── rikuri-domain/
    │   ├── rikuri-kernel/
    │   ├── rikuri-storage/
    │   ├── rikuri-ledger/
    │   ├── rikuri-content/
    │   ├── rikuri-security/             # privacy, project trust, approval, capabilities
    │   ├── rikuri-import/
    │   ├── rikuri-search/
    │   ├── rikuri-context/
    │   ├── rikuri-exec/                 # Pi/harness/tool/runtime contracts and handoff
    │   ├── rikuri-platform/             # OS Capability API and machine abstractions
    │   ├── rikuri-budget/
    │   ├── rikuri-backup/
    │   └── rikuri-testkit/
    ├── adapters/
    │   ├── harness-pi/                  # appears when Pi integration starts
    │   ├── model-openai-compat/
    │   ├── local-openai-compat/
    │   ├── platform-ubuntu/
    │   ├── platform-macos/
    │   ├── envelope-ubuntu/
    │   └── envelope-macos-experimental/
    ├── profiles/
    │   └── harness/                     # versioned family/task/runtime overlays once implemented
    ├── packages/
    │   ├── desktop-ui/                  # Monaco only when coding stage begins
    │   └── i18n/
    ├── schemas/
    │   ├── event/
    │   ├── rpc/
    │   ├── tool/
    │   ├── import/
    │   ├── harness-profile/
    │   └── project-trust/
    ├── docs/ ... build-records / stage-gates / modules / traceability / incidents
    └── tests/ ... contracts / properties / replay / retrieval / security / migration / restore

### 45.1 Dependency direction

`rikuri-domain` stays pure. `rikuri-kernel` depends on Rikuri-owned
traits. Pi, runtime, OS, provider, UI, and future Link implementations
are adapters. UI never opens canonical SQLite directly. Workers never
receive canonical write authority. Project-controlled processes cannot
bypass `rikuri-security`/execution-envelope admission by spawning
descendants.

The repository has no empty `mesh`, `link`, `extension-marketplace`, or
rich `workspace-window-manager` tree before those stages begin. The
contract may exist in docs/schema first; implementation appears only
when wired to a real caller and gate.

# 46. Technology Decision Register

| Area                        | Status                                | v1.4 decision                                                                                                 |
|-----------------------------|---------------------------------------|---------------------------------------------------------------------------------------------------------------|
| Architecture style          | LOCKED                                | modular monolith first; extract processes later                                                               |
| Trusted/core language       | LOCKED Foundation-0                   | Rust                                                                                                          |
| Canonical personal DB       | LOCKED Foundation-0                   | SQLite/WAL/single canonical writer                                                                            |
| Event model                 | LOCKED                                | current state + referenced-content append-only ledger                                                         |
| IDs                         | PROVISIONAL                           | ULID unless ADR changes                                                                                       |
| Human knowledge projection  | LOCKED direction                      | generated Markdown projection; source Markdown imported separately                                            |
| Full-text search            | PROVISIONAL                           | SQLite FTS5 baseline                                                                                          |
| Model identity/economics    | LOCKED direction                      | dynamic registry/access/price/quota data; no model-name kernel branches                                       |
| Governed local/open harness | **LOCKED direction**                  | **Pi is the primary/default substrate**; Rikuri owns profiles/security/evidence/state                         |
| Native/open harnesses       | LOCKED policy                         | reference/benchmark/optional exception adapter only when measured benefit justifies                           |
| Harness compatibility       | LOCKED direction                      | runtime compatibility separate from agent/Harness Profile compatibility                                       |
| Inference runtimes          | OPEN adapters                         | llama.cpp/vLLM/Ollama/LM Studio/others behind runtime adapters                                                |
| Privacy UX                  | LOCKED concept                        | Auto default; Private; Local Only/Vault; project/workspace persistent intent                                  |
| Project execution trust     | LOCKED                                | new project read/untrusted by default; explicit scoped/reversible execution promotion; descendant containment |
| OS abstraction              | LOCKED direction                      | OS Capability API; Ubuntu full Foundation reference, macOS continuous core tests with native assurance experimental, Windows later |
| Linux base                  | **LOCKED reference**                  | Ubuntu LTS; Omarchy ideas optional, not dependency                                                            |
| Egress promise              | LOCKED                                | context admission + proven no-network envelope by construction                                                |
| Budget hard stop            | LOCKED                                | atomic reservation before paid call                                                                           |
| Backup/deletion             | LOCKED direction                      | encrypted verified snapshot + deletion-unit key destruction/tombstone                                         |
| Desktop shell               | PROVISIONAL                           | Tauri-class shell                                                                                             |
| Code editor                 | **LOCKED current direction**          | Monaco embedded in Rikuri; not VS Code workbench                                                              |
| Developer services          | LOCKED direction                      | LSP/DAP/Git/terminal/tests/search/formatters/containers directly integrated                                   |
| Plugin API                  | LOCKED direction                      | Rikuri capability-based plugins/windows; selective VS Code compatibility only                                 |
| Spatial workspace           | LOCKED concept / later implementation | Sterling-style freeform/tiled/detached/multi-monitor windows + links                                          |
| Remote control              | LOCKED direction                      | Rikuri Link: Tailscale first, later native E2E direct/rendezvous/relay; Tailscale fallback                    |
| Personal Mesh               | DEFERRED implementation               | replication after durability/restore and Link; separate from remote control                                   |
| i18n                        | LOCKED plumbing                       | English + pseudo-locale in Foundation; broader localization later                                             |

# 47. Proven Interaction Patterns to Preserve

Rikuri continues to borrow proven **patterns**, not product
dependencies. Foundation-0 implements only patterns that strengthen the
core.

| Proven pattern                                | Rikuri interpretation                                                                           | Foundation-0                          |
|-----------------------------------------------|-------------------------------------------------------------------------------------------------|---------------------------------------|
| Persistent memory/state                       | Rikuri-owned canonical state and provenance                                                     | YES                                   |
| Plan/task contract before action              | explicit task objective + verification + envelope                                               | YES, minimal                          |
| Durable/background work                       | checkpointed task state                                                                         | minimal crash-resume only             |
| Native model strengths                        | native runtimes treated as opaque workers                                                       | architecture only                     |
| Skills/procedures                             | domain packages independent of model                                                            | later                                 |
| Subagents                                     | dependency-aware specialists                                                                    | later                                 |
| Isolated browser/computer                     | dedicated capability-limited worker                                                             | later                                 |
| Visual diffs/tests/proof                      | independent verification artifacts                                                              | basic verification now; rich UI later |
| Connector ecosystem                           | just-in-time capability connection                                                              | later                                 |
| Cloud work computers                          | optional remote workers; persistent owner-node agent preferred when practical                   | later                                 |
| Agent-state consoles (Herdr-class)            | Working/Blocked/Done/Idle aggregation + detach/reattach persistence                             | workspace stage                       |
| Agent-operable OS (Omarchy-class)             | deterministic OS capability commands, skills, crash-to-agent diagnostics; Ubuntu implementation | OS adapter stage                      |
| Mature-tool wrapping (Omaport/rclone pattern) | put Rikuri UX/security around proven infrastructure instead of rebuilding protocols             | ongoing                               |

This keeps useful lessons from OpenClaw-, Hermes-, Cowork-, Cursor-,
Pi-, and other agent systems without making their implementation model a
Rikuri dependency.

# 48. Development Compatibility Matrix

Foundation-0 and post-Foundation stages use an explicit
platform/capability matrix.

**Continuous Foundation targets**

- Ubuntu LTS: primary reference build/test, no-network envelope,
  project-trust/descendant tests, local runtime/Pi integration when
  stage starts;
- macOS: Rust/core build/tests continuously; explicit assurance reports
  for platform-specific enforcement;
- deterministic fake-model/Pi-adapter contract tests with no live
  frontier dependency;
- SQLite migration/recovery, import, retrieval, backup/forget fixtures;
- one cloud/broker adapter and one local runtime integration in opt-in
  tests;
- English + pseudo-locale rendering when desktop begins.

**Stage-gated later targets**

- Windows OS Capability/envelope parity after Ubuntu/macOS abstraction
  is stable;
- rich Monaco/LSP/DAP/Git coding integration;
- Sterling multi-monitor/detach;
- Tailscale Link and later native Link transport;
- plugin lifecycle/sandbox;
- native harness exception adapters;
- full locale set, browser workers, connectors, multi-agent
  orchestration, Personal Mesh.

A platform/capability is not marked supported merely because it
compiles. Its contract/integration/security/health acceptance must pass
on that platform.

# 49. Implementation Stages

The roadmap now separates **Foundation contracts** from **rich
capability implementation** and uses a strict wiring gate: each stage
must be integrated into the running system before a dependent stage
begins.

## Stage 0 / Foundation-0 — trusted core + Pi/security seams

- Engineering Standard, Build Records, Stage Gates, quality/debt
  ratchets, CI/red-test drill;
- Ubuntu-first repository bootstrap; macOS continuous core validation;
- IDs/schema/SQLite/current state + ledger/idempotency;
- threat model, privacy classes, project execution trust
  (`read != execute`), descendant-process rule;
- OS Capability API contract + Ubuntu reference envelope;
- content/deletion primitives; real imports + project registry/resource
  resolver;
- FTS baseline + rebuildable vector boundary + retrieval corpus;
- Context Compiler with Auto/Private/Local Only admission;
- Pi Harness Adapter contract and one Pi-backed local-model coding/tool
  vertical slice;
- model-family Harness Profile schema + runtime-vs-agent compatibility
  record;
- dynamic execution/access/economics minimum + hard budget reservation;
- one cloud adapter + cloud/local handoff;
- trusted approval renderer + prompt-injection/project-execution
  adversarial fixtures;
- encrypted backup/wipe/restore/forget;
- minimal Home shell + i18n plumbing only after core gates.

**Exit:** W1 + F1-F10 pass from a clean checkout; one untrusted
repository can be inspected without execution, explicitly promoted, then
safely edited/tested through Pi under Ubuntu; docs/evidence match
implementation.

## Stage 1 — Pi coding dogfood + native developer platform

- Monaco editor Window;
- terminal/diff/tests/build;
- LSP language intelligence; DAP baseline where valuable;
- Git service + project trust UI;
- formatter/linter/search/container integrations;
- richer Pi coding profiles and review/test loops;
- use Rikuri to perform real Rikuri development under the Build
  Procedure.

**Exit:** a normal Rikuri change can be planned, implemented by/with Pi,
reviewed in Monaco/diff/tests, full-gated, and recorded without leaving
Rikuri except for external tools intentionally invoked.

## Stage 2 — Sterling Workspace substrate

- first-class Window registry and Workspace API;
- freeform resize/move + optional grids/presets;
- Context/Agent/Prompt Links;
- saved layouts;
- detach/multi-monitor;
- Working/Blocked/Done/Idle rollups;
- Rikuri Plugin API + native Window type registration + capability
  permissions.

**Exit:** multiple chat/code/agent/terminal/test windows coexist and
remain logically linked across layout changes/restarts.

## Stage 3 — Rikuri Link / remote owner-node control

- node identity/presence;
- Tailscale connector;
- thin remote client: chat/voice/status/approvals/file-photo handoff;
- larger-client full Workspace path;
- secure reconnect and durable-agent continuity;
- design/prototype native direct+rendezvous+E2E relay; keep Tailscale as
  fallback.

**Exit:** user can leave home, securely phone home to the Ubuntu Rikuri
Node, continue a task, approve/deny work, and recover from connection
loss without exposing a public inbound service.

## Stage 4 — Model Lab + intelligent routing

- dynamic local/provider discovery;
- exact execution identity;
- open-harness change analyzer/benchmark comparisons;
- hardware fit + model-family profile evaluation;
- price/quota/local-cost advisor;
- evidence-based Auto;
- native harness exception adapters only when justified.

### Post Foundation provider candidates

The Local Model Service may evaluate Unsloth-class providers for
efficient fine-tuning, quantization, export, inference, and
OpenAI-compatible serving across supported hardware. Rikuri owns model
identity, routing, policy, evaluation, and provenance; the provider owns
hardware-specific optimization. Local-first execution is selected by
privacy, capability, latency, cost, quota, and measured success, with
frontier escalation only when justified and authorized.

Windows plus RTX remains a first-class portability target after the
Ubuntu reference path and macOS Tier-1 validation. No local-provider
integration may place CUDA, MLX, ROCm, Vulkan, or vendor-specific
assumptions in Kernel contracts.

### Specialist model composition

Stage 4 expands the registry and router from single-profile selection to
bounded composite execution: lead reasoning/coding plus optional vision,
speech, embedding, retrieval, and verification roles. Eco Coding-class
orchestration is a reference pattern, not a dependency. Each role is
independently replaceable and measured, and the headless runtime remains
authoritative while desktop/mobile clients attach.

## Stage 5 — Browser/connectors/personal assistant

Browser/computer worker, email/calendar/contacts, just-in-time
connectors, research/travel/shopping, Attention Firewall maturity.

## Stage 6 — Multi-agent governance + Project Graph

Coordinator/COO, evaluator, Strategy Council, durable DAG/parallel work,
Project Graph, team economics, consolidated user communication.

## Stage 7 — Personal Mesh + persistent responsibilities

Encrypted canonical replication, optional hub, cross-device resource
catalog, durable routines, quiet digests, learned skills.

## Stage 8 — Rich domains / Live Companion

Writing/study/research/media/meeting/lecture and specialized domain
packs on common primitives.

## Stage 9 — Business/team readiness

Tenancy, organization policy/memory, shared projects, admin/audit,
delegated approvals, server deployment.

### Stage transition rule

A stage closes only after:

    contract/unit tests
     -> adapter/integration tests
     -> real running vertical slice
     -> failure/recovery/security path
     -> health/observability evidence
     -> docs/runbook/help update
     -> Scrap Sweep
     -> full gate
     -> Stage Gate COMPLETE

If an item is postponed, its checklist entry becomes
`DEFERRED/RESEQUENCED` with reason, dependency impact, new
stage/position, owner, trigger/acceptance condition, and evidence. All
dependent checklist ordering is updated in the same change.

# 50. Architecture Acceptance Tests

Architecture v1.6 defines ten **Foundation-0 acceptance demonstrations** plus the W1 kernel crash gate. The live Stage Plan also tracks Bootstrap and the F4-lite intermediate policy checkpoint; F4 closes only in Stage 04. The eighth demonstration proves that Rikuri
itself is being built under the same evidence-first project discipline
it should eventually provide to users.

| ID  | Demonstration                                                                                                                                                                                                                                                                                                      | Architectural promise proved                                                                       |
|-----|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------|
| W1  | Kill the trusted core at kernel transaction crash points in Stage 00; restart and reconstruct committed state without duplicate effects. Rerun all eight cross-component points at Foundation-0 release. | Continuity is Rikuri-owned, with full recovery evidence after dependent components exist. |
| F1  | Fresh install opens Home with one prompt box/launcher row; no setup gauntlet.                                                                                                                                                                                                                                      | Usability and empty-state kernel.                                                                  |
| F2  | Import a real ChatGPT export and user Markdown/Obsidian history twice; preserve stable IDs, originals, provenance, and idempotent count. Claude and read-only Git/project manifest importers follow early in Stage 1. | Real import contract and usable provenance in the first slice. |
| F3  | Capture a text idea, retrieve it through baseline lexical search with provenance, open original, and attach it to a project without confusing Idea and Decision. Voice and semantic retrieval remain later product work. | Capture first, structure later, with a visible working path. |
| F4  | Start on a cloud/broker API and hand off to a local runtime without restating project state. Demonstrate that Local Only data is rejected from cloud context and works only in a proven no-network local envelope.                                                                                                 | Continuity + egress by construction.                                                               |
| F5  | Run a task with a \$1 hard budget; atomic reservation prevents a request that could exceed remaining budget.                                                                                                                                                                                                       | Spending control below the model.                                                                  |
| F6  | Rebuild derived indexes; create managed backups before and after forgetting, wipe and restore each with current durable revocation state; a forgotten item cannot materialize. Refuse or quarantine an old backup without trustworthy current revocation. | Derived rebuildability, ownership, and deletion across old managed restore points. |
| F7  | Import malicious instruction-like content; prove it remains searchable data with zero authority, cannot cause side effects, cannot expand permissions, and cannot control approval consequence text.                                                                                                               | Prompt-injection containment by authority separation.                                              |
| F8  | Run a representative change through Plan → Build → Verify → Close Out; prove red CI blocks, evidence and docs are current, orphan cleanup occurred, each existing protected crate has a contract/caller/health/tests, and a Stage Gate closes only with linked receipts. | Professional engineering discipline is enforced throughout Foundation-0. |
| F9  | Open a hostile project containing executable Git/project configuration in untrusted/read-only mode; prove inspection/search cannot execute it. After explicit scoped promotion, run inside the Ubuntu envelope and prove project-controlled descendants cannot escape filesystem/network/secret policy.            | Read does not imply execute; GitSpawn-class/project-controlled execution is contained.             |
| F10 | Run a representative local coding task through Pi + a Rikuri Harness Profile: inspect, edit, test, verify, and record evidence. Swap to a second compatible profile/model fixture without changing the core harness or canonical project state.                                                                    | Pi-first harness architecture and model-family profile separation are real.                        |

Full-platform acceptance tests for rich Monaco/LSP/DAP coding, Sterling
workspace/multi-monitor, Rikuri Link native relay, multi-agent budgets,
privilege broker, multilingual retrieval, Personal Mesh, and
native-harness exception integration remain future-stage gates rather
than Foundation-0 blockers.

# 51. Deferred Prototypes and Post-Foundation Decisions

Architecture v1.6 intentionally closes several v1.0 questions and defers
others until Foundation-0 provides evidence.

### Closed for Foundation-0

- modular monolith instead of premature services;
- current-state tables plus referenced-content event ledger;
- egress by construction rather than universal packet inspection;
- governed workers versus opaque native workers;
- API/broker/local as foundational automation paths;
- one-way Markdown projection plus one-way source import;
- backup/restore before Mesh/CRDT;
- sparse Performance Ledger before learned routing;
- simple single-window shell before spatial workspace;
- English-only product strings with i18n plumbing;
- deletion keys/tombstones designed from the first imported content.

### Prototypes during Foundation-0

1.  E1: transactional kernel recovery versus simplest verified SQLite design; full matrix after other components exist.
2.  E2: two-day throwaway headless Pi/tool-driveability spike.
3.  E3: managed pre-forget backup restore with current revocation and safe refusal when unavailable.
4.  E4: minimal Knowledge Context Compiler against ten required answer/abstain outcomes.
5.  E5: tested Ubuntu network/filesystem envelope after an early namespace/AppArmor feasibility probe.
6.  E6: second compatible Harness Profile with zero kernel diff.

FTS5 is the Foundation-0 retrieval baseline; a VectorIndex quality experiment and native macOS envelope work follow later. The F4 cloud/local integration closes in Stage 04 after the local runtime exists.

### Deferred after Foundation-0

The first model/runtime-economics experiments after Foundation-0 are
intentionally ordered so fast-moving model names remain data rather than
architecture:

1.  Dynamic Model / Access / Economics Registry: provider/local
    discovery, opaque model identifiers, lifecycle, effective-dated
    price cards, flexible quota pools, and confidence-tagged telemetry.
2.  Pi Harness Adapter expansion: Pi is already the primary
    governed-harness substrate; post-Foundation work deepens
    skills/extensions/subagents/session integration and performance
    profiles rather than deciding whether to adopt Pi.
3.  AHP/native-harness client adapter: evaluate native
    Claude/Codex-class harness sessions without making VS Code or a
    vendor harness the Rikuri UI/state authority.
4.  Subscription Quota Registry telemetry: capture aggregate or
    model-specific quota/reset information when exposed and derive
    amortized economics without inventing missing token data.
5.  Model Lab + learned Auto: evaluate dynamically registered execution
    profiles and use verified outcomes only after sufficient evidence.

Additional deferred capabilities:

- native frontier/runtime subscription paths beyond the controlled
  Pi/AHP experiments;
- rich Sterling spatial/multi-monitor workspace beyond the Stage-1
  coding surface;
- production plugin manager/marketplace/MCP/browser worker;
- Model Lab and learned Auto routing;
- Windows strong sandbox parity;
- Rikuri Link native relay (Tailscale may ship earlier) and Personal
  Mesh/CRDT;
- assistant capability packs;
- multi-agent governance;
- screen/audio Live Companion.

Every prototype ends in an ADR with problem, alternatives, evidence,
decision, consequences, and rollback path.

# 52. Architecture Decision Record Process

Major changes after v1.0 use ADRs stored in `docs/architecture/adr/`.

Example:

    ADR-0012: Choose vector-index implementation
    Status: Accepted
    Context
    Options
    Benchmarks
    Security/privacy implications
    Decision
    Consequences
    Migration/rollback

ADRs prevent future contributors or agents from rediscovering and
casually reversing decisions without understanding why they exist.

# 53. Engineering Assurance and Project Quality Management

Rikuri is intended to let a person who may not know software
engineering, MIS project management, CI, testing, security review, or
release management still produce work with professional discipline. The
user supplies ideas, requirements, constraints, judgments, and
acceptance decisions. Rikuri supplies the invisible management machinery
required to keep the project coherent and verifiable.

For Rikuri itself, this section is **binding engineering standard**.
Later, the same architecture becomes a general project capability for
users.

## 53.1 Standard Work hierarchy

Three durable records govern engineering work:

1.  **Engineering Standard** - the long-lived rules for how changes are
    planned, built, debugged, verified, cleaned, documented, and
    released.
2.  **Build Record** - one evidence-bearing record per coherent change.
    It is created during planning and checked as work happens, never
    reconstructed from memory after completion.
3.  **Stage Gate Record** - the live checklist for a
    milestone/week/stage. It links required changes, acceptance
    demonstrations, evidence, blockers, skipped items, and exit status.

An unchecked required item without an explicit, owned, scheduled
exception means the work is not complete.

## 53.1.1 Deferral and resequencing are first-class changes

A skipped/postponed item never remains as an anonymous unchecked box. It
becomes an explicit project-state decision with:

- status: `DEFERRED`, `RESEQUENCED`, `BLOCKED`, `REMOVED`, or
  `SUPERSEDED`;
- why it moved and the evidence/constraint causing the move;
- dependency/acceptance impact;
- new stage/week/ordinal position;
- owner;
- trigger/condition for reactivation;
- updated downstream checklist order;
- ADR/Build Record link when architectural or quality scope changes.

If moving an item invalidates later work, the later work is reordered
rather than allowed to proceed on a missing dependency. Calendar
pressure is not evidence that a dependency disappeared.

## 53.2 Quality Gate Engine

**LOCKED:** where a quality rule can be checked mechanically, Rikuri
should prefer an executable gate over a prose reminder.

Foundation-0 CI progressively includes:

- clean formatting/build;
- compiler/type/static analysis;
- lint;
- unit tests including failure paths;
- contract tests at replaceable interfaces;
- invariant/property tests for protected guarantees;
- migration tests;
- deterministic replay/golden fixtures;
- security fixtures and secret scanning;
- dependency/licensing/provenance policy where available;
- debt ratchets for skips, suppressions, unsafe exceptions, TODO
  classes, or other chosen indicators;
- smoke tests against the running system;
- documentation/help consistency checks;
- clean-tree/scrap-sweep checks where mechanically safe.

A red required gate stops the line. Agents cannot mark it passed,
suppress it, or change the acceptance criteria merely to continue work.
Any permitted override is a visible policy decision with owner, reason,
scope, and due date.

## 53.3 Evidence, not self-attestation

An AI worker may report what it believes it did, but the Project Quality
system distinguishes **claim** from **evidence**.

| Claim                     | Required evidence                                                    |
|---------------------------|----------------------------------------------------------------------|
| “It compiles.”            | Recorded clean build result.                                         |
| “Tests pass.”             | Actual test run and counts.                                          |
| “The bug is fixed.”       | Reproduction plus regression test that fails on old/broken behavior. |
| “Recovery works.”         | Live triggered recovery drill.                                       |
| “The service is healthy.” | Last-success probe, not merely process liveness.                     |
| “The module is complete.” | Module Contract plus required Build/Stage gates.                     |
| “The release is ready.”   | Release gate plus clean-install/restore evidence.                    |

The evidence store may initially be ordinary versioned Markdown plus CI
artifacts and ledger references. Its semantic contract is stable even if
storage changes later.

## 53.4 Module contracts

Every protected-core crate/module maintains a short contract describing:

- purpose and public responsibility;
- what it owns and explicitly does not own;
- trust level and boundaries;
- canonical data touched;
- public interfaces;
- failure and degraded behavior;
- health/last-success signal;
- relevant architectural invariants and ADRs;
- relevant threats/controls;
- contract/invariant tests;
- documentation/help entry points.

The goal is not paperwork. It is to make the codebase legible to future
humans and AI workers without reverse-engineering intent from
implementation.

## 53.5 Contract tests and invariant/property tests

Replaceable adapters share executable contract suites. A new model
adapter, repository backend, vector index, envelope backend, importer,
or backup destination is not considered compatible because it compiles;
it must satisfy the same contract tests as existing implementations.

Protected claims are tested as invariants where practical. Examples
include:

- hard budgets are never exceeded by admitted requests;
- re-importing identical source material never creates duplicate
  canonical objects;
- Local Only material never enters a cloud-bound Context Package;
- derived indexes cannot mutate canonical state;
- forgotten content cannot be materialized without a destroyed key;
- committed consequential effects are not duplicated across
  crash/replay.

Property-based/randomized tests are preferred for state-machine
invariants that have many possible operation sequences.

## 53.6 Scrap Sweep and repository hygiene

The working tree has only two legitimate states for artifacts:
**intentional and wired in**, or **removed**. Temporary scaffolding,
obsolete versions, debug residue, stale generated output, applied
one-shot scripts, dead feature flags, abandoned branches/files, and
proven orphans must not quietly become permanent architecture.

Potential orphans are proven before deletion because dynamic
registration, runbooks, service units, scripts, generated bindings, and
re-exports can hide real callers. Ambiguous candidates are surfaced for
review rather than auto-deleted.

## 53.7 Silent-failure policy

No silent loss, no stale-tail substitution, no false-green status. Every
handoff has a durable ID and acknowledgement; stale/truncated state and
missing ACKs raise typed failures. Health uses actual execution
outcomes, not process liveness or installed scaffolding. A vertical
integration test traces a user-visible action through routing, policy,
tool execution, verified durable result, and visible failure reporting.

Important components expose health around the **absence of success**,
not merely emitted errors. A background subsystem should be able to
report at least:

    state: Healthy | Degraded | Failed
    last_attempt
    last_success
    last_error
    consecutive_failures
    work_completed

If a fallback activates, that activation is an observable event.
Repeated fallback use indicates degraded primary behavior and becomes
reviewable project evidence.

## 53.8 Incident-to-tripwire lifecycle

A production or dogfood incident follows:

    stabilize -> capture evidence -> identify root-cause class
              -> fix -> sweep for sibling cases -> add permanent tripwire
              -> update runbook/docs -> close with evidence

The objective is not merely to fix one instance. Each important incident
should make the system better at preventing or loudly detecting the
entire defect class.

## 53.9 Threat/control/test traceability

Security-sensitive requirements maintain traceability such as:

    Threat / Requirement -> Deterministic Control -> Test / Drill -> Evidence

A protected promise with no named control or no test is incomplete. This
traceability is intentionally simple in Foundation-0 and may later
become canonical structured project data.

## 53.10 Project Quality Manager - future user-facing capability

The same machinery used to build Rikuri becomes a general capability for
user projects. A novice should be able to say what they want to create
and let Rikuri generate the professional structure underneath:

    Idea / requirement
          -> clarified specification
          -> staged plan
          -> acceptance criteria
          -> Build Records / work packages
          -> implementation
          -> automated gates
          -> evidence
          -> documentation
          -> stage review
          -> release readiness

The user sees a calm project view such as:

    Foundation-0                         78%
    Stage: Retrieval

    Passed
      Build / static checks / 487 tests / secret scan

    Needs attention
      3 of 20 retrieval cases still below target
      1 documentation item stale
      restore drill due

    Blocked
      none

The novice does not need to understand the underlying CI system to
benefit from it. Experts may inspect every command, gate, artifact,
test, and policy.

**LOCKED:** deterministic gates remain authoritative. A Project Quality
Manager agent may plan, explain, and coordinate quality work, but it
cannot approve its own failed tests, silently reduce requirements, or
declare completion without evidence.

## 53.11 Foundation-0 scope

Foundation-0 implements the engineering discipline manually plus
lightweight machine enforcement before it attempts the full Project
Quality Manager product experience. Required artifacts include:

- `docs/engineering/ENGINEERING_STANDARD.md`;
- `docs/build-records/`;
- `docs/stage-gates/`;
- module contract READMEs;
- quality/debt baselines;
- CI scripts/workflows;
- traceability records for Foundation security promises;
- evidence links from the live Foundation-0 Stage Plan; the 90-Day Build Plan v1.6 remains historical.

This is intentionally dogfooding: Rikuri’s first managed project is
Rikuri itself.

# 54. Closing Architecture Principle

Rikuri Pi should be able to change rapidly **without making the user’s
intelligence fragile**.

The architecture therefore makes a deliberate trade:

- slow-changing core semantics;
- fast-changing replaceable AI edges;
- strict canonical-data ownership;
- measurable agent performance;
- hard autonomy boundaries;
- progressive user experience;
- structured extension points;
- recoverable state and migrations;
- evidence-first engineering and release discipline.

The intended result is a Personal Intelligence platform that a novice
can simply use and an expert can deeply control, while both users remain
free to benefit from new AI models, runtimes, tools, hardware, and
interaction patterns as they emerge.

> **People should not need to learn Rikuri in order to become
> productive. Rikuri should learn enough about their work, resources,
> preferences, and tools to help them create, discover, decide, execute,
> and verify.**

# Appendix A. Key Canonical Entities

| Entity                     | Purpose                                                       |
|----------------------------|---------------------------------------------------------------|
| User / Workspace           | Personal Intelligence owner and logical space                 |
| Device / Machine           | Physical or virtual execution endpoint                        |
| Project                    | Persistent goal/work container                                |
| Resource Location          | Where a project/artifact currently exists                     |
| Idea                       | Captured possibility not automatically treated as fact        |
| Knowledge Item             | Distilled durable concept/fact/reference with provenance      |
| Entity                     | Person, organization, concept, location, object, model, etc.  |
| Relation                   | Typed connection between stable objects                       |
| Decision                   | Chosen conclusion with rationale/time/status                  |
| Conversation / Message     | Source communication record                                   |
| Source Span                | Exact provenance pointer within source material               |
| Task / Job                 | Executable unit of work                                       |
| Workflow / Node            | Durable dependency graph and node state                       |
| Event                      | Append-only record of meaningful action/change                |
| Artifact                   | Produced or referenced file/output                            |
| Skill                      | Reusable domain procedure                                     |
| Model / Runtime / Provider | AI execution resources                                        |
| Execution Profile          | Exact routed model/runtime/tools/machine/settings combination |
| Performance Observation    | Measured result of an execution profile on a task             |
| Budget Envelope            | Enforceable money/time/call/agent limits                      |
| Security Decision          | Capability/privacy/approval decision                          |
| Extension Package          | Versioned third-party capability                              |
| Documentation Item         | Versioned help/developer source                               |
| Sync Operation             | Replicated state change                                       |
| Restore Point              | Recoverable version of Personal Intelligence state            |

# Appendix B. Representative Workspace Tool Contract

    {
      "tool": "workspace.open",
      "arguments": {
        "type": "terminal",
        "project_id": "project_...",
        "agent_id": "agent_...",
        "preferred_region": "bottom-left"
      }
    }

Result:

    {
      "window_id": "window_...",
      "workspace_id": "workspace_...",
      "monitor_id": "monitor_1",
      "bounds": {"x": 0.0, "y": 0.65, "w": 0.50, "h": 0.35}
    }

Coordinates are logical workspace fractions where possible, not raw
cross-device pixels.

# Appendix C. Representative Performance Observation

    observation_id: perf_...
    task_signature:
      domain: coding
      class: rust_bugfix
      complexity: medium
      language: en
    execution_profile_id: xp_...
    actual:
      success: true
      quality_score: 0.94
      wall_time_seconds: 318
      input_tokens: 12840
      output_tokens: 5120
      cached_tokens: 8200
      model_calls: 6
      tool_calls: 14
      retries: 1
      api_cost_usd: 0.37
      human_interventions: 0
    verification:
      compile: pass
      tests: pass
      regression: pass
    user_feedback:
      accepted_without_revision: true

# Appendix D. Representative Budget Envelope Inheritance

    Project Budget: $10
    │
    ├── Research branch: $2
    │   ├── Worker A: $1
    │   └── Worker B: $1
    ├── Implementation branch: $5
    │   └── Coding worker: up to $5
    ├── Evaluation: $1
    └── Reserve / recovery: $2

Unused child budget returns to the parent pool; children cannot exceed
allocations without an explicit budget reallocation event.

# Appendix E. Glossary

**Personal Intelligence:** durable user-owned state, knowledge, skills,
relationships, projects, preferences, and continuity managed by Rikuri.

**Rikuri Kernel:** stable core responsible for canonical identity/state,
events, governance, security contracts, budgets, workflows, backup
metadata, and versioning.

**Execution Profile:** concrete combination of model, runtime/harness,
provider, machine, tools, settings, context policy, security envelope,
budget, and verification.

**Capability Planner:** service that converts user intent into required
capabilities before routing.

**Agent Performance Model:** learned predictor of expected quality,
success, tokens, time, cost, and intervention for execution profiles and
escalation policies.

**Budget Governor:** hard enforcement and forecasting of autonomous
resource consumption.

**Autonomy Envelope:** combined authority, money, time, privacy,
attention, retry, and parallelism boundaries for delegated work.

**Context Compiler:** service that assembles task-relevant context under
privacy and provenance rules.

**Knowledge/Idea Fabric:** durable system for capture, distillation,
structured relationships, provenance, search, and resurfacing of user
knowledge and ideas.

**Workspace API:** structured interface through which humans and agents
create/manipulate Rikuri work surfaces.

**Dynamic Model Registry:** data-driven catalog of provider/local model
identities, lifecycle, capabilities, revisions, and discovery
provenance; current model names are never Kernel constants.

**Access Profile:** one concrete way to reach a model through a
runtime/harness/account/access path, with references to economics and
quota metadata.

**Price Card:** versioned, effective-dated charging metadata for an
access profile, with extensible billing dimensions.

**Subscription Quota Pool:** provider-reported or inferred capacity
window that may be shared by one or many model access profiles; unknown
quantities remain unknown.

**Personal Mesh:** local-first synchronization of selected Personal
Intelligence state among authorized user devices and optional hub.

**Proof of Work:** evidence that the requested outcome actually
occurred, such as tests, logs, source verification, screenshots,
benchmarks, or state queries.

**End of Rikuri Pi Architecture Specification v1.6**
