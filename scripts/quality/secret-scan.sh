#!/usr/bin/env bash
# Secret scan of committed history (Stage Plan S00.02, T00.02), shared by
# full-gate.sh and its regression test so both run the same command.
# Usage: secret-scan.sh [repo-dir]   (default: the current repository)
# Values are redacted in all output.
#
# Scope: the history of the checked-out commit (HEAD), and nothing else.
# Without --log-opts, gitleaks reads every ref in the clone (`git log --all`),
# so CI's full fetch made one branch's secret fail every other branch's PR,
# and local results depended on which remote branches happened to be
# fetched (found by the T00.01 drill, 2026-10-05; regression test in
# scripts/quality/tests/).
set -euo pipefail
repo="${1:-$(git rev-parse --show-toplevel)}"
config="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)/.gitleaks.toml"
exec gitleaks git "$repo" --config "$config" --redact --no-banner --exit-code 1 \
  --log-opts="--full-history HEAD"
