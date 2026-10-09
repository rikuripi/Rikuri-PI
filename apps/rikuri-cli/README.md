# rikuri-cli — Module Contract

Template C, [Engineering Templates v1.1](../../doc/Rikuri_Pi_Engineering_Templates_v1.1.md#template-c--module-contract). Stage Plan item S00.07.

## Purpose

`rikuri`, the command-line client of `rikurid`: a thin, untrusted client that builds a request envelope, sends it over the daemon's socket and prints the response envelope as one JSON line.

## Owns

- Argument parsing (`clap`) for `init`, `task create|start|step|complete`, `state show`, `ledger tail`, `system health`, and `raw` (a request envelope read from standard input, sent unchanged).
- Mapping the response to the documented exit code (`rikuri-api`): 0 success, 2 usage, 3 request refused, 4 daemon unavailable, 5 daemon failure.
- Locating the daemon: `--home` (default `$RIKURI_HOME`, then `~/.local/share/rikuri`) and `--socket` (default `HOME/run/rikurid.sock`).

## Must not own

- State, decisions or validation of meaning: the daemon validates everything.
- Retries: a `daemon_unavailable` result is printed; the caller retries with the same idempotency key.

## Trust level / boundary

Untrusted client. It holds no state and no authority; anything it sends is validated by `rikurid`.

## Canonical data touched

None.

## Public contracts / interfaces

The `rikuri` command line and its exit codes; output is the `rikuri-api` response envelope, one JSON line (`--pretty` indents it).

## Allowed dependencies

`rikuri-api`, `rikuri-domain`, `clap`, `serde_json`. Test-only: `rikurid` (library), `tempfile`, `ulid`.

## Forbidden dependencies

`rikuri-storage`, `rikuri-ledger`, `rikurid`'s library (allowed only as a test-only dependency, so T00.09 can serve a daemon to the real binary), any network or async crate.

## Failure and degraded behavior

A usage error exits 2 before anything is sent. A daemon that cannot be reached or does not answer within `RESPONSE_TIMEOUT` gives `daemon_unavailable`, exit 4, with the command's outcome unknown.

## Health / last-success signal

N/A (no state between runs). `rikuri system health` asks the daemon.

```text
state / last_attempt / last_success / last_error / consecutive_failures / work_completed:
  N/A — reported by rikurid
```

## Architecture / ADR links

- Architecture §25.4, §42; Stage Plan S00.07.

## Threat / control links

- Threat: a client decides or caches state and disagrees with the daemon. Control: the CLI is stateless and prints the daemon's envelope unchanged.

## Required tests

- [x] T00.09: the CLI and the in-process adapter give byte-identical envelopes and the same exit codes — `tests/t00_09_cli_matches_in_process.rs` (runs the real binary against a daemon served from the `rikurid` library)
- [x] T00.10: smoke through the CLI checks content in `state show` — `scripts/quality/smoke.sh`
- [x] exit codes the parity run does not reach: 2 (usage), 4 (no daemon), 5 (a refused write) — `tests/cli_exit_codes.rs`

## Documentation / help entry points

`rikuri --help` and each subcommand's `--help`; this file.

## Known limitations / explicit debt

- Unix only (local socket).
