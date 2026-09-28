# Build Record — Context delivery and model continuity

| Field | Value |
|---|---|
| Change | Specify timely context delivery, mandatory knowledge discovery on model/session transitions, and retrieval on topic changes or crossings |
| Date | 2026-09-27 |
| Author | Codex |
| Branch | main; no staging, commit or push requested |
| Status | COMPLETE — documentation verified; runtime remains Not yet proven; review is not owner approval |
| Requirements | PRD MEM-001–006, KNOW-001–007, PROJ-002/003, UI-010; new MEM-007–012 |
| Stage gate | No gate status or sequence changed; existing Foundation contracts refined, later connector/media capabilities retain product stages |

## 1. Plan and characterization

Outcome: make knowledge availability an explicit, testable runtime responsibility so a new model and a changed or intersecting topic receive current, scoped context without requiring the user to remind the agent that memory exists.

The owner authorized the preceding proposal to specify update expectations, task packets, refresh triggers, media provenance, coverage disclosure and acceptance evidence, and explicitly added model-switch discovery and immediate topic-triggered search. Read git status, documentation index, Architecture §19–23, PRD §9.2/9.3, the live Stage Plan, Engineering Template A, Test Strategy, REVIEW.md and the verifier instructions before editing. Tree contains documents and the guard/tests/CI configuration, not an application or Stage 00 quality scripts. All Foundation gates remain NOT STARTED. Initial status: only unrelated untracked `Rikuri Pi.code-workspace`; preserve it.

Observed gaps: the compiler/Knowledge Service split and handoff IDs exist in the design, but model bootstrap discovery, topic-trigger dispatch, coverage states, invalidation, source update contracts, measurable latency and delivery receipts are underspecified. Architecture §20.1 also labels Claude/Git as Foundation imports contrary to the controlling live plan; align that paragraph with the existing accepted sequence.

Planned files: Architecture v1.7 §19/§20, PRD v1.2 §9.2 and requirement index, Test Strategy v1.2 context/retrieval guidance, doc/README.md, and this record. Keep the Stage Plan, ADR history, historical records, runtime/guard code and other files unchanged.

I will know this works when the documents require a trusted knowledge capability manifest and actual tool wiring at new-session/model-switch/resume/compaction boundaries; immediate bounded retrieval on changed/crossed topics; a policy-filtered task packet with freshness/coverage/conflict status; invalidation and safe failure; and explicit deterministic acceptance cases plus performance measurements. Local links/anchors, requirement IDs, S/T references, source versions and stage scope must validate, with independent review under all three REVIEW.md passes.

## 2. Risk, caller and wiring

Documentation only. Intended caller: Rikuri task/turn lifecycle → policy admission → Knowledge Service retrieval → Context Compiler → destination harness/model → scoped further retrieval. The model does not have to volunteer the first search. A connector's presence, an agent's acknowledgement, or a prompt instruction alone cannot prove this path is working.

Protected-path risk note: the Test and Acceptance Strategy is protected. The owner's requested acceptance-contract work authorizes this scoped clarification. Preserve its existing gates and ten-case compiler requirement; distinguish existing Foundation fixture refinements from later product acceptance examples and provisional performance experiments. No guard change, weakened gate, scope resequencing or implementation claim. No new authority owner or ADR invariant; apply the existing source, policy and scope decisions.

Trust review must cover untrusted text masquerading as bootstrap instructions; knowledge-space names/query text/metadata leaking across privacy boundaries; permission and deletion changes invalidating caches and pending results; cross-topic retrieval expanding access; stale code-session narratives; model output falsely claiming successful search; retries bypassing budget; summaries losing unresolved work; and unavailable sources being reported as empty. Source originals remain authoritative as already declared; packets, transcripts, indexes and inferred links remain derived evidence. Canonical writes and externally consequential actions retain existing policy. There is no live data migration, connector installation, external collection, spend or deployment.

Foundation references: S01.03/S01.07/S01.09 and T01.06/T01.07; S02.02 and T02.01/T02.15/T02.16; S04.01/S04.02/S04.14/S04.15 and T04.01/T04.06/T04.13/T04.16–18. Stage 00 and the preceding gates remain prerequisites. Post-Foundation source expansion follows product Stages 1, 5 and 8; multi-agent integration follows Stage 6 and autonomous reflection remains Stage 10 research/productization under its prerequisites.

Existing-solution assessment: the preceding read-only research reviewed Glean's indexed/live/hybrid access, LangGraph's thread/cross-thread memory, Anthropic's progressive retrieval, Graphiti's temporal retrieval and TwelveLabs' media addressing. Borrow design patterns; no library/vendor is selected or adopted here. Pinned version/license, locality, lifecycle, integration cost and adapter failure evidence are required at the existing component-admission stage, not claimed by this design.

## 3. Build and debug disposition

Design clarification, not an executable defect fix. Failing-test/fix-lock workflow, runtime Module Contracts, migrations, unit/property/replay execution for new behavior, recovery drills and runtime performance receipts: N/A to this documentation change; future cases stay Not yet proven. The small import-scope wording correction follows the existing live-plan authority. Documents are already large specifications; keep this contract in the owning section with cross-links rather than creating a second roadmap.

## 4. Verification

- PASS: inline Python Markdown path/heading validator checked 33 local links/anchors across all five task files (exit 0); MEM-007–012 each has one definition and matching appendix entry. Foundation S/T references resolve in the live plan.
- PASS: `git diff --check` (exit 0), `git diff --numstat`, final tracked/untracked inspection and source-version/stage-scope review. `git diff --exit-code -- AGENTS.md REVIEW.md .claude .github doc/Rikuri_Pi_Foundation0_Rebaselined_Stage_Plan_v2.1.md doc/Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md` returned 0: policy, live gates, accepted ADR and guard/CI unchanged.
- PASS: independent verifier `/root/verifier` followed `.claude/agents/verifier.md` and all three REVIEW.md passes. Bugs / Security & boundaries / Compliance: no unresolved findings. One Bugs Nit was repaired by explicitly testing removal of the trusted bootstrap and a substantive turn incorrectly classified unchanged while the model never requests memory; verifier inspected both repairs and reran whitespace checks. Final verdict: PASS.
- PASS: verifier ran `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s .claude/hooks -p 'test_*.py'` (exit 0, 16 tests, `OK`). Expected fixture output refused malformed preflight configuration and a deliberately failing preflight command; these were not task failures and no real fix-lock was engaged. Independently confirmed 33 links/anchors, six definition/index pairs, 21 S/T references including expanded ranges, source versions and unchanged protected policy/gate/ADR/guard files.
- Not yet proven: runtime context delivery, adapter readiness, topic detection/retrieval, timing/coverage metrics, full-gate/smoke and GitHub CI. No runtime exists and no CI run is observed in this task.

## 5. Close-out and deferred work

Future implementation owner: Henry/product owner and the implementer of each owning stage. Existing Foundation fixture refinements activate when their named paths are implemented; connector/media/cross-topic intelligence cases activate in their mapped product stages after predecessor gates. Provisional latency targets require a declared benchmark environment and measured evidence before promotion to an implementation performance gate. No downstream stage is advanced by this document.

Help/runbooks/RAG surfaces: N/A as executable surfaces do not exist; documentation index and owning requirements/architecture/test reference are updated together. Stage Gate Record: N/A, no gate state changed. Commit/push: not requested. Scrap Sweep: inspected tracked edits, untracked task files and repository file inventory; no task-created temporary files, obsolete implementation or debug residue. No deletion needed; unrelated workspace file preserved. Design delivered with verified documentation and explicit unproven runtime/benchmark dispositions; no unowned skipped implementation gate.

## 6. Publication authorization and remote blocker

On 2026-09-27 the owner requested “Ok, commit. Pull first then push.” Earlier “not requested” statements describe the state before this authorization. Publish this documentation together with the related scoped-autonomy design and its Build Record; exclude the unrelated `Rikuri Pi.code-workspace` file.

Remote synchronization was attempted before committing. SSH fetch/pull failed with `Permission denied (publickey)` using the configured identity and the other existing Rikuri Pi identities; HTTPS with the existing GitHub CLI login and the GitHub connector both returned repository-not-found responses. No pull or push succeeded, and saved authentication, SSH configuration and Git remote settings were not changed. The local commit preserves the reviewed work; its containing Git revision is the publication record. Owner action: restore GitHub access to `rikuripi/Rikuri-PI`, then pull, reconcile any incoming changes and push without force. Remote publication and CI remain unproven.
