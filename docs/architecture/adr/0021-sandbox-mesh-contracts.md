# ADR 0021 — Replaceable execution envelopes and compute mesh seams

**Date:** 2026-11-06
**Status:** PROPOSED — documentation integration requested; owner adoption, implementation admission and any protected rebaseline are pending.
**Supersedes:** None. ADR 0000 and accepted ADRs remain authoritative.
**Owners:** Product owner; security, storage and platform implementers for affected features.

## Context and proposed decision

Refine existing Option C and Link contracts, subject to [ADR 0020 dispositions](0020-vault-foundation-reconciliation.md). Rikuri owns policy/state; adapters supply observed enforcement. Containers share a kernel; a guest VM is a distinct kernel boundary, with hypervisor/device/shared-folder risks. Rootless execution alone is not a VM. A Linux VM hosting multiple containers does not give each container a private kernel. Host-administrator privacy is not promised.

Sandbox capability requests distinguish required assurance from measured assurance. Versioned interfaces cover probe, prepare, launch, execute, stream, inspect, revoke, terminate, evidence and destroy; snapshot/restore are optional and require current-policy reconciliation. Receipts bind task/run/parent, actor, backend/image/policy digests, OS/kernel/architecture, probes, freshness, limits and typed errors. Stale/missing evidence yields unsupported/refused, never a silent fallback. Profile names from the upload are proposals; adversarial code requiring a separate guest kernel cannot fall back to runc. No engine socket, canonical DB or vault master keys in workers. Native approval surfaces cannot expand Rikuri grants. Avoid competing egress proxies unless composition is proven.

Mesh-ready contracts cover DeviceDescriptor, ModelCapability, JobSpec, JobState, StreamEvent, AuthorizationDecision, WorkerLease and ArtifactCommit. These are design names, not added production schema fields. Device roles client/worker/vault keeper/coordinator require separate authorization. Authenticate actors before binding IDs; grants attenuate and expire; dispatch rechecks policy, freshness and resource reservation. Keep the current TaskContract v1 draft intact; extend by an explicitly versioned, tested compatibility change later.

Place whole jobs first. Scheduler admission observes quality floor, queue, load/transfer/inference/delivery time, resource reservations, fairness, thermal/power state and privacy; local residency manager handles weights/KV/cache eviction. SSD is not RAM; cross-node splitting and GPU partitioning remain optional research. Signed advertisements are observations, not resource reservations. Separate private KV/prefix/media/LoRA/cache state; use dedicated instances, serial execution or distinct workers when shared serving cannot meet assurance. Financial/node services remain a separate trust domain.

Streams carry job/stream IDs, monotonic sequence and authorization-bound resume. Preserve current canonical lifecycle; upload job states are illustrative adapter states requiring explicit mapping, never a new ProjectState owner. At-least-once delivery needs idempotency; exactly-once authoritative effect requires tested transactional semantics, and consequential side effects never get blind replay. Stream-end does not imply verified durable artifact commit. Revocation fences new actions/leases and active work where enforceable, quarantines uncommitted output and records irretractable disclosures.

## Options, stages and evidence

Reject immediate deployment of every backend or a parallel production scheduler/database. OpenShell remains first Ubuntu qualification candidate. Docker/Podman/gVisor/Kata/VM/OpenSandbox are evaluated per pinned release/host/workload, never assumed compatible; the source vendor matrix is historical, unverified here. Tailscale is the first Link adapter, not authorization; no public raw inference ports. Platform claims require separate actual OS/hardware evidence. No native macOS/Windows or GPU parity claim follows from Ubuntu/container tests.

Product Stage 1: proposed single-node interface/refusal/stream/commit proof. Stage 3: paired remote workstation, authenticated gateway, encrypted Link, cancellation/reconnect/revocation. Stage 4: measured scheduler/residency strategy. Stage 6: independently authenticated family users, explicit sharing grants and concurrent isolation/fairness. Stage 7: replication and multi-node resource integration; Stage 8: advanced modalities; Stage 10: learned placement. Existing predecessor gates still apply.

## Consequences, acceptance and rollback

Costs include more conservative admission and potentially duplicated inference weights or queueing; never trade a required isolation boundary for throughput. [Acceptance §21](../../../doc/Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md#21-november-6-amendment-acceptance) specifies ENV/HOME cases. Required actual-host allowed/denied/failure evidence and independent security review remain Not yet proven. Supersede adapter choices through dated evidence; keep previous qualified adapter and data. No runtime switch or migration is performed by this ADR.
