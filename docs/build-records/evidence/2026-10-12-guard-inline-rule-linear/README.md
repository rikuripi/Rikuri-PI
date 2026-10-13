# Evidence: the `inline_tests` rule decides in linear time

Record: [../../2026-10-12-guard-inline-rule-linear.md](../../2026-10-12-guard-inline-rule-linear.md).
Every run was made inside rikuripi/rikurinode's `scripts/quality/safe-run.sh` (at `86db3e9`), because the
machine runs a live node; each output ends with lnd's container start time before and after. Suite, red,
hook and gate runs are on round 6 (and the `round7-*` files on round 7, `444badf`) (Rikuri-PI `f9f58c0`, rikuripi/rikurinode `97a2a8b`); the pattern-only
runs (differential, corpus, bench, variants) do not depend on the guard's version.

## Tools

| File | What it does |
|---|---|
| `rule_new.py` | The rule before (`dc7e327`'s `guard-config.json`, the same at `f9f58c0`) and after: the two new patterns are built from the old rule by exact substring replacement. Run it to print both and see which one the config holds now |
| `differential.py` | Old vs new on random and attribute-shaped text: every line start where `starts_at` begins a match (`re.M`), and `guarded_lines` on every line. Args: seconds, seed |
| `fuzz_power.py` | 11 deliberately wrong variants of the new patterns; the differential must catch each one. Args: seconds per variant |
| `corpus.py` | Old vs new on every `.rs` file under the folders given |
| `bench.py` | One call of each pattern, old and new, on 31 crafted shapes at sizes k and 2k (×4 = quadratic, ×2 = linear). Arg: k |
| `gl_variants.py` | Ways of writing `guarded_lines` that decide the same, timed on a large real file and on one crafted line: why the whole pattern is anchored |
| `survey_multiline.py` | Counts rustfmt-style multi-line `#[cfg(any(`/`#[cfg(all(` test markers, which no rule or tripwire sees (a limit outside this change) |
| `test_texts_table.py` | The texts of `GenericPolicy.test_inline_rules_take_linear_time`, read from the test file, timed against both rules |
| `run.sh`, `suite.sh` | Wrappers: run a command, or the guard suite of a checkout, inside `$SAFE_RUN`, recording time, file hashes, exit code and lnd's start time |
| `verifier-probes/` | Verifier round 3's reproducers, copied unchanged from the origin session's work folder (which is deleted at its close-out): `rule_stall_hook2.py` (the real hook process on a scratch crate, lock held), `rule_bench.py` (single calls), `harness.py`. Each takes a hooks folder: a copy of this repository's `.claude/hooks` without `test_guard.py`, at the base (old) or with this change (new) |

To rerun, from a checkout of this change: `export SAFE_RUN=<rikuripi/rikurinode>/scripts/quality/safe-run.sh`, then e.g.
`run.sh out.txt python3 -B differential.py 60 1`. Outputs have trailing white space removed (`git diff --check`;
the verifier's harness pads its truncated lines) and nothing else, except `gate.out`, a summary of the two
`gate-raw-*.out` files beside it (those have only terminal colour codes and trailing white space removed).

## Outputs

| File | Result |
|---|---|
| `round7-base-py314.out`, `round7-red-py314.out`, `round7-red-py39.out`, `round7-green-py314.out`, `round7-green-py39.out`, `round7-rule_stall_hook2.out` | The same on round 7 (`444badf`), where the change was rebased before the pull request: base 152 OK; red fails only the linear-time test; green 152 OK on both Pythons; the hook reproducers deny in 0.07 s |
| `base-py314.out` | The suite as committed at `f9f58c0`: 142 OK |
| `red-py314.out`, `red-py39.out` | Extended test, old rule: only `test_inline_rules_take_linear_time` fails, on Python 3.14.4 and 3.9.25 |
| `red-test-texts.out` | The four added texts: 0.74–3.3 s on the old rule; every text at most 2.2 ms on the new |
| `green-py314.out`, `green-py39.out` | New rule, same test file: 142 OK on Python 3.14.4 and 3.9.25 |
| `rikurinode-py314.out`, `rikurinode-py39.out` | rikuripi/rikurinode with the same test_guard.py: 142 OK on both |
| `differential-py314.out`, `differential-py39.out` | 41.1 million texts, 157 million lines, no difference; 11 of 11 wrong variants caught |
| `corpus.out` | 40,059 real `.rs` files, the same decisions |
| `bench-k8000.out` | Old ×4 per doubling on 11 shapes (up to 13.4 s a call); new ×2, slowest 5 ms |
| `gl-variants.out` | `guarded_lines` over a 52,857-line file: old 0.029 s, only the `cfg(` branch anchored 0.067 s, both anchored (the rule now) 0.035 s |
| `verifier-rule_bench-old.out`, `verifier-rule_bench-new.out` | The verifier's bench. Rule lines: up to 33.3 s old, at most 5 ms new; the lines after them time the guard's own scanners, which this change does not touch |
| `verifier-round3-rule_stall_hook2.out` | Verifier round 3's own output (before this change): 16.55 s and 18.51 s, the figures the test's comment quotes |
| `survey-multiline.out` | Multi-line test markers: 0 here, 4 files in the standard library, 23 in crates.io sources |
| `verifier-rule_stall_hook2.out` | Through the real hook, lock held: 15.38 s / 17.34 s old, 0.09 s / 0.08 s new, deny each time |
| `gate.out`, `gate-raw-main.out`, `gate-raw-steps.out` | Full gate on a private committed copy |
