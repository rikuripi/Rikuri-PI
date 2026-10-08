# rikurid — Module Contract

Template C, [Engineering Templates v1.1](../../doc/Rikuri_Pi_Engineering_Templates_v1.1.md#template-c--module-contract). Stage Plan item S00.07.

## Purpose

The trusted Rikuri daemon: the only process that changes canonical state (Architecture §6.2). It opens the canonical store under a Rikuri home directory, serves the versioned envelope of `rikuri-api` over a local socket and in-process, and runs every command through one kernel (`Kernel::handle`), so both adapters share all kernel code (T00.09).

## Owns

- The kernel: the Stage 00 commands (`system.init`, `system.health`, `task.create`, `task.start`, `task.step`, `task.complete`, `state.show`, `ledger.tail`) and their validation.
- The one-transaction path for every consequential command (Architecture §8): idempotency check → state change → event append → idempotency record → commit. A retry with the same key returns the stored response and changes nothing; the same key with a changed request is `idempotency_conflict`.
- The Stage 00 entities in the object registry: `Owner`, `Project` (untrusted for execution), `Task` (its title is owner content, with a privacy class and its own deletion unit, never copied into an event), `Run` (completion and verification recorded separately).
- Crash points 1, 2, 3 and 6 (Test Strategy §5): before the transaction, after the state write, after commit, before the response. Production uses `NoCrash`; `crash-injection` builds can hang at a point for a SIGKILL harness or fail in-process.
- Health on evidence (Architecture §27.8, §53.7): components `build`, `store` and `ledger_consistency`; a component that has never succeeded is not Healthy. The ledger-consistency check replays history against current state at open and on `system.health`; it is also the crash matrix's no-duplicate-effect check (W1).
- The local socket server: one request line per connection, one response line, `REQUEST_TIMEOUT`, `MAX_REQUEST_BYTES`; requests logged by command and outcome only, never by parameters.
- The home directory layout: `state/rikuri.db` (via `rikuri-storage`) and `run/rikurid.sock`. Home defaults to `$RIKURI_HOME`, then `~/.local/share/rikuri`.

## Must not own

- Storage mechanics, migrations or the ledger's tables (`rikuri-storage`, `rikuri-ledger`).
- The envelope's shape or error vocabulary (`rikuri-api`).
- Models, tools, network access, policy decisions beyond Stage 00 commands: later stages.

## Trust level / boundary

Trusted. Every request is untrusted input: envelope version, request ID length, command name and parameters are validated, and unknown fields are refused. The socket and its directory are owner-only: an existing socket directory that group or others can access is refused, because the socket briefly has the umask's mode between `bind` and its `chmod 0600`.

## Canonical data touched

`objects` (owner, project, tasks, runs) and, through the ledger, `events` and `idempotency`.

## Public contracts / interfaces

Binary `rikurid [--home DIR] [--socket PATH]`. Library: `Kernel::{open, handle, check_consistency, store}`, `KernelConfig`, `KernelError`, `Idempotency`; `server::{bind, serve, socket_path, REQUEST_TIMEOUT, MAX_SOCKET_PATH_BYTES}`; `crash::{CrashPoint, CrashHook, NoCrash, FailAtPoint, HangAtPoint}`; `health::{ComponentHealth, report, ConsistencyReport}`; `entities`.

## Allowed dependencies

`rikuri-api`, `rikuri-domain`, `rikuri-ledger`, `rikuri-storage`, `serde`, `serde_json`, `thiserror`. Test-only: `proptest`, `tempfile`, `ulid`.

## Forbidden dependencies

Any network, async runtime, provider or model crate in Stage 00.

## Failure and degraded behavior

A command fails with a typed `ApiError`; storage failures map to `storage_failure`. A crash at any point leaves either no change or the whole change, and the retry with the same key returns the original outcome. A build with `crash-injection` reports itself Degraded, and with idempotency off, Critical. A production build that sees the test variables says so instead of ignoring them.

## Health / last-success signal

`system.health` returns the worst component state, then each component:

```text
state / last_attempt / last_success / last_error / consecutive_failures / work_completed:
  build               — Healthy only in the production configuration
  store               — from rikuri-storage's StoreHealth
  ledger_consistency  — last replay of history against current state
```

## Architecture / ADR links

- Architecture §6.2, §8, §25.4, §27.8, §42, §53.7; Test Strategy §5 (crash points).
- ADRs: 0001 (IDs), 0002 (store), 0003 (ledger), 0004 (threat model assumptions, Proposed).

## Threat / control links

- Threat: a retried or crash-replayed command applies twice. Control: one transaction with the idempotency record. Tests: T00.06 property, T00.07 crash matrix, T00.08 regression (not yet written).
- Threat: owner content leaks into logs or the audit trail. Control: requests logged by command and outcome only; task titles never copied into events.
- Threat: health reports green on liveness alone. Control: components report last success; never-succeeded is not Healthy.

## Required tests

- [x] defect regressions — `tests/s00_07_defects.rs`: `state.show` answers in-process and through the socket (deadlock regression, with a deadline), shared socket directory refused, socket and directory owner-only
- [ ] unit/failure
- [ ] T00.09 CLI → kernel → repository → ledger as one real path; socket and in-process adapters give the same envelopes
- [ ] T00.10 smoke checks a content marker in `state.show` (`scripts/quality/smoke.sh`)
- [ ] T00.06/T00.07/T00.08/T00.12 crash matrix, property and regression

## Documentation / help entry points

This file; rustdoc; `rikurid --help`; `rikuri --help`.

## Known limitations / explicit debt

- Fixed 2026-10-08, tests-first ([S00.07 record](../../docs/build-records/2026-10-08-s00-07-rikurid.md)): `state.show` deadlocked the daemon by calling `Store::schema_version` inside `Store::read` (the store now refuses such calls with `Reentrant`, and the kernel reads the version through the transaction); an existing socket directory was not checked for owner-only.
- T00.09, T00.10 (`smoke.sh`) and the crash-matrix tests are not written yet.
- Unix only (local socket); Windows arrives with Windows support (product Stage 9).
