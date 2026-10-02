# Build Record — Reference installation and two-tier sandbox requalification

| Field | Value |
|---|---|
| Change | Record the owner's Foundation-0 reference installation (this machine, the newest stable Ubuntu LTS with its standard kernel: 26.04 LTS, kernel 7.0) and define two requalification tiers so routine kernel updates do not force a full sandbox qualification |
| Date | 2026-09-28 |
| Author / agent session | Claude Code (Opus 5.5) session |
| Branch | main |
| Commits | `41c6e5d` |
| Stage Gate | Stage 00 Stage Gate Record gains the reference-installation decision; no gate closes |
| Related ADR / requirement | ADR 0000 new dated addendum; S00.13, S03.11, E5; T03.05, T03.16; Engineering Standard "Security assurance and upstream maintenance"; Architecture §27.12; Test Strategy §8.2 |
| Status | COMPLETE — documentation only; independent verifier PASS on the final text (third pass); runtime Not yet proven |

## Intent (owner-approved)

The owner is starting Foundation-0. The development machine runs Ubuntu 24.04.5 LTS, kernel 6.8.0-138-generic (checked with `/etc/os-release` and `uname -r`). The owner first decided to upgrade to Ubuntu 26.04 LTS and later move to a 7.3 kernel, with only one full sandbox qualification, and confirmed: "Yes, apply edit." Once the recurring cost of rolling HWE kernels was shown (see Debug), the owner revised the decision: "Yes, we'll work with 26.04 and 7.0. No need to update so quickly. We should run on the newest stable version of LTS." The owner chose 7.0 after being told the kernel is shared with the rikurinode node on this machine.

## Spec

- **Reference installation.** This machine is Foundation-0's Ubuntu reference. It runs the newest stable Ubuntu LTS with its standard (GA) kernel, never HWE, OEM or mainline: Ubuntu 26.04 LTS with kernel 7.0, upgraded before S00.13. S00.13 is admission evidence, not qualification. The full qualification (E5, T03.x, Test Strategy §8.2) runs once at Stage 03 on 7.0. The next LTS (expected 28.04) is adopted after its first point release, as a planned tier-2 requalification between stages.
- **Shared machine.** The same machine runs the owner's Bitcoin node containers on a rootful Docker engine. The existing S00.13 rule (backend controller on a dedicated account or rootless engine) keeps Rikuri-PI's sandbox away from that engine. The OS upgrade is a node maintenance event, run by the owner outside this repository.
- **Two requalification tiers, defined once in the Engineering Standard** (Security assurance and upstream maintenance); S03.11, Architecture §27.12, Test Strategy §8.2, the ADR and the Stage 00 record point to it instead of restating it.
  - *Tier 1: automated regression on the running system*, only for: a kernel patch update within the same upstream series and flavour; a container image digest refresh with no version change; a backend patch release whose changelog and advisories show no change to defaults or enforcement semantics. The S03.11 ADR defines the suite's content.
  - *Tier 2: full qualification* (E5, T03.x, Test Strategy §8.2) for everything else, including kernel series or flavour changes, OS release upgrades, LSM or container engine/runtime/driver changes, backend minor/major releases, configuration, provider-binding or policy-semantics changes, and any tier-1 failure.
  - **Fail-closed default:** a change not listed as tier 1, or matching any tier-2 trigger, is tier 2. The S03.11 ADR may move a component class into tier 1 only with a written reason and a covering tier-1 check.
  - Affected profiles are unavailable from the moment a running component changes until the applicable tier passes. Evidence binds to the running kernel (`uname -r`), not the installed package.
- **Why not HWE.** Ubuntu's HWE and OEM kernels roll to newer series over an LTS's life, each roll a tier-2 requalification, and an interim HWE kernel loses security updates once its successor ships. The standard kernel keeps one series for the LTS's life. Moving to HWE (for example 7.3 via 26.04.2) stays possible as a deliberate owner decision.

## 1. Plan

- [x] One-sentence statement of change: record the reference installation and make requalification proportional to the change without weakening fail-closed behavior.
- [x] Blast radius: documentation only. Protected paths: ADR 0000 (additive dated addendum), Stage Plan (S03.11 wording), Test Strategy (§8.2 row), Engineering Standard, `docs/stage-gates/`. No code, guard, CI, host, OS or node change.
- [x] Current behavior characterized: the Engineering Standard, Architecture §27.12 and Test Strategy §8.2 require requalification on any kernel change without distinguishing a patch update from a series change, which would make every weekly kernel security update a full qualification.
- [x] I will know this works when: the Stage 00 record states the reference installation, sequence and owner fallback choice; the Engineering Standard defines the two tiers once with a fail-closed tier-2 default, and S03.11, Architecture §27.12, Test Strategy §8.2, the ADR and the Stage 00 record point to it without contradicting it; profiles are unavailable until the applicable tier passes; no S/T/E identifier changes; ADR history is preserved; links resolve; `git diff --check` and guard tests pass; an independent verifier reviews it.
- [x] Architecture-impact check: invariant/ADR — yes (dated addendum refining the requalification rule); privacy/security — yes (evidence freshness); all others N/A.
- [x] Threat/control/test: "Backend change or misleading status overstates protection" row in the threat model is served by the tier rule; no new threat row needed.
- [x] Security question: tier 1 must not become a way to keep claiming protection on unverified components. Mitigation: affected profiles are unavailable until the applicable tier passes, unlisted changes default to tier 2, and any tier-1 failure escalates to tier 2.

Planned files: this record; `docs/stage-gates/foundation0/00-bootstrap-kernel.md`; `doc/Rikuri_Pi_Foundation0_Rebaselined_Stage_Plan_v2.1.md` (S03.11); `doc/Rikuri_Pi_Engineering_Standard_v1.2.md`; `doc/Rikuri_Pi_Architecture_Spec_v1.7.md` (§27.12 adapter contract); `doc/Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md` (§8.2 upgrades row); `doc/Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md` (dated addendum).

## 2. Build

- [x] Stage 00 Stage Gate Record: new "Reference installation" section (machine, current OS/kernel, newest-stable-LTS principle, 26.04 with the 7.0 standard kernel before S00.13, one full qualification at Stage 03, next-LTS move, HWE not planned, unchecked upgrade-evidence box).
- [x] Stage Plan S03.11: ADR scope includes the two tiers, unavailable-until-pass and running-kernel binding. Wording only; no identifier or checkbox changed.
- [x] Engineering Standard, Architecture §27.12 adapter contract, Test Strategy §8.2 "User trust and upgrades" row: the same two tiers.
- [x] ADR 0000: new dated addendum (reference installation; tiers as a refinement). Additive only.

## 3. Debug

Not a runtime defect. The first independent verifier pass returned **FAIL**, with three findings rated Important. All were fixed in this change:

- **I1: no fail-closed default.** Changes such as unlisted dependencies, configuration, flavour changes, or anything matching both tiers were left to judgment. Fixed: anything not listed as tier 1 is tier 2. Tier 1 now requires a digest refresh with no version change and a backend patch release whose changelog has been reviewed. A kernel flavour change is tier 2.
- **I2: the documents stated the tiers inconsistently.** Architecture §27.12 omitted backend minor/major releases, tier-1 failure and the running-kernel binding, and "flavour" appeared unevenly. Fixed: the rule is defined once, in the Engineering Standard, and the other documents point to it.
- **I3: "one full qualification on the kept series" was contradicted by Ubuntu's kernel behavior.** The HWE and OEM kernels move to new series. Fixed: the cost was stated. The owner then chose the standard 26.04 kernel (7.0) over HWE 7.3, and the reference-installation text was rewritten to that decision.

The Plan's acceptance line was amended after that FAIL, from "state the same two tiers" to "one definition that the others point to", because restating the rule in five documents caused I2. Nits: the build record status now waits on the verifier re-run. The Stage 00 record labels the running kernel and notes the pending 6.8.0-142. S03.11 now asks its ADR to define the tier-1 suite, with telemetry and control-plane egress checks listed in the Engineering Standard. The ambiguous "release" is gone. The Stage 00 wording no longer overstates.

## 4. Verify

- [x] Independent verifier re-run: **VERDICT: PASS**. I1–I3 closed; its five remaining nits fixed (stale "kept series" text and counts in this record; tier-1 floor and receipt content added to the Engineering Standard; Security line wording).
- [x] Third independent verifier pass, on the text after the owner's switch to the 7.0 standard kernel: **VERDICT: PASS**. Checks 1–4 passed (one consistent decision, no claim beyond evidence, tier rule unchanged, IDs/ADR/links/whitespace/guard tests). Nits fixed: this status line; the next-LTS row said "post-Foundation" while other documents said "between stages"; the Stage 00 principle was absolute although an owner-decided HWE move is allowed; the upgrade path to 26.04 was not evidenced (now checked and recorded, see evidence log).
- **Limitation (verifier silence check):** this repository's guard did not see or log these edits. The authoring Claude Code session was started in another repository, so that repository's hook ran instead of `.claude/hooks/guard.py` here. Future Rikuri-PI sessions should start in the Rikuri-PI directory.
- [x] Author checks: see evidence log. Runtime items N/A (documentation only); tier-1 suite does not exist until Stage 03 and is Not yet proven.

## 5. Close Out

- Full runtime gate: Not yet proven (no `scripts/quality/*` or Rust workspace yet).
- [x] Scrap Sweep: the link checker lives in the session scratchpad, outside the repository; no task files left.
- [x] Stage Gate Record updated.
- [x] Committed and pushed at the owner's request.

## Skipped / Deferred / Resequenced

| Item | Disposition | Reason | Dependency impact | Owner | New stage/ordinal | Reactivation / acceptance condition |
|---|---|---|---|---|---|---|
| Ubuntu 26.04 upgrade of this machine | OWNER ACTION | Host and live-node maintenance, outside this repository; Ubuntu had not opened the 24.04 → 26.04 upgrade path on 2026-09-28 | S00.13 runs after it; S00.01–S00.12 unaffected | Owner | Before S00.13 | `/etc/os-release` shows 26.04 on the reference machine, recorded in the Stage 00 record |
| Adopt the next Ubuntu LTS (expected 28.04) | OWNER ACTION | Newest stable LTS principle; after its first point release | Tier-2 requalification between stages | Owner | Between stages, when available (expected after Foundation-0) | `/etc/os-release` on the reference machine and a passing full qualification |

## Evidence log

```text
$ . /etc/os-release; echo $PRETTY_NAME; uname -r
Ubuntu 24.04.5 LTS
6.8.0-138-generic
$ dpkg -l 'linux-image-6.8.0-14*' -> linux-image-6.8.0-142-generic 6.8.0-142.142 installed (reboot pending)
$ apt-cache policy linux-generic-hwe-24.04 linux-oem-24.04 -> Candidate 7.0.0-34.34~24.04.1 / 6.17.0-1032.32
$ do-release-upgrade -c -> "There is no development version of an LTS available." (exit 1); Prompt=lts
$ curl -s https://changelogs.ubuntu.com/meta-release-lts -> Dist: resolute / Version: 26.04.1 LTS / Supported: 0
$ python3 <scratchpad>/checklinks.py <7 changed/new files>
60 local links checked, 0 bad   (exit 0)
Stage Plan S/T/E identifier set HEAD vs working tree: unchanged; S/T boxes checked: 0
git diff ADR 0000 | removed lines: 0
$ git diff --check -> exit 0
$ python3 -m unittest discover -s .claude/hooks -p 'test_*.py' -> exit 0
```
