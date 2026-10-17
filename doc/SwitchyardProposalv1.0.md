# Proposal: evidence-based model routing and evaluation for Switchyard

Prepared October 16, 2026. Status: proposal for review; no new implementation, model calls, upstream submission, or deployment is included.

## Recommendation

Improve the quality and interpretation of execution evidence before expanding model-switching logic. Then evaluate routing against fixed-model baselines using independently assessed task outcomes, complete costs, and repeated trials.

A model's observed performance depends on the task, available information, tools, environment, prompting, output limits, and randomness. A failed tool call does not establish model incapacity. A passing test does not establish complete correctness. Switching is justified when its expected benefit exceeds its extra cost and disruption under the actual workflow—not merely because an error-looking string appeared.

Proceed first with a narrow reproducible signal-classification improvement. Treat structured outcomes, scoped recovery, and broader evaluation as separate work packages. Keep Switchyard's modular routing core; avoid turning the library into an entire verification platform.

## 1. Evidence we have—and what it does not establish

Tests used Switchyard v0.3.0 at commit 336196f6fbfc97ddc71c1700f6092e564e9f23c2 on Apple Silicon. Live model experiments used GPT-4.1 and GPT-4.1 mini; the repeated-round responses identified snapshots dated 2025-04-14. These results do not assess the latest model pool or establish the current development branch's behavior.

| Observation | Strength of evidence | Interpretation |
|---|---|---|
| Successful source reads containing quoted memory-error text triggered critical escalation | Reproduced with exact inputs; 54 conditions repeated twice produced identical decisions | A concrete signal-classification improvement candidate |
| A local adapter reduced escalations from 27 to 0 across 36 selected noisy successes and preserved eight executed failure cases | Targeted local regression evidence | Structured executor outcomes are promising; no production error rate or inference savings measured |
| Three unrelated passing results aged an unresolved failure out of default history | Executed histories plus source inspection | Recency and verified resolution are different concepts |
| Scoped recovery handled partial passes, stale artifacts, changed tests, and duplicate reports | Host prototype and local tests | Useful integration behavior; no proof that every unresolved issue needs a stronger model |
| Repeated repairs: mini 3/3, stronger 3/3, routed 2/3 | Three trials per arm on one authored case | Insufficient to rank approaches or estimate general reliability |
| Reviews and generated tests missed defects; generated tests also rejected valid reference behavior | Concrete counterexamples from a few calls | A reviewer or generated test is another fallible measurement component |

The recent-history default in this pinned version is **three results**. Earlier reports incorrectly described five; the interpretation was corrected, and original execution records were preserved.

**All existing cases are now development data.** We have examined them and used their failures to shape prototypes. They remain valuable regression tests, but further success on them would not be independent evidence for adoption.

## 2. Separate four things that are currently easy to confuse

| Layer | Question | Example of a measurement mistake |
|---|---|---|
| Execution detection | What actually happened? | Reading a quoted error is classified as executing a failure |
| Outcome evaluation | Did the result satisfy the contract? | A partial suite passes while a required edge case remains broken |
| Model capability | How reliably can this model solve this task under these conditions? | A harness failure or resource limit is blamed on the model |
| Routing policy | Did changing models improve the complete outcome? | One lucky stronger-model response is credited as a routing benefit |

Evaluate each layer independently before judging the combined system. Preserve “unknown” and disputed outcomes instead of forcing every result into success or failure.

## 3. Proposed improvements and ownership

### A. Execution evidence contract — first priority

**Owner:** executor/agent integration produces it; shared protocol carries it; router consumes it.

Create explicit routing evidence separate from raw tool content. The agent should retain the raw source, stdout, and stderr needed for its work. Successful content should not automatically become runtime-failure evidence.

A versioned evidence record should identify:

- Task, session, tool call, and unique execution/run IDs.
- Tool kind and executor-observed status: completed, failed, timed out, cancelled, or unknown.
- Process exit status, when meaningful.
- Test runner outcome and counts: executed, failed, errored, skipped, expected failures, and unexpected successes.
- Individual test IDs and outcomes, planned versus executed inventory, and relevant validation scope.
- Implementation artifact, test-contract, dependency/configuration, and environment identities sufficient for that workflow.
- Failure category, originating subsystem, timestamp/order, and provenance of the classification.

These fields must come from the executor, not be inferred solely from model-authored prose or arbitrary stdout. A false error flag alone is not affirmative proof of success. A hash identifies content relative to a trusted record; it does not authenticate the producer.

Keep explicit handling for missing, contradictory, zero-test, stale, and duplicate reports. Inconclusive evidence should trigger bounded revalidation or diagnosis, with a documented fallback for older integrations. Avoid silently treating unknown as passed or allowing the system to wait indefinitely.

**Acceptance:** new regression tests eliminate the reproduced quoted-content escalation without suppressing actual failures, including zero-exit failed tests, process failures after passing tests, timeouts, and resource exceptions. Existing relevant upstream tests must continue to pass. Test behavior when structured metadata is absent as well as when present.

### B. Failure attribution — distinguish a coding problem from a failed environment

**Owner:** executor and application provide context; routing policy may use the classification.

Separate implementation defects, malformed model output, parser/harness defects, faulty tests, dependency/environment failures, resource limits, transient provider failures, and unknown causes. Keep the raw evidence alongside the label so the attribution can be audited and revised.

A memory exception may require code changes, resource changes, or a rerun. An invalid patch may be a model compliance problem or an application parser problem. Neither event automatically proves the efficient model lacks the needed reasoning ability.

Classify first, then choose among retrying, repairing the environment, improving feedback, revalidating, consulting another model, or escalating. Bound retries and consultations. The best action for real infrastructure failures remains an evaluation question; our deliberately raised MemoryError cases did not resolve it.

### C. Scoped recovery state — separate verification from model choice

**Owner:** preferably an application-level recovery service, with a small optional interface into the router.

Track known failed checks as explicit obligations linked to artifact and test-contract identity. Relevant current results can resolve them; unrelated passes, duplicate reports, stale artifacts, or changed assertions should not silently do so.

Keep verification state separate from model tier. “Issue unresolved” does not necessarily mean “use the strongest model,” and a resolved issue does not imply that every subsequent request is easy. Our prototype's capable-tier floor was an experimental policy, not a default recommendation.

Evaluate confirmation counts and failure-history windows as tunable policies. Two passes did not eliminate an intermittent fault. Reopening every tracked obligation after any code change is conservative but may overvalidate harmless edits. A production policy needs explicit invalidation rules, bounded revalidation, restart persistence, and concurrent/out-of-order result handling.

**Acceptance:** relevant recovery can be recognized without accepting partial or unrelated passes as closure. Stale/duplicate reports cannot accumulate confirmation credit. Restart, concurrency, cancellation, and changed-artifact cases have documented, tested behavior. Model-tier consequences must then be assessed through live task outcomes, not ledger compliance alone.

### D. Independent evaluation and traceability

**Owner:** evaluation harness and reviewers; not solely the model router.

Map requirements to positive, boundary, state-transition, scale, and compatibility checks. Keep a versioned inventory showing what is tested and what remains unverified. Do not use test count alone as a quality measure.

Validate generated tests against the stated contract, correct reference behavior, and known incorrect variants. A reference implementation can share the author's assumptions, so it is supporting evidence rather than an unquestionable oracle. Use independent review or maintainer-confirmed behavior for important ambiguities. Record and adjudicate disagreements rather than adjusting expectations to fit a favored implementation.

Save replayable decision traces: source evidence, classification, unresolved obligations, routing algorithm/configuration, selected model, and whether an application override changed that selection.

## 4. Scorecard: metrics with explicit denominators

| Area | Metric | Definition and caution |
|---|---|---|
| Detection | Failure precision and recall | Compare detected failures with independently labeled execution outcomes; report coding, environment, test, and harness categories separately |
| Detection | False escalation on successful executions | Stronger-tier selections caused by misclassified successful tool results; targeted synthetic rates are not production rates |
| Evidence | Unknown/invalid/stale outcome rates | Count against all execution reports; retain reasons and time to resolve them |
| Evaluation | False acceptance and false rejection | Acceptance of independently confirmed defective results, or rejection of contract-correct results; retain disputed labels |
| Task quality | Independently validated completion rate | Tasks meeting the frozen acceptance contract without confirmed regressions, divided by all assigned tasks; crashes and exhausted budgets stay in the denominator |
| Compatibility | Regression rate | Confirmed API, behavior, or state-invariant regressions introduced by accepted repairs |
| Recovery | Premature closure and reappearance | Linked defects still detected after closure, and recurrence during a defined observation horizon |
| Recovery | De-escalation lag | Additional routing steps after relevant verification before returning to the efficient tier; this is a policy/cost measure, not proof those steps were unnecessary |
| Routing | Benefit of escalation | Outcome and cost difference when continuing an identical saved failing state with each model across repeated trials |
| Cost | Total cost per validated completion | All study costs—including unsuccessful tasks, retries, judging, test generation, routing overhead, and verification—divided by validated completions; undefined if none succeed |
| Latency | End-to-end completion time | Include inference, tools, waiting, retries, and validation; report distributions only with adequate observations |
| Operations | Human review and intervention | Minutes, correction counts, and unresolved decisions; report separately unless an agreed costing model exists |

Report token usage, cached input, output/reasoning usage where applicable, truncation/finish reasons, requested and returned model snapshots, limits, system fingerprints, and configuration. If a router exposes a score called probability or confidence, do not treat it as calibrated task-success probability without a separate calibration study.

A passing suite means “verified under these checks,” not proof of universal correctness. Primary quality measures depend on evaluation validity, so evaluator disagreement and uncovered requirements belong in the report.

## 5. Controlled evaluation plan

### Stage 1: freeze and reproduce locally

Reproduce the quoted-content issue on the pinned version, then check the current upstream development version before proposing a fix there. Keep the reproducer and a narrowly scoped upstream regression-test/patch candidate separate from the broader integration design.

Turn the existing local cases into a development regression pack. Add restart, concurrent result arrival, missing outcome, duplicated result, invalid metadata, altered tests, and artifact/configuration-change cases. No provider inference is needed for these checks.

### Stage 2: shadow the evidence adapter

For the same real execution stream, record the legacy router's decision and the proposed evidence-based decision without allowing the experimental policy to control execution. Independently label disagreements.

This stage estimates which errors are being corrected or introduced before model-output variation confounds the result. It still does not measure the outcome of the alternative decision.

### Stage 3: small live feasibility pilot

Use fresh tasks not used to design the prototypes, including real repository issues with a clear contract and frozen starting revision. Retain authored tasks as diagnostics, not as the sole external validation set. Pin environment, tool access, prompts, model snapshots, and policy settings. Use fresh copies and interleaved trial order.

Compare:

1. Fixed efficient model.
2. Fixed capable model.
3. Stock Switchyard with legacy evidence.
4. Switchyard with structured execution evidence only.
5. The same structured-evidence condition plus a separately specified recovery policy.

This isolates the adapter from recovery policy changes. Keep protocol formatting settings consistent across arms; earlier free-form versus enforced-JSON experiments should not be blindly pooled. Establish common task/tool and wall-time limits, record cost explicitly, and flag truncation or other censoring. Equal output-token caps alone do not guarantee a fair comparison across different model architectures.

Use repeated paired trials. At selected escalation checkpoints, save the identical state and independently continue it with mini and the capable model. These counterfactual continuations help distinguish the effect of switching from task difficulty and ordinary response variation. Count their costs as evaluation overhead, not as normal routed-task operating cost.

### Stage 4: untouched holdout and adoption decision

After the feasibility pilot, freeze the policy and evaluation contract. Assess on a new untouched holdout; do not tune on its outcomes. Account for dependence among repeated runs of the same task rather than treating every test assertion or retry as an independent sample.

Before running the holdout, define the acceptable quality difference, cost objective, latency limits, and treatment of unknown/disputed results. Compare paired task outcomes with uncertainty intervals. Choose sample size from observed variance and the decision margin; no fixed small task count is guaranteed adequate.

Adopt only when the evidence supports the predeclared quality requirement and a useful cost, latency, or intervention benefit. If uncertainty is too wide, report inconclusive. A modest or absent routing benefit is a legitimate result; retaining a fixed efficient model may be the better operational choice.

## 6. Delivery sequence and decisions

| Package | Deliverable | Decision before expanding |
|---|---|---|
| 1. Narrow classification repair | Current-version reproduction, regression tests, reviewable focused patch | Does it remove the demonstrated false positive while preserving genuine failures? |
| 2. Evidence interface | Versioned contract, host adapter, invalid/unknown handling, compatibility behavior | Are executor outcomes reliable enough for routing? |
| 3. Recovery experiment | Optional scoped policy, persistence/concurrency tests, traces | Does explicit recovery evidence help without excessive verification or stronger-model use? |
| 4. Live evaluation | Frozen cases, arm configuration, per-task results, complete scorecard | Is any improvement attributable to the proposed change rather than randomness or evaluation bias? |
| 5. Adoption assessment | Untouched holdout results and uncertainty | Is there enough demonstrated benefit to justify the added machinery? |

Do not make broad architectural changes a prerequisite for the narrow reproduced classification fix. Do not assume a mandatory stronger-model review or permanent capable-tier floor is beneficial merely because it is conservative.

## 7. Budget and present status

Recorded API estimates total **$0.181974**. Under the existing $1 cumulative cap, approximately **$0.818026** remains for future paid experiments; this is experiment headroom, not an account-balance statement. Local tests can continue without provider costs.

Future live pilots remain subject to that existing cap. Estimate per-task cost from a feasibility run, size the pilot accordingly, and stop before the guard would exceed the cap. A larger confirmatory study may not fit and must not be silently represented as completed or statistically sufficient.

This proposal incurred no new API cost. The credential created earlier had a 24-hour lifetime; any future live run must check its validity safely and use the established secure setup workflow if replacement is needed.

No upstream issue or patch has been submitted. The proposed direction is evidence-supported, but our prototypes have not yet demonstrated improved real-world repair quality or total operating cost.

## Evidence index

Paths are relative to this proposal directory:

- [Focused repeats and trigger matrix](/Users/hchome/Documents/Codex/2026-10-15/n/outputs/focused-validation/README.md)
- [Unsent quoted-content reproduction draft](/Users/hchome/Documents/Codex/2026-10-15/n/outputs/focused-validation/upstream-repro.md)
- [Structured feedback adapter](/Users/hchome/Documents/Codex/2026-10-15/n/outputs/feedback-adapter/README.md)
- [Scoped recovery experiment](/Users/hchome/Documents/Codex/2026-10-15/n/outputs/recovery-policy/README.md)
- [Review, generated-test, and property-check failures](/Users/hchome/Documents/Codex/2026-10-15/n/outputs/variety-tests/README.md)
- [Earlier multi-bug repository repairs](/Users/hchome/Documents/Codex/2026-10-15/n/outputs/harder-repair/README.md)
