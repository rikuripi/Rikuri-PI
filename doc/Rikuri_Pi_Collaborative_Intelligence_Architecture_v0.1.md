# Rikuri Pi — Collaborative Intelligence / Inquiry & Discovery Architecture

**Version:** 0.1 Research Baseline  
**Date:** September 25, 2026  
**Status:** Long-term product North Star and post-Foundation research track; not a Foundation-0 implementation commitment

> **Goal:** Increase the effective intelligence of the human–AI collaboration over time.

> **Operating principle:** Use the minimum sufficient compute, money, time, attention, and complexity for a verified outcome. Frame before solving; attend before reasoning; observe cheaply; reflect selectively; abstract cautiously; verify experimentally; preserve alternatives; explore deliberately.

## 1. Why this exists

A memory system answers “what did we know?” A Personal Intelligence system should eventually also help answer:

- What follows from what we know that we have not noticed yet?
- Which questions are more valuable than the question we started with?
- Which patterns recur across our work, tools, models, decisions, failures, and successes?
- Which apparent lessons survive counterexamples and prospective tests?
- Which old beliefs or methods are stale because the world, user, model, or environment changed?
- Which alternative frame, method, model, tool, or perspective is worth trying?
- How has Rikuri successfully helped this particular person, and when should it stay quiet?

The system is intentionally post-training. Foundation-model weights are replaceable. Rikuri learns through durable evidence, retrieval, structured experimentation, orchestration, external tools/verifiers, and versioned derived knowledge around those models.

## 2. Product thesis

Personal Intelligence has three broad responsibilities:

1. **Preserve** — durable user-owned sources, experience, projects, decisions, skills, and history.
2. **Understand** — provenance, relationships, temporal validity, privacy/ownership, conflicts, confidence, usefulness, and supersession.
3. **Discover** — generate candidate questions, reframings, connections, patterns, hypotheses, conjectures, scenarios, and solution paths; challenge them with evidence and tools; retain only appropriately labeled results.

Rikuri's differentiation is not that it owns every memory algorithm, vector engine, model, or agent framework. It owns the contracts and learning loop that make the human + models + tools + accumulated experience into a coherent, evidence-grounded collaboration.

## 3. Epistemic types

Rikuri must never collapse all generated material into “knowledge.” Candidate outputs retain type:

- observation / episode
- source claim
- question
- analogy / connection
- correlation / pattern
- hypothesis / rival hypothesis
- conjecture
- scenario / counterfactual
- heuristic / procedure
- prediction
- tested finding
- formally verified result
- user preference / working-pattern hypothesis

Promotion to stronger status requires domain-appropriate evidence. “No sufficiently strong insight” is a valid result.

## 4. Core cognitive loop

The common loop is:

`Frame -> Attend -> Budget -> Configure -> Act -> Verify -> Observe -> Reflect -> Abstract -> Challenge -> Consolidate -> Revalidate`

### Frame
Generate materially different problem definitions/questions when novelty, uncertainty, repeated failure, or consequence justifies it. The goal is often to find a better question before spending deep compute on an answer.

### Attend
Select only evidence likely to matter under the active frame. Relevance is not enough: provenance, temporal validity, consequence, contradiction, uncertainty, and evidence strength may raise or lower attention.

### Budget
Treat compute like scarce capital. Allocate reasoning/exploration according to uncertainty, novelty, consequence, reversibility, expected learning value, and cost.

### Configure
Operate the selected model well: exact model/revision, runtime, harness, prompt compiler/profile, frame/context profile, reasoning/sampling policy, tools, and verifier.

### Act & Verify
Prefer the cheapest useful attempt and explicit verification. Completion messages are not evidence of outcome.

### Observe
Store meaningful execution/outcome evidence, not indiscriminate surveillance.

### Reflect & Abstract
Cheap deterministic/statistical pattern detection precedes LLM reflection where practical. The model receives selected candidate episodes, not the entire history by default.

### Challenge
Seek counterexamples, rivals, alternate frames, independent methods, and discriminating experiments.

### Consolidate & Revalidate
Useful lessons become scoped, reversible derived heuristics/recipes. They decay or are superseded when versions, environments, evidence, or user behavior changes.

## 5. Experience Intelligence

### 5.1 Experience Ledger

Use the existing event and Performance Ledger as the authoritative execution history. `ExperienceEpisode` is a derived projection grouping meaningful events around a task/outcome. Typical evidence includes:

- task/project/context
- actor/user/agent/model
- exact execution/cognitive recipe identity
- tool/MCP/harness/runtime/machine
- important state transitions
- failures, retries, fallbacks, refactors
- verification and outcome
- cost/latency/tokens where available
- human intervention and feedback
- later usefulness/outcome links

### 5.2 Pattern detection

Start with cheap methods: counts, conditional frequencies, sequences, anomaly/change detection, cluster summaries, outcome/cost comparisons, and version-aware trends. Escalate only candidate patterns that cross evidence/importance thresholds.

### 5.3 Abstraction ladder

`Episode -> Observation -> Repeated pattern -> Hypothesis -> Heuristic -> Tested heuristic -> Learned practice`

Every transition preserves support, counterexamples, alternative explanations, applicability, and uncertainty. First-hand experience is not automatically truth; external evidence is not automatically applicable.

### 5.4 User and collaboration model

Rikuri may derive hypotheses about strengths, recurring friction, habits, knowledge evolution, and successful interventions. These are not identity labels. A useful intervention often changes the environment/default workflow rather than repeatedly admonishing the user.

## 6. Cognitive Recipes and model onboarding

A `CognitiveRecipe` is a versioned post-training configuration for a purpose:

- domain/task/purpose
- frame profile
- prompt compiler/profile
- model + revision/access path
- runtime + harness/profile
- context/retrieval policy
- reasoning effort
- sampling policy (temperature/top-p; seed policy when supported)
- tools/MCPs/capabilities
- diversity policy
- cost/latency/attention limits
- verification/falsification strategy

### 6.1 Model onboarding

For a newly available model:

1. ingest provider/model documentation and capability/economics metadata;
2. run standard task probes;
3. compare minimal/generic versus model-specific prompt/harness profiles;
4. measure stability across repeated runs/seeds where supported;
5. characterize failure signatures, tool behavior, context sensitivity, reasoning/cost tradeoffs, and privacy/access constraints;
6. create provisional recipes rather than global model rankings;
7. revalidate after model/runtime/harness/provider changes.

### 6.2 Seeds

Seeds are reproducibility and diversity coordinates tied to exact runtime/model/sampler conditions. They may be rotated to sample alternatives or fixed for regression tests. Rikuri should not normally infer that a particular numeric seed is intrinsically “better.”

### 6.3 Learning controller

Begin with deterministic rules and evidence tables. Later, contextual-bandit methods may select among known recipes while preserving exploration. Full RL is appropriate only when the action is a meaningful sequence of decisions and after offline safety/evaluation. The learned controller never owns protected policy.

## 7. Optionality and Curiosity Budget

Rikuri uses a **Champion + Challengers** model. The current evidence-backed method may receive most routine work, while meaningful alternatives remain available and occasionally tested.

Exploration increases with novelty, uncertainty, disagreement, repeated failure, expected learning value, and cheap reversibility. It decreases with mature procedure, high consequence of failure, strong evidence, and expensive/irreversible experiments.

Methodological diversity is preferred over multiplying agents that share the same assumptions.

## 8. Cognitive Ensembles

A disciplined ensemble follows:

`independent generation -> cross-critique -> evidence tournament -> synthesis -> verification`

Possible diversity levers:

- different role, same model
- same stable role over time
- different frame/prompt, same model
- different sampling/seed for ideation
- different model families
- different tools/methods (often strongest)

Where independence matters, candidate generators are isolated from prior candidates until comparison. Multi-model agreement is not proof. Disagreement should often produce a cheap discriminating experiment instead of majority voting.

## 9. Inquiry & Discovery / Incubation

An explicit `IncubationSession` may be attached to:

- a user-supplied question/conjecture/problem
- an open hypothesis
- a contradiction
- a recurring pattern/anomaly
- a watched field with new evidence
- an interesting adjacency between projects/ideas

Process:

1. retrieve Personal Intelligence and authorized external evidence;
2. generate alternative frames/questions;
3. generate competing hypotheses/solution paths;
4. use deduction, induction, analogy, simulation, calculation, search, code, proof, or domain tools;
5. seek counterexamples and rivals;
6. verify where possible;
7. synthesize surviving insights and unresolved questions;
8. store only typed candidate outputs with evidence.

Idle/nightly compute may be used within a user-approved budget, but the system should not continuously rethink the entire knowledge base.

## 10. Lightbulb lifecycle

A Lightbulb is a candidate question/connection/hypothesis/what-if, not canonical truth. Suggested fields:

```yaml
lightbulb_id: ...
type: question | connection | hypothesis | what_if | scenario | conjecture
originating_recipe_id: ...
derived_from: [...]
supporting_evidence: [...]
counterevidence: [...]
rival_explanations: [...]
novelty: personal_new | externally_known | unknown
confidence: ...
relevance_now: ...
status: dormant | resurfaced | exploring | tested | useful | incorporated | rejected | superseded | archived
suggested_next_step: ...
created_at: ...
last_revisited_at: ...
```

Dormant items remain cheap. They resurface when future context makes them relevant. Delayed usefulness feeds back into cognitive-recipe evaluation; raw idea count does not.

## 11. Domain patterns

### Mathematics / formal reasoning
Use retrieval, reframing, examples/counterexamples, symbolic/numeric tools, proof search and formal verification where available. Distinguish conjecture from verified proof.

### Research
Use claim ladders, source independence, falsifiers, rival hypotheses, stopping rules, baselines, ablations, replication and current authoritative evidence.

### Software/engineering
Use real repository state, tests, profilers/debuggers, execution traces and vertical verification. Repeated scaffolding-without-wiring failures should become testable workflow hypotheses, not personality judgments.

### Writing / creative work
Use Pattern Mirror, character/plot/reader/world/theme lenses, convention challenge, counterfactuals, return-to-intent, and Ending/Closure Lab. Creative repetition may be signature or habit; Rikuri surfaces choices rather than declaring defects. Exploration budgets shrink near completion so curiosity does not create forever-projects.

## 12. Quiet Guidance

The preferred collaboration style is subtle, timely and transparent. Rikuri should:

- surface a forgotten connection when relevant;
- challenge a repeated framing when evidence justifies it;
- suggest a better default/workflow when repeated outcomes support it;
- amplify strengths while introducing occasional alternative modes;
- compensate for recurring weaknesses through environment/process design where possible;
- stay silent when intervention is unlikely to add value.

Every material suggestion should be explainable: why it appeared, what evidence supports it, and what uncertainty/counterexamples remain.

## 13. Foundation requirements — build now

The current platform must preserve:

1. stable IDs for project/task/run/event/model/runtime/harness/profile/tool/artifact/source/outcome;
2. append-only meaningful event history with typed failures and timestamps;
3. exact execution identity and versioned profile references;
4. verification result distinct from execution completion;
5. cost/latency/token/quota observations where available;
6. retry/fallback/escalation and human-intervention evidence;
7. provenance from derived conclusions back to episodes/sources;
8. canonical-versus-derived separation and transactional promotion;
9. temporal validity/supersession and current-state reconstruction;
10. privacy/security classification and separate read/search/export/execute authorities;
11. extension-friendly schemas for future candidate-intelligence objects;
12. durable user feedback/outcome links, including delayed usefulness;
13. scheduling/idle/budget hooks for bounded future incubation;
14. model/profile onboarding metadata sufficient to re-run comparisons;
15. tests proving these records are actually wired end-to-end, not merely scaffolded.

## 14. Explicit deferrals

Foundation-0 does **not** require:

- continuous observer LLMs
- autonomous user personality/trait modeling
- nightly whole-database reflection
- learned behavioral/policy mutation
- cognitive ensembles
- Lightbulb generation
- autonomous hypothesis promotion
- RL/bandit controllers
- claim of novel discovery capability

These become research/product stages only after the evidence and governance substrate is reliable.

## 15. Evaluation philosophy

Rikuri is its own first guinea pig. Major claims use:

`claim -> prediction -> baseline -> challenger -> experiment -> verification -> evidence/counterevidence -> update claim`

Evaluate the collaboration, not only model accuracy. Candidate metrics include verified outcome per dollar/minute, rework avoided, intervention required, calibration, stale-heuristic detection, useful delayed Lightbulbs, diversity without nuisance, user-rated perspective value, and whether suggestions improve completion rather than create endless exploration.

## 16. Product success criterion

The long-term question is not “How smart is Rikuri?”

It is:

> **Is the person doing better work, asking better questions, learning faster, making fewer repeated mistakes, preserving useful knowledge, exploring beyond habitual limits, and creating or discovering things they were less likely to reach alone?**
