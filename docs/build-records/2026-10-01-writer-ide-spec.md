# Build Record — Writer's IDE architecture and specification

| Field | Value |
|---|---|
| Change | Specify the human-authored Writer's IDE and its learning, identity and collaboration contracts |
| Date | 2026-10-01 |
| Author / agent session | Codex with owner direction in this conversation |
| Branch | codex/creative-studio-security-drafts |
| Commits | Included in this task commit; owner authorized commit on 2026-10-02 |
| Stage Gate | Documentation only; product Stage 2 S2.C1–C4, later 6/8 capabilities; no Foundation gate change |
| Related requirements | STUDIO-001–010; WIDE-001–012 in the linked Writer IDE specification |
| Status | COMPLETE — documentation prepared and independently verified; owner review remains |

## 1. Plan

Outcome: document a writer's creative and learning studio with durable structural identity, versioned collaboration, explicit AI roles, contextual questions, scholarly references and optional personal practice.

Current evidence: repository has planning documents and Claude guard tests, no implemented Writer. September 30 Writer architecture specifies stable objects, shared versioning and Pi collaborators but not sentence lineage, collaboration agreements or lesson/flashcard behavior. Owner explicitly requested architecture/spec documentation after defining these boundaries.

Planned files: this record; new `doc/Rikuri_Pi_Writer_IDE_Spec_v0.1.md`; Writer architecture v1.4; PRD v1.2; documentation index; dated ADR 0000 addendum; Test Strategy v1.2 acceptance extension. Preserve original version filenames and accepted history. Master Build Sequence remains the sequencing authority.

I will know this works when every discussed capability has a requirement, owning module, stage/dependency and positive/negative acceptance case; links resolve; creative AI permissions are consistent across affected documents; independent verifier finds no Important mismatch; diff whitespace checks pass. Runtime behavior remains Not yet proven.

Risk note: ADR and Test Strategy are protected policy documents. The owner's explicit request to document this architecture authorizes the task's dated clarification and future acceptance additions. No gate closure, security weakening, hook change or new Foundation work is intended. Main risks are overstating legal ownership, AI authorship enforcement, archival recovery, or editor capability. Address with explicit boundaries, refusal conditions and unresolved implementation decisions.

Architecture impact: schemas/API concepts proposed only; canonical state stays behind governed services; editors/indexes remain replaceable projections; Local Only, budget, revocation and source permissions still apply. Deletion includes retention and current-revocation checks. Model feedback is not a deterministic oracle. Lessons are explicit saved artifacts, not autonomous user profiling. No executable module, migration or runtime contract exists to change.

Component assessment: Tiptap/ProseMirror is a candidate, not selected or pinned. Compare it with Lexical and writer-app workflows before implementation; record exact versions/licenses/dependency and commercial extension boundaries at Stage 2. No dependency installed here.

## 2. Build

Completed all seven planned documentation files. Specification has 12 requirements, module boundaries, conceptual schema/command contracts and 11 positive/negative acceptance cases. PRD, Writer, index, dated ADR and acceptance strategy link the contract. No runtime, cloud calls, repository policy code or deployment changes.

## 3. Debug

N/A: design specification, not defect remediation; no fix-lock or test-baseline edit.

## 4. Verify

PASS: local links/heading anchors, requirement/stage consistency, whitespace and final diff review. Independent verifier applied REVIEW.md Bugs, Security & boundaries, and Compliance passes; no Important findings. Runtime, integration, failure/recovery, benchmark and live adapter checks: N/A for this documentation change; future contracts remain Not yet proven.

## 5. Close out and dispositions

No stage-gate record update: implementation status unaffected. Owner authorized commit on 2026-10-02; no push requested. User's existing untracked `Rikuri Pi.code-workspace` must remain untouched. Runbooks/help/RAG runtime surfaces N/A because no application exists; maintained index, PRD, architecture, ADR and acceptance documents are the documentation wiring.

Implementation, version pinning, consent/legal review, retention default, sentence segmentation rules, layout/export fidelity and review scheduling are owned Stage 2/6/8 decisions documented in the specification; no implied completion or unscheduled new Foundation work.

## Evidence log

- `git status --short`: five maintained documents modified, two task documents added; pre-existing workspace file untouched.
- `git diff --check`: exit 0, no whitespace errors; repeated after record completion.
- Builder Python local-file-link validator: exit 0; 85 links across seven task files.
- Independent verifier Python local-link/heading-anchor validator: exit 0; 85 links across seven task files.
- Independent verifier `python3 -m unittest discover -s .claude/hooks -p 'test_*.py'`: exit 0; 18 tests passed. No hook files changed; no GitHub CI run claimed.
- Independent review: no Important findings; no implementation/gate claims found. Review is evidence, not owner approval.
- Full/smoke quality scripts absent: Not yet proven, not invoked or reported green. Documentation-only runtime entry point N/A.
- Scrap Sweep: inspected task diff and untracked files; no task temporary probes, debug residue or deletions. Existing user workspace retained; no stashes/branches/state removed.
- Source filenames retain v1.4/v1.2 conventions; new detailed specification is v0.1. Master Build Sequence and Foundation gates unchanged.
- Open editor panel requested for the new specification; app returned queued. Owner authorized commit on 2026-10-02; included in this task commit. No push, dependency installation, publication or deployment performed.
