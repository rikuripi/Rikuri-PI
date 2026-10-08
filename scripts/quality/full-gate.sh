#!/usr/bin/env bash
# Full repository gate (Stage Plan S00.03). CI runs this same script
# (.github/workflows/ci.yml), so a green CI run means this exact sequence
# passed. It stops at the first red step: a red gate stops the line
# (AGENTS.md, law 2). Setup: README > Running the full gate.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

GITLEAKS_VERSION=8.30.1
CARGO_DENY_VERSION=0.20.2
steps=0

step() {
  local name="$1"; shift
  steps=$((steps + 1))
  echo "── GATE ${name}: $*"
  if "$@"; then
    echo "── GATE ${name}: PASS"
  else
    echo "── GATE ${name}: FAIL"
    echo "RESULT: FAIL (${name})"
    exit 1
  fi
}

tool_versions() {
  local ok=0
  command -v gitleaks >/dev/null || { echo "gitleaks not installed"; ok=1; }
  command -v cargo-deny >/dev/null || { echo "cargo-deny not installed"; ok=1; }
  command -v python3 >/dev/null || { echo "python3 not installed"; ok=1; }
  [ "$ok" = 0 ] || return 1
  python3 -c 'import tomllib' 2>/dev/null || { echo "python3 is older than 3.11 (no tomllib)"; return 1; }
  local gl cd
  gl="$(gitleaks version)"
  cd="$(cargo deny --version | awk '{print $2}')"
  echo "rustc: $(rustc --version)   gitleaks: ${gl}   cargo-deny: ${cd}"
  [ "$gl" = "$GITLEAKS_VERSION" ] || { echo "gitleaks ${gl} != pinned ${GITLEAKS_VERSION}"; ok=1; }
  [ "$cd" = "$CARGO_DENY_VERSION" ] || { echo "cargo-deny ${cd} != pinned ${CARGO_DENY_VERSION}"; ok=1; }
  return "$ok"
}

run_selftests() { # the gate's own regression tests (scripts/quality/tests)
  local t rc=0
  for t in scripts/quality/tests/*.sh; do
    echo "self-test ${t}"
    "$t" || rc=1
  done
  return "$rc"
}

# Lints with warnings as errors, then again for rikurid's test-only
# crash-injection feature, which no other step compiles (owner decision
# 2026-10-08) so it cannot rot unnoticed.
run_clippy() {
  cargo clippy --workspace --all-targets --locked -- -D warnings &&
    cargo clippy -p rikurid --all-targets --features crash-injection --locked -- -D warnings
}

step tools tool_versions
step gate-selftests run_selftests
step guard python3 -m unittest discover -s .claude/hooks -p 'test_*.py' -q
step format cargo fmt --all --check
step build cargo build --workspace --all-targets --locked
step clippy run_clippy
step test cargo test --workspace --locked
step dependencies cargo deny --locked check
step secrets scripts/quality/secret-scan.sh
step ratchet scripts/quality/ratchet.sh
step contracts scripts/quality/module-contracts.sh
step crate-policy scripts/quality/crate-policy.sh
step scrap scripts/quality/scrap-sweep.sh

echo "RESULT: PASS (${steps} steps)"
