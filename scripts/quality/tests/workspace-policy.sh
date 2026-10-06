#!/usr/bin/env bash
# Regression tests (PR #7 re-verification, 2026-10-05):
# F1. crate-policy.sh and module-contracts.sh only looked in crates/ and
#     apps/, so a workspace member elsewhere (tools/, adapters/) escaped
#     the lint policy and the contract check. Both must judge every
#     workspace member Cargo knows about.
# F2. A `.cargo/config.toml` or RUSTFLAGS carrying `--cap-lints allow`
#     switched every lint off while all checks passed. The policy must
#     refuse build flags that cap or allow lints.
# Fixture workspaces are generated here; none are committed.
set -euo pipefail
here="$(cd "$(dirname "$0")/.." && pwd)"

tmp="$(mktemp -d)"
cleanup() { [ -n "$tmp" ] && [ -d "$tmp" ] && rm -rf -- "$tmp"; }
trap cleanup EXIT

member() { # member <dir> <lints-table> <write-readme:yes|no>
  mkdir -p "$tmp/$1/src"
  printf '[package]\nname = "%s"\nversion = "0.1.0"\nedition = "2024"\n%s\n' "$(basename "$1")" "$2" > "$tmp/$1/Cargo.toml"
  printf 'pub fn f() {}\n' > "$tmp/$1/src/lib.rs"
  if [ "$3" = yes ]; then
    printf '# x\n\n## Owns\n\n## Must not own\n\n## Health / last-success signal\n' > "$tmp/$1/README.md"
  fi
}
printf '[workspace]\nresolver = "3"\nmembers = ["crates/*", "tools/*"]\n\n[workspace.lints.rust]\nunsafe_code = "forbid"\n' > "$tmp/Cargo.toml"
member crates/good $'[lints]\nworkspace = true' yes

failed=0
expect() { # expect <pass|fail> <label> <must-mention> <command...>
  local want="$1" label="$2" mention="$3"; shift 3
  if "$@" >"$tmp/out" 2>&1; then got=pass; else got=fail; fi
  if [ "$got" != "$want" ]; then
    echo "WORKSPACE FAIL: ${label} (expected ${want}, got ${got})"; sed 's/^/  /' "$tmp/out"; failed=1
  elif [ -n "$mention" ] && ! grep -q -- "$mention" "$tmp/out"; then
    echo "WORKSPACE FAIL: ${label} (output does not name ${mention})"; sed 's/^/  /' "$tmp/out"; failed=1
  else
    echo "WORKSPACE ok: ${label}"
  fi
}

expect pass "a clean workspace passes the crate policy" "" "$here/crate-policy.sh" "$tmp"
expect pass "a clean workspace passes the contract check" "" "$here/module-contracts.sh" "$tmp"

member tools/evil '' no
expect fail "a member outside crates/ and apps/ without workspace lints is refused" "tools/evil" "$here/crate-policy.sh" "$tmp"
expect fail "a member outside crates/ and apps/ without a contract is refused" "tools/evil" "$here/module-contracts.sh" "$tmp"
rm -rf -- "$tmp/tools/evil"

mkdir -p "$tmp/.cargo"
printf '[build]\nrustflags = ["--cap-lints", "allow"]\n' > "$tmp/.cargo/config.toml"
expect fail "a .cargo/config.toml that caps lints is refused" ".cargo/config.toml" "$here/crate-policy.sh" "$tmp"
rm -rf -- "$tmp/.cargo"

expect fail "RUSTFLAGS that cap lints are refused" "RUSTFLAGS" env RUSTFLAGS="--cap-lints allow" "$here/crate-policy.sh" "$tmp"
expect fail "RUSTFLAGS that allow a lint are refused" "RUSTFLAGS" env RUSTFLAGS="-A clippy::unwrap_used" "$here/crate-policy.sh" "$tmp"
exit "$failed"
