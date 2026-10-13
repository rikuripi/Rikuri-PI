#!/usr/bin/env bash
# hook_e2e.sh <work dir> <old hooks dir> <new hooks dir>: the real hook, in a
# scratch repository holding a file whose tests sit under a marker written over
# several lines, with a global fix-lock held. Three setups: old guard + old
# config (before), new guard + old config (tripwire only), new guard + new config.
set -u
work=$1; old=$2; new=$3
unset CLAUDE_CODE_SESSION_ID
rm -rf "$work"; mkdir -p "$work"
src='pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(all(
    test,
    feature = "extra",
))]
mod tests {
    use super::*;

    #[test]
    fn adds() {
        assert_eq!(add(2, 2), 4);
    }
}
'
ev() {  # ev <root> <old_string> <new_string>
  python3 -I -c 'import json,sys; print(json.dumps({"hook_event_name":"PreToolUse","session_id":"e2e","cwd":sys.argv[1],"tool_name":"Edit","tool_input":{"file_path":"src/lib.rs","old_string":sys.argv[2],"new_string":sys.argv[3]}}))' "$@"
}
check() {  # check <root> <label> <old_string> <new_string>
  local d
  d=$(ev "$1" "$3" "$4" | (cd "$1" && python3 .claude/hooks/guard.py) | python3 -I -c 'import json,sys; s=sys.stdin.read().strip(); print(json.loads(s)["hookSpecificOutput"]["permissionDecision"] if s else "allow (no output)")')
  echo "  $2: $d"
}
for setup in "before:$old:$old" "tripwire-only:$new:$old" "after:$new:$new"; do
  IFS=: read -r name g c <<<"$setup"
  root="$work/$name"; mkdir -p "$root/.claude/hooks" "$root/src"
  cp "$g/guard.py" "$g/guard_base.py" "$root/.claude/hooks/"; cp "$c/guard-config.json" "$root/.claude/hooks/"
  printf '%s' "$src" > "$root/src/lib.rs"
  git -C "$root" init -q && git -C "$root" add -A && git -C "$root" -c user.name=e2e -c user.email=e2e@invalid commit -qm e2e
  echo "== $name (guard.py $(sha256sum "$root/.claude/hooks/guard.py" | cut -c1-8), config $(sha256sum "$root/.claude/hooks/guard-config.json" | cut -c1-8))"
  (cd "$root" && python3 .claude/hooks/guard.py fix-lock on docs/build-records/x.md 2>&1)
  check "$root" "edit inside the tests" 'assert_eq!(add(2, 2), 4);' 'assert_eq!(add(2, 2), 5);'
  check "$root" "marker's test, line -> unix," $'    test,\n' $'    unix,\n'
  check "$root" "the fix, above the tests" $'    a + b\n' $'    b + a\n'
  # Sibling of the same class, outside this change (Deferred): attached_above reads one line
  # at a time, so an attribute written over several lines directly above the marker is not
  # part of the locked region. One line (control) / several lines (gap).
  check "$root" "#[cfg(any())] added above the marker" $'}\n\n#[cfg(all(\n' $'}\n\n#[cfg(any())]\n#[cfg(all(\n'
  check "$root" "#[cfg(any(\\n))] added above the marker" $'}\n\n#[cfg(all(\n' $'}\n\n#[cfg(any(\n))]\n#[cfg(all(\n'
done
