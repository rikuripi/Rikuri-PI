# Offline dependency freshness policy

## Task Contract
- Outcome: Run every existing dependency check offline against verified cache inputs refreshed within 24 hours.
- Scope: High-risk control task, S00.01/S00.03. Add scripts/quality/dependency-check.py, its tests/self-test entry, integrate full-gate.sh, add maximum advisory staleness to deny.toml and usage to README.md. Preserve default online/CI behavior, existing deny rules, source/history and OS controls. No Orchard changes.
- Done when: Legitimate cache accepted; absent, incomplete, stale, future-dated, altered or misbound inputs refused before cargo-deny; default command unchanged; actual offline cargo-deny exercised and full Ubuntu gate attempted once after readiness, stop first red; independent verifier. Full green required for overall DONE, not assumed from helper tests.
- Authority: Owner explicitly approved implementation of 24-hour freshness policy, refusal tests and Ubuntu rerun. Reuse admitted disposable namespace/chroot layer, pinned tools and named official download origins. Quality gate modification is this task, not a convenience bypass. No user credentials, live node, global controls, commit/push/PR or CI edits. One independent verifier required under AGENTS.md §5.
- Limits: One task, at most two repairs and two review rounds. Cache fetch <=600 seconds; gate <=1800 seconds/two jobs. Stop on first gate red; later steps unproven. Original index untouched. No third-party code execution outside admitted layer.

## Plan and risk note
The prior red run is recorded in ../build-records/2026-10-14-offline-full-gate.md and chat outputs/offline-full-gate/full-gate.log: cargo-deny0.20.2 fetches advisories unless its explicit CLI offline option is present. This is the reproduced defect. No test baseline or deny-rule relaxation is needed.

Keep online mode as cargo deny --locked check. Explicit offline mode requires both cache-manifest path and a separately trusted SHA256 pin from the preparation runner; any partial mode fails closed. Bind receipt to Cargo.lock/deny.toml, fixed public origins, exact advisory commit, per-file hashes/sizes and separate genuine fetch timestamps for advisory and registry inputs. Reject missing/extra/unsafe cache files and timestamps outside [now-86400, now]. Verify locked crate archives and cached registry entries, keep yank checking and all check categories enabled. cargo-deny also enforces maximum-db-staleness P1D. Offline execution uses cargo deny --locked --offline check; propagate its status with no fallback. Manifest integrity is relative to the trusted preparation runner/pin, not cryptographic upstream attestation or defense against a malicious shell owner. Prepare in an empty cache, fetch before recording time; never touch timestamps to make old data fresh. Reuse root-verified imports; keep selected input trees read-only during gate execution. Record before/after OS controls.

Targeted tests run under admitted Ubuntu containment, including the real wrapper with synthetic command capture to verify exact command/status and refusal-before-execution. Full gate exercises actual pinned cargo-deny. Review only after evidence exists. Integration overlay has no project commit; HEAD-only history scan cannot prove overlay secret coverage; no CI/merge/stage claim.

## Preparation repair1
Initial disposable-clone index preparation exited128: unprivileged Git could not traverse the intentionally private root700 stage. No admission/test ran. Keep root700; use trusted root Git solely for fixed delta staging, with global/system config and hooks disabled, then restore disposable index ownership to UID60000. Global controls remained1. Counts repair1; at most one repair remains.

## Readiness
Contained targeted run exit0: all10 test methods pass, including deterministic freshness boundaries (0/86399 accepted;86400/86401/future refused), origin/binding/inventory/checksum/path refusals and real Python entrypoint command/status propagation. Full admission and read-only cache-tree probes passed. Fresh empty-cache fetch verified101 Cargo.lock archives and official advisory commit edb58f5974b83eb8302332821cb6d389777aa351. Receipt records conservative actual fetch-start timestamps, not import/seal times. Full gate next, once; later checks only proven if reached. No Rust changes, so Rust mutation testing N/A.

## Gate red / repair2 (final)
First full run passed tools, selftests,107guard,format,build,bothClippy,245workspace tests,smoke,7crash tests. Dependency wrapper refused before cargo-deny: advisory database content missing. Builder mistakenly required .toml advisory files; official fetched snapshot has crates/<crate>/RUSTSEC-*.md (embedded TOML). Correct required suffix to .md and correct synthetic fixture format; add explicit missing-advisory-content refusal. This is not removal of an existence/integrity check or advisory ignore. Fixture correction follows actual official format, not a weakened baseline. Remove one redundant overwritten fixture initialization as a nit. Red log/receipt retained as first-gate-format-failure.log/first-gate-format-execution.json. Repair2 consumed: no further repairs. Repeat contained targeted tests then full gate once, stop first red.

## Final verification
Corrected targeted run all11 tests passed; final full gate exit0 / RESULT:PASS(15steps), including actual cargo-deny advisories/bans/licenses/sources OK,107guard,245workspace and7crash tests. Existing duplicate-version/unused-license warnings not relaxed. Receipts retain equal global_before/global_after1. All current implementation hashes matched tested source delta before audit closeout. No CI URL: no project commit/PR/CI run. HEAD173-commit secret history scan excludes uncommitted integration overlay. One identical Parallels transport retry (255 before helper output) required no repair. Two repairs exhausted. Independent review pending. Evidence in chat outputs/offline-advisory-policy/verification.md and manifests/logs. No further tests/code changes without a new scoped task.

## Closeout
Outcome: explicit offline dependency policy implemented with approved24-hour freshness, no online/CI behavior change.
Evidence: full Ubuntu15step gate PASS;11targeted/107guard/245workspace/7crash tests; independent verifier PASS with no Important findings or nits, including1573cachefiles/101checksums/source/control/log integrity.
Safety: admitted namespace layer, immutable verified input trees, global controls unchanged; no node/credentials/project commit/push/PR.
Status: DONE for this bounded local offline policy task. CI/merge/stage readiness remains unproven; HEAD-only history coverage limitation retained.
Next: prepare combined integration change set for final review, not started.
