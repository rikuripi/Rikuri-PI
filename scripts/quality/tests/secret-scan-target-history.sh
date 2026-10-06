#!/usr/bin/env bash
# Regression test (verification of the second PR #7 review fix, 2026-10-05):
# the secret-scan allowlist exempted the top-level target/ in every commit
# of history, so a key force-added under target/ and deleted in the next
# commit stayed in history while the scan passed, and so did the Scrap
# Sweep, which sees only HEAD. target/ is never committed legitimately, so
# the scan judges it like any other path.
set -euo pipefail
scan="$(cd "$(dirname "$0")/.." && pwd)/secret-scan.sh"

tmp="$(mktemp -d)"
cleanup() { [ -n "$tmp" ] && [ -d "$tmp" ] && rm -rf -- "$tmp"; }
trap cleanup EXIT
g() { git -C "$tmp" -c user.email=target-test@example.invalid -c user.name=target-test -c commit.gpgsign=false "$@"; }

g init -q -b main
printf 'target/\n' > "$tmp/.gitignore"; g add .gitignore; g commit -q -m ignore
mkdir -p "$tmp/target"
python3 - "$tmp/target/key.txt" <<'PY'
import base64, os, sys, textwrap
body = base64.b64encode(os.urandom(1190)).decode()
armor = "RSA " + "PRIVATE KEY"  # split so this source is not itself key-shaped
open(sys.argv[1], "w").write("-----BEGIN " + armor + "-----\n" + "\n".join(textwrap.wrap(body, 64)) + "\n-----END " + armor + "-----\n")
PY
g add -f target/key.txt; g commit -q -m forced
g rm -q target/key.txt; g commit -q -m removed

if "$scan" "$tmp" >"$tmp/out" 2>&1; then
  echo "TARGET FAIL: a key committed under target/ and later deleted passed the scan"; sed 's/^/  /' "$tmp/out"; exit 1
fi
echo "TARGET ok: a key committed under target/ fails the scan, even after deletion"
