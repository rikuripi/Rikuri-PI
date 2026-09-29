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

## 2. Standard Work

This document governs how software gets built here: planning, building,
debugging, testing, review, and the periodic loops that keep a system honest
over time. It is portable — copy it into any new project's repo and follow it
from day one. AI assistants working in a repo containing this file are bound
by it.

The goal is Toyota-style quality: not "never make mistakes," but **a system
where mistakes cannot travel** — every defect is caught by the nearest gate,
made loud, and converted into a permanent tripwire.

---

## The Five Laws (apply to every section below)

**Reuse rule:** Evolution for existing solutions; revolution for core strength. Before building routine orchestration or infrastructure, inventory a maintained implementation or established pattern and record license, trust boundary, operational failure modes, lifecycle fit, maintenance cost, and measured acceptance. Document the reason if custom code is chosen. Keep AISecurity, verified context, canonical ProjectState, and transaction integrity under Rikuri authority. An upstream feature list is not evidence that a Rikuri vertical slice works.

**Future-learning evidence rule:** Instrument early; infer late. Every meaningful feature/task path must emit enough structured, privacy-governed evidence to reconstruct what was attempted, with which execution/profile/tool configuration, what actually happened, how it was verified, and where human intervention occurred. Do not add continuous surveillance merely for future learning. Do not let future learning requirements weaken data minimization.

1. **Nothing is silent.** Every component either runs and is gated by a test,
   or it is deleted. A fallback that engages is an ALERT, never just a save.
   (Case study: a watchdog's primary restart path was dead for weeks while the
   sudo fallback quietly recovered everything — the audit trail even recorded
   the dead mechanism as having run.)
2. **Stop the line.** A red gate (build, typecheck, test, smoke) halts all
   other work until green. No "I'll fix it after this feature." No skipped,
   excluded, or `@ts-nocheck`'d code without a counted, scheduled debt entry.
3. **Standard work.** The checklists below are walked mechanically, every
   time, regardless of how small the change feels. Quality must not depend on
   anyone being sharp that day.
4. **Alert on the absence of success, not just the presence of failure.**
   Heartbeats over error handlers. "The upgrade checker must see a repo push
   within 90 days or complain." (Case study: a pinned Docker repo died at
   exactly the running version — the checker reported "up to date" for 18
   months because it only compared against what the dead repo published.)
5. **Encode every lesson as a gate.** A bug fixed is a bug class still alive.
   Every incident ends with a tripwire — a test, a ratchet, a monitor, a
   refusal-to-boot — that makes the whole CLASS impossible or loud.

---

## Phase 0 — Project Bootstrap (before the first feature)

Do these once, at project start. A project without them is a prototype, not
a product.

- [ ] **Version control** with a protected main branch; work happens on
      branches; every merge has a green CI run.
- [ ] **CI that actually gates**: build + typecheck + unit tests + lint +
      secret scan, none marked continue-on-error. Prove it with a
      **red-test drill**: plant a failing test, confirm CI goes red, remove it.
- [ ] **Pre-commit hooks**: typecheck + secret scan (gitleaks or equivalent)
      minimum.
- [ ] **A smoke script** (`scripts/smoke.sh` or equivalent): one command that
      probes every service, port, and page the system claims to provide, with
      distinct content markers (not just "port answers"), and echoes the
      active configuration. This is the session-exit gate forever after.
- [ ] **A debt ratchet**: counted metrics that may only go down
      (`@ts-nocheck` count, `any` count, TODO count, script-style tests…).
      Baselines committed; CI fails on any rise.
- [ ] **Secrets hygiene**: no secret in the repo, ever; secrets live in
      env files/keychains outside the tree; document where each one lives.
- [ ] **Reproducible dependency policy**: resolved dependencies are exact in committed lockfiles; production container images are pinned by immutable digest when containers are used. Ecosystem-appropriate compatible ranges may remain in manifests, but floating deployment tags such as `:latest` are forbidden.
- [ ] **Runtime truth over declared truth**: version/status displays must
      probe the running process, never trust a tag, a cache, or a config.

---

## The Per-Change Loop (every feature, every fix — no exceptions)

### 1. PLAN (before writing code)
- [ ] State the change in one sentence. If you can't, split it.
- [ ] Identify the blast radius: which modules, which data, which auth/funds/
      exec paths does this touch? Anything irreversible?
- [ ] **Characterize before you change**: read the current behavior and prove
      your understanding (run it, log it, test it). Plans built on assumed
      behavior produce fixes for bugs that don't exist and miss the ones that
      do. (Case study: three "big delete" refactor phases found the deletions
      already done — and one "dead" module that was load-bearing.)
- [ ] Decide the verification BEFORE building: "I will know this works when
      ___." If the answer is "it compiles," you don't have a plan yet.
- [ ] Name the **wiring point**: which real caller/entry point will exercise this change before the stage closes?
- [ ] Identify predecessor/successor checklist dependencies. If this item slips, record which downstream work must move with it.

### 2. BUILD
- [ ] Smallest coherent increment; one coherent concern per commit. A commit may include its implementation, tests, schema/docs, and migration when those pieces are required to make the change independently understandable and buildable.
- [ ] New code meets the bar: typed (no new unjustified `any`/`@ts-nocheck`), validated at trust boundaries, documented where non-obvious, and kept cohesive. Files above ~800 LOC trigger a review warning; above ~1200 LOC require explicit justification rather than mechanical splitting.
- [ ] **Clean as you go**: the old version of anything you replace is removed
      in the same change. Scaffolding and probe scripts die with the session.
- [ ] Security question, asked explicitly every time: *does this change touch
      authentication, money, execution, private data, project trust, child processes, or egress?* If yes: which
      existing gate covers it, and is there a test proving the gate holds?
      A hardening pass that breaks a legitimate path is also a defect —
      check both directions. (Case study: a loginless-mode hardening
      accidentally gated the login endpoint itself behind login.)

### 3. DEBUG (when something is wrong)
- [ ] Reproduce first. A fix without a reproduction is a guess.
- [ ] Read the actual error/log — not what you expect it to say. The clue is
      usually verbatim in there. (Case study: a "stuck" node was printing
      `bad-version-reduced_data` — a consensus rejection, not a hang; every
      restart-style "fix" would have been wrong.)
- [ ] Fix the CLASS, not the instance: after the fix works, ask "where else
      does this exact pattern exist?" and sweep. (Case study: one raw-template
      copy bug appeared in three different install scripts.)
- [ ] Before any state-changing remediation (restart, delete, config change):
      does the evidence support THIS action, or does the symptom just
      pattern-match a known failure?

### 4. TEST / VERIFY
- [ ] Unit tests for the logic you added — including the failure paths.
      A test that only covers the happy path certifies nothing.
- [ ] **Watch it actually work** in the running system — the real page, the
      real endpoint, the real container. "Tests pass" and "it works" are
      different claims; make both.
- [ ] If the change alters recovery/fallback behavior: **trigger it**. A
      recovery path that has never been exercised is a hope, not a feature.
- [ ] Regression guard: for any bug you just fixed, prove the test fails on
      the OLD code (revert briefly if needed). A guard that never failed
      guards nothing.
- [ ] **Wire test:** exercise the module through the real production-style entry point/adaptor path, not only directly through its unit-test API.
- [ ] **Boundary test:** test the relevant trust/privacy/budget/project-execution boundary in both allowed and denied directions.
- [ ] **Health test:** prove success updates last-success/health evidence and failure becomes visible/degraded rather than silently falling back.

### 5. CLOSE OUT (end of every change/session)

In addition to ordinary build evidence, preserve the minimum structured execution/outcome evidence required by the Collaborative Intelligence scaffold: stable task/run reference, exact execution/profile identity where exposed, verification result, significant failure/retry/fallback, cost/time when available, and material human intervention. This is evidence capture, not behavioral inference.
- [ ] Run the full gate: build → typecheck → unit tests → ratchet → smoke.
      All green or the session isn't over.
- [ ] **The Scrap Sweep** (every session, no exceptions — see the taxonomy
      below): `git status` plus a look at the working directories. Everything
      present is the real change or a deliberate, load-bearing new file.
- [ ] Update the knowledge layer IN THE SAME CHANGE: docs / RAG / runbooks /
      user-facing help for whatever behavior changed. Stale docs are silent
      failures too.
- [ ] Commit with a message that explains WHY (the incident, the class, the
      decision) — the diff already shows the what.
- [ ] Deploy = build: one scripted path from source to running system. Never
      hand-copy artifacts; drift between source and running code is a
      slow-motion outage.
- [ ] Every unchecked item has a disposition. For `DEFERRED/RESEQUENCED`: write reason, dependency impact, new stage/position, owner, trigger/acceptance condition, and update the master/stage checklist ordering now.
- [ ] Do not begin the next dependent function/feature while this stage has an unresolved required wiring gate.

---

## The Scrap Taxonomy — what "clean" means, precisely

Scrap is anything that helped you get to the change but is not the change.
It is removed **the same session it was created** — scrap left overnight
becomes "maybe load-bearing" by next week and permanent by next month. The
rule in both directions: **if it's junk, dump it (prove it first — zero
importers/callers/unit/systemd/runbook references); if it should exist, wire
it in and gate it. "Sitting there quietly" is never a third state.**

Delete on sight, every session:
- [ ] **Scaffolding & probe scripts** — debug scripts, curl/test one-liners
      saved as files, temporary harnesses. If it was the road to the fix and not
      the fix, it dies with the session (scratchpad if worth keeping, never
      the repo).
- [ ] **One-shot scripts that have been applied** — migration/apply/fixup
      scripts are DANGEROUS after they run: re-running them reinstalls the
      old state. (Case study: an applied one-shot would have reinstalled a
      renamed-away systemd unit; a different one actually DID install a raw
      config template and silently killed the watchdog for four days.)
      Their knowledge lives on in the commit message and runlog; the script
      does not.
- [ ] **The old version beside the new** — a replaced function, route, file,
      or config left "just in case." The version control system is the
      just-in-case; the working tree is not.
- [ ] **Orphans** — code with zero reachable callers from real entry points.
      Prove it (imports, shell scripts, systemd units, runbooks, tool
      registrations, dynamic loading), then delete. Beware: a dead barrel
      can re-export LIVE children — verify per child.
- [ ] **Debug residue in real code** — console.log/print left from
      diagnosis, commented-out blocks, temporarily-loosened checks,
      `.skip`'d tests, hardcoded test values.
- [ ] **Stray files** — `.bak`, `.old`, `.deleted`, editor swap/lock files
      (these have leaked SECRETS from config dirs before), untracked files
      nobody can name a purpose for, stale build artifacts (sweep with a
      CLEAN build — stale artifacts make deleted code look alive).
- [ ] **Stale state** — expired cache files, failure records from bugs since
      fixed, stashes older than a day, branches whose work landed.

Quarantine is never a resting state: anything excluded from CI, `@ts-nocheck`'d,
or `.skip`'d is a **counted, ratcheted, scheduled** debt with a named phase in
which it becomes running-and-gated or deleted.

---

## Periodic Loops (calendar-driven, not mood-driven)

Silent failures are found by time and use, never by the build loop. Schedule
these; do not rely on remembering.

### Weekly (~15 minutes)
- [ ] Run smoke; skim service/upgrade/notification pages.
- [ ] Grep logs for engaged FALLBACKS and repeated warnings — a fallback
      firing weekly is a primary path dying quietly.
- [ ] `git status` on all machines: no uncommitted drift older than a week.

### Monthly (one session) — DRILL, on rotation
Pick one; rotate through all of them over the quarter:
- [ ] **Restart drill**: clean reboot; zero manual interventions allowed.
- [ ] **Failover drill**: kill the primary of something (process, container,
      config); verify detection, recovery, AND that the recovery was loud.
- [ ] **Mode-switch drill**: exercise every supported configuration
      round-trip (engines, networks, feature flags), verifying state and
      funds/data integrity at each hop.
- [ ] **Restore drill**: restore a real backup into a scratch environment.
      An unrestored backup is Schrödinger's backup.
After every drill: findings become fixes become tripwires (Law 5).

### Quarterly (one to two sessions) — ADVERSARIAL REVIEW
- [ ] **Security pass** over auth/funds/exec paths: walk every mutation route
      and name its gate; try the obvious bypasses; verify rate limits and
      lockouts fire.
- [ ] **Dead-code sweep**: import-reachability from real entry points; every
      orphan proven-then-deleted. Verify with a CLEAN build — stale build
      artifacts hide deletions.
- [ ] **Dependency & image audit**: every pin still from a LIVING source
      (check last-publish dates, not just versions); EOL check on every
      runtime (language, OS, daemons).
- [ ] **Architecture check**: do the docs still describe the system? Is every
      documented invariant still tested? One impl per concern still true?
- [ ] **Fresh-eyes review**: automated multi-agent code review of recent
      changes, plus — before any public exposure — a human external audit of
      the security-critical paths.

---

## Incident Procedure (when something breaks in production)

1. **Stabilize** with the least destructive action that evidence supports.
2. **Capture** the state before "fixing" erases it (logs, files, versions).
3. **Root-cause to the class**: "the dashboard 502'd" is a symptom;
   "compose-managed containers lose their network endpoint when the network
   is recreated under them" is a class.
4. **Tripwire it** (Law 5): a test, monitor, guard, or refusal that makes the
   class loud or impossible.
5. **Record it** where the next session will find it: incident log / runbook /
   project memory — including what the false leads were.
6. **Sweep for siblings**: the same class usually lives in more than one place.

---

## Working with AI Assistants (how this document gets enforced)

Rikuri is its own first Collaborative Intelligence research subject. Claims that a prompt profile, harness, model, workflow, framing method, or learned heuristic improves work must be tested prospectively against an appropriate baseline/challenger where practical. Failures and negative results are retained as evidence. No AI-generated explanation of success substitutes for an outcome gate.

- Point every AI session at this file; its checklists are binding.
- Demand **verification, not claims**: "tests pass" must come with the test
  run; "it works" must come with the probe of the running system.
- The AI must report failures verbatim — a session that only reports
  successes is hiding the interesting parts.
- AI-written code follows the same ratchets, the same gates, the same
  clean-as-you-go rule. Speed is not an exemption; it is the reason the
  gates exist.
- Multi-session hygiene: sessions do not commit each other's in-flight work;
  uncommitted changes older than a day get dispositioned (finished, stashed,
  or deleted), never left ambient.

---

## Security assurance and upstream maintenance

September 28, 2026: protecting the user's private, financial, personal/media, professional/proprietary data and secrets is the highest product responsibility. [Option C](Rikuri_Pi_Architecture_Spec_v1.7.md#2712-option-c-policy-ownership-and-enforcement-adapters) reuses qualified enforcement components while Rikuri retains permission and release accountability. Each supported path needs a threat/control/test mapping, positive and negative controls, loss/revocation/recovery evidence, and an explicit platform/version/configuration boundary. Preserve failed experiments and limitations alongside successes.

Before admitting or upgrading a backend, record version/license/advisories, provenance and immutable production image digests, privilege and service dependencies, policy/provider translation, resource cost, maintenance ownership and rollback. Test optional telemetry disabled and inspect control-plane egress. Requalify affected guarantees when dependencies, kernel, driver, images, provider bindings or policy semantics change. A revoked grant must stay revoked after rollback. A required regression or loss of assurance stops dependent execution and release; removing the failing test or quietly switching to an unrestricted adapter is prohibited.

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
