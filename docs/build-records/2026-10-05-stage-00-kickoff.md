# Build Record — Stage 00 kickoff: owner decisions and reference installation

**Date:** October 5, 2026 · **Risk:** low, records only (brief record; touches protected Stage Gate Record and Stage Plan status lines) · **Branch:** `stage-00/kickoff` · **Status:** IN PROGRESS (awaiting independent verification)

## Scope

The owner asked to start the build. The 90-Day Build Plan v1.6 is historical; the live sequence is [Stage Plan v2.1](../../doc/Rikuri_Pi_Foundation0_Rebaselined_Stage_Plan_v2.1.md), so the build starts at Foundation-0 Stage 00 (Bootstrap + Kernel). Before the first build change the Stage Plan requires the six owner-facing demos and the receipt format to be agreed. This record captures that agreement, the owner's other kickoff decisions, and the reference-installation evidence the Stage 00 record was waiting for. No S/T item, gate, date or scope changes.

## Plan

- **I will know this works when** the owner's decisions and the reference-installation evidence are recorded where the next session will find them (this record, the Stage 00 record, the Stage Plan's next-action box), every claim here is reproducible on this machine or attributed to the owner, every local link resolves, and the Stage Plan changes by exactly that one checkbox.
- Wiring point: N/A, records only. No executable behavior changes.

## Review performed

All documents in `doc/` and the security, stage-gate and build-record material in `docs/` were read for Stage 00 obligations before planning. Findings that shape Stage 00 work:

- Stage 00 items S00.01–S00.13 and tests T00.01–T00.12 are as worded in the Stage Plan; no later document modifies them (Master Build Sequence September 30 and October 2 dispositions leave Foundation 00–05 unchanged).
- Enumerations the kernel must use come from Architecture §19.1 and PRD §10.1: `privacy_class: normal | private | local_only | secret`; `instruction_authority: current_user | trusted_policy | none`; `origin: user_live | user_authored | imported | web | tool_output | model_output`; execution intent `auto | private | local_only` is a separate type from data privacy. Project trust states come from S03.04 (`untrusted_read`, `trusted_scoped_execute`, `revoked/review_required`).
- Health must distinguish fresh verified health from unknown/unverified (PRD SEC-030), not only Healthy/Degraded/Critical.
- IDs must be globally unique and never derived from paths or per-table autoincrement (Architecture §4, invariant 2, requires identity independent of filenames or paths. Global uniqueness and "no per-table counter" are this plan's inference from that invariant, from Writer IDE §3, which later needs identities down to sentences, and from Stage 7 replication, which needs IDs unique across nodes).
- Events reference content by ID and never embed sensitive content (Architecture §8). Replay must distinguish a governed-erasure tombstone from a dangling reference (T00.12 versus later forget semantics).

## Owner decisions (2026-10-05, in session)

| Decision | Owner choice | Consequence |
| --- | --- | --- |
| Six owner-facing stage demos | Agreed as written in the Stage Plan's "Owner-facing result at each stage" table | No wording change; each stage's demo is the row for that stage |
| Evidence-receipt format | Adopted: verdict line; a try-it demo script with what the owner should see; a plain-language claim table with **Pass / Fail / Not yet proven**, exact test or probe, date and link; limitations; the decision needed from the owner; a one-line reproduction command | Lives as a "Receipt" section of each Stage Gate Record, so there is no second gate board. One `scripts/demo/stage-NN.sh` per stage, created with the work it demonstrates |
| Protected `main` | GitHub Pro on the `rikuripi` account | GitHub refuses branch protection and rulesets on this private repository under GitHub Free (`GET /repos/rikuripi/Rikuri-PI/rulesets` → 403 "Upgrade to GitHub Pro or make this repository public"). The agent account `hchengit` has write, not admin, so the owner upgrades and enables protection. Until then the Bootstrap gate's "CI blocks bad work" stays Not yet proven |
| Agent GitHub authority | Push task branches, open PRs, run drills on throwaway branches, and merge a PR once CI and the `verifier` are green | Kernel code does not merge before branch protection exists (Stage Plan stop-the-line: "CI / red-test does not actually block → STOP; fix gate before code") |
| macOS CI job | Run the platform-neutral core build/unit/contract job on every merge to `main`, not on PRs | Keeps ADR 0000's "macOS platform-neutral core CI remains continuous" (also Engineering Standard line 28, Architecture §26.2/§46, PRD MACH-005). The owner chose this knowing the stated consequence: a macOS break is found right after merge and fixed as a stop-the-line item, so `main` can be red on macOS until that fix. Reading of Engineering Standard Phase 0 "every merge has a green CI run" and Stage Plan "CI provably blocks bad work": satisfied by the required Ubuntu full gate on every PR; macOS is a post-merge continuous check, not a pre-merge required gate (AGENTS.md also lists macOS core checks separately from the Ubuntu gates). Cost: the agent estimated about 40–60 billed minutes per run when asking (private-repo macOS minutes bill at a multiple of Linux minutes); this is an unmeasured estimate, to be replaced by the first macOS run's actual duration in the bootstrap Build Record |

Correction during the session: the owner first answered "do not run the macOS job for now" to a question that presented it as a cost choice only. Independent verification found that this conflicts with the accepted continuous-macOS commitment above and would have needed a dated ADR 0000 clarification. Asked again with that context, the owner chose main-only runs, which needs no ADR change.

## Reference installation

The Stage 00 record listed the 26.04 upgrade as pending. `/var/log/dist-upgrade/main.log` shows Ubuntu's release upgrader (`release-upgrader version '26.04.25' started`, `Upgrading from noble to resolute`) on 2026-10-03, which is the supported upgrade path the record requires. Evidence on 2026-10-05:

```text
/etc/os-release   PRETTY_NAME="Ubuntu 26.04.1 LTS"  VERSION_ID="26.04"
uname -r          7.0.0-38-generic
linux-generic     7.0.0-38.38 (the release's GA kernel metapackage; no HWE/OEM kernel)
```

An older `linux-image-6.8.0-146-generic` package remains installed but is not running; evidence binds to the running kernel.

**Observation for S00.13 (not a Stage 00 blocker yet):** the owner's account `junc` is a member of `docker` (gid 986) and `lxd` (gid 101), both root-equivalent, and of `sudo` (27, administrative with authentication), `adm` (4) and `ollama` (985). Its other groups (`cdrom`, `dip`, `plugdev`) are not privileged. S00.13 must record that the accounts running `rikurid`, Pi and model/tool servers are outside such groups (ADR 0000 September 28 follow-ups, decision 2). Options at S00.13: a dedicated unprivileged account for those processes, or removing `junc` from the groups (which affects how the owner manages the Bitcoin node containers). The owner chooses when S00.13 is scheduled.

## Line check before starting

The latest `AI-session guard` run on `main` (run 37362916220, commit `95eaaac`) was red. Its first attempt concluded `failure` because the py3.9 job was `cancelled` without running a step ("The job was not acquired by Runner of type hosted even after multiple attempts"); py3.12 passed. Locally `python3 -m unittest discover -s .claude/hooks -p 'test_*.py'` ran 91 tests OK (Python 3.14.4). Re-running only the failed job made both jobs pass. Class: CI runner acquisition, not a code defect. Upcoming risk to the same workflow: GitHub migrates `ubuntu-latest` to Ubuntu 26 from October 19, 2026, and the 3.9 matrix entry depends on `setup-python` still offering 3.9 there; watch the first run after that date.

Full-history secret scan before bootstrap, default ruleset, gitleaks 8.30.1: `gitleaks git . --config <file containing only "[extend] useDefault = true"> --redact --log-opts="--all -m"` scanned all 42 commits on every ref, including the 7 merge commits' own diffs. No leaks found.

## Changes

- This Build Record.
- [Stage 00 Stage Gate Record](../stage-gates/foundation0/00-bootstrap-kernel.md): status IN PROGRESS; reference-installation upgrade checked with evidence; receipt agreement checked; the Receipt section skeleton in the adopted format; progress and the blocker (branch protection).
- [Stage Plan v2.1](../../doc/Rikuri_Pi_Foundation0_Rebaselined_Stage_Plan_v2.1.md) next actions: the demos/receipt agreement box is checked with a link here. Gate board unchanged.

## Verification

- Local Markdown links in changed files resolve; `git diff --check` clean (evidence log below).
- Runtime tests: N/A, no executable behavior changes in this record.

## Deferred

None. The macOS job arrives with the CI workflow in the bootstrap change.

## Independent verification

First `verifier` run on commit c6f4e81: **FAIL**, scope consistency (the macOS deferral conflicted with accepted commitments without reconciliation). All reproducible facts, links, guard tests and the Stage Plan invariance check passed. It also reported: no acceptance line; status set COMPLETE before verification; the gitleaks commit count overstated what the recorded command scanned; the py3.9 job was `cancelled`, not failed; "§4.2" does not exist; additional groups; no upgrade date. All are corrected above. The owner re-decided macOS (main-only).

Second `verifier` run on 6d531e4: **NOT YET PROVEN**. Main-only macOS satisfies the "continuous" commitments (ADR 0000 line 27, Engineering Standard line 28, Architecture §26.2/§46, PRD MACH-005); no source requires macOS before merge. Open items: the record did not address the Engineering Standard's "every merge has a green CI run" reading, and the cost figure was neither measured nor attributed. Both are addressed in the decision table above; the commit message of 6d531e4 says "all groups", which overstated the list (non-privileged groups now named). A third verification follows.

## Evidence log

```text
$ python3 check_links.py <3 changed files>   # session scratch link/anchor checker, not committed
links bad: 0
$ git diff --check
(clean)
$ python3 -m unittest discover -s .claude/hooks -p 'test_*.py'
Ran 91 tests in 8.545s
OK
$ gh run view 37362916220   # after re-running the failed job
conclusion: success; Guard policy tests (py3.9): success; (py3.12): success
$ git rev-list --all --count; git rev-list --all --merges --count
42
7
$ gitleaks git . --config <default ruleset only> --no-banner --redact --log-opts="--all -m"
42 commits scanned
no leaks found
$ grep -E "release-upgrader version|Upgrading from" /var/log/dist-upgrade/main.log
2026-10-03 18:45:54,995 INFO release-upgrader version '26.04.25' started
2026-10-03 18:45:55,067 DEBUG Upgrading from noble to resolute
```
