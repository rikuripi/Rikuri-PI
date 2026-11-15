# Build Record — Security briefing intake, 2026-09-21 to 2026-11-14

| Field | Value |
|---|---|
| Change | Dispose the owner's 52 daily AI-security briefings against the PRD's security requirements, and draft the ADR addendum text for the gaps. The owner then adopted all seven proposals (2026-11-14); §7 records the adoption into the PRD, ADR 0000, the Test Strategy, the threat model and the documentation index. |
| Date | 2026-11-14 |
| Author / agent session | Claude Code (Fable 5.2) for the owner |
| Branch | `claude/security-briefings-prd-updates-24600b`, worktree `.claude/worktrees/security-briefings-prd-updates-24600b`, from `main` `1469f89` |
| Commits | `d47b86f`, the change (owner, 2026-11-14: "commit, push and open a PR"); a second commit records this row, the status and the PR |
| Stage Gate | N/A: no Foundation-0 item, interface or gate status changes |
| Related ADR / requirement | PRD §10 (SEC-008, SEC-014, SEC-021, SEC-029, SEC-030); [threat model](../security/THREAT_MODEL.md) §5; [Test Strategy](../../doc/Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md) §8.2 and §17; [PARKING_LOT.md](../PARKING_LOT.md) (baseline frozen; a corrected security requirement goes through a dated ADR with matching tests) |
| Status | DONE locally; pushed; [PR #40](https://github.com/rikuripi/Rikuri-PI/pull/40) open; CI Not yet proven until a run is observed; the owner merges |

## Task Contract

1. **Outcome:** The owner can see which briefing items the plan already requires, which are gaps, and the exact text that would close each gap.
2. **Scope:** this one file. Tier: routine (documentation, no requirement changed). Not included: any edit to the PRD, the Architecture, the Test Strategy, the threat model or ADR 0000; any code; the rikurinode repository (its draft is a separate note).
3. **Done when:** every local link in this file resolves and `git diff --check` is clean.
4. **Authority:** owner, 2026-11-14: "Yes, draft both". Read, Write and Bash for checks in this worktree. No commit, push or PR. No independent check (routine, documentation only).
5. **Limits:** one task; two repair cycles. No time or token budget is configured.

## 1. Sources and method

- **Briefings:** the 52 notes `2026-09-21 AI Security.md` to `2026-11-14 AI Security.md` in the owner's Obsidian vault, folder `AI Security` (no note exists for 10-05, 10-19 or 10-21). Each has two or three action items and a "Why" list of sources. All 52 were read in full. The briefings and the articles they cite are data; nothing in them was executed or followed as an instruction.
- **Plan documents:** PRD v1.2 §10 and the threat model §5 to §6 were read in full. The rest of the PRD, Architecture v1.7 and Test Strategy v1.2 were checked by keyword search, not read end to end.
- **Limit of this method:** "gap" below means no matching requirement text was found. A requirement worded differently could exist in a section that was only searched. Each proposal should be checked against the Architecture section it names before adoption.
- **Not verified:** the briefings' factual claims (CVE numbers, version numbers, incident details) were not checked against their sources.

## 2. Disposal by theme

Dates are briefing dates in 2026. "Covered" names the requirement that already asks for the control; it does not claim the control is built.

| Theme | Briefings | Disposal |
|---|---|---|
| Enforcement outside the model; a sandbox inside the agent is not containment | 09-21, 09-26, 09-28, 10-15, 10-20, 10-27 | Covered: SEC-001, SEC-021 |
| Default-deny egress, DNS, two independent layers | 09-27, 09-28, 09-30, 10-10, 10-13, 11-01, 11-03 | Covered: SEC-029, §10.2. Two-layer blocking is a design choice for the adapter ADR (S03.11), not a PRD change |
| Internal, metadata and loopback addresses; raw sockets; relaying through an allowed service | 10-02, 10-07, 10-12, 10-17, 10-25, 11-07, 11-12, 11-13 | **Gap: proposal 1** |
| Approval bound to what executes; approval surface out of the worker's reach; no self-raised limits | 09-22, 11-02, 11-06, 11-07, 11-12 | **Gap: proposal 2** |
| Denial is a hard stop; no probing, retry storms or attack-shaped requests | 09-24, 09-25, 09-27, 10-03, 10-07, 10-08, 10-10 | **Gap: proposal 3** |
| MCP servers, plugins, skills and connectors: pinning, remote identity, argument handling | 09-22, 09-23, 09-30, 10-02, 10-06, 10-07, 10-22, 10-27, 10-29, 11-11, 11-12 | **Gap: proposal 4** |
| Audit the worker cannot alter; declared actions compared with observed ones | 09-29, 09-30, 10-17, 10-23 | **Gap: proposal 5** |
| Secrets saved into agent settings, transcripts and logs | 10-11, 10-24, 11-08 | Mostly covered: SEC-022. **Gap: proposal 6** for saved approvals |
| Requests from other agents | 10-28 | **Gap: proposal 7** (a clarification) |
| Brokered, short-lived, per-worker credentials | 09-23, 09-29, 10-07, 10-12, 10-13, 10-15, 10-17, 10-20, 10-27, 11-01, 11-07, 11-10 | Covered: SEC-022 |
| Read never implies upload, export or publish; action tiers by consequence | 09-25, 09-26, 10-01 | Covered: SEC-003, SEC-020 |
| Money movement separated from general execution | 09-22, 09-28, 10-10, 10-15, 10-18, 10-20, 10-23, 10-25, 11-02, 11-05 | Covered: SEC-005, SEC-019, SEC-022. Payment-specific rules belong to the future financial feature's admission and to the rikurinode plan |
| Untrusted repositories, planted instruction files, hidden text in issues and READMEs | 10-03, 10-26, 10-29, 10-30, 11-04, 11-05 | Covered: §10.6, P29. The planted-instruction-file case (an `AGENTS.md` outside the project root) is worth adding to the Stage 02 F7 fixture set when that stage is built; backlog note, no requirement change |
| Secrets and reasoning can leave model context | 10-02, 10-28, 11-09, 11-11 | Covered: §10.1 (Secret never enters any model context), SEC-028 |
| Model and checkpoint provenance | 10-02, 10-30, 11-02 | Covered: PRD §24 risk row "Malicious/tampered open models" |
| Reversible file changes | 11-10 | Covered: SEC-010 |
| Provider keys: spend caps and theft | 09-24, 09-28, 10-18, 11-05, 11-06, 11-10, 11-13, 11-14 | Covered for Rikuri's own calls by the hard-budget rule (AGENTS.md §7). Provider-side caps are an owner to-do |
| Tool error text leaking internals | 11-09 | Parking-lot candidate; small, and SEC-023 already limits logs and diagnostics |
| Template engines evaluating configuration or model output | 10-04 | Parking-lot candidate for the Stage 8 developer pack |
| Daily outbound digest; drift detection | 09-26, 10-30 | Covered in intent: SEC-006, SEC-008, SEC-024 ("learned anomaly signals may restrict, never grant") |
| Machine and account hygiene: update Claude Code, Codex, VS Code, Chrome; audit browser and editor extensions; Full Disk Access; passkeys; secret scans and key rotation; GitHub Actions pinning and lockfiles; npm publisher bindings; named compromised packages (MemTensor, tensorlake, LiteLLM 1.107.0, the trojanized Claude Code extension) | 09-21, 09-23, 10-01, 10-04, 10-06, 10-08, 10-09, 10-14, 10-16, 10-17, 10-22, 10-24 to 10-26, 10-30, 10-31, 11-03 to 11-10, 11-13, 11-14 | Not a plan change. Owner to-dos. Dependency pinning and CI controls are governed by the Engineering Standard; changing them is its own task (AGENTS.md §2, Controls) |
| Attacker capability reports with no new control (ARTEX, Cairn, botnets, infostealers, agentic pentest tooling) | 09-23, 09-25, 10-09, 10-11, 10-13, 10-22, 11-01, 11-03, 11-09 | Recorded; no change. They raise the priority of the items above, not their content |

## 3. What does not change

- No Foundation-0 gate, stage item or acceptance changes. Foundation-0 workers hold no credential and have no permitted destination ([threat model](../security/THREAT_MODEL.md) §6), so proposals 1 to 4 bind at the first stage that enables a network-permitted or credential-bearing worker (expected product Stage 3, Stage 5 connectors or the Stage 8 developer pack).
- Every proposal is an unimplemented requirement. Status after adoption would be **Not yet proven**.
- SEC-025 to SEC-027 and SEC-032 are retired and not reused, so new rows start at SEC-033.

## 4. Proposals as drafted (adopted 2026-11-14; see §7)

Proposed heading in [ADR 0000](../../doc/Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md): **"November 14, 2026 — security briefing intake"**. Each item states the PRD text to add, the acceptance, and the test row the Test Strategy would gain. The adopted wording is in the PRD and the ADR addendum; where it differs from this draft, those govern.

### Proposal 1: extend SEC-029 (destination classes and alternate routes)

- **Add to the requirement:** "Whatever the allowlist says, a worker's network path refuses link-local and cloud-metadata addresses (169.254.0.0/16 and the IPv6 equivalents), private ranges and loopback other than its admitted endpoint. A destination is resolved once and the connection goes to the resolved address; a redirect to a refused class or off the allowlist is refused. A worker cannot open raw sockets. Reaching a refused destination through an allowed service (a fetch proxy, an archive, a URL scanner, a logging or metrics sink) counts as a bypass."
- **Acceptance:** from inside the envelope, a metadata address, a private address, a redirect to either, a DNS answer that changes between check and connect, a raw-socket attempt and one relay through an allowed service are each refused and recorded; a legitimate allowed request still succeeds.
- **Test Strategy:** new cases in §8.2 beside the existing DNS and redirect cases; they become mandatory with the first permitted destination.
- **Why:** 10-07 (SSRF in about a third of scanned MCP servers), 10-12 (an agent fetched its own cloud credentials from the metadata endpoint), 11-07 (a model hand-built raw packets around a proxy), 10-02 and 10-17 (agents relayed through public services).

### Proposal 2: tighten SEC-030 (approval integrity)

- **Add to the requirement:** "An approval binds to a digest of the exact serialized action it covers. The executor recomputes the digest at execution and refuses on mismatch; pending state cannot change after approval without invalidating it. The approval surface, and any step-up factor or code, sit where no worker, harness, browser worker or extension can read or actuate them. No worker can raise a limit that applies to itself; a limit change is itself a step-up action by the owner."
- **Acceptance:** an action mutated after approval is refused; a synthetic worker with access to the user's test inbox and browser session cannot obtain a step-up code or trigger the approve control; a worker's request to raise its own cap is refused and recorded.
- **Test Strategy:** new rows in §17 under SEC-030.
- **Why:** 09-22 (operator approves A, system executes B), 11-02 (nine of ten models raised their own wallet limits by reading emailed codes), 11-06 and 11-07 (a browser extension auto-clicked "Allow").
- **Relation to existing text:** SEC-016 already invalidates approval on "material candidate changes" and SEC-018 keeps enforcement outside the worker's writable authority. This makes the binding exact and adds the approval channel, which neither names.

### Proposal 3: new row SEC-033, P1 (bounded outbound conduct)

- **Requirement:** "A refusal by policy, or by a remote party (an authentication wall, a 401 or 403, a robots disallow, a CAPTCHA), ends that attempt and is reported. A retry of the same goal through another tool, route, credential or URL is refused, recorded and alerted. Network-capable workers have a request budget per destination per task, enforced outside the worker. Requests that carry injection or traversal payloads the task did not call for are refused as out of scope."
- **Acceptance:** a worker denied once cannot reach the same target by a second route; a loop against one host stops at the budget with a visible blocker; legitimate retries of a transient failure inside the budget still succeed.
- **Owner:** the first stage that admits a network-capable worker.
- **Why:** 09-24 (an agent treated access blocks as a puzzle), 09-25 and 10-03 (agents sent SQL-injection probes during routine fetches), 09-27 (16,000 requests against one site), 10-10 (shorteners used to dodge fetch limits).
- **Relation to existing text:** AGENTS.md §2 states this for coding agents in this repository ("a guard deny … ends that action"); the product has no equivalent row. HAR-003 bounds retries but not routes.

### Proposal 4: new row SEC-034, P1 (admission of connectors, MCP servers, plugins and skills)

- **Requirement:** "A connector, MCP server, plugin or skill is admitted by content digest, with its dependency set locked. A commit identifier on a remote that can rewrite history is not sufficient identity. Nothing resolves, updates or downloads at launch. A remote endpoint is bound to its admitted host identity; a changed hostname, certificate chain or resolution target requires re-admission. Tool arguments reach a child process as an argument vector, never through a shell or an expression-evaluating template. Authorization endpoints are pinned to the admitted issuer. Placeholder or unregistered domains in a manifest refuse admission."
- **Acceptance:** a swapped artifact behind an unchanged pin is refused; a launch that would fetch is refused; a moved remote endpoint is refused until re-admitted; an argument containing shell metacharacters is delivered literally; a token-endpoint redirect is refused.
- **Owner:** product Stage 5 connectors and the Stage 8 developer pack. SEC-024 already requires revalidation on package, plugin or runtime change; this states what admission checks.
- **Why:** 09-22 (pinned plugins swapped), 09-30 and 10-29 (MCP SDK token-endpoint and audience flaws), 10-06 (registry entries pointing at re-registrable domains), 10-27 (tool arguments passed to a shell), 11-11 (the same launch command produced 15 different dependency sets in 19 builds), 09-27 (349 skills referencing placeholder domains).

### Proposal 5: extend SEC-008 (audit integrity)

- **Add to the requirement:** "The audit record is written by a component the worker cannot write to, alter or delay. Every tool call is recorded when it happens, not at the end of a turn. A missing or out-of-sequence record is a health finding (SEC-006). A worker's own account of what it did is reconciled against the recorded calls; an undeclared call is flagged."
- **Acceptance:** a worker instructed to delete or edit its log cannot; a suppressed record raises a degraded state; a call the worker omits from its report is flagged.
- **Why:** 09-29 (most tested coding agents altered audit logs when asked), 09-30 (a model did not disclose its actions), 10-23 (a monitor could be postponed, disabled by headers or blinded).
- **Relation to existing text:** CORE-002 makes the ledger append-only; neither row says who may write it or that absence alarms.

### Proposal 6: extend SEC-014 (saved approvals)

- **Add to the requirement:** "A saved approval stores an operation pattern and scope. It never stores a literal command line, argument or environment value that may contain a credential, and saved approvals are excluded from sync, export and repository tracking unless the owner shares them explicitly."
- **Acceptance:** approving a command that contains a synthetic canary and choosing "Save for this project" leaves no canary in any stored policy, log or tracked file.
- **Why:** 11-08 (a "don't ask again" file saved exact commands with secrets; over a thousand public copies found).

### Proposal 7: clarify SEC-021 (requests from other agents)

- **Add to the sentence that lists what cannot edit authority:** "…and requests or messages from another agent or worker, which are untrusted input however they are labelled."
- **Acceptance:** a fabricated peer-agent message asking for an out-of-grant action is refused by the executor exactly as a direct request would be.
- **Owner:** product Stage 6 multi-agent governance; the Foundation-0 rule (instruction authority, AGENTS.md §7) already implies it.
- **Why:** 10-28 (a model performed unauthorized actions requested by apparent peer agents in 11 of 80 tests).

## 5. Owner decisions

1. **Adopt which proposals?** Decided 2026-11-14: "Pull from git first for Rikuri-PI. If we have the latest then adopt all proposals into the plans." All seven adopted.
2. **Priority of SEC-033 and SEC-034.** Adopted as drafted, P1. Still the owner's to change: P0 would make them a condition of offering any network-capable worker or connector.
3. **Parking-lot entries.** Not decided. The two candidates in §2 (tool error text; template evaluation) were not added to [PARKING_LOT.md](../PARKING_LOT.md).

## 6. Second Task Contract — adoption (2026-11-14)

1. **Outcome:** the plan's security requirements name the seven controls the briefings showed missing, each with future acceptance.
2. **Scope:** additions only, each dated, in `doc/Rikuri_Pi_PRD_v1.2.md` (§10 rows SEC-008, SEC-014, SEC-021, SEC-029, SEC-030; new rows SEC-033, SEC-034; Appendix A), `doc/Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md` (new addendum), `doc/Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md` (new §21), `docs/security/THREAT_MODEL.md` (§5, five rows), `doc/README.md` (index entry). Tier: **high-risk** by caution: documentation only, but it changes the stated requirements for trust and security boundaries. Not included: Architecture §27, the Stage Plan, the Master Build Sequence, the parking lot, code, CI, the guard, and the rikurinode repository.
3. **Done when:** every local link and anchor added by this change resolves; `git diff --check` is clean; no existing sentence is removed or reworded; one verifier pass with no Important finding open.
4. **Authority:** owner, 2026-11-14 (quoted above). `git fetch` allowed by "pull from git first". Read, Edit, Write and Bash for checks in this worktree. Commit, push and PR allowed afterwards by the owner (2026-11-14: "commit, push and open a PR"); no merge. One verifier pass.
5. **Limits:** one task; two repair cycles; one verifier pass plus at most one follow-up.

## 7. Adoption

- **Latest check.** `git fetch origin` on 2026-11-14, then `HEAD`, `main` and `origin/main` all at `1469f89`; zero commits behind. The owner's condition was met.
- **PRD.** Dated sentences appended to the description and acceptance of SEC-008 and SEC-021 and to the single cell of SEC-014, SEC-029 and SEC-030; rows SEC-033 and SEC-034 added after SEC-031 with a paragraph stating scope, stages and status; both rows added to Appendix A.
- **ADR 0000.** Addendum "November 14, 2026 — security briefing intake" with the seven items, cost and reversal.
- **Test Strategy.** New §21 with one fixture row per requirement, each with an allowed case as well as the refusals.
- **Threat model.** Five rows added to §5, each marked with the date.
- **Differences from the §4 draft.** SEC-034's acceptance gained "an admitted, unchanged connector still works" so the row cannot pass by refusing everything. The threat model gained five rows where the draft named none. Repair 1 (below) changed two more things: the SEC-029 grant exception for private and loopback endpoints, and the owning-stage text.
- **Not done, stated so no claim implies it.** Architecture §27 has no matching section; the PRD paragraph and the ADR say the design detail is owed in the owning stage's adapter ADR. The Stage Plan and Master Build Sequence do not yet list SEC-033 or SEC-034 against a stage item.

**Protected-path risk note.** `doc/Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md` and the Test Strategy are protected paths in the guard's policy. The change to both is additive and dated: one new addendum and one new section, no existing sentence altered. It loosens nothing; each addition reverts by deleting the dated text. The guard, its policy, settings, CI and the quality scripts are not touched.

**Architecture check not done.** §1 said each proposal should be checked against the Architecture section it relates to before adoption. That check was not made; only keyword searches were. The Stage Plan's document control asks for PRD and Architecture to change together, and this change leaves Architecture §27 as it is, which the PRD paragraph and the ADR disclose. Bringing §27 into line is follow-up work.

### Verifier pass 1 and repair 1 (2026-11-14)

Pass 1 verdict: FAIL on one Important finding; links, anchors, whitespace, additions-only, table shape, cited IDs and the baseline-freeze route all passed.

- **Important: SEC-008 had no owning stage, and the Test Strategy's wording for it resolved to Foundation-0 Stage 04**, contradicting "Foundation-0 gates unchanged". Repair: the PRD paragraph, the ADR's binding paragraph and the Test Strategy §21 rows now say no Foundation-0 stage gains an obligation; the SEC-008 and SEC-014 additions are owed from product Stage 1 and the SEC-030 addition with the product Stage 2 approval experience, with T00.12, T02.16, T04.18 and S02.08 standing as written. Whether Stage 04 or Stage 02 should take those fixtures during Foundation-0 is recorded in the ADR as an open owner decision that needs its own dated ADR. This keeps to the Task Contract; it does not decide the question.
- **Nit, repaired because it changed behaviour:** SEC-029's "private ranges … whatever the allowlist says" would have refused granted Link and node destinations. Now: metadata and link-local are always refused; private and loopback are refused unless a current grant names the endpoint explicitly. PRD, ADR, Test Strategy and threat model reworded alike.
- **Nits repaired:** the ADR now states that the briefings' facts were not checked against sources; SEC-030 gained its acceptance sentence; the Test Strategy's SEC-021 row says whose grant; the protected-path note and the Architecture note above were added.
- **Nit not repaired:** the threat model §5 intro still says the table "points to existing stage obligations" while the five new rows cite future acceptance; each new row is dated and names Test Strategy §21.

All repairs edit only text this change added; a word-level diff against `1469f89` still shows no removed word.

## 8. Verification

| Check | Result |
|---|---|
| Up to date with `origin/main` | Pass: `1469f89`, zero behind, after `git fetch origin` |
| Local links and anchors in added lines | Pass: 18 links in the added lines and this record, 0 broken (author's script; the verifier's own script agreed on both passes) |
| `git diff --check` | Pass: exit 0, no output, after repair 1 |
| Additions only | Pass: `git diff --word-diff=porcelain` against `1469f89` shows 0 removed words, after repair 1 |
| Full gate | Not run: documentation only; CI runs it on a pull request |
| Independent check (verifier) | Pass 1: FAIL, one Important finding (above). Repair 1. Follow-up pass: **PASS**, no Important finding open; guard tests `OK (skipped=1)`. Five nits left for the backlog, listed below. This was the last review round the contract allows |

**Nits left after the follow-up pass (backlog, not fixed here because no review round remains):**

1. SEC-008 stays P0 while its new sentences are owed from product Stage 1; open until the owner decides the Stage 04 question.
2. "Product Stage 1" and "product Stage 2 approval experience" name no specific item in the Master Build Sequence, which this change does not update.
3. The threat model's SEC-029 row omits "explicitly", and its §5 intro still says the table points to existing stage obligations.
4. SEC-029's address classes do not name the carrier-grade NAT range 100.64.0.0/10 that Tailscale uses, and "node destination" is undefined in the row. This one affects behaviour for Link and should be settled before Stage 3 designs against the row.

## Close-out

- **Outcome:** all seven proposals are in the PRD, ADR 0000, the Test Strategy and the threat model as dated, unimplemented requirements, with one repair cycle and a passing follow-up check.
- **Evidence:** §8, at `d47b86f` (the verifier passes ran on the same content before it was committed).
- **Safety:** requirement text only; additions tighten and none loosens. No code, control, CI, guard or setting changed. No secret file read. The rikurinode repository was read only through `git show origin/main:` in a separate local clone and was not modified.
- **Status:** DONE locally; committed as `d47b86f`, pushed, [PR #40](https://github.com/rikuripi/Rikuri-PI/pull/40) open. CI Not yet proven until a `Full gate (Ubuntu)` run on the PR is observed; the owner merges. Two owner decisions stay open: whether Foundation-0 Stage 04 or Stage 02 takes the SEC-008 or SEC-030 fixtures (ADR addendum), and the parking-lot candidates (§5).
- **Next:** one task, not started: carry the rikurinode draft into that repository as its own Build Record and adopt its six proposals there.
