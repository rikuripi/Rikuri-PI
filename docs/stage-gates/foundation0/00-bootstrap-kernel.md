# Stage Gate Record — Foundation-0 Stage 00: Bootstrap + Kernel

| Field | Value |
|---|---|
| Project | Rikuri Pi Foundation-0 |
| Stage | 00 — Bootstrap + Kernel |
| Owner | Henry Chen (product owner); stage implementer per Build Record |
| Planned start | 2026-10-01 |
| Planned exit | mid-November 2026 (estimate; dates never close a gate) |
| Status | NOT STARTED |
| Architecture version | v1.7 with ADR 0000 addenda through September 28, 2026 |

The live checklist, item wording and acceptance tests are in the [Stage Plan, Stage 00](../../../doc/Rikuri_Pi_Foundation0_Rebaselined_Stage_Plan_v2.1.md#stage-00--bootstrap--kernel). This record does not copy them. It tracks status, Build Records, blockers and evidence against those item IDs.

## Goal

`rikurid` survives kill -9 at every kernel transaction point with rollback or deterministic recovery and no duplicate effects, CI provably blocks bad work and planted secrets, and the two early feasibility questions are answered with recorded evidence: whether Pi can be driven headlessly and contained (E2), and whether the pinned Ubuntu execution backend can be admitted without weakening the host (S00.13).

## Required deliverables

Build items S00.01–S00.13 as worded in the Stage Plan. S00.12 (E2) includes the harness-placement decision; S00.13 includes the approval-channel and host-privilege checks ([ADR 0000, September 28 review follow-ups](../../../doc/Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md#september-28-2026--option-c-review-follow-ups-and-foundation-0-credential-scope)).

## Required acceptance / demonstrations

- [ ] T00.01–T00.12 green with linked evidence — evidence:
- [ ] Owner demo: run `init`, create a sample task, see the same state after a forced restart, and see a deliberately broken change fail CI — evidence:
- [ ] Receipt format agreed with the owner before the first build change (Stage Plan next actions) — evidence:

## Required engineering gates

- [ ] All stage modules wired through real call paths / vertical slice (CLI → kernel → repository → ledger).
- [ ] Failure/security/health integration evidence linked.
- [ ] Any deferral has reordered downstream checklist entries.
- [ ] Bootstrap gate and W1 gate green on the live gate board.
- [ ] E1 and E2 outcomes and the S00.13 admission probe recorded, each with a Build Record and probe output.
- [ ] Platform-neutral macOS core suite green.
- [ ] Module Contracts for domain, storage, ledger and testkit.
- [ ] ADR 0001–0004 accepted.
- [ ] Required Build Records complete.
- [ ] Required CI/full-gate green; smoke/running-system probe passed.
- [ ] Relevant contract/property/replay/security suites green.
- [ ] Documentation/help current; ADRs current; Module Contracts current.
- [ ] Threat/control/test traceability current where applicable.
- [ ] Scrap Sweep/clean-tree review complete.
- [ ] No unexplained skipped tests or unowned debt increase.

## Progress

- none yet

## Blockers

- none

## Skipped / Deferred

| Item | Disposition | Reason | Dependency impact | Owner | New stage/ordinal | Reactivation / acceptance condition |
|---|---|---|---|---|---|---|
| none | | | | | | |

## Evidence

| Claim | Evidence |
|---|---|
| — | — |

## Exit decision

- [ ] Every required item above is satisfied or explicitly dispositioned.
- [ ] Stage owner marks COMPLETE only after acceptance evidence exists.
- [ ] Next stage may start.
