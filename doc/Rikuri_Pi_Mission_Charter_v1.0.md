# RIKURI PI

## Mission Charter — Human Intelligence Amplification, Creative Realization and Platform Independence

**Version:** 1.0\
**Date:** October 8, 2026\
**Status:** Adopted by the owner on October 8, 2026 ([ADR 0000 clarification](Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md#october-8-2026--mission-charter-and-platform-independence))\
**Origin:** the owner's "Proposed Addendum A v1.0" (October 7, 2026). Sections 1–12 correspond to its A.1–A.12; the corrections made on adoption are listed in the [review](reviews/2026-10-08-mission-addendum-review.md).

> **Role of this document.** This charter states why Rikuri exists and gives the decision test (§12) for choosing between competing approaches. It is rationale, not a requirements list or a roadmap. The requirements it gave rise to live in the [PRD](Rikuri_Pi_PRD_v1.2.md) (principles P32–P34, [§29 MISSION-001–006](Rikuri_Pi_PRD_v1.2.md#29-october-8-mission-charter-requirements), §22 measures) and [Architecture §26.5](Rikuri_Pi_Architecture_Spec_v1.7.md#265-platform-claim-states-architectures-and-vendor-ai-layers). Where this text and those documents differ, they govern. The [live Stage Plan](Rikuri_Pi_Foundation0_Rebaselined_Stage_Plan_v2.1.md) still controls Foundation-0; this charter adds no stage, gate or date.

---

## 1. Mission

**Rikuri Pi exists to systematically expand and realize human imagination and creativity—by combining personal intelligence, accumulated knowledge, abstraction, synthesis, exploration, and coordinated action—while preserving human authorship, autonomy, and the freedom to pursue the unexpected.**

Rikuri Pi is a Personal Intelligence environment, not merely an AI assistant, model router, agent framework, or application launcher.

Its ultimate purpose is to increase the effective intelligence and creative capacity of the human–AI partnership over time.

Success is measured not simply by how much work AI performs, but by how much more the human can understand, imagine, express, discover, create, and accomplish.

### Why this matters now

Operating-system and hardware vendors are building their own hybrid AI layers, local and cloud, tied to their platforms. Those layers will be capable and convenient, and each one makes its own platform the place where a person's AI-assisted work lives. Rikuri is not another orchestration layer competing for that position. Rikuri's purpose is to free people from being trapped by any one OS or ecosystem and to expand their horizon and capability: their intelligence, knowledge and creative work stay theirs, whichever models, devices and platforms they use along the way (§4.5).

## 2. Foundational philosophy

Human imagination establishes the horizon. Creativity discovers the possibilities. Knowledge and reason illuminate the paths. Technology supplies the means. Conviction gives us the courage to begin, and perseverance carries us through uncertainty and failure.

Rikuri Pi exists to bring these forces together—not to replace human ingenuity, but to cultivate, amplify, and help realize it.

### Core beliefs

1. **Human potential is the purpose.** Technology serves the development and expression of human intelligence, not the reverse.
2. **Technology is necessary but insufficient.** Innovation requires imagination, creativity, curiosity, knowledge, judgment, conviction, experimentation, and perseverance.
3. **Imagination should be expandable.** A person's existing knowledge, skills, experiences, and assumptions should not permanently constrain what they can envision.
4. **Creative possibilities deserve a path to realization.** Rikuri must help translate ideas into experiments, artifacts, discoveries, and achievements.
5. **Failure can generate knowledge.** An unsuccessful attempt should inform future approaches rather than simply become a discarded result.
6. **Belief must coexist with intellectual humility.** Support conviction and perseverance without reinforcing false certainty, sunk-cost commitment, or unsupported claims.
7. **Human independence is a desired outcome.** Rikuri should strengthen the user's own capabilities rather than encourage unnecessary dependency.

## 3. The intelligence amplification mandate

Rikuri shall support six interrelated functions.

### 3.1 Expand imagination

Help users encounter possibilities beyond their existing experience through alternative perspectives, counterfactuals, cross-disciplinary analogies, thought experiments, and unfamiliar conceptual frameworks.

Avoid premature convergence on familiar solutions.

### 3.2 Cultivate creativity

Support divergent exploration, novel combinations, expressive experimentation, creative constraints, and the development of original approaches.

Respect unconventional ideas without treating novelty alone as evidence of value.

### 3.3 Develop understanding

Help users build usable knowledge, recognize relationships, examine assumptions, and improve independent reasoning.

Adapt explanations and assistance to the user's current objectives and to the level of help they ask for.

### 3.4 Realize ideas

Translate intentions into executable plans, experiments, prototypes, artistic works, software, research, designs, and other concrete outcomes.

Coordinate models, agents, applications, tools, and computation without requiring the user to master every underlying technology.

### 3.5 Sustain perseverance

Help the user recognize meaningful obstacles, recurring failures, stalled projects, and unproductive methods.

Provide evidence-based alternatives, achievable milestones, progress visibility, and constructive reflection.

Support informed persistence, reframing, strategic pauses, and responsible abandonment when appropriate.

*Timing:* reflection the user asks for comes first. Rikuri noticing such patterns on its own across the user's history is learned behavior; it arrives only with the evidence, governance and privacy controls that learned behavior requires ([PRD MISSION-006](Rikuri_Pi_PRD_v1.2.md#29-october-8-mission-charter-requirements)).

### 3.6 Enable cumulative growth

Preserve verified lessons, useful abstractions, creative discoveries, decisions, and project history.

Apply relevant knowledge to future challenges while preserving provenance, uncertainty, privacy, and historical context.

*Timing:* lessons are kept when the user chooses to keep them (PRD WIDE-008); learning across the user's history on Rikuri's own initiative follows the timing in §3.5.

## 4. Platform independence

**Rikuri Pi shall be model-agnostic, operating-system-agnostic, and hardware-adaptable by architecture.**

No particular AI model, provider, processor architecture, operating system, or application ecosystem shall become a permanent requirement for the core Personal Intelligence experience.

### 4.1 Model independence

Rikuri shall:

- Support local, remote, open-weight, and proprietary models through versioned adapters.
- Support multiple model families and inference runtimes.
- Select models according to task requirements, demonstrated quality, capabilities, privacy policies, latency, energy, availability, and cost.
- Support collaboration among models and agents when this produces measurable value.
- Maintain model-specific prompting and harness profiles without embedding these dependencies into authoritative project or knowledge state.
- Allow model substitution without loss of user knowledge, project continuity, or creative history.
- Preserve access to work when a model provider becomes unavailable.

Model selection shall be an implementation decision, not a definition of Rikuri's identity.

### 4.2 Operating-system independence

Rikuri shall target:

- Linux, including supported x86-64 and ARM64 environments.
- macOS, including Apple Silicon.
- Windows, including supported x86-64 and ARM64 environments.

Linux may serve as the primary development and integration environment without becoming a permanent architectural dependency.

Platform-specific functionality shall be delivered through isolated adapters and capability discovery.

The system shall distinguish between:

- A feature supported by Rikuri's core architecture.
- A feature implemented on a particular platform.
- A feature verified as operational on that platform.
- A feature unavailable or restricted by underlying system capabilities.

Cross-platform support must never be represented as complete solely because the software compiles or launches.

OS-agnostic does not mean Rikuri delivers identical functionality on every operating system on day one. It means the architecture preserves the user's intelligence, knowledge, projects, and creative history independently of any operating system, even when individual platform capabilities differ. That makes Linux a sensible initial engineering home without making Linux Rikuri's permanent boundary.

### 4.3 Hardware and compute independence

Rikuri shall support heterogeneous computing environments, including CPUs, integrated GPUs, discrete GPUs, NPUs, and remotely accessible computing nodes.

Compute orchestration should consider:

- Prefill throughput and time to first token.
- Decode throughput and end-to-end latency.
- Memory capacity, bandwidth, and KV-cache requirements.
- Multimodal acceleration.
- Energy efficiency and monetary cost.
- Privacy and data-movement restrictions.
- Verified task completion quality.

Prefill/decode disaggregation, cross-device execution, and dynamic resource scheduling may be introduced where measurements justify their complexity.

No hardware vendor shall be assumed to remain permanently superior.

### 4.4 Application independence

Rikuri shall interact with external creative, technical, research, and productivity applications through capability-based adapters.

Examples include Blender, video editing suites, digital painting software, audio production, writing environments, simulation tools, and development environments.

Prefer mature native APIs, scripting interfaces, established interchange formats, and existing protocols over fragile screen automation.

Where screen automation is necessary, enforce permissions, observable outcomes, recovery, and user confirmation for consequential actions.

Application changes or replacements should not destroy project continuity.

### 4.5 Platform AI layers are resources, not homes

The AI layers that operating-system and hardware vendors ship (OS-integrated model runtimes, NPU frameworks, built-in assistants) are welcome resources. Rikuri may use them through the same adapters, admission, privacy and budget rules as any other runtime, and should use them where they help.

None of them becomes the home of the user's knowledge, projects or creative history. Losing or leaving one reduces capability visibly; it never costs the user their continuity. Work the user keeps local goes to a vendor layer only while that layer runs inside Rikuri's network-denied envelope; a layer that can reach its vendor's cloud is treated as a cloud path ([Architecture §26.5](Rikuri_Pi_Architecture_Spec_v1.7.md#265-platform-claim-states-architectures-and-vendor-ai-layers)).

## 5. The knowledge-to-creation cycle

Rikuri shall support an iterative process:

**Observe → Accumulate → Abstract → Synthesize → Imagine → Explore → Experiment → Create → Verify → Reflect → Evolve**

These activities are not mandatory sequential gates. The user may enter, revisit, skip, or combine them as appropriate.

### Knowledge accumulation

Preserve information with provenance, timestamps, relationships, permissions, and uncertainty.

### Abstraction

Identify transferable principles, recurring patterns, causal hypotheses, and generalizable mechanisms.

Clearly distinguish observed evidence from inferred patterns and speculative ideas.

### Synthesis

Connect information across projects, disciplines, modalities, and time periods to generate useful relationships, approaches, and hypotheses.

### Imagination and exploration

Generate possibilities without prematurely restricting them to familiar methods or currently available technologies.

### Experimentation and realization

Translate promising ideas into actions, simulations, prototypes, and deliverables through the existing Project Lifecycle Kernel:

**Frame → Shape → Plan → Execute → Verify → Deliver → Learn**

### Reflection and evolution

Compare predictions against outcomes, preserve lessons, revise assumptions, and make relevant discoveries available for future work. Reflection the user asks for comes first; unprompted reflection across the user's history follows the timing in §3.5.

**Information is data, not truth.** Retrieval supplies evidence candidates, not unquestionable authority.

The Knowledge Context Compiler remains responsible for evaluating relevance, provenance, freshness, conflicts, confidence, and policy before information influences consequential decisions.

## 6. Adaptive human–AI collaboration

Rikuri shall adapt to users without imposing one preferred thinking or working style.

It should distinguish among circumstances requiring:

- Direct execution with minimal interruption.
- Explanation, teaching, and guided practice.
- Broad exploratory brainstorming.
- Independent critique or alternative perspectives.
- Structured planning and disciplined execution.
- Creative experimentation without predetermined objectives.

The level of intervention shall be proportionate to uncertainty, user preference, and the consequences of error.

Experts should be able to work efficiently with minimal guidance. Novices should receive additional scaffolding and protection without unnecessary restriction.

*Timing:* the user chooses the mode first. Adapting automatically to the user's demonstrated competence is learned behavior about the user; it arrives only with the controls that learned behavior requires, and stays inspectable, correctable and revocable ([PRD MISSION-006](Rikuri_Pi_PRD_v1.2.md#29-october-8-mission-charter-requirements)).

Rikuri may recommend, challenge, question, or suggest alternative approaches, but shall not manipulate the user's beliefs, manufacture encouragement, or override legitimate creative decisions.

Personalization must remain inspectable, correctable, and subject to privacy controls.

## 7. Creativity without conformity

Rikuri shall protect the user's capacity to pursue unusual, speculative, unconventional, and exploratory ideas.

Creative exploration and factual verification must coexist without being confused.

A fictional proposition need not be scientifically true to be creatively valuable. A scientific hypothesis, however, must not be represented as established merely because it is compelling.

The system should preserve potentially valuable discarded ideas, alternate versions, unresolved questions, and surprising outcomes, subject to relevance and retention policies.

Novelty shall not be suppressed merely because an established solution is easier to implement.

Equally, the system must not encourage novelty when existing, reliable solutions satisfy the user's actual objective.

**Use the reliable solution when it meets the objective; pursue novelty where genuine differentiation or discovery matters.** This applies to the user's own work. PRD principle P31 applies the same idea to how Rikuri itself is engineered.

## 8. Human authorship, agency, and ownership

The user remains the ultimate authority over creative intent, project objectives, consequential decisions, and final approval.

Rikuri shall:

- Preserve the distinction between human decisions and AI contributions.
- Track provenance and revision history.
- Support inspection, correction, reversal, and export.
- Avoid unauthorized publication, disclosure, spending, or destructive changes.
- Provide understandable explanations of important decisions and limitations.
- Protect sensitive personal and project knowledge through Rikuri's permissions and AISecurity enforcement.
- Support portable, user-controlled exports of knowledge and work, so that a user can leave Rikuri itself without losing what they built.

Rikuri shall not equate increasing automation with increasing human capability.

## 9. Engineering and reliability implications

This mission shall be implemented using the established Rikuri architecture, not through an unrelated philosophical or creative subsystem.

Required foundations include:

- Project Lifecycle Kernel and authoritative structured ProjectState.
- Knowledge Service and Knowledge Context Compiler.
- Model-specific adapters and adaptive harness profiles.
- Shared application capability interfaces.
- AISecurity enforcement and auditable permissions.
- Portable, versioned artifacts and data contracts.
- Verified cross-component handoffs.
- Transactional State Evolution.
- Continuous integration, regression, and end-to-end testing.

**No silent loss. No stale-tail substitution. No false-green status.**

A claimed capability must be verified through actual execution and observable outcomes.

Generated insights, project achievements, behavioral patterns, and inferred user preferences shall not be treated as verified facts solely because an AI model produced them.

## 10. Measures of success

Success shall be evaluated through real-world outcomes rather than primarily through generated tokens, model benchmark scores, agent counts, or tool invocations.

Important measures include:

1. **Capability expansion:** Tasks users can now accomplish that previously exceeded their practical abilities.
2. **Imaginative breadth:** Meaningfully different possibilities explored without excessive cognitive burden.
3. **Creative realization:** Ideas transformed into completed, useful, or expressive outcomes.
4. **Knowledge transfer:** Lessons from earlier work improving later projects.
5. **Independent understanding:** Growth in the user's ability to reason and act without unnecessary assistance.
6. **Perseverance and adaptability:** Effective recovery, reframing, and learning after setbacks.
7. **Resource efficiency:** Quality-adjusted outcomes relative to time, cost, energy, and compute.
8. **Trustworthiness:** Accuracy, permissions, provenance, recoverability, and verified execution.
9. **User agency:** Continued ownership of intentions, decisions, and creative direction.

Assessments must account for task difficulty, novelty, user preferences, and differences among project types.

Rikuri should compare sophisticated methods with simpler alternatives to verify that added complexity actually improves outcomes.

Measures that concern the person (capability, understanding, perseverance) use evidence the user has opted into, never hidden profiling. PRD §22 holds the adopted metric rows.

## 11. Implementation progression

### Foundation — required from the beginning

Define portable state schemas, model/runtime boundaries, platform adapter boundaries, knowledge provenance, project lifecycle integration, security policy, and verification contracts.

Ensure that knowledge, intent, project history, and creative decisions survive model or platform changes.

"From the beginning" means contracts, not an implementation on every platform. Foundation-0 keeps its six stages and its Ubuntu reference. Its plan already keeps OS-specific assumptions ([Architecture §26.2](Rikuri_Pi_Architecture_Spec_v1.7.md#262-platform-order)) and vendor-specific accelerator assumptions (Architecture §49) out of kernel contracts, and tests the platform-neutral core continuously on macOS. [§26.5](Rikuri_Pi_Architecture_Spec_v1.7.md#265-platform-claim-states-architectures-and-vendor-ai-layers) adds processor architecture to that rule.

### Progressive intelligence amplification

Introduce reliable knowledge retrieval, guided exploration, abstraction, synthesis, alternatives generation, creative production adapters, and evidence-backed reflective learning.

Each capability must demonstrate useful outcomes before broad deployment.

### Mature collaborative intelligence

Develop adaptive long-term collaboration, cross-project insight discovery, verified personal-pattern learning, heterogeneous compute optimization, and increasingly sophisticated creative and research workflows.

Advanced autonomy shall remain bounded by policy, observable outcomes, and human control.

These stages describe capability maturity, not isolated architectural layers. Their supporting contracts should be considered from the foundation onward. The [Master Build Sequence](Rikuri_Pi_Master_Build_Sequence_v1.1.md) assigns them to product stages.

## 12. Governing decision principle

Whenever Rikuri's design faces competing approaches, evaluate them against the following question:

**Does this choice sustainably expand the user's ability to imagine, understand, express, create, discover, and accomplish meaningful things—without sacrificing autonomy, trust, security, or portability?**

If a technically impressive feature fails this test, its priority should be reconsidered.

If a simpler feature substantially advances this mission, it may deserve priority over a more sophisticated alternative.

The test informs priority; it does not override a security boundary, a gate, or the user's explicit decision.

---

## Foundational commitment

Rikuri Pi is not being built to win a competition among operating systems, hardware architectures, or AI models.

It is being built to help human beings exceed the limitations imposed by their available knowledge, skills, tools, time, and circumstances.

Models will change. Hardware will evolve. Operating systems will compete. Technologies will emerge and disappear.

**The lasting asset is the intelligence, understanding, imagination, creative work, and capability accumulated by the human–AI partnership.**

Rikuri Pi must preserve and continuously increase the usefulness of that asset.

**Technology is the means. Human flourishing through imagination, creativity, knowledge, and realization is the purpose.**
