#!/usr/bin/env bash
# Regression tests (PR #7 review, 2026-10-05):
# 1. The ratchet's suppression count missed `#[cfg_attr(…, allow(…))]`.
# 2. The secret-scan allowlist `(^|/)target/` exempted a directory named
#    `target` at any depth; only the top-level Rust build directory is exempt.
set -euo pipefail
here="$(cd "$(dirname "$0")/.." && pwd)"

tmp="$(mktemp -d)"
cleanup() { [ -n "$tmp" ] && [ -d "$tmp" ] && rm -rf -- "$tmp"; }
trap cleanup EXIT
g() { git -C "$tmp" -c user.email=ratchet-test@example.invalid -c user.name=ratchet-test -c commit.gpgsign=false "$@"; }
failed=0

# 1. ratchet on a fixture repository whose baseline is all zeros
g init -q -b main
mkdir -p "$tmp/scripts/quality" "$tmp/src"
printf '{\n  "ignored_tests": 0,\n  "lint_suppressions": 0,\n  "todo_markers": 0\n}\n' > "$tmp/scripts/quality/ratchet-baseline.json"
printf '#[cfg_attr(test, allow(dead_code))]\nfn f() {}\n' > "$tmp/src/lib.rs"
g add -A; g commit -q -m fixture
if "$here/ratchet.sh" "$tmp" >"$tmp/out" 2>&1; then
  echo "PATHS FAIL: a cfg_attr allow() suppression did not raise the ratchet"; sed 's/^/  /' "$tmp/out"; failed=1
else
  echo "PATHS ok: a cfg_attr allow() suppression raises the ratchet"
fi

# 2. a key under a nested directory named target
mkdir -p "$tmp/docs/target"
python3 - "$tmp/docs/target/notes.txt" <<'PY'
import base64, os, sys, textwrap
body = base64.b64encode(os.urandom(1190)).decode()
armor = "RSA " + "PRIVATE KEY"  # split so this source is not itself key-shaped
open(sys.argv[1], "w").write("-----BEGIN " + armor + "-----\n" + "\n".join(textwrap.wrap(body, 64)) + "\n-----END " + armor + "-----\n")
PY
g add -A; g commit -q -m nested
if "$here/secret-scan.sh" "$tmp" >"$tmp/out" 2>&1; then
  echo "PATHS FAIL: a key under docs/target/ passed the scan"; failed=1
else
  echo "PATHS ok: a key under docs/target/ fails the scan"
fi
exit "$failed"
