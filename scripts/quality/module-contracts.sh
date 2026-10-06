#!/usr/bin/env bash
# Module Contract check (Stage Plan T00.11): every crate in crates/ and every
# app in apps/ has a README.md contract (Engineering Templates, Template C)
# with at least its Owns, Must not own and Health sections. A heading counts
# when the name ends at a space or the line end, so the template's
# "## Health / last-success signal" counts and "## Healthiness" does not.
# Usage: module-contracts.sh [root] (default: repository root)
set -euo pipefail
cd "${1:-$(git rev-parse --show-toplevel)}"

failed=0 checked=0
for dir in crates/*/ apps/*/; do
  [ -f "${dir}Cargo.toml" ] || continue
  checked=$((checked + 1))
  readme="${dir}README.md"
  if [ ! -f "$readme" ]; then
    echo "CONTRACT ${dir%/}: missing README.md"; failed=1; continue
  fi
  for heading in '## Owns' '## Must not own' '## Health'; do
    grep -qE "^${heading}([[:space:]]|$)" "$readme" || { echo "CONTRACT ${dir%/}: missing '${heading}'"; failed=1; }
  done
done
echo "CONTRACT checked ${checked} crate(s)"
[ "$checked" -gt 0 ] || { echo "CONTRACT no crates found"; failed=1; }
exit "$failed"
