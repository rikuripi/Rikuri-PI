# Review instructions

How any review of a change to this repo is run: `/code-review`, the
`verifier` subagent's report, a human reviewer, a PR bot. This is the
review pass [AGENTS.md](AGENTS.md) §5 requires: one pass after tests,
then at most one follow-up on corrected findings; a review that would
need a third round ends in `NEEDS_OWNER_DECISION` instead. AGENTS.md §7,
"What the code must preserve", is the judgement this
review applies. The change's risk tier (routine or high-risk), named in
its Task Contract, sets how deep the review goes
([Engineering Standard](doc/Rikuri_Pi_Engineering_Standard_v1.2.md#risk-tiers),
"Risk tiers").

## Passes (run all three; tag every finding with its pass)

- **Bugs**: logic errors, edge cases, regressions, non-deterministic tests
  (wall clock, ordering, shared temp paths, live model output used as an
  oracle). A fallback that reports success when the primary path failed.
- **Security & boundaries**: for every change ask **"how would untrusted
  content — an imported file, a retrieved chunk, model or tool output — use
  this to gain authority?"** Review the boundary, not just the diffed lines:
  canonical state mutated outside a governed service; imported, retrieved or
  model content granting a permission or changing an approval; a
  read/inspect path that executes; a Local Only path that reaches cloud HTTP
  construction; a hard budget that unknown cost can pass; a recovery that
  promotes without validate → durable write → read-back.
- **Compliance**: does the diff match the Build Record's Plan, its Task
  Contract's Scope (AGENTS.md §3) and the S/T items it names? Work outside
  the Task Contract is a finding. Plus the rules only this repo has:
  - **Foundation-0 scope**: stages 00–05 only. Work from a later product
    stage entering Foundation-0 without an ADR reconciliation is
    **Important**. Foundation-0 Stage 04 is not product Stage 4.
  - **Claims match evidence**: a document, README, or gate record that
    claims a gate or stage result, a security property, data integrity, or
    completion without linked evidence is **Important**; other imprecise
    wording is a nit. Pass, Fail and Not yet proven stay distinct.
  - **One source**: no second roadmap, checklist or procedure beside
    AGENTS.md and the documents it lists as authorities.
  - Lockfiles committed; production container images pinned by digest.
  - Implemented execution paths record the structured evidence the
    [Engineering Standard](doc/Rikuri_Pi_Engineering_Standard_v1.2.md)
    names (Future-learning evidence rule; Close Out: stable task/run IDs,
    versioned execution identity, verification apart from completion).

## What "Important" means here

Reserve **Important** for findings that let untrusted content gain
authority; mutate canonical state outside governed services; leak private
data or break Local Only; let spending pass a hard budget; lose or corrupt
state during recovery; disable or weaken a gate; claim a gate or stage
passed without evidence; or diverge from the approved plan. Two test gaps
are Important too: a test the change's own "Done when" depends on is
missing, or, in high-risk Rust code, a mutant of the changed lines survives
in a refusal, validation, recovery or integrity check. Everything else is
**Nit**. A change to a protected path (listed in
`.claude/hooks/guard-config.json`) always gets a written risk note in the
Build Record, finding or not.

A review fails only on an Important finding. Nits never fail it and never
start another round: they are fixed in place or listed in the Build Record.

## Cap the nits

At most five nits per review; summarise the rest as a count. Surviving
mutants rated nits are listed in full in the evidence and count as one
nit; each surviving mutant rated Important is its own finding.

## The follow-up pass

It checks the corrections to the first pass's findings and reruns that
pass's mutation scope; it does not search for new findings. A new nit it
happens to see is listed for the backlog. A new Important finding it happens
to see is reported, and the task ends in `NEEDS_OWNER_DECISION`: the owner
decides whether it is fixed now, as its own task, or recorded as debt.

## Do not report

Build output (`target/`, `dist/`, `node_modules/`), generated files, the
historical 90-Day Build Plan and dated review records unless the change
contradicts them, and what CI enforces once it has been observed green (the
guard tests in `.github/workflows/claude-guard.yml` and the full gate in
`ci.yml`).

## Separation of duties

The session that wrote a change does not approve it. Review findings are
evidence in the Build Record; the owner decides. A review does not start
work: findings outside the task go to the backlog or the owner (AGENTS.md
§3), and the independent reviewer never fixes.
