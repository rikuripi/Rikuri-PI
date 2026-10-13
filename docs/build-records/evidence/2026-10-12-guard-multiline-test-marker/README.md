# Evidence: the fix-lock and its tripwire see a test marker written over several lines

Record: [../../2026-10-12-guard-multiline-test-marker.md](../../2026-10-12-guard-multiline-test-marker.md).
Every run was made inside rikuripi/rikurinode's `scripts/quality/safe-run.sh` (identical to `97a2a8b`'s),
because the machine runs a live node; each output ends with lnd's container start time before and after.
Base: Rikuri-PI `f9f58c0` with the linear-time change applied (`2026-10-12-guard-inline-rule-linear.md`).

## Tools

| File | What it does |
|---|---|
| `rule_ml.py` | The rule's `starts_at` and the tripwire's `RUST_INLINE_TEST`, old and new, and the added branch `ML`. Both `starts_at` forms are built from the config (`RULE_CFG`, or `.claude/hooks/guard-config.json` found from `RULE_ROOT`, this folder or the current one) by exact substring replacement, whichever form it holds |
| `apply_config.py` | How `guard-config.json` was changed (once, on the base): exact replacement, then checks that the parsed rule equals `rule_ml.SA_NEW`, every other key is unchanged and no character outside ASCII moved. On a config that already holds the new rule it stops at its first check, by design |
| `survey.py` | Per folder: files whose first `starts_at` match moves (old vs new), files the old and new tripwire marker finds; names any file whose new tripwire marker the new rule does not cover |
| `survey_gl.py` | The gap left for the follow-up: multi-line `cfg`/`cfg_attr` attributes holding `test` that no single line of matches `guarded_lines` and that are no marker |
| `corpus.py` | Old vs new at every line start holding `#`: old matches kept, new-only `starts_at` matches all multi-line, new tripwire within new rule |
| `not_nested.py` | New-only tripwire matches whose committed `test` sits inside `not(…)` |
| `misses.py` | Markers the new `starts_at` still misses: each `cfg(any(`/`cfg(all(` attribute read to its end with guard.py's `rust_attr_end`, comments and strings blanked, judged by its items. Args: a hooks folder, then folders |
| `bench.py` | One call per crafted text at sizes k to 8k (×2 per doubling = linear), then three wrong variants of `ML` timed on the suite's linear-time texts (limit 0.25 s). Arg: k |
| `hook_e2e.sh` | The real hook in scratch git repositories, a global fix-lock held: old guard and config, new guard with old config, both new. Args: work folder, old hooks folder, new hooks folder |
| `run.sh`, `suite.sh` | Wrappers: run a command inside safe-run (`SAFE_RUN`), recording time, exit code and lnd's start time; run the guard suite of a checkout |

`red-to-green-test.diff` is `git diff 1f1eded 8d46d89 -- .claude/hooks/test_guard.py` from the gate clone with trailing white space and the final blank line removed (blank context lines lose their leading space). To rerun, from a checkout of this change: `export SAFE_RUN=<rikuripi/rikurinode>/scripts/quality/safe-run.sh RULE_ROOT=$PWD`,
then e.g. `run.sh out.txt python3 -I corpus.py . <more folders>`. Outputs have trailing white space removed and nothing else.

## Outputs

Round 6 (`f9f58c0` with the linear-time change uncommitted) first; the `r7-` files repeat the runs on 2026-10-13 on the committed base (`35fe118`, sync round 7), each from a clean export of its commit.

| File | Result |
|---|---|
| `red-hashes.txt`, `green-hashes.txt` | The four hook files' sha256 in the red and green runs |
| `red-py314.out`, `red-py39.out` | The new tests on the base: 144 run, exactly the 3 new checks fail, on Python 3.14.4 and 3.9.25 |
| `green-py314.out`, `green-py39.out` | This change: 144 OK on both |
| `red-to-green-test.diff` | Every change to test_guard.py between the red and green runs: the fixture `InlineTests.RUST` (its pattern and comment) only |
| `rikurinode-green-py314.out`, `rikurinode-green-py39.out`, `rikurinode-red-py314.out` | rikuripi/rikurinode `97a2a8b` exported with the shared files: 144 OK on both; red: the 2 expected failures |
| `rule.out` | The patterns, old and new |
| `survey.out` | 0 files change here; std 4 and crates.io 24 files gain or move a region; tripwire files 13 → 13 here, 227 → 239 std, 4,999 → 5,389 crates.io |
| `survey-gl.out` | The follow-up's gap: 0 here, 13 files in std, 62 in crates.io |
| `corpus-py314.out` | 40,059 files, 22.3 M lines: no old match lost, 53 new `starts_at` places all multi-line, tripwire never outside the rule |
| `misses.out` | 1,097 `any()`/`all()` markers holding a `test` item on this machine; the new `starts_at` misses none |
| `not-nested.out` | 1 of 1,097 new tripwire places has its `test` under `not(…)` (the old rule's heuristic, kept) |
| `bench-py314.out`, `bench-py39.out` | ×2 per doubling, slowest 0.028 s; the 3 wrong variants take 4.4–20.2 s on the suite's texts |
| `gate.out`, `gate-raw-main.out`, `gate-raw-steps.out` | Full gate on the private clone `8d46d89` (base, red, fix commits; never pushed), offline: 9 steps PASS, `dependencies` fails on network only; `cargo deny --offline` and the 5 later steps exit 0. The raw files have terminal colour codes and trailing white space removed; `gate.out` is a summary of them |
| `hook-e2e.out` | Before: edits inside the tests and the marker allowed; new guard alone: `fix-lock on` names the file; both new: denied, the fix allowed. Its last two lines per setup show the sibling gap left for the follow-up (`#[cfg(any(\n))]` above the marker allowed) |
| `r7-base-*`, `r7-red-*`, `r7-fix-*` (`-py314`, `-py39`) | Round 7: base 152 OK; red 154 run, the 3 new checks fail; fix 154 OK |
| `r7-rikurinode-py314.out`, `r7-rikurinode-py39.out` | rikuripi/rikurinode round 7 `f77f3f4` exported with the two shared files: 154 OK |
| `r7-hook-e2e.out` | The hook end-to-end on round 7: the same decisions |
| `r7-gate.out`, `r7-gate-raw-main.out`, `r7-gate-raw-steps.out` | Full gate on round 7: 9 PASS, `dependencies` network only; offline deny and the 5 later steps exit 0 |
