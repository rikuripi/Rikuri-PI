#!/usr/bin/env bash
# Regression test (PR #7 re-verification, 2026-10-05, F3): rustfmt wraps a
# long `#[cfg_attr(…, allow(…))]` across lines, and the ratchet only counted
# single-line attributes, so the wrapped form, which the format step
# requires, passed uncounted. Wrapped suppressions must count.
set -euo pipefail
here="$(cd "$(dirname "$0")/.." && pwd)"

tmp="$(mktemp -d)"
cleanup() { [ -n "$tmp" ] && [ -d "$tmp" ] && rm -rf -- "$tmp"; }
trap cleanup EXIT
g() { git -C "$tmp" -c user.email=ratchet-test@example.invalid -c user.name=ratchet-test -c commit.gpgsign=false "$@"; }

g init -q -b main
mkdir -p "$tmp/scripts/quality" "$tmp/src"
printf '{\n  "ignored_tests": 0,\n  "lint_suppressions": 0,\n  "todo_markers": 0\n}\n' > "$tmp/scripts/quality/ratchet-baseline.json"
cat > "$tmp/src/lib.rs" <<'RS'
#[cfg_attr(
    all(),
    allow(clippy::unwrap_used, clippy::expect_used, clippy::print_stdout)
)]
pub fn f() {}

#[
    ignore = "wrapped"
]
#[test]
fn t() {}
RS
g add -A; g commit -q -m fixture
if "$here/ratchet.sh" "$tmp" >"$tmp/out" 2>&1; then
  echo "WRAPPED FAIL: wrapped suppressions and ignores did not raise the ratchet"; sed 's/^/  /' "$tmp/out"; exit 1
fi
grep -q 'lint_suppressions: 1 > baseline 0' "$tmp/out" || { echo "WRAPPED FAIL: wrapped cfg_attr(allow) not counted"; sed 's/^/  /' "$tmp/out"; exit 1; }
grep -q 'ignored_tests: 1 > baseline 0' "$tmp/out" || { echo "WRAPPED FAIL: wrapped #[ignore] not counted"; sed 's/^/  /' "$tmp/out"; exit 1; }
echo "WRAPPED ok: wrapped cfg_attr(allow) and #[ignore] raise the ratchet"
