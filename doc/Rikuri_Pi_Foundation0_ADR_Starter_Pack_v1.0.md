# Rikuri Pi — Foundation-0 ADR Starter Pack

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

| ADR | Decision | Target |
|---|---|---|
| 0001 | Stable ID format | Week 1 |
| 0002 | SQLite + current-state repository model | Week 1 |
| 0003 | Referenced-content append-only ledger + idempotency | Week 1 |
| 0004 | Foundation threat model / trusted-computing-base assumptions | Week 1 |
| 0005 | Per-deletion-unit key / crypto-shredding contract | Week 2 |
| 0006 | Import source identity and re-import idempotency | Week 3–5 |
| 0007 | FTS5 baseline + VectorIndex boundary | Week 3–5 |
| 0008 | Two one-way Markdown flows | Week 3–5 |
| 0009 | Context Package + instruction-authority model | Week 6 |
| 0010 | Hard-budget reservation and unknown-cost behavior | Week 7 |
| 0011 | Egress by construction + envelope assurance levels | Week 8 |
| 0012 | Trusted approval semantics + prompt-injection containment | Week 9 |
| 0013 | Single-window Foundation UI + i18n resource boundary | Week 10–11 |
| 0014 | Backup format / restore verification / deletion-aware recovery keys | Week 12 |
| 0015 | Architecture v1.1 freeze from Foundation evidence | Week 13 |

## Supersession rule

Never rewrite an accepted ADR to hide a changed decision. Mark it Superseded and link the newer ADR. The history is part of Rikuri's own engineering provenance.
