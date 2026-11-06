# Rikuri Pi — Engineering Standard

> Repository consistency revision: September 25, 2026. Source version retained; see [reconciliation record](reviews/2026-09-25-documentation-consistency.md).

**Version:** 1.2 Draft  
**Applies to:** Rikuri Pi Foundation-0 and all later development  
**Current repo path:** `doc/Rikuri_Pi_Engineering_Standard_v1.2.md`
**Planned implementation path:** `docs/engineering/ENGINEERING_STANDARD.md` (move through an explicit repository change; do not maintain two authorities)

> Professional quality must not depend on the builder being able to read every line of code. Rikuri uses standard work, executable gates, evidence, and explicit stage records so a novice can supply ideas and requirements while the platform supplies the project-management and verification discipline.

---

## 1. Purpose

This document governs how Rikuri software is planned, built, debugged, tested, reviewed, documented, cleaned, and released. It is also the seed specification for Rikuri's future **Project Quality Manager**: the platform capability that will apply the same discipline to user projects automatically.

A coding agent may propose and implement work. It may not certify its own work merely by saying it is done. Completion is an evidence state.

---

## Foundation-0 operating profile

ADR 0000 reconciles this Standard with the six-stage Foundation-0 Stage Plan v2.1. The Stage Plan is the live sequence and gate board; the 90-Day Build Plan v1.6 is historical. June 2027 is a planning target with four to eight weeks of contingency. A date never waives a gate. One coherent change still gets one Build Record made during planning, with plan, wiring point, verification, and disposition. Routine low-risk changes use a brief record; protected-core, privacy, security, money, deletion, migration, recovery, or irreversible work records full risk and negative-test evidence. Every existing protected crate has a Module Contract, real caller, health signal, and tests before the relevant gate closes.

The product owner need not read code. Each stage provides a short owner-runnable demonstration and a receipt with Pass, Fail, or Not yet proven, dated test/probe links, limitations, and the next decision. A demo proves visible usefulness; deterministic gates, failure drills, and appropriately independent adversarial review prove protected behavior. Review of high-impact security and restore claims happens before the internal Foundation-0 release. The later public-exposure release gate below retains its separate requirements, including a human external security audit.

Scope changes use an ADR and update the PRD, architecture, affected acceptance tests, and operating docs together. Park speculative additions; correct discovered errors during the build. For Foundation-0, run the platform-neutral core build/unit/contract checks on macOS continuously while full security/exec acceptance remains Ubuntu-only. Probe the pinned Ubuntu execution backend early (S00.13; OpenShell first candidate), including its host privilege and approval-channel settings; never weaken host-wide restrictions to make a profile appear green. F4 closes only after the Stage 04 local runtime and handoff work. F6 requires a managed pre-forget backup restore test with current revocation state and safe refusal without it. The minimal Knowledge Context Compiler must meet ten expected answer/abstain outcomes; an always-abstain path fails. Rerun all eight crash points at release.

---

## 2. How changes are built

`AGENTS.md` governs how an agent carries out one assignment: the Task Contract, the limits of two repair cycles and two review rounds, the outcomes `DONE`, `BLOCKED` and `NEEDS_OWNER_DECISION`, and stopping. This section governs what each step must contain. Where the two differ on how many times to try, `AGENTS.md` applies. Where they differ on what counts as evidence, this section applies.

The goal is a system where mistakes cannot travel. Every defect is caught by the nearest gate, made loud, and turned into a check. The depth of each step follows the change's risk tier.

**Reuse rule:** evolution for existing solutions; revolution for core strength. Before building routine orchestration or infrastructure, look for a maintained implementation or an established pattern. Record its license, trust boundary, failure modes, lifecycle fit, maintenance cost and measured acceptance, and say why when custom code is chosen. AISecurity, verified context, canonical ProjectState and transaction integrity stay under Rikuri authority. An upstream feature list does not prove a Rikuri vertical slice works.

**Future-learning evidence rule:** instrument early; infer late. Every meaningful feature or task path emits enough structured, privacy-governed evidence to reconstruct:

- what was attempted;
- with which execution, profile and tool configuration;
- what happened;
- how it was verified;
- where a human intervened.

No continuous surveillance for future learning. Future learning never weakens data minimization.

## The Five Laws

1. **Nothing is silent.** Every component runs and is gated by a test, or it is deleted. A fallback that engages is an alert, never just a save.
2. **Stop the line.** A red required gate halts dependent work. It does not authorize unlimited repair: after the task's two repair cycles, report `BLOCKED` with the exact failure, and the owner decides. Never disable or weaken the check. Skipped or excluded code is counted, scheduled debt.
3. **Standard work.** The Per-Change Loop applies to every change, at its tier's depth.
4. **Alert on the absence of success.** Last-success times and heartbeats, not only error handlers.
5. **Turn defects into tripwires.** Every fixed defect gets a regression test. A class-wide tripwire (a test, invariant, monitor, ratchet or refusal) follows the risk tier. Every incident ends with one.

## Phase 0 — Project Bootstrap (before the first feature)

A project is a prototype until it has:

- a protected `main` with CI that gates: build, typecheck, tests, lint and secret scan, none allowed to fail, proven by a red-test drill;
- a pre-commit hook with a secret scan;
- a smoke script that checks real content, not only open ports;
- a debt ratchet with committed baselines;
- no secret in the repository;
- exact dependencies in committed lockfiles, and production images pinned by digest;
- status and version shown from the running process, not from a tag or a config file.

For Rikuri Pi these are Stage 00 items, and the Stage Plan tracks them.

## Risk tiers

**High-risk** changes touch any of:

- canonical state or transactions;
- recovery, restore, migration or deletion;
- a trust or security boundary: instruction authority, execution trust, egress and Local Only, credentials;
- money or budgets;
- private data;
- anything irreversible;
- a control that enforces these: CI, the full gate, the guard and its policy, `REVIEW.md` and the verifier.

**Routine** changes are everything else, such as:

- documentation and process text;
- tooling;
- tests;
- scaffolding;
- refactors that leave high-risk behaviour unchanged;
- messages and wording.

How to tier a change:

- The Task Contract's Scope names the tier.
- When unsure, choose high-risk.
- A routine change found to touch high-risk behaviour is re-tiered in its Build Record.
- The tier is not the guard's "protected path": a change to a protected path gets its risk note whatever its tier.

| Step | Routine | High-risk |
| --- | --- | --- |
| Build Record | Brief: Task Contract, plan, verification, disposition | Full, with risk and negative-test evidence |
| Tests during the work | Targeted tests, and a regression test for every defect fixed | The same, plus the boundary, recovery and health tests the change affects |
| Fix-lock | Opt-in; seldom worth it | Opt-in; worth engaging for a defect in high-risk code |
| Fix the class | A search of the module touched | One repository-wide search for the same pattern; directly affected siblings fixed, the rest to the backlog |
| Class tripwire, beyond the regression test | Only for a defect seen twice | For the failure class, where practical |
| Independent review | Documentation only: the `AGENTS.md` §5 checks and the owner's pull-request review, no verifier. Code: one verifier pass | One verifier pass; for Rust code it includes a mutation run on the changed lines |

Either tier may get one follow-up pass on corrected findings.

- **The review's finish line.** Severity is `REVIEW.md`'s: Important or nit. Only an Important finding fails a pass. Nits never fail a pass and never start another one: they are fixed in place or listed in the Build Record (the owner's stop rule of 2026-10-10). A wording finding gets its fix, not a new gate. An overstated sentence is Important only when it claims a gate or stage result, a security property, data integrity or completion that the evidence does not support.
- **Test adequacy.** In routine code, a missing test is Important only when the change's own "Done when" depends on it.
- **The mutation run (high-risk Rust code).** The first verifier pass runs cargo-mutants on the changed lines, inside the outer safety layer. Its settings are in `.claude/agents/verifier.md` step 5.
  - A surviving mutant in a refusal, validation, recovery or integrity check is Important; other survivors are nits.
  - The run surfaces the untested conditions the tool can reach in one pass instead of one per round.
  - It does not find missing rules; the reviewer finds those by reading the change against its Plan.
  - High-risk code in other languages is judged by its own tests and "Done when".
- **The follow-up pass.** It checks the corrections and reruns that mutation scope; it does not search for new findings. A new Important finding it happens to see ends the task in `NEEDS_OWNER_DECISION`, and the owner decides whether it is fixed now, as its own task, or recorded as debt.
- **Controls are adequate when they cover what the repository uses.** This is the owner's decision for the guard, 2026-10-13, recorded in the [Stage 00 gate record](../docs/stage-gates/foundation0/00-bootstrap-kernel.md). A gap for a form the repository does not contain is owned debt, not a new task.

## The Per-Change Loop (every change, at its tier's depth)

### 1. PLAN
- [ ] Task Contract, with the tier (`AGENTS.md` §3). A change you cannot state in one sentence gets split.
- [ ] Characterize before changing: run or read the current behaviour; do not plan against assumed behaviour.
- [ ] "I will know this works when ___". If the answer is "it compiles", there is no plan yet.
- [ ] The wiring point: the real caller or entry point that will exercise the change before its stage closes. Predecessor and successor items affected by a slip.

### 2. BUILD
- [ ] Smallest coherent increment. A commit may carry its implementation, tests, docs and migration together.
- [ ] Typed code, with inputs validated at trust boundaries. Files over about 800 lines get a review warning; over about 1200 lines they need a written reason.
- [ ] Clean as you go: the old version of anything replaced goes in the same change.
- [ ] Ask whether the change touches authentication, money, execution, private data, project trust, child processes or egress. If it does, it is high-risk; name the gate that covers it and the test that proves the gate. Hardening that breaks a legitimate path is also a defect, so check both directions.

### 3. DEBUG
- [ ] Reproduce first, and read the actual error.
- [ ] Commit the failing test before the fix (`AGENTS.md` §5).
- [ ] Fix the class at the tier's depth.
- [ ] Before a restart, a delete or a config change, check that the evidence supports this action.

### 4. TEST / VERIFY
- [ ] Unit tests for the logic added, including its failure paths.
- [ ] A regression test for a fixed bug, shown failing on the old code.
- [ ] Wire test: the change exercised through its real entry point, not only its unit API.
- [ ] High-risk only:
  - **Boundary test:** the trust, privacy, budget or execution boundary, in both the allowed and the denied direction.
  - **Recovery:** a changed recovery or fallback path, triggered in a fixture.
  - **Health test:** success updates last-success evidence, and failure shows as degraded.

### 5. CLOSE OUT
- [ ] Run the full gate (build, typecheck, tests, ratchet, smoke) once, at the acceptance boundary. Targeted checks are enough during the work. A documentation-only change runs the `AGENTS.md` §5 checks. A `BLOCKED` task ends with an honest report, not a green gate.
- [ ] Scrap Sweep (below).
- [ ] Docs, runbooks and help updated in the same change.
- [ ] A commit message that explains why.
- [ ] Deploy equals build: one scripted path from source to the running system, with no hand-copied artifacts.
- [ ] Every unchecked item gets a disposition. A deferral records its reason, dependency impact, owner, new position and trigger. No dependent work begins while this stage has an unresolved required wiring gate.
- [ ] Implemented execution paths record the structured evidence the Future-learning evidence rule requires:
  - a stable task or run ID;
  - the exact execution and profile identity, where it is exposed;
  - the verification result, separate from completion;
  - any significant failure, retry or fallback;
  - cost and time, when available;
  - any material human intervention.

## The Scrap Taxonomy (Scrap Sweep)

Scrap is anything that helped you reach the change but is not the change. Remove what this task created, in the same session:

- probe scripts and temporary harnesses;
- applied one-shot scripts, which are dangerous to re-run;
- the old version left beside the new;
- debug residue and temporarily loosened checks;
- stray files this task made (`.bak`, swap files, editor leftovers);
- proven orphans, after checking imports, scripts, units, runbooks, registrations and dynamic loading.

Keep user data, reproducible migrations, rollback tools, fixtures and evidence.

An untracked file this task did not create is reported, not deleted: age or an unknown purpose is not deletion authority.

Other sessions' work, stale stashes, branches whose work landed and leftover worktrees are reported, not removed: the owner disposes (`AGENTS.md` §2).

Anything excluded from CI or skipped is counted, ratcheted debt with a named stage in which it is fixed or deleted.

## Periodic Loops (owner obligations)

These are scheduled by the owner. They are never an agent's self-assigned work, and they never run against live user state without the owner's instruction. After each one, findings become fixes, and fixes become tripwires.

- **Weekly:** smoke run; a scan for engaged fallbacks and repeated warnings; no uncommitted drift older than a week.
- **Monthly, rotating drills:** restart with zero manual steps; failover, where recovery must be loud; mode switch, with integrity checked at each hop; restore of a real backup into scratch.
- **Quarterly:**
  - a security pass over authentication, funds and execution paths;
  - a dead-code sweep from real entry points;
  - a dependency and image audit (living sources, end-of-life dates);
  - an architecture check of documents against tested invariants;
  - a fresh-eyes review, plus a human external audit before any public exposure.

## Incident Procedure (when something breaks in production)

1. Stabilize with the least destructive action the evidence supports.
2. Capture the state before fixing erases it.
3. Find the cause down to its class, not just the symptom.
4. Add a tripwire that makes the class loud or impossible.
5. Record the incident, including the false leads, where the next session will find it.
6. Sweep for siblings of the same class.

## Working with AI Assistants

AI sessions follow `AGENTS.md`. Its hard limits are backed by `.claude/hooks/guard.py` where a hook can enforce them. Speed is no exemption from the gates.

Rikuri is its own first Collaborative Intelligence research subject:

- A claim that a prompt profile, harness, model, workflow or heuristic improves work is tested prospectively against a baseline where practical.
- Failures and negative results are kept as evidence.
- No AI-generated explanation of success substitutes for an outcome gate.

This section was condensed on 2026-10-13 (`docs/build-records/2026-10-13-trim-agent-rules.md`). It folds in that day's two dated clarifications, "bounded agent operation" and "risk tiers"; their full text is in Git history.

---

## Security assurance and upstream maintenance

September 28, 2026: protecting the user's private, financial, personal/media, professional/proprietary data and secrets is the highest product responsibility. [Option C](Rikuri_Pi_Architecture_Spec_v1.7.md#2712-option-c-policy-ownership-and-enforcement-adapters) reuses qualified enforcement components while Rikuri retains permission and release accountability. Each supported path needs a threat/control/test mapping, positive and negative controls, loss/revocation/recovery evidence, and an explicit platform/version/configuration boundary. Preserve failed experiments and limitations alongside successes.

Before admitting or upgrading a backend, record version/license/advisories, provenance and immutable production image digests, privilege and service dependencies, policy/provider translation, resource cost, maintenance ownership and rollback. Test optional telemetry disabled and inspect control-plane egress. Requalify affected guarantees when dependencies, kernel, driver, images, provider bindings, configuration or policy semantics change, at one of two tiers ([ADR 0000 addendum](Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md#september-28-2026--reference-installation-and-requalification-tiers)). **Tier 1**, the automated regression suite on the running system, applies only to: a kernel patch update within the same upstream series and the same flavour; a container image digest refresh with no version change; and a backend patch release whose changelog and advisories were reviewed and show no change to defaults or enforcement semantics. The S03.11 envelope ADR defines the tier-1 suite, including positive controls, the effective-policy hash, approval-mode and proposal-route checks, automated descendant/revocation checks, telemetry-disabled and control-plane egress checks. **Tier 2**, the full qualification (E5, T03.x, Test Strategy §8.2), applies to everything else: any kernel series or flavour change, OS release upgrade, LSM or container engine/runtime/driver change, backend minor or major release, configuration, provider-binding or policy-semantics change, and any tier-1 failure. A change not listed as tier 1, or matching any tier-2 trigger, is tier 2. The S03.11 ADR may move a component class into tier 1 only with a written reason and a tier-1 check that covers it; kernel series or flavour, OS release, LSM, backend minor/major, policy-semantics changes and tier-1 failures are never tier 1. A tier-1 receipt records the trigger, the running versions, and for a backend patch release the changelog/advisory review and who performed it. From the moment a running component changes, affected profiles are unavailable until the applicable tier passes. Evidence binds to the running kernel (`uname -r`), not the installed package. A revoked grant must stay revoked after rollback. A required regression or loss of assurance stops dependent execution and release; removing the failing test or quietly switching to an unrestricted adapter is prohibited.

Publish a concise assurance statement listing supported platforms/paths, exclusions, test dates/results, unresolved findings, and how users stop work, revoke access, recover and report incidents. A confirmed incident requires containment/revocation, minimal preserved evidence, affected-user communication and a regression check under the existing incident procedure. Comparative reliability/security claims require an explicit shared threat model, reproducible benchmark and independent review; smoke tests and vendor reputation cannot justify superiority claims. The public gate retains independent human security review; agent review is supporting evidence.

## Release Gate — Definition of Done for public exposure

A project may be shown, shipped, or opened to others only when:

- [ ] N× clean restart drill passes with zero manual steps.
- [ ] Fresh-clone deploy on a clean machine works from documentation alone
      and comes back as the same system (same identities, same data).
- [ ] CI is fully gating; the ratchet baselines are at their floor targets.
- [ ] Every mutation endpoint has a named gate and a test proving it.
- [ ] No secrets in history (full-history scan), all credentials rotated.
- [ ] Docs describe the system as it IS; a stranger can operate it from them.
- [ ] The system has run unattended through a soak period (48h+) with every
      scheduled job, backup, and monitor verified to have fired.
- [ ] An adversarial review (automated + human for security-critical paths)
      has run against the release candidate.

Until every box is checked, the honest label is "prototype" — and prototypes
don't take deposits, hold keys, or accept strangers' trust.

## November 6 amendment implementation evidence

Proposed feature-specific refinement under [ADRs 0020–0022](../docs/architecture/adr/0020-vault-foundation-reconciliation.md); this does not change the per-change procedure, risk tiers, review/repair limits or Foundation gate definitions.

Before adopting a vault/sandbox/mesh/reflective feature, map its requirement and actual stage owner to [Test Strategy §21](Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md#21-november-6-amendment-acceptance). Record Pass/Fail/Not yet proven, exact source revision, fixture/commands, OS/kernel/architecture and backend/model/harness/tool/skill/policy digests as relevant, allowed/denied/failure outcomes, current-revocation recovery, independent acceptance and limitations. Synthetic schema tests establish shape only; actual containment, encryption and concurrent isolation require real entry-point evidence on the selected platform. Version changes trigger the existing requalification policy, not a newly invented universal blocker.

New cryptography/storage choices require vetted implementation/license, key lifecycle, metadata/index leakage review, migration/forget/restore proof and rollback before real ingestion. New remote or inference-sharing paths require explicit administrator/plaintext trust, per-scope logs/cache tests, scoped leases and no privacy downgrade. Never copy production secrets into fixtures. Budgets reserve before spending; unknown hard cost refuses the path. Any failed mandatory security, state-integrity or cross-user isolation case blocks dependent feature admission regardless of aggregate benchmark score.

Reflective experiments predeclare tasks, acceptance, tool/skill versions, privacy, budgets, stop/no-progress rules, repeat count and promotion/rollback criteria. Retain useful failed-attempt evidence under consent/minimization/retention, not unrestricted internal reasoning. Report final outcomes and raw resource measurements; ratios with zero/unknown denominators remain undefined/unknown. A model's self-score and generic green tests cannot replace task-specific independent acceptance. No new experiments, installations, accounts or training runs are authorized by this text.
