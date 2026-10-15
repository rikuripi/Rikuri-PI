# Full Ubuntu gate qualification

## Task Contract
- Outcome: Run the exact repository full gate against an isolated integration snapshot, or report prerequisites that prevent a valid run.
- Scope: Full-gate preflight, source/history snapshot and evidence only; no gate/guard/CI/control modification, package install, production data or node operations. Intended execution is high-risk project test work and requires the named outer safety layer; preflight/snapshot preparation is read-only routine work.
- Done when: Exact snapshot identity recorded; pinned tools and outer safety layer admitted; unchanged scripts/quality/full-gate.sh ends RESULT:PASS15steps, with required independent check. If admission fails, stop before tests and mark BLOCKED/NEEDS_OWNER_DECISION, not green.
- Authority: Owner Next approved isolated full gate. Existing namespace exception qualifies only synthetic fixtures and is not expanded to full project tests. No network destinations/package installations/control changes authorized in this task. No commit/push/PR/merge.
- Limits: One task, two repair/review cycles maximum. Preflight commands bounded10seconds. No broad environment provisioning or follow-on task.

## Preflight
Ubuntu26.04 ARM64 kernel7.0.0-38. System Rust/cargo1.93.1 differs from required1.98.1. User rustup has only stable installed (exact version checked separately). Gitleaks8.30.1 and cargo-deny0.20.2 missing on clean system PATH and explicit user install paths. Required safe-run.sh not installed at checked guest paths. Its source exists in the separate local node repository and was read only as source, never run against node services.

The unchanged wrapper requires unprivileged user/mount/network/PID namespaces and refuses otherwise. Prior owner-approved fixture evidence shows user namespace admission denied by this host; the root-created/drop-privilege synthetic fixture layer is not full-gate qualification. No SAFE_RUN_ALLOW_NO_NAMESPACE override, AppArmor/sysctl change, live-node query or gate invocation is permitted as a shortcut. VM architecture also requires ARM64 artifacts; README/CI pin x86_64 download checksums, which cannot be reused for ARM64.

## Admission outcome — NEEDS_OWNER_DECISION
Installed user stable is rustc1.90.0; system1.93.1; pinned1.98.1 unavailable. Gitleaks/cargo-deny absent. No admitted full-gate safety wrapper in guest. Full gate not invoked, not failed/passed. No tool install/network/global setting/node operation attempted. One initial Parallels inline-command argument error was corrected by using a read-only helper file; no test ran.

Prepared262file source overlay with exact SHA256 manifest and verified HEAD history bundle in outputs/full-ubuntu-gate. Snapshot captured this task record before this admission outcome section; it is not an executed or committed revision. Existing original index/branches/config preserved. History-only secret scanning of HEAD cannot qualify new uncommitted overlay files as merge-ready.

Owner decision needed for separately scoped disposable ARM64 tool provisioning and full safety-layer admission. Existing fixture-only exception is not expanded. Intended root-created/drop-privilege full project boundary requires its own high-risk review/admission; no weakening of AppArmor/sysctls or tests. Details and named official download origins in outputs/full-ubuntu-gate/verification.md.

## Independent prerequisite review
PASS with blockers preserved, full gate NOT RUN. Verifier independently checked262archive members/bytes/modes/hashes and bundle/wrapper identities; no guest test/provisioning. Reminder: HEAD-only history scan and tracked-file checks require deliberate index/history coverage before merge-candidate claims. Owner approval requested for separately scoped environment preparation/admission. No dependent work started while pending.
