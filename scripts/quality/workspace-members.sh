#!/usr/bin/env bash
# Lists crates as Cargo itself resolves them (`cargo metadata --no-deps`:
# offline, and it never writes Cargo.lock), one directory per line, relative
# to the workspace root. Shared by crate-policy.sh and module-contracts.sh so
# both judge exactly the crates Cargo builds, wherever they live (PR #7
# re-verification F1).
#
#   workspace-members.sh [root]            workspace members
#   workspace-members.sh --outside [root]  path dependencies of members that
#                                          are not members themselves (an
#                                          excluded or outside crate); Cargo
#                                          compiles them outside the policy.
#                                          Path patches are checked by
#                                          crate-policy.sh.
#
# A workspace Cargo cannot load fails here: its crates cannot be listed, so
# they cannot be judged (and the build step would refuse it too).
set -euo pipefail
outside=0
if [ "${1:-}" = "--outside" ]; then outside=1; shift; fi
root="$(cd "${1:-$(git rev-parse --show-toplevel)}" && pwd -P)"

meta="$(cargo metadata --no-deps --format-version 1 --offline --manifest-path "$root/Cargo.toml")" || {
  echo "MEMBERS cannot list workspace members: cargo metadata failed for $root/Cargo.toml" >&2
  exit 1
}
printf '%s' "$meta" | python3 -c '
import json, os, sys
root, outside = sys.argv[1], sys.argv[2] == "1"
meta = json.load(sys.stdin)
ids = set(meta["workspace_members"])
members = [p for p in meta["packages"] if p["id"] in ids]
member_dirs = {os.path.normpath(os.path.dirname(p["manifest_path"])) for p in members}
if outside:
    dirs = {
        os.path.normpath(d["path"]) for p in members for d in p["dependencies"]
        if d.get("path") and os.path.normpath(d["path"]) not in member_dirs
    }
else:
    dirs = member_dirs
print("\n".join(sorted(os.path.relpath(d, root) for d in dirs)))
' "$root" "$outside"
