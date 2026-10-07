# Build Record — macOS core CI on the owner's Mac (self-hosted runner)

| Field | Value |
|---|---|
| Change | Run the `macOS core` CI job on the owner's Mac, registered as a self-hosted GitHub Actions runner, instead of GitHub's billed `macos-15` machines |
| Date | 2026-10-06 |
| Author / agent session | Claude Code session for Henry Chen |
| Branch | `stage-00/bootstrap` (PR #7) |
| Commits | `1bb159f` (workflow, README, this record; merged in PR #7 as `9ea01c1`); `00e4fdd` (cache-bin fix), `ec7ec84` (two-run evidence; both merged in PR #11 as `e2a7d53`); this close-out (its own PR) |
| Stage Gate | [Stage 00](../stage-gates/foundation0/00-bootstrap-kernel.md), Bootstrap gate |
| Related ADR / requirement | S00.02, T00.03 (macOS half); ADR 0000 "macOS platform-neutral core CI remains continuous"; [kickoff decision on macOS CI](2026-10-05-stage-00-kickoff.md) |
| Status | COMPLETE (2026-10-07: first green macOS run on `main`, run 37567919632; see Close-out) |

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

- **Trust boundary.** The job's condition runs it for pushes to `main` and for manual dispatch; a dispatch runs the chosen branch's code, merged or not (the pre-merge proof run below did exactly that). *Corrected 2026-10-06: this line first said "so it runs merged code".* Anyone with write access (the owner, and the agent account `hchengit` that AI sessions use) can dispatch it or push a branch whose workflow targets the runner. Mitigation: the runner runs as its own macOS user (`rikurici`), never the owner's account, so it cannot read the owner's files or administer the Mac. Residual risk: such a workflow runs with that user's rights and the network. The guard asks before workflow edits only in its `stable` profile; in `development` (current) such edits are allowed and logged in `.claude/state/guard-relaxed.log`.
- **Runner offline** (Mac asleep, offline, runner user logged out): the job waits as Queued on the commit and GitHub fails it after 24 hours. It never reports success without running (law 4).
- **Rollback:** remove the runner under Settings → Actions → Runners and set `runs-on: macos-15`.

## Evidence log

```text
# Prepared before the owner registers the runner (2026-10-06)
.github/workflows/ci.yml   macos-core: runs-on macos-15 → [self-hosted, macOS]; header comment records the decision
README.md                  "macOS runner" section: setup, trust boundary, offline behavior, rollback

# Owner registered the runner (2026-10-06): Standard macOS user `rikurici`;
# `which cargo` → /Users/rikurici/.cargo/bin/cargo; runner shown Idle in Settings → Actions → Runners.
# Committed as 1bb159f; pushed to PR #7.

# Pre-merge proof: CI dispatched on stage-00/bootstrap at 1378ed7, run 37564364375, 2026-10-07T02:56Z
macOS core (build · unit · contract): success   job 112608603122
  runner_name Henrys-MacBook-Air · labels [self-hosted, macOS] · runner group Default
  rustc 1.98.1 (pinned) · cargo build --workspace --all-targets --locked: Finished
  cargo test --workspace --locked: 24 passed (unit) · 4 passed (properties) · 0 failed      48 s on the Mac
Full gate (Ubuntu): success (same run); PR run 37564364533 at 1378ed7: success
```

The macOS half of T00.03 is shown on the owner's Mac before merge. The first run on `main` follows the merge of PR #7.

## 3. Debug — the first run on `main` failed (2026-10-06)

PR #7 merged at 03:01:45 UTC as `9ea01c1`. Its push run 37564819464: `Full gate (Ubuntu)` success; `macOS core` **failure** on `Henrys-MacBook-Air`, at the toolchain step: `rustup: command not found` (exit 127); build and tests skipped. A re-run of the failed job (attempt 2, 03:15 UTC) failed the same way. `main` red on macOS: stop the line.

| Step | Evidence |
|---|---|
| Not the runner's `PATH` (first hypothesis, disproved) | On the Mac (owner, as `rikurici`): `.path` contains `/Users/rikurici/.cargo/bin`; the service `Runner.Listener` (started 19:47:38 −0700, before both runs) has that directory in its `PATH`, before and after a service restart |
| The same process ran the green job at 02:56 and the failing one at 03:01 | Job logs: same runner; the 02:56 job found `rustup` and installed 1.98.1 |
| What ran between them | The green job's post step, `Swatinem/rust-cache` with `cache-bin: true` (its default), cache paths include `/Users/rikurici/.cargo/bin`; at 02:56:51 it logged `... Cleaning cargo/bin ...`, then saved the cache |
| Confirmation on the Mac | Owner reinstalled Rust for `rikurici` (rustup installer, 20:29 −0700), then `ls -la ~/.cargo/bin`: the 13 proxy symlinks (`cargo -> rustup`, `rustc -> rustup`, …) date from the original install at 18:00, while `rustup`, the only regular file, dates from 20:29. The cleanup deleted the regular file and left the symlinks dangling, so `rustup` and every proxy were "not found". `which rustup cargo` now resolves both |

**Cause:** with `cache-bin: true`, the cache action manages `~/.cargo/bin` as cache contents. In the pinned version (`6323deb`), it lists every regular file in that directory when the job starts (`getCargoBins`, `dirent.isFile()`), and its cleanup deletes exactly those files (`cleanBin`, run only when `cache-bin` is true; verifier, from the action's source). On this machine the only regular file is `rustup`; the `cargo`/`rustc` proxies are symlinks to it, so they stayed, dangling. *Corrected 2026-10-07: this line first said the cleanup deletes binaries "that `cargo install` did not record", and README/`ci.yml` said it deleted "`rustup` and `cargo`".* On GitHub's machines, discarded after every job, that is harmless; on a runner that keeps its files it removed the toolchain after the first green run. **Class:** a setting that is safe on ephemeral runners and destructive on a persistent one; sibling check: the Ubuntu job runs on GitHub's machines (unaffected), and no other workflow uses the self-hosted runner.

**Fix:** the macOS job's cache step sets `cache-bin: false`, so the toolchain stays installed for the runner user; the toolchain step now fails with a clear message (`rustup is not installed for the runner user … README > macOS runner`) instead of a bare `command not found`; README says why. The owner reinstalls Rust for `rikurici`. **Tripwire:** a recurrence fails the job loudly at its first step and names the cause; proof needs two consecutive green runs on the Mac, since the failure appeared only on the second.

```text
# Fix commit 00e4fdd on fix/macos-runner-cargo-bin (PR #11); two dispatched runs back to back
run 37567420543  macOS job 112618218980  Henrys-MacBook-Air  03:35:07→03:35:35Z  success
run 37567512663  macOS job 112618504735  Henrys-MacBook-Air  03:36:15→03:36:34Z  success
each: rust-cache with cache-bin: false (no "Cleaning cargo/bin"); rustc 1.98.1; 24 unit + 4 property tests passed;
      the new "rustup is not installed" check did not fire; Full gate (Ubuntu) success in the same runs
```

The `verifier` passed PR #11 at `ec7ec84` (diagnosis confirmed from the job logs and the action's source; no merge blockers); it was merged with a merge commit.

## Close-out (2026-10-07)

```text
PR #11 merged 2026-10-07T03:41:26Z as e2a7d53 (merge commit)
main push run 37567919632 at e2a7d53:
  macOS core (build · unit · contract): success  Henrys-MacBook-Air  03:41:31→03:42:00Z
    rustc 1.98.1 · 24 unit + 4 property tests passed · no "Cleaning cargo/bin"
  Full gate (Ubuntu): success  03:41:31→03:42:27Z
```

- [x] Plan criterion met: the macOS job ran on the self-hosted runner and passed on the PR branch before merge, and on `main` after merge (after the Debug fix). T00.03's macOS half: Pass (run 37567919632). The stop-the-line from run 37564819464 is cleared.
- [x] Stage Gate Record updated (T00.03 progress line and evidence row).
- [x] Docs: README "macOS runner" and the `ci.yml` comments current; corrections dated in place.
- [x] Scrap: nothing task-created left in the repository; the change is the workflow and docs only.

**Not yet proven** (each rests on the owner's report or GitHub's documentation, not on an observation here):

- that `rikurici` is a Standard (non-admin) user; owner to check with `id rikurici` (no `admin` group) or System Settings;
- the 24-hour limit after which GitHub fails a queued self-hosted job (GitHub's documented usage limit; not observed);
- that the launch-agent service keeps running across fast user switching, and what happens after a restart until the runner user logs in;
- the runner's self-update.
