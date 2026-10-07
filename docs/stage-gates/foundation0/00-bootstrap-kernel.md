# Stage Gate Record — Foundation-0 Stage 00: Bootstrap + Kernel

| Field | Value |
|---|---|
| Project | Rikuri Pi Foundation-0 |
| Stage | 00 — Bootstrap + Kernel |
| Owner | Henry Chen (product owner); stage implementer per Build Record |
| Planned start | 2026-10-01 |
| Planned exit | mid-November 2026 (estimate; dates never close a gate) |
| Status | IN PROGRESS (since 2026-10-05) |
| Architecture version | v1.7 with ADR 0000 addenda through September 28, 2026 |

The live checklist, item wording and acceptance tests are in the [Stage Plan, Stage 00](../../../doc/Rikuri_Pi_Foundation0_Rebaselined_Stage_Plan_v2.1.md#stage-00--bootstrap--kernel). This record does not copy them. It tracks status, Build Records, blockers and evidence against those item IDs.

## Reference installation (owner decision, 2026-09-28)

- **Machine:** the owner's current development machine. It also runs the owner's Bitcoin node containers on a rootful Docker engine; Rikuri-PI's sandbox controller uses a dedicated account or a rootless engine (S00.13) and never that engine.
- **On 2026-09-28:** Ubuntu 24.04.5 LTS; running kernel (`uname -r`) 6.8.0-138-generic. Kernel 6.8.0-142 is installed and takes effect at the next reboot.
- **Principle:** the reference runs the newest stable Ubuntu LTS with that release's standard (GA) kernel. No hardware-enablement (HWE), OEM or mainline kernels. Unless the owner decides otherwise, kernel series changes happen only when the reference adopts a new LTS.
- **Before S00.13:** upgrade to Ubuntu 26.04 LTS with its standard 7.0 kernel through Ubuntu's supported upgrade path (owner action, a node maintenance event outside this repository). S00.13 runs on it; it is admission evidence, not qualification.
- **Upgrade path status (2026-09-28):** not yet offered. `do-release-upgrade -c` reports no new LTS, and Ubuntu's `meta-release-lts` lists `resolute` 26.04.1 LTS with `Supported: 0`. S00.01–S00.12 do not depend on it. If it is still unavailable when S00.13 is due, the owner chooses between waiting, a fresh 26.04 install, or running S00.13 on 24.04 as provisional admission evidence that the 26.04 move then repeats. Record the choice here.
- **Full qualification:** E5/T03.x/Test Strategy §8.2 runs once at Stage 03 on 26.04's 7.0 kernel. Afterwards, kernel security patches in the 7.0 series use the automated tier-1 rerun; everything else follows the Engineering Standard's [two-tier rule](../../../doc/Rikuri_Pi_Engineering_Standard_v1.2.md#security-assurance-and-upstream-maintenance).
- **Next LTS:** move to the next LTS (expected 28.04) once its first point release is out and Ubuntu enables LTS upgrades. That move is a planned tier-2 full requalification, scheduled between stages, never inside Stage 03.
- **Not planned:** the 26.04 HWE kernels (which roll to newer series, possibly 7.3 from 26.10, about every six months until the final one). Moving to one is a deliberate owner decision and a tier-2 requalification ([ADR 0000 addendum](../../../doc/Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md#september-28-2026--reference-installation-and-requalification-tiers)).
- [x] Upgrade done — evidence (`/etc/os-release`, `uname -r`): 2026-10-05, `Ubuntu 26.04.1 LTS`, running kernel `7.0.0-38-generic`, `linux-generic` 7.0.0-38.38 (GA). [Kickoff Build Record](../../build-records/2026-10-05-stage-00-kickoff.md#reference-installation). The S00.13 account check is still open: the owner account is in `docker` and `lxd`.

## Goal

`rikurid` survives kill -9 at every kernel transaction point with rollback or deterministic recovery and no duplicate effects, CI provably blocks bad work and planted secrets, and the two early feasibility questions are answered with recorded evidence: whether Pi can be driven headlessly and contained (E2), and whether the pinned Ubuntu execution backend can be admitted without weakening the host (S00.13).

## Required deliverables

Build items S00.01–S00.13 as worded in the Stage Plan. S00.12 (E2) includes the harness-placement decision; S00.13 includes the approval-channel and host-privilege checks ([ADR 0000, September 28 review follow-ups](../../../doc/Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md#september-28-2026--option-c-review-follow-ups-and-foundation-0-credential-scope)).

## Required acceptance / demonstrations

- [ ] T00.01–T00.12 green with linked evidence — evidence:
- [ ] Owner demo: run `init`, create a sample task, see the same state after a forced restart, and see a deliberately broken change fail CI — evidence:
- [x] Receipt format agreed with the owner before the first build change (Stage Plan next actions) — evidence: owner decision 2026-10-05, [kickoff Build Record](../../build-records/2026-10-05-stage-00-kickoff.md#owner-decisions-2026-10-05-in-session); format in [Receipt](#receipt) below.

## Required engineering gates

- [ ] All stage modules wired through real call paths / vertical slice (CLI → kernel → repository → ledger).
- [ ] Failure/security/health integration evidence linked.
- [ ] Any deferral has reordered downstream checklist entries.
- [ ] Bootstrap gate and W1 gate green on the live gate board.
- [ ] E1 and E2 outcomes and the S00.13 admission probe recorded, each with a Build Record and probe output.
- [ ] Platform-neutral macOS core suite green.
- [ ] Module Contracts for domain, storage, ledger and testkit.
- [ ] ADR 0001–0004 accepted. ADR 0001 accepted 2026-10-07; 0002–0004 to come.
- [ ] Required Build Records complete.
- [ ] Required CI/full-gate green; smoke/running-system probe passed.
- [ ] Relevant contract/property/replay/security suites green.
- [ ] Documentation/help current; ADRs current; Module Contracts current.
- [ ] Threat/control/test traceability current where applicable.
- [ ] Scrap Sweep/clean-tree review complete.
- [ ] No unexplained skipped tests or unowned debt increase.

## Progress

- [x] Kickoff: owner decisions, reference installation evidence — [Build Record](../../build-records/2026-10-05-stage-00-kickoff.md)
- [ ] S00.01–S00.03 bootstrap: workspace, full gate, CI, secret scan, ratchet, hooks — [Build Record](../../build-records/2026-10-05-s00-bootstrap-ci-quality-gates.md). T00.01 red/green, T00.02, T00.03 (Ubuntu), T00.11 Pass; T00.01 "blocks" Pass (2026-10-06, drill PR #10, after protection was enabled); `smoke.sh` waits for S00.07; macOS half of T00.03 now runs on the owner's Mac as a self-hosted runner ([owner decision 2026-10-06](../../build-records/2026-10-06-macos-self-hosted-runner.md)): Pass — first green run on `main`, run 37567919632 at `e2a7d53` (24 unit + 4 property tests), after the first attempt (run 37564819464) failed and was fixed in PR #11.
- [ ] PR #7 review defects (merge-commit secrets, lint opt-in, contract headings, ratchet, allowlist, `AnyId` deserialization, bounded errors) — [Build Record](../../build-records/2026-10-05-s00-gate-review-fixes.md). Fixed with regression tests committed first; CI run 37401144525 green (13 steps). Re-verified: the second review found more in the same classes (next line); the whole of PR #7 was verified at `e42901a` on 2026-10-06, PASS.
- [ ] Second PR #7 review defects (lint policy for every workspace member, lint-capping build flags, wrapped suppressions, force-added ignored files, a redaction check that could not fail) — [Build Record](../../build-records/2026-10-05-s00-gate-review-fixes-2.md). Regression tests committed first (fixtures corrected with owner approval), then three verifier rounds that found more bypasses and false positives, each closed tests-first. Owner threat model: the gate is a tripwire for accidental or careless weakening; accepted gaps are listed in the record. Fix `5e8148a`; CI run 37414127507 green (13 steps).
- [ ] S00.04 `rikuri-domain` — [Build Record](../../build-records/2026-10-05-s00-04-rikuri-domain.md). Built and tested; not complete until a real caller uses it (S00.05–S00.07). ADR 0001 Accepted 2026-10-07.

## Blockers

- ~~**Branch protection on `main` (S00.01, Bootstrap gate, T00.01 "blocks").**~~ RESOLVED 2026-10-06: the owner upgraded `rikuripi` to GitHub Pro and created ruleset 24616501 "Protect main" (PR required, no approvals, required check `Full gate (Ubuntu)` on an up-to-date branch, no force-push or deletion, no bypass). T00.01 "blocks" Pass ([Build Record](../../build-records/2026-10-05-s00-bootstrap-ci-quality-gates.md#t0001-blocks-2026-10-06-after-the-owner-enabled-protection)). Kernel code may now merge to `main` once CI and the `verifier` are green (kickoff decision).

## Follow-ups from the pre-merge verification of PR #7 (2026-10-06)

Found by the `verifier` subagent at `e42901a` (verdict PASS, none blocks the merge). Owner of each: Henry Chen.

- ~~**`Provenance` trusts its own `origin` field**~~ RESOLVED 2026-10-07 ([record](../../build-records/2026-10-07-provenance-loaded-authority.md)): a loaded `Provenance` always carries `none`. Was: stored `user_live` JSON yields `CurrentUser` authority. Must be resolved before S00.05/S00.06 merge to `main`; storage and the ledger must not load `Provenance` from untrusted bytes until then. Tracked as a deferral in the [S00.04 record](../../build-records/2026-10-05-s00-04-rikuri-domain.md#skipped--deferred--resequenced).
- **Pre-commit hook skips its checks on a large staged change.** `pipefail` with `grep -q` lets `git diff` die of SIGPIPE, so `cargo fmt` and `cargo check` are skipped without a message (reproduced 3/3). CI still catches it. Fix with a regression test first, in the next gate-script change.
- ~~**T00.03 (Ubuntu) re-run from the current README**~~ RESOLVED 2026-10-07: Pass at `3577d3e`, 13 steps ([evidence](../../build-records/2026-10-05-s00-bootstrap-ci-quality-gates.md#t0003-re-run-from-the-current-readme-2026-10-07)). Was: install commands changed in `ea109b0`; the gate is now 13 steps.
- ~~**`claude-guard.yml`**~~ RESOLVED 2026-10-07 ([record](../../build-records/2026-10-07-guard-workflow-pins.md)): actions pinned by SHA, `permissions: contents: read`, comment corrected. Was: pinned by tag, no `permissions:` block, stale comment.
- **Flaky shared guard test** (2026-10-07): `test_guard.GuardRound2.test_round5_many_cds_and_searches_stay_fast` asserts a 4.0 s wall-clock limit and took 4.13 s on GitHub's runner, turning `main` red at `2b42f61` (run 37571810017); a re-run of the same commit passed. Owner: Henry Chen. Fix in rikurinode's shared `test_guard.py` (for example, time a fixture directory rather than `/usr/share`, or assert on work done rather than seconds) and copy it unchanged to every full-tier repository, per the owner's rule; until then a red `guard` step that names this test is re-run once and recorded, never skipped.
- Smaller, with the next `rikuri-domain` change: an overflowing ID (`8ZZZ…`) gets the message "not in canonical form"; proptest uses a random seed per run; `IdError` has public `String` fields (as `UnknownVariant`).
- Owner housekeeping (decisions 2026-10-07): stale fix-lock `14531436-…` released; ADR 0001 accepted; the `stage-00/kernel` worktree (3 unpushed commits; uncommitted: 15 new files, 2,403 lines, of which 2,264 are Rust, plus `Cargo.toml`/`Cargo.lock` changes) moved out of `/tmp` to `/home/junc/Rikuri-PI-worktrees/stage-00-kernel`, nothing committed or changed (copy verified identical, `target/` left behind, `git worktree repair`). Still open: deleting the merged and drill branches on GitHub, which the guard reserves for the owner to run by hand.

## Skipped / Deferred

| Item | Disposition | Reason | Dependency impact | Owner | New stage/ordinal | Reactivation / acceptance condition |
|---|---|---|---|---|---|---|
| S00.08 `FakeModelAdapter`, `FakeTool` | RESEQUENCED | Stage 00 has no `ModelAdapter` (S02.07) or `Tool` (S04.01) interface; a fake of a missing interface would be scaffolding without a real caller (AGENTS.md) | None for Stage 00 gates (no Stage 00 test uses a model or tool). S02.07/T02.13 and S04.01/T04.10 already require fake and live paths to share kernel code; the fakes now ship with them | Henry Chen (decided 2026-10-05) | `FakeModelAdapter` → S02.07; `FakeTool` → S04.01 | Each fake passes the same contract suite as the live implementation of its interface |

## Evidence

| Claim | Evidence |
|---|---|
| T00.01 planted failing test turns CI red; removed → green | Red: run 37397240985 (`RESULT: FAIL (test)`). Green: run 37399044114 (`RESULT: PASS (12 steps)`). [Build Record](../../build-records/2026-10-05-s00-bootstrap-ci-quality-gates.md#evidence-log). Blocking a merge: Pass, 2026-10-06 — drill PR #10 red (run 37548496990) → GitHub `mergeStateStatus` BLOCKED; test removed (run 37548667310) → CLEAN; closed unmerged |
| T00.02 planted key blocks CI without printing it | Run 37397273842 (`RESULT: FAIL (secrets)`); 0 of 3 key-body canaries in the 534-line log, 3 of 3 in the planted file. [Build Record](../../build-records/2026-10-05-s00-bootstrap-ci-quality-gates.md#evidence-log) |
| T00.03 clean Ubuntu checkout runs the full gate from README alone | Fresh clone, empty HOME, README commands verbatim → `RESULT: PASS (11 steps)` (before the self-test step was added); re-run from the current README at `3577d3e`, 2026-10-07 → `RESULT: PASS (13 steps)`. macOS half: Pass, 2026-10-07 — the core suite on `main` on the owner's Mac (run 37567919632, [runner record](../../build-records/2026-10-06-macos-self-hosted-runner.md)) |
| T00.11 Module Contract check | Passes intact; fails on a renamed required section and on a missing contract (scratch clone). [Build Record](../../build-records/2026-10-05-s00-bootstrap-ci-quality-gates.md#evidence-log) |

## Receipt

Format adopted by the owner on 2026-10-05. Filled in when the stage demo can run; until then every claim is Not yet proven.

```text
Stage 00 receipt — Bootstrap + Kernel      <date>
Verdict: <COMPLETE | NOT COMPLETE>  (<n> Pass · <n> Fail · <n> Not yet proven)

Try it (≈3 min):  scripts/demo/stage-00.sh
  You should see: init → sample task → rikurid killed → same task state
  after restart → a planted bad change failing CI.

What you can check            Result          Proof (test · date · link)
Task survives forced restart  <result>        T00.07 crash matrix + replay IDs · <date> · <link>
CLI → store → ledger is real  <result>        T00.09 trace · <date> · <link>
Broken change is blocked      <result>        T00.01 red-test run · <date> · <link>
Planted key blocked, unshown  <result>        T00.02 · <date> · <link>
Core checks pass on macOS     <result>        T00.03 macOS job on main · <date> · <link>
Pi driveable headlessly       <result>        E2 · <date> · <link>
Sandbox backend admissible    <result>        S00.13 · <date> · <link>

Limitations: <what is not covered>
Your decision: <the next owner decision>
Reproduce one claim: <one command>
```

Not yet issued.

## Exit decision

- [ ] Every required item above is satisfied or explicitly dispositioned.
- [ ] Stage owner marks COMPLETE only after acceptance evidence exists.
- [ ] Next stage may start.
