# Rikuri Pi

Rikuri Pi is a planned Personal Intelligence Platform. This repository currently contains the project planning baseline and Git hygiene only. No Foundation-0 implementation or acceptance gate has passed yet.

## Start here

Read the [documentation index](doc/README.md). The [Foundation-0 Re-baselined Stage Plan v2.0](<doc/Rikuri Pi — Foundation-0 Re-baselined Stage Plan v2.0.md>) is the live implementation sequence and gate board. [ADR 0000](doc/Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md) explains how it fits the broader PRD, architecture, and earlier plans.

Stage 00 starts with repository/bootstrap work, then the Rust kernel and its crash-continuity proof. The stage plan lists the required build items, tests, experiments, and exit evidence. Its unchecked items remain unproven.

## Working on macOS and Linux

The Foundation-0 plan uses Ubuntu as its full execution and security reference. Platform-neutral Rust core checks are also required on macOS. Native macOS execution containment is experimental in this plan.

On Linux, add that machine's SSH public key to the GitHub account with repository access ([GitHub SSH setup](https://docs.github.com/en/authentication/connecting-to-github-with-ssh)). Then clone with:

```sh
git clone git@github.com:rikuripi/Rikuri-PI.git
cd Rikuri-PI
git status
```

Keep credentials, private keys, local databases, and backups outside the checkout where possible. The [`.gitignore`](.gitignore) covers common accidental additions on macOS and Linux, but it cannot protect a file already tracked by Git or replace the secret scan required in Stage 00. Commit `Cargo.lock` when the Rust workspace is created; it is deliberately not ignored.
