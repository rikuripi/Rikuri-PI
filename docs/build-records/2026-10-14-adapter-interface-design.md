# Provider-neutral adapter interface design — 2026-10-14

## Task Contract

- **Outcome:** Specify an implementable worker boundary and independent evidence handoff using the validated TaskContract.
- **Scope:** Routine documentation only: adapter specification, shared contract README and Orchard integration reference. No wire-schema changes, executable implementation, real worker, state migration or stage-gate changes.
- **Done when:** Operations/ownership defined; authoritative admission separate from task/provider content; completion versus verification/application separate; failure/cancel/replay specified; Orchard and Stage00 compatibility gaps explicit; local links/anchors and whitespace pass.
- **Authority:** Owner said Ok to the named next adapter interface/evidence-handoff design task. Local docs only; no network, test processes, commits/push/PR/merge. Routine documentation-only review is the prescribed local checks plus later owner review.
- **Limits:** One task; two repair cycles/two review rounds maximum; no paid services.

## Design evidence

Read current TaskContract schemas/qualification wrapper, Orchard CodexAdapter.run and its preflight/test lifecycle, and Stage00 task.start/task.complete parameters. Stage00 already separates outcome from verification but its API does not carry this richer contract or authenticate an independent receipt. The prototype returns a worker summary after generic tests; neither behavior constitutes implementation of the proposed adapter contract.

## Deferred

Executable protocol/schema additions, event/gate receipt schemas, current revocation/grant service, model/tool/profile version audit, real adapter/cancellation/replay tests and production integration. No Foundation0 item, gate or completion claim is added.

## Verification and disposition

- PASS: read-only current source comparison: Orchard CodexAdapter.run returns summary after preflight/model turn/generic suite; Stage00 task.complete separates outcome/verification but carries no authenticated gate receipt; documented gaps match actual code.
- PASS: local Markdown target/anchor checks across adapter spec, shared README, Orchard integration reference and this record.
- PASS: git diff --check in both repositories.
- Runtime tests/CI: not run; documentation-only design. No worker, VM operation, production wiring, schema registration or stronger assurance claim.
- DONE: interface operations, source/digest/profile bindings, trusted admission, independent artifact/gate evidence, cancellation uncertainty, replay/ack failure, current compatibility and first implementation proposal documented.
- Changes remain uncommitted; prior work preserved.
- Next recommended task, not started: implement interface types and a deterministic simulator with lifecycle/evidence schemas and refusal tests, isolated from production callers.
