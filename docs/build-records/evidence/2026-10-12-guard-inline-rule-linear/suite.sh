#!/usr/bin/env bash
# suite.sh <label> <python> <repo dir> <output file>
# Runs the guard suite in <repo dir> inside rikuripi/rikurinode's safe-run.sh ($SAFE_RUN) and
# writes a header (time, Python, file hashes, exit code, lnd StartedAt before and
# after), every failing test with its assertion line, and the summary.
set -uo pipefail
label=$1 py=$2 dir=$3 out=$4
SAFE=${SAFE_RUN:?set SAFE_RUN to scripts/quality/safe-run.sh of rikuripi/rikurinode}
h() { sha256sum "$dir/.claude/hooks/$1" | cut -c1-8; }
before=$(docker inspect -f '{{.State.StartedAt}}' lnd 2>/dev/null)
t0=$(date +%s); start=$(date '+%F %T')
raw=$(cd "$dir" && "$SAFE" "$py" -B -m unittest discover -s .claude/hooks -p 'test_*.py' 2>&1)
rc=$?
t1=$(date +%s)
after=$(docker inspect -f '{{.State.StartedAt}}' lnd 2>/dev/null)
same="CHANGED"; [ "$before" = "$after" ] && same="unchanged"
{
  echo "# $label, started $start: $($py --version 2>&1); guard.py $(h guard.py)…, test_guard.py $(h test_guard.py)…," \
       "guard_base.py $(h guard_base.py)…, guard-config.json $(h guard-config.json)…; safe-run; exit $rc; $((t1 - t0)) s;" \
       "lnd StartedAt $before -> $after ($same)"
  printf '%s\n' "$raw" | grep -E '^(FAIL|ERROR):|^AssertionError|^[A-Za-z]*Error: ' | sed 's/[[:space:]]*$//' | cut -c1-300
  printf '%s\n' "$raw" | grep -E '^Ran [0-9]+ tests|^OK|^FAILED' | sed 's/[[:space:]]*$//'
} > "$out"
cat "$out"
