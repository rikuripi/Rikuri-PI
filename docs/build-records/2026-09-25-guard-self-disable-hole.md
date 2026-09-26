# Build Record — guard: wildcard deletes of settings and fix-lock state ask

| Field | Value |
|---|---|
| Change | Protect `.claude/settings*` and `.claude/state/*` in the guard policy |
| Date | 2026-09-25 |
| Author / agent session | Claude Code |
| Branch | main |
| Commits | This change |
| Stage Gate | N/A — agent tooling |
| Related ADR / requirement | AGENTS.md > Deterministic gates |
| Status | COMPLETE |

## Scope and plan

Primary record: `rikurinode/docs/build-records/2026-09-25-guard-self-disable-hole.md`, one change across the four full-tier repositories. An independent verifier found that `rm -rf .claude/state/*` and `rm -f .claude/state/fix-*` release the fix-lock, and `rm -f .claude/settings*` deletes the hook wiring, without asking the owner. This policy already protected the exact paths (`2026-09-25-full-tier-sdlc-controls.md`) but not these wildcard forms. Success means both commands ask and the fix-lock can still be engaged.

## Wiring and verification

- `.claude/hooks/guard-config.json`: `protected_globs` gains `.claude/state/*` and `.claude/settings*`; two worked examples (78 → 80).
- Seen failing first: with the examples and without the globs, `FAILED (failures=2)`, each `'allow' != 'ask'`.
- After: Python 3.12 and 3.9, `Ran 16 tests … OK`. The shared `test_fix_lock` case (a Write of `.claude/state/fix-lock` is allowed) still passes.
- `guard.py`, `test_guard.py` and `settings.json` are unchanged and `cmp`-identical to rikurinode, Iceman and AISecurity.

## Disposition

The shared-guard gaps the verifier also found (other redirect forms, `find -delete`, `git clean`, user-level `~/.claude/settings.json`) are deferred in the primary record to the next `guard.py` change. No stage gate, help text or RAG source is affected.
