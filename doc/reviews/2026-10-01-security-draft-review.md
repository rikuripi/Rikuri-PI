# Security Architecture v1.0 — comparison and disposition

**Date:** October 1, 2026. **Result:** compatible additions incorporated with corrections; supplied draft not adopted wholesale. This is a design review, not an AISecurity code audit or an enforcement receipt.

**Source:** `Rikuri_Pi_Security_Architecture_Spec_v1.0.md` from the owner's Downloads directory; SHA-256 `45a7f3852ab04836fb3cb853e9d28231ac576745ce7500aeeb34c26c3ea4a216`. Original file unchanged. Draft backlog/checklists are source content, not authority to run or adopt them.

**Compared against:** [PRD v1.2 §10](../Rikuri_Pi_PRD_v1.2.md#10-security-and-trust-requirements), [Architecture v1.7 §27](../Rikuri_Pi_Architecture_Spec_v1.7.md#27-security-and-trust-architecture), §19.3, §16 and §38.6; [live Stage Plan v2.1](../Rikuri_Pi_Foundation0_Rebaselined_Stage_Plan_v2.1.md); [Master Build Sequence](../Rikuri_Pi_Master_Build_Sequence_v1.1.md); and [Engineering Standard](../Rikuri_Pi_Engineering_Standard_v1.2.md). September 30 creative-studio refinements remain intact. No standalone security spec exists in the repository; the detailed architecture is the comparison baseline.

## What already agrees

The draft's §§3–6, 8–12, 14–15, 19, 21–23 and 27–30 largely reinforce existing capability containment, model/content untrustworthiness, policy outside worker authority, trusted approvals, identity attribution, Ubuntu containment, Local Only egress by construction, Tailscale-first Link, sequence signals, evidence and transactional state. Architecture §§27.8–11 already define assurance, independent boundary hardening and the regression matrix. Avoid treating those restatements as missing work or new implemented guarantees.

## Findings and reconciliations

| Finding / review pass | Draft location | Conflict or missing guarantee | Disposition in maintained docs |
| --- | --- | --- | --- |
| **Important — Security & boundaries:** Secret and privacy vocabulary | §§7.4, 8, 20, 24 | Combined Vault/Secret with local processing and expert overrides could admit credentials into local model context. Public/Private/Confidential/Secret also does not map exactly onto existing execution privacy schema. | SEC-026/028/030 and Architecture §27.14 retain Secret exclusion from **all** models; Local Only is distinct. New labels do not grant egress. Explicit trusted declassification cannot expose Secret to models, and step-up authentication does not override hard policy. |
| **Important — Security & boundaries:** destination access versus data authority | §§4.6, 11, 13, 15, 17 | Hostname allowlists and tailnet membership cannot enforce payload confidentiality against a worker with plaintext and arbitrary permitted networking. Copies can lose path protection; external plaintext cannot be reliably revoked. | SEC-026/029 specify mediated movement/payload/recipient checks, label propagation, bypass tests and refusal where assurance cannot be enforced. Persistent copy protection and forget claims are limited to governed storage/lifecycle. |
| **Important — Bugs / Security & boundaries:** transactional external enforcement and revocation | §§13.4, 16, 18, 29 | Canonical atomic promotion does not atomically activate remote firewall rules or expire existing sockets. Restoring old known-good policy can revive revoked access. | SEC-027/029/031 require effective-state verification, restrictive transition/isolation, failed-activation recovery and current revocation precedence. Lease expiry is distinguished from upstream token/session invalidation. |
| **Important — Compliance:** competing sequence and deferred foundation protections | §§31–32, 39 | Draft S1 containment → S2 provider gate and S7 knowledge encryption/forget diverge from live Foundation provider gate at 02, envelope at 03, ownership/crypto/recovery at 01. S0–S10 resembles product numbering but is different. | Phase taxonomy retained only as proposed topics, mapped below. Do not adopt its build queue, create competing gates or postpone existing controls. No new stage or protected-policy change. |
| **Important — Compliance:** unsupported assurance / evidence assumptions | §§2, 17.3, 26, 35, 40–41 | Draft descriptions of AISecurity matrix outcomes and desired capabilities are not inspectable receipts here; threat-model path is absent. New Unknown state and absolute immediate revocation require an explicit contract. | SEC-030/032 define unverified assurance without silently changing existing health enum. Architecture corrects its nonexistent threat-model claim. Matrix-specific copy behavior remains reported-by-draft until raw evidence is reviewed. All runtime claims Not yet proven. |

The review finds useful incremental detail rather than a need to replace the architecture. SEC-025–032 explicitly cover control/enforcement separation, movement rights, secret brokerage, provider evidence, network capability lifecycle, authentication/health UX, real policy activation and lifecycle assurance. Their [future acceptance](../Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md#17-security-draft-addition-acceptance) includes legitimate, rejected, bypass and broken-control paths.

## Scope and sequence mapping — no second roadmap

This table is traceability to existing stages, not another execution checklist. Owner for deferred topics is the product owner plus implementer of the named stage; reactivate only after predecessor gates and a component-specific contract/evidence plan. Unknown versions/licenses/provider guarantees must be resolved before adoption.

| Draft topic | Existing gate / delivery position | Deferred acceptance / impact |
| --- | --- | --- |
| S0 constitution/contracts | Foundation 00 identity/events; 02 security/policy contracts | Detailed threat-model companion remains planned; its creation must reconcile current sources, not grant authority |
| S1 filesystem/process, S2 provider, S3 network | Foundation 02 provider admission; 03 Ubuntu process/no-network proof; 04 integrated local path | Preserve actual order; richer per-destination networking after Link 3 and governed delegation 6 |
| S4 secret broker | Foundation credential exclusion/one trusted cloud adapter; richer broker product 5 | Broker conformance before enabling credential-bearing connectors; unsafe raw-secret paths refused |
| S5 identity/delegation | Foundation IDs/scoped capabilities; product 6 governance | Prove non-escalating delegation, current revocation and attributable effects |
| S6 private networking | Product 3 Tailscale-first Link; richer service/Pi network delegation 6 | Verify installed rules, application rights, expiry and loss-of-controller behavior; no selected doxx.net/Headscale replacement |
| S7 knowledge protection | Foundation 01 encryption/forget/restore; 02 classification; product 5/6 movement, 7 replication | Foundation protection cannot wait for late DLP; unmanaged exports remain outside forget guarantee |
| S8 behavioral security | Foundation 03 deterministic sequence signals; later governed richer signals | Learned anomaly research follows evidence/privacy/governance prerequisites; no detector can grant rights |
| S9 teams/business | Minimum multi-human controls product 6; replication 7; enterprise/attestation 9 | Preserve S6.C1–C4; no new dependency on enterprise delivery for basic sharing |
| S10 continuous improvement | Engineering Standard from bootstrap and each release | Existing security CI/incident/update obligations; never defer basic vulnerability response until product Stage 10 |

## External claims checked October 1

[NIST SP 800-207](https://csrc.nist.gov/pubs/sp/800/207/final) supports resource-focused authorization without implicit trust from network location. [OWASP agent security guidance](https://cheatsheetseries.owasp.org/cheatsheets/AI_Agent_Security_Cheat_Sheet.html) supports treating tools/content/agents as attack surfaces with least privilege and independent controls. These are design references, not certification.

[Tailscale access-control documentation](https://tailscale.com/docs/features/access-control) describes denied connections unless explicitly permitted by policy and recommends grants. This is not proof that an installed tailnet has narrow rules; review its effective policy and test actual traffic. Tailscale connectivity is not a host-wide Internet exfiltration boundary. Tailnet Lock, device attestation, doxx.net maturity/audit, Headscale operational cost and redistribution/bundling terms remain implementation-time evaluations; no claims or dependency selections made here.

## Evidence limits and next work

The raw AISecurity Protection Matrix PDF and AISecurity source were not supplied for this review. Existing repository assessments are retained, but the draft's specific copy finding is not independently verified. No code, live firewall, provider contract, secret broker or platform probe was exercised. Do not call these additions delivered or advertise their protection. [Build Record](../../docs/build-records/2026-10-01-security-draft-review.md) records document verification; the live Stage Plan continues to govern implementation.
