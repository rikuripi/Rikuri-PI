# Adapter simulator v0.1

Status: isolated implementation and test scaffold for the [adapter design](ADAPTER_INTERFACE.md); no production caller or real worker. Executable source is adapter_simulator.py. New fixture-only schemas define lifecycle, normalized events and gate receipts separately from TaskContract/TaskResult v1; no existing kernel type/API or ID registry changes.

## Implemented slice

- WorkerAdapter Protocol: describe, prepare, start, observe, cancel, collect, release.
- Frozen handles/candidates/receipts and typed refusal/phase values.
- Controller-issued dispatch intent committed before fake runtime effect.
- Exact attempt binding for runtime query, cancellation and capture.
- Unknown acknowledgement/query never causes automatic redispatch.
- Cancellation pending remains STOP_UNKNOWN until actual stop observation; no release while uncertain.
- Trusted immutable candidate capture, independent task-check coverage/hash/actor/version checks, final authority recheck and separate application state.
- Gate evidence and captured bytes retained in synthetic SQLite state; events contain only normalized bounded metadata, not task text.
- Explicit release permit marks logical release without deleting artifacts or receipts.
- Bounded observation cursors reject a future cursor instead of substituting a stale tail.

## Trust and proof limits

FixtureAuthority is a runner-authored service in the same process. Its deterministic handles are neither secret capabilities nor authenticated user permissions. Receipt actor equality and shape checks do not authenticate a real producer. No code/command in task text executes. The paired-source gate recognizes only the exact authored correct/incorrect mean source and invokes its paired trusted function; it is not a verifier of arbitrary generated Python.

The SQLite Journal stores synthetic contracts/content outside event metadata. It is not canonical Rikuri state, a second production control plane or a tamper-resistant audit log. Always supply a fresh test-owned scratch database. Tests model lost acknowledgements and reopen connections; this does not prove SIGKILL/power-loss recovery, distributed leases or concurrent controllers. Internal journal writes are trusted; malicious database tampering is outside this fixture.

The virtual clock checks expiry at admission and verification. There is no real autonomous deadline/revocation monitor, process cancellation or hard budget enforcement. Only zero-cost/no-network/single-attempt authored mean work is supported. Unsupported requirements refuse. Current runtime registry is one synthetic profile, not native Pi/Codex/Claude compatibility. No real application occurs.

## Reusable checks

From this directory inside an approved fixture environment:

    python3 -B -m unittest discover -s tests -v

This includes unchanged original25 and new32 simulator tests. With the separately pinned standard-validator dependency set:

    python3 -B -m unittest discover -s qualification_tests -v

This includes unchanged qualification11 plus new2 schema cases. No test is skipped to avoid a missing package. SQLite databases use TemporaryDirectory and close their connections before cleanup.

The [Build Record](../../../build-records/2026-10-14-adapter-simulator.md) records actual results/review. Test definitions are not evidence until executed. Next real-runtime work requires a separately scoped adapter/cost/containment authorization and appropriately independent verification of actual output; this scaffold grants none.

The fixed demonstration entry point is python3 -B adapter_simulator.py. It prints success, failed gate, lost acknowledgement, uncertain cancellation and denied grant scenarios from fresh test-owned databases; it never creates a native worker.

Capability description is informational in this fixture. The controller enforces supported constraints through runtime checks; negotiated capability proof remains work for a production adapter.
