#!/usr/bin/env bash
# Regression test (T00.10, 2026-10-08): smoke.sh must check content, not
# liveness. Run against fake rikurid/rikuri binaries, it passes only when the
# content is right, and fails, for the right reason, when the task title is
# missing from state.show, leaks into the ledger, health is not healthy, the
# task is gone after a restart, or the daemon refuses the write.
set -euo pipefail
here="$(cd "$(dirname "$0")/.." && pwd)"

tmp="$(mktemp -d)"
cleanup() { [ -n "$tmp" ] && [ -d "$tmp" ] && rm -rf -- "$tmp"; }
trap cleanup EXIT

# fake daemon: holds a socket at HOME/run/rikurid.sock and counts its starts
cat >"$tmp/rikurid" <<'FAKE'
#!/usr/bin/env bash
home="$2"
mkdir -p -m 700 "$home/run"
printf x >>"$home/fake-starts"
exec python3 -c '
import os, socket, sys, time
p = sys.argv[1]
if os.path.exists(p):
    os.unlink(p)
s = socket.socket(socket.AF_UNIX)
s.bind(p)
s.listen(1)
time.sleep(600)' "$home/run/rikurid.sock"
FAKE

# fake CLI: answers like rikuri; FAKE_MODE chooses what is wrong
cat >"$tmp/rikuri" <<'FAKE'
#!/usr/bin/env python3
import json, os, sys
args = sys.argv[1:]
i = args.index("--home")
home = args[i + 1]
del args[i:i + 2]
mode = os.environ.get("FAKE_MODE", "good")
title_file = os.path.join(home, "fake-title")
starts_file = os.path.join(home, "fake-starts")
starts = open(starts_file).read().count("x") if os.path.exists(starts_file) else 0
title = open(title_file).read() if os.path.exists(title_file) else ""

def ok(result):
    print(json.dumps({"v": 1, "ok": True, "result": result}))

if args[:1] == ["init"]:
    ok({})
elif args[:2] == ["task", "create"]:
    if mode == "refused":
        print(json.dumps({"v": 1, "ok": False, "error": {"code": "invalid_request"}}))
        sys.exit(3)
    open(title_file, "w").write(args[args.index("--title") + 1])
    ok({})
elif args[:2] == ["state", "show"]:
    gone = mode == "no-content" or (mode == "lost-after-restart" and starts >= 2)
    ok({"tasks": [] if gone else [{"title": title}]})
elif args[:2] == ["ledger", "tail"]:
    event = {"event_type": "task.created"}
    if mode == "leak":
        event["note"] = title
    ok({"events": [event]})
elif args[:2] == ["system", "health"]:
    ok({"state": "degraded" if mode == "unhealthy" else "healthy"})
else:
    sys.exit(2)
FAKE
chmod +x "$tmp/rikurid" "$tmp/rikuri"

run() { # run <mode>: smoke against the fakes; output in $tmp/out
  env FAKE_MODE="$1" RIKURID_BIN="$tmp/rikurid" RIKURI_BIN="$tmp/rikuri" \
    "$here/smoke.sh" >"$tmp/out" 2>&1
}

failed=0
if run good; then
  echo "SMOKE SELFTEST ok: correct content passes"
else
  echo "SMOKE SELFTEST FAIL: smoke failed on correct content"
  sed 's/^/  /' "$tmp/out"
  failed=1
fi

for case in \
  "refused|task create exited non-zero" \
  "no-content|does not contain the task" \
  "leak|leaked into the ledger" \
  "unhealthy|is not healthy" \
  "lost-after-restart|after a restart"; do
  mode="${case%%|*}" reason="${case#*|}"
  if run "$mode"; then
    echo "SMOKE SELFTEST FAIL: smoke passed although $mode"
    failed=1
  elif grep -q "$reason" "$tmp/out"; then
    echo "SMOKE SELFTEST ok: $mode is caught"
  else
    echo "SMOKE SELFTEST FAIL: $mode failed for another reason:"
    sed 's/^/  /' "$tmp/out"
    failed=1
  fi
done
exit "$failed"
