#!/usr/bin/env bash
# Regression test (2026-10-10): every other gate self-test also passes with
# TMPDIR inside a git work tree. policy-bypasses.sh built its fixtures in a
# bare `mktemp -d`; with TMPDIR under a repository's ignored target/ (a
# verifier's run), git found the enclosing work tree, crate-policy.sh judged
# only tracked sources, the fixture had none, and two refusals passed. A
# self-test must not depend on where TMPDIR points. The gate runs this file
# once with the caller's TMPDIR, like every self-test, and it reruns the
# others here.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
self="$(basename "$0")"

wt="$(mktemp -d)"
cleanup() { [ -n "$wt" ] && [ -d "$wt" ] && rm -rf -- "$wt"; }
trap cleanup EXIT

# A work tree of its own, with the self-tests' TMPDIR in its ignored target/
git -C "$wt" init -q -b main
printf 'target/\n' > "$wt/.gitignore"
inner="$wt/target/tmp"
mkdir -p "$inner"
# The reruns prove nothing unless git sees that TMPDIR inside the work tree
if [ "$(git -C "$inner" rev-parse --is-inside-work-tree 2>/dev/null)" != true ] ||
   ! git -C "$wt" check-ignore -q target/tmp; then
  echo "TMPDIR-IN-GIT FAIL: $inner is not an ignored directory of a git work tree; nothing was checked"
  exit 1
fi
# A rerun not given that TMPDIR fails instead of passing where it always
# did: the TMPDIR it inherits names a directory that does not exist, so its
# `mktemp -d` fails
export TMPDIR="$wt/no-such-dir"

failed=0 ran=0
for t in "$here"/*.sh; do
  [ "$(basename "$t")" != "$self" ] || continue
  ran=$((ran + 1))
  if TMPDIR="$inner" "$t" > "$wt/out" 2>&1; then
    echo "TMPDIR-IN-GIT ok: $(basename "$t")"
  else
    echo "TMPDIR-IN-GIT FAIL: $(basename "$t") fails with TMPDIR inside a git work tree"
    sed 's/^/  /' "$wt/out"
    failed=1
  fi
done
[ "$ran" -gt 0 ] || { echo "TMPDIR-IN-GIT FAIL: no other self-tests next to $self"; failed=1; }
exit "$failed"
