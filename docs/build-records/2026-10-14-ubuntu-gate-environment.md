# Disposable ARM64 full-gate environment

## Task Contract
- Outcome: Provision pinned full-gate tools/dependency cache and qualify a private Ubuntu environment before project execution.
- Scope: High-risk disposable guest setup and containment admission; isolated owned tool/cache/build workspace only. No system-wide install, node/credential access, original repository/index change, gate/CI/control change or global AppArmor/sysctl weakening.
- Done when: Official manifest/checksum verified Rust1.98.1+rustfmt/clippy, gitleaks8.30.1,cargo-deny0.20.2; locked dependency/advisory caches; full-project private mount/network/PID/loopback and dropped-privilege admission proofs; independent review. Full gate only after environment admission, no step skipping or merge-ready claim for uncommitted overlay.
- Authority: Owner explicitly approved preparing/qualifying the environment, checksum-verified pinned tools/dependencies from named official origins, and full-project namespace containment keeping AppArmor/sysctls enabled. HTTPS static.rust-lang.org, github.com release/source endpoints, release-assets.githubusercontent.com, index.crates.io, static.crates.io. One verifier required. No commit/push/PR or paid services.
- Limits: One task; at most two repair/review cycles. Bounded downloads and processes; initial15minute setup budget then reassess actual progress. No installer/project execution before provenance/containment check. A missing immutable release blocks its component; do not substitute versions.

## Plan and risk
Fetch official metadata and inspect ARM64 artifacts/source/licenses/dependencies/likely effects. Use strict HTTPS redirect allowlist, bounded artifact sizes and exact hashes. Root may create mounts and network/PID namespaces only in owned staging, then drop groups/capabilities before admission and code execution. Keep host shares/homes/service sockets masked, root filesystem protected where feasible, only owned work/tool caches writable, private loopback for Rust socket tests; external network unavailable at gate execution. Provisioning network phase is separate, no user credentials. Existing synthetic fixture launcher is unchanged and not claimed qualified for full gate.

No native models, service operations or container runtime. Record current global protections before/after, retain exact setup evidence and clean owned transient resources. Reviewer checks boundary before full-project execution.

## Bounded failures — BLOCKED
All7ARM64 archives downloaded and matched official manifests/checksums. No downloaded binary or installer executed. Tools manually laid out under owned guest /var/tmp/rikuri-gate-env.3qpv7d9l; original system/user toolchain unchanged.

Initial staging failed because an unprivileged index operation could not traverse intentionally root-private700staging. Repair1 performed only local trusted Git index staging as setup root with explicit safe.directory/no hooks, preserving root700staging.
Next attempt failed before probes: setpriv exec returned Resource temporarily unavailable after assigning desktopUID1000 under RLIMIT_NPROC256. Repair2 switched to unused numericUID/GID60000 inside the minimal root and owned work/source; no system account, ceiling or global setting changed.
Final attempt reached Python launch but admission.py could not be read (Errno13). Root-owned control/tool directory traversal modes remain a blocker. No further repair permitted in this task: AGENTS§5two-repair limit reached. Admission NOT PROVEN; full gate NOT RUN. Global userns/AppArmor values unchanged on every attempt.

Verified downloads and root-private guest staging retained for owner-authorized follow-up; no namespace/server/background process intentionally retained. No project test, downloaded executable, system-wide install, host-wide control change, node/service/credential operation, commit/push/PR.

Next recommended task, not started: correct owned sandbox control/tool directory traversal permissions, then rerun containment admission and independent review under a fresh bounded task. Do not reuse current environment as qualified.

## Independent source-only review / closeout
No additional Important findings beyond acknowledged failed/incomplete admission; environment UNQUALIFIED, gate NOT RUN. Reviewer did not execute reviewed code or re-probe guest. Follow-up nits: explicitly inventory/prove permitted inherited descriptors (including0–2 and child inheritance); distinguish boundary/tool-smoke admission from locked dependency/advisory cache readiness/full gate. GNU chroot normally changes cwd to/; preliminary cwd concern was withdrawn and is not a finding.

Permission metadata confirmed root-owned700control/tools/tools-bin, while admission.py is444. Next authorized repair must audit traversal modes throughout minimal-root paths as well as control/tools, keeping outer staging root-private700. Both repair cycles exhausted; no corrective edit attempted after final refusal.

Evidence: /Users/hchome/Documents/Codex/2026-10-14/i-have-two-new-documentation-in/outputs/ubuntu-gate-environment/verification.md, archive/metadata manifests, three failure logs, permissions.json and proposed/unexecuted controls. Verified downloads and root-private guest stage retained. No dependency/advisory cache populated, tool version proved, environment admission or full-gate PASS. Git diff whitespace check passed.
