#!/usr/bin/env bash
# Debt ratchet (Engineering Standard, Phase 0): counted debt may only go
# down. Compares current counts with ratchet-baseline.json and fails on any
# rise. A fall passes with a notice; lower the baseline in a later change.
#
#   ratchet.sh            check against the baseline
#   ratchet.sh --update   print current counts as JSON (a baseline change is a
#                         reviewed change; the fix-lock makes it read-only)
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
baseline=scripts/quality/ratchet-baseline.json

# count <regex> <pathspec...>: matching lines in tracked files.
count() {
  local regex="$1"; shift
  { git grep -hcE -e "$regex" -- "$@" || true; } | awk '{s += $1} END {print s + 0}'
}

current="$(cat <<JSON
{
  "ignored_tests": $(count '#\[ignore' '*.rs'),
  "lint_suppressions": $(count '#!?\[(allow|expect)\(' '*.rs'),
  "todo_markers": $(count '\b(TODO|FIXME|XXX|HACK)\b' '*.rs' '*.sh' '*.toml' '*.yml' ':!scripts/quality/ratchet.sh')
}
JSON
)"

if [ "${1:-}" = "--update" ]; then
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
