# Build Record — Shared guard sync: the fix-lock covers Rust test modules and files that start `#![cfg(test)]`

| Field | Value |
|---|---|
| Change | `.claude/hooks/guard.py` and `test_guard.py`: the shared guard (identical in this repository and rikuripi/rikurinode) follows `mod x;` declarations made under `cfg(test)` and refuses, under a held fix-lock, edits to every file such a declaration may name; `#![cfg(test)]` counts as a test marker, and the `mod` declaration of a file that is all tests is locked in its parent; the inline-test tripwire names any such file the lock would miss. `.claude/hooks/guard-config.json`: the `inline_tests` rule takes `#!` as well as `#` (`starts_at`, `attached_above`), none of its patterns puts two `\s*` side by side any more (`guarded_lines` too), and `_inline_tests_doc` says so. `.claude/settings.json`: the guard hook blocks the call when it fails or runs out of time (`"onFailure": "block"`) |
| Date | 2026-10-11 to 2026-10-12 |
| Author / agent session | Claude Code session for Henry Chen (side session `sad-moore-14776b`, started from the Stage 00 session) |
| Branch | `fix/guard-rust-test-layouts-sync`, on `main` `1ea439d` (PR #32, which holds exactly the tripwire's guard files, rikuripi/rikurinode `aa51a24`); built in a clean clone outside the repository and pushed from there (the app's worktree sync waits for the owner to confirm this repository's pinned origin) |
| Commits | `7103ec5` (rounds 1–4; owner, 2026-10-11: "let's commit"), `dc7e327` (round 5) and the round-6 commit that follows them; pushed (owner, 2026-10-12: "commit and push") |
| Stage Gate | N/A: guard policy only; no Stage 00 item, interface or gate status changes |
| Related ADR / requirement | AGENTS.md "Deterministic gates" (tests and baselines read-only under the fix-lock; test code includes inline `#[cfg(test)]` modules); origin record rikuripi/rikurinode `docs/build-records/2026-10-11-guard-rust-test-layouts.md` (requirements R1–R12, design, mutants, verifier reports) |
| Status | Round 6 (the owner's recalibration after verifier round 3) built, tested (§4), committed and pushed; independent verification: rounds 1–3 FAIL, answered; round 4 next (origin record); the PR waits for the owner |

**Risk note (protected path):** the guard and its policy are protected. The change only tightens: new denies apply only while a fix-lock is held (off by default here while scaffolding, PR #27), and only to files the `inline_tests` glob (`*.rs`) names. With no lock held nothing changes. The rule's patterns match exactly what they matched before, plus `#!`, and none puts two `\s*` side by side any more (runs of spaces stay fast); other slow inputs of its older patterns are split into their own change (Skipped), and the hook now blocks the call if it ever runs out of time.

## 1. Plan

- [x] One-sentence statement of change: copy the shared guard files unchanged from rikuripi/rikurinode, widen this repository's `inline_tests` rule to `#\s*(?:!\s*)?\[` in `starts_at` and `attached_above`, write `guarded_lines`' `\s*!?\s*\(` as `\s*(?:!\s*)?\(` (owner's choices 1(a) and 2(a), 2026-10-11; verifier round 2, F1), and make the guard hook block when it fails (owner, 2026-10-12, "Recalibrate, then finish").
- [x] Blast radius: Claude Code sessions in this repository, only while they hold a fix-lock. No product code, canonical data, money, execution or privacy path. Not irreversible.
- [x] Current behavior characterized before changing: with the rule as it stood and a held lock, an Edit inside a test module kept in its own file (`#[cfg(test)] mod tests;` → `tests.rs`) or inside a file starting `#![cfg(test)]` is allowed, and the tripwire reports nothing (origin evidence `gap_probe_before.out`). Here no tracked file is a test module, starts `#![cfg(test)]` or uses `#[path]`; all 25 `mod x;` declarations sit above the first test marker (`survey_mods.out`, `main` `1ea439d`), so no locked edit here changes verdict (`real_repos_probe.out`: test modules none, tripwire `[]`). The rule as on `main` already had one slow pattern: `guarded_lines`' `\s*!?\s*\(` took 1.3 s on a line of `cfg` and 40,000 spaces (`sync-Rikuri-PI-rule-not-widened.out`).
- [x] Verification defined up front: `cmp` of the shared files; the guard suite with this repository's own policy on Python 3.14 and 3.9.25; this repository's full gate.
- [x] Architecture-impact check: invariant / ADR none; API / schema N/A; canonical data N/A; privacy / instruction authority / security: tightens the AI-session tripwire, nothing loosened; deletion / backup N/A; budget N/A; replay N/A; documentation: `guard.py`'s header and `_inline_tests_doc`; module ownership N/A.
- [x] Relevant threat/control/test: a session turning a failing test green by editing a test module file or an inner-attribute test file instead of the code, by switching such a file off from its parent, or by stalling the hook past its timeout. Control: the shared guard's test-module lookup and the rule. Tests: the shared suite (`TestModuleFiles`, `InlineTests.test_a_file_that_starts_cfg_test_is_tests`, `InlineTestCoverage`, `GenericPolicy.test_inline_rules_find_a_tests_only_file` and `test_inline_rules_take_linear_time`, which read this repository's own rule).
- [x] Existing solution review: the rule is the one this repository adopted in PR #31 (AISecurity's and rikurinode's), changed only in the three patterns above.
- [x] Authoritative state change? N/A.

**Behaviour changes to expose (owner's choices, reversible):** under a held fix-lock, (1) an Edit/Write to a `.rs` file declared as a test module (`mod x;` under `cfg(test)`, or declared inside a test module) is refused, in every place rustc may read it from — both the folder named after the declaring file and the declaring file's own folder ("Lock both places", 2026-10-11; a production file that shares a test module's name there is read-only too while the lock is held); (2) a file that starts `#![cfg(test)]` is test code from that line, and its `mod` declaration in the parent may not be removed or given new attributes — nor may the parent's own inner attributes change (`#![allow(…)]` at the top of `lib.rs` while it declares such a file: a cost the origin record lists). Undo: restore the two files and the rule.

## 2. Build

- [x] Implementation: the shared files, byte-identical to the origin's (guard.py `c884a868…`, test_guard.py `42a7d741…`; guard_base.py `1ef6f5cc…` unchanged). `.claude/settings.json`: one line, `"onFailure": "block"` beside the hook's `"timeout": 10` (Claude Code 2.1.295 or later; this machine runs 2.1.296), as rikuripi/rikurinode has it. `guard-config.json` against `main`: two lines change — the rule line (three patterns) and the `_inline_tests_doc` line (one sentence added on what the guard covers; "copied unchanged" now says when and how this repository's copy changed) — by a checked text replacement (each text found once; the parsed rule equals the old one but for the three pattern spellings; the file keeps its escaped invisible-character ranges as text, and no invisible character was added).
- [x] Inputs validated: `load_config` already validates the rule (shape, regex compile).
- [x] Old implementation removed: nothing left beside the new code.
- [x] Module Contract: N/A (no crate changes).
- [x] Security question answered: no auth, money, execution or private data touched; the change only adds denies under a held lock, and a check past the guard's time budget asks the owner.

## 3. Debug

- [x] Reproduced, failing tests first, root-cause class, siblings: the origin record, §3. Here: the round-5 shared files with the rule as committed in `7103ec5` fail exactly the linear-time test (`round5-red-Rikuri-PI.out`: `starts_at` took 1.47 s), and pass once the rule is rebuilt; the round-6 shared files with the settings as on `main` fail exactly the new `GenericPolicy.test_the_guard_blocks_when_it_fails` (`round6-red-Rikuri-PI.out`), and pass once the hook blocks.

## 4. Verify

- [x] `cmp`: guard.py, test_guard.py and guard_base.py identical to the origin's.
- [x] Guard suite with this repository's own policy: `Ran 142 tests … OK` on Python 3.14 (origin evidence `sync-Rikuri-PI.out`) and on 3.9.25 (`py39-Rikuri-PI.out`), inside rikuripi/rikurinode's `safe-run.sh`, lnd's start time unchanged.
- [x] The rule change is needed: the same shared files with the rule as on `main` fail `GenericPolicy.test_inline_rules_find_a_tests_only_file` and `test_inline_rules_take_linear_time` (`sync-Rikuri-PI-rule-not-widened.out`).
- [x] Full gate on a private clone of `dc7e327` with round 6 committed (never pushed), inside `safe-run.sh` with no network (origin evidence `gate-Rikuri-PI.out`): tools, gate self-tests, guard, format, build, clippy, test, smoke and crash matrix PASS; the gate script then stops at its dependency step, `cargo deny` being unable to fetch the advisory database without network, so that step and the five after it were run one by one: `cargo deny --locked --offline check` advisories, bans, licenses, sources ok against the database cached 2026-10-09; secrets, ratchet, contracts, crate-policy, scrap PASS.
- [ ] Independent verification: the origin record's verifier, round 4 next.
- [x] Diff matches the plan: the two shared files, two lines of `guard-config.json`, one line of `.claude/settings.json`, this record.

## 5. Close out

- [x] Docs updated: `_inline_tests_doc`; `guard.py`'s header. AGENTS.md already says tests and inline test modules are read-only under the lock (no change).
- [x] Scrap Sweep: nothing task-made left in this repository; the clean clone lives outside every repository and is deleted at close-out.
- [x] Committed and pushed (owner, 2026-10-11 and 2026-10-12).
- [ ] Pull request: the owner's call.

## Skipped / Deferred

- **Pull request**: the owner's call. This lands directly on `main` (the tripwire is already there, PR #32). The reworked line continuation (rikuripi/rikurinode `92c39a2`) changes the same two files; whichever lands second is re-applied and rerun. Owner: Henry.
- **Independent verification**: open until the origin record's verifier round 4 reports; the stop rule (no Critical or Important finding ends the change, with the owner's recalibration: a stall is Important, a deliberate multi-step evasion Minor) decides.
- **The rule's older patterns** (verifier round 3, I5: `#[cfg(any(` + 30,000 × `(test)` or a line of 20,000 × `cfg(` still takes 16–18 s in one regex call, as before this change): split into their own change by the owner's choice, proposed as a separate session. With the hook now blocking on failure, such a stall closes the gate instead of opening it. Owner: Henry.
- **Known limits** (none used here today): listed in the origin record's Skipped, each marked whether the tripwire names it.
- **CI** (the `Full gate (Ubuntu)` check, and a fresh advisory database): Not yet proven until the pull request's run is observed.
