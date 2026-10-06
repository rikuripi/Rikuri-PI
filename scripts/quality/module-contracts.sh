#!/usr/bin/env bash
# Module Contract check (Stage Plan T00.11): every crate in crates/ and every
# app in apps/ has a README.md contract (Engineering Templates, Template C)
# with at least its Owns, Must not own and Health sections.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

failed=0 checked=0
for dir in crates/*/ apps/*/; do
  [ -f "${dir}Cargo.toml" ] || continue
  checked=$((checked + 1))
  readme="${dir}README.md"
  if [ ! -f "$readme" ]; then
    echo "CONTRACT ${dir%/}: missing README.md"; failed=1; continue
  fi
  for heading in '## Owns' '## Must not own' '## Health'; do
    grep -q "^${heading}" "$readme" || { echo "CONTRACT ${dir%/}: missing '${heading}'"; failed=1; }
  done
done
echo "CONTRACT checked ${checked} crate(s)"
[ "$checked" -gt 0 ] || { echo "CONTRACT no crates found"; failed=1; }
exit "$failed"
