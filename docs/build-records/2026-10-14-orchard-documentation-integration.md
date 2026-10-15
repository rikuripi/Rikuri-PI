# Orchard documentation integration — 2026-10-14

## Task Contract

- **Outcome:** Make the supplied Orchard architecture proposal discoverable in both builds while preserving maintained baselines.
- **Scope:** Routine documentation-only import, reconciliation and cross-links; no execution, schema implementation, runtime/state migration, gate changes or ADR adoption.
- **Done when:** Supplied sources have provenance; proposed ADR and annotated integration plan are indexed; Orchard roadmaps identify early contracts and later convergence; local links and whitespace checks pass.
- **Authority:** User requested integration and supplied both repositories and three documents. Local documentation writes and read-only verification; no network, commit, push, PR or worker launch. Independent review is not required for routine documentation-only work.
- **Limits:** One task; at most two repair cycles and two review rounds. No model calls or paid services.

## Reconciliation

The downloaded Architecture v1.7 is an older source snapshot, not a replacement for the maintained same-version repository document. Comparison shows missing Option C/security, context, creative-studio, mission and other dated clarifications, as well as superseded stage sequencing. Retain the maintained architecture. The incoming plan references Stage Plan v2.0, Engineering Standard v1.1 and Orchard O0; current baselines are v2.1, v1.2 and Orchard v0.3. Its Stage 1 IDE-first schedule conflicts with the adopted creative-studio sequence; the current Master Build Sequence controls. ADR 0006 remains PROPOSED and requires owner decision before adoption. The documented first work package is a candidate, not authorization to execute it.

## Evidence

Both working trees were clean before edits. Incoming architecture diff was inspected; its replacement would remove maintained decisions. This task changes documentation only. Link/path and whitespace verification results are recorded below after checks. Runtime gates, Foundation-0 closure and Orchard execution were not run or claimed.

## Deferred

Owner adoption of ADR 0006; provider-neutral TaskContract/schema and fake-adapter proof; production migration; native adapter experiments; richer IDE/UI; external execution. Existing Orchard evidence remains limited to its recorded baseline.

## Source provenance

Owner-supplied Downloads files (hashes describe original bytes):

- `Rikuri_Pi_Architecture_Spec_v1.7.md` — SHA-256 `8956957b48ccbd7e0130de7c1a70f94ff641c411883dd67552280a47b8a12a6f`
- `Rikuri_Pi_Orchard_Integration_Plan_2026-10-14.md` — SHA-256 `be780d2fd464a158e6213dfddc1533ca141cd0a0959060b9a8b98d37c7a742ed`
- `ADR_PROPOSED_Rikuri_Orchard_as_Software_Domain.md` — SHA-256 `4ccdcb3148b0948b9f405d341526154a0bd1e4b3efebcd9991ea468cca27fced`

## Verification results and disposition

- **PASS:** Python read-only Markdown local-target and heading-anchor validation across all nine changed/new documents, including existing links in modified documents.
- **PASS:** `git diff --check` in both repositories.
- Base revision `Rikuri-PI`: `13b58bdcc9c8a0a0e8219fff615d2679315d8431`; this change remains uncommitted.
- Base revision `rikuri-orchard`: `3c717939012b52a6582829fb681b97ffdb6da8cf`; this change remains uncommitted.
- CI: not run; no commit, push or PR. Runtime tests/full gate omitted for documentation-only work per Agent Rules.
- **DONE:** documentation reconciliation and discoverability. ADR adoption and implementation remain pending owner decision/separate task. No inference, worker, test process, secret access, production-state write or paid service was used.
