# rikurid — Module Contract

Template C, [Engineering Templates v1.1](../../doc/Rikuri_Pi_Engineering_Templates_v1.1.md#template-c--module-contract). Stage Plan item S00.07.

## Purpose

The trusted Rikuri daemon: the only process that changes canonical state (Architecture §6.2). It opens the canonical store under a Rikuri home directory, serves the versioned envelope of `rikuri-api` over a local socket and in-process, and runs every command through one kernel (`Kernel::handle`), so both adapters share all kernel code (T00.09).

## Owns

- The kernel: the Stage 00 commands (`system.init`, `system.health`, `task.create`, `task.start`, `task.step`, `task.complete`, `state.show`, `ledger.tail`) and their validation. A client idempotency key on a command that takes none (`system.init`, which has its own built-in key, and the read-only commands) is refused, not ignored.
- The one-transaction path for every consequential command (Architecture §8): idempotency check → state change → event append → idempotency record → commit. A retry with the same key returns the stored response and changes nothing; the same key with a changed request is `idempotency_conflict`.
- Read-only quarantine (owner decision 2026-10-08): when the ledger-consistency check fails, because current state and history disagree or because the check could not run, at open or on any later check, the daemon keeps serving reads and `system.health` (with a Critical `writes` component and a `quarantined` field naming the problem, so health is never green while writes are refused, even after the cause has cleared) and refuses every write with `quarantined` until the owner repairs the store and restarts the daemon. A retry of a request already committed still gets its stored response (owner decision 2026-10-08: a replay only reads). The quarantine lasts for the daemon's lifetime.
- The Stage 00 entities in the object registry: `Owner`, `Project` (untrusted for execution), `Task` (its title is owner content, with a privacy class and its own deletion unit, never copied into an event), `Run` (completion and verification recorded separately). `state.show` lists at most 1000 tasks and sets `tasks_truncated` when there are more.
- Crash points 1, 2, 3 and 6 (Test Strategy §5): before the transaction, after the state write, after commit, before the response. Production uses `NoCrash`; `crash-injection` builds can hang at a point for a SIGKILL harness or fail in-process. `RIKURI_CRASH_AT` must name a known point and a command that writes (only those reach a crash point).
- Health on evidence (Architecture §27.8, §53.7): components `build`, `store` and `ledger_consistency`; a component that has never succeeded is not Healthy. The store is judged on reads and writes separately, so a successful read (including health's own consistency replay) cannot hide a failing write, and a refused request is not a store failure. The ledger-consistency check replays history against current state at open and on `system.health`; it is also the crash matrix's no-duplicate-effect check (W1).
- The local socket server: one request line per connection, one response line, `REQUEST_TIMEOUT` per read, `MAX_REQUEST_BYTES`. Every request, including one refused before it parses, is logged by command and outcome only, never by parameters; the command name is escaped and bounded so it cannot forge a log line. A stale socket left by a dead daemon is replaced; anything else at the socket path (a regular file, a directory, a symlink) is refused and never deleted.
- The home directory layout: `state/rikuri.db` (via `rikuri-storage`, whose writer lock makes it one daemon per home) and `run/rikurid.sock`. Home defaults to `$RIKURI_HOME`, then `~/.local/share/rikuri`.

## Must not own

- Storage mechanics, migrations or the ledger's tables (`rikuri-storage`, `rikuri-ledger`).
- The envelope's shape or error vocabulary (`rikuri-api`).
- Models, tools, network access, policy decisions beyond Stage 00 commands: later stages.

## Trust level / boundary

Trusted. Every request is untrusted input: envelope version, request ID length, command name, parameters and idempotency keys are validated, unknown fields are refused, and error text quoting the request is bounded. The socket and its directory are owner-only: an existing socket directory that group or others can access is refused, because the socket briefly has the umask's mode between `bind` and its `chmod 0600`.

## Canonical data touched

`objects` (owner, project, tasks, runs) and, through the ledger, `events` and `idempotency`.

## Public contracts / interfaces

Binary `rikurid [--home DIR] [--socket PATH]` (`--help` prints the usage and exits 0). Library: `Kernel::{open, handle, check_consistency, quarantined, store}`, `KernelConfig`, `KernelError`, `Idempotency`; `kernel::database_path`; `server::{bind, serve, socket_path, REQUEST_TIMEOUT, MAX_SOCKET_PATH_BYTES}`; `crash::{CrashPoint, CrashHook, NoCrash, FailAtPoint, HangAtPoint}`; `health::{ComponentHealth, HealthState, report, ConsistencyReport}`; `entities`.

## Allowed dependencies

`rikuri-api`, `rikuri-domain`, `rikuri-ledger`, `rikuri-storage`, `serde`, `serde_json`, `thiserror`. Test-only: `tempfile`, `ulid`.

## Forbidden dependencies

Any network, async runtime, provider or model crate in Stage 00.

## Failure and degraded behavior

A command fails with a typed `ApiError`; storage failures map to `storage_failure`. A crash at any point leaves either no change or the whole change, and the retry with the same key returns the original outcome. Disagreeing state and history quarantine writes (above). A second daemon on the same home fails to open (`rikuri-storage`'s writer lock). A build with `crash-injection` reports itself Degraded, and with idempotency off, Critical. A production build that sees the test variables warns that it ignores them.

## Health / last-success signal

`system.health` returns the worst component state, then each component, and `quarantined` when writes are refused:

```text
state / last_attempt / last_success / last_error / consecutive_failures / work_completed:
  build               — Healthy only in the production configuration
  store               — from rikuri-storage's StoreHealth: Degraded after 1–2 failed reads or
                        writes in a row, Critical after 3; reads and writes judged separately.
                        Covers the whole write path: an internal kernel fault inside a write
                        transaction counts here too, and the detail names the cause
  ledger_consistency  — last replay of history against current state
  writes              — only while quarantined: Critical, naming why writes are refused
```

## Architecture / ADR links

- Architecture §6.2, §8, §25.4, §27.8, §42, §53.7; Test Strategy §5 (crash points).
- ADRs: 0001 (IDs), 0002 (store), 0003 (ledger), 0004 (threat model assumptions, Proposed).

## Threat / control links

- Threat: a retried or crash-replayed command applies twice. Control: one transaction with the idempotency record. Tests: `replay_and_conflict`; T00.06 property, T00.07 crash matrix, T00.08 regression (not yet written).
- Threat: the daemon keeps writing on top of state it cannot trust. Control: read-only quarantine, sticky, with health Critical throughout. Tests: `disagreeing_state_quarantines_writes`, `quarantine_keeps_health_critical_after_the_cause_clears`, `a_quarantined_daemon_still_replays_committed_requests`.
- Threat: owner content leaks into logs or the audit trail. Control: requests logged by command and outcome only; task titles never copied into events.
- Threat: health reports green while writes fail, or on liveness alone. Control: reads and writes judged separately; never-succeeded is not Healthy; refusals are not failures. Tests: `store_health_reports_failed_writes`, `refused_requests_do_not_mark_the_store_failed`, `a_store_without_success_is_not_healthy`.
- Threat: starting the daemon destroys a file at a mistyped socket path. Control: only a socket is ever replaced. Test: `bind_never_deletes_a_file_that_is_not_a_socket`.
- Threat: two daemons write one home. Control: the store's writer lock. Test: `one_kernel_per_home`.

## Required tests

- [x] defect regressions — `tests/s00_07_defects.rs` (round 1), `tests/s00_07_round2.rs` (round 2) and `tests/s00_07_round3.rs` (round 3: quarantine stays Critical and sticky after the cause clears, replays of committed requests while quarantined, internal faults inside writes are not green, one log line per request with no forging and bounded envelope errors), plus round 2: `state.show` answers (deadlock regression, with a deadline), shared socket directory refused, socket owner-only, health under failed writes and refusals, replay and conflict, quarantine, one kernel per home, no file deleted by `bind`, bounded parameter errors, keys refused where none is taken, `state.show` truncation
- [x] T00.09 CLI → kernel → repository → ledger as one real path; socket and in-process adapters give byte-identical envelopes and the same exit codes — `apps/rikuri-cli/tests/t00_09_cli_matches_in_process.rs`
- [x] T00.10 smoke checks content, not liveness: a task title in `state.show`, never in the ledger, health healthy, the task after a restart — `scripts/quality/smoke.sh` (a gate step), with its self-test `scripts/quality/tests/smoke-checks-content.sh`
- [ ] T00.06/T00.07/T00.08/T00.12 crash matrix, property and regression

## Documentation / help entry points

This file; rustdoc; `rikurid --help`; `rikuri --help`.

## Known limitations / explicit debt

- Fixed 2026-10-08, tests-first ([S00.07 record](../../docs/build-records/2026-10-08-s00-07-rikurid.md)): `state.show` deadlocked the daemon; an existing socket directory was not checked; health showed the store healthy while writes failed; `bind` deleted any non-socket file at the socket path; and the smaller items listed in that record.
- One connection at a time with a per-read timeout: a slow or non-reading client can stall the daemon for others. Acceptable while the socket is owner-only (ADR 0004); must change before any caller other than the owner can reach a socket (for example the Pi harness's mounted socket, ADR 0000 September 28 decision 3).
- The consistency replay checks events against tasks and run links and catches duplicate effects by idempotency key, but does not yet compare every completion event with run state; to be extended with the crash matrix (T00.07).
- The crash-matrix tests (T00.06–T00.08, T00.12) are not written yet.
- Unix only (local socket); Windows arrives with Windows support (product Stage 9).
