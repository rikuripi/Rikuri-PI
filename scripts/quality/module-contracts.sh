#!/usr/bin/env bash
# Module Contract check (Stage Plan T00.11): every workspace member,
# wherever it lives (members come from Cargo via workspace-members.sh), has a
# README.md contract (Engineering Templates, Template C) with at least its
# Owns, Must not own and Health sections. A heading counts when the name
# ends at a space or the line end, so the template's
# "## Health / last-success signal" counts and "## Healthiness" does not.
# Usage: module-contracts.sh [root] (default: repository root)
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "${1:-$(git rev-parse --show-toplevel)}" && pwd)"
members="$("$here/workspace-members.sh" "$root")"
cd "$root"

failed=0 checked=0
while IFS= read -r dir; do
  [ -n "$dir" ] || continue
  checked=$((checked + 1))
  readme="${dir}/README.md"
  if [ ! -f "$readme" ]; then
    echo "CONTRACT ${dir}: missing README.md"; failed=1; continue
  fi
  for heading in '## Owns' '## Must not own' '## Health'; do
    grep -qE "^${heading}([[:space:]]|$)" "$readme" || { echo "CONTRACT ${dir}: missing '${heading}'"; failed=1; }
  done
done <<<"$members"
echo "CONTRACT checked ${checked} crate(s)"
[ "$checked" -gt 0 ] || { echo "CONTRACT no crates found"; failed=1; }
exit "$failed"
