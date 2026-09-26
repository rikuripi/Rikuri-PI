# Rikuri Pi — Foundation-0 ADR Starter Pack

> Repository consistency revision: September 25, 2026. Source version retained; see [reconciliation record](reviews/2026-09-25-documentation-consistency.md).

**Version:** 1.0 Draft

ADRs record decisions when they are made. They are not retrospective documentation.

## ADR template

For adopt/adapt/build decisions, include the evaluated upstream project and pinned version, license, alternatives, exact mechanism, security boundary, integration and maintenance cost, reproduction steps, measured failure/recovery tests, and rollback. Candidate: Paperclip multi-agent control-plane mechanisms in Stage 6 after Foundation-0. Do not mark documented upstream behavior as locally verified.

```text
# ADR-NNNN: Title

Status: Proposed | Accepted | Superseded | Rejected
Date:
Owners:
Related requirements/tests:

## Context / problem

## Decision drivers

## Options considered

## Evidence / prototype

## Decision

## Consequences

## Security / privacy impact

## Data / migration impact

## Rollback or reversal path

## Follow-up tests
```

## Initial ADR queue

Targets follow Stage Plan v2.1, not the historical 13-week calendar. ADR numbers identify decisions, not execution order. ADR 0014 must be accepted in Stage 01 before restore acceptance. Additional Stage 03 project-trust and Stage 04 Pi/profile ADR numbers are allocated when proposed.

| ADR | Decision | Target |
|---|---|---|
| 0001 | Stable ID format | Stage 00 |
| 0002 | SQLite + current-state repository model | Stage 00 |
| 0003 | Referenced-content append-only ledger + idempotency | Stage 00 |
| 0004 | Foundation threat model / trusted-computing-base assumptions | Stage 00 |
| 0005 | Per-deletion-unit key / crypto-shredding contract | Stage 01 |
| 0006 | Import source identity and re-import idempotency | Stage 01 |
| 0007 | FTS5 baseline + VectorIndex boundary | Stage 01 |
| 0008 | Two one-way Markdown flows | Stage 01 |
| 0009 | Context Package + instruction-authority model | Stage 02 |
| 0010 | Hard-budget reservation and unknown-cost behavior | Stage 02 |
| 0011 | Egress by construction + envelope assurance levels | Stage 02 contract; Stage 03 assurance evidence |
| 0012 | Trusted approval semantics + prompt-injection containment | Stage 02 |
| 0013 | Single-window Foundation UI + i18n resource boundary | Stage 05 |
| 0014 | Backup format / restore verification / deletion-aware recovery keys | Stage 01, before F6 |
| 0015 | Architecture v1.7 freeze from Foundation evidence | Stage 05 |

## Supersession rule

Never rewrite an accepted ADR to hide a changed decision. Mark it Superseded and link the newer ADR. The history is part of Rikuri's own engineering provenance.
