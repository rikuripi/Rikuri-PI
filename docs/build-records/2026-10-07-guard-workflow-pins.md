# Build Record — Pin the AI-session guard workflow and limit its token

| Field | Value |
|---|---|
| Change | `.github/workflows/claude-guard.yml`: actions pinned by commit SHA, `permissions: contents: read`, checkout without persisted credentials, runner image named, stale comment corrected |
| Date | 2026-10-07 |
| Author / agent session | Claude Code session for Henry Chen |
| Branch | `ci/pin-guard-workflow` |
| Commits | `70583ad` (workflow and records), close-out commit; PR #14 |
| Stage Gate | [Stage 00](../stage-gates/foundation0/00-bootstrap-kernel.md), follow-up from the pre-merge verification of PR #7 |
| Related ADR / requirement | S00.02 (CI); AGENTS.md "Deterministic gates" |
| Status | COMPLETE (2026-10-07; verifier PASS; merged with PR #14) |

**Risk note (protected path):** `.github/workflows/claude-guard.yml` is CI. The change narrows what the workflow can do; it changes no test and no trigger.

## 1. Plan

- [x] One-sentence statement: bring the guard workflow to the same supply-chain and token standard as `ci.yml`.
- [x] Defects (pre-merge verification of PR #7, 2026-10-06): actions pinned by tag (`actions/checkout@v4`, `actions/setup-python@v5`), so a moved tag changes what runs; no `permissions:` block, so the token's scope is the repository default, which the agent account cannot read (settings API 404); comment says the Stage 00 CI "does not exist yet".
- [x] Choices: `actions/checkout` at the SHA `ci.yml` already runs (`3d3c42e…`, v7.0.1, a major-version step from v4) with `persist-credentials: false`; `actions/setup-python` at the newest v5 release, v5.6.0 (`a26af69…`, tag resolved to commit 2026-10-07), keeping the major version so the Python 3.9 and 3.12 installs behave as before; `ubuntu-24.04` instead of `ubuntu-latest`, matching `ci.yml` and the comment's "3.12 is Ubuntu 24.04's".
- [x] **I will know this works when** the workflow, triggered by this PR (its path filter includes the workflow file), passes on both Python versions with the explicit read-only token, and the required `Full gate (Ubuntu)` passes.

## Evidence log

```text
# PR #14 at 70583ad
AI-session guard run 37570295563: success
  GITHUB_TOKEN Permissions: Contents: read, Metadata: read · Image: ubuntu-24.04
  actions downloaded by commit SHA · checkout persist-credentials: false
  Guard policy tests (py3.9, CPython 3.9.25): Ran 91 tests, OK
  Guard policy tests (py3.12, CPython 3.12.14): Ran 91 tests, OK
Full gate (Ubuntu) run 37570295642: RESULT: PASS (13 steps)
Verifier (2026-10-07): PASS — both SHAs resolve to their tags (v5.6.0 is the newest v5 and the floating v5 tag points to it); YAML otherwise unchanged versus main
```

- **Known warning, recorded:** the run logs `Node.js 20 is deprecated … actions/setup-python@a26af69… [is] forced to run on Node.js 24`. Staying on setup-python v5 (to keep the Python installs as before) brings this warning; v6.3.0 and v7.0.0 exist. Revisit when the guard workflow next changes: move to the newest major after checking that its Python 3.9 and 3.12 installs still pass. Owner: Henry Chen.
- **Flaky guard test seen on `main` (2026-10-07):** after PR #13 merged, `Full gate (Ubuntu)` on `2b42f61` (run 37571810017) failed in `test_guard.GuardRound2.test_round5_many_cds_and_searches_stay_fast`: `4.129990339279175 not less than 4.0`. The test asserts a wall-clock limit (locally 0.10 s); a re-run of the same commit passed (91 tests, `RESULT: PASS (13 steps)`). Not caused by any change here. The fix belongs in the shared `test_guard.py`, which is changed only in rikurinode and copied unchanged to every full-tier repository (owner's rule), so it is recorded as a follow-up in the [Stage 00 record](../stage-gates/foundation0/00-bootstrap-kernel.md), not edited here.
