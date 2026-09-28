# Build Record — Knowledge capture, document history and team conversations

| Field | Value |
|---|---|
| Change | Specify preserved document revisions and projection edits, low-friction capture and conversational retrieval, and secure human–AI project conversations |
| Date | 2026-09-28 |
| Author | Codex |
| Branch | main; no staging, commit or push requested |
| Status | COMPLETE — documentation verified; runtime Not yet proven; independent review is evidence, not owner approval |
| Requirements | KNOW-001–007, MEM-007–012, LINK-001–005, attention/governance requirements; new KNOW-008–011 and COLLAB-001–006 |
| Stage gate | Foundation S01.03/S01.05/S01.08 preservation clarification; future product Stages 1/2/3/5/6/8/9/10; no gate closed |

## 1. Plan and characterization

Outcome: make the owner's adopted document-history behavior and the agreed capture/conversation design concrete in the owning planning documents, with stage dependencies, security boundaries and acceptance evidence.

The owner approved the two one-way Markdown flows, preservation and flagging of edited projections before regeneration, and inclusion of the preceding knowledge-capture and LAN/remote human–AI collaboration proposals in planning. This authorizes the corresponding protected Stage Plan, Test Strategy and dated ADR addendum edits; no further approval is required for this scoped documentation work. Read git status, AGENTS, guard policy, documentation index, affected plan sections, Template A, REVIEW and verifier instructions. Initial status contains only the unrelated untracked `Rikuri Pi.code-workspace`, which is preserved. The repository has planning documents and guard infrastructure, no application or executable quality scripts. An inventory lookup found no `.claude/state` directory; no fix-lock state was changed.

Observed gaps: PRD KNOW-005 and the bidirectional-editing open question conflict with Architecture §20.3 and S01.08; edit detection does not specify durable preservation before replacement. Classification and context retrieval already exist as designs, but capture feedback and audience-aware conversational contributions are incomplete. Link describes owner-to-node access with Tailscale first and native encrypted direct/relay later; minimal multi-human membership/sharing is deferred with enterprise Stage 9 and has no shared-conversation gate.

I will know this works when the PRD, Architecture, live Stage Plan, Master Build Sequence, Test Strategy, dated ADR clarification and documentation index agree on the two flows and recoverable revision handling; automatic classification cannot promote authority; retrieval contributions respect audience, freshness and attention; LAN and remote clients share authenticated application permissions; and a small team slice has explicit prerequisites and allowed/denied/recovery acceptance. Local links/anchors, requirement definitions/index entries, S/T references, source versions and final diff must pass review by the independent verifier.

Planned files: this record; `doc/README.md`; PRD v1.2; Architecture v1.7; Master Build Sequence v1.1; Foundation Stage Plan v2.1; Test and Acceptance Strategy v1.2; ADR 0000 (append dated clarification, preserve prior decisions). No alternate roadmap document.

## 2. Risk, authority and wiring

Documentation-only change with security, recovery and future scope implications. Protected-path risk: Stage Plan and Test Strategy refine existing S01.08 preservation evidence and add explicitly post-Foundation cases; the ADR records owner-approved minimum collaboration resequencing from product Stage 9 to Stage 6. Foundation gates, numbering, six-stage scope, ten compiler cases, no-network Local Only promise, owner-only fix-lock release and current guard remain intact. Stage 6 depends on Stage 3 transport and the staged membership/authorization work before multi-human activation. Stage 9 retains enterprise administration and Windows work.

Future callers: capture adapter → governed source/artifact service → derived extraction/classification → Knowledge Service/Context Compiler → evidence card; conversation event → authorized retrieval → audience/attention checks → governed post. Projection generation → detect local edits → validate/preserve/read back → flag/block or regenerate safely; canonical changes use a separate authorized candidate-promotion path. Link carries authenticated requests; connectivity does not confer project authority or shell access.

Threats and acceptance: overwrite/crash/concurrent edits lose user work; recovery copies bypass forgetting; a generated summary becomes an accepted decision; restricted titles/thumbnails leak to a room; a new or removed member receives unauthorized history or queued results; network fallback weakens identity/encryption; retries duplicate posts/actions; retrieved instructions induce terminal execution. Positive paths must work alongside denial cases. No actual data, network, dependency, deployment, money or live service is changed.

Component assessment: mymind-style capture, Notion-style shared objects and Desktop Commander-style filesystem/terminal connectivity are reference behaviors, not selected dependencies. Existing Knowledge Service, projection, Link, workflow and policy services retain ownership. Evaluate maintained components/protocols before custom implementation, with pinned version/license, trust boundary, lifecycle/integration cost and real failure evidence at the owning stage. No package or cryptographic protocol is selected by this documentation change.

## 3. Build / debug disposition

Planning clarification and consistency repair, no executable defect fix. Runtime unit/property/replay tests, fix-lock workflow, Module Contracts, migrations, performance measurements and live vertical-slice execution: N/A to this change. Planned acceptance cases remain Not yet proven. Source version filenames remain the maintained baseline.

## 4. Verification

- PASS: inline Python validator (exit 0) checked all eight task files, 70 local Markdown paths/anchors, ten new requirement definition/index pairs, 162 Foundation S/T references and unique S6.C1–C4 definitions. The Foundation gate board exactly matches HEAD; all prior ADR 0000 content is preserved as a prefix before the dated addendum.
- PASS: `git diff --check` (exit 0); final diff/numstat, tracked/untracked and relevant directory inventory inspected. `git diff --exit-code -- AGENTS.md REVIEW.md .claude .github doc/Rikuri_Pi_Engineering_Standard_v1.2.md doc/Rikuri_Pi_Engineering_Templates_v1.1.md` returned 0. No enforcement or existing guard/CI changes.
- Review corrections: the independent verifier identified a missing Stage 3 → Stage 6 edge and obsolete aggregate schedule; both were corrected, with the former estimate superseded pending owner/implementer re-estimation. The adjacent current Master version reference and PRD sibling browsing/editing/synchronized-projection wording were reconciled. This is documentation consistency work; no executable regression test or fix-lock was applicable.
- PASS: independent verifier `/root/verifier` followed `.claude/agents/verifier.md` and all three REVIEW passes; final verdict PASS, no unresolved findings. It inspected the dependency/schedule and PRD sibling repairs, independently confirmed 70 local links (42 anchors), ten requirement definition/index pairs, 162 unique Foundation S/T references, unique S6.C1–C4 definitions, exact prior ADR history and all 13 unchanged NOT STARTED gate rows.
- PASS: verifier ran `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s .claude/hooks -p 'test_*.py'` (exit 0, 18 tests, `OK`). Negative fixtures intentionally emitted refusal text including `Nothing was changed. Fix it first, then engage.`; no failing test or live guard-state change occurred. This proves the existing local guard suite only, not any planned application behavior or GitHub CI.
- Not yet proven: all projection preservation, classification, contribution, team/network and recovery runtime behavior, performance/usefulness metrics, full-gate/smoke and GitHub CI. No application runtime or quality scripts exist; no CI run was observed. Local documentation checks cannot close those claims.

## 5. Close-out and dispositions

Implementation owner: product owner and implementer of each owning stage. Foundation preservation activates with S01.08/T01.16; source-card/review UI Stage 2, capture and personal contributions Stage 5, minimal human–AI collaboration Stage 6 after transport/security/governance prerequisites, richer media Stage 8, enterprise controls Stage 9, discovery research Stage 10 under existing Stage 4/6 prerequisites. Stage 1 text classification/semantic experiments require measured gaps and existing evaluation floors. Stage 6's updated scope and aggregate delivery range must be re-estimated before scheduling that stage. No runtime gate is advanced.

Help/RAG: owning docs and index updated; executable help/runbooks are N/A because none exist. Stage Gate Record: N/A, no implementation evidence or gate status changes. Commit/push: not requested. Scrap Sweep: inspected all eight task files and relevant directory inventory; no temporary scripts, debug residue, replaced executable code or proven orphans to delete. The unrelated untracked workspace file remains intact. No unowned skipped implementation gate; future cases carry owning stages and activation conditions above. Documentation delivered with independent review and explicit unproven runtime dispositions.

## 6. Publication authorization

The owner subsequently requested “Let's commit and push.” Earlier “not requested” entries describe the state before that authorization. Publish these eight reviewed task files on the existing main branch; exclude `Rikuri Pi.code-workspace`.

The saved SSH remote rejected authentication (`Permission denied (publickey)`). The existing GitHub CLI account has repository pull/push access; a direct HTTPS fetch using its credential helper succeeded without changing saved remotes or credentials. `git rev-list --left-right --count HEAD...origin/main` returned `0 0` before this commit, so no incoming changes required reconciliation. Whitespace checks passed again. The containing Git revision records the local commit; push confirmation is reported after the remote operation, and no CI run is claimed by this publication note.
