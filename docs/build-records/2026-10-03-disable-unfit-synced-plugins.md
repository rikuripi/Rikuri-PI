# Build Record — switch off unfit synced plugins for this repository

**Date:** October 3, 2026 · **Risk:** low, Claude Code configuration only (brief record; touches protected `.claude/settings.json` at the owner's request) · **Branch:** `claude/project-thread-e0zvgs`

## Scope

The owner added several plugins to their claude.ai account and asked whether they fit Rikuri Pi. Six do not. They load in every session as `<name>@synced` plugins from the account, not from this repository, so deleting or re-creating the claude.ai project would not remove them. This change keeps them out of Rikuri sessions in every environment. Their effect on the owner's other work is out of scope. Foundation-0 stages, S/T items and gates are unaffected.

## Plan and changes

`.claude/settings.json` gains an `enabledPlugins` block that sets these to `false`:

| Plugin | Why it is off here |
| --- | --- |
| `agentic-sdlc@synced` | A competing lifecycle: its agent writes `CLAUDE.md`, it adds its own FR/ADR/PLAN numbering and `sdlc-metadata.yml`, and its PreToolUse hook asks before every `git commit`/`push` |
| `agent-memory@synced` | PostToolUse hook stores every shell command and the first 200 characters of its output in a side SQLite database without secret filtering |
| `evermuse@synced` | Customer-interview evidence tooling; no such corpus exists and its MCP server cannot connect |
| `agent-sdk-dev@synced` | Python/TypeScript Agent SDK scaffolding; Rikuri is a Rust daemon |
| `agent-context-kit@synced` | Its own `docs/`, `how-tos/`, `scripts/` directive system would duplicate `doc/README.md`, AGENTS.md and runbooks |
| `zapier@synced` | Third-party app automation outside Foundation-0 scope |

Kept: `agent-formal-verify` (TLA+/Lean/Kani for crash-point and promotion protocols once Stage 00 code exists), `agent-protocols` (independent reviewer agents), `ai-mentor`, `engineering`, `data`, `cowork-plugin-management`. The full review is in the project file `reviews/plugin-review-2026-10-03.md`.

Mechanism: Claude Code's plugin loading reference, "Control which synced plugins load": a `"<name>@synced": false` key in a project's committed `.claude/settings.json` keeps that plugin out of the project. A plugin the organization marks as required would still load.

## Verification

- `claude plugin list --json` in a scratch repository with the same `enabledPlugins` keys reported `agentic-sdlc@synced` and `agent-memory@synced` as `enabled: false` while the rest stayed `true`: Pass.
- `claude plugin list --json` in this checkout after the change: the six listed plugins report `enabled: false`; `ai-mentor`, `agent-formal-verify`, `agent-protocols`, `engineering`, `cowork-plugin-management` and `data` report `true`. Pass.
- `python3 -m unittest discover -s .claude/hooks -p 'test_*.py'`: Pass (45 tests).
- `git diff --check`: Pass. JSON parses.
- Runtime tests: N/A, no Rikuri executable behavior changes.

## Limitations

The setting is read when a session starts, so sessions already running keep their plugins. It changes nothing on the owner's claude.ai account; the plugins still load in other repositories and in chat until removed under Customize > Plugins.

## Disposition

Delivered as a draft PR to `main`. The owner reviews and merges, because the file is protected.
