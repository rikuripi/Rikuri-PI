# Rikuri Pi — Collaborative Intelligence / Inquiry & Discovery Architecture

> Repository consistency revision: September 25, 2026. Source version retained; see [reconciliation record](reviews/2026-09-25-documentation-consistency.md).

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

### 6.4 Purpose-driven capability presets

September 26, 2026 clarification — **future design, not implemented**. A purpose preset is a user-facing configuration of the existing CognitiveRecipe concept, not a separate authority or agent framework. Users request an outcome (business feasibility, research, mathematical analysis, coding, or video production); Rikuri assembles the smallest sufficient eligible workflow. One model/worker or exact tool is valid; multiple agents are justified by the task and evidence rather than the preset name.

Each versioned preset describes:

- purpose, required inputs, deliverables, success criteria and applicability limits;
- model/execution-profile choices, logical agent roles and independent review where needed;
- context topology: shared source evidence, isolated initial work, explicit critique/join points and permitted context exchanges;
- tools, skills/procedures and plugin/MCP adapter capability requirements;
- method-selection rationale, evidence collection, verification and explanation requirements;
- privacy/project access, execution locality, spending/time limits, approvals and stopping conditions;
- missing-dependency behavior and the exact resolved configuration used for a run.

Context isolation is more than separate chat windows: record which evidence and prior outputs each role could access and when. A join cannot expand any participant's data permissions. An agent's generated text cannot grant authority or override the user's objective. Shared-context collaboration is appropriate when coordination is needed; independent initial judgments remain isolated where bias checks require it.

### 6.5 User customization, defaults and reset

Configuration has three layers: a versioned built-in preset, a saved user customization based on that version, and temporary task overrides. Resolve them in that order, then validate against authoritative policy and installed/evaluated capabilities. Users can select their own eligible models, roles, tools, skills, context topology and budgets, inspect differences from defaults, and save a new named preset. Explain incompatibilities and offer eligible alternatives; do not silently substitute a remote provider, broaden access, or remove required verification.

“Reset to built-in defaults” previews the changes and names the target built-in version (the saved base or a chosen newer version). Preserve the prior configuration for undo. Reset affects only the selected preset/task configuration, never project data, evidence, completed artifacts or work history; it neither installs dependencies nor restores/regrants credentials or permissions. Undo restores configuration only and must revalidate current policy, revocations and capabilities. Running tasks retain their resolved configuration; applying changes requires an explicit governed transition or a new run. Built-in updates never silently overwrite saved customizations; expose a reviewable migration/diff. Imported/shared presets are untrusted configuration until validated, not permission grants.

### 6.6 Appropriate tools and understandable results

Choose methods by the question and validate their prerequisites. Exact comparisons, arithmetic and finite probability calculations belong in deterministic code or symbolic tools when available. Deterministic models use explicit equations; Monte Carlo methods estimate distributions under specified stochastic assumptions; Markov models require justified states and transition assumptions. Semantic decision models may classify or route bounded inputs after task-specific evaluation. Generative models support framing, explanation and creative work. Confidence scores do not replace exact calculation or verification.

A scientific/statistical run records input provenance, units, model assumptions, distributions and dependencies, tool/code versions, seeds where applicable, numerical/convergence diagnostics, sensitivity checks and analytical/reference checks where available. Reproducible simulation does not prove the model describes reality; simulated frequencies are conditional on the supplied model, not automatically real-world probabilities.

Explain what question was answered, why the method/tool was selected, what the result means, what it does not establish, and what evidence could change it. Offer concise plain-language interpretation first and expandable expert detail (sources, equations, parameters, code, diagnostics and limitations) without changing the underlying result or hiding material uncertainty. Distinguish technical verification from subjective aesthetic judgment in creative work.

When required data is missing, identify the area, why it matters and how it limits the conclusion. If asked, recommend ways to collect it or sources to obtain it, with likely cost, effort, access/privacy constraints, quality and representativeness limitations. Prioritize information that could change the decision. A recommendation to collect data is not permission to contact people, purchase data, scrape restricted sources or start monitoring.

Illustrative workflows (not integration claims):

- **Business feasibility:** source research → evidence-gap assessment → financial calculation/simulation where justified → independent challenge → two or three viable plans → traceable decision brief.
- **YouTube production:** inspect footage/transcript → propose editorial plan and alternatives → authorized editing/audio/captions/generated assets → verify export and factual integrity → reviewable draft. DaVinci Resolve or Higgsfield are examples of possible application/service adapters; actual APIs, licenses, access, capabilities and privacy must be verified before offering them. Publication is a separate authorized action.
- **Mathematical analysis:** formalize the question → select exact or numerical method → execute in an admitted environment → check results → explain assumptions and applicability. Do not use simulation when a simple exact answer suffices.

### 6.7 Plugin/MCP choice and admission evidence

Plugins and MCP servers can expand a preset's choices through the existing capability registry and Connector Gateway. Describe operations and input/output contracts, locality and downstream data destinations, permissions, side effects/reversibility, costs/quotas, availability, version and workflow-specific verification evidence. Discovery/install, connection/authentication, task selection, authorization, execution and outcome verification remain distinct. Advertisement or installation is not proof of support, access to all projects, or permission to act.

Prefer available eligible tools according to task fit, measured quality, user preference, editability, hardware, privacy, latency and cost. Expose unavailable/untested/incompatible choices honestly. Fail or request an eligible replacement for a required missing capability; any optional degradation is visible. Evaluate substitutions and material upgrades; preserve resolved tool/profile versions and explicit unknowns in execution records. Models may recommend tools, while deterministic policy excludes ineligible ones and governs effects.

Future admission cases include missing/revoked plugins, changed remote destinations, incompatible user overrides, policy-safe reset/undo, built-in upgrades preserving customizations, context-isolation violations, source restrictions at joins, task restart with pinned configuration, incorrect calculations or misleading simulation interpretation, and novice/expert views retaining the same material limitations. Test real adapter entry points and compare usefulness/cost with simpler workflows. Existing product stages still control component delivery; integrated CognitiveRecipe research follows product Stage 4 evidence and Stage 6 governance, with Stage 10 productization. None of these examples adds a Foundation-0 runtime service.

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

### 8.1 Decision procedures: September 26, 2026 clarification

**Status: post-Foundation design, not implemented or validated.** A solo founder, small business, research team, or coding team should be able to ask an ordinary question and receive a proportionate, evidence-backed decision brief. Rikuri selects a versioned procedure; the user need not configure a council or know analytic terminology. Important assumptions, uncertainty, cost, and authority remain visible even when orchestration is hidden.

A procedure separates explanations (what accounts for observations), scenarios (what could happen), and actions (what the user could choose). The most plausible explanation does not by itself determine the best action: objectives, constraints, downside, reversibility, and user preferences also matter.

| Question | Candidate method | Required result |
|---|---|---|
| Why is this happening? | Competing hypotheses, diagnostic evidence, causal checks | Rival explanations, contradictions, gaps, and evidence that could distinguish them |
| What could happen? | Base rates, scenarios, sensitivity analysis | Explicit assumptions, adverse futures, and observable reassessment indicators |
| What should we do? | Compare options across scenarios and user criteria | Normally two or three materially different viable choices, tradeoffs, and a conditional recommendation |
| How could it fail? | Premortem, red-team and dependency review | Credible failure paths, mitigations, residual risks, and stop conditions |
| What should we learn next? | Value-of-information assessment | A bounded research step or experiment whose result could change the decision |

These are method candidates, not universal prescriptions. Cheap reversible decisions need less process; uncertain, consequential, or hard-to-reverse decisions warrant deeper checks. A procedure records its applicability, version, inputs, method selection rationale, acceptance rubric, evidence requirements, privacy/budget envelope, stopping rules, reviewer independence, and verification method. It can abstain or request decisive missing information. Do not manufacture weak alternatives merely to fill a three-option display.

### 8.2 Evidence and independent challenge

The future workflow is frame → independent alternatives → evidence mapping and targeted collection → challenge → discriminating checks → scenario/option comparison → decision brief → outcome review when authorized.

- Separate observed facts, source claims, assumptions, estimates, forecasts, and owner preferences. Link material claims to source/version/date and record limitations, conflicting evidence, applicability, and collection gaps.
- Track source lineage and dependence. Repeated reports of the same announcement are not independent corroboration; shared model training, retrieval, prompts, and framing can also correlate errors. Diverse model names or unanimous agents do not prove independence or truth.
- Initial proposals are isolated from competing answers. Research assignments vary evidence channels, methods, or frames. Reviewers inspect original evidence and challenge the problem framing as well as candidate conclusions. Logical roles need not map to separate persistent agents.
- Seek concrete evidence both for and against each serious alternative. Red-team and contrarian claims meet the same evidentiary standard; neither forced disagreement nor forced consensus earns credit. Retain an other/combined-explanation possibility where the initial hypothesis set may be incomplete.
- Record expected observations and whether collection could detect them. Absence is informative only in light of observability, collection coverage, and alternative reasons for missing data.
- Use tools, source verification, customer evidence, tests, or experiments to resolve pivotal disagreements. Preserve unresolved dissent with its evidence and decision impact. Missing evidence never becomes a fabricated citation or a confident inference.
- Bound rounds, spend, latency, and user attention. Further research must have an identified question and plausible decision value. Stop on sufficient evidence for a bounded action, exhausted budget, or unresolved uncertainty requiring owner input.

### 8.3 ACH as an optional analytic aid

Analysis of Competing Hypotheses (ACH) is documented in [Heuer's CIA-published Psychology of Intelligence Analysis, Chapter 8](https://www.cia.gov/resources/csi/static/Pyschology-of-Intelligence-Analysis.pdf). Its useful practices include comparing evidence across alternatives, seeking disconfirmation, testing pivotal assumptions, and defining future indicators. Its institutional origin does not establish effectiveness for Rikuri tasks. [Experimental research on probability-judgment coherence](https://sjdm.org/~baron/journal/20/200430/jdm200430.html) found no ACH benefit in the studied tasks; procedure admission requires Rikuri-specific evaluation.

An ACH-style matrix is inspectable support, not a probability engine. Fewest inconsistencies does not automatically mean highest probability. Consider base rates, source reliability/dependence, probabilistic hypotheses, and missing alternatives. A contradiction is decisive only when the evidence and hypothesis warrant that inference. Subjective weighted scores must not be presented as calibrated probabilities. A mismatch between intuition and a matrix requires examination of both, not adjustment until they agree. Separate estimated probability from confidence in the evidence; label subjective ranges and unknowns explicitly. No procedure may promise unbiased analysis.

### 8.4 Decision brief and business/fundraising evidence

The primary artifact is a concise brief with an expandable evidence record: decision and objectives; normally two or three viable actions; recommendation and conditions; supporting and opposing evidence; expected benefits and adverse scenarios; cost/resource/timing assumptions; uncertainty and unresolved dissent; reversibility, mitigation and stop conditions; next discriminating test; and indicators that would change the recommendation. Keep all materially assessed alternatives and rejection reasons in the audit record, even when the main brief is short. Owner choices remain distinct from model recommendations.

For a business launch or fundraising case, distinguish validated customer demand, actual revenue/traction, sourced market estimates, and speculative projections. Preserve market definition, source date, calculation inputs, units, assumptions, and reproducible derivations behind material numbers. Compare relevant alternatives, including delay or a small pilot when viable. Challenge customer acquisition, competition, delivery capacity, financing dependencies, and downside assumptions as applicable. A persuasive narrative cannot upgrade a forecast to fact or hide adverse evidence. Investor-facing claims must trace to the same evidence record; private source access and export rights require separate admission. No report guarantees funding or business success.

### 8.5 Evaluation and admission

Before product admission, compare each versioned procedure with a strong single-agent baseline and independent proposals without discussion under comparable cost and information budgets. Use held-out cases and domain verification where possible. Measure material claim accuracy, citation fidelity, evidence coverage/dependence detection, probability calibration where outcomes permit, option usefulness, sensitivity to pivotal assumptions, latency/cost, and owner correction burden. Record evaluator limitations and delayed real-world outcomes separately from immediate reviewer preferences.

Negative cases include duplicated sources, missing alternatives, fabricated citations, misleading evidence, a confident wrong majority, unsupported precise probabilities, framing changes, a nonviable third option, private information leaking through summaries/search queries, and recommendations attempting to grant themselves execution authority. A failed method is revised, restricted, or withheld; complexity and agent count are not success metrics. An independent reviewer cannot replace required owner approval or external domain/security review.

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

## 13. Foundation-0 evidence requirements and future extension points

Foundation-0 preserves evidence from implemented paths under PRD v1.2 §9.20, Architecture v1.7 §21A, and Stage Plan v2.1. Future object types need extension-friendly contracts, not empty services or autonomous learning. Product Stage 4 Model Lab evidence and product Stage 6 governance precede research prototypes; product Stage 10 owns productization.

The platform must preserve:

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
13. existing budget/cancellation contracts remain extensible; incubation scheduling and idle watchers are post-Foundation work;
14. model/profile onboarding metadata sufficient to re-run comparisons;
15. tests proving evidence from implemented paths is wired end-to-end (T00.12, T02.16, T04.18); future services remain deferred.

Unavailable provider fields are explicitly unknown/unsupported, never invented. Profile references must be versioned or fingerprinted when exposed; replay uses deterministic fixtures and does not promise identical live-model responses. Sensitive content stays in deletion-governed referenced storage, not immutable event payloads. Delayed feedback uses durable outcome links; automated credit assignment remains deferred.

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
