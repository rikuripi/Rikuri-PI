# TaskContract standard-validator qualification — 2026-10-14

## Task Contract

- **Outcome:** Demonstrate that TaskContract v1 shapes validate consistently with a maintained JSON Schema Draft 2020-12 implementation.
- **Scope:** Routine isolated qualification tooling/tests and contract schema corrections only if exposed by adversarial cases; documentation, dependency lock/provenance and this evidence. No real adapters, kernel/production caller, credentials, state migration or stage/gate changes.
- **Done when:** Both schemas pass standard meta-schema checks; positive examples and fake receipts pass; malformed/unauthorized/false-DONE cases are rejected with shape versus cross-field distinctions; checker discrepancies are documented/corrected; existing 25 tests remain green; independent verifier and link/whitespace checks pass.
- **Authority:** User selected the first recommended next step (“1”). Install a pinned standard validator only into disposable chat work/ environment from official PyPI wheels, inspect metadata/source/provenance first, no global install. Preserve prior authorization to run only pure contract tests under normal Codex shell sandbox; containment and full Ubuntu gate remain unproven. No commit, push, PR, merge, real execution or production data writes.
- **Limits:** One task and one independent verifier; two repair cycles/two review rounds maximum; no paid inference or unrelated work.

## Plan and risk note

Use official python-jsonschema 4.26.0, MIT, stable package with documented Draft 2020-12 support. Wheel-only download is inspected before offline installation. Record exact dependency versions, wheel SHA-256 and relevant source-side effects. The validator is a qualification dependency, not a production runtime choice. Schema and cross-field authority remain distinct; valid JSON grants no rights. Remote reference retrieval must refuse and use no network during checks.

Existing in-memory fixture code is routine design scaffolding; this adds no production enforcement path. Uncommitted prior-task documentation is preserved. The initial sandbox wheel download failed due DNS (not an absent package); official PyPI access is requested through normal escalation, not another registry.

I will know this works when the valid mean contract and all four retained receipts pass their shape checks, invalid shapes and false-DONE receipts fail, cross-field violations are still refused, and a standard-validator-backed fixture run preserves all existing outcomes.

## Deferred

Real native-adapter interface and execution; authentication/revocation; OS containment; live budgets; durable replay; importer; full Ubuntu gate; global JSON Schema conformance certification.

## Red evidence before repair

Normal shell sandbox, owner-authorized pure contract scope: PYTHONPATH=<contract>:work/schema-qualification PYTHONNOUSERSITE=1 PYTHONDONTWRITEBYTECODE=1 work/schema-venv/bin/python -B -m unittest discover -s work/schema-qualification/tests -v.
Exit 1; 11 tests run, 2 failures:
- test_check_identity_rejects_trailing_newline: AssertionError: True is not false
- test_digest_rejects_trailing_newline: AssertionError: True is not false

Both standard-validator checks accepted a trailing newline before end-anchor. Nine other qualification tests passed, including both schema meta-checks, offline-reference refusal and all 25 existing regressions rerun with standard shape checks. Repair cycle 1: exact hash length and explicit check alphabet constraints. Negative tests are preserved unchanged.

## Package inspection and provenance

Official python-jsonschema documentation and PyPI 4.26.0 metadata were inspected. All five selected wheels matched SHA-256 published by official PyPI version metadata. Installation was offline with --no-index --require-hashes into the disposable chat environment only. No source-build hooks or .pth files. All five declare MIT; attrs/reference/specification packages supply immutable data and registry/schema machinery. jsonschema's default retrieval function can call urllib; the qualification factory explicitly overrides retrieval with NoSuchResource. rpds-py contains a native extension; its upstream source and official wheel metadata were inspected, but no binary reproducible-build or complete supply-chain/security certification is claimed.

The [pinned wheel provenance](../architecture/contracts/task-contract-v1/qualification-wheel-provenance.json) and [tested platform hash lock](../architecture/contracts/task-contract-v1/qualification-requirements-macos-arm64.txt) are retained. This is CPython 3.13/macOS ARM64 qualification; Ubuntu needs its own verified wheel set. Installation changed no project production dependencies.

## Passing evidence

- Original suite: disposable venv Python -B -m unittest discover -s tests -v, exit 0, 25 tests in 0.036s, OK.
- Standard qualification after schema repair: Python -B -m unittest discover -s qualification_tests -v, exit 0, 11 tests in 1.780s, OK. Includes all 25 original tests run with standard shape validation; no skips.
- standard.py entry point: exit 0, [qualified synthetic outcome evidence](evidence/2026-10-14-standard-validator/qualification-result.json): DONE/BLOCKED/UNVERIFIED/NEEDS_OWNER_DECISION as expected.
- Independent conversation-excerpt review: no Important findings. One Nit: exercise the actual factory in the retrieval-denial test instead of constructing another Registry. Test strengthened (same desired denial; no test relaxation); bounded follow-up resolved it with no remaining Nits. The 11 qualification tests reran after this correction: exit 0, 1.765s, OK. No third source review round.
- Review limitation: automatic approval review rejected the reviewer's required external AISecurity call because internal paths/task details would be disclosed to an unverified external destination. No retry or external disclosure workaround. Independent review used supplied source excerpts and parent-run evidence; no fresh on-disk verification by reviewer. Parent checked installed standard.py equals the authored source and retained current source hashes below.
- PASS: both schemas satisfy standard Draft 2020-12 meta-schema checks; named shape/cross-field/false-completion and remote-reference-refusal cases pass. Documented intentional numeric-encoding distinction remains.
- PASS: local Markdown link targets, retained evidence JSON, git diff --check.

## Disposition

DONE for finite-corpus standard-validator qualification. No global parity/conformance, OS containment, full Ubuntu gate, live adapter, production security, state migration or Foundation-0 completion claim. Normal-shell testing follows the owner's existing explicit pure-contract exception; no other repository tests were run. One schema repair plus one bounded test-coverage correction; no commits, pushes, PRs or merge. Prior uncommitted documentation remains preserved.

Next recommended task, not started: design the real provider-neutral adapter interface and evidence handoff. Live execution requires its separately scoped authorization and qualified containment.

## Qualified source hashes

- task-contract.schema.json — SHA-256 06996df38c65c0486a1b1c461f0e4c215abab6ab46fcb8a5b3d346c45c710a88
- task-result.schema.json — SHA-256 59dfb56f9c62cc47d29d5fca9cbd9d3ad86ca86af59a31aaa17f332d9ac8813b
- validate.py — SHA-256 9cad85b7d2175fbf5262c9b32cd156ae5c4038bb91af8e1f517dd5c12885d39a
- standard.py — SHA-256 8d7201635d3786597c6dbd9d9bfa31297ca7953d0cf252130a2e3de096d2e436
- qualification_tests/test_standard.py — SHA-256 3dcb024c6f24f83699eb4f0ec15c37312afef24c427d8d8980eba13872193bd8
- qualification-requirements-macos-arm64.txt — SHA-256 702dabe05fd3ef141d02dddcd65dbd94ffef4ddf692a97fa1f7717e97183b0f9
- qualification-wheel-provenance.json — SHA-256 a8ccc8a79e39f70bbf41974bb7209b895e2762631a68e5c445b5601c27919ece
