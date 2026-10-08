# Build Record — S00.05 `rikuri-storage`: canonical SQLite store and `Repository<T>`

| Field | Value |
|---|---|
| Change | The canonical store: SQLite bootstrap (WAL, FULL sync, one writer), component migrations, object registry, `Repository<T>` with optimistic versioning and read-back |
| Date | 2026-10-05 |
| Author / agent session | Claude Code session for Henry Chen |
| Branch | `stage-00/storage-ledger`: replayed on 2026-10-08 from the local `stage-00/kernel` branch onto `main` at `c5f0060`, which had gained 41 commits (the PR #7/#11/#13/#17 gate fixes); clean cherry-pick, no conflicts. The old branch and its worktree are kept as a backup |
| Commits | `de5beea` on the original branch, replayed as `9cc42ed` (with the domain prerequisite `6006cd2` → `2b7d34f`); defect fixes `ab41b21` and the following commit ([defect record](2026-10-08-storage-ledger-verifier-defects.md)) |
| Stage Gate | [Stage 00](../stage-gates/foundation0/00-bootstrap-kernel.md) |
| Related ADR / requirement | S00.05, T00.04, S00.10 ([ADR 0002](../architecture/adr/0002-sqlite-current-state-model.md), Proposed), E1; Architecture §8, §21–22, §43; PRD CORE-006 |
| Status | IN PROGRESS |

## 1. Plan

- [x] One-sentence statement: give the kernel a canonical store whose committed state survives a killed process, whose updates cannot lose another writer's change, and whose migrations never leave a half-upgraded database.
- [x] Blast radius: new crate; creates the canonical database format (protected core, irreversible once user data exists — none exists yet). No money, execution or network path.
- [x] Current behavior characterized: no store existed.
- [x] **I will know this works when** the T00.04 contract suite (versioned put/get, conflict with the actual version, not-found, duplicate, rollback leaving nothing) passes; migration tests show fresh apply, idempotent reopen, upgrade preserving data, a failed step leaving the last good version, and refusal of newer or divergent schemas and of malformed lists before any table exists; a model-based property test agrees with the store over random sequences; tampered rows are refused; WAL and FULL sync are confirmed; and deliberately broken versions of the version check, rollback, ID check, newer-schema refusal and checksum comparison each fail a test.
- [x] Architecture impact: ADR 0002 (Proposed); canonical vs derived — the registry is canonical; deletion/backup — `deletion_unit_id` column carried from the start; module ownership — the ledger owns its tables via its own migrations.
- [x] Threat/control/test: in the Module Contract (partial state, lost update, tampered row, half-applied upgrade).
- [x] Existing solution review: rusqlite 0.40.2 (MIT) with bundled SQLite 3.53.2 (libsqlite3-sys 0.38.2, MIT); BLAKE3 1.8.7 for migration checksums (CC0/Apache-2.0); alternatives in ADR 0002. Custom code is the thin registry and runner; no maintained crate provides component-scoped checksummed migrations with version refusal and the repository contract together.
- [x] Authoritative state change? Yes, the store is the authority. Transactional State Evolution for single-transaction changes: preserve (old row untouched until commit) → validate (version check, constraints) → candidate (the UPDATE inside the transaction) → read-back (compare the stored body inside the transaction; this confirms what the transaction holds, not durability) → durable write and atomic promote (one commit with WAL + FULL). *Corrected 2026-10-08: the mapping first listed the commit twice and implied the pre-commit read-back verified durability.* Injected-failure evidence: rollback tests here; kill -9 matrix T00.07 with `rikurid`.

## 2. Build

- [x] `src/store.rs`, `src/migrate.rs`, `src/repo.rs`, `src/error.rs`; tests `repository_contract.rs`, `migrations.rs`.
- [x] Prerequisite in `rikuri-domain` (the branch's first commit, "clock seam and shared wire-enum macro for the kernel crates"): `Clock`/`SystemClock`/`Timestamp` so the store's times are deterministic in tests, and the exported `wire_enum!` the ledger uses for `EventStatus`. Recorded here on 2026-10-08; it had no Build Record of its own. The domain Module Contract lists both.
- [x] Inputs validated: migration lists, stored rows (kind, schema version, ID, JSON), privacy class by CHECK constraint.
- [x] Typed and cohesive; largest file `repo.rs` about 300 lines.
- [x] Module Contract: [`crates/rikuri-storage/README.md`](../../crates/rikuri-storage/README.md).
- [x] Security question: canonical data integrity. Allowed: versioned writes. Denied: stale writers, partial transactions, tampered rows, unknown privacy classes, newer/divergent schemas. Tested both ways.

## 3. Debug

During the build, a test claimed "refused before touching the database" while the runner validated each component only after applying the storage tables. The runner now validates every list first, and the test checks that no table exists after a refusal.

## 4. Verify

- [x] Unit/failure: 12 tests (7 migration, 5 contract/health/tamper/property).
- [x] Shared contract suite: T00.04 generic `contract<R: Repository<Thing>>`.
- [x] Property: `versions_follow_the_model` (64 cases × up to 40 operations).
- [ ] Running-system probe — through `rikurid` (S00.07).
- [x] Regression guard: five mutants, each failing (evidence log).

## 5. Close Out

- [x] Full required gate green on the branch: Pass, 13 steps, after the replay onto current `main` (evidence log).
- [x] Commit; Stage Gate Record progress updated (2026-10-08, owner's request).

## Skipped / Deferred / Resequenced

| Item | Disposition | Reason | Dependency impact | Owner | New stage/ordinal | Reactivation / acceptance condition |
|---|---|---|---|---|---|---|
| Real caller | RESEQUENCED within Stage 00 | `rikurid` is the only writer | S00.05 incomplete until `rikurid` uses it | Stage 00 implementer | S00.07 | T00.09 vertical slice writes through `Store` |
| `synchronous=FULL` latency | Not yet proven | Not measured | None on correctness | Stage 00 implementer | Stage 00 close-out | Write latency recorded on the reference installation |

Execution-evidence fields: N/A (no execution path).

## Evidence log

```text
$ cargo test -p rikuri-storage
tests/migrations.rs           7 passed
tests/repository_contract.rs  5 passed
$ cargo clippy -p rikuri-storage --all-targets -- -D warnings → clean

# Mutants (scratch copy; each restored afterwards)
update ignores the expected version → sqlite_repository_meets_the_contract, versions_follow_the_model FAILED
write commits even on error         → sqlite_repository_meets_the_contract FAILED (rollback assertion)
decode trusts the body's ID         → tampered_rows_are_reported_not_trusted FAILED
newer schema accepted               → database_from_a_newer_build_is_refused FAILED
checksum not compared               → changed_definition_of_an_applied_migration_is_refused FAILED
# 2026-10-08, replayed onto main c5f0060 (branch stage-00/storage-ledger)
$ scripts/quality/full-gate.sh → RESULT: PASS (13 steps), exit 0
  guard 91 tests OK; fmt, build, clippy -D warnings, test, cargo deny, secrets, ratchet (0/0/0) PASS
  CONTRACT checked 3 crate(s); POLICY checked 3 crate(s); SCRAP clean
  tests: rikuri-ledger ledger_contract 9 passed; rikuri-storage migrations 7, repository_contract 5 passed
```
