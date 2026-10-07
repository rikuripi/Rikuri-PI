#!/usr/bin/env bash
# Regression test (PR #7 review, 2026-10-05): a secret written while
# resolving a merge conflict exists only in the merge commit's own diff.
# `git log -p` shows no diff for merge commits unless asked, so the scan
# passed a HEAD whose tree held a key. The scan must fail, and its output
# must show the finding with the value redacted, never the value.
# Second review: "never the value" checked one slice of the key that
# gitleaks does not print even unredacted, so it could not fail. Every line
# of the key is checked now (as in redaction-and-ignored-files.sh).
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

body="$(python3 - "$tmp/config.txt" "$tmp/lines" <<'PY'
import base64, os, sys, textwrap
body = base64.b64encode(os.urandom(1190)).decode()
armor = "RSA " + "PRIVATE KEY"  # split so this source is not itself key-shaped
lines = textwrap.wrap(body, 64)
open(sys.argv[1], "w").write("-----BEGIN " + armor + "-----\n" + "\n".join(lines) + "\n-----END " + armor + "-----\n")
open(sys.argv[2], "w").write("\n".join(l[:20] for l in lines) + "\n")
print(body[320:350])  # inside one 64-character line: proves the key is in HEAD
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
# Here-strings, not `printf | grep -q`: under pipefail a SIGPIPE on printf
# would make a leak read as "not printed" (pre-commit SIGPIPE sweep).
if grep -q -F -f "$tmp/lines" <<<"$out"; then
  echo "MERGE FAIL: the scan printed part of the key"; failed=1
else
  echo "MERGE ok: no line of the key is printed"
fi
if grep -q 'REDACTED' <<<"$out"; then
  echo "MERGE ok: the finding is shown, redacted"
else
  echo "MERGE FAIL: the finding is not shown (expected a REDACTED finding)"; failed=1
fi
exit "$failed"
