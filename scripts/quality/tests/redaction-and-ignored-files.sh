#!/usr/bin/env bash
# Regression tests (PR #7 re-verification, 2026-10-05):
# F4. secret-scan-merge.sh checked a slice of the key that gitleaks never
#     prints even without redaction, so that assertion could not fail.
#     Here every line of the planted key is checked against the output.
# F5. A file force-added under the ignored top-level target/ passed the
#     secret scan (allowlisted) and the Scrap Sweep. Tracked files that
#     .gitignore excludes must fail the sweep.
set -euo pipefail
here="$(cd "$(dirname "$0")/.." && pwd)"

tmp="$(mktemp -d)"
cleanup() { [ -n "$tmp" ] && [ -d "$tmp" ] && rm -rf -- "$tmp"; }
trap cleanup EXIT
g() { git -C "$tmp" -c user.email=redact-test@example.invalid -c user.name=redact-test -c commit.gpgsign=false "$@"; }
failed=0

g init -q -b main
python3 - "$tmp/key.txt" "$tmp/lines" <<'PY'
import base64, os, sys, textwrap
body = base64.b64encode(os.urandom(1190)).decode()
armor = "RSA " + "PRIVATE KEY"  # split so this source is not itself key-shaped
lines = textwrap.wrap(body, 64)
open(sys.argv[1], "w").write("-----BEGIN " + armor + "-----\n" + "\n".join(lines) + "\n-----END " + armor + "-----\n")
open(sys.argv[2], "w").write("\n".join(l[:20] for l in lines) + "\n")
PY
g add key.txt; g commit -q -m key
out="$("$here/secret-scan.sh" "$tmp" 2>&1)" && { echo "REDACT FAIL: the planted key passed the scan"; failed=1; }
if printf '%s' "$out" | grep -q -F -f "$tmp/lines"; then
  echo "REDACT FAIL: part of the key appears in the scan output"; failed=1
else
  echo "REDACT ok: no line of the key appears in the scan output"
fi
printf '%s' "$out" | grep -q 'Secret:.*REDACTED' && echo "REDACT ok: the finding is shown with the secret redacted" \
  || { echo "REDACT FAIL: no redacted finding shown"; failed=1; }

printf 'target/\n' > "$tmp/.gitignore"
mkdir -p "$tmp/target"; echo build > "$tmp/target/artifact.txt"
g add .gitignore; g add -f target/artifact.txt; g commit -q -m forced
if (cd "$tmp" && "$here/scrap-sweep.sh") >"$tmp/out" 2>&1; then
  echo "IGNORED FAIL: a force-added ignored file passed the Scrap Sweep"; sed 's/^/  /' "$tmp/out"; failed=1
else
  echo "IGNORED ok: a force-added ignored file fails the Scrap Sweep"
fi
exit "$failed"
