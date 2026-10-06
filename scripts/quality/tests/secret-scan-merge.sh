#!/usr/bin/env bash
# Regression test (PR #7 review, 2026-10-05): a secret written while
# resolving a merge conflict exists only in the merge commit's own diff.
# `git log -p` shows no diff for merge commits unless asked, so the scan
# passed a HEAD whose tree held a key. The scan must fail, and its output
# must show the finding with the value redacted, never the value.
set -euo pipefail
scan="$(cd "$(dirname "$0")/.." && pwd)/secret-scan.sh"

tmp="$(mktemp -d)"
cleanup() { [ -n "$tmp" ] && [ -d "$tmp" ] && rm -rf -- "$tmp"; }
trap cleanup EXIT

g() { git -C "$tmp" -c user.email=merge-test@example.invalid -c user.name=merge-test -c commit.gpgsign=false "$@"; }
g init -q -b main
echo base > "$tmp/config.txt"; g add config.txt; g commit -q -m base
g switch -q -c side
echo side > "$tmp/config.txt"; g commit -q -am side
g switch -q main
echo main > "$tmp/config.txt"; g commit -q -am main
g merge -q side >/dev/null 2>&1 || true   # conflicts by design

body="$(python3 - "$tmp/config.txt" <<'PY'
import base64, os, sys, textwrap
body = base64.b64encode(os.urandom(1190)).decode()
armor = "RSA " + "PRIVATE KEY"  # split so this source is not itself key-shaped
open(sys.argv[1], "w").write("-----BEGIN " + armor + "-----\n" + "\n".join(textwrap.wrap(body, 64)) + "\n-----END " + armor + "-----\n")
print(body[320:350])  # inside one 64-character line
PY
)"
g add config.txt
g commit -q --no-edit

failed=0
if ! g grep -q -F "$body" HEAD -- config.txt; then
  echo "MERGE FAIL: fixture broken, the key is not in HEAD's tree"; exit 1
fi
if out="$("$scan" "$tmp" 2>&1)"; then
  echo "MERGE FAIL: a key introduced in a merge commit passed the scan"; failed=1
else
  echo "MERGE ok: a key introduced in a merge commit fails the scan"
fi
if printf '%s' "$out" | grep -q -F "$body"; then
  echo "MERGE FAIL: the scan printed the key"; failed=1
else
  echo "MERGE ok: the key is not printed"
fi
if printf '%s' "$out" | grep -q 'REDACTED'; then
  echo "MERGE ok: the finding is shown, redacted"
else
  echo "MERGE FAIL: the finding is not shown (expected a REDACTED finding)"; failed=1
fi
exit "$failed"
