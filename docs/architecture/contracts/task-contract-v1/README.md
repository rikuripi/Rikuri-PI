# TaskContract v1 — design and validation fixture

Status: **experimental contract draft** under owner-accepted ADR 0006; not a production API, grant authority, execution adapter or Foundation-0 gate. Rikuri owns this source; Orchard references it rather than maintaining a second contract definition.

## Contents and validation

- `task-contract.schema.json`: closed JSON Schema 2020-12 shape for one task/run request.
- `task-result.schema.json`: execution/acceptance/application receipt shape and necessary DONE conditions.
- `validate.py`: offline, dependency-free checker for exactly the schema vocabulary used here; cross-field checks and in-memory fake controller. It refuses unsupported vocabulary and does not implement general JSON Schema. Production admission needs a qualified standards-compliant validator and cross-field enforcement.
- `examples/mean-task.json`: synthetic requested change, source revision and identities.
- `tests/test_contract.py`: positive and negative cases, authority binding, failure evidence and stop/replay checks.

From this directory, inside the approved test-safety environment:

```sh
PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s tests -v
PYTHONDONTWRITEBYTECODE=1 python3 validate.py
```

The second command prints synthetic success, acceptance failure, missing-evidence and denial traces. No subprocess, model, network, filesystem artifact write, production DB or arbitrary command execution is part of the fake controller. File loading is limited to schema/example input; tests use temporary authored JSON files. Task text is never evaluated as instructions or code. The runner uses pre-authored paired source/function fixtures, not execution of a generated Python artifact. These traces prove fixture-contract semantics only, not a real coding path, sandbox containment or runtime budgets.

## Ownership and stable references

| Contract concept | Rikuri authority | Orchard role |
| --- | --- | --- |
| Intent/project/task/run | Project Registry + Project Lifecycle Kernel | Seed/software views and requested work |
| Canonical transaction/receipt | `rikurid`, storage and ledger | Projection and evidence links |
| Approval/privacy/capabilities | Rikuri policy/AISecurity + admitted envelope | Present approved scope; cannot issue its own grants |
| Execution profile | Registry and model/harness/runtime adapters | Software-task specialization |
| Acceptance and stopping | Quality Gate Engine + run controller | Display outcomes and required owner choices |
| Editor/workspace/artifact | Shared developer/workspace services | Software presentation, never a second IDE authority |
| Prototype import | Explicit governed importer | Preserve original IDs/receipts; no direct canonical writes |

IDs use ADR 0001's canonical non-nil prefixed ULID form. Existing `project`, `task`, `run`, `source` and `actor` kinds are used directly. `intent_ref`, `workspace_ref`, `profile_ref`, artifact/evidence references and evidence destination are existing `content` IDs for this draft, with distinct fixture identities. This does **not** register new kernel kinds or define those future entity schemas. A production source revision must be an actual validated SHA-256 snapshot digest; the synthetic `a…a` revision is only a fixture.

## Request, authority and profile boundary

A valid contract requests capabilities; it grants none. Production policy must resolve stable references, reconcile current source/trust/privacy/revocation state, then admit only the approved contract version. The fake controller receives a separate trusted fixture grant bound to the canonical JSON SHA-256 (sorted keys, ASCII escaping, finite values). Editing even task text invalidates that approval. Grant inputs here are runner-authored, not a serializable production approval API; authentication, expiry, current revocation, signature/ledger lookup and OS qualification remain unimplemented.

The fixture refuses unauthorized paths/checks, stale revisions, network/paid work, unknown ceilings, unsupported checks or more than one attempt. Its only artifact path is `mean.py`. Contract limits express ceilings; the in-memory demonstration checks ceilings and one attempt, but does not enforce wall-clock, spend, memory or disk against an external worker. Actual hard-budget reservation, cancellation during execution, retry control, durable idempotency and trusted receipts belong to the production controller and are pending.

Privacy names match `rikuri-domain`: normal/private/local_only/secret. Secret content is never admitted to model context. Local Only requests reject network or cloud-profile routing and additionally need an external no-network proof; the fixture has no real proof and its default grant denies that case. No privacy class or profile string proves an OS boundary. Private-data cloud admission and destination/credential qualification remain production requirements.

Check identifiers are provider-neutral named operations, not shell strings. A trusted production registry must resolve versioned, separately controlled checks, bind evidence to source/result hashes and preserve required tests outside worker mutation. The synthetic controller recognizes only the `mean-acceptance-v1` gate and `mean_fractional` / `mean_empty` checks; others block. Pi remains the primary/default production harness; Codex/Claude names are opaque adapter/profile data and do not prove those runtimes integrated.

## Outcomes and explicit stop

A result binds the task/run and exact contract digest. DONE requires completed execution, all task-specific criteria evidenced as passing, the required independent gate, the expected verifier identity, and evidence linked to allowed artifact hashes. Receipt validation is structural consistency, not authentication: only trusted verification services may issue production receipts. A forged JSON claiming success is never proof.

- Correct fractional/empty behavior: execution completed, acceptance passed, gate passed, DONE; not applied.
- Integer division: execution completed, fractional acceptance failed, BLOCKED; no repair or next task.
- Missing evidence: execution completed, acceptance unverified, UNVERIFIED.
- Missing/stale approval: NEEDS_OWNER_DECISION before any worker call.
- Capability/budget/unsupported-operation refusal: BLOCKED before any worker call.
- Cancellation before execution: CANCELLED.

Application remains `not_applied` throughout this fixture; separate consent and a real apply receipt are required. Historical Orchard DONE maps to execution completed with acceptance UNVERIFIED unless separately evidenced. Replay returns the same in-memory receipt without worker or gate repetition; changed contracts cannot reuse run identity. Process restart/replay, concurrent leases and imports are not proven by an in-memory cache.

## Admission and remaining work

This draft adds no stage item or universal gate. Before production admission: independent review; a qualified JSON Schema engine; resolved referenced entities; live policy/approval/revocation and budget integration; credential/destination/OS qualification; real adapter and separately controlled acceptance evidence; current Orchard regressions; durable restart/replay and tested status-preserving importer. Each execution experiment and migration is separately scoped and authorized. Creative-studio sequencing and Foundation-0 Stage Plan v2.1 remain authoritative.

## Standard-validator qualification

The optional qualification harness uses pinned [python-jsonschema 4.26.0](https://pypi.org/project/jsonschema/4.26.0/) and its Draft202012Validator. It does not replace the dependency-free fixture checker or admit a production runtime. Install only in a disposable environment, using the recorded wheel set and hashes in qualification-requirements-macos-arm64.txt. The lock is for the tested CPython 3.13/macOS ARM64 environment; it is not a portable Ubuntu lock. qualification-wheel-provenance.json records versions, licenses, dependencies, official PyPI hash matches and native-wheel limits. No wheels or virtual environment are vendored.

From this contract directory, with that environment's Python and bytecode disabled:

    python -B -m unittest discover -s qualification_tests -v
    python -B standard.py

Both shapes undergo Draft 2020-12 meta-schema checking. Named positive/negative cases compare standard shape behavior and the fixture checker; the original 25 regressions also run with standard shape validation. Cross-field acceptance coverage, source binding and grants remain additional checks. The standard engine's registry explicitly refuses remote reference retrieval; a negative case checks that no urllib request occurs.

JSON Schema integer means a whole numeric value, so it accepts 30.0; the fixture additionally requires Python integer values for canonical wire limits and rejects that encoding. This intentional stricter ingress rule is tested and recorded rather than claimed as shape equivalence. Duplicate-key/nonfinite input handling also belongs to the existing strict loader. The wrapper performs both standard shape and fixture cross-field/wire validation; it does not authenticate a submitted receipt.

Qualification initially reproduced trailing-newline acceptance in hashes/check names. Hashes now require exactly 64 lowercase hexadecimal characters and check IDs explicitly forbid characters outside their alphabet. No task text, scope or authority changes follow. See the [qualification Build Record](../../../build-records/2026-10-14-task-contract-standard-validator.md) for actual tests, red evidence and limitations. OS containment, full Ubuntu gate and real adapter integration remain unproven.

## Ubuntu qualification receipt

The same 11 cases passed on Ubuntu26.04.1 ARM64/Python3.14.4, including the original25 with standard shape validation, inside the owner-approved scoped namespace/drop fixture. Dependencies loaded from a hash-verified immutable snapshot, not system site-packages. qualification-requirements-linux-aarch64-cp314.txt and qualification-wheel-provenance-linux.json record the separate tested Linux wheel set. See the [Ubuntu Build Record](../../../build-records/2026-10-14-ubuntu-validator-qualification.md). These are finite fixture observations; original unprivileged admission is still denied and the full Ubuntu gate/hostile-worker/real-adapter qualification remain pending.

## Proposed real adapter seam

The [adapter interface and evidence-handoff draft](ADAPTER_INTERFACE.md) defines preparation/start/observation/cancellation/capture ownership and independent verification. It is design only; richer authenticated lifecycle/receipt schemas and production integration remain separate tasks.

## Isolated adapter simulator

The [adapter simulator](ADAPTER_SIMULATOR.md) implements typed fixture operations, persisted fake dispatch acknowledgements and independent evidence/refusal cases. It does not wire real adapters or promote canonical state. Its new fixture schemas remain distinct from existing TaskContract/TaskResult v1.

[Read-only Orchard snapshot bridge](ORCHARD_BRIDGE.md)

[Legacy observation schema v1](orchard-observation.schema.json)
