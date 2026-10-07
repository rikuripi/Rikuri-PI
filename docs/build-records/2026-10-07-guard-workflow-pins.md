# Build Record — Pin the AI-session guard workflow and limit its token

| Field | Value |
|---|---|
| Change | `.github/workflows/claude-guard.yml`: actions pinned by commit SHA, `permissions: contents: read`, checkout without persisted credentials, runner image named, stale comment corrected |
| Date | 2026-10-07 |
| Author / agent session | Claude Code session for Henry Chen |
| Branch | `ci/pin-guard-workflow` |
| Commits | fill at close-out |
| Stage Gate | [Stage 00](../stage-gates/foundation0/00-bootstrap-kernel.md), follow-up from the pre-merge verification of PR #7 |
| Related ADR / requirement | S00.02 (CI); AGENTS.md "Deterministic gates" |
| Status | IN PROGRESS |

**Risk note (protected path):** `.github/workflows/claude-guard.yml` is CI. The change narrows what the workflow can do; it changes no test and no trigger.

## 1. Plan

- [x] One-sentence statement: bring the guard workflow to the same supply-chain and token standard as `ci.yml`.
- [x] Defects (pre-merge verification of PR #7, 2026-10-06): actions pinned by tag (`actions/checkout@v4`, `actions/setup-python@v5`), so a moved tag changes what runs; no `permissions:` block, so the token's scope is the repository default, which the agent account cannot read (settings API 404); comment says the Stage 00 CI "does not exist yet".
- [x] Choices: `actions/checkout` at the SHA `ci.yml` already runs (`3d3c42e…`, v7.0.1, a major-version step from v4) with `persist-credentials: false`; `actions/setup-python` at the newest v5 release, v5.6.0 (`a26af69…`, tag resolved to commit 2026-10-07), keeping the major version so the Python 3.9 and 3.12 installs behave as before; `ubuntu-24.04` instead of `ubuntu-latest`, matching `ci.yml` and the comment's "3.12 is Ubuntu 24.04's".
- [x] **I will know this works when** the workflow, triggered by this PR (its path filter includes the workflow file), passes on both Python versions with the explicit read-only token, and the required `Full gate (Ubuntu)` passes.

## Evidence log

```text
(filled when the PR's runs complete)
```
