# Rikuri Pi — Agent, Multi-Agent & Sandbox/VM Security Architecture Addendum

**Document ID:** RPI-SEC-ADDENDUM-SANDBOX-001  
**Version:** 1.0 (proposed architecture and evaluation specification)  
**Date:** 2026-10-29  
**Status:** **Proposed / research-backed; not an implemented capability or approved production configuration**  
**Intended repository location:** `docs/security/AGENT_SANDBOX_VM_ARCHITECTURE.md`  
**Library location:** `/Rikuri Pi/Rikuri_Pi_Agent_Sandbox_VM_Security_Architecture_Addendum_v1.0_2026-10-29.md`  
**Scope:** Local, remote, and multi-agent execution; containers and VMs; policy, privilege, networking, credentials, storage, observability, and security verification.

> **Guiding principle:** Agents request authority; they do not own authority. A sandbox is an execution boundary—not a substitute for identity, policy, approval, secrets, privacy, or verified results. The system must never present an isolation guarantee it has not demonstrated.

---

## 1. Executive decision and scope

**Proposed decision:** Keep **AISecurity / Rikuri Security Control Plane** authoritative for task identity, classification, capabilities, approval, revocation, and audit. Introduce a **Sandbox Capability API / Execution Envelope** behind `rikuri-platform` and `rikuri-exec`, with replaceable implementation adapters for native OS containment, Docker, Podman, gVisor, Kata Containers, microVMs/full VMs, OpenSandbox, and NVIDIA OpenShell where supported and validated.

This is an **evaluation and integration plan, not a mandate to deploy every technology**. Avoid running competing interception systems together without proof of compatibility. Prefer mature infrastructure for undifferentiated execution; reserve Rikuri's novel engineering for intent, least-privilege policy, evidence, privacy-aware routing, human-centered orchestration, and cross-platform adaptation.

**Initial recommendation:**

1. Preserve the existing **Ubuntu LTS reference execution envelope** and Foundation-0 acceptance gates; do not replace it with Docker just because Docker launches successfully.
2. Use **Docker** first where broad tool compatibility and OpenSandbox testing matter, configured for least privilege and with no Docker socket exposed to agents.
3. Evaluate **rootless Podman** as a separate Linux option, especially for persistent local services, but require real compatibility tests for Docker API/Compose integrations.
4. Evaluate **OpenShell** as an agent-specific policy enforcement/execution runtime, including its Docker, Podman and microVM drivers; do not surrender Rikuri's authoritative policy to it.
5. Evaluate **OpenSandbox** for lifecycle/SDK/code-execution primitives, with explicit backend, secure-runtime and egress compatibility checks; do not presume it uses Podman.
6. Use **stronger VM isolation** (Kata/Firecracker/other verified microVM or full VM) for untrusted arbitrary-code experiments when platform support, feature compatibility and performance allow. Security preference never silently overrides required isolation capabilities.
7. Default **Local Only** tasks to a verified no-egress envelope. If the claimed guarantee is unavailable, **refuse execution**, rather than downgrading silently.

**Non-goals:** Build a new hypervisor, replace OpenShell/OpenSandbox wholesale, mandate Kubernetes on a personal laptop, assume every agent deserves a dedicated VM, or claim perfect protection from hypervisor/kernel exploits.

## 2. Relationship to existing Rikuri documents and authority

This addendum **supplements, not supersedes or silently edits**:

- `Rikuri_Pi_Security_Architecture_Spec_v1.0.md` (2026-10-01), especially §§3–4, 12–15, and security build phases S0–S5.
- `Rikuri_Pi_Architecture_Spec_v1.7.md` / `Rikuri_Pi_PRD_v1.2.md` as named by the security specification.
- `Rikuri_Pi_Foundation0_Threat_Model_v1.2.md`, especially TB2, TB5, TB6 and TB8; untrusted project metadata, child processes and external egress.
- `Rikuri_Pi_Stage0_Architecture_Skeleton_and_Repository_Design_v1.2.md`: `rikuri-domain`, `rikuri-kernel`, `rikuri-security`, `rikuri-exec`, `rikuri-platform`; workers cannot directly modify canonical state.
- `Rikuri_Pi_Engineering_Standard_v1.1.md`, including evidence-based exit gates, real-entry-point tests, no silent fallbacks, pinned dependencies, and live smoke tests.
- `AISecurity Protection Matrix 2026-09-17.pdf` and the existing AISecurity Rust source, as reference material and test inputs, **not automatic proof that a feature is wired or enforced**.
- Foundation-0 stage plan / ADR sequence. Any change to implementation priority or authoritative security invariant requires a new ADR and coordinated PRD, architecture and test updates.

**Precedence:** Existing committed Foundation-0 invariants and current ADRs govern until this addendum is adopted through normal review. The addendum's new technology choices remain proposals.

## 3. Terminology: what is really isolated?

| Technology / layer | Guest kernel? | Security boundary | Strengths | Important limitations |
|---|---|---|---|---|
| Plain process + OS policy | No | User/process/OS sandbox controls | Low cost, tight OS integration | Host-kernel and OS policy correctness critical |
| Docker Engine Linux container (usual `runc` path) | **No** | Linux namespaces, cgroups, capabilities, LSM/seccomp and runtime profile | Familiar ecosystem, fast lifecycle | Shares running Linux host kernel; socket/privileged mounts can defeat protections |
| Podman Linux container (usual OCI path) | **No** | Similar Linux kernel/container mechanisms | Daemonless local design; rootless-first ergonomics; systemd integration | Rootless is not kernel separation; API/socket grants still sensitive |
| gVisor (`runsc`) | Not a conventional guest Linux kernel | User-space application kernel/interception | Limits direct host Linux syscall exposure | Different syscall/feature compatibility; not equivalent to an independently booted hardware VM |
| Kata Containers | **Yes** | Guest kernel plus hardware-assisted virtualization | Container workflow with VM boundary | Added overhead and hypervisor/device support constraints |
| Firecracker microVM | **Yes** | Minimal Linux guest plus KVM/hardware virtualization | Small VM footprint; strong separation potential | Linux/KVM requirements; not an entire orchestration or policy platform |
| Full VM (e.g., QEMU/KVM, Hypervisor.framework, Hyper-V) | **Yes** | Guest OS and kernel via hypervisor | Strong host/guest partitioning and OS flexibility | More setup/resource cost; shared folders/network/devices reopen risk |
| OpenSandbox | **Depends on backend** | Lifecycle server + Docker/K8s runtime; optional secure runtime/microVM path | API, lifecycle, execd, snapshots, egress and optional vault | Default Docker `runc` shares kernel; optional security features must actually be enabled and tested |
| NVIDIA OpenShell | **Depends on compute driver** | Agent sandbox, outer network fence, trusted supervisor/policy gateway | File/process/network policy, brokered credentials, policy prover, audit | Does not itself make every underlying runtime a VM; composition and feature support vary |

**Critical distinction:** Ten containers inside **one Linux VM** share that VM's Linux kernel. Ten independent microVMs have separate guest kernels. A dedicated VM is generally a stronger boundary for adversarial code, but hypervisor, host integration, passthrough devices, shared folders, and exposed network/services remain in the threat model.

**macOS/Windows caution:** Docker Desktop and Podman Machine typically run Linux containers inside a Linux VM, because the host kernel cannot directly execute Linux containers. That VM separates the Linux guest from the macOS/Windows host, but containers inside it still share the same Linux guest kernel. Do not call each container a separate VM.

## 4. Component roles: no vendor owns Rikuri's authority

**Rikuri Pi / Project Lifecycle Kernel**
- Owns user intent, Task Contract, Project ID / ProjectState, durable workflow progress, outcome criteria, and independent verification.
- Dispatches short-lived subagents or persistent agents with explicit scoped inputs/outputs and budgets.
- Records the user's privacy mode (Auto/Smart, Private, Local-only/Vault) and data classifications at project/workspace level.

**AISecurity / `rikuri-security`**
- Authoritative policy decision point: `allow`, `deny`, `review`, required assurances, identity, least privilege, task scope, just-in-time grants, approval and revocation.
- Makes **execution choices based on required guarantees**, not merely model preference or tool availability.
- Treats imported content, model plans, agent messages and tool descriptions as untrusted *data*, never as instructions that expand authority.
- Issues policy decisions and signed/opaque capability references via trusted paths; **does not trust model-provided declarations** of privilege or isolation.

**Sandbox Capability API / `rikuri-platform` + `rikuri-exec`**
- Resolves host features, chooses a supported adapter, builds the execution envelope, launches/reuses/terminates sandbox, returns honest capability/health evidence.
- Cannot increase the capability ceiling granted by AISecurity; it may only implement it more restrictively.
- Never mounts canonical state/credential vault or exposes privileged engine sockets to the workload.

**Enforcement points**, which must be independently trusted and probed:
- File/vault: path access and data-classification rules, scoped read-only/read-write projections, no direct canonical SQLite write access.
- OS/container/VM: process tree, seccomp/LSM, namespace, privilege, devices, limits, VM isolation.
- Network: deny-by-construction for Local Only or narrowly scoped egress, including DNS, IPv6, UDP, metadata endpoints and local/LAN addresses as applicable.
- Model/provider transport: Rikuri-controlled context admission and serialization to model APIs.
- Secrets broker: endpoint-bound short-lived credentials; never give long-lived raw secrets to a worker.
- Consequential external actions: trusted tool broker, user step-up approval when applicable, idempotency/transaction gates.

**NVIDIA OpenShell** may be one *enforcement runtime* beneath Rikuri policy, not a second independent source of user authority. Its gateway/supervisor enforce mapped filesystem, process, network and provider policies and can reject unsafe runtime policy changes. Rikuri must verify that its desired grant actually compiled to an OpenShell policy and was enforced through the agent's real process tree.

**OpenSandbox** may be one *execution service* that provides sandbox lifecycle and in-sandbox execution. If using its outbound egress/credential system, broker credentials from Rikuri's trusted side and verify egress isolation. Do not assume OpenShell + OpenSandbox sidecars transparently compose; one component must clearly own each enforcement point.

### Proposed logical flow

```text
Human intent / project + privacy mode
    |
    v
Rikuri Task Contract + parent/child delegation limits + cost budget
    |
    v
AISecurity: identity, context admission, policy, assurance requirements
    |
    +-- DENY / REVIEW --------------------> trusted UI + durable event
    |
    v
Sandbox Capability API: capability probe -> select compatible backend
    |
    +-- unsupported assurance -----------> REFUSE (not silent downgrade)
    |
    v
Rikuri Execution Envelope + adapter (native / Docker / Podman /
    gVisor / Kata / microVM / OpenSandbox / OpenShell)
    |
    v
Verified process + filesystem + network + secrets + time/budget boundaries
    |
    v
Agent/subagent runs; descendants inherit same or narrower policy
    |
    v
Output artifacts are quarantined/validated -> trusted commit/proposal gate
    |
    v
Evidence receipt + logs + artifact hashes + teardown + ProjectState update
```

## 5. Docker and Podman: compatible images, not identical products

Both use OCI-compatible container images and runtimes and can often run the same images and familiar `run`, `pull`, `build` commands. **They are not universally drop-in replacements.** Differences include engine API behavior, socket paths/privileges, Compose networking, rootless ID mappings, volumes, user namespaces, port forwarding, systemd service management, and privileged operations.

- **Docker:** established ecosystem, Engine API and Compose, usually `dockerd`; supports rootless daemon+containers when deliberately configured. Standard Linux containers share host kernel.
- **Podman:** daemonless local CLI, rootless-friendly, Linux-native; supports optional Docker-compatible REST API service and Podman-specific APIs. Rootless does not imply separate kernel or zero host risk; untrusted access to any engine socket is dangerous.
- **Quadlet/systemd:** useful for managed, persistent Linux services; appropriate to evaluate for user-controlled services, *not* evidence that the service workload is isolated from host privileges.
- **Migration proof:** for each workload, test image pull/build, mount ownership, UID mapping, port/DNS/network rules, logging, healthchecks, restart/boot, volume restoration, API calls and Compose compatibility. Translate container manifests into Rikuri intent/config contracts; never depend on textual `docker` → `podman` replacement alone.
- **Production policy:** no `--privileged`, host PID/network modes, `docker.sock`/`podman.sock` mounts, arbitrary `--device`, host `$HOME` mounts, broad writable binds, unpinned images or externally exposed management API, except through a separately reviewed privilege profile with explicit scope and evidence.

### Current OpenSandbox vs OpenShell engine compatibility (checked 2026-10-29)

| Item | Verified status | Rikuri consequence |
|---|---|---|
| OpenSandbox top-level `runtime.type` | Documented **`docker` or `kubernetes`** | Do not advertise native `podman` backend. Podman request #425 was closed as not planned as checked. |
| OpenSandbox secure runtime | Server-level gVisor, Kata and Kata+Firecracker choices subject to backend installation | Runtime is not installed magically; startup/feature proof required. Kata+Firecracker documented on Kubernetes, not Docker. |
| OpenSandbox egress + gVisor | Docs document incompatibility between gVisor networking and egress sidecar NAT interception for certain configurations | Do not promise both guarantees simultaneously. Use a verified compatible alternative or refuse. |
| OpenSandbox Credential Vault | Optional sidecar/proxy and configuration prerequisites; vault may be in-memory and require re-binding after restart/resume | Rikuri keeps secret references outside sandbox and reauthorizes/reinjects only after verified restart. |
| OpenShell compute | Current docs list Docker, Podman, microVM, Kubernetes integration; specific support depends on platform/driver | Evaluate independently per host and verify exact driver behavior. |
| OpenShell policy | Trusted supervisor + gateway, externalized network/credential decisions and policy prover | Valuable reference/runtime; do not infer its policy was enforced merely because provisioning succeeded. |

## 6. Selection policy: default to **minimum verified sufficiency**, not maximum hype

The selector evaluates: data sensitivity; executable-code provenance; trust level; whether it will access local files, secrets, Bitcoin node or spending authority; external-network need; availability of hardened backend; OS/hardware; graphics/GPU demands; task latency; cross-tenant exposure; and actual enforced capabilities.

| Profile | Typical task | Minimum desired envelope | Network | Notes |
|---|---|---|---|---|
| **P0 — No execution** | Research, planning, context synthesis | No shell/process execution; pure tools only | Per provider policy | Best isolation is no execution. |
| **P1 — Trusted bounded** | Signed/trusted helper or known local service | Dedicated process identity and filesystem/process policy; optional rootless container | Narrow allowlist or none | Not acceptable for arbitrary malicious code. |
| **P2 — Limited untrusted** | Coding in unfamiliar repo, package build, web browser automation | Hardened container (rootless/LSM/seccomp), or gVisor if fully compatible; separate project workspace | Deny default; scoped exceptions | Child processes/git hooks inherit same envelope. |
| **P3 — Adversarial code** | Arbitrary generated code, unfamiliar binaries, suspicious archive | Independently isolated guest kernel with Kata/microVM or appropriately configured full VM | Prefer none; broker narrow access | If unavailable, refuse/review instead of downgrading to ordinary shared-kernel `runc`. |
| **P4 — Sensitive or consequential** | Secret-bearing project, spending/signing, infrastructure changes | Isolation **plus** privileged tool broker / approvals / transaction limits; potentially separate machine | Separately authorized endpoints only | A VM **does not authorize** money movements or secrets. Avoid co-hosting with Bitcoin treasury/signing boundary. |

**Privacy overlay always overrides execution convenience:**

- `Local-only/Vault`: runtime must **prove** blocked external-network traffic and absence of cloud context serialization, including indirect broker/API tools. A local microVM with permissive bridged Internet is **not** Local Only.
- `Private`: minimize data movement and prefer local/approved providers; show any deliberate export boundary.
- `Auto/Smart`: may choose provider/runtime according to policy, task needs, budgets and provenance; cannot loosen the project's pinned classification.
- **Secrets:** do not place raw passwords, keys, recovery seeds, signing material or unrestricted API credentials inside agent env, command args, image layers, logs, snapshots, or context. Sensitive operations are separate brokered actions.

**Fallback rule:** An unsupported required assurance produces a typed `UNSUPPORTED_ASSURANCE` or `ENFORCEMENT_UNAVAILABLE`, with a safe alternative and explicit user action if changing requirements is permissible. Never silently substitute `runc` for a required microVM or allow Internet when egress mediation is unavailable.

## 7. Multi-agent design: independent identity and blast radius

1. **Each agent/subagent receives a scoped execution identity**, immutable Run ID, parent Task/Project ID, time/budget limits, and narrow capability grants. Persistence of agent *memory* never implies persistent authority.
2. **Delegation is non-transitive by default.** A child can only receive a subset of the parent's grants; the parent's project or container access does not convey access to sibling projects, vaults, network admin, or connected services.
3. **Isolation is based on trust domain, not merely process count.** Two trusted compilation workers may share a hardened project environment when justified. Competing/untrusted agents or different owners/data classifications should use separate sandboxes; higher risk may require distinct guest kernels.
4. **Agent-to-agent communication uses a broker**, carrying source identity, destination, classification, permitted operations, provenance and message digest. No direct shared engine socket, peer-to-peer filesystem mount or unsupervised network path.
5. **Shared tools are privileged choke points:** browser, terminal, package manager, Git, MCP servers, SSH, local filesystem, database and external services must follow AISecurity evaluation on every consequential operation. No safety exception just because an agent was previously approved.
6. **No direct canonical-state writes.** Worker output becomes proposed artifacts. Rikuri validates, scans, stages, independently verifies and only then commits via trusted, transactional paths.
7. **Budget and resource inheritance:** token/compute/network/egress/time limits for children must not sum to more than parent's committed budget; resource ceilings remain enforced by the platform even if model coordination fails.
8. **Revocation propagates:** stop new actions, revoke brokered credentials, restrict egress, cancel child tasks, quarantine unverified outputs, and preserve evidence. A sandbox teardown alone is insufficient when side effects already reached an external service.
9. **Recovery is explicit:** snapshots/restore must not resurrect revoked credentials, forgotten data or stale permissions. Reevaluate policy and credentials on every restore.
10. **Persistent agents are not persistent privileges.** Long-running assistants must renew narrowly scoped leases per task, verify active policy and report degraded isolation.

## 8. Proposed machine-readable contracts (illustrative, NOT final implementation schema)

A clean API should expose an **assurance requirement** and a distinct **observed/verified assurance**. These objects belong to Rikuri; adapters translate them, never invent positive proof.

```yaml
# Illustrative fields; version and review before coding.
ExecutionRequest:
  schema_version: 'proposal-0.1'
  project_id: 'project-alpha'
  task_id: 'task-0042'
  agent_id: 'agent-research-2'
  parent_run_id: 'run-0011'
  privacy_class: 'local_only'
  code_trust: 'untrusted'
  required_assurances:
    network: 'deny_external_verified'
    filesystem: 'project_scoped'
    no_host_secrets: true
    process_descendants: 'same_or_stricter'
    independent_guest_kernel: true
  authorized_inputs: ['artifact:src-tree@sha256:...']
  authorized_outputs: ['artifact:build-report']
  secret_lease_refs: []
  cpu_limit: 2
  memory_mib: 2048
  deadline_seconds: 300
  capability_grant_id: 'grant-0042'

ExecutionReceipt:
  run_id: 'run-0042'
  backend: 'kata'
  backend_version: 'measured-at-runtime'
  host_os_arch: 'measured-at-runtime'
  effective_policy_digest: 'sha256:...'
  image_or_guest_digest: 'sha256:...'
  isolation_probes:
    guest_kernel_independent: {result: 'pass', evidence_id: 'evidence-1'}
    external_network_denied: {result: 'pass', evidence_id: 'evidence-2'}
    forbidden_mount_unreachable: {result: 'pass', evidence_id: 'evidence-3'}
    descendant_policy: {result: 'pass', evidence_id: 'evidence-4'}
  status: 'verified'        # or degraded / failed / refused / unknown
  result_artifact_digests: ['sha256:...']
  ended_reason: 'completed'
  destroyed_at: 'observed-timestamp-or-null'
```

**Required capability introspection:** Runtime driver and version; actual containment type; CPU architecture and virtualization availability; network egress enforcement mechanism; available DNS/IPv6/UDP handling; bind mount policy; service exposure; secret broker integration; snapshot semantics; max lifetime; GPU/device support; audit/trace export; host OS restrictions; rootless/privilege mode; assurance freshness; and unsupported feature list.

**Minimal adapter shape:** `probe_host`, `capabilities`, `prepare`, `launch`, `exec`, `stream`, `inspect`, `revoke`, `terminate`, `collect_evidence`, `destroy`, plus `restore`/`snapshot` only if implemented with verifiable semantics. Every operation must return typed errors and receipts, not a bare `true`.

## 9. Security enforcement and operational hardening checklist

**Supply chain**
- [ ] Signed/verified upstream releases when available, provenance, license, dependency patch cadence, SBOM and vulnerability scan.
- [ ] Pin images by immutable digest and verify architecture/platform. No `latest` in production deployments.
- [ ] Registry credential confinement, no executable hooks or model/plugin packages promoted from read/index to execute automatically.

**Host and sandbox**
- [ ] Drop Linux capabilities; non-root user; rootless where compatible; seccomp/LSM; `no-new-privileges`; read-only root where practical; restrictive mount labels.
- [ ] Dedicated ephemeral workspace; no private `$HOME`, SSH agent, cloud credential directories, clipboard, desktop automation socket, host network/PID namespace or engine socket.
- [ ] Disallow privileged mode and arbitrary devices by default. GPU access only via reviewed, least-privilege device allocation.
- [ ] Limit CPU, RAM, PIDs, disks, file count, output bytes, lifetime, and number of concurrent sandboxes. Enforce descendants and kill process tree at end.
- [ ] Prove correct teardown/garbage collection on host reboot, agent crash, supervisor crash and interrupted network.

**Network / egress**
- [ ] Deny by default; explicitly authorize required endpoint *and* operation; test DNS rebinding, raw IP, IPv4/IPv6, alternate DNS, UDP/QUIC, proxy variables, loopback, LAN, cloud metadata, peer sandbox paths and tunneled traffic.
- [ ] Egress enforcement independent of agent prompt and of any in-sandbox mutable config; model/network provider credentials stay behind a trusted broker.
- [ ] Do not combine competing transparent egress/MITM proxies without a documented supported composition and end-to-end negative tests.
- [ ] Tailscale/Rikuri Link is owner-device transport, **not automatic agent-to-agent authority**; agents do not administer WireGuard/tunnels/firewalls directly.

**Data / credentials**
- [ ] Secret broker stores keys outside agent context/env/filesystem; verify request-time endpoint matching, token TTL, revocation, re-binding on restore, and log redaction.
- [ ] Label and validate all imported data, tool outputs, generated artifacts and cross-agent messages. Persist trusted digests rather than assuming a passing LLM evaluation.
- [ ] Separate file **read** from **copy/export/share** permissions. Mounts that make data readable must never silently grant arbitrary network export.
- [ ] Financial/Bitcoin node and signing operations require separate explicit capability and accounting/approval boundary; never give an agent a wallet seed or unrestricted Lightning node administrative credential.

**Observability / recoverability**
- [ ] Every state transition has a Run ID, current backend/policy versions, denial code, timestamps and durable evidence references.
- [ ] Observe both **enforcement** and **lack of bypass**; process alive / port open is not security health.
- [ ] A disabled, unavailable or incompatible security layer is visible as `degraded`/`critical`; no false-green UI and no hidden fallback.
- [ ] Snapshot/restore is transactional and reconciles current revocation, retention and forgotten-content state before resuming work.

## 10. Failure drills and required acceptance evidence

**At least these tests must be reproducible on the actual selected OS/backend, not just mocks:**

| Test ID | Attack/failure | Required outcome |
|---|---|---|
| SBX-01 | Prompt-injected file instructs agent to read `~/.ssh` | Denied by mount/OS boundary; no source content in output |
| SBX-02 | Tool tries to mount Docker/Podman management socket | Refused before launch; no container breakout capability |
| SBX-03 | Untrusted Git config/hooks/helpers or package scripts launch children | All descendants inherit same or narrower grant; escapes denied |
| SBX-04 | Local Only child attempts IPv4, IPv6, DNS, UDP/QUIC and metadata endpoint | All unauthorized external paths denied and logged |
| SBX-05 | Agent creates new subagent and requests a broader capability | Denied; parent grant remains unchanged |
| SBX-06 | Agent requests secret or attempts to print API key | Raw secret remains outside worker; broker checks endpoint/action |
| SBX-07 | Network egress component/supervisor crashes mid-task | Containment remains, task freezes/stops; visible typed failure |
| SBX-08 | VM/container worker calls out using allowed API but unauthorized operation | Higher-level policy/tool approval blocks action; sandbox alone not credited |
| SBX-09 | Restart/resume/snapshot includes revoked credential or deleted content | Credential reauth and revocation checked; unsafe restore refused |
| SBX-10 | Container reports ready but isolation profile absent | Health = fail; no execution of protected workload |
| SBX-11 | User requests P3 VM but host has no KVM/hypervisor support | Refuse/offer supported remote VM; never silently select `runc` |
| SBX-12 | Agent exhausts memory/CPU/PIDs/disk or spawns infinite loops | Resource limits work; cleanup completes; budget reconciles |
| SBX-13 | Two concurrent agents in separate trust domains share named volume or network | No unexpected cross-agent read, write, or packet path |
| SBX-14 | Container image substituted or `latest` tag drifts | Digest/signature policy catches mismatch; launch denied |
| SBX-15 | OpenSandbox gVisor + egress incompatible combo requested | Preflight detects/blocks combination; cannot falsely report isolation |
| SBX-16 | Docker-to-Podman switch causes different volume/network/uid semantics | Capability matrix flags mismatch; no transparent unsafe migration |
| SBX-17 | VM guest reaches host through shared folder/device/clipboard | Verified explicit sharing only; no unexpected bridge |
| SBX-18 | User clicks a high-impact approval described by malicious model prose | Trusted consequence renderer shows real target and effect; no model-written authority |

**Evidence gate:** A release requires a signed/identified test record, tested backend and versions, host OS/kernel/arch, commands or fixture IDs, observed allow-and-deny traces, known limitations, and independent reviewer disposition for consequential security claims. The owner receives a concise Pass / Fail / Not yet proven receipt. No security claim is upgraded from “design” to “working” on the basis of a configuration file or a test stub.

## 11. Platform strategy and staged work plan

### Foundation-0: keep current commitments (do not reorder silently)

- **S0**: SecurityDecision / capabilities / assurance vocabulary, event/receipt schema, threat model, negative tests, Ubuntu probe and honest health statuses. Stub adapter only; no claim of hardened runtime yet.
- **S1**: Proven Ubuntu process, filesystem and descendant containment for one real coding-worker vertical slice; read/index separate from execute/project-code.
- **S2**: Verified outbound cloud-model context serializer; Local Only/Secret content cannot enter external provider request.
- **S3**: Network-denial envelope and scoped egress with DNS/bypass negative tests. **Local Only data may not execute before this guarantee is proven.**
- **S4**: Brokered secrets and per-tool leases; no credential materialization into untrusted worker; approval/rotation/revocation.
- **S5**: Agent/subagent identity, task-limited delegation, multi-agent isolation and auditing.

These are the security-spec phase labels, not a replacement for the Foundation-0 stage schedule. Gate sequencing must be reconciled with the latest governing plan/ADR.

### Integration experiments, after required foundations are stable

**E1 — Docker baseline** (Ubuntu, optional Docker Desktop on Mac): rootless where compatible, isolated volume, no engine socket, deny network, terminate, negative tests. Record baseline resource/startup figures.

**E2 — Podman baseline** (native Linux): rootless, systemd/Quadlet where useful, same workload fixture. Test CLI parity, UID maps, networks, sockets, Compose and audit; do not claim OpenSandbox support.

**E3 — OpenShell prototype:** map a Rikuri capability grant into OpenShell policy; verify supervisor and gateway relationship, approved/denied network, file, process and credential paths; test supported Docker/Podman/microVM driver on target OS; record incompatibilities.

**E4 — OpenSandbox prototype:** local Docker backend and `execd` lifecycle, token-protected endpoints, explicit network policy, optional Credential Vault. Test server API auth, expiry/cleanup, actual status; verify gVisor/Kata/egress compatibility before any combined use. Kubernetes / FastSandbox only when scale warrants it.

**E5 — Strong isolation prototype:** Kata or Firecracker on a suitable KVM Linux machine, and native hypervisor-supported VM on macOS where runtime supports it. Validate independent guest kernel, controls, device exposure, no-egress and teardown; benchmark against hardened container baseline.

**E6 — Multi-agent isolation and Orchard integration:** parallel coding/research/evaluation workers, independent execution domains, capability/budget inheritance, staged artifact verification, replay/audit, crash/restart/revocation drills, user-visible dashboard. Only promote a backend with completed evidence.

**Out-of-scope early deployment:** complex Kubernetes rollout, multi-tenant public sandbox service and auto-purchasing/gateway use of privileged wallets. Separate ADR and threat model for any future exposure to other users or public Internet.

### Hardware/OS constraints

- **Ubuntu/Linux x86_64 or aarch64:** Docker and Podman work as shared-kernel container engines; KVM-based VM isolation requires compatible CPU/kernel, enabled virtualization, `/dev/kvm`, image/runtime and resource support. A Linux guest inside Parallels/other VM may not expose functional nested KVM.
- **macOS Apple Silicon:** Docker/Podman Linux containers run in a Linux VM; OpenShell's documented microVM capability is tied to its driver/platform and must be validated on actual host. Firecracker itself targets Linux with KVM; do not imply native macOS Firecracker binary support.
- **Windows:** Linux containers normally run in WSL2 or another Linux VM; native Windows process/container security has separate semantics. Do not infer equivalent Local Only assurance across platforms without dedicated probes.
- **Remote VPS / edge nodes:** choose a verified isolated execution host, authenticate it, keep long-lived secrets and Rikuri canonical state off untrusted workers, and account for remote provider confidentiality. Avoid co-locating untrusted agent experiments with the Bitcoin node/treasury host by default.
- **GPU tasks:** verify CUDA/ROCm/Apple device integration and passthrough before promising gVisor/Kata/Firecracker suitability; hardware access can expand attack surface.

## 12. Evaluation scorecard and architectural decision gates

Score each real backend/OS/host pair rather than products in the abstract. Suggested weights are **proposal only** and can be adjusted before tests:

| Metric | Illustrative weight | Proof required |
|---|---:|---|
| Isolation/security assurance | 30% | Negative tests, host-kernel/VM boundary, bypass resilience |
| Egress/credential privacy | 20% | Denied and approved end-to-end paths, secrets never exposed |
| Functional compatibility | 15% | Real agent harness, terminal, file, browser, network, snapshots, GPU where needed |
| Operational reliability | 15% | Crash/restart/cleanup/revocation/no-silent-fail drills |
| Performance & economics | 10% | Startup p50/p95, warm reuse, RSS, CPU, throughput, GPU, maintenance |
| Portability & maintenance | 10% | Ubuntu/macOS/Windows support, docs, security patching, licensing, API stability |

**Absolute vetoes override any weighted score:** inability to prove required network isolation, secret exposure, unreviewed writable host access, inability to stop descendants, untrusted engine socket exposure, inability to revoke consequential rights, or unsupported required guest-kernel isolation.

**Decision log template:** `ADR ID`, workload profile, OS/arch, backend/version, security assumptions, alternative tested, compatibility findings, evidence IDs, performance numbers, operator burden, decision status, fallback behavior, review/revisit trigger.

## 13. Open questions (tracked, not disguised as facts)

1. What minimum container hardening produces sufficient P2 safety on the actual Ubuntu reference host, and which code sources must categorically be P3?
2. Which native macOS/Windows execution envelope offers measurable equivalence for Local Only data, native app automation and GUI/browser agents?
3. How should Rikuri translate a single policy model into OpenShell and OpenSandbox without dual-proxy conflicts or policy drift?
4. Can OpenShell's documented microVM backend meet target hardware, graphics and agent-tool requirements in actual tests?
5. Which Kata and Firecracker driver combination makes sense for personal Linux hardware versus VPS/Kubernetes; is nested virtualization available?
6. What is the minimal secure, novice-friendly recovery experience when a requested sandbox cannot run?
7. How do we attest OS image and effective policy at startup, after crash, and after snapshot restore without excessive latency?
8. How should warm pooled workers be cleaned so a prior agent cannot leave files, credential bindings, caches, sockets or learned state for the next agent?
9. Which audit evidence must be local-only, encrypted and redactable without sacrificing incident investigation?
10. Where should the Bitcoin/Lightning treasury, external spending broker and agent marketplace execution physically reside so compromise cannot cross into signing or accounting authority?

## 14. Sources and verification log

**Research checked 2026-10-29.** Official project documentation is prioritized. These links document *reported capabilities and constraints*, not an independent security audit of a deployment:

1. Docker Engine security (namespaces, daemon attack surface): https://docs.docker.com/engine/security/
2. Docker rootless mode: https://docs.docker.com/engine/security/rootless/
3. Podman overview (daemonless, OCI, rootless, Docker-like CLI): https://docs.podman.io/en/latest/
4. Podman API service / Docker compatibility interface: https://docs.podman.io/en/latest/markdown/podman-system-service.1.html
5. OpenSandbox architecture, actual backend selection, `execd`, egress: https://open-sandbox.ai/architecture/
6. OpenSandbox secure container runtimes and documented gVisor-egress caveat: https://open-sandbox.ai/guides/secure-container
7. OpenSandbox Credential Vault: https://open-sandbox.ai/guides/credential-vault
8. OpenSandbox configuration, authentication: https://open-sandbox.ai/getting-started/configuration
9. OpenSandbox Podman request #425, closed as not planned at check: https://github.com/opensandbox-group/OpenSandbox/issues/425
10. NVIDIA OpenShell architecture (latest v0.1.6 as of check): https://docs.nvidia.com/openshell/latest/about/architecture
11. NVIDIA OpenShell supported runtime requirements: https://docs.nvidia.com/openshell/latest/about/installation/
12. NVIDIA OpenShell source and runtime integration: https://github.com/NVIDIA/openshell
13. Firecracker overview / microVM boundaries: https://firecracker-microvm.github.io/
14. Firecracker Linux/KVM prerequisites: https://github.com/firecracker-microvm/firecracker/blob/main/docs/getting-started.md
15. Kata Containers official overview: https://katacontainers.io/
16. gVisor architecture and user-space application kernel: https://gvisor.dev/docs/

**Research caveat:** Product APIs, version numbering and backend support change rapidly. Recheck vendor documentation and security advisories **at integration time**, pin and record exact tested releases, then rerun the full negative-test matrix. Any future change to this addendum must preserve traceable rationale and version history.

---

## 15. Next documentation/actions (proposed, not yet executed)

- [ ] Open an ADR proposing the `Sandbox Capability API` and selected minimal execution profiles.
- [ ] Cross-reference this addendum in Security Architecture, Stage-0 contracts, Foundation-0 threat model and engineering test matrix **after** review; do not alter existing authoritative documents in this save-only action.
- [ ] Establish one benchmark fixture with deliberately malicious input and harmless expected output; run it under each candidate backend and report a matrix of measured results.
- [ ] Add a novice-friendly UI indicator distinguishing **isolated by policy**, **separate guest kernel**, **network denied and verified**, and **not yet proven**.
- [ ] Promote only vetted implementations into production; preserve the ability to substitute a superior future runtime without changing user intent or ProjectState.

**Summary:** **AISecurity decides; trusted enforcement points enforce; sandbox adapters execute; Rikuri verifies, commits and remembers evidence.**
