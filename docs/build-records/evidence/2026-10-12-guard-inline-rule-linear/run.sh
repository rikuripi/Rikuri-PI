#!/usr/bin/env bash
# run.sh <output file> <command…>: runs the command inside rikuripi/rikurinode's
# scripts/quality/safe-run.sh (path in $SAFE_RUN; used at 86db3e9), with a header
# (start time, command) and a footer (end time, exit code, lnd StartedAt before
# and after) in the output file, trailing white space removed. This machine
# runs a live node.
set -uo pipefail
out=$1; shift
SAFE=${SAFE_RUN:?set SAFE_RUN to scripts/quality/safe-run.sh of rikuripi/rikurinode}
start=$(date '+%F %T')
before=$(docker inspect -f '{{.State.StartedAt}}' lnd 2>/dev/null)
raw=$("$SAFE" "$@" 2>&1); rc=$?
after=$(docker inspect -f '{{.State.StartedAt}}' lnd 2>/dev/null)
same=" (CHANGED)"; [ "$before" = "$after" ] && same=" (unchanged)"
{ echo "# started $start, safe-run: $*"; printf '%s\n' "$raw" | sed 's/[[:space:]]*$//'
  echo "# ended $(date '+%F %T'); exit $rc; lnd StartedAt $before -> $after$same"; } > "$out"
cat "$out"
exit $rc
