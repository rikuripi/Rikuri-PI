# Build Record — The fix-lock is off by default while the code is scaffolding

| Field | Value |
|---|---|
| Change | AGENTS.md: fixing a defect no longer requires the fix-lock; red-before-fix evidence, checked by the verifier, stands in for it until the owner turns the lock back on |
| Date | 2026-10-10 |
| Author / agent session | Claude Code (Opus 5.5) for Henry Chen |
| Branch | `docs/fix-lock-optional`, begun on `main` at `48bb7ee`, moved onto `1e96a54` (PR #26 merged during verification; it touches none of this change's files) |
| Commits | one commit on the branch (this record included) |
| Stage Gate | [Stage 00](../stage-gates/foundation0/00-bootstrap-kernel.md), working rules (no Stage Gate item changes) |
| Related ADR / requirement | AGENTS.md > Deterministic gates and law 5 (defects become tripwires); owner rule "a choice not exposed is not a choice" |
| Status | COMPLETE locally (verifier PASS; gate steps all PASS, in two runs); CI and the owner's merge Not yet proven |

**Risk note (protected paths):** `AGENTS.md` is this repository's constitution and `.claude/agents/verifier.md` is part of the Claude setup. This change relaxes a working rule; it changes no gate, CI step, guard code or guard policy.

## 1. Plan

- [x] One-sentence statement: during scaffolding, a defect fix needs a red test shown before the fix and an explained diff of test changes, not a fix-lock.
- [x] Owner decision (2026-10-10, in this session): "Let's not put on fix-lock while we're still scaffolding at this stage … modify AGENTS.md so you can fix without needing to lift fix-lock … Once this app is more mature, we could turn it back on to protect what's built and working." Asked how to do it, the owner chose "Do it here"; asked when the lock comes back, "When Stage 00's gate closes (Recommended)"; asked about committing, "Yes, commit+push+PR (Recommended)" once the verifier passes. The owner confirms on the PR by merging it.
- [x] Why: the lock froze tests in the middle of fast change, needed an owner round-trip for each release, and blocked fixes whose defect is in test code (PR #26: the leak was in `reentrancy.rs`, a test file, so the lock would have frozen the fix itself).
- [x] What the lock protected, and what replaces it: a fix reaching green by weakening its test. In its place: the failing test committed before the fix (or the red run recorded when a commit is impractical), the fix in its own commit, a Build Record reason for every change to test code or baselines from the red commit to the change's last commit, and a verifier step that checks it (a test or baseline loosened without a reason that holds is a Fail). "Test code" is named (`tests/` files, inline `#[cfg(test)]` modules, `rikuri-testkit`, gate self-tests) because the lock's own `test_globs` never covered inline tests either (`guard-config.json` has no `inline_tests` key). PR #26 is the worked example (`git diff ca737b4 5f50728 -- scripts .github` empty; the regression test identical in both).
- [x] Unchanged: the guard and its policy (a session can still engage the lock; while engaged it behaves as before), denies ("stop and ask the owner"), protected-path asks, the development profile, all gates.
- [x] Reversal: the lock becomes the default again when the Stage 00 gate closes (owner, 2026-10-10), written into AGENTS.md. Nothing mechanical reminds anyone at that point: the Stage 00 close-out must include it (recorded as a follow-up below; the Stage Gate Record is not edited here because PR #25 is editing it). One bullet in AGENTS.md and one step in `verifier.md` revert it.
- [x] **I will know this works when** AGENTS.md states the default, the substitute evidence and the reactivation condition; the verifier's instructions include the weakened-test check; the guard tests, links, `git diff --check` and the full gate pass; the verifier finds no unresolved Important finding.
- [x] Architecture impact: none (working rules only). Authoritative state change: none.

## 2. Build

- [x] `AGENTS.md` > Deterministic gates: the "Fixing a defect" bullet rewritten (default off, the evidence that stands in, the lock still available, back on when the Stage 00 gate closes).
- [x] `.claude/agents/verifier.md`: step 4, "For a defect fix, check the test was not weakened" (including: build the red commit in its own target directory, since a shared one can rerun a stale binary).
- [x] `scripts/quality/ratchet.sh`: a comment now says an *engaged* fix-lock makes the baseline read-only (it no longer always is).
- [x] Not changed, still accurate: the guard's description in AGENTS.md ("test edits while the fix-lock is engaged"), the verifier's rule never to engage or release the lock. Passages elsewhere that describe the current fix-lock (Architecture §16 clarification and repair-lifecycle note, the PRD's SEC-013–019 clarification, the Test Strategy's 2026-09-27 note, `doc/README.md`, ADR 0000, the Master Build Sequence) describe the mechanism and its owner-approved release, which are unchanged; none says a fix must engage it. (A first draft of this line called them all future product design; verifier round 1 showed several describe the current rule.)

## 3. Debug

N/A: not a defect.

## 4. Verify

- [x] Guard tests; `git diff --check`; local links; result: `Ran 91 tests … OK`; no whitespace errors; all 13 local links in the changed files resolve. The real guard profile is still `development`.
- [x] Full gate (outer safety layer; steps 10–15 with the host network shared, as `cargo deny` cannot fetch without it); result: steps 1–9 PASS, `dependencies` unable to fetch without a network, then dependencies, secrets, ratchet, contracts, crate policy and scrap PASS. `lnd` start time unchanged (`2026-10-09T20:20:27.903167252Z`).
- [x] Independent verifier round 1 (**PASS**, on `48bb7ee` plus the first draft; same layers, `lnd` start time unchanged): guard tests 91 OK, links (and the two inbound links to `AGENTS.md#deterministic-gates`) resolve, all 15 gate steps pass split as above; nothing still says the lock is mandatory for a fix; guard code, policy, settings, CI and scripts untouched; the worked example holds (PR #26's red commit failed 5 of 5, the fix passed 5 of 5; the regression test byte-identical in both). It ran the new step 4 itself and was first misled by a shared target directory rerunning the red binary. Nits: (1) "the test files" undefined (inline `#[cfg(test)]` modules, `rikuri-testkit`); (2) the diff window stopped at the fix commit, missing later test changes in the same change; (3) "until the owner releases it" is instruction, not enforcement, under the development profile; (4) this record mischaracterised other documents as future design only; (5) the two files worded the Fail rule differently, and neither said how to confirm a red commit; plus: nothing reminds anyone when Stage 00 closes, and `ratchet.sh`'s comment overstated the lock. Not yet proven: CI, the change on the new `main`, the owner's confirmation.
- [x] Responses, round 1: (1), (2), (5) both files now name the test code, diff `<red>` against the change's last commit, use one Fail wording, and the verifier builds the red commit in its own target directory; (3) reworded ("until it is released"); (4) corrected above; the reminder is a follow-up below; `ratchet.sh` comment corrected. The branch moved onto `1e96a54`.
- [x] Gate on the new `main` with the round-1 changes, same layers; result: steps 1–9 PASS (now including PR #26's `RESIDUE none`), `dependencies` unable to fetch without a network, then the six remaining steps PASS with the host network shared; `lnd` start time unchanged. The round-1 responses are wording in the same three documents and this record; no second verifier round was run.
- [ ] CI: Not yet proven.

## Follow-ups

- When the Stage 00 gate closes: turn the fix-lock default back on (revert the AGENTS.md bullet's scaffolding clause and `verifier.md` step 4's "while off" sentence), as decided 2026-10-10. Owner: Henry; whoever closes Stage 00 puts it on the close-out list.
