# Full-project containment admission repair

## Task Contract
- Outcome: Correct owned sandbox traversal modes and prove containment/tool smoke admission.
- Scope: High-risk repair of disposable guest launcher/probes and owned control/tool directories only; outputs/ubuntu-admission-repair plus this record. No project execution, cache provisioning, full gate, original gate/guard/CI edits or global controls.
- Done when: Outer staging remains root700; readable/traversable readonly tools/control and minimal-root directories; nonrootUID60000/groups/capabilities/NoNewPrivs, private namespaces/loopback/no external network, masked homes/services, readonly source/tools, exact tool versions; parent/child descriptor policy proven; independent review. Failed prerequisite stops execution.
- Authority: Owner Ok Next explicitly approved fixing directory permissions and rerunning admission as a fresh bounded task. Reuse prior verified tools and approved root-created minimal-root namespace design. No system account/install, downloads, live node, secrets, commit/push/PR or project tests.
- Limits: Fresh maximum two repair cycles/two review rounds. Each admission bounded180seconds, helpers190seconds. One verifier, no parallel coding agents.

## Red evidence / plan
Prior outputs/ubuntu-gate-environment/admission-red.txt records Python Errno13 reading/opt/control/admission.py. permissions.json confirms root700tools/control/tool-bin with444probe. The launcher also creates minimal-root directories under inherited restrictive umask. Keep outer root700; explicitly normalize only owned readonly tool/control directories to755 and use umask022 inside newly created root. No safety check is removed.

Close inherited descriptors through subprocess close_fds; stdin/dev/null, stdout/stderr pipes. Inventory0–2 and absence of extra descriptors in admission and stripped-environment child, in addition to existing privilege/network/filesystem probes. Tool smoke admission remains separate from missing locked dependency/advisory caches and full gate.

## Verification
Boundary/tool smoke admission passed exit0. Corrected owned tool/control directory traversal and inside-root umask while outer stage remainedroot700. NonrootUID60000, no groups/all capabilitieszero/NoNewPrivs1; changedmount/net/pid namespaces; empty homes/shares/services/sys; read-only usr/source/tools/control; private loopback works; externalIPv4 ENETUNREACH.
Parent/child descriptor inventories exactly0dev-null,1pipe,2pipe. Negative regular-stdin and leaked-fd child checks refused. Pinned rustc/cargo1.98.1,rustfmt1.9.0,clippy0.1.98,gitleaks8.30.1,cargo-deny0.20.2 executed successfully after boundary checks.
One initial green smoke run, then final green after adding negative descriptor coverage; no failed repair cycles this task. Globals userns/AppArmor remain1. No project execution/download/cache provisioning. Guest stage retainedprivate for laterauthorizedwork.
Independent review pending; fullgate/cache readiness not yet proven. Evidence /Users/hchome/Documents/Codex/2026-10-14/i-have-two-new-documentation-in/outputs/ubuntu-admission-repair/admission-output.txt and exact controls.

## Closeout — DONE for boundary/tool admission only
Independent review PASS, no Important findings. Reviewer verified control hashes/helper design and scoped evidence; no guest re-execution. One Nit fixed in place: enforce explicit cargo/rustfmt/clippy expected-version assertions in addition to recorded outputs. Final fixed controls rerun exit0, all boundary/descriptor-refusal/version checks passed. No failed repair cycles. No second review needed for Nit perREVIEW.
Final evidence and hashes in outputs/ubuntu-admission-repair; retained workspace pointer updated with exact final controls, caches_readyfalse/full_gate_runfalse. Source/doc whitespace checkpassed.
No cache provisioning or project execution this task. Fullgate/merge/stage result remain unproven, no commit/push/PR orglobal changes. Next recommended task, not started: populate offline locked dependency/advisory caches and run the unchanged full Ubuntu gate in this admitted boundary, retaining overlay/history coverage caveats.
