#!/usr/bin/env bash
# Regression test (2026-10-10): no-temp-residue.sh fails when a command
# leaves anything in its TMPDIR (the reentrancy tests left
# /tmp/.tmp*/state/rikuri.db.lock behind on most runs) and when it removes
# its TMPDIR or replaces it with a symlink (without following the link),
# passes a command that cleans up, keeps a failing command's exit status,
# gives the command a fresh private TMPDIR, and removes that directory every
# time.
set -euo pipefail
here="$(cd "$(dirname "$0")/.." && pwd)"

tmp="$(mktemp -d)"
# chmod first: a broken wrapper can leave the unreadable case's directory here
cleanup() { [ -n "$tmp" ] && [ -d "$tmp" ] && { chmod -R u+rwx -- "$tmp"; rm -rf -- "$tmp"; }; }
trap cleanup EXIT

# run <shell snippet>: the wrapper around the snippet, with the caller's own
# TMPDIR at $tmp/outer; output in $tmp/out, status in $rc, the TMPDIR the
# snippet saw in $seen
run() {
  mkdir -p "$tmp/outer"
  rm -f -- "$tmp/seen"
  rc=0
  TMPDIR="$tmp/outer" "$here/no-temp-residue.sh" \
    bash -c "printf '%s' \"\$TMPDIR\" >'$tmp/seen'; $1" >"$tmp/out" 2>&1 || rc=$?
  seen="$(cat "$tmp/seen")"
}

failed=0
fail() { echo "RESIDUE SELFTEST FAIL: $*"; sed 's/^/  /' "$tmp/out"; failed=1; }

# every run: a fresh directory under /tmp, not the caller's TMPDIR, gone after
check_private() { # check_private <case>
  case "$seen" in
    /tmp/rikuri-tests.*) ;;
    *) fail "$1: the command's TMPDIR was '$seen', not a fresh /tmp/rikuri-tests.*"; return ;;
  esac
  [ ! -e "$seen" ] || { fail "$1: the private TMPDIR $seen was not removed"; chmod -R u+rwx "$seen"; rm -rf -- "$seen"; }
}

run 'd="$(mktemp -d "$TMPDIR/.tmpXXXXXX")"; : >"$d/file"; rm -r -- "$d"'
check_private "clean"
if [ "$rc" = 0 ] && grep -q "RESIDUE none" "$tmp/out"; then
  echo "RESIDUE SELFTEST ok: a command that cleans up passes"
else
  fail "a command that cleans up did not pass (status $rc)"
fi

for case in \
  "the defect's tree|mkdir -p \"\$TMPDIR/.tmpLeak/state\"; : >\"\$TMPDIR/.tmpLeak/state/rikuri.db.lock\"|.tmpLeak/state/rikuri.db.lock" \
  "a hidden file|: >\"\$TMPDIR/.hidden\"|.hidden" \
  "an empty directory|mkdir \"\$TMPDIR/.tmpEmpty\"|.tmpEmpty" \
  "an unreadable directory|mkdir -p \"\$TMPDIR/locked/inner\"; chmod 000 \"\$TMPDIR/locked\"|locked"; do
  name="${case%%|*}" rest="${case#*|}"
  snippet="${rest%|*}" expected="${rest##*|}"
  run "$snippet"
  check_private "$name"
  if [ "$rc" = 0 ]; then
    fail "$name left in TMPDIR passed"
  elif grep -q "^RESIDUE: " "$tmp/out" && grep -qF "  $expected" "$tmp/out"; then
    echo "RESIDUE SELFTEST ok: $name is caught and named"
  else
    fail "$name: failed (status $rc) without naming $expected"
  fi
done

run 'rm -rf -- "$TMPDIR"'
check_private "removed TMPDIR"
if [ "$rc" = 1 ] && grep -q "^RESIDUE: the command removed or replaced its TMPDIR" "$tmp/out"; then
  echo "RESIDUE SELFTEST ok: a command that removes its TMPDIR fails (nothing left to check)"
else
  fail "a command that removed its TMPDIR gave status $rc"
fi

run 'rm -rf -- "$TMPDIR"; exit 5'
check_private "removed TMPDIR, failing"
if [ "$rc" = 5 ] && grep -q "^RESIDUE: the command removed or replaced its TMPDIR" "$tmp/out"; then
  echo "RESIDUE SELFTEST ok: a failing command that removes its TMPDIR keeps its status"
else
  fail "a command that removed its TMPDIR and exited 5 gave status $rc"
fi

# replaced by a symlink: refused, and the cleanup must not follow the link
mkdir -p "$tmp/elsewhere"
chmod 500 "$tmp/elsewhere"
run "rm -rf -- \"\$TMPDIR\"; ln -s '$tmp/elsewhere' \"\$TMPDIR\""
check_private "TMPDIR replaced by a symlink"
mode="$(stat -c %a "$tmp/elsewhere")"
chmod 700 "$tmp/elsewhere"
if [ "$rc" = 1 ] && grep -q "^RESIDUE: the command removed or replaced its TMPDIR" "$tmp/out" && [ "$mode" = 500 ]; then
  echo "RESIDUE SELFTEST ok: a TMPDIR replaced by a symlink fails, and the link is not followed"
else
  fail "a TMPDIR replaced by a symlink gave status $rc, and its target's mode became $mode (was 500)"
fi

run 'exit 3'
check_private "failing command"
if [ "$rc" = 3 ]; then
  echo "RESIDUE SELFTEST ok: a failing command's status is kept"
else
  fail "a command exiting 3 gave status $rc"
fi

run ': >"$TMPDIR/.left"; exit 4'
check_private "failing command with residue"
if [ "$rc" = 4 ] && grep -qF "  .left" "$tmp/out"; then
  echo "RESIDUE SELFTEST ok: a failing command's residue is named and its status kept"
else
  fail "a command leaving .left and exiting 4 gave status $rc"
fi

exit "$failed"
