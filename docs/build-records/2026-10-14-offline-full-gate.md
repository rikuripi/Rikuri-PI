# Offline full Ubuntu gate run

## Task Contract
- Outcome: Populate verified offline cache inputs and run the unchanged full gate once inside the admitted full-project sandbox.
- Scope: High-risk cache preparation/fixed gate entrypoint in owned disposable workspace; no repository gate/guard/CI edits, original index/history changes or security-check skipping. Source snapshot remains HEAD13b58bd plus recorded integration overlay.
- Done when: Locked crates and official advisory snapshot verified/provenanced, admitted boundary rerun before fixed full-gate entry, exact gate output/exit retained, independent review. Any failure reported red with precise remaining scope, never a merge/stage pass for overlay-only history coverage.
- Authority: Owner Next approved offline cache population/full-gate run; prior named official HTTPS download origins persist. Empty temporary HOME/CARGO_HOME for fetch-only prefetch, no user credentials/build scripts. Root private staging, UID60000, zero caps, private loopback/external-network denial preserved. No project commits/push/PR/merge; scratch Git commits made by existing gate self-tests are disposable test behavior.
- Limits: One task; two repair/review cycles maximum. Cargo fetch600seconds, gate1800seconds (matches CI),2build jobs; stop on gate first failure. No code/control modifications to make green. One verifier required.

## Plan and evidence boundaries
Reuse exact pinned binaries and admission checks. Prefetch only locked crates with verified source/checksums and public official advisory revision; all compiler/project execution remains offline inside sandbox. Add only fixed gate entry after successful admission, with no arbitrary command argument. Existing full-gate.sh unchanged. Isolated clone index includes overlay for tracked-file checks; HEAD history secret scan excludes new uncommitted overlay contents and cannot prove merge-candidate coverage.

Pinned cargo-deny source inspection shows its offline flag is a CLI option, not CARGO_NET_OFFLINE. Preserve unchanged gate command and actual no-network boundary; do not inject flags/skip fetching or suppress dependency failures. Record any resulting offline incompatibility truthfully.

## Red gate run / repair1
First actual unchanged gate ran after successful admission and stopped at tools: awk command not found, resulting parsed cargo-deny versionempty and RESULT:FAIL(tools), exit1. Pinned cargo-deny itself had independently reported0.20.2. Red log/receipt retained in outputs/offline-full-gate/first-tools-failure.log and first-tools-execution.json.
Minimal root omitted public/etc/alternatives links behind usr/bin/awk and compiler aliases. Relevant environment repair1 creates only known alias symlinks whose resolved executable stays under/usr; no host/etc bind, gate/pin/check edit or override. Admission now smoke-checks awk/cc/c++ before rerunning unchanged gate.

## Red admission retry / repair2 (final)
Retry stopped before gate entry because newly added C++ smoke required an absent executable. Guest provides awk and cc; inspected native dependencies use C. Remove only this extraneous new prerequisite, retain awk/cc and all original containment/tool pin checks, and explicitly report C++ unavailable. Gate script remains unchanged; any actual C++ requirement must fail normally. Admission failure evidence retained in runtime-preflight-failure.log and runtime-preflight-execution.json. This consumes repair2; stop on any further failure.

## Final bounded result
Admission and tools/selftests/107 guard tests/format/build/both Clippy configurations/245 workspace tests/smoke/7 crash-matrix tests passed. Unchanged gate stopped exit1 at dependencies: cargo-deny attempted advisory Git refresh, blocked DNS/network. Later secrets/ratchet/contracts/crate-policy/scrap unrun. Wrapper confirmed both global protections remain1. Two repairs consumed; no further repair/retry. Complete evidence: chat outputs/offline-full-gate/verification.md, full-gate.log and execution-receipt.json. Independent final review pending. No fullgate/CI/merge readiness claim.

Independent final review PASS, no Important findings (Bugs, Security & boundaries, Compliance). Hashes and test total independently verified. Gate remains BLOCKED at dependencies; stop current task. Optional future receipt enhancement: persist global_after as well as observed wrapper comparison.
