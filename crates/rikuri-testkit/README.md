# rikuri-testkit — Module Contract

Template C, [Engineering Templates v1.1](../../doc/Rikuri_Pi_Engineering_Templates_v1.1.md#template-c--module-contract). Stage Plan item S00.08 ([Build Record](../../docs/build-records/2026-10-09-s00-08-rikuri-testkit.md)).

## Purpose

The shared test fixtures: deterministic clocks and IDs, and an in-process crash injector for the kernel's crash points. Every suite in the workspace takes these from here instead of defining its own, so "deterministic" means the same thing everywhere.

## Owns

- `FixtureClock`: a `Clock` that reads a fixed time, or steps forward a fixed amount on every reading.
- `FixtureIds`: an `IdSource` giving ULIDs 1, 2, 3, … at a fixed time, with a shared switch that makes it fail with `IdError::SourceUnavailable`.
- `CrashInjector`: a `CrashHook` that records every crash point it reaches (point, command, `Progress`), and once armed fails the next matching point with `Crashed`, then disarms. It can be armed for any command or for one command. Clones share one state, and it always reports itself armed, so a kernel built with it is a test kernel.
- `T0` and `T0_TIME`, the one fixed time the fixtures use unless told otherwise (as a ULID time and as a `Timestamp`).

## Must not own

- The crash-point vocabulary or the seams themselves (`CrashPoint`, `Crashed`, `Progress`, `CrashHook`, `Clock`, `IdSource`): those live in `rikuri-domain` (owner decision 2026-10-09), so the kit depends only on the vocabulary, never on the code it tests.
- The kernel's own hooks (`NoCrash`, `HangAtPoint`): `rikurid`.
- Fakes of interfaces that do not exist yet: `FakeModelAdapter` arrives with `ModelAdapter` (S02.07) and `FakeTool` with `Tool` (S04.01), resequenced by the owner on 2026-10-05.
- Anything a runtime path uses: no runtime crate depends on this one except as a dev-dependency.

## Trust level / boundary

Test-only. It handles no untrusted input and never runs in a shipped binary.

## Canonical data touched

None.

## Public contracts / interfaces

`FixtureClock::{fixed, stepping}`; `FixtureIds::{sequential, failure_switch}`; `CrashInjector::{new, arm, arm_for, seen, clear_seen}` (arming again replaces a pending point); `Reached`; `T0` (ULID time) and `T0_TIME` (the same instant as a `Timestamp`).

## Allowed dependencies

`rikuri-domain`, `ulid`. Test-only: `serde_json`.

## Forbidden dependencies

`rikurid`, `rikuri-storage`, `rikuri-ledger`, `rikuri-api` and any app: they are the code under test, and storage's or the ledger's suites must not pull in the daemon to get a clock. Network, async runtime, provider or model crates. The reverse is enforced: the gate's crate policy refuses `rikuri-testkit` as any member's normal or build dependency (`scripts/quality/crate-policy.sh`, rule 5).

## Failure and degraded behavior

A switched-off `FixtureIds` fails every request with `IdError::SourceUnavailable`. `CrashInjector` never panics on its own lock: none of its methods panic while holding it, and a poisoned lock would be recovered rather than unwrapped.

## Health / last-success signal

N/A: no runtime component. Its correctness is its own test suite (below), and the suites that use it.

## Architecture / ADR links

- Stage Plan S00.08; Test Strategy §5 (crash points).
- The crash-point vocabulary's home: `rikuri-domain` Module Contract.

## Threat / control links

- Threat: suites disagree on what "deterministic" means, so a property holds in one suite and silently not in another. Control: one set of fixtures. Tests: `tests/fixtures.rs`.
- Threat: a test kernel passes for a production one. Control: `CrashInjector::is_armed` is always true, so `system.health` reports a test build.

## Required tests

- [x] `tests/fixtures.rs`: fixed and stepping clocks; one `T0` for clocks and IDs; sequential IDs at a fixed time and the failure switch; ADR 0001's ID properties for `FixtureIds` (canonical text, text and JSON round-trip as their own kind only, `AnyId`, text order equals ID order); the injector records every point in order with its progress (including the one it crashes at), crashes once at an armed point for any command, ignores other commands when armed for one, disarms, and arming again replaces a pending point.
- [x] Real callers: the `rikuri-storage` and `rikuri-ledger` contract suites (T00.04, T00.05) and their other suites, `rikurid`'s T00.06 property test, crash-point placement, health and S00.07 round-3 suites, and the CLI's T00.09. (The real-process crash matrix, T00.07/T00.08, drives a daemon and uses none.)

## Documentation / help entry points

This file; rustdoc.

## Known limitations / explicit debt

- `rikuri-domain` keeps three local fixtures: `tests/id_properties.rs` injects arbitrary ULIDs from proptest, which `FixtureIds` does not produce; the unit-test fixtures in `src/id.rs` and `src/deletion.rs` cannot use this crate, because a crate's own unit tests would compile a second copy of domain through it (its integration tests could).
