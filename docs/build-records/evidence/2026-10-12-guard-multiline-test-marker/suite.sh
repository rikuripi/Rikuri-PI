#!/usr/bin/env bash
# suite.sh <python> <repo root>: the guard suite as AGENTS.md runs it; exits as unittest does.
set -o pipefail
cd "$2" && "$1" --version && "$1" -m unittest discover -s .claude/hooks -p 'test_*.py' 2>&1 | tail -60
