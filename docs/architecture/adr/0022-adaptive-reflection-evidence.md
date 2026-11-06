# ADR 0022 — Adaptive reflection with independent control and evidence

**Date:** 2026-11-06
**Status:** OWNER-ACCEPTED — architectural direction accepted November 6, 2026; normal merge pending; implementation admission remains Not yet proven.
**Supersedes:** None. ADR 0000 and accepted ADRs remain authoritative.
**Owners:** Product owner; security, storage and platform implementers for affected features.

## Owner acceptance — November 6, 2026

The owner explicitly accepted ADRs 0020–0022 after independent review (PASS; no Important findings) and correction of the nonblocking Orchard wording nit. The proposed decision below is now accepted architectural direction; its original proposal wording is retained for history. Acceptance authorizes no implementation, experiment, migration, protected rebaseline or gate closure. Existing Foundation commitments remain unchanged. See the [acceptance record](../../build-records/2026-11-06-adr-0020-0022-owner-acceptance.md).

## Context and source limits

The [supplied AREX paper and discussion](../../../doc/references/2026-11-amendments/README.md) motivate experiments, not a production dependency. Task-time improvement does not demonstrate persistent weight updates, general reliability, or superiority to all external loops. The coding benchmark uses an optimized harness; the skills ablation partially confounds skills and rounds. Rikuri architectural recommendations below are hypotheses, not paper results or measured Rikuri benefits.

## Proposed decision

Separate model-led planning/reflection from the adaptive harness (context/skills/tools/feedback) and trusted control plane (identity, policy, budgets, isolation, independent acceptance, durable commit and termination). Capability profiles bind exact model/revision/quantization, native harness/runtime, task class, tools, skills/context versions, measured behavior, uncertainty and expiry. Guided, hybrid and model-directed modes may reduce scaffolding only after measured evidence; no profile changes hard gates, authority or repair limits. Choose one reflective worker unless specialization, independent verification or permitted parallelism has measurable value; no recursive team creation.

Operational Skills Engine is a versioned interface over governed knowledge and the Context Compiler, not a new database or instruction authority. Separate facts/evidence from task procedures. Skills carry source/license, allowed capabilities, task applicability, version/hash, tests, expiry/conflicts, approval state and rollback. Retrieve minimal authorized skills and evaluate their contribution against no-skill/baseline configurations. Imported or generated skills are candidates, never auto-installed executable authority.

Feedback/Experimentation uses isolated candidate branches/workspaces only when authorized, fixed acceptance criteria and independent test feedback, best-known verified checkpoints, failure records and rollback. Preserve failed attempts with privacy-governed bounded retention, not unrestricted raw traces or internal reasoning. Task-local revision, validated persistent lessons and future model training are separate states. Training reuse is later research and requires explicit consent/licensing/privacy admission; no automatic online fine-tuning.

Predeclare verified utility/quality measure and baseline; report final success and regressions as well as improvement. Improvement efficiency is delta verified utility divided by measured seconds/tokens/compute-energy/cost, separately per denominator. Report raw values, variance, failures and intervention; zero/unknown cost yields undefined/unknown ratio, never infinity or a fabricated price. Negative improvement remains negative. Do not compare incompatible task scales or claim hardware-independent rankings. Stop at success, cancellation, policy failure, deadline, retry/budget ceiling or predeclared no-progress rule; retain prior verified candidate.

## Alternatives and scheduling

A universal rigid retry graph can waste competent-model budget; fully model-owned authority undermines security; reject both as defaults without evidence. Foundation-0 retains evidence hooks and fixed gates only. Product Stage 1 can specify minimal versioned skill/feedback seams for supporting execution; Stage 4 owns matched-budget conventional/model-directed/hybrid trials and routing evidence; Stage 6 owns team governance; Stage 10 owns cross-episode learning after evidence/governance prerequisites. Orchard M3/M4/M5 use the same control boundaries, not another production control plane.

## Admission, consequences and reversal

[Acceptance §21](../../../doc/Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md#21-november-6-amendment-acceptance) defines ARI cases. Paired trials fix task/source, privacy, tool set, skills and budgets; vary skill and round factors separately; repeat stochastic runs and report all failures. Promotion requires verified outcome/cost benefit without security regression, independent review and rollback. No winner or implementation is claimed. Revert a candidate profile/skill to the prior verified version on regression; preserve experiment provenance. Protected policy cannot be learned or self-modified.
