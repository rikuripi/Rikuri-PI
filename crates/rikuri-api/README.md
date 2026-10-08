# rikuri-api — Module Contract

Template C, [Engineering Templates v1.1](../../doc/Rikuri_Pi_Engineering_Templates_v1.1.md#template-c--module-contract). Stage Plan item S00.07.

## Purpose

The agent-facing contract of the Rikuri kernel: a versioned JSON envelope (one request line in, one response line out) with typed success and error states, stable error codes and documented CLI exit codes (Architecture §25.4, §42). The daemon's socket and the in-process adapter speak the same envelope (T00.09).

## Owns

- The envelope: `Request` (`v`, `command`, `params`, optional `request_id`, optional `idempotency_key`) and `Response` (exactly one of `result` and `error`, plus `replayed`).
- The error vocabulary: `ErrorCode` with stable wire names, each with an exit code and a retryable flag; `ApiError` (code, message, retryable, optional details).
- The Stage 00 command list and their parameter types (`command`), each refusing unknown fields.
- The local-socket client (`client::send`, Unix only), so every caller speaks the protocol the same way.
- Limits: `ENVELOPE_VERSION`, `MAX_REQUEST_BYTES`, `client::RESPONSE_TIMEOUT`.

## Must not own

- Any decision about a command: validation of meaning, state, policy and idempotency happen in `rikurid`.
- Storage, ledger or domain logic beyond the types it serializes.
- Human rendering: the CLI prints the envelope; prettier output is a layer above.

## Trust level / boundary

Shared by the trusted daemon and untrusted clients. Parsing is strict (unknown fields refused), so a misspelled or injected field is an error rather than silently ignored. A client-side failure to reach the daemon is reported as `daemon_unavailable`, with the outcome of the command unknown; the caller retries with the same idempotency key.

## Canonical data touched

None.

## Public contracts / interfaces

`Request`, `Response` (`success`, `failure`, `exit_code`), `ApiError`, `ErrorCode` (`invalid_request`, `unsupported_envelope_version`, `unknown_command`, `not_initialized`, `not_found`, `version_conflict`, `idempotency_conflict`, `invalid_state`, `storage_failure`, `internal`, `daemon_unavailable`); `ENVELOPE_VERSION`, `MAX_REQUEST_BYTES`; `command::{ALL, is_consequential, NoParams, TaskCreate, TaskStart, TaskStep, TaskComplete, LedgerTail}` and the command-name constants; `client::send` (Unix).

Exit codes: 0 success, 2 usage, 3 request refused, 4 daemon unavailable, 5 daemon failure.

## Allowed dependencies

`rikuri-domain`, `serde`, `serde_json`, `thiserror`.

## Forbidden dependencies

`rikuri-storage`, `rikuri-ledger`, `rikurid`, any network, async runtime or provider crate.

## Failure and degraded behavior

Every failure is a typed `ApiError` inside a `Response`; nothing panics on bad input. The client never retries on its own; it reports `daemon_unavailable` and leaves the retry to the caller.

## Health / last-success signal

N/A as a library (no state of its own). Health is reported by `rikurid` (`system.health`).

```text
state / last_attempt / last_success / last_error / consecutive_failures / work_completed:
  reported by rikurid
```

## Architecture / ADR links

- Architecture §25.4 (agent-facing API), §42 (error and exit-code contract).
- ADRs: ADR 0001 (IDs in responses).

## Threat / control links

- Threat: an injected or misspelled parameter is silently ignored. Control: `deny_unknown_fields` on every parameter type. Test: unit tests in `command.rs`.
- Threat: an unbounded request exhausts the daemon. Control: `MAX_REQUEST_BYTES`, enforced by the daemon's reader.

## Required tests

- [x] unit (wire names, exit codes, parameter parsing) — inline tests in `lib.rs` and `command.rs`
- [ ] shared contract — the same requests through the socket and in-process give the same envelopes (T00.09, with `rikurid`)

## Documentation / help entry points

This file; rustdoc; `rikuri --help`.

## Known limitations / explicit debt

- The envelope is version 1; a version change needs both adapters and the CLI updated together.
- The client is Unix-only (Unix domain socket); Windows transport arrives with Windows support (product Stage 9).
