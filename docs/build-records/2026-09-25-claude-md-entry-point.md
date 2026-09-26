# Build Record — CLAUDE.md entry point

| Field | Value |
|---|---|
| Change | Add root CLAUDE.md that directs Claude Code to AGENTS.md |
| Date | 2026-09-25 |
| Author / agent session | Claude Code |
| Branch | main |
| Commits | This change |
| Stage Gate | N/A — agent configuration |
| Related ADR / requirement | N/A |
| Status | COMPLETE |

## Scope and plan

Claude Code reads `CLAUDE.md`, not `AGENTS.md`, so sessions in this repository started without the agent standard work. Add a root `CLAUDE.md` that imports `AGENTS.md` with Claude Code's `@AGENTS.md` import rather than copying it, so the rules keep one source and cannot drift. Success means a Claude Code session started in the checkout has the AGENTS.md rules in context. No application, security boundary, or stage gate changes.

## Wiring and verification

Claude Code loads the root `CLAUDE.md` at session start and expands the `@AGENTS.md` import. Verified the local link resolves, no trailing whitespace, working tree status, and the final diff. Runtime tests are N/A because only agent instructions changed.

- Real-caller check: PASS. A fresh `claude -p` session started in the checkout, with no tools, quoted working law 4 exactly and attributed it to `AGENTS.md` through the `@AGENTS.md` import.

## Disposition

AGENTS.md is unchanged and remains the only source of the rules. No stage gate, architecture document, help text, or RAG source is affected. No temporary files were created in the checkout.
