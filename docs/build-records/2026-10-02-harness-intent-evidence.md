# Build Record — harness intent and evidence

| Field | Value |
| --- | --- |
| Change | Clarify harness intent, evidence routing and bounded evolution |
| Date | 2026-10-02 |
| Author | Codex |
| Branch | codex/creative-studio-security-drafts |
| Commits | Commit containing this record; message: Clarify harness intent and evidence contracts |
| Stage Gate | Documentation refinement; no gate closure |
| Requirements | HAR-001–004; Architecture §11.1–11.3 |
| Status | COMPLETE — documentation only |

## Plan and characterization

Define a preserved-intent/clarification contract and evidence-directed small-model orchestration, connect bounded verification/repair and evaluated profile promotion to existing architecture without expanding Foundation. I will know this works when all four requirements map to allowed/adverse acceptance, stage ownership remains consistent, local links resolve, diff checks pass and independent verifier review reports no unresolved Important findings.

Read current status, documentation index, Capability Planner, context admission, Pi/profile/router contracts, live plan, master sequence, templates and review/verifier policy. Initial status contained only the unrelated untracked `Rikuri Pi.code-workspace`; preserve it. Current design has the components but lacks this unified contract. No application/Rust workspace/full gate exists.

Planned files: Architecture, PRD, Test Strategy, ADR 0000 dated addendum, documentation index, this record. The owner explicitly authorized these refinements; this includes the affected protected acceptance/ADR documentation. Risk note: wording could silently expand scope, imply runtime guarantees or let model evidence grant authority. Preserve deterministic Foundation routing, Pi-primary substrate, policy before assembly/action, no-network Local Only, Secret exclusion, hard budget, current fix-lock and independent review. No policy/core implementation changes.

## Build and boundaries

Architecture owns the contract; PRD owns HAR IDs; Test Strategy owns future fixtures; ADR preserves history and clarifies scope; index wires discovery. Task/turn controller is the intended caller. Measurements and RAG are distinct, mixed tasks use both, and failure cannot produce verified completion. Model confidence, citations and tool invocation are insufficient proof. Creative assumptions remain explicit. Exact small-model hardware behavior is unproven.

No dependencies selected or installed: infrastructure evaluation, module contracts, migrations, recovery execution and runtime tests are N/A for documentation-only work. Debug/fix-lock is N/A: no executable defect repaired. Existing large architecture file retained as the authoritative specification rather than split into a competing source.

## Verify and close-out

Pass: git diff --check (exit 0); all 98 local file links resolve (exit 0); independent verifier confirms five added fragment anchors and current version/S/T/product-stage consistency. Guard suite: python3 -m unittest discover -s .claude/hooks -p 'test_*.py' — 18 tests OK, exit 0; expected refusal diagnostics are adverse-test fixtures, not repository failures. Independent verifier ran Bugs, Security & boundaries and Compliance passes: no Important findings. One Nit about blanket factual lookup wording was clarified in HAR-002 to use declared task evidence requirements and optional every-answer lookup profiles. No self-approval is claimed; owner decides acceptance. Scrap Sweep: inspected tracked diff and untracked task record; removed /tmp/rikuri-harness-refine.py; retained unrelated workspace file. No disabled checks, deleted durable artifacts or unexplained debt. Owner requested commit and push after verification; include only the six planned task files and push the current task branch. Runtime, model/hardware benchmarks, node integration and CI are Not yet proven; no live system exercised. Deferrals retain owning product stages in Architecture §11.3 and Test Strategy §19, owner product owner/owning-stage implementer, activation after predecessor gates and allowed/adverse evidence. Stage Gate Records N/A because no implementation gate changed. User help/RAG discovery updated through documentation index; runtime runbooks N/A.
