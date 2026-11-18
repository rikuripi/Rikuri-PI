# RIKURI PI

> Repository consistency revision: September 25, 2026. Source version retained; see [reconciliation record](reviews/2026-09-25-documentation-consistency.md).

**Creative Studio powered by Personal Intelligence**
**Product Requirements Document**  
Version 1.2 | September 25, 2026 | Collaborative Intelligence North Star

> *“Models are replaceable. Your Personal Intelligence is not.”*

A user may lose or replace a device without losing the accumulated knowledge, context, projects, skills, and continuity of their digital life.

| **Product**  | Rikuri Pi                                                            |
|--------------|----------------------------------------------------------------------|
| **Meaning**  | Pi = Personal Intelligence                                           |
| **Document** | Product Requirements Document v1.2                                   |
| **Status**   | Updated product baseline adding the Collaborative Intelligence North Star while preserving the Foundation-0 implementation boundary |

## Revision 1.2 focus

This revision keeps the original product vision while incorporating implementation decisions made after v1.0 and adds the long-term Collaborative Intelligence objective agreed September 25, 2026. The new objective does **not** expand Foundation-0 scope. It defines what the foundation must preserve so later Experience Intelligence, Inquiry & Discovery, and adaptive collaboration can be built without re-architecting the trusted core.

New long-term commitments in v1.2:

- **Personal Intelligence is more than memory.** Rikuri should eventually help the human–AI collaboration learn from experience, frame better questions, notice useful patterns, generate and test hypotheses, surface non-obvious connections, and improve working methods over time.
- **Instrument early; infer late.** Foundation stages capture stable execution identity, meaningful events, provenance, outcomes, verification, cost, human intervention, and feedback. They do not autonomously infer user traits or rewrite policy.
- **Economical Collaborative Intelligence.** Use the minimum sufficient compute and complexity for a verified outcome: frame before solving, attend before deep reasoning, use deterministic analysis where sufficient, and escalate selectively.
- **Optionality by design.** Learned methods are scoped, revisable evidence-backed defaults, not permanent truths. Rikuri preserves challenger methods and a bounded curiosity/exploration budget.
- **Quiet guidance.** Future collaboration should surface timely questions, patterns, connections, and alternatives without covert manipulation or constant interruption. Guidance must be explainable, reversible, and subordinate to user intent.
- **Models are operated, not merely ranked.** Future Model Lab learning considers model revision + runtime + harness + prompt/frame/context profile + sampling/reasoning settings + tools + verification + task context.
- **Discovery outputs are candidates.** Questions, Lightbulbs, hypotheses, correlations, abstractions, and conjectures retain provenance, uncertainty, counterevidence, scope, and lifecycle state before they can influence durable behavior or canonical knowledge.

- **Pi is the primary/default harness substrate for local/open models.** Rikuri builds model-family profiles, skills, security, verification, and project behavior around Pi. Qwen Code, DeepSeek Harness, and future open harnesses are reference implementations and benchmark comparators; a native harness is used only when measured evidence shows a material advantage.
- **Runtime compatibility and agent compatibility are separate.** llama.cpp/vLLM/Ollama/LM Studio answer “can this model run?” while Rikuri Harness Profiles answer “can Rikuri use it well?”
- **Privacy is an intent-level user choice.** Auto is default; Private and Local Only/Vault are persistent project/workspace policies translated by AISecurity into execution, egress, model, and sanitization rules.
- **Projects are data before they are executable.** A newly opened repository is untrusted until explicitly promoted. Read authority does not imply execute authority; project-controlled child processes inherit the same capability boundary.
- **Monaco is Rikuri's embedded editor, not VS Code.** Rikuri supplies its own developer platform around Monaco using LSP, DAP, Git, terminals, tests, search, formatters/linters, Docker/dev tools, and native plugins/windows. VS Code compatibility is selective rather than a product dependency.
- **Rikuri Link is the last mile.** Phone/tablet/laptop clients can securely “phone home” to user-owned Rikuri Nodes. Tailscale is the initial transport and later becomes a fallback when a native end-to-end encrypted Rikuri rendezvous/relay path is mature.
- **Ubuntu is the primary Linux reference platform.** macOS remains a continuously tested Tier-1 target; Windows follows once the OS capability abstraction is stable. Omarchy is a source of patterns, not a dependency.

## Foundation-0 planning baseline — September 24, 2026

The **Foundation-0 Re-baselined Stage Plan v2.1** is the live implementation and gate schedule: six dependent stages from October 2026 toward a June 2027 planning target, with four to eight weeks of contingency. A date cannot close an unfinished gate. This PRD states the larger product promise. **P0 means essential to its corresponding product release, not automatically due in Foundation-0.** The fifteen-item Foundation Release below is a later, broader product milestone; Foundation-0 is the narrower, single-node technical and owner-visible proof. ADR 0000 records the scope decision. No later-stage capability is silently treated as delivered at Foundation-0.

| Foundation-0 result | What is due in this slice | Preserved later product promise |
| --- | --- | --- |
| Imports and ideas | ChatGPT plus Markdown/Obsidian import; text Idea capture, provenance, FTS retrieval and project attach | Claude and read-only Git importers early Stage 1; voice capture, semantic retrieval and rich topic distillation after Foundation-0 |
| Model and privacy | One tested cloud access path; one Pi-backed local runtime; task handoff at Stage 04; Local Only policy refusal at Stage 02 and proven Ubuntu no-network envelope at Stage 03 | Additional providers/access modes and rich automatic routing require their own contract and security evidence |
| Knowledge | A minimal Knowledge Context Compiler returns current supported facts or explicit abstention across ten required conflict/supersession outcomes | Broader classification, entity resolution, multilingual knowledge and ranking grow from this verified contract |
| Recovery and security | Managed pre-forget and post-forget backups cannot silently restore forgotten content; old backup without current revocation state refuses or quarantines; untrusted project inspection and descendants remain bounded | Multi-device synchronization, external-copy erasure and native macOS enforcement are separate later claims |
| Platform and interface | Ubuntu is the only full execution/security assurance target; platform-neutral macOS core builds and tests continuously; one-window English Home plus pseudo-locale/CJK checks | Seven-language product localization, voice Home, rich workspace, native macOS parity and cross-lingual search remain later milestones |

Each stage produces a short owner-runnable demonstration and a plain-language Pass/Fail/Not yet proven receipt linked to deterministic tests. The owner judges usefulness and scope; technical and security claims require test evidence and independent challenge. Changed promises update this PRD, architecture, tests, and ADR together.

# Contents

1\. Executive Summary

2\. Product Vision and User Promise

3\. Problem Statement

4\. Goals and Non-Goals

5\. Product Principles

6\. Target Users and Core Use Cases

7\. Core User Experience and Design Language

8\. System Capability Model

9\. Functional Requirements

10\. Security and Trust Requirements

11\. Knowledge & Idea Fabric, Projects, and Search

12\. Model, Runtime, and Harness Strategy

13\. Local Model Intelligence

14\. Machine Intelligence and Execution

15\. Model Economics and Usage Transparency

16\. Personal Mesh, Sync, and Backup

17\. Portability and Deployment

18\. Observability, Health, and Recovery

19\. Data and State Model

20\. Domain Workspace Strategy

21\. Business and Multi-User Readiness

22\. Success Metrics

23\. Staged Roadmap

24\. Risks and Mitigations

25\. Open Technical Decisions

26\. Definition of Foundation Release

Appendix A. Priority Requirement Matrix

27. September 30 creative-studio requirements

28. October 1 Writer's IDE requirements refinement

29. October 8 mission charter requirements

Appendix B. Example User Journeys

Appendix C. Key Terms

# 1. Executive Summary

Rikuri Pi is a local-first, model-agnostic creative studio powered by a durable Personal Intelligence platform. Its initial audience is writers and storyworld creators, with game, animation and marketing workflows expanding incrementally. A creator can begin with a fragment, scene, question or image, explore alternatives with persistent AI collaborators, connect the pieces and develop a coherent work without choosing one AI ecosystem for its lifetime. Rikuri organizes and facilitates the creative process; its trusted infrastructure chooses admitted models, tools and execution paths within the creator’s authority.

The platform is designed around continuity rather than chat sessions. User knowledge, project state, decisions, skills, conversations, machine context, and execution history belong to Rikuri, not to any one model vendor. Frontier models, local models, specialized runtimes, tools, and computers are replaceable resources that Rikuri selects according to capability, cost, privacy, reliability, and task requirements.

General personal and executive assistance is a later capability-pack opportunity, rather than the initial product promise. Through those admitted packs, after the user connects the services they actually use, ordinary requests such as “check my email,” “find what matters,” “draft a reply,” “schedule a meeting,” “take notes on this lecture,” or “plan this trip” should work without requiring the user to create agents, configure MCP servers, edit prompts, understand model parameters, or build automation workflows.

Rikuri’s interface is designed around **a simple entrance and an expandable spatial workspace**. The default Home surface is a universal conversation/voice box with a small horizontal row of capability shortcuts such as Agent, Code, Research, Browser, Projects, and More. When work becomes complex, Rikuri can unfold into a workstation of freely movable, resizable, tileable, linkable, detachable windows containing agents, live documents, code, browser sessions, project graphs, diffs, tests, research, dashboards, and other work surfaces. The user may arrange these manually or tell Rikuri how to arrange them.

Rikuri also treats knowledge and ideas as a durable user-owned fabric. Text, voice notes, photos, videos, chats, documents, handwritten captures, research, code, decisions, and fragments of inspiration may enter through a low-friction Idea Inbox and later be linked into a searchable Knowledge Web and Idea Web. The user does not need to organize every thought at capture time; Rikuri preserves the original, adds provenance, connects it to relevant projects and concepts, and can resurface it when useful.

Rikuri is internationalized by design. The user interface, conversation layer, search, knowledge retrieval, model routing, and later voice surfaces must support multilingual use without fragmenting the user’s Personal Intelligence into language-specific silos. The initial localization set is English, Spanish, Simplified Chinese, Traditional Chinese (Taiwan), Japanese, French, and German, with additional languages addable through externalized locale resources rather than application rewrites.

For difficult work, Rikuri acts as a management institution rather than a single all-knowing agent. A coordinator can assemble specialist workers, a diverse strategy council can develop competing approaches, an independent evaluator can audit results and coordinator performance, and the human remains the principal who sets goals, values, constraints, and consequential decisions.

> **North-star experience**
>
> The user should not need to choose a model, manage an agent, understand Bash, remember where a project lives, organize every chat, or manually synchronize knowledge. Rikuri should understand the request, find the right information and resources, execute competently within its authority, verify the outcome, remember what mattered, and stay quiet when everything is healthy.

> **Ultimate Personal Intelligence goal**
>
> Rikuri should increase the effective intelligence of the human–AI collaboration over time. It should preserve meaningful experience, help the user recognize strengths and recurring weaknesses, learn which interventions and working methods actually help, cultivate useful habits and discipline, keep knowledge current and evidence-grounded, and use accumulated Personal Intelligence to produce better questions, decisions, products, creative work, experiments, solutions, and discoveries. The measure of success is not how often Rikuri speaks or how much compute it spends, but whether the person can do better work and reach useful possibilities they were less likely to reach alone.

> **Mission (adopted October 8, 2026)**
>
> Rikuri Pi exists to systematically expand and realize human imagination and creativity—by combining personal intelligence, accumulated knowledge, abstraction, synthesis, exploration, and coordinated action—while preserving human authorship, autonomy, and the freedom to pursue the unexpected. It frees people from being trapped by any one model, operating system or ecosystem: platform AI layers are resources Rikuri uses, never the home of the user's intelligence. The [Mission Charter](Rikuri_Pi_Mission_Charter_v1.0.md) gives the rationale and the decision test; principles P32–P34 and [§29](#29-october-8-mission-charter-requirements) hold the requirements.

# 2. Product Vision and User Promise

Rikuri Pi should feel like a creative studio whose work and intelligence belong to the creator. The creator can freewheel, explore several possibilities and return later to find the fragments, sources, choices and consequences connected. Its purpose is to preserve continuity across time, models, devices, operating systems, and projects while making powerful AI execution simple enough that the infrastructure largely disappears from view.

- One Personal Intelligence, many interchangeable AI brains.

- One coherent memory and project space across the owner’s chosen devices.

- One simple interface for questions, ideas, research, execution, review, and ongoing responsibilities.

- A user can replace a computer or AI provider without reconstructing their history or explaining prior work again.

- A blank installation contains the Rikuri foundation but none of another user’s personal state; a new user can build an independent Personal Intelligence from zero.

# 3. Problem Statement

| **Problem**                  | **Why it matters**                                                                                                                                                                        |
|------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Fragmented intelligence      | Useful knowledge is scattered across ChatGPT, Claude, local-model chats, Obsidian, code repositories, local disks, cloud storage, email, documents, and other applications.               |
| Chat as a poor archive       | Long conversations are stream-of-consciousness. Topics drift, useful ideas are buried, and users cannot remember which thread contains a decision or insight.                             |
| Vendor-bound state           | A model provider may remember a conversation inside its product, but that context does not naturally transfer to another provider, local model, machine, or runtime.                      |
| Model capability mismatch    | Different frontier models and their native harnesses excel at different tasks. A lowest-common-denominator API can discard valuable native capabilities.                                  |
| Local-model harness burden   | Local inference engines serve tokens but do not automatically supply robust orchestration, retrieval policy, validation, security, or task-specific behavior.                             |
| Opaque AI spending           | Users rarely know which model consumed what tokens, what a task cost, whether subscription quota was used efficiently, or whether a cheaper sibling could have produced the same outcome. |
| Unsafe or unusable execution | Raw shell access can be powerful but approval prompts are not meaningful when the user cannot evaluate the command. High-risk systems require bounded, deterministic authority.           |
| Broken sync mental model     | Traditional file-sync products can create conflicts, stale files, duplication, and dependence on a single cloud folder. Knowledge and state need local-first synchronization semantics.   |
| Device amnesia               | Frontier models troubleshooting a machine often must rediscover OS, hardware, services, containers, recent upgrades, and prior successful fixes.                                          |
| Agent-platform complexity    | Current agent systems often require users to understand connectors, MCP, skills, browser profiles, cron, prompts, permissions, and model plumbing before they receive basic assistant value.          |
| Attention overload            | Autonomous agents can create notification fatigue, unnecessary interruptions, or surprise actions unless attention and proactive authority are explicitly governed.                                  |
| Single-manager blind spots    | A single coordinator model may choose the wrong strategy, tools, or specialists for an unfamiliar or novel domain and then efficiently execute a flawed plan.                                       |
| Privacy/exfiltration risk     | Email, documents, photos, video, credentials, financial data, and security-sensitive files can leak through models, browsers, plugins, shell/network paths, or compromised tools without end-to-end controls. |
| Model discovery burden        | Open-model users must interpret model cards, quantizations, hardware fit, runtime settings, safety, and benchmarks themselves even when they only want a capability such as audio or video generation. |
| Agent-workspace rigidity      | Chat sidebars, IDE panels, terminals, and fixed split panes make parallel AI work hard to observe, compare, rearrange, and supervise; users need a spatial workbench rather than a stack of agent threads. |
| Idea loss and capture friction | Valuable ideas often arrive as fragments in voice notes, photos, chats, notebooks, sticky notes, or unrelated work and are lost when capture requires immediate organization. |
| Language fragmentation        | Users may think, write, search, speak, and collaborate in different languages; a Personal Intelligence must preserve meaning and continuity across languages without duplicating or corrupting canonical data. |
| Competing sources of truth    | Files, Markdown, SQL records, embeddings, graph indexes, chats, and application state can drift unless Rikuri explicitly declares which representation is authoritative for each object. |

# 4. Goals and Non-Goals

## 4.1 Product goals

The September 30 creative-studio direction prioritizes human authorship, reusable creative objects, connected spatial views and disciplined exploration. Coding, executive assistance and broader domain goals below remain supporting or later capability packs; they are not all first-release commitments.

- Model-agnostic continuity across frontier, open, and local models.

- Preserve and exploit each model/runtime’s native strengths rather than flattening them into one generic API.

- Convert chats and activity into durable, searchable knowledge while preserving original source history.

- Provide reliable access to projects, code, files, notes, and machine resources without manual zipping or repeated uploads.

- Enable safe execution through tools, code, browser, computer control, and OS commands with least authority and post-action verification.

- Continuously optimize quality, latency, privacy, quota, and spend using measured task outcomes.

- Work locally on macOS, Linux, and Windows with optional home-hub/server deployment and multi-device knowledge sharing.

- Make security, audit, health, backup, and recovery foundational rather than optional add-ons.

- Keep the visible interface simple, calm, and task-oriented.


- Provide a ready-to-use personal/executive assistant experience for email, calendar, contacts, notes, files, web tasks, learning, travel planning, and routine coordination after just-in-time account connection.

- Enable natural text and voice interaction in which the user manages outcomes while Rikuri manages the necessary agents, tools, models, and machines.

- Protect the user’s attention through explicit proactive-authority and interruption policies; autonomous work should be quiet by default.

- Support human-directed multi-agent work with coordinator, strategy-council, evaluator, and governance roles without requiring the user to manually supervise individual workers.

- Continuously evaluate models, tools, coordination strategies, and workflows from real task outcomes and use that evidence to improve routing and recommendations.

- Discover and safely acquire compatible open models for missing capabilities, subject to user approval, provenance/security screening, hardware fit, licensing, and task-specific evaluation.

- Enforce privacy classifications and outbound-data policy across models, browsers, tools, networks, and workers rather than relying on model obedience alone.

- Provide a universal spatial AI workspace in which agents, live artifacts, code, documents, browsers, project graphs, research, tests, and dashboards can be opened as independent windows, linked by context, rearranged freely, and controlled by text or voice.

- Make complex multi-agent work visually understandable through a live project/execution graph that can expand from objectives to tasks, agents, artifacts, dependencies, evaluations, and decisions.

- Preserve low-friction ideas and observations from text, voice, photos, video, chats, files, and handwritten captures, then make them searchable, linkable, reusable, and quietly resurfaced when relevant.

- Establish explicit data authority from day one: each object has a stable identity and an authoritative canonical representation even when multiple human-readable and machine-optimized projections exist.

- Support rich domain workspaces—beginning with coding, research, writing, and study—without creating separate silos of memory, projects, or user data.

- Internationalize Rikuri from the foundation so interface strings, locale formatting, multilingual search, model routing, and content handling can support English, Spanish, Simplified Chinese, Traditional Chinese (Taiwan), Japanese, French, and German initially, with additional languages added without architectural changes.

## 4.2 Non-goals for v0.x

- Build a new foundation model or inference engine.

- Replace Git, object storage, relational databases, or operating-system security primitives when proven components are adequate.

- Force all data into one physical storage system or require GitHub/cloud storage.

- Expose agent graphs, vector databases, MCP servers, token plumbing, or model settings to ordinary users by default.

- Automate financial transactions, credential export, seed/key access, or other high-consequence actions without explicit specialized policy and strong isolation.

- Guarantee that every vendor subscription exposes programmatic quota telemetry; Rikuri must label unavailable or estimated data honestly.


- Require ordinary users to edit the Rikuri Constitution, agent charters, prompts, routing policies, model parameters, or source code.

- Allow Rikuri to silently rewrite protected core components such as the Constitution, AISecurity, governance authority, secret boundaries, or production core code.

- Use agent count as a success metric; swarms are an execution technique only when task structure and measured economics justify them.

- Rebuild mature office, source-control, browser, or media engines solely to own the UI; Rikuri may integrate proven open/native tools behind a consistent workspace abstraction.

- Force every user into a complex workstation. The advanced spatial workspace must remain optional; ordinary users can accomplish supported work through the universal conversation/voice surface.

- Treat a vector database, graph database, Obsidian vault, or any one storage technology as the entire “brain.” Rikuri owns a coherent knowledge fabric with declared canonical sources and rebuildable derived views.

# 5. Product Principles

| **ID** | **Principle**                                            | **Meaning**                                                                                                                           |
|--------|----------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------|
| P1     | Models are replaceable; state is durable.                | Memory, projects, decisions, skills, event history, and canonical user data belong to Rikuri.                                         |
| P2     | Preserve native superpowers.                             | Choose model + runtime + tools, not merely model name. Use native harnesses when they materially improve capability.                  |
| P3     | Users express intent, not infrastructure.                | Rikuri chooses routing, context, tools, machines, retries, and escalation unless the user overrides.                                  |
| P4     | Quiet competence.                                        | Routine healthy operation should not interrupt the user; degradation, uncertainty, or consequential decisions should surface clearly. |
| P5     | Least expensive intelligence that reliably succeeds.     | Optimize cost per successful outcome, not token price alone.                                                                          |
| P6     | Prefer structured capability over raw control.           | Normalized tools/API/MCP/browser actions should precede GUI or raw shell where practical.                                             |
| P7     | AI chooses what; deterministic systems constrain how.    | High-risk execution should use validated tools/runbooks and hard policy boundaries.                                                   |
| P8     | Verify outcomes, not command completion.                 | A successful tool call is not proof the user’s goal was achieved.                                                                     |
| P9     | Canonical data is portable; derived data is rebuildable. | Indexes, embeddings, caches, and summaries must be disposable.                                                                        |
| P10    | Knowledge retains provenance and time.                   | Ideas, hypotheses, decisions, and superseded thinking must remain distinguishable and traceable to source.                            |
| P11    | Local-first, mesh-capable.                               | Devices remain useful offline; selected shared state converges when devices reconnect.                                                |
| P12    | Security is in the execution path.                       | No model, runtime, tool, or worker bypasses the same foundational security boundary merely because the vendor changes.                |
| P13    | User alignment over vendor alignment.                    | Rikuri represents the user’s goals, privacy, preferences, resources, and constraints rather than favoring a model, OS, cloud, advertiser, or app ecosystem. |
| P14    | Provider neutrality.                                   | Models, runtimes, hardware, clouds, and applications are interchangeable resources selected by measured user benefit.                  |
| P15    | Complexity is optional; competence is not.              | A simple one-frontier-plus-one-local setup should work well, while advanced users may expose richer multi-model and multi-agent capacity. |
| P16    | No surprise autonomy.                                  | Rikuri may pursue delegated goals but does not invent responsibilities, recurring jobs, spending authority, communications, or interruptions. |
| P17    | Attention is protected.                                | Agents request user attention through Rikuri policy; subagents do not independently interrupt, notify, call, or message the user.     |
| P18    | Evidence outranks agreement.                            | Multiple agents agreeing is not proof; reproducible evidence, verification, provenance, and independent evaluation outweigh majority voting. |
| P19    | Learn continuously; change core deliberately.            | Rikuri may adapt learned state and bounded profiles automatically, but protected core authority changes require controlled review and deployment. |
| P20    | Start with intent; reveal structure only when useful.         | The default experience is conversation/voice and a small capability launcher; graphs, agents, logs, terminals, and advanced controls appear only when they help. |
| P21    | Work, not agents, is the center of the interface.             | Users should see goals, artifacts, progress, decisions, and outcomes; agents are workers that may be exposed or hidden according to need. |
| P22    | Simple entrance; expandable workspace.                         | A beginner and an expert enter through the same Home surface; the workspace can grow from one conversation to a multi-monitor workstation without changing the underlying project. |
| P23    | Capture first; structure later; never lose the thought.        | Ideas and observations should be preserved with provenance even when they arrive incomplete or unclassified; organization may occur later. |
| P24    | One conceptual truth, appropriate physical stores.            | “Single source of truth” means each object has declared authority, not that every kind of data must live in one database. |
| P25    | Borrow proven patterns across disciplines.                    | When AI creates a difficult workflow problem, look beyond AI products for fields that already solved analogous problems; adapt principles rather than copying surface form. |
| P26    | Language is a presentation layer, not a data silo.             | Rikuri preserves original content and canonical identity across languages while allowing interface, conversation, search, translation, and voice to adapt to the user’s preferred locale. |
| P27    | Users choose privacy intent; Rikuri translates it into policy. | Auto/Private/Local Only are understandable choices; provider retention, network isolation, sanitization, and execution placement are technical consequences handled underneath. |
| P28    | The persistent agent belongs close to the user’s data. | Agents, credentials, files, memory, and durable workspaces should remain on user-controlled nodes when practical; cloud models are reasoning resources, not mandatory homes for the whole agent environment. |
| P29    | Read is not execute. | Opening, indexing, or discussing a project never implicitly authorizes project-controlled code, hooks, build scripts, or descendant processes to execute. |
| P30    | Reuse mature capability; own the user experience and trust boundary. | Rikuri integrates proven open tools behind stable contracts rather than cloning entire IDEs, operating systems, transfer stacks, or extension ecosystems. |
| P31    | Evolution for existing solutions; revolution for core strength. | Begin with working implementations and established patterns for scheduling, leases, retries, adapters, budgets, approvals, and audit. Invest original design in verified personal knowledge, adaptive harnessing, security, state integrity, and intent-led coordination. Record an evidence-based reason before replacing commodity machinery. |
| P32    | Capability over automation. | Success is what the person can understand, imagine, create and accomplish, not how much work AI performs. Infrastructure may disappear from view (P3); the person's own reasoning and craft should grow, and dependency is not a goal. See [Mission Charter §2, §8](Rikuri_Pi_Mission_Charter_v1.0.md). |
| P33    | Use platforms; belong to none. | Operating systems, hardware and vendor AI layers (OS-integrated model runtimes, NPU frameworks, built-in assistants) are resources Rikuri uses through adapters. None holds canonical state or becomes a permanent requirement; losing one reduces capability visibly, never continuity. |
| P34    | Platform claims at their proven level. | New or revised claims that a feature is available on a platform or processor architecture describe it as architecture-supported, implemented, verified, or unavailable/restricted. These states describe availability, not protection; security assurance keeps its own levels ([Architecture §26.5](Rikuri_Pi_Architecture_Spec_v1.7.md#265-platform-claim-states-architectures-and-vendor-ai-layers)). Compiling or launching is not verification. |

# 6. Target Users and Core Use Cases

## 6.1 Primary creator

A writer or storyworld creator who wants to develop ideas, characters, scenes and research across AI models without losing continuity, authorship or ownership. Game and animation creators extend this audience through later production-specification packs. The creator should not need AI infrastructure expertise.

## 6.2 Advanced/technical personal user

A user who operates local models, development environments, servers, GPU machines, Docker, home infrastructure, or specialist projects and wants Rikuri to bridge frontier intelligence with local execution and machine-specific context.

## 6.3 Business/team extension

A later deployment in which organizational knowledge, roles, approval boundaries, auditability, and shared projects use the same core architecture but with stronger identity, tenancy, and policy controls.

## 6.4 Personal/executive-assistant user

A user who wants an AI assistant immediately useful for communication, scheduling, meeting/lecture notes, travel, shopping, research, reminders, and coordination without becoming an administrator of agents or integrations. The user may know very little about AI infrastructure and should not need to understand it.

## 6.5 Creative and knowledge worker

A writer, researcher, marketer, product developer, student, analyst, creator, or other knowledge worker who generates ideas in many forms and needs them preserved, connected, searched, compared, edited, and developed into finished work without maintaining separate AI silos.

## 6.6 Representative use cases

- Brainstorm with any AI; later search by topic and recover relevant ideas, decisions, and source chats.

- Ask “What do I have related to KV cache?” and receive related notes, chats, projects, code references, experiments, papers, and open tasks across machines.

- Ask Rikuri to troubleshoot Linux or Docker; it supplies machine context, investigates live state, executes within policy, and verifies recovery.

- Use a local model for routine/private work; automatically escalate to a frontier model when confidence, capability, or verification requires it.

- Route research to a model/runtime with superior browsing or current-source capability, coding to the best coding harness, design work to a runtime with native creative-app integration, and combine outputs under one project state.

- Continuously see model tokens, cost, quota, equivalent API value, local inference cost, and recommendations for better price/performance.

- Share selected knowledge across three home computers while keeping machine-specific state, secrets, large models, and selected projects local.

- Replace a computer, reinstall Rikuri, attach the user’s Personal Mesh, and recover knowledge/project continuity without rebuilding the “brain.”


- Ask “Check my email and tell me what I need to deal with today”; Rikuri triages connected inboxes, surfaces important threads, drafts responses, and sends only within the user’s granted authority.

- Ask “Find a meeting time with Susan and schedule it”; Rikuri checks calendar/contacts, proposes or creates the event according to authority, and records the commitment.

- Ask “Take notes while I watch this lecture”; Rikuri uses captions/transcript or permitted local audio capture, creates structured course notes, explanations, questions, and future study material.

- Ask “Plan ten days in Barcelona”; Rikuri researches current options, uses known preferences, returns 2–3 scenarios when useful, and can continue into booking while escalating payment/2FA/commitment steps appropriately.

- Delegate a difficult project; Rikuri decides whether one agent, a specialist team, or a larger swarm is justified, coordinates workers, evaluates results, and only escalates material decisions to the human.

- Ask for a capability not currently installed, such as local video generation; Rikuri finds compatible open models, screens them, recommends candidates, downloads only after approval, tests them, and learns how well they perform over time.

- Open three independent model windows from one prompt, compare Claude/GPT/local-model answers side by side, and have Rikuri synthesize agreements, disagreements, evidence, and a recommendation.

- Watch a coding agent work through linked Agent, Live Code, Diff, Test, and Browser windows; resize or tile the workspace freely; detach a window to another monitor or macOS Space; interrupt or take control when useful.

- Capture a voice idea on a phone, a photo of a location, a handwritten note, or a fragment from a chat without filing it; later ask Rikuri to recover related ideas and connect them to a writing, research, marketing, or product project.

- Use a Writing Workspace with a live manuscript, editor agent, research, character/story canon, continuity checks, global semantic edits, and parallel section drafting while keeping the manuscript and source material user-owned and portable.

- Use Rikuri in Spanish, Japanese, German, French, Simplified Chinese, or Traditional Chinese (Taiwan); search English source material in another supported language, preserve the originals, and receive localized explanations or translations without creating a separate knowledge silo.

# 7. Core User Experience and Design Language

> **Primary interaction**
>
> The default interaction is a universal conversational/task surface: **“What would you like me to do?”** Rikuri should answer directly when a reply is sufficient and transparently execute when action is required. Infrastructure choices are hidden unless the user asks to inspect or override them. The same surface accepts text and voice and can create, manipulate, and navigate richer workspaces when the task warrants them.

## 7.1 Default navigation

| **Surface / area** | **Purpose** |
|---|---|
| Quick Assist | Optional compact popover/voice surface available without opening a full project workspace. |
| Home | Calm default entry: universal prompt/voice box, a small horizontal capability launcher, recent work, and only meaningful exceptions. |
| Projects | Persistent goals, state, code/files, related knowledge, tasks, decisions, activity, and responsible workers. |
| Knowledge & Ideas | Searchable notes, concepts, source chats, ideas, decisions, questions, references, visual inspiration, and relational graphs. |
| Tasks & Routines | Active, scheduled, triggered, waiting, and completed work. |
| Workspace | Expandable spatial workbench containing independently movable/resizable/linkable windows for live work. |
| Dashboards | Specialized glanceable views for ongoing systems/projects such as Bitcoin, machines, model usage, or agent swarms. |
| Resources | Optional advanced view of machines, models, connected services, storage locations, and project locations. |
| Settings | Sharing, privacy, security, model/provider accounts, budget policy, backups, and advanced controls. |

## 7.2 Calm-status widgets

- Security: Protected / Degraded / Critical, with last health check and exceptions only when meaningful.

- AI Usage: current model/runtime, task cost, today/month totals, subscription quota where available, and optimization advice.

- Personal Mesh: devices online/offline, common-knowledge sync state, and conflict count.

- Active work: current projects/jobs, status, waiting reason, and user decisions required.

## 7.3 Approval experience

Approval should be based on user-understandable intent and consequence, not raw implementation syntax. Advanced details may reveal exact commands and diffs, but the primary decision should describe what will change, risk level, affected scope, reversibility, and expected verification.

## 7.4 Out-of-the-box Personal & Executive Assistant

Rikuri shall ship with a built-in Assistant Capability Pack. Ordinary users must not need to create agents, configure MCP servers, write prompts, define cron schedules, or install workflow frameworks to receive baseline assistant value. Capabilities should activate through just-in-time account/service connection when first needed.

Baseline assistant domains include email triage and correspondence, calendar and scheduling, contacts, meeting preparation/follow-up, notes, files/documents, web research, travel planning, shopping assistance, reminders/routines, project coordination, and computer assistance. Meeting/lecture capture and transactional booking may mature in later phases but must use the same assistant surface and governance model.

## 7.5 Natural conversation and voice

The same Rikuri identity should support ordinary conversation, brainstorming, delegated work, follow-up questions, and voice interaction without forcing the user to switch “modes.” Voice may be exposed through desktop/mobile applications, headphones, messaging voice notes, or optional telephony. Rikuri-initiated phone calls are disabled by default and require explicit trigger authorization.

## 7.6 Just-in-time onboarding

Setup should ask only for capabilities relevant to the user. Rather than presenting a long integration marketplace, Rikuri asks in context—for example, “I need access to Gmail to do this. Connect Gmail?” Users who do not use Slack, Asana, Microsoft 365, or another service should not be required to understand or configure them.

## 7.7 Attention Firewall and proactive behavior

Rikuri is reactive by default. It may continue jobs already assigned and quietly maintain knowledge, indexes, sync, health checks, and other authorized responsibilities. New recurring responsibilities, outbound communications, spending, and user-initiated interruption channels require explicit authority. All workers route requests for user attention through a central Attention Firewall that considers urgency, relevance, duplication, quiet hours, current activity, assigned responsibility, and user preference.

Most proactive work should collapse into a digest or morning brief. A subagent should not directly notify, call, or message the user unless explicitly authorized through Rikuri governance.

## 7.8 Home and capability launcher

The default Home screen should remain intentionally sparse: a centered or prominent text/voice conversation box plus a small horizontal row of named capability icons. The initial row should favor a small number of obvious entry points such as **Agent, Code, Research, Browser, Projects, and More**. Additional shortcuts may be pinned later based on user choice or repeated use. The icons are shortcuts into Rikuri capabilities and workspace templates, not separate silos of state.

A user should be able to ignore the icons entirely and accomplish the same supported work through natural language. Conversely, common actions that can be done by clicking should also be expressible through text or voice where practical.

## 7.9 Rikuri Workspace: spatial workbench

When work benefits from multiple simultaneous views, Rikuri should expand into a spatial workspace inspired by mature workstation software rather than fixed chat/IDE sidebars. A **Window** is a first-class work surface and may contain an agent, conversation, document, code editor, diff, browser, terminal, project graph, research view, test output, dashboard, preview, evaluator, media, or future domain surface.

Windows may be freely created, moved, resized, minimized/collapsed, maximized, closed, tiled, detached into native operating-system windows, or placed on another monitor/desktop space where the platform permits it. Window controls may be hidden to maximize content area. A layout control should support fast equal grids such as 2×2, 3×2, 3×3, 4×4, plus asymmetric presets such as Focus + 3 or Compare + Result. Tiling is a convenience, not a permanent constraint; any tiled window can immediately be resized independently.

## 7.10 Links: context, agent, and prompt

Rikuri should generalize the linked-window concept used by professional trading workstations:

- **Context Link:** multiple windows follow the same underlying object, project, task, file, person, chapter, research question, stock, trip, or other entity. Changing the selected object in one linked window can update the others unless a window is locked.
- **Agent Link:** several windows are different views of one worker—for example Agent Chat, Live Code, Diff, Tests, and Browser all following the same coding agent.
- **Prompt Link:** one instruction is broadcast to several independent agents/models so their responses can be compared.

Links and physical layout are independent. A user can resize, move, or detach a window without changing its logical links.

## 7.11 Per-window model selection and Compare Mode

An Agent or compatible work window should expose a compact model selector, hidden when not needed. **Auto** remains the default; advanced users may pin GPT-, Claude-, Grok-, Gemini-, local-, or future model/runtime profiles that are compatible with the task and privacy policy. Switching the model assigned to a worker should preserve Rikuri-owned task/project state rather than restarting the work from scratch.

A **Compare Mode** should allow one prompt to be sent to two or more independent model/agent windows. Initial reasoning should remain isolated so one answer does not contaminate another; a separate comparison/evaluator surface may then summarize agreements, disagreements, evidence, weaknesses, and a recommendation. “Duplicate with another model” should be a fast path from any compatible window.

## 7.12 Project Graph and live execution map

The Project Graph is the visual big-picture control surface for complex work. It should be capable of showing objectives, subprojects, tasks, agents, artifacts, dependencies, evaluations, decisions, blocked states, and human-review points. Nodes may collapse into summaries such as “Research · 8 agents” and expand progressively to individual workers, evidence, or artifacts.

Clicking a node should focus an existing window, open an appropriate new window, or reveal details. Multi-select may open several nodes side by side. The graph and workspace are two synchronized representations of the same project: **graph for understanding, windows for working, links for coordinated context**.

## 7.13 Live Follow and observability depth

For active work, users should be able to choose among at least three observation levels:

- **Quiet:** work proceeds autonomously; surface only meaningful status and decisions.
- **Follow:** automatically show the file, document, browser page, test, or artifact currently receiving meaningful changes.
- **Deep:** expose detailed diffs, logs, tool calls, terminals, branches/worktrees, and low-level diagnostics for advanced users.

The interface should show meaningful changes as they occur when technically possible, but should not simulate distracting token-by-token typing merely for spectacle.

## 7.14 Saved workspaces, multi-monitor, and conversational layout control

Users may save named workspace layouts such as Coding, Research, Compare, Writing, Study, Review, or custom project-specific arrangements. Layout includes window type, size, position, link groups, and view state. Machine-specific geometry such as monitor placement or macOS Space should remain device-specific while the conceptual workspace can synchronize.

The workspace itself should be controllable conversationally: “Put Claude, GPT, and Qwen across the top and the comparison underneath,” “make the code window bigger,” “show only things that need me,” or “move research to my second monitor” should map to the same operations available through direct manipulation.

## 7.15 Interface freedom and native ecosystems

Users may choose to work primarily in Rikuri or continue using preferred native ecosystems such as ChatGPT/Codex, Claude/Cowork/Claude Code, vendor browsers, office suites, IDEs, or other applications. Rikuri should preserve the durable project/knowledge/state layer around that work when integrations permit it. The user owns canonical data and should not lose Personal Intelligence continuity merely because a preferred model or native UI changes.

## 7.16 Design language

Rikuri should feel calm, modern, and native rather than like an enterprise agent console. The design language should favor minimal chrome, clean spacing, compact pill/chip controls where appropriate, readable labels, context-sensitive icons, progressive disclosure, and the ability to hide controls for maximum working area. The exact visual system remains a design-specification decision, but the following product phrases are normative:

> **Simple entrance. Expandable workspace.**
>
> **Start with intent. Reveal structure only when structure helps.**
>
> **Work, not agents, is the center of the interface.**

## 7.17 Internationalization and language experience

Rikuri shall be internationalized (i18n) from the foundation and localized (l10n) initially for **English, Spanish, Simplified Chinese, Traditional Chinese (Taiwan), Japanese, French, and German**. User-facing strings must be externalized from application logic so additional languages can be added without redesigning the product.

The user’s **interface language**, **preferred assistant response language**, and **source-content language** are separate concepts. A user may run the interface in English, converse with an agent in Spanish, and work on Japanese or Chinese source material without altering the canonical originals. Per-user defaults should be available, with optional per-conversation or per-window overrides where useful.

Localization must include locale-aware dates, times, time zones, numbers, currencies, pluralization, sorting/collation, input methods, Unicode filenames/text, CJK line wrapping/font fallback, and enough layout flexibility for translated strings that expand or contract. The visual system should remain ready for future right-to-left language support even though RTL languages are outside the initial set.

Rikuri should support multilingual and cross-lingual knowledge retrieval. A query in one supported language may find relevant material written in another language when semantic confidence is sufficient. Machine-generated translations, summaries, or normalized multilingual representations are derived artifacts linked to the original source and must never silently overwrite canonical content.

# 8. System Capability Model

USER INTENT / CONVERSATION / VOICE  
\|  
v  
RIKURI PI — persistent user-owned intelligence  
\|-- Constitution / User Alignment  
\|-- Governance & Decision Rights  
\|-- Attention Firewall  
\|-- Context & Memory / Project State  
\|-- Localization & Language Service  
\|-- Capability Planner  
\|-- Coordinator / COO Role (dynamically selected)  
\|-- Strategy Council (when warranted)  
\|-- Independent Evaluation / Audit  
\|-- Model + Runtime Router  
\|-- Model Economics / Resource Optimizer  
\|-- Rikuri Security / Privacy Boundary  
\|  
+--\> Native Frontier Runtimes  
+--\> Generic Rikuri Harness --\> Local/Open Models  
+--\> Specialist Agents / Swarms  
+--\> Browser / Connector / MCP / API Tools  
+--\> Computer/GUI Worker  
+--\> Machine Tools / Shell (bounded)  
\|  
v  
VERIFY OUTCOME --\> EVALUATE WORK + COORDINATOR --\> UPDATE STATE / KNOWLEDGE / PROFILES / LEDGER --\> SYNC SELECTED STATE

> **Routing unit**
>
> Rikuri routes an execution profile, not just a model: model + reasoning level + runtime/harness + tools + machine + provider/payment path + security/privacy envelope + governance role + budget + verification.

## 8.1 Human–Rikuri management model

The human is the principal: responsible for goals, values, priorities, risk tolerance, and consequential judgment. Rikuri is the management institution. A coordinator model temporarily occupies the operational “COO” role and manages execution; it is replaceable and independently evaluated.

| **Layer** | **Primary responsibility** |
|---|---|
| Human Principal | Goals, values, priorities, risk tolerance, taste, consequential final decisions |
| Rikuri Governance | Decision rights, budgets, privacy/security, attention, escalation, authority |
| Coordinator / COO | Plans, staffing, model/tool/machine selection, execution management, recovery |
| Strategy Council | Competing approaches for hard, novel, unfamiliar, or high-impact problems |
| Specialist workforce | Research, coding, analysis, domain work, browsing, local execution |
| Independent Evaluator | Outcome quality, evidence, cost, missed alternatives, coordinator performance |

## 8.2 Strategy Council and scenario analysis

Rikuri should not rely on one coordinator’s know-how when a problem is novel, unfamiliar, high-uncertainty, high-impact, or materially contested. It may solicit diverse independent proposals from different models/skills, gather evidence, criticize alternatives, and adjudicate. Majority vote is not sufficient; evidence and verification outrank agreement.

When human judgment is useful, Rikuri should preferentially return 2–3 decision-ready scenarios with tradeoffs and a recommendation—for example Fastest/Lowest Cost, Strongest Expected Result, and Best Balance—rather than forcing a non-expert user to make a low-level technical choice.

## 8.3 Constitution, governance, charters, and task contracts

Rikuri should separate durable principles from transient work instructions:

- **Rikuri Constitution:** small, durable, model-independent principles such as user alignment, privacy, evidence, user control, and provider neutrality.
- **Governance Policy:** authority, spending, privacy/security, attention, escalation, and human decision rights.
- **Agent Charter:** role, mission, responsibilities, capabilities, and boundaries for coordinator, researcher, coder, reviewer, etc.
- **Task Contract:** objective, deliverables, constraints, budget, deadline, tools, success tests, and escalation conditions for the current assignment.
- **AISecurity/Rikuri Security:** hard enforcement of what an actor can actually access or do.

Constitutional guidance is not a substitute for deterministic enforcement.

## 8.4 Evaluation and continuous improvement

The coordinator must not be its own sole judge. Rikuri combines objective telemetry, deterministic verification, independent AI evaluation where appropriate, and human usefulness feedback. Coordinator performance should be measured over time by goal success, cost, latency, unnecessary/missed escalations, recovery quality, tool/model choice, and user satisfaction.

Rikuri may automatically adapt low-risk learned state such as routing preferences, model profiles, tool preferences, successful remedies, and bounded harness settings. It may propose new tools, skills, methods, or architecture changes. Protected core components—Constitution, AISecurity, governance authority, secret boundaries, update authority, and production core code—require controlled developer review, tests, versioning, and deployment.

# 9. Functional Requirements

## 9.1 Rikuri Core / Control Plane

| **ID**   | **Pri** | **Requirement**             | **Description**                                                                                                                       | **Acceptance**                                                                                      |
|----------|---------|-----------------------------|---------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------|
| CORE-001 | P0      | Persistent project state    | Maintain explicit current state, goals, open tasks, decisions, artifacts, responsible workers, and recent activity for every project. | A new model/runtime can continue a project without the user restating information already recorded. |
| CORE-002 | P0      | Append-only event ledger    | Record meaningful actions, decisions, state changes, model/runtime used, artifacts, verification outcome, and cost metadata.          | State can be reconstructed/audited after restart, migration, or worker failure.                     |
| CORE-003 | P0      | Capability planner          | Classify task needs across reasoning, tools, modality, privacy, latency, cost, context, machine, and consequence.                     | Planner generates a structured execution profile before non-trivial delegated work.                 |
| CORE-004 | P0      | Durable jobs                | Long-running work survives UI closure and recoverable process/device failures when supported by deployment mode.                      | Interrupted work resumes from a checkpoint rather than silently restarting or disappearing.         |
| CORE-005 | P1      | Persistent responsibilities | Support named responsibilities/routines (e.g., Systems, Research, Bitcoin) that retain context independent of individual chats.       | User can ask status of a responsibility without finding a prior thread.                             |
| CORE-006 | P0 | Transactional State Evolution | For every authoritative migration or transformation: preserve/read old state, validate inputs, transform candidate, validate full candidate, durably write, read back and verify, atomically promote, then retire/archive old state. | Injected failure at any pre-commit step leaves the last known-good state intact and retryable. |
| CORE-007 | P0 | No silent loss or false-green status | Cross-component handoffs use IDs and acknowledgements; missing, stale, or truncated state yields a typed error and visible health signal. Completion follows end-to-end verification of the durable outcome. | Missing ACK, dead caller, stale tail, or unconnected scaffold cannot appear as a successful feature. |

## 9.2 Context and Memory Engine

| **ID**  | **Pri** | **Requirement**    | **Description**                                                                                                                        | **Acceptance**                                                                            |
|---------|---------|--------------------|----------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------|
| MEM-001 | P0      | Context compiler   | Build task-specific context packages from user/project/machine state, relevant history, knowledge, permissions, and tool availability. | Models receive relevant context without loading entire chat history or vault.             |
| MEM-002 | P0      | Memory classes     | Distinguish working, project, episodic, semantic, preference, procedural, organizational, and decision memory.                         | Different memory classes can have independent retention, sharing, and retrieval policies. |
| MEM-003 | P0      | Provenance         | Every distilled knowledge item must retain source references to original chats/files/events where available.                           | Rikuri can answer “where did this come from?” and navigate to source.                     |
| MEM-004 | P0      | Temporal state     | Represent current, superseded, tentative, and historical conclusions without overwriting the past.                                     | Later decisions can supersede earlier ones while preserving the thought trail.            |
| MEM-005 | P1      | Learned procedures | Convert repeated successful workflows into draft reusable skills subject to policy/testing.                                            | A proven fix can become a reusable procedure rather than requiring rediscovery.           |
| MEM-006 | P0 | Knowledge Context Compiler | Treat retrieved memory, summaries, embeddings, documents, and agent output as evidence candidates; check source authority, observed/valid time, supersession, freshness, relevance, conflicts, confidence, and privacy before use, with risk-based revalidation or abstention. | A stale contradicted memory cannot be silently presented as current fact or control an authorized action. |
| MEM-007 | P0 | Knowledge discovery on arrival | On every new model/session, model or harness switch, resume, and compaction, the runtime must supply trusted knowledge-use instructions, an authorized capability manifest, and actual callable connectors, scoped Knowledge Service tools, or a tested host-mediated equivalent. Keep those instructions distinct from retrieved evidence. | The destination receives the available context and retrieval route without a user reminder; missing wiring, failed delivery, or unsupported capability is visibly unavailable/degraded and cannot silently report Ready. |
| MEM-008 | P0 | Retrieval at topic boundaries | Run a context preflight each turn. Explicit topic/project changes or topic crossings trigger immediate, bounded retrieval before a context-dependent response or action; semantic detection may advise additional searches but cannot replace deterministic triggers or grant access. Preserve the original objective and pending work. | A changed or intersecting topic searches permitted existing knowledge even when the model never asks for memory. The result or a visible coverage/failure state precedes dependent work; unrelated scopes remain inaccessible. |
| MEM-009 | P0 | Versioned task packet and handoff | Deliver a compact, versioned packet containing the objective, constraints, accepted decisions, source references, verified results, pending work, freshness, coverage, conflicts, and retrieval routes. Preserve links to originals when summarizing. | A replacement model can locate supporting evidence and continue unfinished work; stale, truncated, or unacknowledged packets cannot count as a successful handoff. |
| MEM-010 | P0 | Source freshness and invalidation | Incrementally capture authorized source changes with source version, capture time, last successful update, and a declared freshness expectation. Distinguish no match, not searched, unavailable, stale, partial access, and revoked access. Deletion or permission changes invalidate affected indexes, packets, caches, and pending results before reuse. | Freshness-sensitive decisions revalidate against the authoritative source or disclose the limitation and defer unsupported dependent work; an inaccessible or outdated source cannot masquerade as a complete empty search. |
| MEM-011 | P0 | Cross-media and coding provenance | Keep permitted originals and addressable evidence: source spans, transcript timestamps, image/frame references, and video time ranges as applicable. Preserve coding-session decisions, attempts, verification, and pending work alongside repository identity, revision, and current checkout evidence. | An agent can inspect the relevant source segment; a transcript alone cannot prove visual content, and a coding-session claim cannot prove the current repository implements or passes it. Merely viewing or downloading media does not establish authorized ingestion. |
| MEM-012 | P0 | Context delivery evidence | Record runtime-observed trigger, retrieval, compilation, delivery, and verification outcomes with stable IDs, profile/version identity, elapsed time, coverage, and explicit failures. Evaluate required-evidence recall, freshness, contradictions, task usefulness, and time/cost using fixed cases and observed outcomes. | Measured receipts show whether useful context reached the destination in time. Connector presence, model acknowledgement, or model self-assessment alone cannot prove retrieval completeness, delivery, or usefulness. |

The detailed lifecycle, packet, freshness, provenance, and evaluation contracts are in [Architecture §19.5–19.10](Rikuri_Pi_Architecture_Spec_v1.7.md#195-context-delivery-lifecycle). These requirements describe intended behavior, not implemented capability. The existing Foundation-0 subset uses authorized ChatGPT/Markdown imports and lexical retrieval (S01.03/S01.07/S01.09), the versioned policy-aware context package (S02.02), and Pi lifecycle, scoped knowledge tools, and cloud-to-local handoff (S04.01/S04.02/S04.14/S04.15). Within that subset, explicit lifecycle/topic triggers and visible failure refine the existing context and handoff contracts. Richer topic/semantic discovery and Claude/Git sources remain product Stage 1; email/calendar/service connectors remain Stage 5; multi-agent integration remains Stage 6; richer media capture remains Stage 8. All remain subject to their predecessor gates in the live Stage Plan and Master Build Sequence; these P0 requirements do not expand Foundation-0 scope or close any gate.

## 9.3 Conversation and Knowledge Distillation

| **ID**   | **Pri** | **Requirement**              | **Description**                                                                                                          | **Acceptance**                                                                                                        |
|----------|---------|------------------------------|--------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------|
| KNOW-001 | P0      | Conversation archive         | Preserve raw imported/native conversations as immutable or versioned source records.                                     | Distillation never destroys the original conversation.                                                                |
| KNOW-002 | P0      | Topic segmentation           | Split stream-of-consciousness conversations into coherent topics independent of thread boundaries.                       | One chat may generate/update multiple topic notes; a topic may span multiple chats.                                   |
| KNOW-003 | P0      | Statement classification     | Classify extracted content as fact, idea, hypothesis, question, decision, preference, task, result, or reference.        | Brainstorming is not silently promoted to an accepted decision.                                                       |
| KNOW-004 | P0      | Entity resolution and dedupe | Resolve topics/entities against existing knowledge before creating new notes.                                            | Duplicate notes such as “FlashAttention”, “FA ideas”, and “Flash Attention notes” are avoided or merged deliberately. |
| KNOW-005 | P0 | Obsidian projection | Generate browsable Markdown with properties and links from Rikuri records; import editable user-authored vault notes separately as versioned sources. | Two one-way flows; edited generated files are preserved and flagged before safe regeneration, never automatically merged into canonical state. |
| KNOW-006 | P1      | Semantic knowledge graph     | Maintain typed relationships such as USES, DEPENDS_ON, RELATED_TO, SUPERSEDES, DERIVED_FROM, DECIDED_IN, and APPLIES_TO. | Rikuri can answer relationship questions beyond plain text similarity.                                                |
| KNOW-007 | P1      | Thought trail                | Show how an idea/project evolved over time across chats, experiments, and decisions.                                     | Topic view can present chronological evolution with source links.                                                     |
| KNOW-008 | P1 | Capture without prior organization | Offer a common Save/share action for admitted source types with source identity, version, project/conversation scope and visible capture/extraction/search status. | A saved original remains recoverable when extraction fails; no false saved/searchable status or capture merely because content was viewed. |
| KNOW-009 | P1 | Correctable derived classification | Produce versioned descriptive tags, entities, topics and media descriptions with provenance and uncertainty; preserve user corrections separately. | Metadata can be applied automatically and corrected; classification cannot accept decisions, execute tools, expand sharing or hide originals from baseline retrieval. |
| KNOW-010 | P1 | Conversational evidence contributions | Surface cited passages, images or files through bounded retrieval, source cards and attention controls; support request/mention, quiet suggestions and explicitly enabled automatic contributions. | Relevant evidence arrives with its source and reason; stale, duplicate, unhelpful or audience-ineligible material is suppressed; supported media and coverage limits are explicit. |
| KNOW-011 | P0 | Recoverable document revisions | Preserve captured source versions and Rikuri-authored revision ancestry; durably preserve edited projections before regeneration with base references and verified recovery content. | Failed preservation or racing edits prevent overwrite. A later reviewed proposal can create a canonical revision through governed services; history/recovery copies obey privacy, retention and forget policy. |

September 28, 2026 clarification: [Architecture §20.3.1](Rikuri_Pi_Architecture_Spec_v1.7.md#2031-revision-history-and-edited-projections) defines history and projection recovery; [§20.6](Rikuri_Pi_Architecture_Spec_v1.7.md#206-capture-classification-and-conversational-evidence) defines capture/contributions. Foundation S01.03/S01.05/S01.08 covers imported versions and safe projection preservation/flags only. Rich review UI follows product Stage 2; text classification/semantic experiments follow Stage 1 evaluation prerequisites; connectors and personal contributions Stage 5; shared conversations Stage 6; richer media Stage 8. History covers observed/captured versions, not every external keystroke or uncaptured past. All behavior is Not yet proven.

## 9.4 Projects, Resources, and Universal Search

| **ID**   | **Pri** | **Requirement**        | **Description**                                                                                                        | **Acceptance**                                                                                                      |
|----------|---------|------------------------|------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------|
| PROJ-001 | P0      | Project registry       | Track projects and all known locations: local machines, Git remotes, NAS, cloud/object storage, and related knowledge. | Search finds a project even when its source is not present on the current machine.                                  |
| PROJ-002 | P0      | Live resource resolver | Resolve indexed references to current authoritative files/repositories before execution.                               | Agents do not modify stale index copies when the live resource is available.                                        |
| PROJ-003 | P0      | Universal search       | Combine semantic, lexical, metadata, symbol/code, Git-history, graph, and recency signals.                             | A query such as “KV cache” returns categorized knowledge, projects, chats, code, experiments, decisions, and tasks. |
| PROJ-004 | P1      | Code-aware indexing    | Index repositories by files, symbols, definitions/references, branches/commits, and relevant history.                  | User can ask where a concept was implemented or changed and receive code-level results.                             |
| PROJ-005 | P1      | Resource catalog       | Provide a human-readable inventory of projects/resources and where they physically live.                               | User can ask “What AI projects do I have?” without remembering machine/path.                                        |

## 9.5 Model and Runtime Routing

| **ID**    | **Pri** | **Requirement**                    | **Description** | **Acceptance** |
|-----------|---------|------------------------------------|-----------------|----------------|
| ROUTE-001 | P0 | Model-independent state | No model provider’s private conversation state may be required for Rikuri continuity. | Changing model/runtime preserves task/project continuity. |
| ROUTE-002 | P0 | Pi-primary governed harness | Pi is the default harness substrate for local/open models and provider-neutral governed execution; Rikuri owns policy, context, skills, verification, profiles, evidence, and UI around it. | A supported local model can perform a coding/tool task through Pi without a Qwen/DeepSeek-specific full harness dependency. |
| ROUTE-003 | P0 | Native-harness exception path | Native/open/vendor harnesses may be routed as replaceable adapters only when capability or measured evaluation materially justifies them. | Switching to a native harness does not transfer canonical state/security authority to that harness. |
| ROUTE-004 | P0 | Access-path routing | Support local inference, direct API, broker API, officially supported native/subscription runtime, and remote-node execution where permitted. | User can use the best permitted path rather than API-only access. |
| ROUTE-005 | P1 | Automatic escalation/de-escalation | Start at appropriate capability and escalate/downshift based on verification, failure, privacy, cost, hardware, and evidence. | Routine work avoids unnecessary cost; difficult work can escalate. |
| ROUTE-006 | P0 | Simple privacy/execution modes | Expose Auto, Private, and Local Only/Vault as persistent project/workspace intent; advanced users may inspect/override exact model/runtime/provider. | A nontechnical user can obtain enforceable privacy behavior without knowing ZDR, inference runtimes, or network policy. |
| ROUTE-007 | P0 | Harness compatibility registry | Track model-family/profile compatibility separately from runtime support, including tool-call, context, planning, editing, reasoning, and subagent behavior. | New models normally reuse a family profile or a small override; harness core changes are exceptional. |

### Harness intent and evidence refinement — October 2, 2026

These product requirements refine Architecture §11.1–11.3; priority is not Foundation scope. Acceptance: Test Strategy §19. All runtime capability is **Not yet proven**.

| ID | Priority | Requirement | Acceptance |
| --- | --- | --- | --- |
| HAR-001 | P1 | Preserve versioned intent, constraints, authority, evidence needs and success criteria across adapters/handoffs; clarify material ambiguity and retain answers/assumptions | Consequential alternatives trigger focused questions; independent work continues; silence grants no authority; factual uncertainty uses available inspection |
| HAR-002 | P1 | Route factual work according to declared task evidence requirements to authorized measurements, scoped RAG or both; constrained-model profiles may require lookup for every factual answer | Missing/stale/wrong-target/irrelevant evidence cannot produce verified facts; legitimate supported answers succeed; creative assumptions remain explicit |
| HAR-003 | P1 | Bound execution/verification/repair transitions with cancellation, visible blockers and task-specific completion evidence | Retry exhaustion, revocation and failed verification cannot appear completed or expand rights; partial work remains inspectable |
| HAR-004 | P1 | Compile admitted intent using versioned adapters and promote evaluated profile candidates under existing review/rollback policy | Constraints survive model switches; candidate profiles cannot promote themselves or change policy; regressions preserve known-good versions |

## 9.6 Local Model Lab and Dynamic Harness

| **ID** | **Pri** | **Requirement** | **Description** | **Acceptance** |
|---|---|---|---|---|
| LOCAL-001 | P0 | Model/runtime discovery | Detect models available through supported runtimes and inspect metadata/templates/capabilities. | Known-family models onboard without Kernel changes. |
| LOCAL-002 | P0 | Composable Harness Profile | Maintain family/model-size/task/runtime/security profile components rather than one monolithic bespoke harness per model. | A Qwen/DeepSeek-family update can be represented primarily as profile data/skills/extensions. |
| LOCAL-003 | P0 | Pi-based coding behavior | Provide repository exploration, planning, context management, edit/diff strategy, compile/test/debug loop, Git awareness, verification, and bounded subagent behavior through Pi/Rikuri. | Local coding model completes a representative edit-test-verify task through Pi. |
| LOCAL-004 | P1 | Harness-source analysis | Inspect open native harnesses (e.g. Qwen Code/DeepSeek Harness) for model-specific prompts, tool schemas, context/compaction strategies, and useful agent techniques. | Useful upstream behavior can be tested and adopted without making the upstream harness a dependency. |
| LOCAL-005 | P1 | Evaluation suite | Benchmark model + runtime + harness-profile combinations on task-specific suites and real work. | Native harness exceptions and profile changes are justified by recorded evidence. |
| LOCAL-006 | P1 | Adaptive profiles | Update bounded recommendations from verified success, retries, latency, resource use, and user feedback. | Profile evolution is versioned, reversible, and does not self-expand security authority. |
| LOCAL-007 | P1 | Constrained output | Use grammar/schema-constrained generation when supported for tools and structured data. | Structural failure rate is measured and minimized. |

## 9.7 Machine Intelligence and Execution

| **ID** | **Pri** | **Requirement** | **Description** | **Acceptance** |
|---|---|---|---|---|
| MACH-001 | P0 | Machine profile | Maintain hardware, OS/kernel, drivers, installed software, services, containers, AI runtimes, network, resources, and permitted locations for each device. | Agents do not repeatedly ask for discoverable machine facts. |
| MACH-002 | P0 | Live state retrieval | Provide structured tools to query services, processes, logs, packages, hardware, Docker, network, storage, and health. | Agent investigates current state before change. |
| MACH-003 | P0 | Observe-diagnose-act-verify loop | Execution tasks define intended outcome and independently verify actual state. | Exit code 0 alone is never proof. |
| MACH-004 | P0 | OS Capability API | Expose normalized capabilities such as package/service/process/network/power/filesystem/sandbox/window operations through OS adapters. | The same Rikuri intent maps to Ubuntu, macOS, or Windows-specific implementation without model prompt forks. |
| MACH-005 | P0 | Ubuntu reference adapter | Ubuntu is the first full Linux implementation and security reference; macOS is continuously tested and Windows follows after abstraction stability. | Foundation security/exec acceptance is proven on Ubuntu and core tests stay green on macOS. |
| MACH-006 | P1 | Incident/remedy memory | Record symptoms, diagnosis, attempted actions, successful/failed remedies, environment, and verification. | Recurrent machine problems can reuse proven remedies. |
| MACH-007 | P1 | Multi-machine routing | Choose an available node based on hardware/software/resources/project location/policy. | Tasks can route to the best permitted owner-controlled machine. |

## 9.8 Tools, Skills, and Automation

| **ID**   | **Pri** | **Requirement**          | **Description**                                                                                                                               | **Acceptance**                                                                            |
|----------|---------|--------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------|
| TOOL-001 | P0      | Tool hierarchy           | Prefer native connector/API/MCP or structured Rikuri tool, then browser, then GUI/computer control, then raw shell when needed.               | Planner records why a less structured execution method was required.                      |
| TOOL-002 | P0      | Skill packages           | Separate reusable domain procedure from model/harness; skills may define tools, retrieval rules, verification, output style, and risk policy. | Bitcoin, Linux, research, coding, etc. can share generic harness infrastructure.          |
| TOOL-003 | P1      | Teach by demonstration   | Allow user-demonstrated workflows to be captured as draft skills/routines with review/testing.                                                | Repeated manual workflows can become reusable automation.                                 |
| TOOL-004 | P1      | Scheduled/event routines | Support time/event-triggered responsibilities with persistent state.                                                                          | Rikuri can execute ongoing duties without requiring a chat to remain open.                |
| TOOL-005 | P1      | Worker coordination      | Allow specialist workers to collaborate under a coordinator without requiring the user to manually orchestrate agents.                        | Multi-agent behavior is hidden behind project/task status unless inspection is requested. |

## 9.9 Personal & Executive Assistant

| **ID** | **Pri** | **Requirement** | **Description** | **Acceptance** |
|---|---|---|---|---|
| ASST-001 | P0 | Ready assistant | Provide useful assistant behavior after connecting services the user actually uses; no agent/workflow engineering required. | User can ask a common assistant request without creating agents, prompts, cron jobs, or MCP configuration. |
| ASST-002 | P0 | Email triage and drafting | Read/classify authorized inboxes, identify important messages, summarize threads, and prepare replies. Sending obeys explicit authority. | “Check my email and tell me what matters” yields prioritized results and draftable follow-up actions. |
| ASST-003 | P0 | Calendar/contacts coordination | Read calendars/contacts, check availability, create or propose meetings, and retain commitments. | “Find a meeting time with X” can be completed through natural language within permission policy. |
| ASST-004 | P1 | Meeting and lecture intelligence | Capture permitted transcript/audio, segment topics, extract notes/decisions/questions, summarize, explain, and create study material. | User can turn a supported meeting/course session into useful notes and follow-up learning artifacts. |
| ASST-005 | P1 | Travel and shopping assistance | Research, compare, plan, and prepare bookings/purchases; consequential commitment/payment/2FA follows governance. | Rikuri can carry a task from intent through decision-ready options and to the permitted transaction boundary. |
| ASST-006 | P1 | Natural voice surface | Support real-time conversational voice and voice instructions over supported devices/interfaces. | User can move between conversation and delegated work without a separate agent mode. |

## 9.10 Attention and Proactive Authority

| **ID** | **Pri** | **Requirement** | **Description** | **Acceptance** |
|---|---|---|---|---|
| ATTN-001 | P0 | Attention Firewall | Route all worker requests to interrupt/notify the user through a centralized attention policy. | Subagents cannot independently call, notify, or message the user outside granted policy. |
| ATTN-002 | P0 | Reactive by default | Fresh installs may respond, continue assigned work, and perform quiet maintenance but may not invent new responsibilities. | No surprise recurring job, outbound communication, spending, or phone call occurs without delegated authority. |
| ATTN-003 | P0 | Digest aggregation | Collapse routine proactive findings into concise digests unless urgency/policy warrants interruption. | Multiple worker completions do not become multiple user notifications. |
| ATTN-004 | P1 | Learned attention preference | Adapt interruption/digest recommendations from user behavior without silently expanding authority. | Rikuri can reduce nuisance alerts while preserving explicitly critical triggers. |

## 9.11 Governance, Multi-Agent Teams, and Strategy Council

| **ID** | **Pri** | **Requirement** | **Description** | **Acceptance** |
|---|---|---|---|---|
| GOV-001 | P0 | Decision-rights policy | Define what the human, coordinator, specialists, evaluator, and deterministic systems may decide or execute. | A task can state budget/authority/escalation without embedding vendor-specific prompts. |
| GOV-002 | P0 | Replaceable coordinator | Coordinator/COO is a role filled by a routed model/runtime, not permanent Rikuri identity. | Rikuri can switch coordinator while preserving project state and governance. |
| GOV-003 | P1 | Automatic team sizing | Use one agent by default and scale to specialist teams/swarms only when task structure, quality, latency, or economics justify it. | Agent count is chosen by measured need rather than a fixed workflow. |
| GOV-004 | P1 | Strategy Council | For hard/novel/high-impact work, solicit diverse independent strategies and critique/evidence before committing execution. | Council output records alternatives, supporting evidence, risks, and recommendation. |
| GOV-005 | P1 | Scenario decision packet | When human judgment is needed, present 2–3 understandable options with tradeoffs, recommendation, confidence, reversibility, and required decision. | Non-expert user can make an executive-level choice without choosing low-level technology. |
| GOV-006 | P1 | Single user communication path | Specialist workers report to coordinator/governance rather than directly to the user by default. | User sees consolidated status/decisions rather than agent chatter. |
| GOV-007 | P1 | Independent coordinator evaluation | Evaluate management quality separately from execution. | Coordinator scorecard includes task success, cost, escalation quality, recovery, and user usefulness. |
| GOV-008 | P1 | Durable multi-agent control plane | Maintain stable worker and task identities, goal ancestry, atomic ownership/lease, event-driven wakeups, checkpoints, run records, approval and budget gates, and explicit pause/resume/cancel. Keep these behind Rikuri-owned contracts and authoritative ProjectState. | Two workers competing for one task cannot both commit; restart, retry, handoff, and cancellation do not silently lose or duplicate work. |
| GOV-009 | P1 | Intent-led team assembly | Propose the smallest useful team from the task and project lifecycle, showing roles, access, estimated spend, duration, review points, and material risks before activation. A single worker is a valid team. Hide runtime/adapter settings under advanced controls. | A user can ask for research, implementation, and independent review without manually hiring agents or configuring an org chart. |
| GOV-010 | P1 | Lifecycle-specific coordination | Use Frame, Shape, Plan, Execute, Verify, Deliver, Learn as the common Project Lifecycle Kernel over one Project ID and authoritative ProjectState; domain profiles specialize artifacts and gates. | Coding, research, and writing teams share state and transitions without being forced into a company hierarchy. |
| GOV-011 | P1 | Reference implementation evaluation | Study Paperclip's heartbeat, atomic checkout, adapter, session, team template, budget, approval, and run history mechanisms in a pinned test installation. Adopt a mechanism only after license/security review and Rikuri-specific failure, recovery, and usability tests. | A pattern decision records source version, observed behavior, integration cost, failure modes, acceptance evidence, and reason to reuse, adapt, or decline. |

## 9.12 Constitution and Continuous Improvement

| **ID** | **Pri** | **Requirement** | **Description** | **Acceptance** |
|---|---|---|---|---|
| IMP-001 | P0 | Layered governance artifacts | Maintain Constitution, Governance Policy, Agent Charters, and Task Contracts as distinct versioned concepts. | Durable principles do not need to be duplicated into every task prompt. |
| IMP-002 | P1 | Operational learning | Learn model/tool/workflow preferences from measured success, verification, cost, retries, and user feedback. | Future routing changes can cite observed performance evidence. |
| IMP-003 | P1 | Improvement proposals | Detect recurring weaknesses and propose new tools, skills, models, workflow changes, or developer work. | Rikuri can explain the observed weakness, options, expected benefit, and risk. |
| IMP-004 | P0 | Protected-core boundary | Constitution, AISecurity, governance authority, secret boundaries, update authority, and production core code cannot silently self-modify. | Core changes require versioned developer-controlled review/test/deploy workflow. |
| IMP-005 | P2 | Bounded extension self-update | Future low-risk skills/profiles may self-update only inside a tested, versioned, rollback-capable modifiable zone. | No extension can expand its own security authority as part of self-update. |

## 9.13 Model Discovery, Acquisition, and Lifecycle Evaluation

| **ID** | **Pri** | **Requirement** | **Description** | **Acceptance** |
|---|---|---|---|---|
| MDL-001 | P1 | Capability-driven model search | When a capability is missing or materially improvable, search trusted registries/public sources for suitable models by task/modality. | User can ask for “local video generation” without naming a model. |
| MDL-002 | P1 | Hardware-fit screening | Compare candidate RAM/VRAM/unified memory, runtime, storage, bandwidth, context, acceleration, and machine compatibility before recommendation. | Incompatible or impractical artifacts are filtered or clearly labeled. |
| MDL-003 | P1 | Evidence-ranked recommendation | Consider model cards, license, provenance, independent benchmarks, community evidence, adoption signals, and Rikuri’s own prior experience. | 2–3 candidates can be recommended with evidence and uncertainty rather than popularity alone. |
| MDL-004 | P1 | Approved acquisition | Obtain user approval before substantial downloads or new runtime/dependency installation. | Approval shows estimated download size, storage, hardware target, license, and security implications. |
| MDL-005 | P1 | Supply-chain screening | Pin exact revision/hash, prefer safe weight formats, quarantine/scan, review custom code/dependencies, and preserve evaluated artifact identity. | Rikuri knows exactly which artifact/runtime/configuration was tested and trusted. |
| MDL-006 | P1 | Task-specific baseline evaluation | Run modality/task-appropriate initial tests before normal service. | LLM, speech, image, audio, and video models are evaluated with relevant metrics rather than one generic score. |
| MDL-007 | P1 | Background probation and ongoing eval | Continuously update model performance from real jobs while respecting idle/thermal/battery/resource policies. | Rikuri learns where the model succeeds/fails over time without degrading interactive use. |
| MDL-008 | P1 | Task-specific parameter profiles | Tune supported generation/load parameters per model+task only when evidence shows repeatable benefit; retain defaults and rollback. | Users need not understand temperature/top-p/context/GPU-offload to obtain sensible settings. |
| MDL-009 | P1 | Evaluated execution identity | Profile exact model revision + quantization + runtime/version + settings + hardware rather than model name alone. | Changes that can affect behavior trigger selective revalidation. |

## 9.14 Workspace and Human-Agent Interaction

| **ID** | **Pri** | **Requirement** | **Description** | **Acceptance** |
|---|---|---|---|---|
| UI-001 | P0 | Universal intent surface | Provide one text/voice interaction surface that can answer, delegate, open capabilities, and manipulate workspaces. | A user can perform a supported action without learning navigation hierarchy or agent terminology. |
| UI-002 | P0 | Minimal Home launcher | Provide a small horizontal set of named capability icons with user-pinnable expansion through More. | Home remains understandable to a first-time user while frequent capabilities can be added later. |
| UI-003 | P0 | Progressive disclosure | Hide infrastructure and advanced controls until relevant or requested. | Ordinary assistant tasks expose no MCP/runtime/worktree/model-parameter setup. |
| UI-004 | P1 | First-class work windows | Represent agents, chats, documents, code, diffs, browser, terminals, graphs, research, tests, dashboards, previews, and evaluators as independent workspace windows. | Supported window types can coexist in one workspace and retain project/context identity. |
| UI-005 | P1 | Freeform + tiled layout | Allow move, resize, maximize, collapse, close, tile, equal-grid, asymmetric preset, and re-resize after tiling. | User can create a 3×2 grid and then enlarge one window without rebuilding the layout. |
| UI-006 | P1 | Native detach and multi-monitor | Allow compatible windows to detach into OS-level windows and move across monitors/desktops/spaces. | A detached window remains linked to the same Rikuri workspace and project. |
| UI-007 | P1 | Context Link Groups | Link multiple windows to the same entity/context with optional per-window lock. | Changing the active entity updates linked views but leaves locked/unlinked views unchanged. |
| UI-008 | P1 | Agent Link | Let multiple windows follow different surfaces of one worker. | One coding worker can simultaneously drive linked Chat, Code, Diff, Tests, and Browser views. |
| UI-009 | P1 | Prompt Link / broadcast | Send one prompt to selected independent agents/models. | User can select 2–3 agents, prompt once, and receive separate responses without manual copy/paste. |
| UI-010 | P1 | Per-window model selector | Allow Auto or an explicitly compatible model/runtime profile to be assigned to a worker/window. | Model choices are filtered by capability/privacy; state survives a model handoff. |
| UI-011 | P1 | Compare Mode | Arrange independent model responses plus a synthesis/evaluator view. | “Ask Claude, GPT, and my local model” produces isolated answers and a comparison without cross-contaminating initial reasoning. |
| UI-012 | P1 | Project Graph | Visualize objectives, tasks, agents, dependencies, artifacts, decisions, evaluations, and status with expandable nodes. | Clicking a node can focus/open the corresponding work surface and reveal progressively deeper detail. |
| UI-013 | P1 | Live Follow | Show meaningful live artifact/file/browser/test changes for a selected worker. | User can observe what an agent is currently changing without waiting for a final posted file. |
| UI-014 | P1 | Saved workspace layouts | Save/restore window types, sizing, positions, links, and project-specific workspace presets. | “Open my KV Cache workspace” restores the conceptual work arrangement. |
| UI-015 | P1 | Conversational workspace control | Expose layout/link/window operations through natural language/voice in addition to direct manipulation. | Spoken/text commands can create, rearrange, link, focus, or close supported windows. |
| UI-016 | P0 | Native ecosystem freedom | Permit users to continue working in preferred external AI/office/developer ecosystems without surrendering Rikuri-owned canonical state. | Provider/UI choice does not become the sole repository of project memory or user knowledge. |

## 9.15 Knowledge & Idea Fabric

| **ID** | **Pri** | **Requirement** | **Description** | **Acceptance** |
|---|---|---|---|---|
| IDEA-001 | P0 | Frictionless Idea Inbox | Capture short or incomplete ideas from text, voice, photo, video, file, chat, clipboard/share sheet, or supported handwritten-note image with minimal required metadata. | User can save an idea without choosing a folder, project, tags, or final category. |
| IDEA-002 | P0 | Preserve original capture | Retain original audio/image/text/document plus timestamp/source/provenance; derived transcript/summary never replaces the source. | User can inspect the original item from any derived note or idea record. |
| IDEA-003 | P0 | Stable idea identity and state | Give each idea a durable ID and lifecycle such as Captured, Exploring, Promising, Attached, Adopted, Implemented, Rejected, or Superseded. | An idea can evolve across projects and devices without being confused with a canonical decision/fact. |
| IDEA-004 | P0 | Automatic linking without forced filing | Suggest or create confidence-scored links to projects, topics, people, scenes, research questions, artifacts, and related ideas. | Capture does not require organization, but related context is recoverable later. |
| IDEA-005 | P0 | Semantic and exact recovery | Search ideas by meaning, words, source, time, project, people, media, and relationships. | User can recover a forgotten idea without remembering where or how it was captured. |
| IDEA-006 | P1 | Quiet resurfacing | Surface potentially useful prior ideas in the current project without interrupting normal work. | Related Ideas appears as a passive cue/digest unless user policy elevates it. |
| IDEA-007 | P1 | Knowledge Web + Idea Web | Represent typed relationships among knowledge, ideas, decisions, artifacts, projects, people, experiments, and source material. | Rikuri can explain how a current concept connects to earlier ideas and their outcomes. |
| IDEA-008 | P1 | Cross-domain reuse | Allow the same underlying idea fabric to serve writing, research, marketing, product development, business, media, study, and coding. | Domain workspaces project specialized views without copying ideas into separate silos. |

## 9.16 Domain Workspaces and Writing

| **ID** | **Pri** | **Requirement** | **Description** | **Acceptance** |
|---|---|---|---|---|
| DOM-001 | P1 | Common workspace substrate | Build specialized domains on the same Window, Link, Project, Knowledge, Idea, Task, and Event primitives. | Coding, writing, research, and study do not create incompatible state systems. |
| DOM-002 | P1 | Coding workspace | Provide code-centric arrangements for live source, agent, diff, tests, terminal/browser, project graph, and review. | User can observe and manage parallel coding work without relying solely on separate IDE chat threads. |
| DOM-003 | P1 | Writing editor integration | Support a Word-like/rich-text editing surface through an embeddable/open editor or automation of compatible external editors while preserving portable document formats. | Rikuri can proofread, suggest edits, comment, track revisions, and export common formats without owning a proprietary document silo. |
| DOM-004 | P1 | Writing assistance tools | Provide grammar, spelling, dictionary, synonym/antonym, alternative verbs/phrasing, rewrite, clarity, tone, repetition, style, and evidence/citation assistance as appropriate. | User can invoke writing help on selected text, sections, or whole documents with reviewable changes. |
| DOM-005 | P2 | Story/long-form canon | Represent characters, locations, timeline, world rules, plot/argument structure, sources, and established facts separately from speculative ideas. | Continuity checks distinguish Canon from Idea/Hypothesis. |
| DOM-006 | P2 | Semantic/narrative refactoring | Propagate conceptual changes across affected passages and linked knowledge rather than blind find/replace. | A changed character trait or report assumption yields a reviewable impact set and coherent proposed revisions. |
| DOM-007 | P2 | Parallel section work | Allow independent agents to draft/review different sections while an editorial coordinator checks voice, contradictions, repetition, dependencies, and integration. | Multi-agent writing can proceed in parallel without silently producing incompatible sections. |
| DOM-008 | P1 | Visual idea/sticky-note board | Offer a spatial sticky-note/index-card view backed by the canonical Idea objects. | Notes can be arranged visually while remaining searchable and linked outside the board. |
| DOM-009 | P1 | Study workspace | Combine media/lecture, transcript, notes, tutor conversation, concepts, questions, and quizzes on the common workspace substrate. | A lecture can become structured, searchable learning material tied to source context. |

## 9.17 Internationalization, Localization, and Multilingual Intelligence

| **ID** | **Pri** | **Requirement** | **Description** | **Acceptance** |
|---|---|---|---|---|
| I18N-001 | P0 | Externalized locale resources | Keep user-facing strings, labels, prompts presented as UI copy, date/number formats, and locale metadata outside application logic with stable localization keys and fallback behavior. | A new language pack can be added without changing core business logic or database schemas. |
| I18N-002 | P0 | Initial locale set | Localize core Home, Quick Assist, Workspace navigation, approvals, security/privacy notices, Settings, Projects, Knowledge & Ideas, and common assistant flows for English, Spanish, Simplified Chinese, Traditional Chinese (Taiwan), Japanese, French, and German. | A user can select any initial language during onboarding or later and complete core workflows without falling back to English except for explicitly untranslated advanced/developer content. |
| I18N-003 | P0 | Independent language preferences | Separate interface locale, assistant-response language, content/source language, and optional per-conversation/per-window overrides. | Changing UI language does not translate or rewrite source documents, stored conversations, project names, or canonical knowledge automatically. |
| I18N-004 | P0 | Locale-aware formatting and Unicode | Support locale-aware dates, time zones, calendars where applicable, numbers, currencies, pluralization, sorting/collation, Unicode text/filenames, CJK input and line wrapping, and resilient layouts for text expansion. | The same data renders correctly under each initial locale and no supported-language text is corrupted or clipped in core UI tests. |
| I18N-005 | P0 | Multilingual knowledge and search | Preserve original-language sources while enabling exact-language search plus cross-lingual semantic retrieval across supported languages. | A supported-language query can retrieve relevant knowledge written in another supported language while retaining links to the original source. |
| I18N-006 | P1 | Translation as derived data | Store machine-generated translations, transliterations, summaries, and language-normalized representations as derived/versioned artifacts with provenance rather than replacing originals. | Users can inspect the source language and translation origin and regenerate translations with another model later. |
| I18N-007 | P1 | Language-aware model routing and evaluation | Include language/modality competence in Model Lab profiles and route tasks to models that perform well in the requested language; evaluate important local models across relevant configured languages. | Rikuri does not assume a model that performs well in English performs equally well in Japanese, Chinese, Spanish, French, or German. |
| I18N-008 | P1 | Multilingual voice | Support language-aware speech recognition and speech synthesis where runtimes permit, including explicit language selection and safe auto-detection. | Voice conversations and captured notes can use supported languages without forcing translation to English first. |
| I18N-009 | P0 | Localization quality gates | Add missing-key checks, pseudo-localization/text-expansion tests, CJK rendering tests, locale-format tests, and regression coverage for core user journeys. | A release cannot report an initial locale as supported when critical core surfaces have missing strings or broken layout. |

## 9.18 Rikuri Link and Remote Owner-Node Access

| **ID** | **Pri** | **Requirement** | **Description** | **Acceptance** |
|---|---|---|---|---|
| LINK-001 | P1 | Phone-home access | A phone/tablet/laptop can securely reach the user's own Rikuri Node for chat, task continuation, status, approvals, and file/photo handoff. | Remote use does not require a cloud-hosted persistent agent. |
| LINK-002 | P1 | Tailscale initial connector | Use Tailscale as the first secure private-network transport without public port forwarding. | Authorized device connects through simple Rikuri UI; user need not configure VPN/IP/SSH concepts. |
| LINK-003 | P1 | Durable reconnect | Remote disconnect/reconnect does not duplicate or lose committed task state. | Active task resumes from Rikuri-owned checkpoint. |
| LINK-004 | P2 | Native Rikuri transport | Add direct connectivity plus rendezvous/discovery and E2E encrypted relay when direct connection fails. | Relay cannot read plaintext task traffic; Tailscale remains fallback. |
| LINK-005 | P2 | Multi-node placement | Expose owner nodes and later route work according to capability/project locality/privacy. | `Run on: Auto` chooses an eligible node and records why. |

## 9.19 Native Developer Platform and Plugins

| **ID** | **Pri** | **Requirement** | **Description** | **Acceptance** |
|---|---|---|---|---|
| DEV-001 | P1 | Monaco editor | Monaco is Rikuri's embedded code editor Window; the application is not built on the VS Code workbench. | Code/diff/selection/diagnostic workflows operate inside Rikuri state. |
| DEV-002 | P1 | Standard developer services | Integrate LSP, DAP where useful, Git, terminal, tests/tasks, formatters/linters, search, and container/dev tools behind Rikuri contracts. | Core developer capability does not depend on arbitrary VS Code extensions. |
| DEV-003 | P1 | Native plugin/window API | Plugins may add editor/agent/tool/service/window capabilities under explicit AISecurity permissions. | Plugin cannot silently acquire unrestricted filesystem/network/secrets/canonical writes. |
| DEV-004 | P1 | Verified plugin lifecycle | Provenance/pin/quarantine/activate/health/update/rollback/remove lifecycle. | Broken or unsafe update can be disabled/rolled back with evidence. |
| DEV-005 | P2 | Selective VS Code compatibility | Use `monaco-vscode-api`-class compatibility only where direct/native integration is not preferable. | Rikuri retains its own workspace UX and security boundary. |

## 9.20 Collaborative Intelligence Foundation Hooks

These requirements exist **now** so later Collaborative Intelligence can be scaffolded safely. Foundation-0 implements evidence capture and contracts only; autonomous reflection, user-pattern inference, Lightbulb generation, and adaptive policy learning remain post-Foundation capabilities.

1. Meaningful executions MUST receive stable task/run IDs and an exact execution profile where available: model and revision, provider/access path, runtime, harness/profile revision, prompt/profile revision, context policy, reasoning/sampling settings exposed by the runtime, tool/MCP identities, machine, privacy envelope, and verification strategy.
2. Rikuri MUST emit structured lifecycle/outcome events rather than relying on transcript reconstruction. Event capture SHOULD focus on meaningful state transitions, failures, retries, interventions, verification, cost, and outcomes rather than keystroke/screen surveillance.
3. Performance evidence and future experience-learning evidence MUST share stable references to the same underlying runs and outcomes so routing, workflow, and collaboration learning do not create contradictory private ledgers.
4. User feedback MUST distinguish immediate preference from verified task outcome. The architecture MUST allow delayed usefulness signals so an idea or question ignored today can later be credited if it proves useful.
5. Derived patterns, hypotheses, heuristics, model preferences, and future Lightbulbs MUST be non-canonical candidate intelligence with explicit provenance, support/counterexample references, scope, confidence, timestamps, version/environment scope, and supersession/retirement state.
6. The Knowledge Service remains the sole governed mutation path into canonical Personal Intelligence. Agents may propose candidate knowledge; they do not independently reorganize or rewrite the user's authoritative knowledge store.
7. The schema MUST leave room for versioned cognitive recipes: task purpose, frame/prompt/context profiles, model/runtime/harness, tools, reasoning/sampling policy, optionality budget, verification profile, and observed outcomes. Exact seed values, when supported, are experiment/reproducibility metadata rather than permanent model-quality labels.
8. Foundation telemetry MUST be privacy-classified, user-controlled, minimizable, exportable/deletable under policy, and excluded from sensitive spaces when observation is not authorized.
9. No later learning mechanism may directly self-modify protected core policy, security boundaries, canonical truth, or irreversible user state. Learned suggestions/promotions require explicit gates appropriate to consequence.
10. The user MUST be able to inspect why a future suggestion, learned heuristic, or intervention was surfaced and which observations support it.

## 9.21 Evidence-based decision support (post-Foundation)

September 26, 2026 clarification: Rikuri should help individuals and small teams make credible strategic decisions through proportionate research, explicit alternatives, and inspectable evidence. Method details live in [Collaborative Intelligence §8](Rikuri_Pi_Collaborative_Intelligence_Architecture_v0.1.md#8-cognitive-ensembles). These requirements are future product scope; they add no Foundation-0 gate or runtime service. Product Stage 4 evidence and Stage 6 governance precede research prototypes; Stage 10 owns productization.

| ID | Pri | Requirement | Acceptance for future implementation |
|---|---|---|---|
| DEC-001 | P1 | Simple request, rigorous procedure | An ordinary strategy question selects an inspectable, versioned method with explicit objectives, constraints, privacy, budget and stopping conditions; no agent configuration required. |
| DEC-002 | P1 | Evidence for and against | Material claims retain provenance, source dependence, date, assumptions, contradictions and collection gaps; repeated sources do not become independent confirmation. |
| DEC-003 | P1 | Meaningful choices | Normally two or three viable actions are compared across plausible scenarios with downside, reversibility, mitigation and reassessment conditions; unsupported alternatives and insufficient evidence remain visible. |
| DEC-004 | P1 | Independent challenge and verification | Initial alternatives precede cross-critique; tests or targeted research address pivotal disagreement; dissent survives synthesis; agreement and ACH scores are not proof or calibrated probabilities. |
| DEC-005 | P1 | Credible business and fundraising claims | Actual traction, estimates and forecasts are distinguished; market and financial calculations are reproducible; opposing evidence and financing dependencies are disclosed; claims remain traceable in exported material under source permissions. |
| DEC-006 | P1 | Evaluated decision quality | Procedures outperform or justify their tradeoffs against simpler baselines on predeclared cases, including negative tests, cost and user effort; no promise of unbiased judgment, funding or business success. |
| DEC-007 | P1 | Privacy-aware collaboration placement | Local, remote and mixed work use explicit data/authority admission; proprietary source-derived content cannot silently move to a VPS or cloud model; advice cannot authorize execution. |

Foundation compatibility is limited to the existing §9.20 evidence hooks and Architecture §21A.8. New decision objects, ACH engines, councils, business workflows, watchers, and VPS executors remain deferred.

## 9.22 Purpose-driven presets and tool-assisted work (post-Foundation)

September 26, 2026 clarification: users should request a useful outcome and receive an appropriate, verifiable workflow with explanations suited to their needs. [Collaborative Intelligence §6.4–6.7](Rikuri_Pi_Collaborative_Intelligence_Architecture_v0.1.md#64-purpose-driven-capability-presets) defines preset semantics; Architecture §21A.3 and §25 define integration boundaries. Existing component stage order remains; integrated recipe research requires product Stage 4 evidence and Stage 6 governance, with Stage 10 productization. These requirements add no Foundation-0 gate.

| ID | Pri | Requirement | Acceptance for future implementation |
|---|---|---|---|
| PRE-001 | P1 | Purpose-driven composition | An outcome selects a versioned recipe covering models/roles, tools, skills, plugins, context topology, success criteria and governance; use the smallest sufficient configuration. |
| PRE-002 | P1 | User choice and customization | Built-in defaults, saved custom presets and task overrides resolve transparently; users inspect differences and save variants; incompatible selections explain why and offer eligible alternatives. |
| PRE-003 | P1 | Safe reset and upgrades | Reset previews its target version and changes, supports policy-revalidated undo, preserves project data/work/evidence, and never regrants authority; upgrades preserve customizations and active runs retain resolved configuration. |
| PRE-004 | P1 | Isolated and joint context | Initial independent work and later joint review have explicit information boundaries; source access restrictions survive sharing and synthesis. |
| PRE-005 | P1 | Extensible tool choices | Plugins/MCP capabilities become selectable only with truthful capability, locality, cost, permission and evaluation information; installation does not grant execution or cross-project access. |
| PRE-006 | P1 | Method and tool fit | Exact, numerical, stochastic and semantic tasks select appropriate tools; results preserve assumptions and reproducibility evidence; deterministic checks remain outside probabilistic authority. |
| PRE-007 | P1 | Interpretation with expandable depth | Novices receive practical meaning and limitations; experts can inspect sources, methods, code and diagnostics; both views preserve the same material uncertainty. |
| PRE-008 | P1 | Actionable evidence gaps | Missing data and decision impact are explicit; on request suggest collection/source options and their limitations, effort and cost without initiating unauthorized collection. |
| PRE-009 | P1 | Outcome verification and graceful failure | Real tool outcomes are checked, missing dependencies/degradation are visible, and substituted tools require eligibility/evaluation; creative judgments remain distinct from technical checks. |

Business, research, mathematical and video-production presets are illustrative future applications, not promises of specific commercial integrations. Foundation-0 retains only existing §9.20 evidence, identity, privacy and extension-friendly contracts; it does not implement a preset manager, plugin marketplace, team orchestrator or simulation/media suite.

## 9.23 RIEMS — outcome-driven execution management (post-Foundation)

November 17, 2026 — **owner-approved future architectural direction; not implemented**. [ADR 0007](../docs/architecture/adr/0007-riems-outcome-driven-execution.md) establishes RIEMS above the semantic scheduler/durable runtime, after the existing Capability Planner. It consumes `TaskRequirements` and current task, authority and budget contracts; the existing router still resolves concrete execution profiles. These are P1 post-Foundation requirements, not Foundation-0 gates or evidence of a working optimizer.

| ID | Pri | Requirement | Acceptance for future implementation |
|---|---|---|---|
| RIEMS-001 | P1 | Outcome-first contract | Versioned objective, deliverable, acceptance/evidence, quality floor, deadline/pace, budget, privacy, risk and human authority survive handoffs without a competing intent schema. |
| RIEMS-002 | P1 | Minimum adequate strategy | For complex tasks compare deterministic tooling, strong single-worker baseline and a justified limited specialist option; no fixed crew-size target. |
| RIEMS-003 | P1 | Dependency and resource admission | Critical path, slack, waiting, integration, local/mesh/cloud resources, quotas, conflict and unknowns are evaluated; policy and scheduler still govern admission. |
| RIEMS-004 | P1 | Explainable selection | Versioned decisions record candidates, constraint checks, evidence, confidence, predicted time/cost/quality/risk and fallback, with proportional user-facing explanation. |
| RIEMS-005 | P1 | Adaptive bounded replanning | Blocked tools, slippage, verification failure, task revision or capacity changes yield traceable reconsideration; never skip policy, approval, budget or mandatory quality gates. |
| RIEMS-006 | P1 | Durable execution | Shared runtime yields waiting workers; resumed work is authenticated and revision-correct, with idempotent dispatch, retries and no duplicate/silent side effects. |
| RIEMS-007 | P1 | Independent outcome verification | Acceptance against current task requirements and artifact revisions, not agent declarations, process exit or UI status, controls completion. |
| RIEMS-008 | P1 | Evidence-based learning | Existing Performance Ledger compares predictions with actual cost/time/quality, resource occupancy, failure and rework; promotions are bounded, reviewed and reversible. |
| RIEMS-009 | P1 | Human-friendly control | Simple outcome/pace/cost controls with optional expert override, inspection, pause and cancel; no model/app/agent expertise required. |

**Sequence:** product Stage 1 measures single-worker/tool-only baselines; Stage 4 compares routing economics; Stage 6 qualifies critical-path-aware strategy and justified teams; Stage 7 separately qualifies trusted mesh placement. [Orchard RIEMS boundary](Rikuri_Pi_Orchard_RIEMS_Integration_2026-11-17.md) is a software-project domain specialization, not a second authority. Foundation-0 Stage Plan v2.1, W1/F1–F10, F4-lite and single-worker defaults remain unchanged. Evaluate verified completion rate, quality, deadline reliability, total time/cost per verified outcome, rework, incidents and uncertainty—not agent count.

# 10. Security and Trust Requirements

**Protecting the user's private information, financial records, personal files/photos/videos, proprietary and professional databases, and secrets is Rikuri Pi's highest product responsibility.** Confidentiality, protection against unauthorized changes, and dependable recovery take priority over convenience, autonomy and feature coverage. A capability stays unavailable when its required protection cannot be established.

September 28, 2026 adoption: **Option C** keeps canonical permissions under Rikuri authority, uses AISecurity as a trusted policy/decision service, and delegates containment to qualified, replaceable enforcement adapters. OpenShell is the first Ubuntu candidate; adoption is conditional on qualification. [Architecture §27.12](Rikuri_Pi_Architecture_Spec_v1.7.md#2712-option-c-policy-ownership-and-enforcement-adapters) defines ownership, and the [threat model](../docs/security/THREAT_MODEL.md) defines assets, boundaries and test mapping. These are requirements; the current repository does not implement a secure personal assistant.

| **ID**  | **Pri** | **Requirement**                  | **Description**                                                                                                                                        | **Acceptance**                                                                                   |
|---------|---------|----------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------|
| SEC-001 | P0      | Common security boundary         | All workers—frontier, local, native harness, generic harness, browser, GUI, and shell—must pass through Rikuri security/policy controls.               | Changing model/runtime does not expand authority automatically.                                  |
| SEC-002 | P0      | Capability envelopes             | Grant task/worker-specific access to files, tools, network destinations, machines, credentials, and operations.                                        | Worker can operate freely inside granted scope but cannot silently escape it.                    |
| SEC-003 | P0      | Risk-class execution             | Classify observe/read-only, known repair, novel modification, destructive/high-consequence, and financial/credential operations.                       | Approval/enforcement policy follows consequence rather than generic “tool call” status.          |
| SEC-004 | P0      | Validated runbooks for high risk | Support deterministic Rust/other trusted operations for sensitive systems such as a Bitcoin node.                                                      | AI can diagnose and choose an operation without inventing arbitrary high-risk shell commands.    |
| SEC-005 | P0      | Financial safety boundary        | Isolate wallet seeds, private keys, signing, fund movement, sensitive LND/Bitcoin credentials, and transaction authority from general AI shell access. | General machine administration cannot imply financial authority.                                 |
| SEC-006 | P0      | Security health supervisor       | Each protection module must expose heartbeat, last success, event count, errors, enforcement state, and capability coverage.                           | “Protected” means required modules are healthy, not merely that a daemon HTTP endpoint responds. |
| SEC-007 | P0      | Quiet degradation alerts         | Healthy security stays quiet; degraded or critical protection is surfaced immediately and can restrict dependent capabilities.                         | Silent subsystem death cannot remain indistinguishable from healthy silence.                     |
| SEC-008 | P0      | Audit ledger                     | Record allowed, constrained, blocked, and approved actions with actor, intent, scope, policy, and outcome. November 14, 2026: the record is written by a component no worker can write to, alter or delay; each tool call is recorded when it happens, not at the end of a turn; a worker's own account of what it did is reconciled against the recorded calls. | Security decisions are reviewable without flooding the user during normal use. November 14, 2026: a worker told to delete, edit or delay its record cannot; a suppressed or out-of-sequence record raises degraded health (SEC-006); a call the worker leaves out of its own report is flagged. |
| SEC-009 | P1      | Intent consistency               | Compare proposed actions with task/user intent as a soft security signal while deterministic policy remains final authority.                           | Unrelated sensitive actions are detected even if syntactically valid.                            |
| SEC-010 | P1      | Transactional changes and undo   | Checkpoint reversible project/file changes where practical.                                                                                            | User can inspect or undo autonomous changes without understanding Git internals.                 |
| SEC-013 | P1 | Scoped autonomy presets | Offer Inspect and plan, Isolated development, Staging and integration, and Live maintenance presets, with unattended operation as a separate option. Resolve authority by task, environment, data, services and consequences; interpreter name, project age and model confidence do not determine risk. | A bounded development grant permits ordinary authorized coding, debugging, testing, fixture cleanup and project-process restarts without repeated command approvals; operations beyond the effective envelope remain constrained or refused. |
| SEC-014 | P1 | Understandable approval settings | Provide Allow once, Allow for this task and Save for this project controls with scope and expiry, a versioned effective-policy preview, and understandable consequences, recovery options and governing blocker layer. Users need not edit permission files or application code. | A novice can inspect what will run automatically, what will wait and why; Rikuri, repository, harness, operating-system, connector and organization restrictions are distinguished. A preset cannot override a stricter governing layer. November 14, 2026: a saved approval stores an operation pattern and scope, never a literal command line, argument or environment value that may hold a credential, and is excluded from sync, export and repository tracking unless the owner shares it explicitly; approving a command that contains a synthetic canary and saving it leaves no canary in any stored policy, log or tracked file. |
| SEC-015 | P1 | Unattended readiness and bounded progress | Before unattended work, exercise representative permitted operations in the intended environment and inspect likely blockers, time/budget/resource limits, credentials, dependency sources, checkpoints and recovery. Report Ready, Ready with limits or Not ready from evidence, with a bounded wait/stop policy. | Predictable first-operation permission failures are caught before departure. Missing required assurance prevents Ready; unforeseen blockers remain visible. The completion report distinguishes verified work, failures, queued decisions and the reason progress stopped; absence of a user response never grants permission. |
| SEC-016 | P1 | Approval-aware independent work | Under an existing grant, prepare and verify recoverable candidates in an isolated environment while consequential activation awaits approval. Queue a concrete candidate and continue independent authorized work. Preserve dependency and required red-gate boundaries. | Waiting to apply a live change does not halt unrelated permitted work. Isolation does not imply access to live mounts, credentials or services; no dependent action advances past its unresolved approval or required failed gate. Material candidate changes invalidate approval for the previous candidate. |
| SEC-017 | P1 | Defect-scoped regression evidence | Prospectively support per-defect repair cycles with immutable original regression tests, fixtures, expected outcomes and failure receipts, independent verification, and trusted-controller completion only under an explicitly admitted development policy. Preserve an auditable amendment route for faulty evidence or changed requirements. | Routine verified cycles can complete within that grant without recurring owner-release prompts. Tests, discovery or verification cannot be weakened to obtain success; adding independent tests remains possible. The coding agent cannot release its own restrictions, and the current owner-only fix-lock rule remains active until separately reconciled. |
| SEC-018 | P1 | Grant continuity and revocation | Carry authenticated grant references, scope, expiry and pending approvals through model/harness switches, handoffs, resume and compaction. Revalidate against current trusted policy, revocation, environment and material changes before use; keep effective enforcement outside the worker's writable authority. | A replacement model receives valid authority without needless reapproval, but cached context cannot revive an expired/revoked grant or expand scope. Executable-metadata changes still trigger project-trust revalidation; an agent's edits to policy, verifier or application code cannot activate their own authority. |
| SEC-019 | P1 | Live-service and financial consequences | Evaluate service availability, resource consumption, credentials, network exposure and durable data independently of custody, signing, wallets or channel funds. Permit only explicitly scoped live queries or admitted maintenance procedures. | An unfunded live-node/API development scenario can allow isolated coding and bounded authorized queries while live restarts, configuration or durable-data changes require their own grant or procedure. Zero funds neither grants live-system authority nor subjects every development action to financial approval. |
| SEC-020 | P0 | Personal-data protection duty | Default to the least data and operations needed for the task. No blanket home-directory, media-library, professional database or everyday browser access. Distinguish reading, modifying, deleting, exporting and sharing: read/search never grants copy, export, upload, share, sync or bulk extraction, and destination permission alone cannot authorize payload disclosure. | Synthetic private assets remain inaccessible without a current scoped grant; legitimate granted work succeeds; a read-only grant denies export. Permission explanations name the data, destination, consequence and duration. |
| SEC-021 | P0 | Qualified enforcement adapters | Keep Rikuri permission authority independent of backend choice. Rikuri owns grants, the AISecurity policy service evaluates them, and qualified adapters enforce them at actual resource boundaries. Authenticated identity is attribution, not trust; models, plugins, manifests, role names and backend-native proposals or approvals cannot edit effective authority. Bind execution to a verified backend, platform, effective policy and current grant; fail closed on unsupported, stale or failed assurance. November 14, 2026: a request or message from another agent or worker is untrusted input however it is labelled, and cannot edit effective authority. | Backend loss or replacement cannot launch an unrestricted worker or preserve expired/revoked authority; descendants and control channels satisfy the same contract; forged roles/manifests and child escalation are denied. November 14, 2026: a fabricated peer-agent message asking for an out-of-grant action is refused exactly as a direct request would be. |
| SEC-022 | P0 | Brokered credentials | Keep real secrets outside worker files, environment, arguments, model context and logs. A trusted broker performs only granted operations, with endpoint/operation/task scope, expiry and revocation; it rejects unsafe redirects and destinations and distinguishes lease revocation from upstream credential invalidation. Providers without bounded revocation disclose it and refuse profiles requiring it. | Synthetic canaries cannot be extracted or redirected; unauthorized use and stale grants fail. Financial read access does not permit signing, payment, trading or custody. |
| SEC-023 | P0 | Private-data lifecycle and integrity | Source restrictions govern derived text, media metadata, thumbnails, embeddings, summaries, caches, logs, versions and backups. Attachments and cross-project copies retain restrictions; only a trusted explicit declassification with recorded basis and provenance lowers one, and contradictory labels refuse rather than downgrade. Preserve originals and verify recovery before claiming reversibility. | Derived data cannot bypass privacy, retention, audience or current-revocation checks; failed/racing mutations preserve the last valid state. Unrecoverable actions require explicit consequence handling. |
| SEC-024 | P0 | Evidence-backed trust | Show protection per supported path/platform and tested configuration, including gaps, last verification and degradation. Publish scoped assurance and measure both attack refusal and successful authorized work. Record minimized, access-controlled request/grant/approval/revocation/decision/outcome evidence; prove authorized, forbidden, bypass and broken-enforcement paths per advertised guarantee; revalidate on package/plugin/runtime change at the Engineering Standard tier. Learned anomaly signals may restrict, never grant. | No unsupported path is labeled protected; no comparative reliability claim appears without a reproducible comparison and independent review. Required failure/regression evidence blocks release. |

> **Approval principle**
>
> Do not ask the user to approve syntax they cannot evaluate. Present the intended change, consequence, affected scope, risk, rollback, and verification plan. Raw command details belong behind an Advanced view.

September 27, 2026 clarification: SEC-013–019 describe **future product behavior, not implemented approval settings or a change to current repository permissions**. [Architecture §16.5–16.9](Rikuri_Pi_Architecture_Spec_v1.7.md#165-scoped-autonomy-and-user-presets) owns the detailed contract. Product Stage 1 retains scoped settings and supporting execution; the September 30 resequencing moves the advanced single-worker repair controller to the optional Stage 8 developer pack after its test/trust contracts and explicit policy/ADR reconciliation; product Stage 2 adds richer workspace UX, and product Stage 6 owns durable branching/wait recovery and multi-agent governance. Foundation-0 retains its existing capability, budget, approval and execution-envelope commitments without expansion. [AGENTS.md](../AGENTS.md#claude-guard-and-fix-lock) still requires owner release of the current fix-lock, and the [verifier](../.claude/agents/verifier.md) remains report-only. The live-node example illustrates consequence separation; it authorizes no access or change to the user's current project. All new behavior remains **Not yet proven** until the owning stages supply implementation and acceptance evidence.

## October 1 security-draft reconciliation

The supplied Security Architecture v1.0 is reviewed in [the comparison record](reviews/2026-10-01-security-draft-review.md). Compatible additions below supplement SEC-001–024. October 3 consolidation: the draft rows that refined Option C were folded into it (SEC-025 into SEC-021, SEC-026 into SEC-020/023, SEC-027 into SEC-022, SEC-032 into SEC-024); those four numbers are retired and not reused. SEC-028–031 remain because Option C does not cover them; where any text overlaps, the Option C rows and Architecture §27.12–27.13 govern. Architecture §27 remains the maintained security design; the draft's phase sequence and backlog are not adopted. Product-level P0 describes a required protection when its dependent capability is offered, not a new Foundation-0 commitment. All additions are unimplemented requirements.

| ID | Pri | Requirement | Acceptance |
| --- | --- | --- | --- |
| SEC-028 | P0 | Provider privacy evidence | Record endpoint, access mode/account contract, retention/training/region conditions where relevant, evidence provenance and last verification/expiry. Unknown or stale required assurances prevent that routing choice; provider/model switches cannot weaken privacy. Sanitization alone cannot make Local Only/Secret content eligible. |
| SEC-029 | P1 | Private-network and egress capability lifecycle | Network identity does not replace application authority. Verify installed policy, source/destination/protocol limits, DNS/redirect/alternate-route behavior and expiry/revocation including existing sessions. Controller failure cannot retain forbidden access; stop/isolate if revocation cannot be proved within the measured latency bound set by the owning adapter ADR (S03.11). Tailscale is initial Link transport, not arbitrary Internet egress enforcement. November 14, 2026: a worker's network path always refuses link-local and cloud-metadata addresses (169.254.0.0/16 and the IPv6 equivalents); it refuses private-range (RFC 1918 and IPv6 unique-local) and loopback addresses unless a current grant names that endpoint explicitly, as an admitted Link or node destination does, so a hostname or wildcard allowlist entry that resolves to such an address is refused; a destination is resolved once and the connection goes to the resolved address; a redirect to a refused class or off the allowlist is refused; a worker cannot open raw sockets; reaching a refused destination through an allowed service (a fetch proxy, an archive, a URL scanner, a logging or metrics sink) is a bypass. Each case is refused and recorded while a legitimate allowed request still succeeds. |
| SEC-030 | P1 | Step-up authentication and understandable security state | Trusted approval states actor, concrete target/data/destination, duration, cost, reversibility and verification. Stronger authentication for configured high-impact operations proves identity and cannot override forbidden policy, Secret exclusion, forgetting or unavailable assurance. Show Unknown/Unverified separately from fresh verified health and prevent dependent use without required proof. November 14, 2026: an approval binds to a digest of the exact serialized action it covers; the executor recomputes the digest at execution and refuses on mismatch, so pending state cannot change after approval without invalidating it. The approval surface and any step-up factor or code sit where no worker, harness, browser worker or extension can read or actuate them. No worker can raise a limit that applies to itself; a limit change is itself an owner step-up action. An action mutated after approval is refused; a synthetic worker with access to the user's test inbox and browser session cannot obtain a step-up code or actuate the approve control; a worker's request to raise its own limit is refused and recorded; an approved, unchanged action still executes. |
| SEC-031 | P0 | Transactional security policy activation | Validate, durably write/read back and atomically promote versioned policy/grants/keys; independently verify the effective enforcement state before exposing the capability as active. Partial external rule changes, failed activation or rollback isolate/refuse dependent work with visible health. Recovery uses current revocation; old known-good policy cannot resurrect revoked access. Activation and revocation meet the adapter's measured latency bound. |
| SEC-033 | P1 | Bounded outbound conduct (November 14, 2026) | A refusal by policy, or by a remote party (an authentication wall, a 401 or 403, a robots disallow, a CAPTCHA), ends that attempt and is reported. A retry of the same goal through another tool, route, credential or URL is refused, recorded and alerted. A network-capable worker has a request budget per destination per task, enforced outside the worker. Requests carrying injection or traversal payloads the task did not call for are refused as out of scope. Acceptance: a worker denied once cannot reach the same target by a second route; a loop against one host stops at the budget with a visible blocker; a legitimate retry of a transient failure inside the budget still succeeds. |
| SEC-034 | P1 | Admission of connectors, MCP servers, plugins and skills (November 14, 2026) | Admit each by content digest with its dependency set locked; a commit identifier on a remote that can rewrite history is not sufficient identity. Nothing resolves, updates or downloads at launch. A remote endpoint is bound to its admitted host identity; a changed hostname, certificate chain or resolution target requires re-admission. Tool arguments reach a child process as an argument vector, never through a shell or an expression-evaluating template. Authorization endpoints are pinned to the admitted issuer. A placeholder or unregistered domain in a manifest refuses admission. Acceptance: a swapped artifact behind an unchanged pin, a launch that would fetch, a moved remote endpoint and a token-endpoint redirect are each refused; an argument containing shell metacharacters is delivered literally; an admitted, unchanged connector still works. |

November 14, 2026 security-briefing intake: the dated sentences in SEC-008, SEC-014, SEC-021, SEC-029 and SEC-030 and the rows SEC-033–034 were adopted by the owner from a disposal of 52 daily AI-security briefings ([ADR 0000 addendum](Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md#november-14-2026--security-briefing-intake); [Build Record](../docs/build-records/2026-11-14-security-briefing-intake.md)). Earlier text is preserved. All are unimplemented requirements and **Not yet proven**. Foundation-0 gates are unchanged, and no Foundation-0 stage gains an acceptance obligation from this intake. Owning stages: the SEC-029, SEC-033 and SEC-034 additions bind at the first stage that enables a network-permitted or credential-bearing worker (expected product Stage 3 Link, Stage 5 connectors or the Stage 8 developer pack), because Foundation-0 workers hold no credential and have no permitted destination; the SEC-008 and SEC-014 additions are owed from product Stage 1, with Foundation-0's existing ledger and audit acceptance (T00.12, T02.16, T04.18) standing as written; the SEC-030 addition is owed with the product Stage 2 approval experience, with S02.08 standing as written; the SEC-021 clause belongs to Stage 6 multi-agent governance. Pulling the SEC-008 or SEC-030 fixtures forward into Foundation-0 Stage 04 or Stage 02 would change a gate and needs its own dated ADR; that choice is open and is the owner's. Architecture §27 is not changed by this intake; the design detail is owed in the owning stage's adapter ADR. [Test Strategy §21](Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md#21-security-briefing-intake-acceptance) maps the future evidence.

[Architecture §27.14](Rikuri_Pi_Architecture_Spec_v1.7.md#2714-october-1-security-contract-reconciliation) specifies the boundary/interpretation; [Test Strategy §17](Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md#17-security-draft-addition-acceptance) maps future evidence and stage dependencies. Existing Foundation gates are unchanged. Foundation-0 workers hold no credential and have no permitted destination; broker and destination qualification (Test Strategy §8.2) precede the first stage that enables either, expected Stage 5 connectors or the Stage 8 developer pack. Rich secret brokerage/connectors belong to product Stage 5, network/device integration to Stage 3 with richer delegation at Stage 6, replication to Stage 7, enterprise/attestation to Stage 9, and learned anomaly methods only after their research/evidence prerequisites. Existing Foundation credential handling, cryptography, identity and basic sequence signals remain required at their existing stages.

## 10.1 Privacy classification and data minimization

The existing context schema uses **Normal**, **Private**, **Local Only**, and **Secret**, combined with Auto/Private/Local Only execution intent. Proposed Public/Confidential labels are descriptive sensitivity metadata, not replacement execution modes or permission grants; an unspecified mapping refuses admission. User changes use trusted authorized policy, not inferred reclassification. Local Only/Vault content and Secret credentials must remain distinct: Secret never enters even a local model context. Classification must affect context assembly before model invocation. Local Only raw content cannot be sent to an external model/service. Secret material such as seed phrases, private signing keys, OAuth refresh tokens, SSH private keys, and comparable credentials should not be exposed to AI models at all.

Where feasible, Rikuri should process sensitive source data locally and send only the minimum sanitized/derived result to a frontier model when policy permits. Derived data retains restrictions unless a trusted explicit declassification approves a distinct non-secret outbound artifact; an LLM summary or detector score cannot authorize it. Secret values remain excluded from all model context. Multimodal privacy classification must eventually cover text, documents, images, audio, video, transcripts, metadata, and extracted text.

SEC-020–024 apply as each capability is admitted. Foundation-0 implements the existing scoped file/model/worker/storage paths through Stages 00–05; the [ADR addendum](Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md#september-28-2026--option-c-and-personal-data-protection) and Stage Plan specify the refined qualification work. Native desktop, media, connectors, collaboration and financial operations retain their later product stages and require their own evidence before enablement.

Unclassified imports begin with conservative private/local handling until a trusted user or admitted policy assigns their permitted use. Model suggestions and source text cannot lower a restriction. Sanitization or summarization alone does not declassify data; a separately authorized, tested disclosure decision is required. Derived content inherits the applicable source restrictions, including audience and retention. Secrets are excluded from general indexing and model context; credential operations use the broker.

## 10.2 Egress enforcement for supported paths

The privacy router cannot be merely optional middleware. Rikuri Security must become an unavoidable capability and egress authority across supported AI calls, APIs/connectors, browsers, file uploads, clipboard, email attachments, GUI workers, shell/network access, and custom code. A path that bypasses policy is a security defect. Each path must be qualified before being offered as protected; this is not a claim to intercept all activity on the owner's computer. A permitted destination can still receive stolen data, so endpoint allowlists do not replace context admission and separation of private data from network-capable workers. Local Only requires both refusal before external request construction and a proven no-external-network execution profile. Optional telemetry and support uploads are off by default and must satisfy the same privacy rules when explicitly enabled.

## 10.3 Secrets and financial authority

AI may request operations that require credentials without receiving the credentials themselves. A secret/credential broker should authenticate on behalf of authorized tools and return only necessary results. Financial and cryptographic signing authority must remain segregated from general AI execution, with dedicated policy, human authorization where required, and hardware/isolated signers when appropriate.

## 10.4 Secure working material and browser isolation

Highly sensitive material should minimize persistent plaintext exposure; secure views/ephemeral workspaces are preferred to broadly decrypting files for long periods. Rikuri browser/computer workers should use isolated profiles and controlled upload/download/network capabilities rather than automatically inheriting the user’s unrestricted everyday browser session.

## 10.6 Project/repository execution trust

A newly opened, cloned, downloaded, shared, or imported project is **untrusted data by default**. Rikuri distinguishes `read/index` authority from `execute/project-code` authority. Reading files, running semantic search, or asking a model questions about a project must not implicitly authorize Git-config helpers, hooks, package lifecycle scripts, build systems, Dockerfiles, MCP/workspace configuration, or other project-controlled executables.

Project trust elevation is explicit, scoped, reversible, and recorded. Child processes inherit the execution envelope; approving a parent command such as `git status` does not authorize an arbitrary descendant helper configured by the project. Untrusted projects initially execute, if at all, inside a restricted environment with constrained filesystem, network, secrets, and privilege.

## 10.5 Personal Mesh cryptographic privacy

Shared Rikuri state should be end-to-end encrypted between authorized owner devices where practical; an optional home hub should be able to relay encrypted state without requiring plaintext knowledge. Device identity, revocation, key rotation, and recovery must be part of mesh design.

# 11. Knowledge & Idea Fabric, Projects, and Search

Rikuri must treat conversations, ideas, Obsidian notes, source code, documents, photos, audio/video, experiments, project state, machine resources, and external references as one searchable conceptual space while preserving their distinct authoritative storage locations. The user experiences one **Knowledge & Idea Fabric** even though different representations are optimized for durability, human readability, transactions, semantic retrieval, graph relationships, or live execution.

| **Layer** | **Role** | **Examples** |
|---|---|---|
| Original source archive | Durable evidence and original capture | Raw chats, photos, audio, video, PDFs, documents, handwritten-note images |
| Canonical structured state | Authoritative identity/state/relationships for Rikuri-managed objects | Projects, ideas, tasks, decisions, people, entities, provenance, sharing, lifecycle states |
| Human-readable Markdown | Browse generated views; edit authoritative user notes | Obsidian/Markdown sources and separately generated reports/projections under §9.3 |
| Authoritative external resources | Source of truth owned by an external mature tool | Git repository for code, office document file, original media file, live service/device state |
| Project/resource registry | What exists and where | Lean_llama.cpp on Ryzen + Git remote; AISecurity on Mac; dataset on NAS |
| Derived retrieval fabric | Fast exact/semantic/relational retrieval | Vector embeddings, full-text index, code/symbol index, graph projections, rerankers |
| Live resolver | Current authority for execution | Current repo HEAD, current file, current document version, current service state |

## 11.1 Capture first, structure later

Rikuri should make preservation cheaper than organization. A user may capture a fragment such as “What if we quantize K differently?”, dictate a longer thought, photograph a location, save a screenshot, import a notebook image, or preserve a chat insight without knowing where it belongs. The system preserves the source immediately, then classifies and links it later with confidence and provenance.

Ideas must not be silently promoted into facts, decisions, or canon. A later adopted decision can reference the earlier idea while preserving the evolution trail. Rejected or superseded ideas may remain searchable because they can become useful again.

## 11.2 Knowledge Web and Idea Web

The Knowledge Web captures concepts, established facts, decisions, procedures, evidence, and relationships. The Idea Web captures tentative possibilities, inspirations, hypotheses, creative fragments, proposals, and unrealized opportunities. The two webs are linked: ideas may become experiments, decisions, artifacts, canon, or rejected alternatives; knowledge may inspire new ideas.

Rikuri should be able to answer not only “What do I know about this?” but also “What ideas have I had about this?”, “Which ideas were never tested?”, “What did this idea become?”, and “What old idea is newly feasible because my tools/models changed?”

> **Search example**
>
> Query: “KV cache”. Desired response categories: Knowledge, Projects, Conversations, Code references, Experiments, Research papers, Decisions, Open tasks. Rikuri should know that a project such as Lean_llama.cpp exists on another owner device even when its full source is not cloned locally.

# 12. Model, Runtime, and Harness Strategy

## 12.1 Execution layers

Rikuri separates **model**, **inference runtime**, **agent harness**, **Rikuri Harness Profile**, **skills**, and **security envelope**. These are replaceable independently.

| Layer | Primary responsibility | Examples |
|---|---|---|
| Model | Intelligence/checkpoint | Qwen-, DeepSeek-, Gemma-, Llama-class local models; frontier models |
| Inference runtime | Load/serve model, tokenizer/chat template, sampling, structured output | llama.cpp, vLLM, Ollama, LM Studio/llmster |
| Agent harness | Persistent agent loop, tools, session state, task execution | **Pi by default for governed local/open work** |
| Rikuri Harness Profile | Model-family/tool/context/planning/edit/subagent tuning | Qwen-family, DeepSeek-family, small-model, coding profile |
| Skill/domain pack | Task-specific procedures and verification | coding, Linux, research, Bitcoin, writing |
| Security/execution envelope | Capabilities, privacy, project trust, network, filesystem, budget | AISecurity/Rikuri Security |

**Default rule:** Pi + Rikuri profiles is the primary governed harness platform. Qwen Code, DeepSeek Harness, Hermes, and future open harnesses are inspected as research/reference implementations and benchmarked as optional adapters. Rikuri may route to one only when measured benefit justifies the extra integration surface.

## 12.2 Two compatibility questions

A newly released local model is accepted only after answering separately:

1. **Runtime compatibility:** can the selected inference runtime execute the architecture correctly and efficiently?
2. **Agent compatibility:** can Pi/Rikuri use the model reliably for tool calling, context management, planning, editing, verification, and task autonomy?

The compatibility registry records both. The ordinary user sees a simple status such as `Optimized`, `Compatible`, `Experimental`, or `Unavailable`; expert detail remains inspectable.

## 12.3 Execution profile

```text
model + model revision/quantization
+ inference runtime/version/settings
+ harness (Pi default) + Harness Profile
+ skills/tools
+ machine/node
+ access path/provider
+ privacy/project-trust/security envelope
+ budget/time/retry limits
+ verification plan
```

The router selects the entire profile rather than a model name alone.

## 12.4 Privacy-preserving hybrid execution

A frontier model may be used without receiving the entire sensitive workspace. Local processing can inspect the real project, reduce the problem to a sanitized/minimal reproduction, send only policy-permitted context to the frontier model, and apply the reasoning locally. “No training” and “zero retention” remain provider properties; Local Only means raw protected content never leaves the owner-controlled execution boundary.

# 13. Local Model Intelligence

Inference systems such as Ollama, LM Studio/llmster, llama.cpp, vLLM, MLX, or future engines provide execution. Pi provides the default agent loop. Rikuri provides the model-aware behavior, security, evaluation, and product integration around both.

| Layer | Responsibility |
|---|---|
| Runtime adapter | Architecture support, tokenizer/chat template, model serving, structured-output/tool-call transport |
| Pi harness adapter | Session/agent loop, tool dispatch, persistent agent execution |
| Rikuri Harness Profile | Model-family behavior, context policy, planning/edit strategy, allowed tool complexity, subagent policy, known failure modes |
| Coding/domain profile | Repository workflow, language/build/test procedure, verification |
| Model Lab | Compare exact model+runtime+harness-profile+hardware identities and learn bounded recommendations |

## 13.1 Model-family profile composition

Profiles should compose rather than duplicate:

```text
Base Coding
+ model family (Qwen/DeepSeek/...)
+ size/capability override
+ runtime profile
+ hardware profile
+ privacy/project-trust profile
+ language/project profile
```

A new related checkpoint should first reuse the existing family profile. Only measured incompatibility creates an override. Core Pi/Rikuri harness code changes are the last resort.

## 13.2 Open-harness learning loop

When a model developer updates an open harness, Rikuri development tooling may diff the prompts/tool schemas/agent-loop/context strategy, identify model-specific changes, and evaluate those techniques against current profiles. Borrow techniques; do not inherit an upstream harness as an unconditional dependency.

## 13.3 Automatic parameter and runtime tuning

Runtime/model settings remain at safe developer defaults until evidence shows repeatable task-specific benefit. Exact model revision + quantization + runtime + settings + Harness Profile + hardware form the evaluated execution identity.

## 13.4 Background evaluator daemon

Evaluation runs under resource policy, prefers idle capacity, pauses for heat/battery/interactive load, and treats real verified jobs as evidence. Model/runtime/harness-profile changes trigger selective revalidation.

# 14. Machine Intelligence and Execution

Rikuri must maintain a machine-specific knowledge layer so AI workers can act with relevant local facts rather than repeatedly interrogating the user. Machine context is dynamic and should be retrieved on demand rather than injected wholesale into every prompt.

User: “Bluetooth stopped working again.”  
-\> Identify target machine  
-\> Retrieve OS/kernel + Bluetooth hardware + recent upgrades  
-\> Query service/rfkill/driver/log state  
-\> Diagnose  
-\> Prefer known validated remedy if matching prior incident  
-\> Execute under security envelope  
-\> Verify controller/service state  
-\> Record incident/remedy if useful

Known successful novel repairs may graduate into tested Rikuri skills/runbooks. For high-consequence systems, the AI should remain a diagnostician/planner while trusted deterministic code retains execution authority.

# 15. Model Economics and Usage Transparency

AI usage economics must be visible and continuously optimized. Rikuri should maintain a model/provider price catalog, a per-task usage ledger, subscription/quota awareness, local inference estimates, and empirical quality/success metrics.

| **Metric**          | **Requirement**                                                                                                  |
|---------------------|------------------------------------------------------------------------------------------------------------------|
| Current execution   | Show model, runtime, access path, and why it was selected when expanded.                                         |
| Tokens              | Input, output, cached, reasoning, and other provider-exposed token classes.                                      |
| Actual API cost     | Record provider/broker reported cost when available; do not replace with estimates.                              |
| Subscription usage  | Track quota/allowance when programmatically exposed; otherwise label status as unavailable.                      |
| Equivalent API cost | Optional estimate of what subscription-backed work would have cost via API; clearly labeled as not actual spend. |
| Local cost          | Show \$0 API cost plus optional estimated electricity/runtime/hardware accounting.                               |
| Task economics      | Measure cost per successful verified task, retries, latency, and quality—not token price alone.                  |
| Advisor             | Recommend better model sibling/reasoning level/access path based on recent task performance and prices.          |
| Budget control      | Allow per-task/project/day/month budgets and Economy/Auto/Best/Private routing modes.                            |
| Team economics      | Estimate and measure the marginal value of one agent vs specialist team vs swarm, including retries, evaluator cost, latency, and verified success. |

> **Router separation**
>
> Rikuri should distinguish model selection from provider/backend selection. A broker such as OpenRouter may be useful for API provider routing, pricing, fallback, and accounting, but Rikuri remains the higher-level router across native subscriptions, APIs, brokers, local models, tools, and machines.

# 16. Personal Mesh, Sync, and Backup

Rikuri distinguishes **remote control** from **state replication**. A user should be able to securely reach a home Rikuri Node before the more complex Personal Mesh replication system exists.

## 16.1 Rikuri Link — the last mile

A phone, tablet, laptop, or another computer acts as a thin client to the user's own Rikuri Node. The node retains persistent agents, files, credentials, memory, tools, project state, and local models whenever practical. Remote clients emphasize chat/voice, task status, approvals, file/photo handoff, notifications, and—on larger devices—the full Workspace.

Initial transport is **Tailscale/WireGuard-based private connectivity**, requiring no public port forwarding. Later Rikuri gains its own device identity plus direct/rendezvous/end-to-end encrypted relay path; Tailscale remains an available fallback connector.

Remote access does not mean cloud ownership: a frontier model may still be called for reasoning, but the persistent working environment remains on the user's node and only policy-permitted context is sent outward.

## 16.2 Personal Mesh replication

Selected canonical state may later replicate among owner devices with an optional hub. Devices remain usable offline. Stable identities, event/version semantics, encryption, device revocation, key rotation, conflict handling, and backup remain distinct from the remote-control channel.

## 16.3 Data-specific sync policy

- knowledge/decisions/project metadata: structured/event sync;
- conversation archive: append-only/versioned sync;
- source code: Git/existing source control;
- documents/assets: policy-scoped object/file sync;
- large models/datasets: location-aware catalog, not automatic sync;
- indexes/caches: rebuild locally;
- machine state: device-specific;
- secrets: never copied merely because knowledge is shared.

## 16.4 Backup is separate from sync

Synchronization propagates current state; backup preserves recoverable history. Deletion sync must not silently destroy the user's recovery history, while managed restore must respect cryptographic forgetting semantics.

# 17. Portability and Deployment

Rikuri is cross-platform at the product level while using a deliberate implementation order.

| Priority | Platform | Role |
|---|---|---|
| Tier 1 reference | Ubuntu LTS | Primary development/reference platform for agent execution, sandboxing, local models, Docker/CUDA, headless services, and Rikuri Link |
| Tier 1 continuous | macOS | Continuous core validation and native adapter target; reuse appropriate AISecurity Rust/Swift work |
| Tier 1 later implementation | Windows 11 | Full adapter after Linux/macOS prove the OS Capability API |
| Optional enhanced | Omarchy/other Linux | Rich adapter when installed; never a required base |

The core exposes an **OS Capability API** rather than teaching every model three operating systems. Normalized calls such as `service.status`, `package.install`, `process.inspect`, `sandbox.create`, or `window.move` map to the appropriate platform adapter and capability policy.

Deployment modes remain standalone local, owner multi-node/Link, Personal Mesh/hub, and later business/server. Canonical data formats and identities remain portable across operating systems.

# 18. Observability, Health, and Recovery

| **Area**         | **Required health evidence**                                                       |
|------------------|------------------------------------------------------------------------------------|
| Core services    | Process/service health, last heartbeat, queue/checkpoint status, errors            |
| Security modules | Per-module heartbeat, last success, enforcement state, event count, coverage       |
| Sync/mesh        | Per-device last sync, pending operations, conflict state, hub reachability         |
| Model providers  | Availability, auth state, quota/cost telemetry availability, recent errors/latency |
| Local inference  | Loaded runtime availability, model health, memory/resource pressure                |
| Automations      | Last run, next run, result, retries, waiting/blocked reason                        |
| Projects/jobs    | Current state, last verified outcome, checkpoint, responsible worker               |

The normal UI should summarize these as simple Healthy / Degraded / Critical states. Detailed diagnostics must be available without being constantly visible.

# 19. Data and State Model

## 19.1 “Single source of truth” means declared authority

Rikuri must not require all data to live physically in one database. Instead, every durable object/resource type must have a declared authoritative representation and stable identity. Examples: Git is authoritative for source code; the original photo/audio/video file is authoritative for media; the raw conversation archive is authoritative for historical chat; Rikuri structured state is authoritative for project status, idea lifecycle, governance, relationships, and event history. Human-readable Markdown may be authoritative for selected user notes or a generated one-way projection of Rikuri records. User-note imports retain captured source versions; edited projections are preserved/flagged and never automatically merged back under KNOW-005/011 and Architecture §20.3.1.

If a derived index disagrees with the authoritative live resource, the authoritative resource wins. High-impact actions must resolve indexed references to the current authoritative object before execution.

## 19.2 Stable IDs and strong-core/flexible-extension schema

Every durable Rikuri-managed entity—user, device, project, idea, knowledge item, conversation, task, decision, artifact, model profile, agent session, event, etc.—must receive a stable identifier independent of file path, title, provider, or device. Renaming or moving a resource must not break its conceptual history.

The canonical schema should keep a strong stable core for identity, ownership, timestamps, provenance, version, authority, permissions, lifecycle state, relationships, and events while permitting flexible domain metadata and future extension without redesigning the entire store.

## 19.3 Canonical vs derived

| **Canonical / durable**                | **Derived / rebuildable**         |
|----------------------------------------|-----------------------------------|
| User/workspace identity and policies   | Vector embeddings                 |
| Projects and explicit current state    | Semantic/lexical indexes          |
| Event ledger and decisions             | Knowledge-graph projection caches |
| Raw conversation/source archive        | Context summaries                 |
| Normalized knowledge and relationships | Model recommendation caches       |
| Skills and runbooks                    | Search caches                     |
| Original documents/code locations      | UI thumbnails/previews            |
| Sharing scopes and resource registry   | Transient worker scratch data     |

## 19.4 Core entities

- User / Workspace

- Locale / Language Profile

- Device / Machine

- Project

- Task / Job

- Responsibility / Routine

- Conversation / Source

- Knowledge Item / Entity

- Idea / Hypothesis / Inspiration

- Decision

- Skill / Runbook

- Artifact / Resource

- Workspace / Window / Layout / Link Group

- Model

- Runtime / Harness

- Tool / Capability

- Execution Event

- Security Decision

- Usage / Cost Record

- Sync Operation / Version

# 20. Domain Workspace Strategy

Rikuri should not build separate “mini-products” with independent memory for every profession. Coding, writing, research, study, marketing, product development, media creation, business planning, trading, and future specialties should reuse the same canonical primitives and add domain-specific windows, skills, evaluators, and knowledge projections.

## 20.1 Coding as a supporting workspace specialization

Coding uses source files, Git branches/worktrees, diffs, tests, terminals, browsers, previews, code indexes, and coding-specialist agents. The user may observe parallel work through linked windows while Rikuri manages isolation, integration, and review under the common project/event/security model.

## 20.2 Writing as the lead creative workspace

Writing uses manuscripts/reports/scripts, comments/revisions, research sources, idea cards, style guidance, structured outlines, and optional domain canon. Rikuri should initially integrate mature open or automatable editors rather than recreate a full office suite solely for ownership. A later Rikuri-native editor may provide deeper semantic structure while preserving import/export to common user-owned formats.

For novels and other long-form work, optional structures may include characters, locations, timeline, scenes/chapters, plot relationships, world rules, voice/style guidance, and **Canon** distinct from speculative ideas. Continuity agents may detect contradictions, while semantic/narrative refactoring proposes coherent changes across affected passages rather than performing blind global replacement.

## 20.3 Research, study, marketing, product, and creator work

The same workspace and Knowledge/Idea Fabric should support parallel research, hypothesis comparison, lecture/media notes, campaign concepts, product ideas, business opportunities, YouTube/TikTok scripts, design inspiration, and other creative or analytical work. Domain templates may change the default windows and evaluators but must not create isolated knowledge silos.

# 21. Business and Multi-User Readiness

The personal product should not be burdened by enterprise UI, but its core boundaries should avoid blocking later business use.

- Separate user, device, worker, project, and organization identities.

- Role- and capability-based access rather than assuming one omnipotent agent.

- Project/organizational memory distinct from private personal memory.

- Auditability for actions, data access, model/provider use, and approvals.

- Policy-driven data residency/provider/privacy constraints.

- Secret management independent of model prompts and conversation memory.

- Administrative cost/budget controls and model allow/deny policies.

- Export and deletion boundaries compatible with organizational governance.

- Per-user and organization-default locale/language policy so multilingual teams can share canonical knowledge without forcing one interface or working language.


Business/team deployments must preserve the same user-alignment principles while introducing organization policy, role-based authority, separation of personal and organization knowledge, audit, approval delegation, and administrator controls. Enterprise connectors such as Slack, Teams, Asana, or project systems should be discoverable when relevant but invisible to users who do not use them.

## 21.1 Shared human–AI project conversations

September 28, 2026 — adopted post-Foundation scope. Product Stage 6 brings forward minimum membership, sharing and audit controls from Stage 9 so a small team can work with agents on one authoritative Rikuri node. It requires Stage 3 Link, Stage 5 source/contribution controls and Stage 6 governance first; Stage 9 retains enterprise tenancy/admin and Windows assurance. Stage 7 replication is optional. [Architecture §38.6](Rikuri_Pi_Architecture_Spec_v1.7.md#386-shared-humanai-project-conversations) owns the contract and [Master Build Sequence](Rikuri_Pi_Master_Build_Sequence_v1.1.md#stage-6--multi-agent-governance--project-graph) owns order. No shared-conversation runtime or security outcome is proven by these requirements.

| ID | Pri | Requirement | Description | Acceptance |
|---|---|---|---|---|
| COLLAB-001 | P1 | Human and agent membership | Use separate human, device and worker identities, scoped expiring invitations, acceptance, member/guest roles, removal and explicit history grants. | Joining permits only granted conversation operations; guest membership cannot approve actions, invoke shell tools or read unrelated projects. |
| COLLAB-002 | P1 | Shared project objects | Persist conversation events referencing artifacts, tasks, source revisions and decisions; preserve authenticated authorship and revision history through governed services. | Humans and agents can discuss the same object; conversational claims cannot mutate accepted decisions or verified task status without authorized transitions. |
| COLLAB-003 | P1 | Audience-safe knowledge | Admit retrieval and posts against source policy and the conversation audience, including history readers; personal/agent-private context remains separately scoped. | Restricted content, metadata, previews and citations cannot leak through summaries, posts or history. New members receive only admitted history; source or membership changes invalidate pending delivery. |
| COLLAB-004 | P1 | Controlled agent participation | Bind mention/quiet/automatic participation to explicit conversation scope, sources, audience, triggers, budgets and revocation, using the Attention Firewall. | Authorized useful contributions do not require repeated permission; redundant bursts, stale topics and unsupported conclusions do not become public agent chatter. |
| COLLAB-005 | P1 | Authenticated LAN and remote collaboration | Apply the same application permissions over proven encrypted Link paths; Tailscale first, native direct/rendezvous/opaque relay later with Tailscale fallback. | A LAN and an outside participant share an authorized conversation. Network location and fallback do not expand access or weaken encryption; the node's plaintext processing and separate model disclosures are explicit. |
| COLLAB-006 | P1 | Durable delivery and revocation | Use stable event IDs, acknowledgements, replay-safe effects and membership/device revalidation on reconnect; invalidate revoked subscriptions, pending posts and context. | Disconnect/retry cannot duplicate posts/actions; removed members cannot obtain new delivery or unauthorized history. Node outage is visible; already disclosed external copies are outside recall guarantees. |

Stage 6's initial slice is text/file collaboration and evidence cards over supported sources. Rich image/audio/video extraction and explicitly activated meeting capture follow Stage 8. A voice instruction channel does not imply a live group voice/video conferencing implementation.

# 22. Success Metrics

| **Metric**            | **Target / intent**                                                                                                                             |
|-----------------------|-------------------------------------------------------------------------------------------------------------------------------------------------|
| Continuity            | A different model/runtime can continue an active project with no user restatement of recorded state in ≥95% of tested handoffs.                 |
| Knowledge retrieval   | Top-level search finds the correct project/chat/note/resource for representative known-item queries in ≥95% of benchmark cases.                 |
| Distillation quality  | Human review finds ≥90% of important decisions/ideas/questions correctly classified, with low false promotion of brainstorming to decisions.    |
| Execution reliability | For supported repair/task benchmarks, ≥90% of completed executions satisfy independent post-action verification without manual command editing. |
| Cost efficiency       | Automatic routing reduces spend versus “always strongest model” baseline while maintaining target verified success rate.                        |
| Security qualification | Every enabled sensitive path has current positive, denial, failure and recovery evidence; zero known boundary violations in the required release suite. Report suite/platform/version coverage and limitations, not a universal safety percentage. |
| Security health       | Required security modules report measurable heartbeat/coverage; no “Protected” status while mandatory protection is known degraded.             |
| Portability           | Restore/migrate a test workspace to another supported OS/device and recover canonical knowledge/projects without manual reconstruction.         |
| Mesh reliability      | Selected shared state converges after offline concurrent edits with no silent data loss in test scenarios.                                      |
| UX simplicity         | Common user flows—ask, find project, review cost, see health, share knowledge—require no infrastructure terminology.                            |
| Assistant usefulness  | A newly configured user can triage email and coordinate a calendar request without creating agents/workflows or understanding integration plumbing. |
| Attention quality     | No unauthorized proactive calls/messages/recurring jobs; routine multi-agent activity is aggregated with low nuisance-interruption rate.          |
| Governance quality    | Coordinator/evaluator tests detect strategy drift, unsupported conclusions, or missed escalation in defined benchmark scenarios.                 |
| Model Lab usefulness  | Newly added compatible models receive a baseline profile and their routing recommendations improve measurably from real-task outcomes.             |
| Privacy enforcement   | Local Only/Secret test data cannot leave through supported model, browser, connector, upload, shell/network, or GUI egress paths.                  |
| Internationalization  | Core workflows pass localization/format/layout tests in all seven initial language targets; cross-lingual search retrieves representative known items without altering canonical originals. |
| Collaborative uplift (post-Foundation) | On selected tasks, evidence shows that Rikuri-assisted framing, method selection, reflection, or learned workflow support improves verified outcomes or reduces repeated rework versus the user's/model's prior baseline without unacceptable cost or nuisance. |
| Learning calibration (post-Foundation) | Learned heuristics retain support/counterexamples and are downgraded, scoped, or retired when conditions change; the system reports insufficient evidence rather than inventing confidence. |
| Discovery usefulness (research track) | Candidate questions/Lightbulbs are evaluated by later user adoption, experiment value, verified connection, or productive redirection—not raw idea count. |
| Capability expansion (research track) | Users complete tasks that previously exceeded their practical ability, judged against their own earlier attempts or a simpler baseline with task difficulty recorded. AI output volume, tokens, agent count or tool calls never stand in for it (P32, MISSION-002). |
| Independent understanding (research track) | With the user's opt-in, explanation, lessons or practice improve the user's own later performance without assistance. Measured only from evidence the user chose to share, never from hidden profiling (MISSION-006). |
| Creative realization (research track) | With the creator's opt-in, ideas are carried to completed, useful or expressive outcomes, judged by the creator and compared with work the creator declares stalled or abandoned; abandonment the creator chose is not counted as failure. |

# 23. Staged Roadmap

The build order is dependency-driven: each stage must be wired into the running system and pass its Stage Gate before dependent feature work becomes active. Interface skeletons may appear early when they prevent architectural rework; rich behavior waits for its stage.

| Stage | Scope | Exit intent |
|---|---|---|
| **0 — Trusted Core / Foundation-0** | Six gate-closed stages: engineering/kernel; ChatGPT and Markdown import, provenance, lexical search, managed backup/forget; minimal knowledge/policy/budget; Ubuntu envelope and project trust; Pi local coding and cloud/local handoff; one-window English Home. Continuous macOS core tests, native macOS assurance experimental. | One coherent, recoverable, bounded single-node proof with owner-runnable demonstrations and linked evidence. |
| **1 — Capture continuity and supporting execution** | Claude and read-only project imports; source coverage; admitted coding/tool slice and external handoff. Full IDE deferred. | Sources survive restart and permitted execution/handoff remains verified. |
| **2 — Creative Studio essentials / Sterling Workspace** | Shared creative objects, flexible linked windows, scenes/characters/world notes, requested alternatives, author-approved canon, resume/export and local plugin prototype. | S2.C1–C5 and Test Strategy §16 essentials slice; creator completes an end-to-end story exploration without a full IDE. |
| **3 — Rikuri Link / Remote Control** | Tailscale initial connector; phone/tablet/laptop thin client; node identity/presence; approvals/status/chat/voice/file handoff; later native direct/rendezvous/E2E relay with Tailscale fallback. | User can securely “phone home” to the owner-controlled Rikuri Node. |
| **4 — Model Lab + Intelligent Routing** | Dynamic discovery; compatibility registry; open-harness diff/eval; hardware fit; Auto routing; price/quota/local-cost advisor; controlled native-harness exceptions. | Routing is evidence-based across model+runtime+harness+machine, not model-name folklore. |
| **5 — Creative research, connectors and distribution** | Supported capture, research, attention controls, bounded proactive suggestions and host/plugin adapters; optional general assistant packs. | Traceable research/capture and governed distribution with visible privacy/coverage limits. |
| **6 — Multi-Agent Governance & Project Graph** | Evaluate Paperclip-like control-plane mechanisms; add coordinator/evaluator, durable DAG, Project Lifecycle Kernel and Project Graph. After Link and contribution/governance controls, deliver minimum human–AI conversation membership, sharing and audit brought forward from Stage 9. | A governed team works against durable ProjectState; two humans on LAN/remote clients and an agent share authorized evidence, reconnect without duplicates and enforce revocation. |
| **7 — Personal Mesh & Persistent Responsibilities** | Encrypted replication, optional hub, cross-device resource catalog, durable routines, quiet digests, learned skills. | One Personal Intelligence spans owner devices without sync becoming canonical authority. |
| **8 — Advanced creative domains and optional developer pack** | Deeper world/character systems, science-backed scenarios, media, games/animation, marketing; optional full IDE stack in original internal order. | Domain-specific validation reuses trusted services; essentials do not depend on these packs. |
| **9 — Business/Organization Readiness** | Tenancy, organizational memory, role policy, audit/admin, shared projects, enterprise deployment. | Institutional use without weakening personal/user-alignment foundations. |
| **10 — Collaborative Intelligence / Inquiry & Discovery** | Experience Intelligence; selective reflection and consolidation; pattern abstraction; cognitive recipes; champion/challenger optionality; bounded curiosity; Question/Frame generation; Cognitive Ensembles; Lightbulb lifecycle; incubation sessions; domain-specific discovery labs; cautious adaptive routing/learning. Research prototypes may begin earlier once product Stage 4 (Model Lab) and product Stage 6 (multi-agent governance) evidence/governance prerequisites exist, but this stage is not a Foundation dependency. | Rikuri can demonstrate reproducible, evidence-backed improvement in how the user + models + tools work together and can surface useful non-obvious questions/connections without silently converting speculation into truth or preventing project completion. |

> **Stage rule:** no downstream stage is declared active merely because its calendar date arrived. The prior dependency gate must be green, or the checklist records an explicit DEFERRED/RESEQUENCED decision with reason, impact, new position, owner, and acceptance conditions.

# 24. Risks and Mitigations

| **Risk**                                     | **Mitigation**                                                                                                               |
|----------------------------------------------|------------------------------------------------------------------------------------------------------------------------------|
| Over-engineering before usefulness           | Ship a coherent single-device core first; preserve interfaces for later mesh/business complexity.                            |
| Dependency lock-in                           | Wrap external harnesses, routers, runtimes, and indexes behind Rikuri-owned interfaces; canonical state remains independent. |
| Memory pollution / false facts               | Classify provenance/confidence/status; preserve source; require user/verification pathways for important decisions.          |
| Local-model regressions                      | Version model profiles and evals; do not assume a new model/fine-tune is better because it is newer.                         |
| Agent security bypass                        | Make security boundary mandatory below runtime selection; capability enforcement must not depend solely on model compliance. |
| Approval fatigue                             | Use risk classes and bounded authority; surface intent/consequence rather than routine low-risk command prompts.             |
| Sync corruption/conflicts                    | Local-first event/operation model, immutable/versioned history, semantic conflict review, backup separate from sync.         |
| Cost optimization harms quality              | Optimize cost per verified success with quality floors and automatic escalation; user can select Best mode.                  |
| Provider changes / subscription restrictions | Maintain multiple access paths and explicit capability/entitlement metadata; avoid undocumented scraping as a foundation.    |
| Index becomes mistaken source of truth       | Always resolve to authoritative current file/repo/system state before modifying or making high-stakes claims.                |
| Coordinator monoculture / blind spot           | Use replaceable coordinators, diverse Strategy Council when warranted, independent evaluation, evidence-first adjudication, and human decision packets. |
| Agent swarm waste / complexity                | Start with one capable agent; scale only when parallelism/specialization/economics justify it; measure cost per verified success.                       |
| Surprise autonomy / attention overload        | Reactive default, explicit responsibility grants, central Attention Firewall, digest aggregation, calls off by default.                               |
| Sensitive-data exfiltration                   | Privacy classification before context assembly, universal egress enforcement, secret broker, browser isolation, local-only processing, audit.        |
| Malicious/tampered open models                | Exact revision/hash identity, provenance/signature when available, safe formats, quarantine, dependency/custom-code review, sandboxed evaluation.     |
| Self-improvement destabilizes core            | Separate adaptive learned state from protected core; require developer-controlled testing/versioning/deployment for core changes.                      |
| Localization drift / language silos           | Externalize locale resources, preserve original-language canonical data, version translations, test cross-language retrieval, and include locale competence in model profiles. |
| Workspace becomes intimidating                 | Keep Home/Quick Assist minimal; make workspace optional; provide templates, natural-language layout control, and progressive disclosure.               |
| Window/agent visual clutter                     | Work-centric status, collapsible controls, attention indicators, saved layouts, and graph summaries rather than exposing every worker by default.       |
| Ideas become unmanageable                       | Capture freely but classify/link/dedupe in background; distinguish Idea from Decision/Canon; surface organization views only when useful.               |
| Multiple stores drift                           | Declare authority per object, use stable IDs/events, treat indexes/embeddings/graph projections as derived, and resolve live resources before action.    |
| Domain workspaces fork the product              | Reuse common project/window/link/knowledge/event/security primitives; domain features are projections and skills, not new state silos.                  |
| Untrusted repository executes code during inspection | Project starts read-only/untrusted; inspect executable metadata; sandbox descendants; explicit scoped trust promotion; regression fixtures include malicious Git config/hooks/build scripts. |
| Cloud-agent convenience expands trust boundary | Keep persistent agent/workspace on owner-controlled nodes when practical; send minimal/sanitized context to frontier models; expose privacy mode simply. |

# 25. Open Technical Decisions

- Compare Paperclip at a pinned revision as a reference implementation, including license and dependency implications. Decide per mechanism whether to reuse code, adapt a pattern, or build a Rikuri-specific implementation; documentation is evidence of design, not proof of production reliability. Paperclip cannot become authority over AISecurity, the Knowledge Context Compiler, or canonical ProjectState.

The following are intentionally not frozen by this PRD. They should be evaluated against the requirements rather than selected because they are fashionable.

- Desktop shell/UI framework (e.g., Tauri-class vs alternatives).

- Personal-store implementation and SQLite/PostgreSQL boundary.

- Canonical knowledge schema and event-log/CRDT strategy.

- Universal search implementation: lexical, vector, code/symbol, graph, reranking.

- Exact Pi integration/API boundary, extension/package strategy, and the criteria for permitting a native-harness exception. Pi is the default harness substrate; Qwen/DeepSeek/Hermes-class harnesses are references/benchmarks unless evidence justifies an adapter.

- Native frontier runtime integration strategy and entitlement/auth handling.

- Linux and Windows sandbox/enforcement primitives; macOS entitlement-dependent enhancements.

- Rikuri Link native direct/rendezvous/relay protocol and device identity design after the initial Tailscale connector; Personal Mesh replication remains a separate protocol problem.

- Implementation primitives for safe preservation of externally edited Markdown projections, comparison/recovery UI, and separately authorized revision proposals under Architecture §20.3.1. Two one-way flows are settled; automatic bidirectional merge is excluded.

- Durable workflow/checkpoint engine choice for long-running responsibilities.

- Model-provider price catalog ingestion and subscription quota telemetry strategy.

- Hardware-cost estimation policy for local inference.

- Business multi-tenancy and organization boundary implementation.


- Constitution representation, amendment/versioning workflow, and relationship to policy-as-code.

- Coordinator/evaluator routing strategy and management scorecard metrics.

- Strategy Council trigger policy, diversity criteria, adjudication method, and scenario-packet schema.

- Attention Firewall implementation and cross-device notification/voice/telephony policy.

- Email/calendar/contact connector architecture and just-in-time connection UX across Google/Microsoft/Apple ecosystems.

- Audio/video/lecture capture architecture and local-first transcription/diarization/privacy controls.

- Universal egress enforcement strategy across browser, shell, GUI/computer use, plugins/connectors, custom code, and network layers.

- Secret broker / credential proxy architecture and financial/crypto signing boundary.

- Trusted model-registry sources, provenance/signature policy, artifact hashing/quarantine, and model-license policy.

- Per-modality Model Lab evaluation suites and background resource scheduler.

- Localization framework/resource format, translation workflow, fallback rules, terminology/glossary management, and community/developer contribution process for future language packs.

- Multilingual embedding/reranking strategy, cross-lingual search quality thresholds, language detection, and per-language Model Lab evaluation suites.

- Multilingual speech-recognition/synthesis providers and local-vs-cloud privacy policy for voice in each language.

- Sterling-style Workspace implementation: internal/freeform window manager, native detach, multi-monitor/macOS Spaces behavior, persistence, accessibility, GPU/rendering cost, and status aggregation.

- Project Graph representation, layout algorithm, incremental event updates, collapse/expand semantics, and mapping between graph nodes and work windows.

- Context/Agent/Prompt Link protocol, lock/follow semantics, broadcast isolation, and state handoff when a window switches models.

- Rikuri Plugin API capability model, sandbox/permission lifecycle, provenance/pinning/update/rollback, and selective VS Code API compatibility boundary.

- OS Capability API implementation details for Ubuntu/macOS/Windows, including privilege brokerage and window-control boundaries.

- Rich-text/writing editor strategy: integrate an open embeddable editor, automate LibreOffice/ONLYOFFICE-class tools, build a Rikuri-native structured editor later, or combine approaches.

- Idea capture/share-sheet/mobile architecture, multimodal transcription/classification, handwriting extraction, and privacy-preserving local preprocessing.

- Canonical structured-store schema and authority registry: which entity types are authoritative in SQL/event state, Markdown, Git, original files, or external live systems.

# 26. Definition of Foundation Release

The Foundation Release is a **later product milestone**, distinct from the narrower Foundation-0 technical release defined by the Stage Plan. It remains intentionally smaller than the complete vision. The items below retain the broader product promise, including voice, meaning-based retrieval, fuller knowledge distillation, attention policy, and seven-language support; they are not all June 2027 gates. Foundation-0 closes only on W1 and F1–F10 as narrowed in ADR 0000 and the live Stage Plan.

1. A user installs a blank Rikuri instance and receives the simple Home experience: one text/voice intent surface plus a small capability launcher, without configuring agent frameworks or editing prompts.

2. Core durable entities use stable IDs and declared authority; canonical state can be backed up/exported and restored to a fresh installation without reconstructing knowledge manually.

3. The user can capture a text or voice idea without filing it, later retrieve it by meaning, inspect its original/provenance, and link it to a project while keeping Idea distinct from Decision/Fact/Canon.

4. Rikuri imports or captures conversations and distills them into searchable topic knowledge with source provenance and human-readable Markdown/Obsidian output.

5. Rikuri discovers/registers at least one live project/resource and can find it by concept, then resolve the current authoritative source before read/review or supported modification.

6. The user connects at least one frontier model/access path and one local-model runtime; Rikuri can hand a project/task between execution profiles without asking the user to restate recorded context, while privacy mode remains enforceable.

7. A local model executes through Pi using a Rikuri Harness Profile; runtime compatibility and agent compatibility are recorded separately, and a family-profile override can be changed without modifying the core harness.

8. One coding/project-assistance vertical slice opens an untrusted project read-only, explicitly promotes execution trust, uses Pi to inspect/edit/test within the Ubuntu envelope, and verifies the resulting diff/tests.

9. Attention policy prevents unauthorized proactive calls/messages/recurring responsibilities and consolidates routine status rather than exposing agent chatter.

10. At least one low-risk machine/tool task passes through the common security boundary and is independently verified against the intended outcome.

11. Local Only and Secret test data demonstrate enforced privacy behavior in the supported Foundation paths: Local Only raw content cannot be sent externally, and Secret test credentials are never placed in model context.

12. AISecurity-derived health reporting proves required protection modules are healthy or shows Degraded rather than silently claiming protection.

13. Rikuri records per-task model/runtime/cost metadata sufficient to explain which execution profile was used and to support later cost-per-success optimization.

14. The event ledger can reconstruct recent project activity and support continuation after restart or worker failure.

15. The core interface can be switched among English, Spanish, Simplified Chinese, Traditional Chinese (Taiwan), Japanese, French, and German; representative Home, Workspace, approval, and search flows render correctly, and a cross-lingual query can retrieve relevant supported-language source material without rewriting the canonical original.

> **Not required for the Foundation Release**
>
> The full floating/tiled Workspace, Project Graph, multi-monitor detachment, complete Compare Mode, production Rikuri Link mobile client/native relay, production multi-device mesh, hundreds-of-agent swarms, mature writing/story-canon environment, complete travel/purchase automation, all telephony surfaces, advanced multilingual voice/media coverage, bounded autonomous extension updates, business tenancy, or complete macOS/Linux/Windows enforcement parity may ship in later stages. The foundation must make them additive rather than requiring a replacement of canonical data, identity, language architecture, governance, security, routing, or event semantics.

# Appendix A. Priority Requirement Matrix

P0 = foundational/required for the corresponding product release; P1 = important next capability; P2 = later enhancement. Priority is not a Foundation-0 delivery date: the live Stage Plan and ADR 0000 define the smaller Foundation-0 acceptance subset. The detailed requirements above remain the broader product baseline; this matrix is a compact index.

| **ID**    | **Pri** | **Requirement**                    |
|-----------|---------|------------------------------------|
| CORE-001  | P0      | Persistent project state           |
| CORE-002  | P0      | Append-only event ledger           |
| CORE-003  | P0      | Capability planner                 |
| CORE-004  | P0      | Durable jobs                       |
| CORE-005  | P1      | Persistent responsibilities        |
| CORE-006  | P0      | Transactional State Evolution      |
| CORE-007  | P0      | No silent loss or false-green status |
| MEM-001   | P0      | Context compiler                   |
| MEM-002   | P0      | Memory classes                     |
| MEM-003   | P0      | Provenance                         |
| MEM-004   | P0      | Temporal state                     |
| MEM-005   | P1      | Learned procedures                 |
| MEM-006   | P0      | Knowledge Context Compiler        |
| MEM-007   | P0      | Knowledge discovery on arrival    |
| MEM-008   | P0      | Retrieval at topic boundaries     |
| MEM-009   | P0      | Versioned task packet and handoff |
| MEM-010   | P0      | Source freshness and invalidation |
| MEM-011   | P0      | Cross-media and coding provenance |
| MEM-012   | P0      | Context delivery evidence         |
| KNOW-001  | P0      | Conversation archive               |
| KNOW-002  | P0      | Topic segmentation                 |
| KNOW-003  | P0      | Statement classification           |
| KNOW-004  | P0      | Entity resolution and dedupe       |
| KNOW-005  | P0      | Obsidian projection                |
| KNOW-006  | P1      | Semantic knowledge graph           |
| KNOW-007  | P1      | Thought trail                      |
| KNOW-008  | P1      | Capture without prior organization |
| KNOW-009  | P1      | Correctable derived classification |
| KNOW-010  | P1      | Conversational evidence contributions |
| KNOW-011  | P0      | Recoverable document revisions |
| PROJ-001  | P0      | Project registry                   |
| PROJ-002  | P0      | Live resource resolver             |
| PROJ-003  | P0      | Universal search                   |
| PROJ-004  | P1      | Code-aware indexing                |
| PROJ-005  | P1      | Resource catalog                   |
| ROUTE-001 | P0      | Model-independent state            |
| ROUTE-002 | P0      | Native-runtime preservation        |
| ROUTE-003 | P0      | Native-harness exception path      |
| ROUTE-004 | P0      | Access-path routing                |
| ROUTE-005 | P1      | Automatic escalation/de-escalation |
| ROUTE-006 | P0      | Simple privacy/execution modes     |
| ROUTE-007 | P0      | Harness compatibility registry     |
| LOCAL-001 | P0      | Model discovery                    |
| LOCAL-002 | P0      | Model profile                      |
| LOCAL-003 | P0      | Pi-based coding behavior           |
| LOCAL-004 | P1      | Open-harness learning loop         |
| LOCAL-005 | P1      | Evaluation suite                   |
| LOCAL-006 | P1      | Adaptive profiles                  |
| LOCAL-007 | P1      | Constrained output                 |
| MACH-001  | P0      | Machine profile                    |
| MACH-002  | P0      | Live state retrieval               |
| MACH-003  | P0      | Observe-diagnose-act-verify loop   |
| MACH-004  | P0      | OS Capability API                  |
| MACH-005  | P0      | Ubuntu reference adapter           |
| MACH-006  | P1      | Incident/remedy memory             |
| MACH-007  | P1      | Multi-machine routing              |
| TOOL-001  | P0      | Tool hierarchy                     |
| TOOL-002  | P0      | Skill packages                     |
| TOOL-003  | P1      | Teach by demonstration             |
| TOOL-004  | P1      | Scheduled/event routines           |
| TOOL-005  | P1      | Worker coordination                |
| SEC-001   | P0      | Common security boundary           |
| SEC-002   | P0      | Capability envelopes               |
| SEC-003   | P0      | Risk-class execution               |
| SEC-004   | P0      | Validated runbooks for high risk   |
| SEC-005   | P0      | Financial safety boundary          |
| SEC-006   | P0      | Security health supervisor         |
| SEC-007   | P0      | Quiet degradation alerts           |
| SEC-008   | P0      | Audit ledger                       |
| SEC-009   | P1      | Intent consistency                 |
| SEC-010   | P1      | Transactional changes and undo     |
| SEC-011   | P0      | Project read-vs-execute trust          |
| SEC-012   | P0      | Descendant-process containment          |
| SEC-013   | P1      | Scoped autonomy presets                  |
| SEC-014   | P1      | Understandable approval settings         |
| SEC-015   | P1      | Unattended readiness and bounded progress |
| SEC-016   | P1      | Approval-aware independent work          |
| SEC-017   | P1      | Defect-scoped regression evidence        |
| SEC-018   | P1      | Grant continuity and revocation          |
| SEC-019   | P1      | Live-service and financial consequences  |
| SEC-020   | P0      | Personal-data protection duty            |
| SEC-021   | P0      | Qualified enforcement adapters           |
| SEC-022   | P0      | Brokered credentials                    |
| SEC-023   | P0      | Private-data lifecycle and integrity     |
| SEC-024   | P0      | Evidence-backed trust                    |
| SEC-028   | P0      | Provider privacy evidence                |
| SEC-029   | P1      | Private-network and egress lifecycle     |
| SEC-030   | P1      | Step-up authentication and security state |
| SEC-031   | P0      | Transactional security policy activation |
| SEC-033   | P1      | Bounded outbound conduct                 |
| SEC-034   | P1      | Connector, MCP, plugin and skill admission |
| LINK-001  | P1      | Phone-home remote access                |
| LINK-002  | P1      | Tailscale initial connector             |
| LINK-003  | P1      | Durable remote reconnect                |
| LINK-004  | P2      | Native Rikuri E2E transport             |
| LINK-005  | P2      | Multi-node execution placement          |
| COLLAB-001 | P1 | Human and agent membership |
| COLLAB-002 | P1 | Shared project objects |
| COLLAB-003 | P1 | Audience-safe knowledge |
| COLLAB-004 | P1 | Controlled agent participation |
| COLLAB-005 | P1 | Authenticated LAN and remote collaboration |
| COLLAB-006 | P1 | Durable delivery and revocation |
| DEV-001   | P1      | Monaco editor Window                    |
| DEV-002   | P1      | Native developer services               |
| DEV-003   | P1      | Rikuri Plugin/Window API                |
| DEV-004   | P1      | Verified plugin lifecycle               |
| DEV-005   | P2      | Selective VS Code API compatibility     |
| ASST-001  | P0      | Ready assistant                      |
| ASST-002  | P0      | Email triage and drafting             |
| ASST-003  | P0      | Calendar/contacts coordination        |
| ASST-004  | P1      | Meeting and lecture intelligence      |
| ASST-005  | P1      | Travel and shopping assistance        |
| ASST-006  | P1      | Natural voice surface                 |
| ATTN-001  | P0      | Attention Firewall                    |
| ATTN-002  | P0      | Reactive by default                   |
| ATTN-003  | P0      | Digest aggregation                    |
| ATTN-004  | P1      | Learned attention preference          |
| GOV-001   | P0      | Decision-rights policy                |
| GOV-002   | P0      | Replaceable coordinator               |
| GOV-003   | P1      | Automatic team sizing                 |
| GOV-004   | P1      | Strategy Council                      |
| GOV-005   | P1      | Scenario decision packet              |
| GOV-006   | P1      | Single user communication path        |
| GOV-007   | P1      | Independent coordinator evaluation    |
| GOV-008   | P1      | Durable multi-agent control plane     |
| GOV-009   | P1      | Intent-led team assembly              |
| GOV-010   | P1      | Lifecycle-specific coordination       |
| GOV-011   | P1      | Reference implementation evaluation   |
| IMP-001   | P0      | Layered governance artifacts          |
| IMP-002   | P1      | Operational learning                  |
| IMP-003   | P1      | Improvement proposals                 |
| IMP-004   | P0      | Protected-core boundary               |
| IMP-005   | P2      | Bounded extension self-update         |
| MDL-001   | P1      | Capability-driven model search        |
| MDL-002   | P1      | Hardware-fit screening                |
| MDL-003   | P1      | Evidence-ranked recommendation        |
| MDL-004   | P1      | Approved acquisition                  |
| MDL-005   | P1      | Supply-chain screening                |
| MDL-006   | P1      | Task-specific baseline evaluation     |
| MDL-007   | P1      | Background probation and ongoing eval |
| MDL-008   | P1      | Task-specific parameter profiles      |
| MDL-009   | P1      | Evaluated execution identity          |
| UI-001    | P0      | Universal intent surface               |
| UI-002    | P0      | Minimal Home launcher                  |
| UI-003    | P0      | Progressive disclosure                 |
| UI-004    | P1      | First-class work windows               |
| UI-005    | P1      | Freeform + tiled layout                |
| UI-006    | P1      | Native detach and multi-monitor        |
| UI-007    | P1      | Context Link Groups                    |
| UI-008    | P1      | Agent Link                             |
| UI-009    | P1      | Prompt Link / broadcast                |
| UI-010    | P1      | Per-window model selector              |
| UI-011    | P1      | Compare Mode                           |
| UI-012    | P1      | Project Graph                          |
| UI-013    | P1      | Live Follow                            |
| UI-014    | P1      | Saved workspace layouts                |
| UI-015    | P1      | Conversational workspace control       |
| UI-016    | P0      | Native ecosystem freedom               |
| IDEA-001  | P0      | Frictionless Idea Inbox                |
| IDEA-002  | P0      | Preserve original capture              |
| IDEA-003  | P0      | Stable idea identity and state         |
| IDEA-004  | P0      | Automatic linking without forced filing|
| IDEA-005  | P0      | Semantic and exact recovery            |
| IDEA-006  | P1      | Quiet resurfacing                      |
| IDEA-007  | P1      | Knowledge Web + Idea Web               |
| IDEA-008  | P1      | Cross-domain reuse                     |
| DOM-001   | P1      | Common workspace substrate             |
| DOM-002   | P1      | Coding workspace                       |
| DOM-003   | P1      | Writing editor integration             |
| DOM-004   | P1      | Writing assistance tools               |
| DOM-005   | P2      | Story/long-form canon                  |
| DOM-006   | P2      | Semantic/narrative refactoring         |
| DOM-007   | P2      | Parallel section work                  |
| DOM-008   | P1      | Visual idea/sticky-note board          |
| DOM-009   | P1      | Study workspace                        |

# Appendix B. Example User Journeys

## B.1 Search and continue a forgotten research thread

- User asks: “What have I done related to KV cache?”

- Rikuri searches topic knowledge, original conversations, projects, code index, decisions, experiments, and resources across the owner’s catalog.

- Rikuri returns a categorized result including a project such as Lean_llama.cpp on the Ryzen machine and relevant prior conversations/notes.

- User says: “Continue the 2-bit divergence experiment.”

- Rikuri loads current project state, resolves the live repository, selects a suitable coding/runtime profile, executes within policy, verifies tests/results, and updates project state.

## B.2 Troubleshoot a Linux Bluetooth regression

- User says: “Bluetooth stopped working again.”

- Rikuri identifies the intended machine, retrieves OS/kernel, Bluetooth hardware, recent upgrade history, prior incidents, and live service/driver state.

- If a validated prior remedy matches, Rikuri may use it under the appropriate risk policy; otherwise a frontier/local specialist investigates.

- Rikuri executes only the necessary bounded changes and verifies service/controller functionality afterward.

- If the repair is novel and repeatedly successful, Rikuri can propose converting it into a tested reusable skill/runbook.

## B.3 Cross-model project delegation

- User asks for competitor research, Python analysis, a frontend prototype, and creative assets.

- Capability Planner decomposes the goal and chooses execution profiles based on current model/runtime/tool strengths and budget.

- Research may use a superior browsing runtime; code may use a native coding harness; analysis may use Python; design may use a creative connector/computer worker.

- Every worker reads/writes the same Rikuri project state and event ledger rather than isolated vendor chat memory.

- Rikuri verifies outputs, assembles the result, records cost, and retains decisions/knowledge for later continuation.

## B.4 Three-computer Personal Mesh

- User marks Rikuri architecture knowledge as “My devices,” keeps Bitcoin machine state device-only, and keeps a large model on Ryzen only.

- Mac and Ryzen both maintain local copies of shared canonical knowledge; optional hub relays changes when the devices are not online simultaneously.

- Windows later reconnects and receives the shared knowledge but not device-scoped secrets or large model files.

- Project registry remains aware of code/data located on other machines and offers Browse/Open/Clone or remote execution according to permissions.

- Backup retains prior versions independently of synchronization so accidental deletion can be recovered.

## B.5 Out-of-box executive assistant

- User installs Rikuri and says, “Check my email and tell me what I need to deal with today.”
- Rikuri asks to connect only the user’s actual email provider, explains read/draft/send authority in human terms, and completes triage.
- User says, “Reply to the important appointment email and set up the meeting.” Rikuri drafts/sends or requests the appropriate approval, checks the calendar, and creates the event.
- The completed interaction updates contacts/commitments/project knowledge as appropriate; no agent or workflow setup is exposed.

## B.6 Lecture companion

- User says, “I’m starting this online lecture. Take notes.”
- Rikuri uses available captions/transcript or permitted local audio capture, respecting privacy classification and site policy.
- It creates structured notes, concepts, examples, questions, and source provenance.
- Afterward the user says, “I didn’t understand backpropagation—teach it differently, then quiz me.” Rikuri teaches from the actual lesson context and records useful study state.

## B.7 Multi-agent research with human scenario decision

- User defines an objective but lacks deep domain expertise.
- Coordinator recognizes novelty/high uncertainty and invokes a Strategy Council with diverse models/skills.
- Council gathers evidence and produces three viable approaches; an independent evaluator challenges unsupported claims and compares expected cost/risk.
- Rikuri presents 2–3 human-readable scenarios and recommends one. The user makes the consequential choice; Rikuri then staffs and executes the project.

## B.8 New local model acquisition

- User asks for local video generation but no suitable model is installed.
- Rikuri searches trusted model sources, filters candidates by hardware/runtime/license/security/evidence, and recommends two or three.
- After approval, Rikuri downloads an exact pinned artifact, verifies/hash-records it, quarantines/screens dependencies/custom code, and runs task-specific baseline tests.
- The model enters probation; subsequent real jobs update its capability/profile and routing rank automatically.

## B.9 Privacy-preserving financial analysis

- User asks Rikuri to analyze sensitive bank statements marked Local Only.
- Context Compiler and Security prevent raw statements from reaching external models; local tools/models perform extraction and analysis.
- If deeper frontier reasoning is useful, Rikuri may request permission to send a sanitized aggregate that excludes account numbers and raw transactions.
- Credentials and payment/signing authority remain in the secret/financial boundary and are never exposed to model context.

## B.10 Spatial multi-model comparison

- User asks, “Have Claude, GPT, and my local model review this architecture independently.”
- Rikuri opens or reuses three Prompt-Linked agent windows sharing the same permitted project context but isolates their initial responses.
- The user can tile the three windows equally, enlarge one, or detach one without changing logical links.
- A fourth comparison/evaluator window summarizes agreements, disagreements, evidence, weaknesses, and a recommendation.
- The user may say, “Make GPT large and close the others,” or manipulate the layout directly.

## B.11 Live multi-agent coding workspace

- User opens a Coding workspace for a project and asks Rikuri to split work between two coding agents.
- Each worker receives isolated execution state (for example, branch/worktree/process space) while the user sees project-centric windows rather than low-level isolation machinery.
- Agent A is followed through linked Chat, Live Code, Diff, and Tests windows; Agent B works quietly in the background.
- The Project Graph shows dependencies, status, blocked work, and one human decision.
- The user can interrupt, take control, compare changes, or allow Rikuri’s integration/review flow to continue.

## B.12 Capture and rediscover an idea

- While away from the desktop, the user dictates, “Idea: use the train station encounter near the ending,” and attaches a photo.
- Rikuri immediately preserves audio/photo/original timestamp and creates an Idea record without requiring a folder or tags.
- Later, the Knowledge Distiller links it to the relevant writing project, characters, and location as a tentative idea rather than canon.
- Months later, while editing the relevant chapter, Rikuri quietly shows “Related ideas · 1.” The writer can inspect the original capture and adopt, revise, or reject it.

## B.13 Writing workspace and semantic global change

- A writer opens a saved Writing workspace containing Manuscript, Rikuri Editor, Story Map, Character/Canon, Research, Continuity, and Idea Board windows.
- The writer says, “Change Elena from an architect from Madrid to a photographer from Valencia; show me what this affects before editing.”
- Rikuri identifies passages, timeline facts, relationships, scene dependencies, research references, and canon entries that would be impacted.
- It proposes a coherent reviewable change set rather than blind string replacement; the writer accepts/rejects individual or grouped changes.
- Original versions and rationale remain recoverable.

## B.14 Native-ecosystem continuity

- User prefers a task inside Claude/Cowork or ChatGPT/Codex rather than Rikuri’s own window.
- Rikuri hands off only permitted context/resources through the supported integration path and records resulting artifacts/decisions/events when observable.
- The user later returns to Rikuri or switches model/provider; project continuity comes from Rikuri-owned state rather than requiring the original vendor conversation.

## B.15 Multilingual Personal Intelligence

- User runs Rikuri’s interface in Traditional Chinese (Taiwan), asks a question in Chinese about a project whose source notes and papers are mostly English, and receives a Chinese answer with links to the original English evidence.
- Rikuri preserves the original notes, filenames, and citations; any translation is stored as derived, provenance-linked material rather than replacing the source.
- The user opens a second Agent window and switches only that conversation to English for a technical coding discussion while the rest of the UI remains in Traditional Chinese.
- Later the same project is opened on another device using Japanese UI; project identity, history, and canonical state remain unchanged.

# Appendix C. Key Terms

| **Term**                     | **Definition**                                                                                                                       |
|------------------------------|--------------------------------------------------------------------------------------------------------------------------------------|
| Personal Intelligence        | The durable user-owned knowledge, state, skills, relationships, projects, and continuity managed by Rikuri.                          |
| Model                        | The underlying AI intelligence (frontier or local).                                                                                  |
| Runtime / Harness            | The environment that turns a model into an effective worker: context, tool loop, execution behavior, session handling, etc.          |
| Execution profile            | The routed combination of model, reasoning level, runtime, tools, machine, access path, security envelope, budget, and verification. |
| Context compiler             | Builds the smallest relevant task-specific context package from Rikuri state and sources.                                            |
| Knowledge Distiller          | Transforms raw conversations/activity into topic knowledge, typed statements, links, and provenance.                                 |
| Project Registry             | Catalog of projects and their locations/resources/relationships.                                                                     |
| Resource Resolver            | Finds and opens the current authoritative instance of an indexed project/file/resource.                                              |
| Model Lab                    | Discovers, evaluates, profiles, and continuously learns how to use local models and their harnesses.                                 |
| Machine Intelligence         | Per-device hardware/software/live-state knowledge and incident/remedy history.                                                       |
| Personal Mesh                | Local-first synchronization of selected owner knowledge/state across devices with optional hub.                                      |
| AISecurity / Rikuri Security | Trusted security layer enforcing capability boundaries, privacy, audit, health, and high-risk execution policy.                      |
| Attention Firewall           | Central policy controlling when autonomous work may interrupt, notify, message, or call the user.                                      |
| Rikuri Constitution         | Small model-independent set of durable user-alignment and behavioral principles governing all Rikuri roles.                           |
| Governance Policy           | Decision rights, budgets, privacy/security rules, attention policy, escalation, and authority boundaries.                              |
| Agent Charter               | Persistent role definition for coordinator, specialist, evaluator, or other worker.                                                     |
| Task Contract               | Per-assignment objective, deliverables, constraints, budget, success tests, and escalation conditions.                                  |
| Coordinator / COO role      | Replaceable routed model/runtime that operationally manages planning, staffing, execution, and recovery under Rikuri governance.        |
| Strategy Council            | Temporary diverse set of independent advisers used for hard, novel, uncertain, or high-impact decisions.                                |
| Independent Evaluator       | Separate evaluation function that measures outcome and coordinator performance using evidence, telemetry, and appropriate model review. |
| Evaluated execution identity| Exact model revision, quantization, runtime/version, settings, and hardware combination whose behavior Rikuri has measured.             |
| Model acquisition lifecycle | Discovery, compatibility/security/license screening, approval, pin/hash/quarantine, baseline eval, probation, and ongoing revalidation. |
| Quick Assist                 | Compact text/voice surface for asking Rikuri to act without opening a full workspace. |
| Rikuri Workspace             | Spatial workbench containing independently movable/resizable/tileable/linkable windows tied to shared Rikuri state. |
| Window                       | First-class visual work surface such as Agent, Document, Code, Browser, Diff, Project Graph, Research, Tests, or Dashboard. |
| Context Link                 | Logical group causing multiple windows to follow the same underlying project/entity/object, independent of their physical layout. |
| Agent Link                   | Logical relationship in which multiple windows expose different live views of one worker. |
| Prompt Link                  | Broadcast relationship for sending one instruction to multiple independent agents/models for separate responses. |
| Project Graph                | Expandable live graph of objectives, tasks, agents, artifacts, dependencies, evaluations, status, and decisions. |
| Idea Inbox                   | Low-friction capture point for incomplete text/voice/photo/video/file/chat ideas before manual organization. |
| Knowledge Web                | Linked representation of established concepts, evidence, decisions, procedures, and relationships. |
| Idea Web                     | Linked representation of tentative ideas, hypotheses, inspirations, proposals, and unrealized opportunities. |
| Canon                        | Domain-specific set of established facts/rules to be enforced for consistency, especially in long-form writing; distinct from tentative ideas. |
| Semantic / narrative refactoring | Reviewable propagation of a conceptual change across dependent content and knowledge rather than blind text replacement. |
| Authoritative representation | Declared source of truth for a given object/resource type, which may be Rikuri state, Git, an original file, Markdown, or a live external system. |
| Internationalization (i18n) | Architectural separation of language/locale concerns from core application logic so Rikuri can support additional languages without redesign. |
| Localization (l10n) | Language- and locale-specific presentation of interface text, formats, terminology, voice, and related user experience. |
| Locale / Language Profile | User/workspace preferences for interface locale, assistant response language, formatting, and optional per-window/conversation overrides. |
| Cross-lingual retrieval | Ability to find semantically relevant content written in another supported language while preserving the original source and provenance. |
| Canonical data               | Durable source/state that must survive migration and from which derived indexes can be rebuilt.                                      |
| Derived data                 | Embeddings, indexes, caches, summaries, or projections that can be regenerated.                                                      |

# 27. September 30 creative-studio requirements

This dated refinement controls product emphasis while retaining the detailed Personal Intelligence requirements above. General assistant and full IDE capabilities remain optional supporting packs. The first differentiated product is the creator's connected studio; initial focus is fiction/storyworld work. Games/animation and marketing expand after that experience proves useful.

| ID | Requirement | Delivery / acceptance |
| --- | --- | --- |
| STUDIO-001 | Freeform capture and modular linked authoring views over shared canonical creative objects; organization can happen later | Stage 2 S2.C1–C2; Test Strategy §16 |
| STUDIO-002 | Preserve author intent, branch ancestry, canon versus candidates, source evidence and unresolved contradictions; merge only with authorized semantic resolution | Stage 2 S2.C3; §16 |
| STUDIO-003 | Preserve distinct scenario packets, rationale, assumptions, uncertainties, consequences, comparison and execution recipe; partial work visibly incomplete | Stage 2 sequential; governed parallel/overnight after Stage 6; §16 |
| STUDIO-004 | Requested reflection/abstraction offers useful reframing and questions; advanced cross-episode learning retains CI evidence/promotion/lifecycle controls | Requested Stage 2; learned Stage 10 after 4/6; §16 |
| STUDIO-005 | Science-backed exploration distinguishes supported external facts, unknowns, modeled consequences and deliberate fictional departures | Stage 8 domain tools; §16 |
| STUDIO-006 | Capture coverage is explicit per connector/source/revision; no memory engine, screen monitor or provider login implies complete history access | Foundation import contracts; Stage 1/5/8 adapters; §16 |
| STUDIO-007 | Proactive discovery may suggest relevant disciplines or current research within a scoped grant; queries, costs, source sharing and interruptions obey existing policy | Stage 5 bounded discovery; learned incubation Stage 10; §16 |
| STUDIO-008 | Pi identity and responsibilities persist independently of model, worker or upstream Pi harness; creator can inspect, revoke and change its role | Basic identity Stage 2; governed delegation Stage 6; persistence Stage 7; §16 |
| STUDIO-009 | Portable local work remains readable/searchable/exportable after model changes or subscription cancellation; paid services never hold creative data hostage | Stage 2 storage/export; distribution Stage 5; §16 |
| STUDIO-010 | Production handoff preserves decisions, constraints, assets, unresolved issues and rights/provenance; coding or publication remains separately authorized | Stage 2 brief; richer Stage 8 specs; §16 |

**Naming:** use **Pi** (plural **Pis**) for a persistent collaborator, with the creator’s proposed family **StrawberryPi, ChocolatePi, PumpkinPi and ApplePi**. **Pie/Pies** may be an informal nickname. Names are expressive identities, not fixed specialties: a creator may assign and revise each Pi’s role without renaming it or binding it to a model. Pi means Personal Intelligence in the product name. Technical contracts call the separate upstream dependency the **Pi harness**. A named collaborator is a Rikuri-owned identity/role with governed state and capabilities, not a model, a license to act, or a promise of continuous inference. Final public naming and availability review remain open.

**Free and advanced packages:** proposed free essentials include durable capture, basic connected authoring, retrieval, model choice, requested critique and portable export. Advanced subscriptions may add deeper domains, governed ensembles, extended research, professional integrations and collaboration. Exact limits/prices are not decided. Security, correction, forgetting and access to existing work are baseline protections. Compute is separately disclosed and budgeted, including BYO/local options; subscription never implies unlimited provider calls or an entitlement to reuse another service's quota.

**Measures:** creator can recover a fragment/source, resume a project, compare and resolve alternatives and export a usable brief; assess time/rework and creator-rated usefulness against simpler chat/document baselines. Measure actual use of suggestions, continuity errors, capture gaps, budget adherence and nuisance. Creative quality and audience impact need human evaluation; verbosity, generation counts and arbitrary numerical aesthetic scores do not prove improvement.

**End of Rikuri Pi Product Requirements Document v1.2**

# 28. October 1 Writer's IDE requirements refinement

The [Writer IDE Specification v0.1](Rikuri_Pi_Writer_IDE_Spec_v0.1.md) is the detailed contract for WIDE-001–012 under STUDIO-001–010. It defines stable sentence/paragraph/unit identity, branch-aware references and recovery, reviewed structural changes, contextual craft questions, optional lessons/flashcards, collaboration agreements, scholarly references and editor-independent portability. Test Strategy §18 links its planned acceptance cases; none is implementation evidence.

Creative projects promise human manuscript authorship: agents advise, teach, research and propose non-prose structures, with short labeled illustrative examples; finished scene/chapter generation and serial ghostwriting are prohibited. Earlier scenario/alternative language means structural possibilities in creative mode. Business-production projects may explicitly permit drafting, without promising legal copyrightability/exclusivity. A policy change cannot occur silently. Roles/action permissions, rights agreements and consent to AI processing are distinct.

Stage 2 S2.C1–C4 delivers the text-first solo slice; Stage 6 supplies governed multi-human/agent agreements; Stage 8 richer voice, scholarly publishing and domain tools; Stage 10 learned inference stays gated. Saved lessons are user-chosen artifacts, not permission for automatic user profiling. Master Build Sequence remains the roadmap; Foundation-0 unchanged.

# 29. October 8 mission charter requirements

The owner adopted the [Mission Charter](Rikuri_Pi_Mission_Charter_v1.0.md) on October 8, 2026 ([ADR 0000 clarification](Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md#october-8-2026--mission-charter-and-platform-independence)). Most of the charter restates promises already in this PRD: P1, P2, P9, P13, P14 and P30 (model, OS, hardware and application neutrality), §2 (continuity across models, devices and operating systems), STUDIO-005 (fiction versus established fact), STUDIO-009 (work stays readable after a model change or cancellation), GOV-010 (Project Lifecycle Kernel), MACH-004/005 (OS Capability API, Ubuntu reference) and the §22 Portability metric. Those remain the controlling requirements. Principles P32–P34, three §22 rows and the table below add only what was new. Foundation-0 stages 00–05, gates, S/T/E IDs and dates are unchanged.

| ID | Requirement | Delivery / acceptance |
| --- | --- | --- |
| MISSION-001 | **Decision test.** When an entry leaves the parking lot, and when post-Foundation stages are re-estimated, the record states its answer to this question (charter §12): *Does this choice sustainably expand the user's ability to imagine, understand, express, create, discover, and accomplish meaningful things—without sacrificing autonomy, trust, security, or portability?* A feature that fails it is reprioritized. The test never overrides a security boundary, a gate or the user's explicit decision. | The parking lot's "Leaving the lot" procedure and each post-Foundation re-estimation, first at the review held with S05.10. A process step outside S05.10's acceptance; Test Strategy §20 |
| MISSION-002 | **Capability over automation.** Reports and measures keep work AI completed separate from capability the user gained; automation volume and token, agent or tool counts never stand in for capability. Until product Stage 10, the capability side shows only what the user chose or declared: lessons and practice they saved (WIDE-008) and outcomes they recorded. Rikuri computes no capability or competence figure from the user's history before then. Explanation and practice are available on request (WIDE-006). Rikuri does not manufacture encouragement or steer the user's beliefs. | Stage 2 with WIDE-006/008's planned teaching and chosen-lesson surfaces; measured capability growth is Stage 10 research (§22); Test Strategy §20 |
| MISSION-003 | **Platform claim states.** New or revised claims about whether a feature is available on a platform or processor architecture, in documentation, release notes and capability reports, use one of four states: architecture-supported, implemented, verified (naming platform, OS and architecture versions, date and evidence), or unavailable/restricted (with the reason). A build, compile or launch alone cannot claim verified. Existing text is not rewritten retroactively. These states describe availability, not protection: security assurance levels (the S03.11 envelope levels, such as `assurance: experimental`, and SEC-024's scoped assurance) and gate evidence (Pass / Fail / Not yet proven) keep their own vocabularies and are reported alongside; verified never implies an assurance level. | Documentation claims from this change; runtime capability reports from product Stage 2 onward, adding no scope to Foundation-0's Home or health surfaces (S05.02, S05.08); Architecture §26.5; Test Strategy §20 |
| MISSION-004 | **Vendor AI layers are adapters.** OS-integrated model runtimes, NPU frameworks and built-in vendor assistants are admitted only as runtime or tool adapters under the existing admission, privacy, budget and provenance rules. They never hold canonical state. Removing one degrades capability visibly while knowledge, projects and creative history stay usable with other runtimes. A layer may serve Local Only work only while the runtime itself executes inside an admitted envelope whose network is Denied for that execution (Architecture §27.2), with its evidence requalified when the runtime or OS changes. A layer that runs as an OS service with its own network access, or can route work to its vendor's cloud, is a cloud path and ineligible for Local Only. Its own memory, history or personalization counts as provider retention under SEC-028: unless retention is disabled and verified, the layer is ineligible for Private and Local Only data. A built-in assistant that can take actions is an opaque native worker (Architecture §13.5, §27.3) and fails closed (SEC-021). | Constraint from this change (Architecture §26.5); adapters arrive with product Stage 4 Model Lab, Windows-specific ones with Stage 9; Test Strategy §20 |
| MISSION-005 | **Platform and architecture targets.** Linux (x86-64 and ARM64), macOS (including Apple Silicon) and Windows (x86-64 and ARM64) are targets. The Foundation-0 Ubuntu reference installation (ADR 0000, September 28) runs on x86-64; macOS core tests continue, and Windows follows at Stage 9 as already planned. Each OS and architecture pair reaches verified only through its own evidence. | Existing platform order unchanged. Cross-platform continuation: Stage 7 (second device) across the pairs verified by then, and Not yet proven while fewer than two pairs are verified; Windows pairs at Stage 9. ARM64 timing is not yet sequenced: it enters a stage through the parking lot; Test Strategy §20 |
| MISSION-006 | **Chosen collaboration modes; learned adaptation later.** The user should be able to choose how Rikuri works with them: direct execution, explanation and teaching, broad exploration, independent critique, structured planning, or open experimentation. Adapting automatically to demonstrated competence, and noticing stalled projects or recurring failures across the user's history, is learned behavior about the user: Stage 10, after Stage 4 evidence and Stage 6 governance, and always inspectable, correctable and revocable. Stage 2 offers only what is already planned there: creative-mode advice and teaching (WIDE-006), text contextual questions (WIDE-007) and requested reflection (STUDIO-004); voice Ask and Dictate arrive at Stage 8. A general mode selector across domains is not yet sequenced (parking lot). | Stage 2 creative-mode advice, text questions and requested reflection (WIDE-006/007, STUDIO-004); voice Stage 8; general selector via the parking lot; learned adaptation Stage 10; Test Strategy §20 |

Charter items that are new features rather than promises are recorded in the [parking lot](../docs/PARKING_LOT.md), not here: ARM64 CI, NPU scheduling and prefill/decode disaggregation, vendor-runtime adapters, creative production app adapters (Blender, video, audio, painting), and a general collaboration-mode selector.
