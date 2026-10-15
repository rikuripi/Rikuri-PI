# Read-only Orchard observation bridge

## Task Contract
- Outcome: Interpret exported current Orchard task rows without granting Rikuri acceptance or execution authority.
- Scope: Routine isolated scaffold in task-contract-v1: orchard_bridge.py, tests/test_orchard_bridge.py, ORCHARD_BRIDGE.md and README link. Preserve existing changes; no Orchard edits, database connection, worker import/start, canonical writes or TaskResult changes.
- Done when: Bounded JSON CLI accepts a current-format synthetic row; rejects malformed/unknown states; DONE never becomes verified acceptance; application states remain reported claims; sensitive payloads omitted; pure regressions pass through approved Ubuntu fixture layer and independent review.
- Authority: Owner Next approved the proposed read-only compatibility bridge. Reuse approved inspected-fixture Ubuntu namespace setup, normal shell pure tests, one required verifier. No commit/push/PR, native server/model or full project gate.
- Limits: One task, at most two repair cycles/two review rounds; no dependencies, paid work or follow-on task.

## Plan and risk
Crew.list() returns id/request/status/evidence/receipt/application_status. Interpret only id/status/application_status, omit task text/evidence/receipt. Historical DONE is a reported completed execution, never verified Rikuri DONE; BLOCKED/restart cannot prove stopped. Separate reported application from unverified actual application. This is a snapshot observation, not a live status query or authority service. No production caller or capability admission added.

## Completion — DONE for scoped scaffold
- Host ten new bridge tests passed (python3 -B -m unittest discover -s tests -p test_orchard_bridge.py -v).
- Ubuntu fixed151file snapshot: 67 contract/simulator/bridge tests and 13 schema tests, exact counts/no skips enforced, exit0. Actual stdin CLI accepts synthetic DONE/APPLIED without promotion and refuses unknown status; sentinel payload omitted.
- Approved namespace admissions passed; global protections unchanged; owned guest staging removed. Launcher/dependencies unchanged. No real worker or storage read.
- Independent source review PASS, no Important findings/nits (bugs/security/compliance); source states match actual Crew.list and ReviewStore producers. Reviewer did not execute tests independently.
- Local Markdown links resolve; git diff --check passes. No repair cycles used.
- Evidence: /Users/hchome/Documents/Codex/2026-10-14/i-have-two-new-documentation-in/outputs/orchard-bridge/verification.md and manifest.json. Returned output excerpt is tool-truncated; exact-count assertions and exit0 prove counts.
- Full project gate, actual runtime admission and Orchard regressions not yet proven. Orchard application source untouched. Existing uncommitted documentation/scaffold changes preserved; no commit/push/PR.
- Next recommended task, not started: version a shared observation schema and validate it at the export boundary before production wiring.
