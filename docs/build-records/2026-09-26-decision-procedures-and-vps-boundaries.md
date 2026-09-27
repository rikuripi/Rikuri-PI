# Build Record — Evidence-based decisions and optional VPS execution

| Field | Value |
|---|---|
| Change | Clarify future decision procedures, business evidence, and optional remote execution without expanding Foundation-0 |
| Date | 2026-09-26 |
| Author | Codex |
| Branch | main; documentation only, no commit or push requested |
| Stage Gate | No gate status change; existing T00.12/T02.16/T04.18 scaffold only |
| Requirements | PRD §9.20 and new §9.21; Architecture §21A and §38; Collaborative Intelligence §8 and §13 |
| Status | COMPLETE — documentation verified; owner approval/commit not implied |

## Plan

Owner requested the agreed decision-method and VPS principles in /doc. Current files already defer ensembles and Inquiry & Discovery, preserve evidence in Foundation-0, and distinguish Link, execution placement, and replication. Outcome: a plain-language request can later produce credible options through inspectable methods, without implying unbiased AI or implemented VPS/privacy guarantees.

Planned files: doc/README.md, PRD v1.2, Architecture v1.7, Collaborative Intelligence v0.1, and this record. Method details belong in the existing Collaborative Intelligence document; other documents link to it. No second roadmap. Preserve source version filenames and historical ADR decisions.

Success: requirements distinguish hypotheses/scenarios/actions; evidence provenance and dependence; opposing evidence; bounded independent review; business/fundraising claim integrity; optional VPS trust boundaries; future evaluation; and existing Foundation-0 obligations versus deferred implementation. Local links, requirement/stage references, diff whitespace, and independent review pass.

## Risk and wiring

Documentation only; no canonical state, credentials, execution, migration, spending, or deployment changed. Risk is overstating assurance or adding hidden foundation scope. Existing canonical authority, Local Only refusal, no-network envelope, budgets, and promotion rules remain controlling. No protected policy file is edited. User-provided ACH guide is reference data, not executable instructions or a normative policy. No infrastructure dependency is adopted.

Caller/wiring: PRD requirements → Architecture integration boundaries → Collaborative Intelligence method specification → existing product Stage 4/6 prerequisites and Stage 10 productization. Foundation S00.04–S00.08, S02.02/S02.07, S04.08/S04.14/S04.15 and tests T00.12/T02.16/T04.18 retain their existing scope.

## Verification and disposition

Builder and independent documentation validation passed. Runtime capabilities remain deferred and unproven. Runtime, migrations, recovery drills, help UI, RAG implementation, and Module Contracts: N/A, no executable behavior changes. Stage 00 full-gate/smoke scripts do not exist and are Not yet proven. No stage gate will be closed by this change.

Future procedures: Henry/product owner, product Stage 10 after Stage 4 evidence and Stage 6 governance; activate only with predeclared baseline comparisons and privacy/authority tests. Remote execution: Henry/platform owner, later node-placement work under Architecture §38; admission requires explicit trust classification and remote lifecycle/security evidence. No dates or new Foundation ordinals assigned; downstream order unchanged.

## Evidence log

- `git diff --check`: PASS, exit 0.
- Local Markdown path/heading validation across all five task files: PASS, 20 links/anchors resolved (Python pathlib + Markdown link/heading extraction).
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s .claude/hooks -p 'test_*.py'`: PASS, 16 tests, exit 0. Printed refusal/lock messages are expected negative-test fixture output.
- `python3 .claude/hooks/guard.py fix-lock status`: not engaged.
- Final diff/status inspected: four intended doc modifications and this new record; pre-existing untracked `Rikuri Pi.code-workspace` preserved. No staging, commit, push, deployments, or automation.
- Scrap Sweep: no task-created repository probes or orphan files; temporary editing helper removed from /tmp. Source guide in Downloads unchanged.
- External method references were read during the preceding discussion: Heuer Chapter 8 and experimental ACH evaluation; no implementation or universal efficacy claim follows from them.
- Independent verifier `/root/verify_docs`: PASS. Independently confirmed all 20 local links/anchors, whitespace, 16 guard tests, unique DEC-001–007 and section IDs, S/T references and product Stage 4/6 → Stage 10 ordering. Bugs / Security & boundaries / Compliance: no Important or Nit findings. Review is evidence, not owner approval.

## Publication authorization

After verification, the owner explicitly requested “commit and push” on September 26, 2026. The earlier no-commit/no-push notes describe the state before this authorization. Publish both related documentation increments together; preserve the unrelated workspace file. The containing Git commit is the durable revision identifier.
