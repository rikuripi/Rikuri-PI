# ADR 0020 — Vault foundation and amendment reconciliation

**Date:** 2026-11-06
**Status:** OWNER-ACCEPTED — architectural direction accepted November 6, 2026; normal merge pending; implementation admission remains Not yet proven.
**Supersedes:** None. ADR 0000 and accepted ADRs remain authoritative.
**Owners:** Product owner; security, storage and platform implementers for affected features.

## Owner acceptance — November 6, 2026

The owner explicitly accepted ADRs 0020–0022 after independent review (PASS; no Important findings) and correction of the nonblocking Orchard wording nit. The proposed decision below is now accepted architectural direction; its original proposal wording is retained for history. Acceptance authorizes no implementation, experiment, migration, protected rebaseline or gate closure. Existing Foundation commitments remain unchanged. See the [acceptance record](../../build-records/2026-11-06-adr-0020-0022-owner-acceptance.md).

## Context and evidence

Three [unique source amendments](../../../doc/references/2026-11-amendments/README.md) and the AREX discussion refine existing promises. Existing Foundation-0 Stage 01 already requires encrypted blobs, per-deletion-unit keys, forget and managed restore; Stage 02 owns context policy and Stage 03 owns Ubuntu containment. Product Stage 1 is later capture/supporting execution, not Foundation Stage 01. No gate result changes.

## Conflict dispositions

| Source wording or conflict | Reconciled disposition |
| --- | --- |
| Vault as first/Stage-1 foundation | Keep encryption/forget/recovery in Foundation Stage 01; add proposed product Stage 1 readiness refinements before expanded ingestion. Never defer existing protection to product Stage 1. |
| SQLCipher plus per-vault keys | Candidate only. ADR 0002 SQLite and Stage 01 XChaCha20-Poly1305 per-deletion-unit DEKs/BLAKE3 remain. Per-vault wrapping must preserve deletion-unit erasure. Whole-DB metadata encryption requires separate measured migration/key/FTS/restore decision, not an assumed replacement. |
| Knowledge Broker commits state; AISecurity owns authority | Broker is a governed interface inside Rikuri ownership: only rikurid commits canonical state; owner authorizes, trusted AISecurity policy evaluation informs enforced decisions. Detectors and LLMs are not the root decision authority. |
| Markdown is a view | Preserve external user Markdown as authoritative for those notes and Git as source-code authority; generated projections remain one-way and preserve edited versions. |
| Stage 1 A/B/C/D blockers and two-user tests | Proposed product Stage 1 admission refinements, not new Foundation gates or a claim of shipped multi-user UI. Existing Foundation custody and admission requirements continue to govern its imports. The additional principal/vault/space scope model and simulated multi-user proofs are proposed product Stage 1 prerequisites before expanded ingestion; they do not add a Foundation gate. Full family membership/key management remains Stage 6/7. |
| Mesh source Stages 2/3/4 | Existing product Stage 2 creative studio stays; remote Link Stage 3, evaluation/routing Stage 4, family governance Stage 6, replication/resource catalog Stage 7, advanced multimedia Stage 8; learned adaptation Stage 10. |
| Source security S0–S5 and experiments E1–E6 | Source-local labels only; not Foundation 00–05, existing E1–E5, or authorization to run experiments. Credential/network-permitted workers stay deferred until matching broker/destination qualification passes. |
| Docker first / optional no-isolation descriptors | OpenShell remains first Ubuntu candidate under Option C; no technology default or assurance downgrade adopted. `none` describes lack of containment and cannot admit protected work. |
| Private / Trusted Devices | Routing policy, not identity or grant; secret material stays out of every model context. Local Only/Vault retains the existing requirement for a proven no-network path; full Rikuri integration and qualification remain Not yet proven; separate compute endpoints require distinct authorized privacy policy. |
| New source references to absent Security/Stage-0/Threat Model files and Engineering v1.1 | Maintained PRD v1.2, Architecture v1.7, Engineering v1.2, Stage Plan v2.1 and docs/security/THREAT_MODEL.md control; do not create competing authorities. |
| Starter ADR queue 0005/0006 versus actual ADR filenames | Existing queue and actual accepted ADRs already diverge. Preserve history and do not overwrite either; allocate 0020–0022 for this proposal. Reconcile outstanding queue IDs explicitly when that work is authorized. |

## Proposed decision

Adopt stable principal/vault/space/project/thread/object ownership and denial-first broker semantics before expanded personal ingestion, with scoped derived stores, encrypted recovery, model/device portability and transparent privacy destinations. A singleton owner must be an explicit stable principal, never inferred from a provider/email/device. Use existing typed-ID rules; UUID strings in supplied examples are illustrative, not an ADR 0001 replacement.

Do not automatically backfill unknown owners, collapse project scopes, or migrate prototype data. A migration must stage candidates, validate identity/permissions/checksums/history, durably write/read back and atomically promote with rollback. Managed restore consults current revocations; absent current state refuses/quarantines. Export cannot recall already disclosed copies.

## Alternatives and consequences

Appending proposals without reconciliation creates duplicate authority; reject. Replacing SQLite/crypto now lacks migration evidence; defer. Keeping ownership until family sharing creates data debt; prefer explicit design now, implemented only under scoped stage work. The additional product Stage 1 proof costs engineering time; estimate it after owner adoption, without changing Foundation dates or creative-studio priorities.

## Acceptance and reversal

[Acceptance strategy §21](../../../doc/Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md#21-november-6-amendment-acceptance) specifies VLT cases, all Not yet proven. Reject or supersede this proposal by dated ADR; retain originals and accepted baseline. No production data rollback is required for these docs.
