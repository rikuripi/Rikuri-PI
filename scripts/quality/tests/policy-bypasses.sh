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
# Verification round 2 found more: rustflags that read an @argfile or
# --force-warn lints (-D warnings does not promote them), a member build
# script that sets CLIPPY_ARGS through rustc-env, and a test `runner` that
# makes failing tests pass. The rest of the cases below pin refusals first
# checked by hand: wrapper and tool keys, path patches, every dependency
# kind, and a listing failure that still names the configuration.
# Legitimate configuration still passes: a path dependency on another
# member, a patch to a member, an unrelated alias, [env] variable or build
# script output, an empty RUSTC_WRAPPER (Cargo reads empty as unset).
# Round 3: the rustc-env check must not refuse names no compiler reads
# (RUSTC_VERSION) or judge untracked files, which CI never sees.
# Threat model (owner, 2026-10-05): a tripwire for accidental or careless
# weakening, not for deliberate subversion by someone with commit access.
# Fixture workspaces are generated here.
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

# Round 2: rustflags that read flags from a file or force-warn lints
mkdir -p "$tmp/tools"; printf -- '--cap-lints\nallow\n' > "$tmp/tools/lints.txt"
config $'[build]\nrustflags = ["@tools/lints.txt"]'
expect fail "rustflags reading an @argfile are refused" "@tools/lints.txt" "$policy" "$tmp"
config $'[build]\nrustflags = ["--force-warn", "clippy::unwrap_used"]'
expect fail "rustflags with --force-warn are refused" "--force-warn" "$policy" "$tmp"
rm -rf -- "$tmp/.cargo"
expect fail "RUSTFLAGS reading an @argfile are refused" "RUSTFLAGS" env RUSTFLAGS="@$tmp/tools/lints.txt" "$policy" "$tmp"
expect fail "CARGO_BUILD_RUSTFLAGS with --force-warn are refused" "CARGO_BUILD_RUSTFLAGS" env CARGO_BUILD_RUSTFLAGS="--force-warn clippy::all" "$policy" "$tmp"
rm -rf -- "$tmp/tools"

# Round 2: a member build script that sets clippy's or rustc's environment
build_script() { # build_script <rustc-env assignment>
  printf 'fn main() {\n    let _ = std::io::Write::write_all(&mut std::io::stdout(), b"cargo::rustc-env=%s\\n");\n}\n' "$1" > "$tmp/crates/good/build.rs"
}
build_script 'CLIPPY_ARGS=--cap-lints=allow__CLIPPY_HACKERY__'
expect fail "a build script that sets CLIPPY_ARGS is refused" "crates/good/build.rs" "$policy" "$tmp"
build_script 'RUSTC_BOOTSTRAP=1'
expect fail "a build script that sets RUSTC_BOOTSTRAP is refused" "crates/good/build.rs" "$policy" "$tmp"
build_script 'APP_BUILD=1'
expect pass "a build script that sets an unrelated variable passes" "" "$policy" "$tmp"
rm -f -- "$tmp/crates/good/build.rs"

# Round 2: a test runner replaces how `cargo test` runs the tests
config $'[target.\'cfg(all())\']\nrunner = "true"'
expect fail "a config test runner is refused" "runner" "$policy" "$tmp"
rm -rf -- "$tmp/.cargo"
expect fail "CARGO_TARGET_<triple>_RUNNER is refused" "CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER" \
  env CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER=true "$policy" "$tmp"

# Refusals first checked by hand: compiler keys, [env], aliases, environment
for key in rustc rustc-workspace-wrapper rustdoc; do
  config "$(printf '[build]\n%s = "tools/x"' "$key")"
  expect fail "a config build.${key} is refused" "build.${key}" "$policy" "$tmp"
done
config $'[env]\nRUSTC_BOOTSTRAP = "1"'
expect fail "[env] RUSTC_BOOTSTRAP is refused" "RUSTC_BOOTSTRAP" "$policy" "$tmp"
config $'[alias]\nfmt = "check"'
expect fail "a config [alias] that shadows fmt is refused" "alias" "$policy" "$tmp"
rm -rf -- "$tmp/.cargo"
for var in RUSTC RUSTDOC RUSTC_WORKSPACE_WRAPPER CARGO_BUILD_RUSTC CARGO_BUILD_RUSTC_WRAPPER \
    CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER CARGO_BUILD_RUSTDOC CLIPPY_ARGS CARGO_ALIAS_FMT CARGO_ALIAS_DENY; do
  expect fail "${var} is refused" "${var} " env "${var}=$tmp/x" "$policy" "$tmp"
done
expect fail "CARGO_ENCODED_RUSTDOCFLAGS that allow lints are refused" "CARGO_ENCODED_RUSTDOCFLAGS" \
  env CARGO_ENCODED_RUSTDOCFLAGS=$'-A\x1fwarnings' "$policy" "$tmp"

# Every dependency kind that can name an excluded crate
member crates/sneaky ''
workspace $'exclude = ["crates/sneaky"]\n\n[workspace.dependencies]\nsneaky = { path = "crates/sneaky" }'
member crates/good "$lints" $'[dependencies]\nsneaky.workspace = true'
expect fail "an excluded crate inherited from [workspace.dependencies] is refused" "crates/sneaky" "$policy" "$tmp"
workspace 'exclude = ["crates/sneaky"]'
for table in dev-dependencies build-dependencies "target.'cfg(unix)'.dependencies"; do
  member crates/good "$lints" "$(printf '[%s]\nsneaky = { path = "../sneaky" }' "$table")"
  expect fail "an excluded crate in [${table}] is refused" "crates/sneaky" "$policy" "$tmp"
done
member crates/good "$lints" $'[dependencies]\nrenamed = { package = "sneaky", path = "../sneaky" }'
expect fail "an excluded crate under another name is refused" "crates/sneaky" "$policy" "$tmp"
rm -rf -- "$tmp/crates/sneaky" "$tmp/Cargo.lock"
member crates/good "$lints"

# Path patches and overrides
mkdir -p "$tmp/vendor/foo/src"
printf '[package]\nname = "foo"\nversion = "0.1.0"\nedition = "2024"\n' > "$tmp/vendor/foo/Cargo.toml"
printf 'pub fn f() {}\n' > "$tmp/vendor/foo/src/lib.rs"
workspace $'\n[patch.crates-io]\nfoo = { path = "vendor/foo" }'
expect fail "a [patch] to a crate outside the workspace is refused" "vendor/foo" "$policy" "$tmp"
workspace $'\n[patch.crates-io]\nfoo = { path = "crates/good" }'
expect pass "a [patch] to a member passes" "" "$policy" "$tmp"
workspace $'\n[replace]\n"foo:0.1.0" = { path = "vendor/foo" }'
expect fail "a [replace] to a crate outside the workspace is refused" "vendor/foo" "$policy" "$tmp"
workspace
config $'[patch.crates-io]\nfoo = { path = "vendor/foo" }'
expect fail "a config [patch] to a crate outside the workspace is refused" "vendor/foo" "$policy" "$tmp"
config $'paths = ["vendor/foo"]'
expect fail "a config paths override is refused" "paths override" "$policy" "$tmp"
rm -rf -- "$tmp/.cargo" "$tmp/vendor"

# A workspace Cargo cannot load still has its configuration named
rm -f -- "$tmp/crates/good/src/lib.rs"
config $'[alias]\nclippy = ["test", "--no-run"]'
expect fail "an unloadable workspace still names a weakening config" "alias" "$policy" "$tmp"
rm -rf -- "$tmp/.cargo"
member crates/good "$lints"

# Round 3: the rustc-env check refused a build script setting RUSTC_VERSION
# (read by no compiler; the usual way to embed the toolchain version)
build_script 'RUSTC_VERSION=1.98.1'
expect pass "a build script that sets RUSTC_VERSION passes" "" "$policy" "$tmp"
rm -f -- "$tmp/crates/good/build.rs"

# Round 3: refusals first checked by hand in round 1, now pinned
config $'[target.x86_64-unknown-linux-gnu]\nrustflags = ["-A", "warnings"]'
expect fail "[target.<triple>] rustflags that allow lints are refused" "weaken lints" "$policy" "$tmp"
config $'[build]\nrustdocflags = ["--cap-lints", "allow"]'
expect fail "build.rustdocflags that cap lints are refused" "weaken lints" "$policy" "$tmp"
config 'not [valid toml'
expect fail "an unreadable Cargo config is refused" "cannot read Cargo config" "$policy" "$tmp"
rm -rf -- "$tmp/.cargo"
expect fail "RUSTDOCFLAGS that allow lints are refused" "RUSTDOCFLAGS " env RUSTDOCFLAGS="-A warnings" "$policy" "$tmp"
expect fail "CARGO_TARGET_<triple>_RUSTFLAGS that cap lints are refused" "CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS" \
  env CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS="--cap-lints allow" "$policy" "$tmp"
expect fail "CARGO_ENCODED_RUSTFLAGS that allow lints are refused" "CARGO_ENCODED_RUSTFLAGS" \
  env CARGO_ENCODED_RUSTFLAGS=$'-A\x1fwarnings' "$policy" "$tmp"
mkdir -p "$tmp/home"; printf '[build]\nrustflags = ["--cap-lints", "allow"]\n' > "$tmp/home/config.toml"
expect fail "a CARGO_HOME config that caps lints is refused" "home/config.toml" env CARGO_HOME="$tmp/home" "$policy" "$tmp"
rm -rf -- "$tmp/home"
mkdir -p "$tmp/outer/.cargo" "$tmp/outer/ws/crates/a/src"
printf '[build]\nrustflags = ["-A", "warnings"]\n' > "$tmp/outer/.cargo/config.toml"
printf '[workspace]\nresolver = "3"\nmembers = ["crates/*"]\n\n[workspace.lints.rust]\nunsafe_code = "forbid"\n' > "$tmp/outer/ws/Cargo.toml"
printf '[package]\nname = "a"\nversion = "0.1.0"\nedition = "2024"\n\n[lints]\nworkspace = true\n' > "$tmp/outer/ws/crates/a/Cargo.toml"
printf 'pub fn f() {}\n' > "$tmp/outer/ws/crates/a/src/lib.rs"
expect fail "a parent directory's Cargo config that allows lints is refused" "outer/.cargo/config.toml" "$policy" "$tmp/outer/ws"
rm -rf -- "$tmp/outer"

# Round 3: in a git work tree only tracked sources are judged, as CI sees
# them (the scan read untracked files, so local and CI results differed)
gt() { git -C "$tmp" -c user.email=policy-test@example.invalid -c user.name=policy-test -c commit.gpgsign=false "$@"; }
gt init -q -b main; gt add -A; gt commit -q -m fixture
build_script 'CLIPPY_ARGS=--cap-lints=allow__CLIPPY_HACKERY__'
expect pass "an untracked build script is not judged (CI does not see it)" "" "$policy" "$tmp"
gt add crates/good/build.rs; gt commit -q -m tracked
expect fail "a tracked build script that sets CLIPPY_ARGS is refused" "crates/good/build.rs" "$policy" "$tmp"
exit "$failed"
