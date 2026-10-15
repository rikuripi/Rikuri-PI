# Adapter types and deterministic simulator — 2026-10-14

## Task Contract

- **Outcome:** Prove the proposed worker/controller/gate lifecycle in a deterministic simulator with refusal and restart/replay evidence.
- **Scope:** Routine isolated scaffolding: Python interface types/controller/adapter/gate, synthetic SQLite journal, three new fixture-only schemas, 32 simulator tests and two standard-schema cases; docs only references. No production callers, native process/model/network, canonical DB, permissions/gate or original wire-schema changes.
- **Done when:** Typed operations; no effect during prepare; exact grant/source/runtime binding; durable intent before synthetic start; no redispatch on lost acknowledgement; explicit uncertain cancel; independent evidence and coverage; retained candidate/receipt across recreated connections; finite stopping; new and existing pure suites pass in approved test environment; independent review; links/whitespace pass.
- **Authority:** Owner said Ok Next to implementation of interface types and deterministic simulator with refusal tests. Existing pure-fixture normal-shell exception and scoped root-created Ubuntu fixture setup are reused only for inspected code. No commit/push/PR/merge, package install, real worker or full gate.
- **Limits:** One task, one verifier, at most two repairs/two review rounds. No paid services or follow-on task.

## Plan and risk note

Implement a Protocol and frozen Prepared/Candidate/GateReceipt/check types. Publish separately named fixture lifecycle/event/receipt schemas, not additional TaskResult v1 fields. Use a real SQLite scratch journal with dispatch intent committed before a durable fake-runtime row; exceptions simulate lost acknowledgements. Recreated controller connections query the same fake runtime and never blindly redispatch.

A runner-owned authority/virtual clock models current grant, source and zero-cost scope; grant handles and actor equality are not production authentication. A trusted paired-source gate accepts only authored mean fixtures, not arbitrary code. All SQLite data is synthetic and test-owned; never open a Rikuri/Orchard database. Single-controller sequential operation only: no distributed leases/concurrency or machine/process/power-loss certification.

Risk tier is routine code scaffolding because no production enforcement/canonical path is touched. Running the approved Ubuntu namespace setup remains the existing high-risk test-environment exception; its launcher and safety admissions remain unchanged. All package code loads after privilege/mask/network/readonly admission. No live node/service access.

## Deferred

Real adapter, authenticated grants/cancellation/receipt transport, producer proofs, automatic active-run deadline/revocation service, hard paid budgets, distributed ownership, crash matrix, real generated-code verifier, production promotion, full Ubuntu gate and Orchard regressions.

## Completion evidence

DONE for this isolated scaffold. Host simulator tests: 32 passed; original contract and standard-schema suites passed during development. Final immutable snapshot on Ubuntu 26.04.1 ARM64/Python 3.14: 57 contract/simulator tests plus 13 schema qualification tests passed with no skips; demonstration passed, guest exit 0. Qualification includes a nested original-25 regression run, not an additional named-case count.

Independent verifier: PASS, no Important findings. Addressed informational-capability documentation nit. Repository diff whitespace check passed. No implementation repair was required after review.

Unchanged approved namespace launcher and verified dependency snapshot; 149 fixed hash-allowlisted files. Admission proved nonroot/no groups/zero capabilities/NoNewPrivs, masked directories, no route, ENETUNREACH for synthetic external IPv4, and read-only fixture. Global protections unchanged; owned guest staging removed.

Evidence: /Users/hchome/Documents/Codex/2026-10-14/i-have-two-new-documentation-in/outputs/adapter-simulator/verification.md, test-output.txt, manifest.json and demonstration.json.

No real worker, canonical-state mutation or artifact application. Full Ubuntu project gate and Orchard regressions remain unproven. Changes are uncommitted; no push, PR or merge.
