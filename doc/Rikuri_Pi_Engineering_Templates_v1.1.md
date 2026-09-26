# Rikuri Pi — Engineering Templates

> Repository consistency revision: September 25, 2026. Source version retained; see [reconciliation record](reviews/2026-09-25-documentation-consistency.md).

**Version:** 1.1 Draft  
**Purpose:** Canonical checklist/evidence templates used by the Engineering Standard and Foundation-0 build plan.

---

# Template A — Build Record

> Copy to `docs/build-records/YYYY-MM-DD-<slug>.md` when planning begins. Check boxes while work happens. An unchecked required box without an explicit SKIPPED/DEFERRED record means the change is not done.

| Field | Value |
|---|---|
| Change | |
| Date | YYYY-MM-DD |
| Author / agent session | |
| Branch | |
| Commits | fill at close-out |
| Stage Gate | |
| Related ADR / requirement | |
| Status | IN PROGRESS / COMPLETE / ABANDONED |

## 1. Plan

- [ ] One-sentence statement of change:
- [ ] Blast radius: modules, canonical data, auth/money/exec/private-data paths, irreversible effects:
- [ ] Current behavior characterized before changing; evidence:
- [ ] Verification defined up front — “I will know this works when…”:
- [ ] Architecture-impact check completed:
  - [ ] invariant / ADR
  - [ ] API / schema / event format
  - [ ] canonical vs derived data
  - [ ] privacy / instruction authority / security
  - [ ] deletion / backup / restore
  - [ ] budget / cost
  - [ ] nondeterministic AI / replay
  - [ ] documentation / help / RAG
  - [ ] module ownership / dependency direction
- [ ] Relevant threat/control/test identified where applicable:
- [ ] Existing solution review (for infrastructure): pinned version/license, comparable mechanism, adoption or adaptation cost, failure tests, and reason for custom work if chosen:
- [ ] Authoritative state change? Preserve -> validate -> candidate -> validate -> durable write -> read-back -> atomic promote -> archive; injected failure/rollback evidence:

## 2. Build

- [ ] Smallest coherent implementation completed.
- [ ] Inputs crossing trust boundaries validated.
- [ ] New code is typed and cohesive; large-file warning/justification handled.
- [ ] Old/replaced implementation removed in the same change.
- [ ] Module Contract updated if responsibility/interface changed.
- [ ] Security question answered: auth / money / execution / private data touched? Which gate/test proves both allowed and denied paths?

## 3. Debug — mark N/A if not a defect

- [ ] Problem reproduced before fix; evidence:
- [ ] Root-cause **class** named:
- [ ] Sibling cases swept:
- [ ] State-changing remediation justified by evidence rather than pattern matching:

## 4. Verify

- [ ] Unit tests added/updated, including failure paths; result:
- [ ] Shared contract suite run if boundary/adapter changed; result:
- [ ] Invariant/property test added/run if protected guarantee changed; result:
- [ ] Replay/golden fixture updated if nondeterministic path changed; result:
- [ ] Running-system probe observed; result:
- [ ] End-to-end action -> routing/policy -> tool -> verified durable result traced; handoff IDs/ACKs and typed stale/truncated failures inspected where applicable:
- [ ] Recovery/fallback path triggered live if touched; result:
- [ ] Regression guard proven to fail against intentionally broken/old behavior when applicable:
- [ ] Cost/performance/retrieval benchmark checked if relevant:

## 5. Close Out

- [ ] Full required gate green: format/build/static/unit/contract/property/replay/security/ratchet/smoke as applicable.
- [ ] Scrap Sweep complete; deleted artifacts listed:
- [ ] No unexplained skipped/disabled tests or new unowned debt.
- [ ] Documentation / RAG / runbook / user help updated in the same change.
- [ ] Traceability updated if security/architecture promise changed.
- [ ] Stage Gate Record updated with this change/evidence.
- [ ] Commit message explains WHY; Build Record committed with change.

## Skipped / Deferred / Resequenced — visible, never silent

> Every unchecked required item above gets a line here with reason, owner, and due date/stage. “To save time” is not a reason.

| Item | Disposition | Reason | Dependency impact | Owner | New stage/ordinal | Reactivation / acceptance condition |
|---|---|---|---|---|---|---|
| none | | | | | | |

For execution changes, link the stable run/outcome, versioned execution/profile identity where exposed, verification result, significant retry/failure/intervention, and cost/time where available. Record unknown fields explicitly; exclude unauthorized sensitive telemetry. Documentation-only work records these fields as N/A.

## Evidence log

```text
Paste receipts: commands, test totals, smoke RESULT line, replay IDs, benchmark before/after, probe output, CI link/reference, screenshots/artifacts where relevant.
```

---

# Template B — Stage Gate Record

> Copy to `docs/stage-gates/<project>/<NN-stage-slug>.md` when the stage begins.

| Field | Value |
|---|---|
| Project | |
| Stage | |
| Owner | |
| Planned start | |
| Planned exit | |
| Status | NOT STARTED / IN PROGRESS / BLOCKED / COMPLETE |
| Architecture version | |

## Goal

One paragraph describing what must become true, not a list of implementation tasks.

## Required deliverables

- [ ] Deliverable 1
- [ ] Deliverable 2

## Required acceptance / demonstrations

- [ ] Acceptance criterion 1 — evidence:
- [ ] Acceptance criterion 2 — evidence:

## Required engineering gates

- [ ] All stage modules wired through real call paths / vertical slice.
- [ ] Failure/security/health integration evidence linked.
- [ ] Any deferral has reordered downstream checklist entries.

- [ ] Required Build Records complete.
- [ ] Required CI/full-gate green.
- [ ] Smoke/running-system probe passed.
- [ ] Relevant contract/property/replay/security suites green.
- [ ] Documentation/help current.
- [ ] ADRs current.
- [ ] Module Contracts current.
- [ ] Threat/control/test traceability current where applicable.
- [ ] Scrap Sweep/clean-tree review complete.
- [ ] No unexplained skipped tests or unowned debt increase.

## Progress

- [ ] Work item / Build Record:
- [ ] Work item / Build Record:

## Blockers

- none

## Skipped / Deferred

| Item | Disposition | Reason | Dependency impact | Owner | New stage/ordinal | Reactivation / acceptance condition |
|---|---|---|---|---|---|---|
| none | | | | | | |

## Evidence

| Claim | Evidence |
|---|---|
| | |

## Exit decision

- [ ] Every required item above is satisfied or explicitly dispositioned.
- [ ] Stage owner marks COMPLETE only after acceptance evidence exists.
- [ ] Next stage may start.

---

# Template C — Module Contract

> Store in the protected-core crate/module `README.md` or `docs/modules/<module>.md`.

# <module-name>

## Purpose

## Owns

- 

## Must not own

- 

## Trust level / boundary

## Canonical data touched

## Public contracts / interfaces

## Allowed dependencies

## Forbidden dependencies

## Failure and degraded behavior

## Health / last-success signal

```text
state:
last_attempt:
last_success:
last_error:
consecutive_failures:
work_completed:
```

## Architecture / ADR links

- Invariants:
- ADRs:

## Threat / control links

- Threat:
- Control:

## Required tests

- [ ] unit
- [ ] failure paths
- [ ] shared contract suite
- [ ] invariant/property tests
- [ ] replay/golden fixtures
- [ ] integration/smoke

## Documentation / help entry points

## Known limitations / explicit debt

---

# Template D — Threat / Requirement / Control / Test Traceability

| ID | Threat or requirement | Deterministic control | Test / drill | Evidence | Status |
|---|---|---|---|---|---|
| | | | | | |

Rules:

- a protected promise with no named control is incomplete;
- a control with no test/drill is unproven;
- evidence must come from an actual run or durable artifact, not an agent claim;
- when a control changes, update the linked test and ADR/Build Record in the same change.

---

# Template E — Engineering Debt Entry

| Field | Value |
|---|---|
| Debt ID | |
| Introduced by Build Record | |
| Reason | |
| Why immediate removal is unsafe/impractical | |
| Scope | |
| Risk | |
| Owner | |
| Due date/stage | |
| Ratchet metric affected | |
| Removal verification | |

Debt is visible and scheduled. Quarantine is not a resting state.

---

# Template F — Incident / Tripwire Record

| Field | Value |
|---|---|
| Incident | |
| Date | |
| Severity | |
| Affected version/module | |
| Related project/stage | |

## 1. Stabilize

What least-destructive action restored safety/service?

## 2. Capture

Logs, versions, state, screenshots, traces gathered before remediation:

## 3. Root-cause class

Describe the general defect class, not just the symptom.

## 4. Fix

## 5. Sibling sweep

Where else could the same class exist?

## 6. Permanent tripwire

- [ ] regression test
- [ ] invariant/property test
- [ ] monitor/last-success alert
- [ ] refusal/policy gate
- [ ] ratchet
- [ ] other:

## 7. Documentation / runbook update

## 8. Evidence and close-out

- [ ] Tripwire demonstrated against intentionally broken/old behavior where practical.
- [ ] Build Record linked.
- [ ] Stage Gate/project memory updated.
