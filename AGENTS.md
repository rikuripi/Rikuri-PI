# Rikuri Pi — Agent Standard Work

Applies to work throughout this repository. Adapted from the supplied Build Procedure and Build Checklist, using Rikuri Pi's engineering and acceptance baseline. Preserve the principle: defects must be caught near their source, made visible, and converted into durable checks.

## Start with the actual repository

1. Read `git status` and the relevant files before editing. Preserve user changes and other sessions' work; stage only changes belonging to this task.
2. Read the [documentation index](doc/README.md), the active stage and affected requirements. Do not infer current files from stale editor tabs.
3. Identify the task's stage, predecessor gates, real caller, and acceptance evidence. Documents describe intended behavior, not proof it exists.
4. At this revision the repository contains planning documents, Git hygiene, the AI-session guard below with its tests and CI workflow, and Stage 00 bootstrap: a Rust workspace whose only crate is `rikuri-domain`, the full gate `scripts/quality/full-gate.sh` with its ratchet, Module Contract and Scrap Sweep scripts, the pre-commit hook in `scripts/quality/hooks/`, and the CI workflow `.github/workflows/ci.yml`. There is no application (`rikurid`, CLI) or `smoke.sh` yet. `main` is protected by the ruleset "Protect main" (since 2026-10-06): changes arrive by pull request, and the `Full gate (Ubuntu)` check must pass on a branch that is up to date with `main`; no bypass. Check the tree each session; update this paragraph as Stage 00 adds them.

## Sources of authority

| Source | What it controls |
| --- | --- |
| [Stage Plan v2.1](doc/Rikuri_Pi_Foundation0_Rebaselined_Stage_Plan_v2.1.md) | Live Foundation-0 sequence, S/T item IDs, experiments, and gate board |
| [ADR 0000 and dated addendum](doc/Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md) | Foundation-0 scope reconciliation and preserved historical decisions |
| [Engineering Standard v1.2](doc/Rikuri_Pi_Engineering_Standard_v1.2.md) | SDLC, risk-scaled evidence, engineering and release discipline |
| [Engineering Templates v1.1](doc/Rikuri_Pi_Engineering_Templates_v1.1.md) | Build Records, Stage Gate Records, Module Contracts, traceability, debt, incidents |
| [Test and Acceptance Strategy v1.2](doc/Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md) | Contract, invariant, replay, failure, security, and integration verification |
| [Architecture v1.7](doc/Rikuri_Pi_Architecture_Spec_v1.7.md) and [PRD v1.2](doc/Rikuri_Pi_PRD_v1.2.md) | Architectural invariants and broader product requirements |
| [Master Build Sequence v1.1](doc/Rikuri_Pi_Master_Build_Sequence_v1.1.md) | Post-Foundation dependencies and product stages |
| [Mission Charter v1.0](doc/Rikuri_Pi_Mission_Charter_v1.0.md) | Why Rikuri exists, and the decision test for competing design choices (§12). Rationale only: the PRD and Architecture govern requirements, and the Stage Plan governs Foundation-0 |

This file operationalizes those sources; it does not create a competing roadmap. If a material conflict remains, identify it and reconcile the affected requirements, ADR, tests, and documentation before implementing the disputed behavior. Preserve accepted ADR history through a dated clarification or a linked superseding decision.

Foundation-0 stages are **00–05**; product stages are **0–10**. Product Stage 0 contains all Foundation-0 stages. Product Stage 4 (Model Lab) is not Foundation-0 Stage 04 (Pi/local coding). The 90-Day Build Plan is historical. Product P0 does not automatically mean Foundation-0 scope. Dates never waive gates.

## Five working laws

1. **Nothing important fails silently.** Required components have real callers, tests, health and last-success evidence. A fallback is observable; it cannot pretend the primary path succeeded.
2. **Stop the line on a required red gate.** Investigate and repair it before advancing dependent work. Never disable a check or weaken a security promise to obtain green. If blocked, record the failure and incomplete state honestly.
3. **Use standard work every time.** One coherent change gets one Build Record, scaled to risk, begun during planning.
4. **Detect absence of success.** Process liveness alone cannot prove useful work, successful backup, enforcement, or recovery.
5. **Turn defects into tripwires.** Significant defects require a regression test, invariant, monitor, ratchet, or refusal that catches the failure class.

## Deterministic gates

Instructions can be skipped; a hook cannot. The hook is a tripwire, not a sandbox: it catches the common forms of each action, not every possible one. In Claude Code sessions, `.claude/hooks/guard.py` runs before every file and shell tool call (wired in `.claude/settings.json`). It refuses reads of secret files, force-pushes, and test edits while the fix-lock is engaged. It asks the owner before edits to protected paths (this file, CLAUDE.md, REVIEW.md, `.gitignore`, Claude settings, the guard, CI, quality scripts, the Engineering Standard and Templates, the Test and Acceptance Strategy, ADRs, the Stage Plan, stage-gate records), pushes to `main`, `gh pr merge`, and GitHub API writes, workflow, secret, or history changes. Policy and worked examples: `.claude/hooks/guard-config.json`. `guard.py` and `test_guard.py` are shared unchanged with the owner's other full-tier repositories; change the policy file, not them.

- **Strictness is one command** (owner, 2026-09-27): `python3 .claude/hooks/guard.py profile development|stable`. `stable` asks at every gate above, for work close to completion. `development` (the current setting) asks only before deleting or archiving the GitHub repository, printing a GitHub token, and touching secret files; every other ask is allowed and logged to `.claude/state/guard-relaxed.log`, and the agent's own auto mode still judges it. Denies never change. Loosening asks the owner; tightening is free.
- A deny is not an obstacle to route around. Stop and ask the owner.
- Fixing a defect: write the failing test, see it fail, then `python3 .claude/hooks/guard.py fix-lock on <build-record>`. Tests and baselines become read-only until the owner releases the lock.
- Verify with the `verifier` subagent (`.claude/agents/verifier.md`); review per [REVIEW.md](REVIEW.md).
- The hook covers Claude Code only. Other agents follow the same rules by instruction.

## Per-change SDLC

### 1. Plan and characterize

- State the outcome in one sentence and identify affected S/T items, modules, dependencies, and wiring point.
- Read and, where implemented, exercise current behavior before proposing a fix. Record observed evidence, not assumptions.
- Begin a Build Record at `docs/build-records/YYYY-MM-DD-<slug>.md` using Template A. Routine low-risk work may use a brief record with scope, plan, wiring, verification, and disposition. Protected-core, security, money, execution, privacy, migration, deletion, recovery, and irreversible changes require expanded risk and negative-test evidence.
- Define success before coding: expected behavior, allowed and denied paths, failure/recovery behavior, and evidence to collect. Compilation alone is insufficient.
- For infrastructure, evaluate maintained components or established patterns before custom code. Record license, pinned version, trust boundary, lifecycle and integration cost, and failure behavior. Keep Rikuri's protected decisions under Rikuri authority.
- Apply the [ADR queue](doc/Rikuri_Pi_Foundation0_ADR_Starter_Pack_v1.0.md). Scope or invariant changes require explicit reconciliation; ordinary implementation choices within the accepted scope do not require a new approval ceremony.

### 2. Build and wire

- Implement the smallest coherent increment. Keep implementation, meaningful tests, migrations, and required documentation together.
- Validate inputs at trust boundaries; use typed contracts and explicit errors. Avoid unjustified type escapes or disabled checks. Review code files above roughly 800 lines; justify those above 1200 rather than splitting mechanically.
- Every existing protected crate needs a Module Contract: what it owns, must not own, dependencies, failure behavior, health, caller, and tests before its gate closes.
- Exercise the real entry point through production-style adapters. Unused abstractions, disconnected scaffolding, and successful mocks alone do not constitute a delivered feature.
- Commit resolved dependency lockfiles. Pin production container images by immutable digest when used; do not copy the reference procedure's blanket ban on compatible manifest ranges.
- Add minimum structured execution evidence to implemented paths: stable task/run/event IDs, exposed model/runtime/harness/tool and versioned profile identity, verification distinct from completion, outcomes, retries, cost/time, and intervention/feedback. Unknown fields stay explicit; privacy minimization still applies.

### 3. Debug and correct

- Reproduce the problem and inspect actual errors, traces, state, and configuration. Preserve relevant evidence before remediation; redact secrets.
- Use the least destructive action supported by evidence. Restarting, deleting state, or changing policy is not a substitute for diagnosis.
- Identify the defect class and search sibling paths. Add a meaningful permanent check and, where practical, demonstrate it fails against the old behavior in an isolated fixture or checkout.
- Follow Template F for incidents: stabilize, capture, determine cause, fix, sweep siblings, add tripwire, update runbook and evidence.

### 4. Verify

- Run checks appropriate to changed behavior: unit/failure, shared contracts, invariants, replay, integration, boundary/security, and recovery tests as applicable.
- Observe a real vertical slice. Test both legitimate use and rejection; hardening that breaks authorized use is a defect.
- Trigger changed fallback/recovery paths and verify both recovery and visible failure/health reporting. Use controlled fixtures or scratch environments.
- Keep core tests deterministic and runnable without API credentials. Fake and live adapters must share kernel, policy, ledger, budget, and verification paths. Live model output is not a deterministic oracle.
- Record commands, results, and durable artifacts. Distinguish **Pass**, **Fail**, and **Not yet proven**. Never invent successful test output or mark an unrun check green.

### 5. Close out

- Run the applicable repository gate, review the diff, and perform the Scrap Sweep below.
- Update affected docs, runbooks, help/RAG sources, requirements, and traceability in the same change. Mark nonexistent or unaffected surfaces N/A with a reason.
- Every required unchecked item needs a visible disposition. Deferrals record reason, dependency impact, owner, new stage/ordinal, and reactivation/acceptance condition; update downstream ordering.
- Finish the Build Record with results and limitations. Update the actual Stage Gate Record only when its work is affected; documentation changes cannot close implementation gates.
- Use coherent commits explaining why. Prefer a task branch and required CI/PR workflow once bootstrap establishes it. Do not force-push, overwrite unrelated work, or treat a code-change request as authorization to deploy.
- Report what changed, what was verified, failures or unavailable checks, and remaining work. A blocked session can end with an honest incomplete handoff; it cannot report completion.

## Verification by repository maturity

**Documentation-only work now:** validate local links, current version references, stage/item references, scope consistency, and `git diff --check`; inspect `git status` and the final diff. Runtime tests are N/A when no executable behavior changes. Do not create trivial tests solely to satisfy a checkbox. When `.claude/` changes, run `python3 -m unittest discover -s .claude/hooks -p 'test_*.py'`. `.github/workflows/claude-guard.yml` runs the same tests on GitHub; do not call it green until a run has been observed.

**Stage 00 bootstrap (in progress):** run `scripts/quality/full-gate.sh` (setup in README > Running the full gate) for every change; it includes the guard tests. `scripts/quality/smoke.sh` is still planned and arrives with `rikurid` (S00.07). Protected `main` is in place, and the red-test drill (including "blocks") and the secret-scan drill passed (2026-10-05/06). Still to establish: the macOS core checks observed green on `main` (they first run when PR #7 merges). Do not claim CI or branch protection works until a run has been observed.

**Once implemented:** use the actual documented gate commands and CI configuration. The full gate grows with the codebase; add frontend type/static checks when that stack exists. Smoke must check meaningful content and behavior, not merely an open port. Inspect configuration without printing credentials. If a required tool/platform is unavailable, record the gap and leave its dependent claim unproven.

## Rikuri boundaries to preserve

- **Canonical authority:** Rikuri owns state and transactions. Models, Pi, tools, workers, UIs, and derived indexes do not directly mutate canonical storage. Use governed service interfaces and validated promotion.
- **Instruction authority:** imported, retrieved, model-generated, and tool content is untrusted data. It cannot grant permissions, alter approval consequences, or override user intent.
- **Execution trust:** read/inspect never implies execute. Projects require scoped trust promotion; child processes inherit the same envelope. Never weaken host-wide controls to make a sandbox pass.
- **Privacy and money:** Local Only is refused before cloud HTTP construction and runs only in a proven no-network envelope. Hard budgets reserve before calls; unknown maximum cost cannot silently bypass a hard ceiling.
- **Recovery:** preserve last known-good state, validate candidates, durably write/read back, atomically promote, and prove rollback/replay. Managed pre-forget restore consults current revocation; absent trustworthy state, refuse or quarantine. External exports are outside the forget guarantee.
- **Platform assurance:** Ubuntu is Foundation-0's full execution/security reference. macOS platform-neutral core validation is required; native macOS containment and desktop parity remain experimental until proven.
- **Scope:** Foundation-0 has six stages, W1/F1–F10, Bootstrap and F4-lite. Full F4 closes at Stage 04; the ten knowledge-compiler cases must include correct answers as well as abstentions. All eight crash points rerun at release.
- **Instrument early; infer late:** T00.12/T02.16/T04.18 prove evidence capture. Autonomous reflection, user-trait inference, learned routing, Lightbulbs, incubation, and policy learning remain deferred. Writer's domain foundation/MVP is not Foundation-0.

## Scrap Sweep and shared-work safety

Inspect tracked changes, untracked task files, and relevant working directories. Remove task-created temporary probes, replaced code, debug residue, and proven orphans; inspect dynamic registrations, scripts, adapters, tests, and runbooks before declaring a file unused.

Retain reproducible migrations, rollback tools, fixtures, evidence receipts, and historical documents when the supported lifecycle or audit trail needs them. An applied migration is not automatically scrap. Preserve user files, other sessions' changes, stashes, branches, backups, and durable state; age or lack of an obvious caller is not deletion authority. Record ambiguous cleanup as owned debt rather than deleting blindly.

Skipped tests, quarantined code, and temporary exceptions must be counted, scheduled, owned, and gated. No silent debt increases, lingering disabled enforcement, or unexplained alternate implementation.

## Periodic operation and release

Once runtime components exist, follow the Engineering Standard's weekly health/fallback reviews, monthly restart/failover/mode-switch/restore drills, and quarterly security/dependency/architecture reviews. Capture outcomes and turn failures into checks. These are planned operational obligations, not instructions to run destructive drills against live user state or install unattended jobs during an unrelated task.

Internal Foundation-0 completion requires its stage-plan gates and owner-runnable demos with linked evidence. Public exposure has a separate release gate, including clean-machine operation, soak evidence, and human external security review. Agent review or an attractive demo cannot substitute for those requirements.
