# Build Record — Mission Charter and platform independence

| Field | Value |
|---|---|
| Change | Adopt the owner's mission as a Mission Charter and fold its new promises into the PRD, Architecture, Test Strategy and ADR 0000 |
| Date | 2026-10-08 |
| Author / agent session | Claude Code (Opus 5.5), owner-requested documentation change |
| Branch | `docs/mission-charter` |
| Commits | The commit containing this record on `docs/mission-charter` ("docs(mission): adopt the Mission Charter …"), at the owner's request; push and PR wait for the owner |
| Stage Gate | Documentation refinement; no gate closure; Foundation-0 stages 00–05 unchanged |
| Related ADR / requirement | ADR 0000 October 8 clarification; PRD P32–P34, MISSION-001–006, §22 rows; Architecture §26.5; Test Strategy §20 |
| Status | COMPLETE — documentation only; runtime Not yet proven; committed, not pushed |

## 1. Plan

- [x] One-sentence statement of change: the owner's mission (Rikuri expands what a person can imagine, understand and accomplish, and frees them from any one model, OS or ecosystem) becomes a Mission Charter, and its genuinely new promises enter the maintained baseline under the September 28 freeze without changing Foundation-0.
- [x] Blast radius: planning documents only. No code, schema, canonical data, auth, money, execution or private-data path changes. Protected paths touched: ADR 0000 and the Test Strategy (owner chose this change on 2026-10-08; guard profile `development`).
- [x] Current behavior characterized before changing; evidence: the owner supplied "Proposed Addendum A v1.0" (dated 2026-10-07) in conversation on 2026-10-08, with the framing that Rikuri should free people from being trapped by any one OS or ecosystem and expand their horizon, not serve as an OS vendor's orchestration layer. The comparison found most of the draft already in PRD §1–2, P1/P2/P9/P13/P14/P30/P31, STUDIO-005/009, GOV-010, MACH-004/005 and Architecture §26; see the [review](../../doc/reviews/2026-10-08-mission-addendum-review.md). Initial `git status`: only the unrelated untracked `rikurinode.code-workspace` (preserve it).
- [x] Owner decision, 2026-10-08: "Fold in + keep charter" (offered against fold-in only, adopt as a governing addendum above the PRD, or parking lot only). Default stated to the owner and not overridden: Foundation-0 stages and gates unchanged; ARM64/Windows CI goes to the parking lot.
- [x] Verification defined up front — I will know this works when: the charter states its role (rationale; PRD and Architecture govern requirements); every new PRD promise has an ID, an owning stage and future allowed/adverse acceptance in Test Strategy §20; ADR 0000 records the decision with no Foundation stage, gate, S/T/E ID or date change; new features sit in the parking lot rather than the PRD; the draft's conflicts (A.11 scope, A.3.5/A.6 inference timing, A.7 slogan collision, paste residue) are corrected and listed in the review; local links and anchors resolve; `git diff --check` passes; the guard tests pass; and the independent verifier finds no unresolved Important findings.
- [x] Architecture-impact check completed:
  - [x] invariant / ADR — ADR 0000 dated clarification; no invariant weakened. New constraint: vendor AI layers never hold canonical state; a hybrid vendor runtime counts as a cloud path for privacy until its local-only mode is proven.
  - [x] API / schema / event format — N/A, no contract changes.
  - [x] canonical vs derived data — reinforced: vendor runtimes are replaceable resources, never canonical holders.
  - [x] privacy / instruction authority / security — Local Only is not weakened; vendor hybrid runtimes are classified as cloud paths unless proven local.
  - [x] deletion / backup / restore — N/A.
  - [x] budget / cost — N/A; vendor adapters use the existing budget path.
  - [x] nondeterministic AI / replay — N/A.
  - [x] documentation / help / RAG — documentation index updated.
  - [x] module ownership / dependency direction — N/A at runtime; Architecture §26.5 keeps architecture- and vendor-specific code in adapters.
- [x] Relevant threat/control/test identified: Option C (SEC-020–024) and the Local Only no-network envelope govern vendor hybrid runtimes; Test Strategy §20 maps future acceptance.
- [x] Existing solution review: N/A, no infrastructure selected. Vendor runtimes named as examples only; none is adopted or installed.
- [x] Authoritative state change: N/A.

## 2. Build

- [x] Smallest coherent implementation completed: charter, review, PRD (P32–P34, §1 Mission block, §22 rows, §29, table of contents including the previously missing §28 entry), Architecture §26.5, Test Strategy §20, ADR 0000, documentation index, parking lot, this record.
- Inputs crossing trust boundaries, typed code, Module Contracts: N/A (documentation only).
- [x] Security question answered: no auth, money, execution or private-data path changes now. The new vendor-runtime privacy rule is future acceptance in Test Strategy §20.

## 3. Debug — N/A

Not a defect repair. No fix-lock needed.

## 4. Verify

- [x] Local link and anchor check: Pass (two independent checkers, 0 broken).
- [x] `git diff --check`: Pass, including the three new files (intent-to-add).
- [x] Guard tests: Pass, 91 tests OK (no `.claude/` change).
- [x] Full gate `scripts/quality/full-gate.sh`: Pass, 13 steps. It checks code, the guard and committed history, not document content.
- [x] Independent verifier, three rounds of Bugs, Security & boundaries and Compliance passes: round 1 FAIL (six Important findings), round 2 FAIL (one new Important finding introduced by a round-1 fix), round 3 PASS. Findings and corrections are listed in the [review](../../doc/reviews/2026-10-08-mission-addendum-review.md#verifier-round-1--october-8) and the evidence log below.
- Unit, contract, replay, running-system and recovery checks: N/A, no executable behavior changes.

## 5. Close Out

- [x] Scrap Sweep: tracked diff is exactly the nine planned files. Task-created scratch files (link checker, gate logs) live in the session scratchpad outside the repository. The unrelated untracked `rikurinode.code-workspace` is preserved. The three new files are marked intent-to-add (`git add -N`) so `git diff --check` covers them; nothing is staged with content.
- [x] Documentation index updated (Mission section and October 8 paragraph).
- [x] Traceability: MISSION-001–006 → Test Strategy §20; ADR 0000 October 8 items 1–5.
- Stage Gate Record: N/A, no implementation gate affected.
- [x] Commit: at the owner's request on 2026-10-08, the nine task files only. Push and PR wait for the owner; `main` requires a PR with the `Full gate (Ubuntu)` check.

## Skipped / Deferred / Resequenced — visible, never silent

| Item | Disposition | Reason | Dependency impact | Owner | New stage/ordinal | Reactivation / acceptance condition |
|---|---|---|---|---|---|---|
| ARM64 and Windows build/test of the platform-neutral core in CI | Parking lot | New feature under the baseline freeze; CI cost and bash-based gate scripts need evaluation | None on Foundation-0 | Product owner | Earliest: a non-required job during Foundation-0 | During Foundation-0 through a dated ADR under the Stage Plan's document control; afterwards through the Master Build Sequence protocol |
| NPU adapters, prefill/decode disaggregation, cross-device execution | Parking lot | Charter allows them "where measurements justify"; no measurement yet | None | Product owner | Product Stage 4 Model Lab or later | Measured benefit against a simpler baseline |
| Vendor AI runtime adapters (OS-integrated runtimes, NPU frameworks) | Parking lot for the adapters; constraint adopted now as MISSION-004 | Adapters are features; the constraint is a promise | None | Product owner | Product Stage 4; Windows ones Stage 9 | Test Strategy §20 MISSION-004 rows |
| Creative production system (owner's October 8 diagram: human director, Rikuri orchestrator, Blender/Krita, Resolve/Kdenlive, REAPER/Ardour, editors/Inkscape, shared production foundation) | Parking lot | Future capability under the baseline freeze; four questions open before admission: production-mode authorship, script-bearing project files, voice likeness consent, per-application license and API | None on Foundation-0 | Product owner | Product Stage 5 adapters, Stage 6 orchestration, Stage 8 media | Master Build Sequence protocol with decision-test answer; the open questions resolved by ADR |
| General collaboration-mode selector across domains | Parking lot | Stage 2 plans only creative-mode advice and teaching (WIDE-006) and text questions (WIDE-007); purpose presets are post-Foundation recipe configurations | None | Product owner | Product Stage 2 or later | Master Build Sequence protocol |
| Measurement method for capability expansion, independent understanding and creative realization | Deferred to research | Needs opt-in evidence without trait inference | None on Foundation-0 | Product owner | Product Stage 10 research | Method declared with baseline before any claim |

Execution evidence fields: N/A, documentation-only.

## Evidence log

```text
full-gate.sh (first run, before verifier fixes)  RESULT: PASS (13 steps), exit 0; guard: Ran 91 tests, OK
verifier round 3 (after all fixes)               full-gate.sh RESULT: PASS (13 steps), exit 0; guard 91 tests OK;
                                                 git diff --check exit 0; gitleaks dir on 9 files: no findings;
                                                 local links 185 checked, 0 broken (verifier's checker)
own link checker (final tree)                    links checked: 226, broken: 0
git diff --check (final tree, incl. new files)   exit 0
git diff --stat                                  9 files changed, 662 insertions(+), 2 deletions(-)
scripts/quality/smoke.sh                         Not yet proven: does not exist yet (S00.07)

Verifier round 1 FAIL, six Important:
  MISSION-006 Stage 2 via purpose presets (unsupported) -> Stage 2 keeps planned surfaces; selector to parking lot
  MISSION-003 scope and assurance vocabulary            -> new/revised availability claims; assurance separate; Stage 2 onward
  Vendor Local Only rule weaker than §27.2/§27.3        -> per-execution Denied envelope; SEC-028 retention; SEC-021 opaque workers
  MISSION-001 not wired; "Foundation-0 unchanged"       -> "Leaving the lot" records the answer; ADR item 5; README wording
  MISSION-005 no owning stage                           -> Stage 7 second device; Windows Stage 9
  MISSION-002 Stage 2 capability inference              -> only saved lessons and declared outcomes before Stage 10
Verifier round 2 FAIL, one new Important:
  MISSION-006 cited voice Ask/Dictate (Stage 8) in Stage 2 -> text questions only; voice Stage 8
Verifier round 3 PASS; remaining trivial note (ADR cost line) applied afterwards.
Owner follow-up (same day): a creative-production-system diagram, folded into the parking-lot entry only
  (no PRD/Architecture/ADR change; the parking lot carries no gate). Self-checked: links 5/0 broken,
  git diff --check exit 0. Not re-run through the verifier.
Charter correction list completeness: Not yet proven (draft not in repository; stated in the review).
```
