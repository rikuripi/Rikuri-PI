# Versioned legacy observation schema

## Task Contract
- Outcome: Give the read-only bridge a closed versioned wire contract validated before export.
- Scope: Routine isolated scaffold: orchard-observation.schema.json, orchard_bridge.py, tests/test_orchard_observation.py, qualification_tests/test_orchard_observation_schema.py, ORCHARD_BRIDGE.md and README. No production wiring, Orchard edits or original TaskContract/TaskResult changes.
- Done when: Every bridge observation is validated before return; closed shape/version and status consistency enforced; metadata digest binding checked separately; unauthorized promotion refused; custom/standard schema engines agree on representative refusals; Ubuntu pure fixture regressions and actual CLI pass; independent review and links/diff checks.
- Authority: Owner Next approved shared schema/export validation. Existing normal shell pure-test exception and inspected-fixture Ubuntu namespace layer; one required verifier. No real worker, database, credentials, network, commit/push/PR or full gate.
- Limits: One task, at most two repair cycles/two review rounds. No dependencies or paid work.

## Plan and risk
Name the schema narrowly for legacy snapshot observations; it does not represent authenticated runtime events or canonical receipts. Preserve version1 output shape and field semantics, adding closed wire/schema constraints and semantic digest verification. Null legacy application is an explicit enum member, compatible with the intentionally small custom validator vocabulary. Schema validity cannot confer provenance, freshness, admission or authority.

## Completion evidence — DONE for this scaffold
- Host14 bridge/observation tests and3 new schema qualification tests passed through unittest entry points.
- Final Ubuntu immutable154file snapshot through unchanged approved namespace launcher:71 pure tests +16qualification tests, no skips; CLI accepted/refused paths passed, exit0. Nested original25 regressions are not counted as additional named cases.
- Namespace admissions passed, global protections unchanged and owned staging removed. No new dependency, worker, database, model or production caller.
- Independent source review PASS, no Important/nits across bugs/security/compliance. Reviewed representative schema parity and digest scope. Reviewer ran git status/diff checks; tests/fullgate were not independently rerun.
- Local Markdown links and git diff --check passed. Zero repair cycles. Full project gate, guard suite and Orchard regressions remain unproven, no merge/stage claim.
- Evidence: /Users/hchome/Documents/Codex/2026-10-14/i-have-two-new-documentation-in/outputs/observation-schema/verification.md, test-output.txt and manifest.json.
- Changes uncommitted, prior work preserved; no commit/push/PR.
- Next recommended task, not started: add an explicitly read-only observation export to Orchard with disposable application regression coverage, using this schema boundary.
