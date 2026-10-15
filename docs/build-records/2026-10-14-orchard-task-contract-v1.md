# Orchard TaskContract v1 — 2026-10-14

## Task Contract

- **Outcome:** Define a provider-neutral task/result contract and demonstrate truthful success/failure/denial outcomes in a fake adapter.
- **Scope:** Routine design scaffolding under `docs/architecture/contracts/task-contract-v1/`, documentation index and Orchard integration reference. No production Rust/API/state/policy changes, ID registration, live worker, migration, dependency install, network or new stage gate.
- **Done when:** Versioned strict request/result schemas; maintained ID/privacy vocabulary; external exact-contract approval boundary; negative tests for missing/unauthorized/terminal-state cases; fake success/failure/denial traces; independent verifier pass; approved targeted test checks and whitespace/link validation with honest gate disposition.
- **Authority:** User said “Go ahead” to TaskContract/schema and fake-adapter validation. Local files and authored fixtures only; no commit/push/PR/merge. A macOS safety-layer exception and full-Ubuntu-gate deferral have been requested; tests wait for the owner's choice.
- **Limits:** One task, one verifier; at most two repairs/two review rounds. No model calls or paid services; fake controller performs one attempt and starts no follow-on job.

## Plan and risk note

1. Reuse existing prefixed ULID IDs and privacy enums; content references avoid adding kernel entity kinds.
2. Publish closed JSON Schema request/result shapes. No existing JSON Schema library is installed; use a dependency-free fixture checker for exactly this local vocabulary, refusing unknown keywords. This is explicitly not a production or general JSON Schema validator.
3. Test a runner-authored in-memory mean fixture, exact digest-bound external grants, independent fractional/empty checks, terminal semantics, no automatic repair/follow-on and in-memory replay.
4. Link Orchard to the single Rikuri-owned contract source, review and record evidence.

I will know this works when a correct mean fixture produces DONE with independent criterion evidence, integer division is BLOCKED despite completed execution, absent evidence is UNVERIFIED, and missing/stale/capability-exceeding grants cannot start a worker.

Risk tier is routine scaffolding: this code is not imported by any production caller and never executes arbitrary task code or commands. Permission, budget, privacy and idempotency logic is simulated; it is not a production enforcement path. No gate/security control or protected Rust path is changed. Production admission needs a qualified schema engine and real policy/approval/revocation, budget, independent-gate and durable-state integration. Current uncommitted documentation from earlier tasks is preserved, not committed or claimed newly authored here.

## Deferred

Real native adapters; OS containment qualification; strict budget reservation/timers; cancel-during-run; retry scheduling; durable/concurrent replay; entity schemas; current Orchard full regressions; importer/migration; full Ubuntu release checks. No implementation or Foundation-0 completion claim follows from these fixture tests.

## Verification progress

- Independent read-only review: no Important source findings in routine fixture scope; two Nits (check identifier length mismatch and unresolved fake gate ID) corrected. Bounded follow-up confirmed both corrections.
- Owner approved disposable macOS sandbox for new contract tests, with full Ubuntu gate deferred.
- FAIL / environment: initial sandbox denial probe exited 134 with no output before test execution. Allowing executable mapping for Python startup retained denials but the second launch also exited 134 with no output. No containment check or unit test is reported passed.
- PASS: local documentation link targets and git diff --check in both repositories.
- Not yet proven: fixture tests, synthetic trace entry point, macOS containment, full Ubuntu gate and current Orchard regressions.
- Pending owner choice: pure contract-only tests under the normal Codex shell sandbox, or verification on Ubuntu. No further native-sandbox startup repair is attempted in this task.

## Final approved verification and close-out

The owner explicitly authorized only the pure contract tests under Codex's normal shell sandbox after the native sandbox startup failures. OS containment and the full Ubuntu gate remain Not yet proven. This exception does not authorize general repository tests or real worker execution.

- PASS: in the contract directory, PYTHONNOUSERSITE=1 PYTHONDONTWRITEBYTECODE=1 python3 -B -m unittest discover -s tests -v; exit 0, 25 tests, 0.034s, OK. Tests used an agent-created temporary directory and bytecode was disabled.
- PASS: python3 -B validate.py; exit 0; success DONE, failed acceptance BLOCKED, missing evidence UNVERIFIED, missing approval NEEDS_OWNER_DECISION. A second entry-point invocation retained deterministic [synthetic trace receipts](evidence/2026-10-14-task-contract/traces.json) as task evidence, not another live-worker run.
- Independent verifier: PASS for fixture scope, no Important findings. Both initial nits corrected and bounded follow-up confirmed. Final verdict assessed the parent-run, owner-approved checks; reviewer did not rerun tests or perform a third source review.
- PASS: final whitespace, local document link targets and retained evidence JSON parse.
- Not yet proven: OS containment, full Ubuntu gate, real adapter integration, canonical migration, durable replay or hard resource enforcement. Current Orchard full regressions were not run.
- DONE: contract/schema, negative tests and synthetic success/failure/denial evidence meet this task's scoped acceptance. No Foundation-0 gate or production admission is claimed.
- No credentials, model calls, network installs, production data mutation, commit, push, PR or merge. Previous uncommitted documentation remains preserved.
- Next recommended task, not started: independent qualification with a standards-compliant JSON Schema engine and a real adapter interface design; live execution remains separately authorized.

## Tested source hashes

- README.md — SHA-256 9805caf168675a68aea6c8edf786015443a0013cac5fe40407b0e2e934d749fc
- examples/mean-task.json — SHA-256 62851c91d477714161a4e4439947e017f6289261d79af67882cd9e204f4010f7
- task-contract.schema.json — SHA-256 4ba9c4c7b50b37d9ff0a8bbe77140c28fddacb91afe5d4902f37d2de3d8a8bc0
- task-result.schema.json — SHA-256 4576a695a0a4cf1226fdd60210b1f784d8f3f4c0e446cfb1c22a919d93dac8d6
- tests/test_contract.py — SHA-256 12eefd8c7ee6684e8b56f02e20dc2e94d6b77534663814ab6868b96aba0e3dbf
- validate.py — SHA-256 9cad85b7d2175fbf5262c9b32cd156ae5c4038bb91af8e1f517dd5c12885d39a
