# Rikuri Pi personal-data threat model

**Status:** adopted design requirements, September 28, 2026; runtime protection **Not yet proven**. This is the companion required by [Architecture §27.1](../../doc/Rikuri_Pi_Architecture_Spec_v1.7.md#271-foundation-threat-model). The [Stage Plan v2.1](../../doc/Rikuri_Pi_Foundation0_Rebaselined_Stage_Plan_v2.1.md) owns implementation order and gate status; [ADR 0000](../../doc/Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md#september-28-2026--option-c-and-personal-data-protection) records the Option C decision.

## 1. Highest responsibility

Rikuri Pi runs where people keep material they cannot afford to expose or lose. Protecting that material is the highest product responsibility. The product must prevent unauthorized disclosure and changes, limit damage from mistakes or compromised agents, and provide verified recovery where promised. Useful automation proceeds only inside demonstrable protection. Missing required protection makes the dependent capability unavailable.

This is an engineering obligation, not a promise that every computer or attack is covered. Trust must be earned through supported boundaries, repeatable tests, understandable permissions and independent review. We aspire to excellent comparative reliability; we do not claim a ranking without evidence.

## 2. Assets and permitted use

| Asset | Required protection |
| --- | --- |
| Identity, private correspondence, health/family information and other personal records | Minimize collection and context; scope source and recipient; refuse unauthorized external processing or sharing. |
| Financial records, tax documents, account information and transactions | Separate read/analysis from export, account administration, trading, payment and signing. Spending budgets do not grant access to funds. |
| Personal files, photos, audio and videos | No whole-library access by default. Protect originals, location/EXIF metadata, faces, OCR, transcripts and thumbnails under the same applicable restrictions. |
| Professional/proprietary databases, documents, source code and customer records | Scope projects, objects, rows/columns and operations at trusted services; read/query permission does not imply bulk export, write or administration. Respect audience and organization restrictions. |
| Passwords, API/OAuth tokens, SSH/signing keys, wallet seeds and recovery keys | No general model or worker access. Broker only scoped operations; isolate financial/signing authority and prove expiry/revocation and secret-free outputs. |
| Canonical state, grants, audit, budgets, backups and recovery material | Govern mutations, protect integrity, retain required evidence without secret payloads, and restore using current revocation. Workers cannot alter their own authority. |
| Derived and temporary material | Apply source restrictions to embeddings, summaries, prompts, logs, caches, crash dumps, exports, revisions and backups. A transformed copy is not automatically safe to disclose. |

Use the PRD's **Normal / Private / Local Only / Secret** classes. Unknown content receives conservative private/local treatment until an authorized classification decision. Source or model text cannot lower restrictions. Secret detection supplements access controls; its false negatives cannot be the sole boundary. Do not scan or index arbitrary personal directories just to classify them without scoped permission.

## 3. Threats and trusted components

Assume imported files, repositories, web pages, prompts, model output, tool results, local model processes, extensions and external networks can be hostile. Include accidental edits, unintended uploads, confused-deputy credential use, malicious dependency hooks, descendants escaping a parent, resource exhaustion, stale approvals and stopped security services. Network destinations allowed for legitimate work can still receive stolen data.

Foundation-0 trusts the host kernel and authenticated owner account. Root/admin compromise, a compromised host kernel/hypervisor, hardware/firmware compromise and malicious actions by the owner are outside its guarantee. The product must still constrain its own workers even when the user session has broad access. Unrelated native applications and all activity elsewhere on the computer are not automatically covered. Operating-system and device security remain dependencies.

The trusted computing base includes Rikuri's authorization/storage services, the AISecurity policy service, credential brokers, the admitted enforcement backend and its privileged runtime/driver dependencies. Keep this base small and outside worker mutation authority; authenticate service calls, validate scope and prove IPC/mount/process isolation. A different process name is not a security boundary.

For the OpenShell candidate the base concretely includes the gateway, the supervisor container (host networking; the mediated egress point) and the container engine. Access to a rootful engine's socket is root-equivalent, as is membership of groups such as `docker` or `lxd`. The backend controller therefore runs under a dedicated service account or a rootless engine, and the accounts that run `rikurid`, Pi, local model servers and tool servers stay outside those groups; otherwise any same-user process could go around every envelope. The Pi harness is itself untrusted-input processing: it runs inside the envelope, or with its built-in tools and extension loading disabled, and it calls models only through `rikurid` ([Architecture §13.1](../../doc/Rikuri_Pi_Architecture_Spec_v1.7.md#131-pi-backed-governed-workers)).

The owner authorizes scope and consequences. Untrusted content cannot grant permission, change approval text, weaken verification, remove a budget ceiling or promote generated advice to canonical authority. Inspecting a project does not authorize executing its metadata or hooks.

## 4. Option C boundaries

1. **Rikuri decides authority.** Canonical intent, privacy, project trust, grants, budgets and revocation stay under governed services.
2. **AISecurity evaluates and translates.** Trusted policy evaluation emits a versioned plan within the current grant; detectors only contribute signals.
3. **Qualified adapters enforce.** OpenShell is the first Ubuntu candidate. Bind actual enforcement to the plan and execution identity before admitting data. Unknown or stale assurance refuses the dependent operation. The backend's own approval features are not an authority: OpenShell's in-sandbox proposal route and automatic approval are disabled or routed to Rikuri as untrusted requests.
4. **Optional independent monitoring adds coverage.** Sentry's public design is a research input. Hardware independence is not implied by a local daemon, and no DPU is required for Foundation-0.

The detailed contract is [Architecture §27.12](../../doc/Rikuri_Pi_Architecture_Spec_v1.7.md#2712-option-c-policy-ownership-and-enforcement-adapters). There is no silent fallback to unrestricted execution. A last-known-good policy cannot extend an expired grant or revive revoked credentials. Stop/revoke must fence active work and descendants within a tested limit without model cooperation.

Local Only requires refusal before external request construction plus a proven no-external-network worker profile. Secret material never enters general model context. An endpoint allowlist alone cannot protect private data given to a network-capable worker. Broker and control-plane traffic, telemetry, crash reporting and diagnostic exports require separate admission and observation.

## 5. Threat, control and acceptance mapping

This table points to existing stage obligations; it does not claim their implementation. [Test Strategy §8.2](../../doc/Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md#82-option-c-qualification-and-personal-data-acceptance) supplies the detailed backend qualification cases.

| Threat | Required control | Owning acceptance |
| --- | --- | --- |
| Prompt/project injection gains execution or authority | Data/instruction separation, scoped trust promotion, trusted consequence rendering, descendant containment | Stage 02 F7; T03.06–11; F9 |
| Worker reads or changes unrelated personal files | Least mounts/operations, process and service isolation, protected originals, allowed/denied controls | SEC-020/021/023; T03.02/05/09/10; Stage 04 governed edits |
| Private data leaves through models or allowed services | Context admission, proven offline profile, scoped provider paths, actual sink observation | F4-lite; T03.03/04/14/16; Stage 04 full F4 |
| Credentials stolen or broker used for a different target | External broker, identity/endpoint/operation binding, expiry/revocation, canary tests | SEC-005/022; T03.05/09/16 before enabling each credential path |
| Financial read turns into signing/payment/trading | Separate authority and dedicated trusted procedures; no implicit custody | SEC-005/019/022; future financial feature admission under the Master Build Sequence |
| Revocation or security-service failure leaves an agent active | Bounded enforcement independent of worker, descendant fencing, visible degradation, current-state recovery | S03.08/11; T03.04/12/15; SEC-006/007/021 |
| Private information leaks through derived stores or diagnostics | Inherit source restrictions, minimize logs, telemetry off by default, scoped diagnostic sharing | Stage 01 storage; T02.16/T03.16; later media/connector feature gates |
| Corruption/deletion or old backup restores forbidden data | Validated durable changes/read-back, protected originals, tested restore with current revocation | W1/F6; Stage 01 and eight-point release crash matrix; later feature-specific recovery |
| Worker widens its own policy through the backend's proposal/approval feature | Approval mode pinned manual at gateway scope and verified at launch; worker proposal route blocked or routed to Rikuri as an untrusted request; OpenShell approval surface unused | S00.13; S03.11; T03.05 |
| Rikuri process reaches the container engine and bypasses every envelope | Dedicated controller account or rootless engine; Rikuri/Pi/model/tool accounts outside root-equivalent groups; supervisor host reach tested | S00.13; S03.11; Test Strategy §8.2 backend admission |
| Harness itself acts on hostile input with host authority or its own credential | Pi inside the envelope or built-in tools/extensions disabled; model calls only via the `rikurid` endpoint with admission and budget; no provider key in Pi | E2/S00.12; S04.02; T04.05; F4/F5 |
| Backend change or misleading status overstates protection | Version-bound qualification, positive/negative/failure tests, current evidence and independent review | S00.13; T03.05/12/13/16; Stage 05 and public release gate; SEC-024 |
| CPU/memory/disk/process loops harm computer availability | Task limits and tested resource controls, bounded cancellation and recovery | S03.02/08, T03.15/16; later workload-specific admission |

Use synthetic data. Include legitimate success as well as refusal; a broken fixture or system that blocks everything is not successful protection. Run controlled failure drills in disposable environments. Report measured outcomes and missing coverage honestly.

## 6. Current evidence and limits

The [OpenShell v0.1.2 receipt](evidence/2026-09-28-openshell-v0.1.2/README.md) records ten expected smoke outcomes on this Linux host. It supports continued qualification, not production trust. Credential brokering, controller failure, active revocation, DNS/IPv6, permitted OpenShell egress, full control-plane egress and Rikuri integration are unproven. No Foundation gate passed from that test. The run also auto-drafted one network proposal (its later status was not captured), logged an unexplained provider-environment settings change with no providers attached, and denied network through both container `network_mode: none` and OpenShell policy; qualification must explain the first two and test the configuration Rikuri will actually ship. The receipt directory is an archive covered by `SHA256SUMS` and is not edited; these notes live here.

Foundation-0 enables no credential-bearing or network-permitted worker ([ADR addendum](../../doc/Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md#september-28-2026--option-c-review-follow-ups-and-foundation-0-credential-scope)). Broker and permitted-destination rows become mandatory when a later stage first enables such a path.

Existing AISecurity detectors, policy vocabulary, audit and vault components are reuse candidates. The inspected Linux/non-macOS launcher has an unsandboxed path and cannot serve as a qualified fallback. macOS scaffolding does not prove native protection. Ubuntu is the first full reference; native macOS remains experimental, and Linux-in-a-VM evidence does not qualify native desktop access.

Managed retention and forgetting cover admitted Rikuri-controlled stores and restore paths. They cannot revoke already disclosed external copies or guarantee erasure of unmanaged originals/exports. Encryption at rest does not prevent an authorized running process from reading exposed plaintext. State these limits before users entrust sensitive material.

## 7. Maintaining trust

Before enabling a sensitive feature, link its assets, permissions, platform/configuration, observed boundary/failure/recovery evidence and remaining limits. Explain data access and destinations in ordinary language, with a usable stop/revoke path. Security health must reflect actual required protection; a live daemon alone is insufficient.

Follow the [Engineering Standard](../../doc/Rikuri_Pi_Engineering_Standard_v1.2.md#security-assurance-and-upstream-maintenance) for upstream updates, incident handling, assurance statements and independent review. Keep private payloads out of public test receipts. Benchmark both attack refusal and useful authorized work before making comparative claims. Reassess this model whenever a feature introduces new data, trust boundaries, recipients, credentials or platform authority.
