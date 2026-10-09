# ADR-0004: Foundation threat model and trusted-computing-base assumptions

Status: Proposed
Date: 2026-10-05
Owners: Henry Chen (product owner, accepts); Stage 00 implementer
Related requirements/tests: Stage Plan S00.10, S00.12, S00.13; Architecture §7.3, §27.1, §27.12–27.14; [threat model](../../security/THREAT_MODEL.md); ADR 0000 (September 28 addenda); PRD §10

## Context / problem

Every Foundation-0 protection claim depends on what is assumed trusted. The threat model and Architecture §27 describe assets and threats in detail; this ADR fixes the assumptions that bound Foundation-0's guarantees, so a later claim cannot quietly widen them and a later change to them is visible as a decision.

## Decision drivers

- Claims must name what they rely on; an unstated assumption becomes an unproven promise.
- The trusted base must stay small and outside worker authority.
- No guarantee may rest on a model's cooperation or a detector's accuracy.

## Options considered

1. **Trusted host kernel and authenticated owner account; everything Rikuri launches or ingests is untrusted** (chosen; Architecture §27.1, threat model §3).
2. **Also defend against a compromised owner account or root**: needs hardware isolation or a separate machine; out of scope for a single-machine personal Foundation and not claimed.
3. **Trust local model processes and Pi as "ours"**: rejected; they process untrusted content (Architecture §7.3).

## Evidence / prototype

Assumption record, not runtime evidence. The September 28 OpenShell receipt is candidate evidence only; S00.13 and Stage 03 qualify the backend. All protections below are Not yet proven until their owning tests pass.

## Decision

**Trusted (assumptions):**

- The host operating-system kernel and its enforced security modules on the admitted reference installation (Ubuntu 26.04 LTS, GA 7.0 kernel; ADR 0000 September 28 addendum).
- The currently authenticated owner account and the owner's deliberate actions.
- Rikuri's own trusted services: `rikurid` (canonical state, ledger, policy decisions, budgets, provider adapters), the AISecurity policy service, credential brokers when a later stage enables one, and the admitted enforcement backend with its privileged runtime and driver dependencies. For the OpenShell candidate this includes the gateway, the supervisor container (host networking, the mediated egress point) and the container engine.

**Untrusted (adversarial or fallible):** imported files and histories, repositories and their metadata, web pages, prompts, model output, tool output, local model processes, the Pi harness process, extensions, workers and their descendants, external networks and providers. Accidental edits, runaway loops, confused-deputy use, malicious dependency hooks, descendants escaping a parent, resource exhaustion, stale approvals and stopped security services are expected.

**Out of scope for Foundation-0 guarantees:** root or administrator compromise; a compromised host kernel, hypervisor, firmware or hardware; malicious actions by the owner; unrelated applications and activity elsewhere on the computer; copies exported or disclosed outside Rikuri's managed stores. OS and device security are dependencies, not Rikuri guarantees.

**Rules that follow:**

1. Only the owner authorizes scope and consequences. Untrusted content cannot grant permission, change approval text, weaken verification, remove a budget ceiling or promote advice to canonical truth (`instruction_authority: none`; `rikuri-domain`'s `Provenance` derives authority from origin, and a loaded `Provenance` always carries `none`).
2. Only `rikurid` mutates canonical state. Clients and workers are untrusted callers of versioned contracts.
3. Root-equivalent access stays out of Rikuri's processes: the accounts that run `rikurid`, Pi, model servers and tool servers are not members of `docker`, `lxd` or similar groups; a backend controller that needs a rootful engine runs under a dedicated service account or a rootless engine (ADR 0000 September 28, decision 2). On the reference machine the owner account is currently in `docker` and `lxd`; S00.13 records the owner's resolution before any worker is admitted.
4. Foundation-0 workers hold no credential and have no permitted external destination (ADR 0000 September 28, decision 4). Model access stays inside `rikurid`'s provider adapter.
5. A backend's own approval channel is not an authority (OpenShell `policy.local` proposals; ADR 0000 September 28, decision 1).
6. No silent fallback to unrestricted execution. Missing or stale enforcement makes the dependent capability unavailable and visibly degraded.
7. A last-known-good policy cannot extend an expired grant or revive a revoked credential.
8. Encryption at rest does not protect plaintext deliberately given to a running worker; forgetting covers only Rikuri-managed stores (Architecture §21.3).

## Consequences

- Protection claims in later ADRs, receipts and the assurance statement are scoped by this list; widening it is a new dated decision.
- Local tamper evidence (e.g. ledger triggers, ADR 0003) defends against accidents and ordinary code paths, not against the trusted owner account.
- The owner account's current membership of root-equivalent groups is an open item for S00.13, not a Stage 00 kernel blocker.

## Security / privacy impact

This ADR is the security baseline for Foundation-0. It introduces no protection by itself.

## Data / migration impact

None.

## Rollback or reversal path

Changing an assumption is a new dated decision that names every claim, test and receipt it affects; the earlier text stays as history.

## Follow-up tests

- S00.12 (E2) records where Pi runs and proves its built-in tools/extensions disabled or contained.
- S00.13 records controller account, group membership of the `rikurid`/Pi/model accounts, approval mode and supervisor reach.
- Stage 02 F7 and Stage 03 F9/T03.x test the untrusted-content and containment rules.
