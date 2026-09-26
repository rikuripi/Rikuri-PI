# Review instructions

How any review of a change to this repo is run: `/code-review`, the
`verifier` subagent's report, a human reviewer, a PR bot. Referenced from
AGENTS.md > Deterministic gates. AGENTS.md > "Rikuri boundaries to preserve"
is the judgement this review applies.

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
- **Compliance**: does the diff match the Build Record's Plan and the S/T
  items it names? Plus the rules only this repo has:
  - **Foundation-0 scope**: stages 00–05 only. Work from a later product
    stage entering Foundation-0 without an ADR reconciliation is
    **Important**. Foundation-0 Stage 04 is not product Stage 4.
  - **Claims match evidence**: a document, README, or gate record that says
    something exists, works, or passed without linked evidence is
    **Important**. Pass, Fail and Not yet proven stay distinct.
  - **One source**: no second roadmap, checklist or procedure beside
    AGENTS.md and the documents it lists as authorities.
  - Lockfiles committed; production container images pinned by digest.
  - Implemented execution paths record the structured evidence AGENTS.md
    names (stable IDs, versioned identity, verification apart from
    completion).

## What "Important" means here

Reserve **Important** for findings that let untrusted content gain
authority; mutate canonical state outside governed services; leak private
data or break Local Only; let spending pass a hard budget; lose or corrupt
state during recovery; disable or weaken a gate; claim a gate or stage
passed without evidence; or diverge from the approved plan. Everything else
is **Nit**. A change to a protected path (listed in
`.claude/hooks/guard-config.json`) always gets a written risk note in the
Build Record, finding or not.

## Cap the nits

At most five nits per review; summarise the rest as a count.

## Do not report

Build output (`target/`, `dist/`, `node_modules/`), generated files, the
historical 90-Day Build Plan and dated review records unless the change
contradicts them, and what CI enforces once it has been observed green (the
guard tests in `.github/workflows/claude-guard.yml`; later, the Stage 00
gates).

## Separation of duties

The session that wrote a change does not approve it. Review findings are
evidence in the Build Record; the owner decides.
