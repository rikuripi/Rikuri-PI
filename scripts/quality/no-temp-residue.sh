#!/usr/bin/env bash
# Runs a command (the gate's and CI's `cargo test --workspace`, smoke and the
# crash matrix) with TMPDIR set to a fresh private directory, and fails if
# the command fails, leaves anything in that directory, or removes or
# replaces the directory (then nothing can be checked): a test cleans up its
# scratch files. Before
# 2026-10-10, rikuri-storage's reentrancy tests let the process exit while
# their threads were still removing their temporary directories, and most
# runs left /tmp/.tmp*/state/ behind
# (docs/build-records/2026-10-10-test-temp-residue.md).
#
# It sees what honours TMPDIR (tempfile::tempdir(), std::env::temp_dir()),
# not scratch made at a fixed path. The private directory is under /tmp, not
# $TMPDIR: a Unix socket path in a test's home must stay short (about 104
# bytes on macOS), and $TMPDIR there can be long. The crash matrix and smoke
# put their daemon homes in TMPDIR only while the socket path fits (until
# 2026-10-10 always in /tmp, outside this check;
# docs/build-records/2026-10-10-crash-and-smoke-temp-residue.md), so keep
# this path short. It is removed afterwards, once anything left in it has
# been listed.
# Usage: no-temp-residue.sh <command> [args...]
set -euo pipefail
[ "$#" -gt 0 ] || { echo "usage: $0 <command> [args...]" >&2; exit 2; }

scratch="$(mktemp -d /tmp/rikuri-tests.XXXXXX)" # 0700
# chmod lets rm into unreadable leftovers; never through a link put in its place
cleanup() {
  [ -L "$scratch" ] || chmod -R u+rwx -- "$scratch" 2>/dev/null || true
  rm -rf -- "$scratch"
}
trap cleanup EXIT

rc=0
TMPDIR="$scratch" "$@" || rc=$?

if [ -L "$scratch" ] || [ ! -d "$scratch" ]; then
  echo "RESIDUE: the command removed or replaced its TMPDIR ($scratch), so what it left cannot be checked"
  [ "$rc" != 0 ] || rc=1
  exit "$rc"
fi
# find's own errors (an unreadable directory) are listed too: something is there
residue="$(cd "$scratch" && find . -mindepth 1 2>&1 | LC_ALL=C sort)" || true
if [ -n "$residue" ]; then
  echo "RESIDUE: the command left $(wc -l <<<"$residue" | tr -d ' ') entries in its TMPDIR; a test did not clean up:"
  head -n 200 <<<"$residue" | sed 's|^\./|  |'
  [ "$rc" != 0 ] || rc=1
elif [ "$rc" = 0 ]; then
  echo "RESIDUE none: TMPDIR stayed empty"
fi
exit "$rc"
