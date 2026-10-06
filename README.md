# Rikuri Pi

Rikuri Pi is a planned Personal Intelligence Platform. This repository contains the project planning baseline, Git hygiene, the AI-session guard described in AGENTS.md, and the start of Foundation-0 Stage 00: a Rust workspace with its first crate (`rikuri-domain`) and the repository's full quality gate. No Foundation-0 acceptance gate has passed yet.

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

   It prints each step and stops at the first failure. A passing run ends with `RESULT: PASS (13 steps)`. The steps are: pinned tool versions, the gate's own regression tests (`scripts/quality/tests/`), AI-session guard tests, `cargo fmt --check`, build, clippy with warnings as errors, unit, contract and property tests, `cargo deny` (advisories, licenses, sources), gitleaks over the checked-out history including merge commits (findings shown with values redacted), the debt ratchet, the Module Contract check, the crate policy (every crate opts into the workspace lints), and the Scrap Sweep. The sweep lists untracked files and stashes without failing; each needs a disposition before a change is closed out.

The macOS job in CI builds and tests the same workspace on every merge to `main`. Every crate stays platform-neutral; Ubuntu remains the only full execution and security reference.
