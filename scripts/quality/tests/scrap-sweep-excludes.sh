#!/usr/bin/env bash
# Regression test (verification of the second PR #7 review fix, 2026-10-05):
# the Scrap Sweep's force-added check used --exclude-standard, so a personal
# excludes file or .git/info/exclude listing a tracked file (Cargo.lock in
# the verifier's run) failed the sweep on one machine and not another. Only
# the repository's .gitignore files may decide what was force-added.
set -euo pipefail
here="$(cd "$(dirname "$0")/.." && pwd)"

tmp="$(mktemp -d)"
cleanup() { [ -n "$tmp" ] && [ -d "$tmp" ] && rm -rf -- "$tmp"; }
trap cleanup EXIT
g() { git -C "$tmp/repo" -c user.email=sweep-test@example.invalid -c user.name=sweep-test -c commit.gpgsign=false "$@"; }

mkdir -p "$tmp/repo"
g init -q -b main
echo lock > "$tmp/repo/Cargo.lock"; g add Cargo.lock; g commit -q -m lock
printf 'Cargo.lock\n' > "$tmp/personal-excludes"
printf 'Cargo.lock\n' >> "$tmp/repo/.git/info/exclude"

if (cd "$tmp/repo" && GIT_CONFIG_COUNT=1 GIT_CONFIG_KEY_0=core.excludesFile GIT_CONFIG_VALUE_0="$tmp/personal-excludes" \
    "$here/scrap-sweep.sh") >"$tmp/out" 2>&1; then
  echo "EXCLUDES ok: personal and info/exclude rules do not fail the sweep"
else
  echo "EXCLUDES FAIL: a tracked file named only in personal excludes failed the sweep"; sed 's/^/  /' "$tmp/out"; exit 1
fi
