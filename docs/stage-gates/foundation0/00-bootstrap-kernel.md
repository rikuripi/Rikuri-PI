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

## Reference installation (owner decision, 2026-09-28)

- **Machine:** the owner's current development machine. It also runs the owner's Bitcoin node containers on a rootful Docker engine; Rikuri-PI's sandbox controller uses a dedicated account or a rootless engine (S00.13) and never that engine.
- **Now:** Ubuntu 24.04.5 LTS; running kernel (`uname -r`) 6.8.0-138-generic. Kernel 6.8.0-142 is installed and takes effect at the next reboot.
- **Principle:** the reference runs the newest stable Ubuntu LTS with that release's standard (GA) kernel. No hardware-enablement (HWE), OEM or mainline kernels. Unless the owner decides otherwise, kernel series changes happen only when the reference adopts a new LTS.
- **Before S00.13:** upgrade to Ubuntu 26.04 LTS with its standard 7.0 kernel through Ubuntu's supported upgrade path (owner action, a node maintenance event outside this repository). S00.13 runs on it; it is admission evidence, not qualification.
- **Upgrade path status (2026-09-28):** not yet offered. `do-release-upgrade -c` reports no new LTS, and Ubuntu's `meta-release-lts` lists `resolute` 26.04.1 LTS with `Supported: 0`. S00.01–S00.12 do not depend on it. If it is still unavailable when S00.13 is due, the owner chooses between waiting, a fresh 26.04 install, or running S00.13 on 24.04 as provisional admission evidence that the 26.04 move then repeats. Record the choice here.
- **Full qualification:** E5/T03.x/Test Strategy §8.2 runs once at Stage 03 on 26.04's 7.0 kernel. Afterwards, kernel security patches in the 7.0 series use the automated tier-1 rerun; everything else follows the Engineering Standard's [two-tier rule](../../../doc/Rikuri_Pi_Engineering_Standard_v1.2.md#security-assurance-and-upstream-maintenance).
- **Next LTS:** move to the next LTS (expected 28.04) once its first point release is out and Ubuntu enables LTS upgrades. That move is a planned tier-2 full requalification, scheduled between stages, never inside Stage 03.
- **Not planned:** the 26.04 HWE kernels (which roll to newer series, possibly 7.3 from 26.10, about every six months until the final one). Moving to one is a deliberate owner decision and a tier-2 requalification ([ADR 0000 addendum](../../../doc/Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md#september-28-2026--reference-installation-and-requalification-tiers)).
- [ ] Upgrade done — evidence (`/etc/os-release`, `uname -r`):

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
