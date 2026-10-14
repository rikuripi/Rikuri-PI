#!/usr/bin/env bash
# Scrap Sweep (Engineering Standard, Scrap Taxonomy; Stage Plan S00.03).
# Fails on mechanically certain residue in tracked files. Lists untracked
# files and stashes for a human/agent disposition without failing: age or an
# unknown purpose is not deletion authority (Engineering Standard > Scrap Taxonomy).
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
failed=0

report() { # report <label> <lines>
  if [ -n "$2" ]; then echo "SCRAP $1:"; echo "$2" | sed 's/^/  /'; failed=1; fi
}

report "stray backup/editor files tracked" \
  "$(git ls-files | grep -E '(\.bak|\.old|\.orig|\.rej|\.swp|\.swo|~|\.DS_Store)$' || true)"
report "debug macros in Rust" "$(git grep -nE '\bdbg!\(' -- '*.rs' || true)"
report "skipped tests without a reason (use #[ignore = \"why\"])" \
  "$(git grep -nE '#\[ignore\]' -- '*.rs' || true)"
report "merge conflict markers" "$(git grep -nE '^(<<<<<<<|>>>>>>>)( |$)' || true)"
# Tracked files that .gitignore excludes were force-added: build output,
# local state or secrets that bypassed the ignore rules. Only the
# repository's .gitignore files count (not .git/info/exclude or a personal
# excludes file), so the result is the same on every machine. This sees
# HEAD only; the secret scan covers what was force-added and later deleted,
# except under paths gitleaks' built-in allowlist skips (accepted gap, see
# .gitleaks.toml).
report "tracked files that .gitignore excludes (force-added)" \
  "$(git ls-files --cached --ignored --exclude-per-directory=.gitignore)"

untracked="$(git ls-files --others --exclude-standard)"
[ -z "$untracked" ] || { echo "SCRAP untracked files (need a disposition; not failed):"; echo "$untracked" | sed 's/^/  /'; }
stashes="$(git stash list | wc -l | tr -d ' ')"
[ "$stashes" = 0 ] || echo "SCRAP ${stashes} stash(es) (need a disposition; not failed)"

[ "$failed" = 0 ] && echo "SCRAP clean"
exit "$failed"
