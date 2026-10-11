# Rikuri Pi

Rikuri Pi is a planned Personal Intelligence Platform. This repository contains the project planning baseline, Git hygiene, the AI-session guard described in AGENTS.md, and the start of Foundation-0 Stage 00: a Rust workspace with its first library crates (`rikuri-domain`, `rikuri-storage`, `rikuri-ledger`, `rikuri-api`), the daemon `rikurid` and its CLI `rikuri` (S00.07, in progress), and the repository's full quality gate. No Foundation-0 acceptance gate has passed yet.

For repository work, follow [AGENTS.md](AGENTS.md), which adapts the build procedure to Rikuri Pi’s SDLC and current repository maturity.

## Start here

Read the [documentation index](doc/README.md). The [Foundation-0 Re-baselined Stage Plan v2.1](doc/Rikuri_Pi_Foundation0_Rebaselined_Stage_Plan_v2.1.md) is the live implementation sequence and gate board. [ADR 0000](doc/Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md) explains how it fits the broader PRD, architecture, and earlier plans.

Stage 00 starts with repository/bootstrap work, then the Rust kernel and its crash-continuity proof. The stage plan lists the required build items, tests, experiments, and exit evidence. Its unchecked items remain unproven.

## Protecting the user's computer and data

Protecting private information, financial records, personal files/photos/videos, professional and proprietary databases, and secrets is Rikuri Pi's highest product responsibility. Access must be limited to the task, changes recoverable where promised, and protection claims backed by tests and independent review.

The adopted **Option C** direction keeps permissions under Rikuri's control, with AISecurity evaluating policy and qualified enforcement adapters applying it. OpenShell is the first Ubuntu candidate. See the [security threat model](docs/security/THREAT_MODEL.md) and [bounded OpenShell test receipt](docs/security/evidence/2026-09-28-openshell-v0.1.2/README.md). This is a planning decision with initial candidate evidence; production protection remains unproven.

## Working on macOS and Linux

The Foundation-0 plan uses Ubuntu as its full execution and security reference. Platform-neutral Rust core checks are also required on macOS. Native macOS execution containment is experimental in this plan.

On Linux, add that machine's SSH public key to the GitHub account with repository access ([GitHub SSH setup](https://docs.github.com/en/authentication/connecting-to-github-with-ssh)). Then clone with:

```sh
git clone git@github.com:rikuripi/Rikuri-PI.git
cd Rikuri-PI
git status
```

Keep credentials, private keys, local databases, and backups outside the checkout where possible. The [`.gitignore`](.gitignore) covers common accidental additions on macOS and Linux, but it cannot protect a file already tracked by Git or replace the secret scan required in Stage 00. Commit `Cargo.lock` when the Rust workspace is created; it is deliberately not ignored.

## Running the full gate

`scripts/quality/full-gate.sh` is the repository's required gate. CI runs the same script on every pull request and every push to `main`, so a local pass and a CI pass mean the same thing. On a clean Ubuntu machine:

1. Install [rustup](https://rustup.rs) and `curl`. The pinned toolchain in `rust-toolchain.toml` (with rustfmt and clippy) installs itself the first time `cargo` runs in the checkout. Python 3 (Ubuntu's default `python3`) runs the guard tests and the ratchet.
2. Install the two pinned gate tools, checking their published SHA-256 digests:

   ```sh
   dl="$(mktemp -d)"
   curl -sSfL -o "$dl/gitleaks.tgz" https://github.com/gitleaks/gitleaks/releases/download/v8.30.1/gitleaks_8.30.1_linux_x64.tar.gz
   echo "551f6fc83ea457d62a0d98237cbad105af8d557003051f41f3e7ca7b3f2470eb  $dl/gitleaks.tgz" | sha256sum -c -
   tar -xzf "$dl/gitleaks.tgz" -C "$dl" gitleaks && install -D -m 755 "$dl/gitleaks" ~/.local/bin/gitleaks

   curl -sSfL -o "$dl/cargo-deny.tgz" https://github.com/EmbarkStudios/cargo-deny/releases/download/0.20.2/cargo-deny-0.20.2-x86_64-unknown-linux-musl.tar.gz
   echo "9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f  $dl/cargo-deny.tgz" | sha256sum -c -
   tar -xzf "$dl/cargo-deny.tgz" -C "$dl" && install -D -m 755 "$dl/cargo-deny-0.20.2-x86_64-unknown-linux-musl/cargo-deny" ~/.cargo/bin/cargo-deny
   rm -r -- "$dl"
   ```

   `~/.local/bin` and `~/.cargo/bin` must be on `PATH` (rustup adds the second). Python 3.11 or newer is needed for the crate-policy check (Ubuntu 24.04 and later ship it).
3. Turn on the pre-commit hook, which scans the staged change for secrets and checks formatting and types when Rust files change:

   ```sh
   git config core.hooksPath scripts/quality/hooks
   ```

4. Run the gate from the repository root:

   ```sh
   scripts/quality/full-gate.sh
   ```

   It prints each step and stops at the first failure. A passing run ends with `RESULT: PASS (15 steps)`. The steps are: pinned tool versions, the gate's own regression tests (`scripts/quality/tests/`), AI-session guard tests, `cargo fmt --check`, build, clippy with warnings as errors (a second pass covers `rikurid`'s test-only `crash-injection` feature), unit, contract and property tests (run by `scripts/quality/no-temp-residue.sh` with a fresh, private `TMPDIR` that must still be empty afterwards, so a test that leaves scratch files behind fails the gate; the smoke test and the crash matrix below run through it too, with their daemon homes in that `TMPDIR`, and the macOS job runs its tests and crash matrix the same way), the smoke test (`scripts/quality/smoke.sh`, T00.10: the real `rikurid` and `rikuri` on a scratch home, checking that a task's title shows in `state.show` and never in the ledger, that health is healthy, and that the task survives a restart; it needs `python3`), the crash matrix (W1: T00.07 stops a `crash-injection` build of `rikurid` at each Stage 00 crash point of each consequential command, kills it with SIGKILL, restarts it and retries with the same key, expecting one effect and a clean consistency replay; T00.08 proves the check fails with idempotency off; T00.12 checks IDs and links survive; built in its own target directory, `target/crash-injection`, so `target/debug/rikurid` stays the production build), `cargo deny` (advisories, licenses, sources), gitleaks over the checked-out history including merge commits (findings shown with values redacted), the debt ratchet (skipped tests and lint suppressions in `.rs` files, TODO markers in code and config; an attribute counts however it is wrapped or spelled, text in comments and strings does not, and attributes a macro generates are not seen), the Module Contract check, the crate policy (every workspace member Cargo lists, wherever it lives, opts into the workspace lints; every local crate Cargo builds is a member; and the common ways to switch lints or tests off are refused in Cargo config, the environment and tracked member sources: lint-weakening compiler flags, an alias for a command the gate runs, a compiler replacement or wrapper, `[env]` settings for clippy or rustc, a `rustc-env` line that sets `CLIPPY_*` or `RUSTC_BOOTSTRAP`, a test runner), and the Scrap Sweep. The contract check and crate policy ask Cargo for the member list, so the workspace must load. The sweep fails on tracked files that the repository's `.gitignore` files exclude (force-added). It lists untracked files and stashes without failing; each needs a disposition before a change is closed out.

   Run the gate without a compiler wrapper such as sccache: the crate policy refuses `RUSTC_WRAPPER` and its relatives, since a wrapper can switch lints off.

   The gate is a tripwire for accidental or careless weakening, not a defence against deliberate subversion by someone with commit access; review and branch protection cover that. Known gaps are listed in the [Build Record](docs/build-records/2026-10-05-s00-gate-review-fixes-2.md#limitations-and-follow-ups).

The macOS job in CI builds and tests the same workspace, and runs the crash matrix (W1; SQLite recovery after SIGKILL is platform-neutral core), on every merge to `main`, on the owner's Mac registered as a self-hosted runner (below). Every crate stays platform-neutral; Ubuntu remains the only full execution and security reference.

## macOS runner

The `macOS core` CI job runs on the owner's Mac instead of GitHub's billed macOS machines (owner decision 2026-10-06). It is not a required check, so pull requests never wait for it; it runs after each merge to `main`, and a red run stops the line.

**Trust boundary.** A self-hosted runner executes whatever a workflow sends it. This job runs after merges to `main`, but also on manual dispatch, which runs the chosen branch's code, merged or not; and anyone with write access to the repository (the owner and the agent account `hchengit`, which AI sessions use) can dispatch it or push a branch whose workflow targets the runner. So the runner gets its own unprivileged macOS user, never the owner's.

Setup (about 15 minutes, once):

1. In System Settings → Users & Groups, add a **Standard** (not Administrator) user, for example `rikuri-ci` (the runner user below). Log in as that user for the remaining steps.
2. Install Rust for that user with rustup (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`), then open a new terminal so `~/.cargo/bin` is on `PATH`. The pinned toolchain in `rust-toolchain.toml` installs itself on the first run.
3. Open the repository's **Settings → Actions → Runners → New self-hosted runner**, choose macOS and your Mac's architecture, and run the download and `./config.sh` commands GitHub shows, in that user's home directory. Accept the default labels (`self-hosted`, `macOS`, and the architecture); the workflow selects `[self-hosted, macOS]`.
4. Install and start it as a service: `./svc.sh install && ./svc.sh start`. On macOS this is a launch agent, so it runs while the runner user is logged in; switching back to your own account (fast user switching) keeps it running. After a restart, log that user in again.

When the Mac is asleep, offline, or the runner user is logged out, the job waits as **Queued** on the commit, and GitHub fails it after 24 hours, so a missed run shows up rather than passing silently. To retire the runner, remove it under Settings → Actions → Runners and change `runs-on` back to `macos-15`.

The Rust toolchain stays installed for the runner user: the job's cache step uses `cache-bin: false`, because on a machine that keeps its files the cache action's cleanup otherwise deletes `rustup` from `~/.cargo/bin` and leaves `cargo`, `rustc` and the other proxy links pointing at nothing (it did on 2026-10-06). If a run fails with "rustup is not installed for the runner user", reinstall Rust for that user (step 2).
