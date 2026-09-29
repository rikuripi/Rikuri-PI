# Build Record — Option C review follow-ups and Stage 00 readiness

| Field | Value |
|---|---|
| Change | Close the gaps found in the independent review of the Option C security change, record the owner's Foundation-0 credential/egress scope choice, and prepare Stage 00 to start |
| Date | 2026-09-28 |
| Author / agent session | Claude Code (Opus 5.5) review session |
| Branch | main; no staging, commit or push requested |
| Commits | `3b0c80e` (combined with the other September 28 Option C record because both edited the same files; owner asked to commit and push) |
| Stage Gate | Documentation only; no gate closes. Creates the Stage 00 Stage Gate Record (NOT STARTED) |
| Related ADR / requirement | ADR 0000 new dated addendum; SEC-005/020–022; S00.12, S00.13, S03.11, S04.02, E2, E5; T03.05, T03.16, T04.05; Test Strategy §8.1–8.2 |
| Status | COMPLETE — documentation only; independent verifier PASS; runtime Not yet proven |

## Intent (owner-approved)

The owner asked for a review of the Option C security update and the overall plan, accepted the review, and said: "make the changes based on your proposal. I'll start soon." The proposal had six points:

1. OpenShell's own in-sandbox policy-proposal channel must not become a second authority path.
2. Access to the container engine is root-equivalent; the reference installation must keep it away from Rikuri's own processes.
3. The plan must decide where the Pi harness runs and route every model call through `rikurid`.
4. Stage 03 grew without a date change. The review recommended option (b): Foundation-0 stays credential-free for workers and admits no permitted outbound destination for workers, so broker and permitted-destination qualification are N/A until a later stage enables such a path. The owner's "based on your proposal" is recorded as approval of (b); it is reversible (see ADR addendum).
5. Freeze the baseline, create the parking lot, and complete the Stage Plan's own start-up actions that do not need the owner.
6. Remove wording left over from the bubblewrap-first plan.

## Spec

- **Backend approval channel.** OpenShell v0.1.2 exposes `http://policy.local` inside the sandbox (`POST /v1/proposals`), auto-drafts proposals from denials, and has a `proposal_approval_mode` setting whose `auto` value merges prover-clean proposals without a person. Verified in the pinned source: `git grep` at tag `v0.1.2` finds `PROPOSAL_APPROVAL_MODE_KEY` in `crates/openshell-core/src/settings.rs:100` and `/v1/proposals` in the conformance scenarios; the default resolves to manual in `crates/openshell-server/src/grpc/policy.rs` (`resolve_proposal_approval_mode`). The archived test log shows `Flushed denial analysis to gateway proposals=1`. Required: approval mode pinned to manual at gateway scope and checked as part of the effective policy at every launch; the worker's proposal route blocked, or its proposals treated only as untrusted requests into Rikuri's approval flow; OpenShell's own approve/reject surface never used for Rikuri workers; a negative test proving a worker proposal leaves the effective policy unchanged.
- **Host privilege.** The test gateway drove rootful Docker through `/var/run/docker.sock`; the test account is in the `docker` and `lxd` groups (`id` on this host). Membership of either is root-equivalent. The supervisor container uses host networking. Required: gateway under a dedicated service account or rootless engine; accounts running `rikurid`, Pi, local model servers and tool servers outside the root-equivalent groups; the admission probe records group membership and socket ownership; the supervisor is in the trusted computing base and its reach to host loopback services is tested.
- **Pi placement and model calls.** No document states where the Pi process runs. Pi's published coding agent is understood to ship its own file/shell tools and load extensions in-process (to be confirmed by E2; Pi is not installed on this host). Required: Pi runs inside the admitted envelope, or E2 proves its built-in tools and extension loading are disabled and it holds no host authority; Pi's model provider is a `rikurid`-owned endpoint (no provider key in Pi) that binds each request to the task's grant, privacy class and budget; the Local Only profile reaches the local runtime through that endpoint, preferably over a mounted local socket, so worker network policy stays empty.
- **Foundation-0 scope (option b).** No Foundation-0 worker holds a real credential or has a permitted external destination. Cloud access stays inside `rikurid`'s provider adapter. Broker and permitted-destination rows of Test Strategy §8.2 and T03.16 are N/A in Foundation-0 and become mandatory when the first such path is enabled. Stage dates are unchanged. Cost: Foundation-0 cannot demonstrate a worker using an account, API key or web destination.
- **Readiness.** Baseline frozen for Foundation-0 execution; `docs/PARKING_LOT.md` exists; Stage 00 Stage Gate Record exists and links the live plan instead of copying it; the plan's "copy this plan" action is superseded because a copy would be a second gate board. Agreeing the demo/receipt format remains the owner's action.
- **Stale wording.** Engineering Standard, Master Build Sequence, Architecture E5 list and the Stage Plan next action describe the admission probe backend-neutrally; Architecture §46 gains an execution-containment row.

## 1. Plan

- [x] One-sentence statement of change: make the planning baseline close the backend approval, host-privilege and harness-placement gaps before S00.13/E2, record the owner's Foundation-0 scope choice, and make Stage 00 startable.
- [x] Blast radius: documentation only. Protected paths touched: ADR 0000 (new dated addendum; prior text preserved), Stage Plan, Test Strategy, Engineering Standard. No code, guard, CI, policy, dependency, host service or user data changes.
- [x] Current behavior characterized: see Spec; OpenShell source and host facts gathered by command in the review session (evidence log).
- [x] I will know this works when: every document that describes backend admission, the Pi slice or Foundation-0 credentials agrees with the Spec; no S/T/E identifier is added, removed or checked; the ADR keeps all earlier text; the evidence receipt files still match `SHA256SUMS`; local links and anchors resolve; `git diff --check` passes; guard tests pass; the independent verifier reviews all three REVIEW.md passes.
- [x] Architecture-impact check: invariant/ADR — yes (addendum); API/schema/event — N/A; canonical vs derived — N/A; privacy/instruction authority/security — yes (authority channel, host privilege, model routing); deletion/backup/restore — N/A; budget — yes (every model call through `rikurid`'s reservation path); nondeterministic AI/replay — N/A; documentation/help/RAG — index and README updated, help/RAG surfaces do not exist; module ownership — `harness-pi` and `envelope-ubuntu` responsibilities clarified.
- [x] Threat/control/test: threat model §3–§6 rows added; Test Strategy §8.1–8.2 and T03.05/T03.16/T04.05 carry the acceptance.
- [x] Existing solution review: OpenShell v0.1.2 (Apache-2.0) retained as first candidate; this change constrains its configuration, it does not replace it.
- [x] Authoritative state change: N/A (documentation).

**Protected-path risk note.** Touched protected paths: ADR 0000, Stage Plan, Test Strategy, Engineering Standard and the new `docs/stage-gates/foundation0/` record. Risks: silently narrowing a security requirement, erasing ADR history, or creating a second gate board. Mitigations: the Foundation-0 scope choice is recorded as a dated owner decision with cost, reactivation stage and reversal, and appears in the Stage Plan deferral table; ADR edits are additive only; the Stage Gate Record links the live plan and copies no checklist items; S/T/E identifiers and S/T checkbox states are unchanged; the owner authorized the change.

Planned files: this record; `doc/Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md`; `doc/Rikuri_Pi_Architecture_Spec_v1.7.md` (§13.1, §27.2, §27.12, prototypes list, §46); `doc/Rikuri_Pi_Foundation0_Rebaselined_Stage_Plan_v2.1.md`; `doc/Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md`; `doc/Rikuri_Pi_Engineering_Standard_v1.2.md`; `doc/Rikuri_Pi_Master_Build_Sequence_v1.1.md`; `doc/README.md`; `docs/security/THREAT_MODEL.md`; new `docs/PARKING_LOT.md`; new `docs/stage-gates/foundation0/00-bootstrap-kernel.md`. The archived evidence directory is not edited; receipt notes go in the threat model.

## 2. Build

- [x] ADR 0000: new dated addendum with the five decisions; all earlier text preserved (`git diff HEAD` on the ADR shows zero removed lines).
- [x] Architecture: §13.1 harness placement and model routing; §27.2 local worker reaches the local runtime only through the `rikurid` endpoint; §27.12 new "Backend-native approval channels" and "Backend host privilege" paragraphs, and the Foundation-0 credential/egress scope in "Credentials and egress"; E5 prototype line made backend-neutral; §46 execution-containment row.
- [x] Stage Plan: S00.12 (placement), S00.13 (approval mode, `policy.local`, controller account, root-equivalent groups, supervisor reach), S03.11 (ADR scope), Stage 03 adopt table (broker only when a later stage enables a credential path), T03.05 (worker proposal refused/unchanged policy), T03.16 (broker/permitted-destination rows N/A in Foundation-0), September 28 clarification note, S04.02, T04.05 (hostile inputs against Pi itself), E2 kill criterion, next actions (Stage 00 record created; probe wording backend-neutral). No S/T/E identifier added or removed; no S/T box checked.
- [x] Test Strategy: §8.1 steps 4 and 6; §8.2 backend admission, private outbound, broker authority and untrusted-input rows; review follow-up paragraph with the receipt's three observations.
- [x] Threat model: §3 concrete trusted computing base and harness placement; §4 item 3 approval channel; §5 three new threat rows; §6 receipt notes and Foundation-0 credential scope. The SHA256-covered receipt directory is not edited.
- [x] Engineering Standard and Master Build Sequence: stale namespace/AppArmor probe wording replaced.
- [x] New `docs/PARKING_LOT.md`; new `docs/stage-gates/foundation0/00-bootstrap-kernel.md` (Template B, NOT STARTED, links the live plan); `doc/README.md` index entry.
- [x] Security question: no runtime path changes. The documents now close three routes by which untrusted content could gain authority (worker policy proposals, same-user container-engine access, an unconfined harness) and state the allowed and denied acceptance for each.

## 3. Debug

N/A — planning clarification, not a runtime defect.

## 4. Verify

- [x] Documentation checks (author): links/anchors, S/T/E ID set, checkbox states, ADR history, evidence hashes, whitespace, guard tests — see evidence log.
- Runtime unit/contract/property/replay/probe/end-to-end items: N/A, documentation only; the acceptance they describe remains Not yet proven.
- [x] Independent verifier — VERDICT: PASS, no Important findings; nits fixed (see evidence log).

## 5. Close Out

- Full runtime gate: Not yet proven — `scripts/quality/*` and a Rust workspace do not exist yet (Stage 00 builds them).
- [x] Scrap Sweep: the link checker and ID lists live in the session scratchpad, outside the repository; the negative-control probe directory `.probe-tmp/` was created and removed. No other task files.
- [x] Traceability updated: threat model §5 rows map to S00.13, S03.11, T03.05, E2/S00.12, S04.02, T04.05.
- [x] Stage Gate Record: Stage 00 record created, NOT STARTED.
- [ ] Commit: not requested; the working tree also holds the uncommitted Option C change this builds on.

## Skipped / Deferred / Resequenced

| Item | Disposition | Reason | Dependency impact | Owner | New stage/ordinal | Reactivation / acceptance condition |
|---|---|---|---|---|---|---|
| Option C broker and permitted-destination qualification (T03.16, Test Strategy §8.2) | DEFERRED | Owner scope choice (b): no Foundation-0 worker holds a credential or has a permitted destination; a loopback/host-service allowance would count as one | Stage 03 dates unchanged; no credentialed or network-permitted worker demo in Foundation-0 | Product owner and security implementer | First stage enabling such a path; expected product Stage 1 (dependency fetch, dev tools) or Stage 5 (connectors) | That path's broker/destination rows pass before enablement; reversal inside Foundation-0 by a dated ADR addendum re-estimates Stage 03 |
| Agree six owner-facing stage demos and receipt format | OWNER ACTION | Needs the owner's judgement of what he can inspect | Stage 00 exit receipt format | Owner | Before first Stage 00 build change | Format recorded in the Stage 00 Stage Gate Record |

## Evidence log

Review-session facts (commands run 2026-09-28):

```text
$ git -C /tmp/rikuri-openshell-review-20260928 grep -n 'PROPOSAL_APPROVAL_MODE_KEY: &str' v0.1.2 -- crates
v0.1.2:crates/openshell-core/src/settings.rs:100:pub const PROPOSAL_APPROVAL_MODE_KEY: &str = "proposal_approval_mode";
$ git grep -n '/v1/proposals' v0.1.2 -- 'crates/*.rs'   (conformance scenario posts to /v1/proposals via policy.local, expects 202)
resolve_proposal_approval_mode(): returns (value == "auto", ...) at gateway, then sandbox scope; default (false, "default")
archived sandbox-log: "Flushed denial analysis to gateway proposals=1"
$ id   -> groups include 986(docker), 101(lxd)
$ ls -l /var/run/docker.sock -> srw-rw---- root docker
```

Author verification after the edits:

```text
$ python3 <scratchpad>/checklinks.py <11 changed/new files>
107 local links checked, 0 bad            (exit 0)
negative control (bad anchor + missing file) -> 2 bad, exit 1
Stage Plan S/T/E identifier set, HEAD vs working tree: unchanged (168 ids)
Checked S/T boxes: 0; checked boxes overall 1 -> 2 (the next-action line for the Stage 00 record)
git diff HEAD -- ADR 0000 | removed lines: 0
$ sha256sum -c --quiet SHA256SUMS (receipt dir) -> evidence hashes OK
$ git diff --check -> exit 0
$ python3 -m unittest discover -s .claude/hooks -p 'test_*.py' -> Ran 25 tests, OK (exit 0); guard profile still development
```

Independent verifier (`verifier` subagent, read-only), first pass: **VERDICT: PASS**, no Important findings. Gate: `git diff --check` exit 0; guard tests 25 OK exit 0; `SHA256SUMS` 18/18 OK; 168 S/T/E IDs unchanged, 0 S/T boxes checked; ADR removed lines 0; its own link checker 107 links / 0 bad with a biting negative control; OpenShell source and host claims re-verified at tag `v0.1.2` (commit 6648bd0c). `scripts/smoke.sh`, `scripts/quality/*` and `Cargo.toml` absent → runtime gates Not yet proven.

Verifier nits and dispositions (all fixed in this change):

1. Local Only "preferably over a socket" left a loopback path that would silently make the Foundation-0 N/A false → socket route required; a loopback/host-service allowance is declared a permitted destination with its rows returning (ADR decision 3/4, Architecture §13.1/§27.2, T03.16, Test Strategy §8.2).
2. Scope choice (b) had no deferral row → rows added to this record and the Stage Plan deferral table; Master Build Sequence Stage 1 now blocks a credentialed or network-permitted worker until that qualification passes.
3. "Private cannot reach a cloud model" was stricter than PRD §10.1 without an ADR → reworded: Local Only cannot; Private follows PRD §10.1 admission.
4. Pi-outside-envelope "no host authority" had no test; E2 scope grew with the same time box → E2 records Pi's account, readable paths and network reach, T04.05 re-verifies (Architecture §13.1); E2 time box notes one recorded extra day; Architecture prototype line for E2 updated.
5. Protected-path risk note omitted `docs/stage-gates/` → risk note added above.
6. (Counted nits) Stage 00 record restored the omitted Template B engineering-gate lines; parking-lot text no longer claims S05.10/S05.11 say something they do not; "left pending" (unevidenced) reworded to "later status was not captured" in the threat model and Test Strategy; Architecture E2 prototype line (item 4 above).
