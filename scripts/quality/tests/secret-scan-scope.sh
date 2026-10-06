#!/usr/bin/env bash
# Regression test for the secret scan's scope (defect found by the T00.01
# drill, 2026-10-05: CI's full fetch brought in another branch, and the scan
# read every ref, so an unrelated PR went red). The scan must judge only the
# checked-out history:
#   1. a clean HEAD passes even when another branch holds a secret;
#   2. a HEAD whose own history holds a secret fails.
# The fake key is random bytes generated here; nothing secret-shaped is
# committed to Rikuri.
set -euo pipefail
scan="$(cd "$(dirname "$0")/.." && pwd)/secret-scan.sh"

tmp="$(mktemp -d)"
cleanup() { [ -n "$tmp" ] && [ -d "$tmp" ] && rm -rf -- "$tmp"; }
trap cleanup EXIT

git -C "$tmp" init -q -b main
git -C "$tmp" config user.email scope-test@example.invalid
git -C "$tmp" config user.name scope-test
echo "clean" > "$tmp/readme.txt"
git -C "$tmp" add readme.txt
git -C "$tmp" commit -q -m clean

git -C "$tmp" switch -q -c side
python3 - "$tmp/planted.txt" <<'PY'
import base64, os, sys, textwrap
body = base64.b64encode(os.urandom(1190)).decode()
armor = "RSA " + "PRIVATE KEY"  # split so this source is not itself key-shaped
open(sys.argv[1], "w").write("-----BEGIN " + armor + "-----\n" + "\n".join(textwrap.wrap(body, 64)) + "\n-----END " + armor + "-----\n")
PY
git -C "$tmp" add planted.txt
git -C "$tmp" commit -q -m planted

failed=0
git -C "$tmp" switch -q main
if "$scan" "$tmp" >/dev/null 2>&1; then
  echo "SCOPE ok: clean HEAD passes while another branch holds a secret"
else
  echo "SCOPE FAIL: clean HEAD failed because another branch holds a secret"; failed=1
fi
git -C "$tmp" switch -q side
if "$scan" "$tmp" >/dev/null 2>&1; then
  echo "SCOPE FAIL: a secret in HEAD's own history passed"; failed=1
else
  echo "SCOPE ok: a secret in HEAD's own history fails"
fi
exit "$failed"
