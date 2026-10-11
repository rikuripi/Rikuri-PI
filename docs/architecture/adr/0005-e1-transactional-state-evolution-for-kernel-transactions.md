# ADR-0005: E1 — Transactional State Evolution for kernel transactions

Status: Accepted (2026-10-09, Henry Chen, product owner: "Accept as proposed"; build the scoped check now). Clarified 2026-10-10: the pre-commit check is redesigned to validate what each command wrote (owner: "Redesign it now", "Land it"); see [Clarification 2026-10-10](#clarification-2026-10-10-validate-what-the-command-wrote).
Date: 2026-10-09
Owners: Henry Chen (product owner, accepts); Stage 00 implementer
Related requirements/tests: Stage Plan S00.11 and experiment register E1; Architecture §8, §43; ADR 0002 (store), ADR 0003 (ledger and idempotency); W1 (T00.06–T00.08, T00.12, crash-point placement); [E1 Build Record](../../build-records/2026-10-09-s00-11-e1-tse-spike.md)

## Context / problem

Architecture §43 defines Transactional State Evolution (TSE): preserve the old authoritative state → validate inputs → transform a complete candidate → validate the candidate → durably write → read back and verify → atomically promote → only then retire the old state; on any pre-commit failure the last known-good state stays authoritative and the operation is retryable. E1 asks for the simplest implementation that satisfies it for kernel transactions, comparing a versioned SQLite transaction with an explicit candidate/promotion design at the Stage 00 crash points. Its decision rule: "If versioned SQLite transactions satisfy the same invariant and crash tests, omit redundant ceremony; if neither passes, Stage 00 remains open."

ADR 0002 already argued that one SQLite transaction with read-back answers E1 for single-store changes, but without the comparison, without evidence for every step, and with `synchronous=FULL`'s cost unmeasured.

## Decision drivers

- Each TSE step needs a mechanism and a test that fails without it (W1's lesson: a claim without a biting test is not evidence).
- The last known-good state must stay authoritative when a candidate is wrong, not only when a process dies.
- No ceremony that adds commits, fsyncs or recovery states without adding a guarantee.

## Options considered

1. **One SQLite transaction per command, as built** (WAL, `synchronous=FULL`, typed plan, repository read-back, event append and idempotency record in the same commit). Candidate validation beyond typed construction and read-back happens only after the commit, in the consistency replay, which quarantines writes.
2. **Option 1 plus a candidate check before the commit, scoped to the command's own objects** (the task it touches, that task's runs and events through the ledger's `by_task` index), refusing the command if the candidate disagrees with history. A command that touches no task (`system.init`, which writes into an empty store) gets the full replay instead, which costs nothing there.
3. **Option 1 plus the full consistency replay before every commit.**
4. **Explicit candidate/promotion**: commit 1 stages a complete candidate outside authoritative state; commit 2 reads it back, validates it, writes it, appends its events, records idempotency and retires the candidate; a restart must discard (or finish) staged candidates.

## Evidence / prototype

Measured on the reference installation (Ubuntu 26.04.1, kernel 7.0, ext4 on LVM; release builds; stores on disk, not tmpfs). Prototypes ran in a scratch copy; the Build Record holds the commands and output.

| | Option 1 | Option 2 | Option 3 | Option 4 |
|---|---|---|---|---|
| Crash before the commit (points 1, 2) | last known-good, retry fresh (W1) | same | same | same, plus an orphan candidate after the staging commit |
| Crash after the commit (points 3, 6) | whole change, retry replayed (W1) | same | same | same |
| Planted kernel defect (completion leaves the task pointing at its run) | **promoted**, caught afterwards; daemon quarantined, every later write refused | **refused before promotion**; last known-good kept; other writes continue | refused before promotion | not tested (validation is per candidate, same choice as 1 vs 2) |
| False refusals over the `rikurid` and CLI suites and the real-process crash matrix | — | none | — | — |
| Commits / fsyncs per create (`strace`) | 1 / 1 | 1 / 1 | 1 / 1 | 2 / 2 |
| ms per create, `synchronous=FULL` (100 / 1,000 / 3,000 tasks) | 3.2–3.3 | 3.5 / 3.3 / 3.1 | 4.4 / 8.6 / 18.6 (grows with the ledger) | 6.1 |
| Extra recovery state | none | none | none | staged candidates to discard or finish at every open |

`synchronous=FULL` versus `NORMAL` (option 1): 3.26–3.31 ms versus 0.39–0.42 ms per create, about 2.9 ms per commit for the fsync. `NORMAL` in WAL mode survives a killed process but may lose the last commits on power loss.

Note 2026-10-10 (verifier round 1): option 4 was prototyped for `task.create` only and run at its own crash points (before staging, after staging, mid-promotion, none), each recovering to one task with a clean replay. Its cells in the two crash rows are mapped from those points, not run at W1's points 1, 2, 3 and 6. The harness, the switches and the commands are now committed under [the E1 evidence directory](../../build-records/evidence/2026-10-09-s00-11-e1/README.md), with a code comparison.

## Decision

- **Keep one SQLite transaction per kernel command; drop explicit candidate/promotion for kernel transactions** (option 4 adds a commit, an fsync and a recovery state without adding a guarantee for a single-store change).
- **Add the scoped candidate check before the commit (option 2)**: TSE's "validate candidate" step, so a wrong candidate is refused while the last known-good state is still authoritative, instead of being promoted and quarantining the daemon. Its cost is within measurement noise; the full replay before every commit (option 3) is rejected for growing with the ledger. *Clarified 2026-10-10: both claims were wrong for option 2 as built (its cost grew with the touched task's history, and it missed defects outside the tasks its events named); it is replaced by validating what the command wrote, below.* The post-commit replay at open and on `system.health` stays as the second line, for drift that does not come from a command.
- **Keep `synchronous=FULL`**: "committed means durable" includes power loss; 3.3 ms per command is acceptable for a personal kernel.
- **Explicit candidate/promotion remains the design for multi-store or multi-file transformations** (backup and restore, Stage 01; migrations that move data between stores), where one SQLite commit cannot be atomic (ADR 0002 consequences).

TSE step by step for kernel transactions (evidence in the Build Record): preserve old state (WAL; points 1–2 leave the last known-good state, T00.07); validate inputs (kernel parameter and version checks); transform a complete candidate (typed plan); validate the candidate (typed construction, the repository's read-back, and the write-set check, see the clarification); durably write (WAL with `synchronous=FULL`, pinned by the storage migration tests; power loss is Not yet proven); read back and verify (`a_write_that_does_not_read_back_rolls_back`; the read-back runs inside the transaction, before the commit makes it durable, so it checks what the transaction holds, not the bytes on disk, and §43's order "durably write → read back" is collapsed into one commit); atomically promote (the commit, crash-point placement test); retire old state (the WAL checkpoint; earlier object versions are not kept, the ledger keeps the events that produced them); retryable after a pre-commit failure (idempotency, T00.06–T00.07).

## Consequences

- The kernel gains one query set per command (the touched task's events by index, its runs). Implementation is S00.11's build step, tests first, with mutants: every check it runs must bite. *Clarified 2026-10-10: the write-set check reads instead the command's own write list, the events it appended (by sequence number) and an index-only count per written task; see the clarification.*
- A kernel defect that produces an inconsistent candidate now fails that command with an internal error and leaves the store writable, rather than quarantining the daemon. The failure is visible in the response and in health (the store's write evidence), not silent.
- Power loss itself is not tested (only killed processes are); durability on power loss rests on SQLite's documented guarantee with `synchronous=FULL` (Not yet proven by a Rikuri test).

## Security / privacy impact

None: no new data, interface or authority.

## Data / migration impact

None: no schema change. *Clarified 2026-10-10: `ledger` migration 2 adds one index, `events_by_task_type` ([ADR 0003](0003-ledger-and-idempotency.md) note), and `storage` migration 2 adds one, `objects_by_deletion_unit` ([ADR 0002](0002-sqlite-current-state-model.md) note); both additive, no row rewritten, and existing stores upgrade at open.*

## Rollback or reversal path

The scoped check is one call in `Kernel::consequential`; removing it restores option 1. Switching to candidate/promotion later is possible because idempotency already makes every command retryable. *Clarified 2026-10-10: the write-set check that replaced it is a few calls in one place there (`candidate::check_command`, or the full replay and `candidate::check_init` for `system.init`, then `candidate::check_answer` and the idempotency-record checks); removing them restores option 1, and the write list and the indexes can stay, being additive.*

## Follow-up tests

- The scoped check's own tests: each comparison it runs refuses a planted defect before promotion and passes every legitimate history (the full suites). *Clarified 2026-10-10: now the write-set check's rules, each with its own unit test, the kernel-level `candidate_check.rs`, and the planted defects D1–D8, V1–V9, V11, N1, N2, N5, N6, N8, N9, X2–X4, R1, R3 and S1 in the E1 evidence (the patch and, for N and X, the daemon driver).*
- All eight crash points rerun at Stage 05 and release (Stage Plan T00.07).
- A power-loss drill (for example a VM cut) before public release, owned by the release gate.

## Clarification 2026-10-10: validate what the command wrote

The independent verification of option 2 as first built (E1 record, round 1) disproved two claims above, with measurements reproduced on the reference installation:

- **Cost.** The scoped check read every event of the touched task on each command, so it grew with that task's history: 7.7 ms per step after 1,000 steps on one task and 22.9 ms after 6,000, against 3.2–3.4 ms without a check.
- **Coverage.** Its scope came from the tasks the command's events named, so a defect that wrote something else (an extra run nothing names, another task's step count) was still promoted and then quarantined the daemon.

The owner chose to redesign it ("Redesign it now"), saw a scratch prototype, and approved landing it ("Land it"). **The pre-commit check now validates what the command wrote and appended, and the idempotency record it wrote:**

- `rikuri-storage` lists, per transaction, every object written through a repository with its body before and after (`Tx::written`, `Written`); an update costs one more indexed read for its before.
- `rikurid`'s `candidate::check_writes` reads both lists from the transaction itself, the write list and the events appended since the command began (`rikuri_ledger::after`, from the ledger's newest sequence number at the start), so a write or an append that the command's plan does not list is checked too. Its rules:
  - every event appended is about a task (and, except `task.created`, a run) the command wrote, and no two carry the same type and idempotency key;
  - every existing task or run written is named by one of those events, and every new one has exactly its creation or start event;
  - each change matches its events: a new task or run starts empty, step and run counts move by exactly the events appended, a completion links its own `run.completed` and `run.verified`, records what they say, and the verification follows the completion; a completed run is not changed;
  - a task that leaves its current run ends that run in the same command;
  - each written task holds the task ↔ run pointer invariants after the change, and each written run that is still running has no completion link, outcome or verification;
  - each written task's counts match the ledger, counted through a new index;
  - object kinds and event types without rules are refused;
  - each task command's exact effect (`candidate/spec.rs`, `candidate::check_command`): the event types it appends, in order and how many; each event by the owner, succeeded, under the command's idempotency key, with exactly the command's references and metadata; exactly the task and run it writes; and every field of them equal to the before-state with only that command's own changes applied (the frame rule). A command with no spec is refused. A step or completion must name the task's own running current run, as the kernel's request validation requires; a new task's deletion unit must be its own, counted through a new storage index (forgetting one task must not forget another's content).
- With idempotency enforced, the record the command wrote under its key must be its own (that command and request) and point at its first event (`candidate::check_recorded`). The records the command wrote are read from the ledger by rowid from a mark taken when it began (`rikuri_ledger::record_mark`, `records_after`): with idempotency enforced exactly one, under the command's key, storing exactly the response the command returns; with it off, none (`candidate::check_record_written`). Both checks are chosen by the kernel's idempotency mode, never by its own note that it wrote a record.
- The answer a command returns, which with idempotency enforced is also what its record stores and every retry returns, must name exactly what it wrote: its task, run, version and events, and the steps, outcome and verification it reports (`system.init`: its owner, Inbox and event) (`candidate::check_answer`). Every event the command appended must carry the ledger's current schema version (`rikuri_ledger::EVENT_SCHEMA`).
- `rikuri-ledger` migration 2 adds the index `events (task_id, event_type)`, so `rikuri_ledger::task_counts` is answered from the index alone.
- `rikuri-storage` migration 2 adds the index `objects (deletion_unit_id)`, so `rikuri_storage::deletion_unit_size` is an index lookup.
- `system.init` (which writes the owner and project into an empty store) keeps the full replay before its commit, chosen by name, and has its own spec (`candidate::check_init`): exactly one new owner and one new Inbox project with default trust and intent, the owner's project being that one, and its `system.initialized` event by the owner; every other command, including a future one, goes through the write-set rules, which refuse object kinds and event types they do not cover.

Evidence (E1 record and evidence directory): thirty planted defects (D1, a completion that leaves the task pointing at its run; D2, a start that also inserts an unnamed run; D3, a step that also bumps another task's count; D4–D8, V1–V9, V11, N1, N2, N5, N6, N8, N9, X2–X4, R1, R3 and S1 below, the N, X, R and S ones through the real daemon and CLI) are each refused before promotion, with no quarantine and the other task unchanged; no legitimate command is refused across every suite and the real-process crash matrix. Cost: the index-only count grows with the written task's history, roughly 0.1 to 0.2 ms per 1,000 events on the task (from within noise to +0.7 ms per command after 1,000 steps, +0.5 to +1.0 ms after 6,000, +1.9 ms after 20,000, across rounds 2 to 7), against +2.4 ms at 6,000 without the index; the round-2 rules and the per-command specs add no measurable cost. One test per rule, per planted field and per planted defect; cargo-mutants on the check, its specs, the kernel's call, the write list, the read-back, the answer check, the deletion-unit count, the two event reads and the two record reads: 249 tested, 229 caught, 19 unviable, 1 missed (the `Store::health` fallback, outside this change and deferred in the W1 placement record).

Verifier round 2 (2026-10-10) planted five more defects that the first version of these rules passed: D4, a second start on a running task; D5, a completion link on a running run; D6, one step's event appended twice under one key; D7, an event appended inside the plan but outside its list; D8, another task's privacy class and title rewritten with no event. In the verifier's runs D4–D7 were promoted and then quarantined the daemon (with the committed harness, D5 does not quarantine, because the later completion overwrites the planted link). D8 was never noticed, because titles and privacy classes never reach the ledger, so the replay cannot see it either. The owner chose to close them ("Close them now"); the rules for named writes, leaving a current run, a running run's links, duplicate effects and reading the appended events from the ledger were added tests-first.

Verifier round 3 (2026-10-10) found the same pattern one level down: a command could still change, on an object it legitimately writes, a field no rule names. V1, a step that downgrades its own task's privacy class from `local_only` to `normal`; V2, one that replaces its deletion unit; V3, one that moves the task to another project; V5, a start that leaves the last run stale; V6, a completion that leaves the task `created`; and smaller ones (a wrong attempt, an event marked failed, a hidden extra task, a twin event under another key). Most of these the replay can never see. Rather than add one more rule per finding, the owner chose per-command effect specs with a frame rule ("Per-command effect specs"), which close the class: anything a command's spec does not change must stay as it was. Two cases remain outside the check's design and are recorded in the E1 record: a write made with raw SQL around the repository (V10), and whether a command did what its request asked (V12, for example recording the outcome the client sent), which each command's own tests cover.

Verifier round 4 (2026-10-10) found what the first specs still allowed. N1, a step that names another task's running run: the specs checked the fields of whatever run the event named but not that it was the task's own running current run, so it was promoted, and the replay, which counts steps per run without regard to task, never saw it; N2, the same for a completion, which then quarantined the daemon. Smaller ones: N5, a new task sharing an existing task's deletion unit; N6 and N9, the idempotency record missing or written under another command, which silently breaks retries; N8, `system.init` creating the Inbox trusted for execution. The owner chose to close them all ("Close now", "Close all now", with storage migration 2 for the deletion-unit index). Outside the check, recorded in the E1 record: an event's `recorded_at`, which comes from the kernel's clock and is taken as appended (N7), and its siblings (an owner's `initialized_at`, objects' stored times, a record's `recorded_at_ms`).

Verifier round 5 (2026-10-10) found the task and run side holding against every new defect, and three gaps left in the idempotency record, all Minor: X4, a command that writes no record and also clears the kernel's own note that it did, which skipped the record check (a retry then duplicated the task); X2, a record storing another task's ID as its response, so a retry answered with that task; X3, an extra record under another key, which later refused a legitimate command. The owner chose to close all three ("Close all three now"): the records a command wrote are read from the ledger and checked by the idempotency mode, as above.

Verifier round 6 (2026-10-10) found two more, both Minor: R1 and R3, a command whose answer names something other than what it wrote (a create answering with another task's ID, `system.init` with another project's), which the record then stores consistently, so the first call and every retry give it and the client's next command targets the wrong task; and S1, an event stamped with another schema version, which only a defect below the kernel could do. The owner chose to close both ("Close both now") and set the rule that ends E1: **the first verifier round with no Critical or Important finding ends it**; its Minor findings are fixed if they are documentation errors and otherwise recorded as owned debt with a reactivation condition.

Verifier round 7 (2026-10-11) found no Critical or Important finding, so E1's verification ends there. Its one Minor coverage gap is recorded as owned debt in the E1 record: the check runs once, after the plan, the events and the record, so a write made where a replay returns early, or after the check, would not be validated. Today's code writes in neither place, and the natural forms of that mistake fail existing tests.

**What changes in meaning.** Drift in an object the command does not write (for example an earlier run of the task) is no longer judged before the commit; the full replay finds it at open and on `system.health`, and quarantines writes, as before E1. Drift carried in a written task's counts or pointers is still refused before the commit. A written run's step count is checked as a change (it moves by exactly the step events appended), not against the ledger's total for that run: drift already in a run's total is the replay's to find (owner decision 2026-10-10, which avoids a second index for run totals).
