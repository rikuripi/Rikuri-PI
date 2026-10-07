# Build Record — macOS core CI on the owner's Mac (self-hosted runner)

| Field | Value |
|---|---|
| Change | Run the `macOS core` CI job on the owner's Mac, registered as a self-hosted GitHub Actions runner, instead of GitHub's billed `macos-15` machines |
| Date | 2026-10-06 |
| Author / agent session | Claude Code session for Henry Chen |
| Branch | `stage-00/bootstrap` (PR #7) |
| Commits | fill at close-out |
| Stage Gate | [Stage 00](../stage-gates/foundation0/00-bootstrap-kernel.md), Bootstrap gate |
| Related ADR / requirement | S00.02, T00.03 (macOS half); ADR 0000 "macOS platform-neutral core CI remains continuous"; [kickoff decision on macOS CI](2026-10-05-stage-00-kickoff.md) |
| Status | IN PROGRESS (waits for the owner to register the runner) |

**Risk note (execution on the owner's machine):** a self-hosted runner executes whatever a workflow sends it. This changes `.github/workflows/ci.yml`, a protected path.

## Owner decision (2026-10-06, in session)

The owner asked whether macOS CI is needed at all, having run the checks by hand on a MacBook Air after merging, to avoid GitHub's macOS cost. The governing documents require the macOS core checks *in CI*, continuously (Stage Plan S00.02 and T00.03 "passes on macOS CI"; ADR 0000; Engineering Standard line 28); they do not require GitHub-hosted machines. Three options were put to the owner:

| Option | Cost | Consequence |
|---|---|---|
| **Self-hosted runner on the owner's Mac (chosen)** | No billed minutes; ~15 min setup; the Mac must be awake with the runner user logged in | Still CI; no ADR change. Missed runs show as Queued, then fail after 24 h |
| GitHub-hosted `macos-15` (the kickoff setup) | Billed macOS minutes per merge (kickoff estimate 40–60 billed minutes per run) | No setup |
| Manual runs after merge | None in money; a step for the owner per merge | Not CI: needs a dated ADR 0000 clarification and Stage Plan wording, and a skipped run goes unnoticed without an added tripwire |

The kickoff decision (macOS on merges to `main` only, not on PRs) stands; only the machine changes.

## 1. Plan

- [x] One-sentence statement: the existing `macOS core` job selects `runs-on: [self-hosted, macOS]`, and README documents the runner's setup and trust boundary.
- [x] Blast radius: one job's `runs-on`; the required `Full gate (Ubuntu)` check is unchanged; the macOS job stays non-required and post-merge.
- [x] Existing solution review: GitHub's own self-hosted runner (the `actions/runner` application GitHub's settings page serves), no custom agent. Runner version is GitHub's current one at registration and updates itself.
- [x] **I will know this works when** a CI run on this branch, dispatched after the owner registers the runner, shows the `macOS core` job picked up by the self-hosted runner (job log "Runner name") and green; the `Full gate (Ubuntu)` job is unaffected; and after PR #7 merges, the push run on `main` is green on the same runner (T00.03 macOS half).

## Risk and failure behavior

- **Trust boundary.** The job's condition runs it only for pushes to `main` and manual dispatch, so it runs merged code. That is not a hard boundary: anyone with write access (the owner, and the agent account `hchengit` that AI sessions use) can push a branch whose workflow targets the runner, or dispatch this workflow from a branch. Mitigation: the runner runs as its own **Standard** macOS user (`rikuri-ci`), never the owner's account, so it cannot read the owner's files or administer the Mac. Residual risk: such a workflow runs with that user's rights and the network. The guard asks before workflow edits only in its `stable` profile; in `development` (current) such edits are allowed and logged in `.claude/state/guard-relaxed.log`.
- **Runner offline** (Mac asleep, offline, runner user logged out): the job waits as Queued on the commit and GitHub fails it after 24 hours. It never reports success without running (law 4).
- **Rollback:** remove the runner under Settings → Actions → Runners and set `runs-on: macos-15`.

## Evidence log

```text
# Prepared before the owner registers the runner (2026-10-06)
.github/workflows/ci.yml   macos-core: runs-on macos-15 → [self-hosted, macOS]; header comment records the decision
README.md                  "macOS runner" section: setup, trust boundary, offline behavior, rollback
```
