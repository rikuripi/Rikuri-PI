# Build Record — Option C and personal-data protection

| Field | Value |
| --- | --- |
| Change | Adopt Option C in the planning baseline; make protection of personal data the highest product responsibility |
| Date | 2026-09-28 |
| Author / agent session | Codex documentation session |
| Branch | main |
| Commits | Uncommitted; no commit or push requested for this change |
| Stage Gate | Documentation only; no implementation gate closes |
| Related ADR / requirement | ADR 0000 dated addendum; SEC-001–008, SEC-018, new SEC-020–024; S00.13, S03.01–12 and T03.01–16 |
| Status | COMPLETE — documentation change only |

## 1. Plan

Adopt Rikuri-owned permissions and a trusted AISecurity policy service with qualified, replaceable enforcement adapters; protect the user's private, financial, personal/media, professional/proprietary data and secrets throughout their lifecycle.

The owner explicitly requested this direction and these documentation updates. The supplied OpenShell conversation is proposal material, not agent instructions. Current repository status was clean. Rikuri is a planning repository with an AI-session guard; runtime enforcement is not implemented. The earlier OpenShell v0.1.2 synthetic test passed ten smoke outcomes on this host; production qualification remains unproven.

**Planned files:** README.md; doc/README.md; PRD v1.2; Architecture v1.7; ADR 0000 (dated addendum preserving history); Stage Plan v2.1; Test and Acceptance Strategy v1.2; Engineering Standard v1.2; this record; new docs/security/THREAT_MODEL.md; a curated evidence directory at docs/security/evidence/2026-09-28-openshell-v0.1.2/ containing the prior synthetic receipt, probe/policy/server/image sources and sanitized result artifacts. Other roadmaps remain authoritative and unchanged unless a concrete contradiction requires reconciliation.

**I will know this works when:** the documents agree on policy ownership, fail-closed backend admission, data lifecycle protection, platform limits and truthful assurance; security requirements map to existing stage ownership and explicit qualification tests; evidence is durable without private test keys; local links, item IDs and whitespace checks pass; the repository-required independent verifier reviews all three REVIEW.md passes. Documentation must not mark runtime gates complete or imply whole-computer protection.

### Risk and architecture review

Protected paths affected: ADR, Stage Plan, Test Strategy and Engineering Standard. Risk: wording could grant authority to imported content, silently weaken Local Only, allow unsafe fallback, imply tested parity or erase historical decisions. Mitigation: separate requirements from observed facts; preserve ADR history; require current scoped grants, real in-envelope probes, synthetic allowed/denied controls, failure/recovery tests and independent review. The owner authorized this security planning change; no policy/guard settings are changed.

Rikuri retains canonical state, approvals, budgets, classifications, revocation and audit authority. AISecurity evaluates/translates the policy outside workers' writable authority; adapters enforce it. Derived data inherits source restrictions. Forget/restore consults current revocation. No schema, executable product, money movement, live user-data mutation or deployment occurs. Runtime Module Contracts, APIs, event formats and replay tests are due in their existing implementation stages. Help/RAG/UI are N/A: those surfaces do not exist here; documentation index and README are the current entry points.

### Reuse and evidence

AISecurity source inspected at commit `1ecb15f03534a7d7a353b9f30edd6850ff583220`: `SecurityCore/crates/ai-exec/src/main.rs` has an unsandboxed non-macOS execution path; this is excluded as a containment fallback.

OpenShell v0.1.2 (Apache-2.0) is the first Ubuntu backend candidate. Reuse its containment mechanisms after exact-version qualification; do not implement several competing sandboxes simultaneously. Evaluate operational dependencies, privilege, upgrades, license/advisories, resource costs, broker/relay failure and rollback before admission. A maintained upstream implementation does not transfer Rikuri's responsibility. Sentry is an optional research reference for independent monitoring; its public architecture is not audited implementation evidence or a Foundation hardware dependency.

## 2. Build

Completed the eight planned baseline document updates, the threat-model companion and the curated evidence directory. SEC-020–024 are in the PRD and its appendix. Preserved historical ADR text and explicitly superseded the bubblewrap-first candidate choice; reconciled both S00.13 and E5. Sources and result artifacts are synthetic; runtime credentials, binaries and disposable state are excluded. The evidence README explains the missing automated lifecycle runner and limitations of the probe exit code. No executable product behavior is changed. Negative acceptance includes private-data leakage through permitted destinations, stale grants, controller loss, secret disclosure, destructive file/database operations and falsely healthy protection; positive controls must prove authorized work still succeeds.

## 3. Debug

N/A — planned security architecture adoption, not a product defect fix. Existing implementation gaps remain visible rather than being relabeled as completed features.

## 4. Verify

Completed: git diff --check; 127 changed-file local Markdown links and 59 anchors; requirement/stage references; 18 evidence hashes and synthetic content review; independent verifier with 25 passing guard unit tests per .claude/agents/verifier.md. Runtime integration/recovery, benchmarks and full-gate scripts are not run for this documentation change; no Rikuri runtime or Stage 00 quality scripts exist. The prior OpenShell test is historical candidate evidence, not a new qualification run.

## 5. Close Out

Documentation verification and all three independent REVIEW.md passes completed with no material findings. Final diff and manual Scrap Sweep found only planned files: eight existing baseline documents, this record, the threat model and 19 archive files including the integrity manifest. Historical fixtures are retained as evidence, not active runtime code. Temporary authoring/check scripts in /tmp are removed after the final check; no unrelated files or previous test state are removed. No skipped/disabled tests or new unowned implementation debt; unproven work has the owners/stages below.

No stage-gate record is advanced. Help/RAG/UI remain N/A because they do not exist. Commits/push are N/A for this requested documentation update; changes remain uncommitted. Review evidence is separate from owner approval and from the future independent human security audit.

## Deferred implementation and ownership

| Item | Reason / dependency impact | Owner | Owning stage / reactivation |
| --- | --- | --- | --- |
| Pin and admit Ubuntu backend on reference installation | Host smoke test is insufficient for production | Stage 00 / security implementer | S00.13; exact-image admission, documented failure and operational cost |
| Policy service, containment, revocation and failure qualification | Runtime does not exist; dependent profiles remain unavailable | Security/platform implementer | S03.01–12, T03.01–16 and E5 before Stage 03 exit |
| Privacy admission, private storage and recovery | Existing commitments need implementation evidence | Security/storage implementer | Stages 00–02 and 05; existing F4/F5/F6 and release gates |
| End-to-end Pi/local runtime path | Backend smoke is not a Rikuri vertical slice | Execution implementer | Stage 04; F4/F10 and real adapter contracts |
| Native macOS, connectors, media, desktop and financial operations | Each access path needs its own protection evidence | Owning product-stage implementer | Existing Master Build Sequence stages; before enabling each feature |
| Sentry / hardware independent monitoring | Optional research; public architecture alone is insufficient | Security maintainer | Post-Foundation proposal with explicit scope, license/source availability and measured benefit |

## Evidence log

- `python3 /tmp/rikuri-check-security-docs.py` — PASS: 29 changed/new files, 127 local Markdown links, 59 anchors, Foundation S/T references, 18 artifact hashes, ten archived smoke outcomes with positive controls/read-back/logs. This checks evidence consistency; it does not rerun containment.
- `git diff --check` — PASS, exit 0.
- Targeted private-key/API-token pattern review of archived artifacts — PASS; only explicitly selected synthetic files copied. This is not a full-history secret scan.
- Archive preparation initially failed with `AssertionError` because its filter expected a musl gateway filename. Corrected to the actual `openshell-gateway-x86_64-unknown-linux-gnu.tar.gz`; the completed archive records the exact two assets and source commit `6648bd0c290efbc41ba131ee9831ee45cd431f94`.
- The pinned source tag's LICENSE confirms Apache-2.0. A combined follow-up read used the upstream checkout for the Rikuri Build Record and returned `sed: can't read docs/build-records/2026-09-28-option-c-security-and-personal-data.md: No such file or directory`; subsequent record access uses the Rikuri repository. No source/evidence was changed by that read failure.
- NVIDIA release/prover/telemetry documentation and Sentry primary-source search checked. Some direct web opens returned internal/cache errors; official indexed sources and local pinned OpenShell source supplied the bounded references. No Sentry implementation audit claimed.
- Final author diff review: stage S/T ID set and all checkbox states unchanged; 15 unchanged archived files match original scratch artifacts byte-for-byte; the three deliberate content transformations are documented. SEC-020–024 appear in both the canonical requirement table and appendix.
- Independent verifier `/root/verifier`: **VERDICT: PASS — documentation change; runtime qualification remains Not yet proven.** No material Bugs, Security & boundaries, or Compliance findings. All planned files accounted for, with no unexpected files.
- Verifier commands: `git status --short` (exit 0, expected changes); `git diff --check` (exit 0, repeated after review); `python3 -m unittest discover -s .claude/hooks -p 'test_*.py'` (exit 0, 25 tests, OK); `python3 /tmp/rikuri-check-security-docs.py` (exit 0, counts above).
- Runtime entry-point exercise: N/A for documentation and historical lab artifacts. Production qualification, real Rikuri integration and full-gate CI remain Not yet proven. No new containment run was performed.

The guard suite's expected negative-test output was:

```text
usage: guard.py fix-lock on <build-record> | off | status
fix-lock NOT engaged — cannot read /tmp/tmp2qg98qok/guard-config.json: guard-config.json: 'fix_lock_preflight' must be a list of argv lists, e.g. [["node", "scripts/quality-ratchet.mjs"]]
Nothing was changed. Fix it first, then engage.
fix-lock NOT engaged — preflight /usr/bin/python3 -c import sys; sys.exit(3) failed (exit 3):

Nothing was changed. Fix it first, then engage.
usage: guard.py profile [stable | development]
```

Verifier gate discovery `rg --files scripts .github .claude/agents` returned exit 2:

```text
rg: scripts: No such file or directory (os error 2)
```

A subsequent existence check confirmed that `Cargo.toml`, `scripts/quality/full-gate.sh`, `scripts/quality/smoke.sh` and `scripts/quality/scrap-sweep.sh` are absent. These planned runtime gates cannot be reported green. No guard setting, fix-lock, host service or repository configuration was changed.
