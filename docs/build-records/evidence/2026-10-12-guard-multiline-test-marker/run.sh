#!/usr/bin/env bash
# run.sh <output file> <command…>: runs the command inside rikuripi/rikurinode's
# safe-run.sh (identical to 97a2a8b's; $SAFE_RUN names it, else the work folder's copy), with a
# header and footer (time, command, exit code, lnd StartedAt before/after) in
# the output file.
set -uo pipefail
out=$1; shift
SAFE=${SAFE_RUN:-/home/junc/claude-wip/guard-multiline-test-marker/tools/safe-run.sh}
before=$(docker inspect -f '{{.State.StartedAt}}' lnd 2>/dev/null)
raw=$("$SAFE" "$@" 2>&1); rc=$?
after=$(docker inspect -f '{{.State.StartedAt}}' lnd 2>/dev/null)
same=" (CHANGED)"; [ "$before" = "$after" ] && same=" (unchanged)"
{ echo "# $(date '+%F %T') safe-run: $*"; printf '%s\n' "$raw"
  echo "# exit $rc; lnd StartedAt $before -> $after$same"; } > "$out"
cat "$out"
exit $rc
