# Build Record — Linux ignore rules

| Field | Value |
|---|---|
| Change | Ignore additional Linux generated files |
| Date | 2026-09-25 |
| Author / agent session | Codex |
| Branch | main |
| Commits | This change |
| Stage Gate | N/A — repository hygiene |
| Related ADR / requirement | N/A |
| Status | COMPLETE |

## Scope and plan

Review existing ignore rules and tracked files, then add narrow rules for Linux temporary files and crash dumps. Success means Git ignores representative generated files while a normal source document remains visible. No application, security boundary, or stage gate changes.

## Wiring and verification

The root `.gitignore` applies throughout the checkout. Verified representative paths with `git check-ignore`; checked `git diff --check`, working tree status, and the final diff. Runtime tests are N/A because only ignore rules changed.

## Disposition

No existing tracked files were removed. No stage gate, architecture document, help text, or RAG source is affected. No temporary files were created in the checkout.
