# Build Record — Scoped autonomy and repair cycles

| Field | Value |
|---|---|
| Change | Document consequence-based autonomy presets, unattended readiness, approval-aware progress and defect-scoped regression evidence |
| Date | 2026-09-27 |
| Author | Codex |
| Branch | main; no staging, commit or push requested |
| Status | COMPLETE — documentation verified; future runtime behavior remains Not yet proven |
| Requirements | SEC-002/003/004/008/010, MEM-007/009, new SEC-013–019 |
| Stage gate | None changed; Foundation remains the existing security/budget/envelope/approval substrate |

## 1. Plan and characterization

Outcome: specify how users authorize routine development once within a bounded environment, keep useful independent work moving while approvals wait, and complete verified defect cycles without recurring human lock-release prompts once a separately admitted policy supports it.

The owner explicitly requested documentation and deferred work on their current project. The mempool.api/live-node experience is an illustrative user report, not an inspected deployment or permission grant. Do not access that project or any node, install anything, alter active approvals, or engage/release any lock.

Read git status, documentation index, Engineering Template A, PRD security requirements, Architecture §16/§17/§27, live Stage Plan Stage 02–04, Master Build Sequence Stages 1/2/6, REVIEW.md and verifier instructions before editing. Existing requirements already distinguish consequence from tool syntax, bound capabilities and money, and require explicit project trust. Missing details are novice-facing autonomy presets, unattended preflight, partial progress during an approval wait, and a per-defect evidence lifecycle. Current AGENTS.md explicitly makes fix-lock release owner-only; the shared guard and report-only verifier cannot implement the proposed automatic cycle completion today.

Preserve all preceding context-delivery work: four modified documents and `docs/build-records/2026-09-27-context-delivery-and-model-continuity.md`, plus unrelated `Rikuri Pi.code-workspace`. A temporary baseline snapshot of 13 affected/protected files was captured at `/private/tmp/rikuri-autonomy-baseline-2okj9s2o` for per-turn comparison; it is not a runtime artifact. Planned edits are additive only: Architecture v1.7 §16.5–16.9, PRD v1.2 security requirements/index, Test Strategy v1.2 future acceptance cases, doc/README.md, and this new record.

I will know this works when the docs distinguish authorization from containment and quality verification; offer Inspect / Isolated development / Staging / Live maintenance plus a separate unattended option; define scoped/versioned/expiring grants and understandable block reasons; preserve live-system and financial boundaries independently; describe safe candidate work while activation waits; and define immutable per-defect regression evidence, independent verification and trusted-controller completion without giving today's agents release authority. Acceptance must test both uninterrupted legitimate development and meaningful refusal, with stage mapping and unproven implementation status.

## 2. Risk, caller and wiring

Proposed caller: user autonomy settings/task grant → trusted policy/capability validation → tested execution envelope → task lifecycle/approval queue → independent verification evidence → trusted repair controller or separate activation gate. Reuse the existing authority, registry, ledger and budget boundaries; no model or editable repository file becomes a new authority source.

Protected-path risk note: the Test Strategy is protected. The owner's explicit documentation request authorizes adding prospective acceptance cases; existing acceptance gates and current fix-lock behavior remain unchanged. AGENTS.md, REVIEW.md, .claude, CI, Engineering Standard/Templates, live Stage Plan and ADR history are outside this edit. The future alternative to owner-only release requires a separately approved policy/ADR reconciliation and implementation evidence before activation, including consistency with the Engineering Standard and shared-guard lifecycle. This design does not perform that reconciliation or supersede current instructions.

Threats to cover: an empty-wallet label granting broader authority; sandbox access to live mounts/credentials/services; query/resource exhaustion; policy or verifier edits activating their own new rules; stale approval of a changed candidate; executable-metadata trust invalidation being skipped; deletion of user/other-session data; retries/model swaps reviving expired grants; green tests obtained by changing the runner/fixtures/discovery rather than behavior; incorrect original tests with no amendment route; dependent work crossing an unresolved approval/red gate; silent overnight stalls and unbounded idle waiting.

Authority/canonical data/privacy/deletion/budget: no live change. Future grants and audit records remain governed canonical objects; source/policy references retain privacy and deletion rules. Application changes are recoverable candidates until separately authorized promotion. Financial authorization, Local Only, secrets, hard budgets, descendant containment and no-self-approval remain intact. No runtime schema migration or executable code change.

Foundation references are context only: S02.01/S02.05/S02.08, T02.05–07/T02.11; S03.04–09, T03.09–13; S04.14/S04.15, T04.13/T04.16; S05.04/T05.04. New development-policy UI/repair lifecycle is prospective product Stage 1 after Foundation and policy reconciliation; richer workspace UI follows Stage 2; durable branching/waits and multi-agent governance follow Stage 6. No new Foundation service or automatic lock release is promised.

Existing-solution review: prior read-only research considered official Claude Code sandbox and VS Code approval documentation, which distinguish approval decisions from execution containment. This change adopts no component. Pin/license/trust/lifecycle/integration cost and fault evidence belong to later implementation admission; command-name matching remains a convenience/tripwire rather than the enforcement boundary.

## 3. Build and debugging disposition

Documentation-only future design, not a fix to the existing guard or the user's live-node project. Runtime unit/property/replay tests for the new behavior, deployments, recovery drills, Module Contracts, schema migration, test-lock operations and before/after running-system probes are N/A in this change. Future acceptance scenarios remain Not yet proven. Keep the contracts in the existing owning documents rather than introducing a second procedure or roadmap.

## 4. Verification

- PASS: inline Python validator resolved 45 local Markdown links/anchors across the five task files (exit 0), matched all seven SEC-013–019 definition/index ID-priority-title pairs, and found no unknown Foundation S/T references. Source version filenames retained; new contracts explicitly use product Stages 1/2/6.
- PASS: per-turn baseline comparison found only additions in the four preexisting documents (+2 index, +68 Architecture, +16 PRD, +27 Test Strategy lines), preserving previous context-delivery text. Nine protected/prior files were byte-identical, including the earlier Build Record, AGENTS/REVIEW, guard/config/tests/verifier, live Stage Plan and accepted ADR. `git diff --check` returned 0; status shows only planned docs/records plus the preserved unrelated workspace file.
- PASS: independent verifier `/root/context_plan_audit` followed `.claude/agents/verifier.md` and REVIEW.md. Bugs / Security & boundaries / Compliance all passed with no Important findings or nits. Verified 45 local links/anchors, seven new requirement definition/index pairs, 42 unique Foundation S/T references including expanded ranges, source versions, additive-only edits and nine unchanged protected/prior files. The PRD readiness label was aligned to Architecture's “Ready with limits” before the final pass.
- PASS: verifier ran `python3 -m unittest discover -s .claude/hooks -p 'test_*.py'` (exit 0, 16 tests, `OK`). Expected failing-preflight/lock messages were confined to temporary test fixtures, not the repository's real lock. No failing checks; `git diff --check` was independently green. Final verdict: PASS; independent review is evidence, not owner approval to activate new policy.
- Not yet proven: runtime autonomy, isolation/preflight, automatic repair-cycle completion, unattended progress, approval UX/usability and performance. Stage 00 full-gate/smoke are absent; GitHub CI not observed.

## 5. Close-out and owned deferrals

Owner: Henry/product owner and implementer of each owning stage. Activate new development grants/repair policy only after Foundation assurance, explicit policy reconciliation and the listed allowed/denied/failure cases; workflow branching/wait recovery remains product Stage 6. These are future product requirements, not waived current gates. User's current mempool.api project is explicitly outside scope and remains for a later task.

Help/RAG/runbook implementations: N/A, no application exists; index/PRD/Architecture/Test Strategy updated together. Stage Gate Record: N/A, no status changed. Commit/push: not requested. Scrap Sweep inspected tracked edits, untracked records and repository source inventory; no task-created source probes, obsolete implementation or debug residue required removal. Temporary baseline snapshots remain outside the repository as review evidence. Previous context-delivery changes/record and unrelated workspace file preserved. Documentation is complete; activation and external-project work remain explicitly deferred, with no current permission changes.

## 6. Publication authorization and remote blocker

On 2026-09-27 the owner requested “Ok, commit. Pull first then push.” Earlier “not requested” statements describe the state before this authorization. Publish this documentation together with the related context-delivery design and its Build Record; exclude the unrelated `Rikuri Pi.code-workspace` file.

Remote synchronization was attempted before committing. SSH fetch/pull failed with `Permission denied (publickey)` using the configured identity and the other existing Rikuri Pi identities; HTTPS with the existing GitHub CLI login and the GitHub connector both returned repository-not-found responses. No pull or push succeeded, and saved authentication, SSH configuration and Git remote settings were not changed. The local commit preserves the reviewed work; its containing Git revision is the publication record. Owner action: restore GitHub access to `rikuripi/Rikuri-PI`, then pull, reconcile any incoming changes and push without force. Remote publication and CI remain unproven.
