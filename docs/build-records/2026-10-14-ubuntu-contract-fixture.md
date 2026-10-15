# Ubuntu contract-fixture run — 2026-10-14

## Task Contract

- **Outcome:** Prove the original synthetic contract tests run on the upgraded Ubuntu guest inside a scoped safety layer.
- **Scope:** High-risk privileged test-environment setup, limited to reviewed temporary namespaces and immutable authored fixture snapshot; then 25 dependency-free tests. No real workers, production data, global protection changes, dependency installs or stage-gate closure.
- **Done when:** Exact source hashes/allowlist; private namespaces; non-root test process without groups/capabilities/privilege gain; masked shares/socket paths; read-only snapshot; no routes plus ENETUNREACH probe; original 25 tests pass; global values unchanged; own staging removed.
- **Authority:** Owner approved preparation, then explicitly said Yes to executing reviewed temporary guest-root setup followed by privilege drop. Existing unprivileged layer is unavailable; this is a scoped exception, not its replacement. No commit/push/PR/merge/deployment.
- **Limits:** One fixture task, no follow-on work; two bounded repairs and two focused execution-phase review rounds. The earlier preparation review/clarification was a separately authorized task. No paid model use.

## Plan, risk and review

A reviewed launcher uses guest-root access only to establish private mount/network/PID namespaces and masked directories, then drops to henry with empty environment, cleared groups/capabilities and NoNewPrivs. Inspected synthetic fixtures alone run; no hostile-worker claim. Staging helper checks exact seven-file manifest/hash, root ownership and immutable regular files before running. Source/probe changes are recorded below. No protected Rust or existing quality script was modified.

Preparation review found no Important issues for the inspected fixture. PID 1 guard and limitations on guest system/delegated sysfs/descriptors were clarified. Execution-phase focused reviews confirmed both bounded corrections preserve admission criteria. Reviewers performed no VM actions.

## Evidence and red runs

- First attempt: staging passed; mask assertion failed before tests. Diagnostic isolated /run. Global restrictions unchanged; staging removed.
- Repair 1: mount -n suppresses userspace mount bookkeeping; kernel mounts/masks/refusals unchanged. Reviewed. Retry passed mask checks but route-file header assumption failed before tests.
- Diagnostic: isolated route file has zero lines.
- Repair 2: accept empty table or sole recognized header, refuse routes/unknown content; additionally require synthetic IPv4 connect failure ENETUNREACH. Reviewed. Original five contract/schema/test files unchanged.
- Final approved run: exit 0; admission checks PASS; 25 tests in 0.033s, OK.
- Both kernel unprivileged-userns and AppArmor restriction values were 1 before/after.
- Guest staging removed by parent helper.
- [Exact output](evidence/2026-10-14-ubuntu-contract-fixture/Ubuntu-contract-test-output.txt), [manifest](evidence/2026-10-14-ubuntu-contract-fixture/ubuntu-contract-payload-manifest.json), [launcher](evidence/2026-10-14-ubuntu-contract-fixture/ubuntu-contract-safety.sh), [probe](evidence/2026-10-14-ubuntu-contract-fixture/safety_probe_and_tests.py).
- Local evidence links and git diff --check verified at close-out. CI not run.

## Disposition

DONE for original Ubuntu fixture. Regular-user namespace admission remains denied. Standard-validator Ubuntu tests, full Rust/Ubuntu gate, hostile-worker/production containment, adapter execution and migrations remain Not yet proven. No global settings or canonical state changed, no other work committed. Earlier uncommitted documentation is preserved.

Next recommended task, not started: qualify a Linux validator wheel set and the 11-test standard qualification suite in the scoped fixture environment.
