#!/usr/bin/env bash
# Regression tests (PR #7 review, 2026-10-05):
# 1. A crate without `[lints] workspace = true` escaped the workspace lint
#    policy (unsafe, unwrap, print) and passed the full gate. crate-policy.sh
#    must refuse such a crate and accept one that opts in.
# 2. module-contracts.sh matched heading prefixes, so "## Healthiness"
#    counted as "## Health". Headings must match exactly.
# Fixture crates are generated here; none are committed. Both checks ask
# Cargo for the workspace members (second review, F1), so the fixture is a
# workspace Cargo can load: a root manifest, and a target in every crate.
set -euo pipefail
here="$(cd "$(dirname "$0")/.." && pwd)"

tmp="$(mktemp -d)"
cleanup() { [ -n "$tmp" ] && [ -d "$tmp" ] && rm -rf -- "$tmp"; }
trap cleanup EXIT

crate() { # crate <name> <lints-table> <health-heading>
  mkdir -p "$tmp/crates/$1/src"
  printf '[package]\nname = "%s"\nversion = "0.1.0"\nedition = "2024"\n%s\n' "$1" "$2" > "$tmp/crates/$1/Cargo.toml"
  printf 'pub fn f() {}\n' > "$tmp/crates/$1/src/lib.rs"
  printf '# %s\n\n## Owns\n\n## Must not own\n\n%s\n' "$1" "$3" > "$tmp/crates/$1/README.md"
}
printf '[workspace]\nresolver = "3"\nmembers = ["crates/*"]\n\n[workspace.lints.rust]\nunsafe_code = "forbid"\n' > "$tmp/Cargo.toml"

failed=0
expect() { # expect <pass|fail> <label> <command...>
  local want="$1" label="$2"; shift 2
  if "$@" >"$tmp/out" 2>&1; then got=pass; else got=fail; fi
  if [ "$got" = "$want" ]; then echo "CRATES ok: ${label}"; else
    echo "CRATES FAIL: ${label} (expected ${want}, got ${got})"; sed 's/^/  /' "$tmp/out"; failed=1; fi
}

crate good $'[lints]\nworkspace = true' '## Health'
expect pass "a crate that opts into workspace lints is accepted" "$here/crate-policy.sh" "$tmp"
expect pass "exact contract headings are accepted" "$here/module-contracts.sh" "$tmp"

crate nolints '' '## Health'
expect fail "a crate without [lints] workspace = true is refused" "$here/crate-policy.sh" "$tmp"
rm -rf -- "$tmp/crates/nolints"

crate override $'[lints]\nworkspace = true\n\n[lints.clippy]\nunwrap_used = "allow"' '## Health'
expect fail "a crate overriding workspace lints is refused" "$here/crate-policy.sh" "$tmp"
rm -rf -- "$tmp/crates/override"

crate prefix $'[lints]\nworkspace = true' '## Healthiness'
expect fail "'## Healthiness' does not count as '## Health'" "$here/module-contracts.sh" "$tmp"
exit "$failed"
