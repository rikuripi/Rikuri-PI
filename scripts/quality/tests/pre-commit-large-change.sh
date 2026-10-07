#!/usr/bin/env bash
# Regression test (pre-merge verification of PR #7, 2026-10-06): the
# pre-commit hook piped `git diff --cached --name-only` into `grep -q` under
# pipefail. On a large staged change grep exits at its first match, git dies
# of SIGPIPE, and the hook skipped `cargo fmt --check` and `cargo check`
# without a word. A misformatted Rust file must be refused whether it is
# staged alone or among thousands of other files.
set -euo pipefail
here="$(cd "$(dirname "$0")/.." && pwd)"
repo_root="$(cd "$here/../.." && pwd)"

tmp="$(mktemp -d)"
cleanup() { [ -n "$tmp" ] && [ -d "$tmp" ] && rm -rf -- "$tmp"; }
trap cleanup EXIT

fixture() { # fixture <dir>: a workspace whose one crate is misformatted
  mkdir -p "$1/crates/a/src"
  git -C "$1" init -q -b main
  cp "$repo_root/.gitleaks.toml" "$repo_root/rust-toolchain.toml" "$1/"
  printf '[workspace]\nresolver = "3"\nmembers = ["crates/*"]\n' > "$1/Cargo.toml"
  printf '[package]\nname = "a"\nversion = "0.1.0"\nedition = "2024"\n' > "$1/crates/a/Cargo.toml"
  printf 'pub fn f( ) -> u8 {1}\n' > "$1/crates/a/src/lib.rs"
}

failed=0
check() { # check <label> <repo> [VAR=value ...]
  local label="$1" repo="$2"; shift 2
  if (cd "$repo" && env "$@" "$here/hooks/pre-commit") > "$tmp/out" 2>&1; then
    echo "HOOK FAIL: $label (the hook passed a misformatted Rust file)"; tail -5 "$tmp/out" | sed 's/^/  /'; failed=1
  elif grep -q "formatting differs" "$tmp/out"; then
    echo "HOOK ok: $label"
  else
    echo "HOOK FAIL: $label (refused, but not for formatting)"; tail -5 "$tmp/out" | sed 's/^/  /'; failed=1
  fi
}

fixture "$tmp/small"
git -C "$tmp/small" add -A
check "a misformatted Rust file staged alone is refused" "$tmp/small"

# About 400 KB of staged names, far beyond a pipe buffer (64 KB on Linux).
fixture "$tmp/large"
mkdir -p "$tmp/large/z"
python3 - "$tmp/large/z" <<'PY'
import os, sys
for i in range(4000):
    open(os.path.join(sys.argv[1], f"{i:05d}-" + "n" * 90 + ".txt"), "w").close()
PY
git -C "$tmp/large" add -A
check "a misformatted Rust file among 4,000 staged files is refused" "$tmp/large"

# Verifier on PR #17: git's pathspec environment variables change what a
# pathspec matches, so a hook that selects Rust paths by pathspec could
# skip its checks silently. Only a nested Rust file is staged here, so no
# top-level Cargo.toml can trigger the checks by itself.
fixture "$tmp/nested"
printf 'pub fn f() -> u8 {\n    1\n}\n' > "$tmp/nested/crates/a/src/lib.rs"
git -C "$tmp/nested" add -A
git -C "$tmp/nested" -c user.email=hook-test@example.invalid -c user.name=hook-test -c commit.gpgsign=false commit -q -m base
printf 'pub fn f( ) -> u8 {1}\n' > "$tmp/nested/crates/a/src/lib.rs"
git -C "$tmp/nested" add crates/a/src/lib.rs
check "a staged nested Rust file is refused" "$tmp/nested"
check "… with GIT_LITERAL_PATHSPECS=1 set" "$tmp/nested" GIT_LITERAL_PATHSPECS=1
check "… with GIT_GLOB_PATHSPECS=1 set" "$tmp/nested" GIT_GLOB_PATHSPECS=1
check "… with GIT_NOGLOB_PATHSPECS=1 set" "$tmp/nested" GIT_NOGLOB_PATHSPECS=1
exit "$failed"
