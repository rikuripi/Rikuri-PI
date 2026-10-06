#!/usr/bin/env bash
# Regression tests (verification of the second PR #7 review fix, 2026-10-05):
# F1. A crate excluded from the workspace and used as a path dependency is
#     compiled into the product, yet escaped the lint policy, the contract
#     check and `clippy --workspace`. Every local crate Cargo builds must be
#     a workspace member.
# F2. A Cargo config could still switch clippy off without any lint flag:
#     an [alias] that shadows `clippy`, a rustc wrapper that appends
#     --cap-lints, or [env] CLIPPY_ARGS with force (all shown to let clippy
#     pass code it refuses). The environment forms likewise: RUSTC_WRAPPER,
#     CARGO_ALIAS_CLIPPY, and CARGO_BUILD_RUSTDOCFLAGS that allow lints.
# Legitimate configuration still passes: a path dependency on another
# member, an unrelated alias or [env] variable, an empty RUSTC_WRAPPER
# (Cargo reads empty as unset). Fixture workspaces are generated here.
set -euo pipefail
here="$(cd "$(dirname "$0")/.." && pwd)"

tmp="$(mktemp -d)"
cleanup() { [ -n "$tmp" ] && [ -d "$tmp" ] && rm -rf -- "$tmp"; }
trap cleanup EXIT

workspace() { # workspace [<extra [workspace] keys>]
  printf '[workspace]\nresolver = "3"\nmembers = ["crates/*"]\n%s\n\n[workspace.lints.rust]\nunsafe_code = "forbid"\n' "${1:-}" > "$tmp/Cargo.toml"
}
member() { # member <dir> <lints-table> [<dependencies-table>]
  mkdir -p "$tmp/$1/src"
  printf '[package]\nname = "%s"\nversion = "0.1.0"\nedition = "2024"\n%s\n%s\n' "$(basename "$1")" "$2" "${3:-}" > "$tmp/$1/Cargo.toml"
  printf 'pub fn f() {}\n' > "$tmp/$1/src/lib.rs"
  printf '# x\n\n## Owns\n\n## Must not own\n\n## Health / last-success signal\n' > "$tmp/$1/README.md"
}
config() { mkdir -p "$tmp/.cargo"; printf '%s\n' "$1" > "$tmp/.cargo/config.toml"; }
lints=$'[lints]\nworkspace = true'

failed=0
expect() { # expect <pass|fail> <label> <must-mention> <command...>
  local want="$1" label="$2" mention="$3"; shift 3
  if "$@" >"$tmp/out" 2>&1; then got=pass; else got=fail; fi
  if [ "$got" != "$want" ]; then
    echo "BYPASS FAIL: ${label} (expected ${want}, got ${got})"; sed 's/^/  /' "$tmp/out"; failed=1
  elif [ -n "$mention" ] && ! grep -q -- "$mention" "$tmp/out"; then
    echo "BYPASS FAIL: ${label} (output does not name ${mention})"; sed 's/^/  /' "$tmp/out"; failed=1
  else
    echo "BYPASS ok: ${label}"
  fi
}
policy="$here/crate-policy.sh"

workspace
member crates/good "$lints"
expect pass "a clean workspace passes" "" "$policy" "$tmp"

# F1
member crates/lib2 "$lints"
member crates/good "$lints" $'[dependencies]\nlib2 = { path = "../lib2" }'
expect pass "a path dependency on another member passes" "" "$policy" "$tmp"
rm -rf -- "$tmp/crates/lib2" "$tmp/Cargo.lock"

workspace 'exclude = ["crates/sneaky"]'
member crates/sneaky ''
member crates/good "$lints" $'[dependencies]\nsneaky = { path = "../sneaky" }'
expect fail "an excluded crate used as a path dependency is refused" "crates/sneaky" "$policy" "$tmp"
rm -rf -- "$tmp/crates/sneaky" "$tmp/Cargo.lock"
workspace
member crates/good "$lints"

# F2, Cargo config
config $'[alias]\nclippy = ["test", "--no-run"]'
expect fail "a config [alias] that shadows clippy is refused" "alias" "$policy" "$tmp"
config $'[alias]\nxtask = "run -p xtask"'
expect pass "an unrelated alias passes" "" "$policy" "$tmp"
config $'[build]\nrustc-wrapper = "tools/wrap.sh"'
expect fail "a config rustc-wrapper is refused" "rustc-wrapper" "$policy" "$tmp"
config $'[env]\nCLIPPY_ARGS = { value = "--cap-lints=allow", force = true }'
expect fail "[env] CLIPPY_ARGS is refused" "CLIPPY_ARGS" "$policy" "$tmp"
config $'[env]\nAPP_MODE = "test"'
expect pass "an unrelated [env] variable passes" "" "$policy" "$tmp"
rm -rf -- "$tmp/.cargo"

# F2, environment
expect fail "RUSTC_WRAPPER is refused" "RUSTC_WRAPPER" env RUSTC_WRAPPER="$tmp/wrap.sh" "$policy" "$tmp"
expect pass "an empty RUSTC_WRAPPER passes" "" env RUSTC_WRAPPER= "$policy" "$tmp"
expect fail "CARGO_ALIAS_CLIPPY is refused" "CARGO_ALIAS_CLIPPY" env CARGO_ALIAS_CLIPPY="test --no-run" "$policy" "$tmp"
expect fail "CARGO_BUILD_RUSTDOCFLAGS that allow lints are refused" "CARGO_BUILD_RUSTDOCFLAGS" env CARGO_BUILD_RUSTDOCFLAGS="-A warnings" "$policy" "$tmp"
exit "$failed"
