# Build Record — S00.05/S00.06 defects found by the first independent verification

| Field | Value |
|---|---|
| Change | Close three false claims in the canonical store and ledger (append-only, owner-only, all-or-nothing upgrade) and the related smaller gaps, tests first |
| Date | 2026-10-08 |
| Author / agent session | Claude Code (Opus 5.5) for Henry Chen |
| Branch | `stage-00/storage-ledger` |
| Commits | `ab41b21` (tests, red by design), then the fix in the commit containing this record, on `stage-00/storage-ledger` |
| Stage Gate | [Stage 00](../stage-gates/foundation0/00-bootstrap-kernel.md); S00.05, S00.06 |
| Related ADR / requirement | [ADR 0002](../architecture/adr/0002-sqlite-current-state-model.md), [ADR 0003](../architecture/adr/0003-ledger-and-idempotency.md) (both Proposed); T00.04, T00.05; Architecture §4.11, §43; [S00.05 record](2026-10-05-s00-05-rikuri-storage.md), [S00.06 record](2026-10-05-s00-06-rikuri-ledger.md) |
| Status | COMPLETE — the defects are fixed and verified; S00.05/S00.06 themselves stay incomplete until `rikurid` is their real caller (S00.07); CI on the PR is Not yet proven until observed |

## 1. Plan

- [x] One-sentence statement: make the store and ledger do what their contracts and ADRs say (ledger rows cannot be rewritten by `REPLACE`, the database and its directory are owner-only, a failed upgrade changes nothing), and close the smaller gaps the same review found, each with a test that failed first.
- [x] Blast radius: protected core (canonical store and history). Changes the ledger's v1 migration (never shipped; no user database exists), `Store::open` refusal behavior, the migration runner's transaction scope, and validation in the ledger. No money, execution or network path. Irreversible effects: none; nothing is merged or deployed.
- [x] Current behavior characterized, with evidence from the independent verifier's scratch-copy probes on 2026-10-08 (branch `stage-00/storage-ledger` at `9de5061`, after the replay onto `main`):
  1. `INSERT OR REPLACE` on `events` and `idempotency` succeeds: a committed event was rewritten (`type=task.deleted status=Failed metadata={"rewritten": true}`) and a changed request replayed a forged response. Triggers cover UPDATE and DELETE only; SQLite does not fire DELETE triggers for REPLACE unless `recursive_triggers` is on, and the store never sets it.
  2. A fresh store creates `rikuri.db` and `-wal` as 0644; an existing 0755 directory is accepted. ADR 0002 and the storage README promise owner-only.
  3. Migrations commit step by step per component: with an old build at `aaa[1] bbb[1]` and a new build at `aaa[1,2] bbb[1, failing 2]`, `aaa` is upgraded and committed before `bbb` fails, after which the new build fails and the old build refuses (`aaa schema is at version 2, newer than this build supports`). A divergent definition of an applied migration has the same effect. Architecture §43 says a failed migration must not leave a half-upgraded store.
  4. Smaller: `EventType::new` echoes its whole input into the error (1 MiB in, 1 MiB message; main fixed this class in PR #7 with `wire::bounded`, which is crate-private); storage `Corrupt` errors echo unbounded row text; `privacy_class` and `deletion_unit_id` columns are never checked against the body, so tampering them goes unnoticed; metadata keys have no length limit although strings are limited to 200 characters; idempotency `command` and `response` are unvalidated although the response promises no content; a bookkeeping gap (v2 missing, v3 recorded) still opens; the row-kind check and read rollback are untested (mutants survive).
- [x] **I will know this works when** each new test below fails against the current code and passes after the fix; the existing T00.04/T00.05 suites still pass unchanged; the full gate passes; and the independent verifier finds no unresolved Important finding.
- [x] Architecture-impact check:
  - [x] invariant / ADR: ADRs 0002 and 0003 (Proposed) are corrected to state exactly what is enforced: append-only covers UPDATE, DELETE and REPLACE through SQL on any connection, not schema changes such as `DROP TABLE`; owner-only is enforced on Unix and refused otherwise; upgrades are all-or-nothing.
  - [x] API / schema / event format: ledger v1 migration gains two BEFORE INSERT triggers (v1 never shipped; a database made by the 2026-10-05 build is refused by the checksum check, which is the designed behavior). `rikuri_domain::bounded` becomes public. New validation errors in the ledger.
  - [x] canonical vs derived data: unchanged.
  - [x] privacy / security: owner-only files; privacy and deletion-unit columns cross-checked against the body on every read.
  - [x] deletion / backup / restore: `deletion_unit_id` now verified on read; forget itself is Stage 01.
  - [x] budget / cost, nondeterministic AI / replay: N/A.
  - [x] documentation: storage, ledger and domain READMEs (Module Contracts), ADRs 0002/0003, AGENTS.md repository paragraph.
  - [x] module ownership: `bounded` stays owned by `rikuri-domain` and is reused, not copied.
- [x] Relevant threat/control/test: Module Contract threats "tampered row", "half-applied upgrade", "history edited"; tests listed in §4.
- [x] Existing solution review: N/A; fixes within SQLite (triggers, one transaction) and std (`OpenOptionsExt::mode`).
- [x] Authoritative state change: the migration runner now preserves (no commit until every check passes) → validate (all components' lists, recorded versions contiguous, checksums, not newer) → candidate (all pending steps in one IMMEDIATE transaction) → durable write and atomic promote (one commit). Injected-failure evidence: the multi-component failure tests.
- [x] Protected-path risk note: edits to ADRs 0002/0003 (Proposed drafts), the Stage Gate Record and AGENTS.md (its repository paragraph, which AGENTS.md itself says to update as Stage 00 adds code). Guard profile `development`; each edit is logged. The risk is overstating a guarantee; each ADR change narrows or matches a claim to a test.

## 2. Build

- [x] Smallest coherent fix completed: ledger REPLACE guards and validation; owner-only directory and files; all-or-nothing upgrade with contiguity check; privacy/deletion-unit cross-check; bounded errors via `rikuri_domain::bounded`.
- [x] Inputs crossing trust boundaries validated (stored rows, file modes, metadata keys, idempotency command/response).
- [x] Module Contracts updated: storage, ledger and domain READMEs.
- [x] Security question: canonical data integrity and confidentiality. Allowed and denied paths tested (§4): legitimate reopen of a private store, append, replay and upgrade pass; REPLACE, shared directories/files, tampered columns and gapped history are refused.

## 3. Debug

- [x] Problem reproduced before fix; evidence: the new tests run red (evidence log).
- [x] Root-cause classes:
  - **A guarantee stated more broadly than its mechanism.** Append-only was enforced against two of the three SQL ways to change a row; owner-only only when Rikuri created the directory; "rolled back" per step, not per upgrade. The tests exercised the mechanism, not the guarantee.
  - **A fixed defect class re-entering through new code** (unbounded error echo). The tripwire lived in one crate and the helper was private.
  - **Stored metadata not cross-checked** (privacy and deletion-unit columns, bookkeeping contiguity).
- [x] Sibling cases swept: every trigger-protected table (events, idempotency: both covered); every file Rikuri creates (database, `-wal`, `-shm`, directory); every place a component commits during open (only the migration runner); every error that formats stored or caller text (`EventType`, `Corrupt` reasons, serde errors, metadata keys).
- [x] State-changing remediation justified by evidence: the verifier's probes, reproduced by the red tests before the fix.

## 4. Verify

New tests (written before the fix; must fail first unless marked *coverage*):

| Test | Defect |
|---|---|
| `ledger_rows_refuse_replace_too` (ledger) | 1 |
| `database_and_directory_are_owner_only` (storage, Unix) | 2 |
| `shared_directory_or_database_is_refused` (storage, Unix) | 2 |
| `failed_multi_component_upgrade_changes_nothing` (storage) | 3 |
| `refusal_in_one_component_upgrades_no_other` (storage) | 3 |
| `recorded_versions_must_be_contiguous` (storage) | 4 |
| `event_type_errors_are_bounded` (ledger) | 4 |
| `corrupt_row_errors_are_bounded` (storage) | 4 |
| `privacy_and_deletion_columns_must_match_the_body` (storage) | 4 |
| `metadata_keys_are_bounded` (ledger) | 4 |
| `idempotency_command_and_response_are_validated` (ledger) | 4 |
| `rows_of_another_kind_are_refused` (storage) | 4, *coverage* (kills a surviving mutant) |
| `reads_never_commit` (storage) | 4, *coverage* (kills a surviving mutant) |

- [x] Each test red before the fix (evidence log); the two *coverage* tests pass already, as expected.
- [x] All green after the fix. Existing suites unchanged except the rename of `malformed_migration_lists_are_refused_before_touching_the_database` to `…_before_any_table_exists` (before the lock; the old name overstated) and the two owner-approved edits under the lock: the path-only fixture correction and the lint-only helper change (evidence log).
- [x] Full gate: PASS, 13 steps (evidence log).
- [x] Independent verifier: round 1 found three Important defects (the reason for this record); round 2 on the fix, VERDICT PASS, with Nits. Its cheap Nits were fixed after round 2 (evidence log); the rest are dispositioned below.

## 5. Close Out

- [x] Full required gate green (final run in the evidence log).
- [x] Scrap Sweep: `crates/rikuri-ledger/tests/ledger_contract.proptest-regressions`, written by the red run (it recorded the fixture failure, not a counterexample), deleted after the lock release. Scratch probes, mutants and logs live in the session scratchpad outside the repository; the temporary worktree was removed. The unrelated untracked `rikurinode.code-workspace` is preserved.
- [x] Documentation updated: Module Contracts, ADRs 0002/0003 (dated corrections), AGENTS.md repository paragraph, README.md, S00.05 record.
- [x] Stage Gate Record updated: S00.05/S00.06 lines link this record; the domain follow-ups and this record's deferrals are rescheduled to S00.07.
- [x] Commit, owner's request 2026-10-08: tests first (`ab41b21`, red by design), then the fix with the docs and records. Push and PR also requested.

## Skipped / Deferred / Resequenced

| Item | Disposition | Reason | Dependency impact | Owner | New stage/ordinal | Reactivation / acceptance condition |
|---|---|---|---|---|---|---|
| `Tx` derefs to the connection, so raw `COMMIT`/`BEGIN` inside `write` can escape the transaction | DEFERRED | Needs a narrower `Tx` API than `Deref<Target = Connection>`; every component's SQL goes through it | None until a component misuses it; only kernel crates hold a `Tx` | Stage 00 implementer | S00.07 (first real caller) | `Tx` exposes `execute`/`query` only, or a test proves raw transaction control is refused |
| Store health counts `NotFound`/`VersionConflict` as failures and never clears `last_error` | DEFERRED | Health semantics belong with `system.health` | Health may over-report failure | Stage 00 implementer | S00.07 | `system.health` test distinguishes domain refusals from store failures |
| Property tests are randomly seeded | DEFERRED | Already listed as a Stage 00 follow-up for the next domain change | A rare failure may not reproduce | Stage 00 implementer | Stage 00 close-out | Seeds fixed or recorded on failure |
| `insert` maps every constraint violation to `AlreadyExists` | DEFERRED | No reachable non-key violation through the typed API (privacy class and JSON come from typed values) | Misleading error if a later schema adds constraints | Stage 00 implementer | When the registry gains a constraint | Map only primary-key/unique violations; test with the new constraint |
| Guard false positive: under the fix-lock, a shell command whose inline Python edited two Markdown records was denied ("inline code in this command writes test/baseline tests") because its text named test files | RECORDED (owner chose to make the same edits with the file-edit tool, which the guard checks by real path; nothing bypassed) | The shared `guard.py` is changed only through the owner's full-tier sync | Mild friction under a lock; no protection lost | Product owner | Next shared-guard change | The inline-code check matches written paths, not words in the code |
| Test that append-only holds on a raw connection without `recursive_triggers` (verifier round 2, Nit 2: removing either insert guard survived the suite because tests used the store's connection) | DONE after the lock release: `ledger_rows_refuse_replace_on_a_raw_connection` | Coverage, not a red-first defect test (the behavior was already verified by the verifier's raw-connection probes) | — | Stage 00 implementer | This change | Mutants each fail it: whole events guard removed, only its `seq` clause removed, idempotency guard removed (evidence log). Removing `recursive_triggers` alone is not detectable while the guards hold; it is defense in depth |
| Delete `ledger_contract.proptest-regressions` (scrap from the red run) | DONE after the lock release | It recorded the red fixture failure, not a counterexample | — | Stage 00 implementer | This change | File absent |
| Migration SQL that ends the transaction itself (`COMMIT; BEGIN;`) is not detected (verifier round 2, Nit 4) | DEFERRED | `is_autocommit` cannot see `COMMIT; BEGIN;`; a reliable check needs statement-by-statement execution (`rusqlite::Batch`) that refuses transaction-control statements | Only in-repository, reviewed migrations run; documented in `migrate.rs` and ADR 0002. A lone `COMMIT` already fails visibly | Stage 00 implementer | Before the first migration after v1 | Runner refuses BEGIN/COMMIT/END/ROLLBACK/SAVEPOINT/RELEASE/VACUUM statements; a test with each fails |
| Raw SQL can insert an event with an explicit, earlier `seq`; rows at `seq ≤ 0` are not covered by the insert guard on a connection without `recursive_triggers` (verifier round 2, Nit 3) | ACCEPTED, documented in ADR 0003 | Out of scope under ADR 0004's trusted-owner assumption, like deliberate raw edits generally; the crate never writes such rows | None for code paths that use the crate | Product owner | Revisit when cross-process or cross-device trust boundaries arrive (hash-chained events, ADR 0003 option 3) | ADR 0003 states the limit |
| Guard coverage is inconsistent: the fix-lock blocked a verifier's write to a `tests/` path inside `/tmp` scratch, while sibling worktrees outside the project are not covered | RECORDED | Shared `guard.py`, owner's full-tier sync | A verifier's scratch-only probes can be blocked; worktree edits are not | Product owner | Next shared-guard change | One rule for paths outside the project directory |
| "ADR 0004" cited before it exists | DEFERRED | The ADR 0004 draft arrives with S00.07 | Dangling reference until then | Stage 00 implementer | S00.07 | ADR 0004 file present |

## Evidence log

```text
# Red run before any fix, in the guarded checkout /home/junc/Rikuri-PI on stage-00/storage-ledger
$ cargo test -p rikuri-storage -p rikuri-ledger --locked --no-fail-fast
FAILED  ledger_rows_refuse_replace_too                  (INSERT OR REPLACE on events was allowed)
FAILED  event_type_errors_are_bounded                    (1 MiB name → ~1 MiB error)
FAILED  metadata_keys_are_bounded                        (3000-char key accepted: Ok(Event …))
FAILED  idempotency_command_and_response_are_validated   (empty command accepted)
FAILED  database_and_directory_are_owner_only            (rikuri.db is 644)
FAILED  shared_directory_or_database_is_refused          (0755 directory: opened)
FAILED  failed_multi_component_upgrade_changes_nothing   (old build: SchemaTooNew aaa found 2 supported 1)
FAILED  refusal_in_one_component_upgrades_no_other       (old build: SchemaTooNew aaa found 2 supported 1)
FAILED  recorded_versions_must_be_contiguous             (unwrap_err on Ok: store opened)
FAILED  privacy_and_deletion_columns_must_match_the_body (privacy downgraded: Ok(Some(…)))
FAILED  corrupt_row_errors_are_bounded                   (error message is 1048652 bytes)
ok      rows_of_another_kind_are_refused, reads_never_commit (coverage; kill surviving mutants)
totals: ledger 9 passed 4 failed; files 0/2; migrations 7/3; repository_contract 7/2

# After the fix (fix-lock engaged 2026-10-08T10:23 for this record)
$ cargo test -p rikuri-domain -p rikuri-storage -p rikuri-ledger --locked --no-fail-fast   (umask 002)
repository_contract 9/9 ok, files 2/2 ok; ledger_contract 1/13 and migrations 0/10: every failure is the
fixture's Store::open refused with "the data directory has mode 775 … requires owner-only (chmod 700)".
Cause: tempfile::tempdir() follows the umask (0775 here, 0755 on a 022 runner), and the ledger fixture and
the migration helpers put rikuri.db directly in it. repository_contract and files use a subdirectory the
store creates 0700, so they pass.
$ (umask 077; cargo test -p rikuri-domain -p rikuri-storage -p rikuri-ledger --locked --no-fail-fast)
domain 28+4 ok; ledger_contract 13/13 ok; files 2/2 ok; migrations 10/10 ok; repository_contract 9/9 ok.
→ the fixes work; the fixtures need the database in a store-created subdirectory, a test edit that waits
  for the owner under the fix-lock.

# Owner decision 2026-10-08: "Approve fixture fix". Lock released on the owner's instruction, the
# database path in the ledger fixture and the migration helpers (5 occurrences, 3 of them in pre-existing
# tests) changed from join("rikuri.db") to join("state/rikuri.db"); no assertion changed (diff: 0 removed
# assert lines). Lock re-engaged until review.
$ cargo test -p rikuri-domain -p rikuri-storage -p rikuri-ledger --locked --no-fail-fast   (umask 002)
domain 28+4 ok; ledger_contract 13/13; files 2/2; migrations 10/10; repository_contract 9/9

# Full gate then failed at clippy (unwrap_used): four new test helpers called unwrap() outside a
# #[test] function (files.rs mode/set_mode/write_one_row, repository_contract.rs set_column).
# Lesson: run clippy on new tests before engaging the fix-lock. Owner decision 2026-10-08: "Approve
# lint-only fix". Lock released; the helpers return TestResult and use `?` like the file's existing
# helpers, callers in #[test] functions unwrap; 0 assertion lines removed. Lock re-engaged.
$ cargo clippy --workspace --all-targets --locked -- -D warnings   → clean
$ cargo test -p rikuri-domain -p rikuri-storage -p rikuri-ledger --locked --no-fail-fast   (umask 002)
domain 28+4 ok; ledger_contract 13/13; files 2/2; migrations 10/10; repository_contract 9/9
$ scripts/quality/full-gate.sh → RESULT: PASS (13 steps)

# Independent verifier, round 2 on the fix: VERDICT PASS. Red-first reproduced (the 11 named tests
# fail against the 9de5061 sources, all pass on the fix); REPLACE refused on a raw connection with
# recursive_triggers off (explicit seq + new event_id, same event_id, text/real seq, upserts); mutants
# killed: refuse_if_shared never refuses, database created 0644, commit per component, contiguity off,
# privacy cross-check off. Test edits under the lock confirmed path-only and lint-only. Nits fixed
# afterwards in source and docs: ensure_private_dir re-checks the mode after creating (a directory that
# appeared between exists() and create() was never checked; no test, since reproducing the race needs
# fault injection: the verifier showed it in a scratch copy); the contiguity error no longer lists every
# recorded version; ADR 0002 pragma list and Transactional State Evolution wording; Stage Gate test count
# and rescheduled follow-ups; this record's checkboxes. Remaining Nits: rows in the deferral table.
$ cargo fmt --all --check && scripts/quality/full-gate.sh → RESULT: PASS (13 steps), exit 0

# Owner decision 2026-10-08: "Release, commit, push, PR". Fix-lock released by the owner's instruction.
# Added ledger_rows_refuse_replace_on_a_raw_connection (raw connection with recursive_triggers and
# foreign_keys both off; a first draft relied on the bundled SQLite's default foreign keys, which hid the
# seq branch, so the test now turns them off). Deleted the proptest-regressions scrap.
$ cargo test -p rikuri-ledger --locked --test ledger_contract → 14 passed
# Mutants in a scratch copy (src only), each FAILED the new test at its own statement:
#   events_no_replace removed              → allowed: INSERT OR REPLACE INTO events (seq, …)
#   only its `NEW.seq > 0 …` clause removed → allowed: INSERT OR REPLACE INTO events (seq, …)
#   idempotency_no_replace removed          → allowed: INSERT OR REPLACE INTO idempotency …

# Location: the tests were first written in a sibling worktree, where the guard's fix-lock
# does not apply (known gap, owner decision 2026-10-05). Before engaging the lock the work was
# stashed, that worktree removed, and the branch checked out in the guarded project directory.
```
