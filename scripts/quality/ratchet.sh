#!/usr/bin/env bash
# Debt ratchet (Engineering Standard, Phase 0): counted debt may only go
# down. Compares current counts with ratchet-baseline.json and fails on any
# rise. A fall passes with a notice; lower the baseline in a later change.
#
#   ratchet.sh [root]            check against the baseline
#   ratchet.sh --update [root]   print current counts as JSON (a baseline change
#                                is a reviewed change; the fix-lock makes it
#                                read-only)
set -euo pipefail
update=0
if [ "${1:-}" = "--update" ]; then update=1; shift; fi
cd "${1:-$(git rev-parse --show-toplevel)}"
baseline=scripts/quality/ratchet-baseline.json

# count <regex> <pathspec...>: matching lines in tracked files. git grep
# exits 1 for "no match", which counts as 0; any other failure stops the
# ratchet instead of reading as zero debt.
count() {
  local regex="$1" out rc=0; shift
  out="$(git grep -hcE -e "$regex" -- "$@")" || rc=$?
  if [ "$rc" -gt 1 ]; then echo "RATCHET git grep failed (exit $rc) for ${regex}" >&2; return 1; fi
  printf '%s\n' "$out" | awk 'NF {s += $1} END {print s + 0}'
}

# Each count in its own assignment, so a failed count stops the script.
ignored_tests="$(count '#\[ignore' '*.rs')"
# Suppressions: #[allow(…)], #![expect(…)] and #[cfg_attr(…, allow(…))].
lint_suppressions="$(count '#!?\[(cfg_attr\(.*)?(allow|expect)\(' '*.rs')"
todo_markers="$(count '\b(TODO|FIXME|XXX|HACK)\b' '*.rs' '*.sh' '*.toml' '*.yml' ':!scripts/quality/ratchet.sh')"
current="$(cat <<JSON
{
  "ignored_tests": ${ignored_tests},
  "lint_suppressions": ${lint_suppressions},
  "todo_markers": ${todo_markers}
}
JSON
)"

if [ "$update" = 1 ]; then
  echo "$current"
  exit 0
fi

python3 - "$baseline" "$current" <<'PY'
import json, sys
base = json.load(open(sys.argv[1]))
cur = json.loads(sys.argv[2])
failed = False
for key in sorted(set(base) | set(cur)):
    b, c = base.get(key), cur.get(key)
    if b is None or c is None:
        print(f"RATCHET {key}: present in only one of baseline/current"); failed = True
    elif c > b:
        print(f"RATCHET {key}: {c} > baseline {b} (debt rose)"); failed = True
    elif c < b:
        print(f"RATCHET {key}: {c} < baseline {b} (improved; lower the baseline in a later change)")
    else:
        print(f"RATCHET {key}: {c} (baseline {b})")
sys.exit(1 if failed else 0)
PY
