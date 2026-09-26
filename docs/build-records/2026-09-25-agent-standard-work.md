# Build Record — Rikuri Pi agent standard work

- Date: 2026-09-25
- Status: COMPLETE
- Scope: Root AGENTS.md and documentation entry points; no runtime or protected-state changes.
- Request: Adapt the supplied generic build procedure and checklist to Rikuri Pi's planned SDLC.
- Baseline: No AGENTS.md, implementation workspace, or quality scripts exist. Engineering Standard v1.2, Engineering Templates v1.1, Stage Plan v2.1, and ADR 0000 already define the workflow.
- Plan: Create concise agent-facing instructions that link those authorities; retain the five laws, plan/build/debug/verify/close loop, safe cleanup, incident discipline, and periodic reviews. Distinguish current documentation checks from planned implementation gates.
- Wiring: Root AGENTS.md is discovered by repository coding agents; root README and documentation index link it.
- Verification: Resolve all local Markdown links; check required workflow sections, current baseline references, and git diff whitespace; review for invented commands, scope expansion, destructive cleanup, and unsupported completion claims.
- N/A: Runtime tests, recovery drills, execution telemetry, and Module Contracts; this change contains documentation only. No implementation stage is closed.

## Evidence and close-out

- Local Markdown validation: PASS, 39 local links across 17 files resolved; required SDLC headings and T00.12/T02.16/T04.18 references verified against Stage Plan v2.1.
- `git diff --check`: PASS.
- Manual review: AGENTS.md links current sources; planned quality-script paths are explicitly unavailable; source checklist is adapted through existing Template A rather than duplicated; cleanup preserves migrations/evidence/shared work; internal and public-release gates remain distinct.
- Scrap Sweep: only AGENTS.md, two README entry points, and this record changed. No temporary files added to the repository.
- Runtime build/test/security gates: N/A for this documentation-only change. All Foundation-0 gate statuses remain unchanged.
- Commit: this record is committed with the agent workflow and README links; locate via `git log -- docs/build-records/2026-09-25-agent-standard-work.md`.
