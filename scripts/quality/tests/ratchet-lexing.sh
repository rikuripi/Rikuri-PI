#!/usr/bin/env bash
# Regression tests (verification of the second PR #7 review fix, 2026-10-05):
# The whole-file ratchet scanner matched cfg_attr parentheses without
# knowing Rust strings or comments, so a ")" inside a string or comment
# ended the attribute early and hid an allow(…) that the line-based ratchet
# had counted (a regression, fmt- and clippy-clean). Two ignore forms were
# never counted: #[cfg_attr(…, ignore)] and a comment inside #[ignore].
# Each case must count exactly; attribute text inside comments and string
# literals is not an attribute and must not count.
set -euo pipefail
here="$(cd "$(dirname "$0")/.." && pwd)"

tmp="$(mktemp -d)"
cleanup() { [ -n "$tmp" ] && [ -d "$tmp" ] && rm -rf -- "$tmp"; }
trap cleanup EXIT
g() { git -C "$tmp/repo" -c user.email=ratchet-test@example.invalid -c user.name=ratchet-test -c commit.gpgsign=false "$@"; }

failed=0
check() { # check <label> <ignored_tests> <lint_suppressions>; Rust source on stdin
  local label="$1" want="$2 $3" got
  rm -rf -- "$tmp/repo"; mkdir -p "$tmp/repo/scripts/quality" "$tmp/repo/src"
  printf '{\n  "ignored_tests": 0,\n  "lint_suppressions": 0,\n  "todo_markers": 0\n}\n' > "$tmp/repo/scripts/quality/ratchet-baseline.json"
  cat > "$tmp/repo/src/lib.rs"
  g init -q -b main; g add -A; g commit -q -m fixture
  if ! "$here/ratchet.sh" --update "$tmp/repo" > "$tmp/out" 2>&1; then
    echo "LEXING FAIL: ${label} (ratchet --update failed)"; sed 's/^/  /' "$tmp/out"; failed=1; return
  fi
  got="$(python3 -c 'import json, sys; c = json.load(open(sys.argv[1])); print(c["ignored_tests"], c["lint_suppressions"])' "$tmp/out")"
  if [ "$got" = "$want" ]; then echo "LEXING ok: ${label}"; else
    echo "LEXING FAIL: ${label} (ignored/suppressions expected ${want}, got ${got})"; failed=1; fi
}

check 'a ")" in a string inside cfg_attr does not hide its allow' 0 1 <<'RS'
#[cfg_attr(all(), doc = ")", allow(clippy::unwrap_used))]
pub fn f() {}
RS
check 'a ")" in a comment inside cfg_attr does not hide its allow' 0 1 <<'RS'
#[cfg_attr(all() /* ) */, allow(clippy::unwrap_used))]
pub fn f() {}
RS
check 'a ")" in a raw string inside cfg_attr does not hide its allow' 0 1 <<'RS'
#[cfg_attr(all(), doc = r#")"#, allow(clippy::unwrap_used))]
pub fn f() {}
RS
check 'cfg_attr(…, ignore) counts as an ignored test' 1 0 <<'RS'
#[cfg_attr(all(), ignore)]
#[test]
fn t() {}
RS
check 'a comment inside #[ignore] still counts' 1 0 <<'RS'
#[/* skip */ ignore]
#[test]
fn t() {}
RS
check 'lifetimes, escaped quotes, char and byte literals do not derail the count' 0 2 <<'RS'
pub fn f<'a>(x: &'a str) -> &'a str { x }
pub const Q: &str = "\"(";
pub const P: u8 = b')';
pub const C: char = '(';
#[allow(dead_code)]
fn g() {}
#[cfg_attr(all(), allow(dead_code))]
fn h() {}
RS
check 'raw-identifier attribute names count (verification round 2)' 1 4 <<'RS'
#![r#allow(dead_code)]
#[r#allow(clippy::unwrap_used)]
pub fn f() {}
#[r#expect(dead_code)]
fn g() {}
#[r#cfg_attr(all(), allow(dead_code))]
fn h() {}
#[r#ignore]
#[test]
fn t() {}
RS
check 'attribute text inside comments and strings does not count' 0 0 <<'RS'
// #[allow(clippy::unwrap_used)] in a line comment
/// Doc text: #[ignore] and #[cfg_attr(test, allow(dead_code))]
/* outer /* nested */ #[allow(dead_code)] still a comment */
pub const S: &str = "#[allow(dead_code)] #[ignore]";
pub const R: &str = r#"#[expect(dead_code)]"#;
RS
exit "$failed"
