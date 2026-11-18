# ADR 0007 — Rikuri Intelligent Execution Management System (RIEMS)

**Date:** 2026-11-17\
**Status:** Owner-approved architectural direction; proposed repository adoption; implementation not yet proven\
**Scope:** Rikuri Pi control plane; Rikuri Orchard; domain workspaces; future Rikuri Link/Home Mesh\
**Supersedes:** Nothing. Extends existing Capability Planner, Execution Profile/Router, Performance Ledger, Workflow Engine, semantic scheduler, and durable asynchronous runtime.\
**Authority:** AISecurity, project/source-of-truth state, vault boundaries, user approvals, and authoritative verification remain binding. RIEMS does not supersede them.

## 1. Decision

Establish **RIEMS (Rikuri Intelligent Execution Management System)** as a first-class product goal and outcome-driven execution management capability. Given a user's authorized objective, RIEMS chooses and continuously evaluates *how* to achieve a verified result—not how many agents to launch.

RIEMS operates above the semantic project scheduler and durable runtime. It must consider deterministic software, tools, a single model/agent, multiple specialist agents, and local/cloud/hybrid execution, and select the **least complex adequate strategy** expected to satisfy quality, deadline, privacy, security, budget, and human-oversight constraints.

A more complex strategy is justified only by a reasoned, measurable expected improvement. Agent count, worker utilization and token throughput are intermediate signals, **not success criteria**.

## 2. Architectural placement and ownership

- **User-facing Intent / Task Contract** defines desired outcome, acceptance evidence, deadline or pace preference, permitted cost, privacy, authority, and desired human involvement. Do not require the user to know agent roles, APIs, or software plumbing.
- **Existing Capability Planner** produces `TaskRequirements`. RIEMS consumes this profile and produces a strategy decision; do not create a second intent parser or competing requirement schema.
- **RIEMS Strategy Planner** compares candidate approaches, predicts risks and tradeoffs, identifies dependency critical path and safe parallelism, and writes a versioned, explainable `ExecutionStrategyDecision` with evidence/confidence/alternatives.
- **Execution Profile and Router** implement the chosen strategy by selecting the model, harness, machine, tools and access path. They also select the verification profile and the applicable privacy and budget envelope.
- **Semantic/project scheduler** alone determines DAG readiness, blocked prerequisites, priorities, leases, causal invalidation, approval gates and task revision admissibility. RIEMS may propose changes, but cannot waive this authority.
- **Durable asynchronous runtime** dispatches bounded work, checkpoints, sleeps and wakes, handles timers/retries/cancellation and records outcomes. Execution may continue when the UI detaches, without retaining an active worker during idle waits.
- **AISecurity / Personal Intelligence Vault** mediate access to secrets, tools, projects, memories, machine execution and network egress before protected actions and on resumed execution. No optimizer may silently downgrade privacy, alter authorized scope or bypass step-up approvals.
- **Performance Ledger and Verification Manager** retain actual elapsed time, spend, resource occupancy, quality evidence, retries, failures, rework, confidence and user feedback. RIEMS uses these observations for calibrated future decisions; Foundation-0 remains measurement-first, not self-modifying.

## 3. Choice criteria and optimization

Apply **hard constraints before optimization**: security authorization, privacy/data handling, scope, minimum quality, explicit approval boundaries, spending ceiling and deadline policy. Among admissible strategies, balance:

1. Probability of *verified* success against task-specific acceptance tests.
2. Deadline reliability and expected time-to-verified-completion, including integration/verification/recovery time.
3. Cost per verified outcome (tokens/API/quota, CPU/GPU/RAM, storage, energy where measurable, tools and integration effort).
4. Failure and rework probability; coordination overhead; project contention.
5. User attention and desired participation; reversibility and explainability.

Deadline behavior: Identify critical path, total slack and uncertainty. Increase parallelism only for causally independent tasks with adequate capacity and integration contracts. Never assume that more agents increase throughput or quality. If a deadline is impossible while satisfying mandatory requirements, present the constraint and a safe tradeoff rather than fabricate confidence or silently reduce quality.

For exploratory/creative work, preserve an explicit optional exploration budget where appropriate; the cheapest route is not always the highest-quality outcome. Stop when criteria and proportional confidence are met or a defined stop condition occurs.

## 4. Required functional behavior

**RIEMS-001 — Outcome contract:** Versioned objective, deliverable, acceptance criteria/evidence, risk, quality floor, deadline/pace, budget, privacy, security and human-control profile.

**RIEMS-002 — Candidate strategies:** Compare at least deterministic tooling, strong single-agent baseline and one justified multi-agent/hybrid alternative when task complexity warrants it. Do not fan out gratuitously.

**RIEMS-003 — Resource and dependency analysis:** Estimate available local/mesh/cloud resources, provider rate limits, conflicts, DAG critical path, anticipated waiting and integration overhead. Explicitly track unknowns.

**RIEMS-004 — Explainable selection:** Record candidates considered, constraints, rationale, expected cost/time/quality/risk, uncertainty, selection and fallback. User-facing explanation is proportional to user preference.

**RIEMS-005 — Adaptive control:** On new evidence (slippage, model failure, blocked tool, changed task, resource contention, verification failure), propose replan/reprioritization/strategy switch; consolidate or scale workers only when useful. Revalidate project and policy versions before dispatch; keep a complete audit trail.

**RIEMS-006 — Durable execution:** Work waits and resumes without blocking scarce worker slots; use idempotency and transactionally recorded dispatch outcomes; guard against duplicate effects, stale retries and late superseded events.

**RIEMS-007 — Verification:** Completion is determined by task-specific, independent evidence against the current requirements and artifact revisions. Agent declarations, process exit codes or UI status alone cannot produce a green result.

**RIEMS-008 — Learning with bounded autonomy:** Compare predicted and actual outcomes; store provenance and confidence; promote changes only after measured evaluations and appropriate user/policy authorization. No autonomous rewrite of protected policy or authoritative state.

**RIEMS-009 — User control and transparency:** Offer simple outcome/pace/cost controls with optional expert inspection, override, pause and cancel. Do not require app, model or prompt expertise.

## 5. Canonical data contracts (conceptual, not a finalized schema)

- `TaskContract`: `task_id`, `task_revision`, `objective`, `deliverable`, `acceptance`, `quality_floor`, `deadline_policy`, `budget_envelope_id`, `privacy_scope`, `security_policy_version`, `human_authority`, `stop_conditions`.
- `StrategyCandidate`: `strategy_id`, `task_revision`, `work_dag_ref`, `worker_topology`, `execution_profiles`, `resource_requirements`, `predicted_time_distribution`, `predicted_cost_range`, `predicted_quality_confidence`, `risks`, `assumptions`.
- `ExecutionStrategyDecision`: `decision_id`, `selected_candidate`, `alternatives`, `constraint_checks`, `rationale`, `evidence_refs`, `model_profile_versions`, `decided_at`, `replan_trigger_set`.
- `ExecutionObservation`: `decision_id`, `artifact_revisions`, `verification_evidence`, `actual_duration`, `actual_cost`, `resource_time`, `wait_time`, `coordination_cost`, `failures`, `rework`, `feedback`, `attribution_confidence`.

These should reuse existing canonical entities and ledger/event contracts. Create no parallel source of truth, independent task status enum, or shadow budget/authorization store.

## 6. Invariants and negative requirements

- Never use agent count or utilization as a proxy for user success.
- Never admit task execution through a RIEMS recommendation without current scheduler readiness, valid leases and AISecurity authorization.
- Never infer that more parallel jobs imply sooner completion; measure critical-path impact.
- Never reuse another user's or project's private context, KV caches, embeddings, or artifacts across a mesh worker without explicit authorization and isolation.
- Never allow an adaptive strategy to waive acceptance tests or claim progress after missing ACKs, incomplete receipts, stale state or unconfirmed side effects.
- Preserve prior authoritative state during migrations; validate, write, read-back, commit atomically, then retire the prior state.
- Treat tool outputs, provider reports and execution predictions as evidence with provenance and uncertainty, not unquestioned truth.
- Reuse proven runtime/resource infrastructure if it passes Rikuri's trust, lifecycle, license and failure-gate evaluations. Innovate primarily in outcome/strategy intelligence.

## 7. Phased implementation, respecting live gate authority

**Foundation-0 (current v2.1 Stage Plan):** Do not add new release-blocking RIEMS gates or introduce a predictive optimizer. Record outcome contract elements, execution identity, observed time/cost, verification, wait/retry/failure evidence, and no new mandatory strategy IDs beyond existing stage evidence contracts. Keep F1–F10, W1, and F4-lite authority and existing acceptance standards intact. Document RIEMS as long-term architecture and an evidence consumer.

**Post-Foundation Stage 1 (coding dogfood):** Baseline single-worker execution; collect task outcomes from real plan→code→test→gate workflows. A simple deterministic tool-only alternative should be measured where appropriate.

**Stage 4 (Model Lab/routing):** Use the existing Execution Profile, router and Performance Ledger to compare model/harness/access choices under policy. Test predicted versus realized time and cost; do not enable learned auto-routing without evidence gates.

**Stage 6 (multi-agent governance):** Introduce RIEMS strategy selection for one versus justified teams, explicit critical-path/slack estimates, resource-admission policy, adaptive replanning with supervisor/user boundaries, and benchmark comparisons against the strong single-agent baseline. Integrate with the semantic scheduler and durable runtime addendum.

**Stage 7 (mesh):** Extend placement decisions to available trusted machines while retaining per-user vault/project isolation; no pooled unified VRAM assumption, no financial-node compute enrollment by default.

**Later research:** Calibration, constrained optimization and measured exploration can be added after sufficient trustworthy data accumulates. Keep challenger strategies and rollback.

## 8. Acceptance / benchmark design

A RIEMS Stage-6 vertical slice is accepted only if it demonstrates, with durable evidence:

1. A tool-only or single-agent method is selected over a 4+ agent team when benchmarks show no meaningful advantage from collaboration.
2. On a genuinely parallelizable task, RIEMS selects limited parallelism and improves verified completion time without violating budgets/quality.
3. On a serial bottleneck, extra agents are rejected or suspended rather than spinning unproductively.
4. A blocked agent yields and resumes after authenticated, revision-correct evidence; no worker slot is consumed while waiting.
5. Failed or delayed work triggers a versioned, traceable replan that respects leases, approvals and authoritative ProjectState.
6. Simulated deadline pressure never bypasses privacy, authorization, security or minimum quality requirements.
7. A worker or scheduler crash, duplicate event, stale result and uncertain external side effect cannot produce false success or duplicate consequential actions.
8. Report per-task verified outcome, elapsed time, monetary/resource cost, verification failures, rework and prediction calibration; compare to simple baseline strategies over representative workloads.

Measure: **verified completion rate, quality-floor satisfaction, deadline reliability, time to verified outcome, cost per verified outcome, coordination/rework waste, failure and incident rate**. Report uncertainty and task-class breakdown. Reject cherry-picked agent-count demonstrations.

## 9. External reference evidence (not mandates to adopt)

- Google Research / DeepMind / MIT, *Towards a science of scaling agent systems* (2026): https://research.google/blog/towards-a-science-of-scaling-agent-systems-when-and-why-agent-systems-work/
- Nature Machine Intelligence, *Capable language models can outgrow the benefits of collaboration* (2026): https://www.nature.com/articles/s42256-026-01268-y
- Temporal Workflow Execution: https://docs.temporal.io/workflow-execution
- Ray scheduling and resource management: https://docs.ray.io/en/latest/ray-core/scheduling/ and https://docs.ray.io/en/latest/ray-core/scheduling/resources.html
- The supplied proposal cites a Rikuri durable runtime addendum dated 2026-11-06 for execution slices, scheduler barriers and event provenance. The addendum is absent from this checkout. See [Architecture §12.4](../../../doc/Rikuri_Pi_Architecture_Spec_v1.7.md#124-riems-strategy-selection-boundary-future-not-implemented) for the repository’s RIEMS authority path.

## 10. Consequences and open decisions

**Benefits:** Outcome-centered product identity; disciplined use of agents; clearer cost/quality tradeoffs; hardware-neutral and vendor-neutral execution; testable evidence of value.

**Costs/risks:** More strategy metadata and traceability; bad estimates or misclassified dependencies; optimizer-induced thrashing; latency from planning; feedback contamination; overbroad permissions. Control these through conservative defaults, minimum decision overhead, versioned revisions, time-boxed replans, sampled evaluation, human override and rollback.

**Open decisions:** Canonical schema module locations, first Stage-6 benchmark suite, weights/lexicographic priorities for task classes, runtime adoption, and privacy-safe telemetry retention. Decide through standard ADR process after inspecting the current repository.
