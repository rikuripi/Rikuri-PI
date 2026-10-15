# Ubuntu standard-validator qualification — 2026-10-14

## Task Contract

- **Outcome:** Prove the 11 standard-validator qualification cases on Ubuntu with exact compatible dependencies.
- **Scope:** High-risk scoped fixture environment only: unchanged reviewed root namespace setup, immutable inspected fixture/dependency snapshot, then non-root qualification tests. No real workers, global settings/install, full gate, canonical migration or production adapter.
- **Done when:** CPython3.14/ARM64-compatible five-wheel set verified against official PyPI; exact staged allowlist/hashes; existing safety admission checks pass before package imports; package identity/path checks; 11 qualification tests pass (including original25 under standard shape engine); synthetic entry-point outcomes correct; globals unchanged and staging cleaned; independent source review and evidence retained.
- **Authority:** Owner said Ok to the named next Ubuntu qualification task after approving scoped root setup. Only named official PyPI package metadata/wheel fetches on host; guest has no network. No commit/push/PR/merge/deploy.
- **Limits:** One fixture task; one verifier and at most two review rounds/two repairs. No paid services, real model or follow-on task.

## Plan and risk

Reuse the previously tested launcher byte-for-byte. Extract inspected Linux wheels as inert regular files into a guest-local root-owned immutable snapshot, not system site-packages. The admission probe runs as henry without groups/capabilities/privilege gain and with shares/socket paths masked; only afterwards does it add the immutable vendor path and import the standard validator/native extension. No install/build hooks or .pth files. Exact package versions and imported paths must match. A native official wheel is included, but no binary supply-chain certification or hostile-worker sandbox claim follows.

The staging helper has a fixed hash-bound file allowlist, rejects duplicate/extra/nonregular entries, checks bounded sizes before writing and removes only its freshly created guest directory. Kernel protection values remain1. Tests retain exact existing bodies; the standard entry point runs only synthetic in-memory fixtures.

## Deferred

Full Rust/Ubuntu gate, current Orchard application regressions, real adapters, hard production budgets/replay, hostile-worker containment, public security and Foundation-0 gates.

## Actual verification and review

- Official PyPI hashes verified for all five Linux wheels. Four pure wheels are identical to prior inspected copies; rpds-py native wheel matches cp314/aarch64. No binary reproducible-build/security certification.
- Parent exact 143-file manifest/archive limits/source equality checks PASS; reviewed namespace launcher unchanged.
- Independent source review PASS, no Important findings. Sole Nit: check global values even after timeout/error. Moved comparison into finally with nested cleanup; bounded follow-up confirmed corrected. No third review round or execution repair.
- Approved guest helper exit0: all admission and dependency-origin/version checks PASS, then 11 qualification tests in1.434s, OK. Includes original25 run under standard shape checks; no skips.
- Standard entry-point outcomes DONE/BLOCKED/UNVERIFIED/NEEDS_OWNER_DECISION correct.
- Global restriction values both1 before/after; guest staging removed. No system packages installed; guest test namespace had no network.
- [Exact output](evidence/2026-10-14-ubuntu-validator/test-output.txt), [manifest](evidence/2026-10-14-ubuntu-validator/manifest.json), [wheel provenance](evidence/2026-10-14-ubuntu-validator/wheel-provenance.json).
- Local links, evidence JSON and git diff --check verified at close-out. CI/full gate not run.

## Disposition

DONE for Ubuntu finite-corpus standard-validator fixture. User-approved scoped root setup was reused; original unprivileged admission remains denied. Not a hostile-worker/production security, real adapter or Foundation-0 gate claim. Remaining full Rust/Ubuntu gate and Orchard regressions are Not yet proven. No secrets, model calls, canonical migration or Git publication. Prior uncommitted changes preserved.

Next recommended task, not started: real provider-neutral adapter interface and evidence handoff.

## Current qualified source hashes

- validate.py — SHA256 9cad85b7d2175fbf5262c9b32cd156ae5c4038bb91af8e1f517dd5c12885d39a
- standard.py — SHA256 8d7201635d3786597c6dbd9d9bfa31297ca7953d0cf252130a2e3de096d2e436
- task-contract.schema.json — SHA256 06996df38c65c0486a1b1c461f0e4c215abab6ab46fcb8a5b3d346c45c710a88
- task-result.schema.json — SHA256 59dfb56f9c62cc47d29d5fca9cbd9d3ad86ca86af59a31aaa17f332d9ac8813b
- qualification_tests/test_standard.py — SHA256 3dcb024c6f24f83699eb4f0ec15c37312afef24c427d8d8980eba13872193bd8
- qualification-requirements-linux-aarch64-cp314.txt — SHA256 276f0f6324c60b0c71c59857f27a38f1ad1304d95297e8b9a14429a27014958b
- qualification-wheel-provenance-linux.json — SHA256 c7ff4fdee1da57e4c7dab600c409c82226b54f3bd04e381c89507a827f5d6013
