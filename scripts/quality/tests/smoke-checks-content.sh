#!/usr/bin/env bash
# Regression test (T00.10, 2026-10-08): smoke.sh must check content, not
# liveness. Run against fake rikurid/rikuri binaries, it passes only when the
# content is right, and fails, for the right reason, when the task title is
# missing from state.show, leaks into the ledger, health is not healthy, the
# task is gone after a restart, or the daemon refuses the write.
#
# And (2026-10-10) its scratch home goes under a short TMPDIR, so the gate's
# no-temp-residue.sh sees one left behind: a smoke killed from outside (no
# cleanup runs) fails the wrapped step and names the home. Until then smoke
# always used /tmp, outside the wrapper's check. A TMPDIR too long for the
# daemon's socket, or none, keeps /tmp.
set -euo pipefail
here="$(cd "$(dirname "$0")/.." && pwd)"

tmp="$(mktemp -d)"
bound="" # the limit cases' directory under /tmp
cleanup() {
  [ -z "$bound" ] || rm -rf -- "$bound"
  [ -n "$tmp" ] && [ -d "$tmp" ] && rm -rf -- "$tmp"
}
trap cleanup EXIT

# fake daemon: holds a socket at HOME/run/rikurid.sock and counts its starts;
# records its home in $FAKE_HOMES; FAKE_MODE=killed kills smoke (its parent)
# with SIGKILL instead, as a run stopped from outside would be
cat >"$tmp/rikurid" <<'FAKE'
#!/usr/bin/env bash
home="$2"
mkdir -p -m 700 "$home/run"
printf x >>"$home/fake-starts"
printf '%s\n' "$home" >>"$FAKE_HOMES"
if [ "${FAKE_MODE:-}" = killed ]; then kill -KILL "$PPID"; exit 0; fi
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

# run <mode> [command...]: smoke against the fakes, through the command if
# given; output in $tmp/out, the homes the fake daemon got in $tmp/homes
run() {
  local mode="$1"; shift
  : >"$tmp/homes"
  env FAKE_MODE="$mode" FAKE_HOMES="$tmp/homes" RIKURID_BIN="$tmp/rikurid" RIKURI_BIN="$tmp/rikuri" \
    "$@" "$here/smoke.sh" >"$tmp/out" 2>&1
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

# placed <case> <pattern>: smoke made its home (the fake daemon's first)
# at a path matching the pattern, and removed it
placed() {
  local home
  home="$(head -n 1 "$tmp/homes")"
  case "$home" in
    $2) ;;
    *)
      echo "SMOKE SELFTEST FAIL: $1: smoke's home was '$home', not $2"
      sed 's/^/  /' "$tmp/out"
      failed=1
      return
      ;;
  esac
  if [ -e "${home%/home}" ]; then
    echo "SMOKE SELFTEST FAIL: $1: smoke left ${home%/home} behind"
    failed=1
  else
    echo "SMOKE SELFTEST ok: $1"
  fi
}

# Through the gate's wrapper (a short private TMPDIR, /tmp/rikuri-tests.*):
# the home is inside it, and a clean run leaves nothing there
rc=0
run good "$here/no-temp-residue.sh" || rc=$?
if [ "$rc" = 0 ] && grep -q "^RESIDUE none" "$tmp/out"; then
  placed "under the gate's wrapper, the home is in its TMPDIR" \
    "/tmp/rikuri-tests.??????/rikuri-smoke.??????/home"
else
  echo "SMOKE SELFTEST FAIL: smoke under the gate's wrapper gave status $rc"
  sed 's/^/  /' "$tmp/out"
  failed=1
fi

# A smoke killed from outside runs no cleanup: the wrapped step fails and
# names the home it left (before 2026-10-10 it stayed in /tmp, unseen)
rc=0
run killed "$here/no-temp-residue.sh" || rc=$?
home="$(head -n 1 "$tmp/homes")"
if [ "$rc" = 137 ] && grep -q "^RESIDUE: " "$tmp/out" &&
  grep -qE '^  rikuri-smoke\.[A-Za-z0-9]{6}/home$' "$tmp/out"; then
  echo "SMOKE SELFTEST ok: a killed smoke's home fails the wrapped step and is named"
else
  echo "SMOKE SELFTEST FAIL: a killed smoke under the wrapper gave status $rc without naming its home ($home)"
  sed 's/^/  /' "$tmp/out"
  failed=1
fi
# a home outside the wrapper's TMPDIR is this test's own leak: remove it
case "$home" in
  /tmp/rikuri-smoke.??????/home) [ ! -d "${home%/home}" ] || rm -rf -- "${home%/home}" ;;
esac

# placed_with <case> <pattern> <TMPDIR setting for env>: smoke passes with
# that TMPDIR and puts its home at a path matching the pattern
placed_with() {
  if run good env "$3"; then
    placed "$1" "$2"
  else
    echo "SMOKE SELFTEST FAIL: $1: smoke failed"
    sed 's/^/  /' "$tmp/out"
    failed=1
  fi
}

# /tmp when TMPDIR is too long for the daemon's socket, unset, relative, or
# the root
long="$tmp/$(printf 'l%.0s' $(seq 70))"
mkdir "$long"
placed_with "a TMPDIR too long for the socket keeps /tmp" "/tmp/rikuri-smoke.??????/home" TMPDIR="$long"
placed_with "no TMPDIR keeps /tmp" "/tmp/rikuri-smoke.??????/home" -uTMPDIR
placed_with "a relative TMPDIR keeps /tmp" "/tmp/rikuri-smoke.??????/home" TMPDIR=relative-tmp
placed_with "TMPDIR=/ keeps /tmp" "/tmp/rikuri-smoke.??????/home" TMPDIR=/

# The limit (rikurid's MAX_SOCKET_PATH_BYTES, 103): a TMPDIR whose socket
# path is exactly 103 bytes is used, one byte longer is not. Built under
# /tmp, so its length is known whatever TMPDIR this test runs with.
bound="$(mktemp -d /tmp/rikuri-bound.XXXXXX)"
at_limit="$bound/$(printf 'b%.0s' $(seq 36))"
over="${at_limit}b"
mkdir "$at_limit" "$over"
sock_bytes() { printf '%s' "$1/rikuri-smoke.XXXXXX/home/run/rikurid.sock" | wc -c | tr -d ' '; }
if [ "$(sock_bytes "$at_limit")" != 103 ] || [ "$(sock_bytes "$over")" != 104 ]; then
  echo "SMOKE SELFTEST FAIL: the limit cases give sockets of $(sock_bytes "$at_limit") and $(sock_bytes "$over") bytes, not 103 and 104"
  failed=1
fi
placed_with "a TMPDIR whose socket path is 103 bytes is used" \
  "$at_limit/rikuri-smoke.??????/home" TMPDIR="$at_limit"
placed_with "one byte longer keeps /tmp" "/tmp/rikuri-smoke.??????/home" TMPDIR="$over"
exit "$failed"
