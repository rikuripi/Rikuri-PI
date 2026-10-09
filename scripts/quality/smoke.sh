#!/usr/bin/env bash
# T00.10 smoke (Stage Plan S00.07): start the real rikurid on a scratch home,
# drive it with the real rikuri CLI, and check content, not liveness:
#   - a unique task title appears in state.show,
#   - the title never appears in the ledger (owner content stays out of it),
#   - system.health reports healthy,
#   - after the daemon is stopped and restarted, the task is still there.
# Exits 0 and prints "SMOKE PASS" only if every check holds.
#
# Uses target/debug (or $CARGO_TARGET_DIR) binaries built by the gate's
# build step. RIKURID_BIN and RIKURI_BIN override them (the gate's self-test
# uses that to prove these checks fail on wrong content).
set -euo pipefail
cd "$(dirname "$0")/../.." # the repository root, like the other quality scripts

target="${CARGO_TARGET_DIR:-target}"
rikurid="${RIKURID_BIN:-$target/debug/rikurid}"
rikuri="${RIKURI_BIN:-$target/debug/rikuri}"

# Under /tmp, not $TMPDIR: a Unix socket path must stay short (about 104
# bytes on macOS), and a long $TMPDIR would push run/rikurid.sock past it.
work="$(mktemp -d /tmp/rikuri-smoke.XXXXXX)" # 0700
home="$work/home"
log="$work/rikurid.log"
pid=""

stop_daemon() {
  if [ -n "$pid" ]; then
    kill "$pid" 2>/dev/null || true
    wait "$pid" 2>/dev/null || true
    pid=""
  fi
}
cleanup() { stop_daemon; rm -rf -- "$work"; }
trap cleanup EXIT

fail() {
  echo "SMOKE FAIL: $*"
  if [ -s "$log" ]; then sed 's/^/  daemon: /' "$log"; fi
  exit 1
}

cli() { "$rikuri" --home "$home" "$@"; }

# Ready means the daemon answers, not that a socket file exists: after a
# restart the stopped daemon's socket is still there until the new one
# replaces it.
start_daemon() {
  "$rikurid" --home "$home" >>"$log" 2>&1 &
  pid=$!
  local i
  for i in $(seq 1 100); do
    kill -0 "$pid" 2>/dev/null || fail "rikurid exited during startup"
    cli system health >/dev/null 2>&1 && return 0
    sleep 0.1
  done
  fail "rikurid did not answer within 10 s"
}

# expect_ok <label> <envelope>: the response envelope says ok
expect_ok() {
  python3 -c 'import json,sys; sys.exit(0 if json.loads(sys.argv[1]).get("ok") is True else 1)' "$2" \
    || fail "$1 did not succeed: $2"
}

[ -x "$rikurid" ] || fail "no rikurid at $rikurid (run cargo build first)"
[ -x "$rikuri" ] || fail "no rikuri at $rikuri (run cargo build first)"

marker="smoke-$$-$RANDOM-$(date +%s)"

start_daemon
out="$(cli init)" || fail "init exited non-zero: $out"
expect_ok "init" "$out"
out="$(cli task create --title "$marker" --idempotency-key "smoke-key-$$-$RANDOM")" \
  || fail "task create exited non-zero: $out"
expect_ok "task create" "$out"

state="$(cli state show)" || fail "state show exited non-zero: $state"
expect_ok "state show" "$state"
python3 -c '
import json, sys
tasks = json.loads(sys.argv[1])["result"]["tasks"]
sys.exit(0 if any(t.get("title") == sys.argv[2] for t in tasks) else 1)' "$state" "$marker" \
  || fail "state.show does not contain the task titled $marker"

ledger="$(cli ledger tail --limit 50)" || fail "ledger tail exited non-zero: $ledger"
expect_ok "ledger tail" "$ledger"
python3 -c '
import json, sys
events = json.loads(sys.argv[1])["result"]["events"]
sys.exit(0 if any(e.get("event_type") == "task.created" for e in events) else 1)' "$ledger" \
  || fail "the ledger has no task.created event"
case "$ledger" in
  *"$marker"*) fail "the task title leaked into the ledger" ;;
esac

health="$(cli system health)" || fail "system health exited non-zero: $health"
python3 -c 'import json,sys; sys.exit(0 if json.loads(sys.argv[1])["result"]["state"] == "healthy" else 1)' "$health" \
  || fail "system.health is not healthy: $health"

# the committed task survives a restart
stop_daemon
start_daemon
state="$(cli state show)" || fail "state show after restart exited non-zero: $state"
python3 -c '
import json, sys
tasks = json.loads(sys.argv[1])["result"]["tasks"]
sys.exit(0 if any(t.get("title") == sys.argv[2] for t in tasks) else 1)' "$state" "$marker" \
  || fail "after a restart, state.show no longer contains the task"
stop_daemon

# the daemon logs commands and outcomes, never the owner's content
if grep -qF -- "$marker" "$log" 2>/dev/null; then
  fail "the task title appeared in the daemon log"
fi

echo "SMOKE PASS: init, task create, state.show content, ledger and log without content, health, restart"
