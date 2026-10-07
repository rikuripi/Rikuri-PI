#!/usr/bin/env bash
# Debt ratchet (Engineering Standard, Phase 0): counted debt may only go
# down. Compares current counts with ratchet-baseline.json and fails on any
# rise. A fall passes with a notice; lower the baseline in a later change.
#
#   ratchet.sh [root]            check against the baseline
#   ratchet.sh --update [root]   print current counts as JSON (a baseline change
#                                is a reviewed change; the fix-lock makes it
#                                read-only)
set -euo pipefail
update=0
if [ "${1:-}" = "--update" ]; then update=1; shift; fi
cd "${1:-$(git rev-parse --show-toplevel)}"
baseline=scripts/quality/ratchet-baseline.json

# count <regex> <pathspec...>: matching lines in tracked files. git grep
# exits 1 for "no match", which counts as 0; any other failure stops the
# ratchet instead of reading as zero debt.
count() {
  local regex="$1" out rc=0; shift
  out="$(git grep -hcE -e "$regex" -- "$@")" || rc=$?
  if [ "$rc" -gt 1 ]; then echo "RATCHET git grep failed (exit $rc) for ${regex}" >&2; return 1; fi
  printf '%s\n' "$out" | awk 'NF {s += $1} END {print s + 0}'
}

# rust_attrs: counts Rust attributes in tracked .rs files, prints
# "<ignored_tests> <lint_suppressions>". Attributes may span lines (rustfmt
# wraps long ones), so this reads whole files, not lines. Comments and the
# contents of string and char literals are blanked first, so attribute text
# in a comment or string neither counts nor ends an attribute early (a ")"
# in a string once hid an allow; verifier, 2026-10-05). Counted: #[ignore …],
# #[allow(…)], #![expect(…)], and any #[cfg_attr(…)] whose body holds
# allow(…)/expect(…) (a suppression) or ignore (an ignored test); names may
# be raw identifiers (#[r#allow(…)]). Not counted (recorded limitations):
# attributes a macro generates, and files that are not .rs but are compiled
# through #[path] or include!.
rust_attrs() {
  python3 - <<'PY'
import re, subprocess, sys

listed = subprocess.run(["git", "ls-files", "-z", "--", "*.rs"], capture_output=True)
if listed.returncode != 0:
    sys.stderr.write("RATCHET git ls-files failed\n")
    sys.exit(1)

IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
PLAIN = re.compile(r"[^/\"'A-Za-z_]+")
RAW_OPEN = re.compile(r'(#*)"')
CHAR = re.compile(r"'(?:\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F_]{1,6}\}|.)|[^\\'\n])'")

def code_only(text):
    """The source with comments removed and literal contents emptied."""
    out, i, n = [], 0, len(text)
    while i < n:
        if text.startswith("//", i):
            j = text.find("\n", i)
            i = n if j < 0 else j
            out.append(" ")
        elif text.startswith("/*", i):  # block comments nest
            depth, i = 1, i + 2
            while i < n and depth:
                if text.startswith("/*", i):
                    depth, i = depth + 1, i + 2
                elif text.startswith("*/", i):
                    depth, i = depth - 1, i + 2
                else:
                    i += 1
            out.append(" ")
        elif text[i] == '"':
            i += 1
            while i < n and text[i] != '"':
                i += 2 if text[i] == "\\" else 1
            i += 1
            out.append('""')
        elif text[i] == "'":
            m = CHAR.match(text, i)
            if m:
                out.append("''")
                i = m.end()
            else:  # a lifetime or loop label
                out.append("'")
                i += 1
        elif m := IDENT.match(text, i):
            word, i = m.group(), m.end()
            raw = RAW_OPEN.match(text, i) if word in ("r", "br", "cr") else None
            if raw:
                close = '"' + raw.group(1)
                end = text.find(close, raw.end())
                i = n if end < 0 else end + len(close)
                word += '""'
            out.append(word)
        elif m := PLAIN.match(text, i):
            out.append(m.group())
            i = m.end()
        else:  # a "/" that starts no comment, or a non-ASCII character
            out.append(text[i])
            i += 1
    return "".join(out)

# An attribute opener, then the name, which may be a raw identifier (r#allow).
ATTR = r"#\s*!?\s*\[\s*(?:r#)?"
ignored = suppressions = 0
for path in filter(None, listed.stdout.decode().split("\0")):
    with open(path, encoding="utf-8", errors="replace") as f:
        code = code_only(f.read())
    ignored += len(re.findall(ATTR + r"ignore\b", code))
    suppressions += len(re.findall(ATTR + r"(?:allow|expect)\s*\(", code))
    for m in re.finditer(ATTR + r"cfg_attr\s*\(", code):
        depth, i = 1, m.end()
        while i < len(code) and depth:
            depth += {"(": 1, ")": -1}.get(code[i], 0)
            i += 1
        body = code[m.end():i]
        if re.search(r"\b(?:allow|expect)\s*\(", body):
            suppressions += 1
        if re.search(r"\bignore\b", body):
            ignored += 1
print(ignored, suppressions)
PY
}

# Each count in its own assignment, so a failed count stops the script.
attrs="$(rust_attrs)"
ignored_tests="${attrs% *}"
lint_suppressions="${attrs#* }"
todo_markers="$(count '\b(TODO|FIXME|XXX|HACK)\b' '*.rs' '*.sh' '*.toml' '*.yml' ':!scripts/quality/ratchet.sh')"
current="$(cat <<JSON
{
  "ignored_tests": ${ignored_tests},
  "lint_suppressions": ${lint_suppressions},
  "todo_markers": ${todo_markers}
}
JSON
)"

if [ "$update" = 1 ]; then
  echo "$current"
  exit 0
fi

python3 - "$baseline" "$current" <<'PY'
import json, sys
base = json.load(open(sys.argv[1]))
cur = json.loads(sys.argv[2])
failed = False
for key in sorted(set(base) | set(cur)):
    b, c = base.get(key), cur.get(key)
    if b is None or c is None:
        print(f"RATCHET {key}: present in only one of baseline/current"); failed = True
    elif c > b:
        print(f"RATCHET {key}: {c} > baseline {b} (debt rose)"); failed = True
    elif c < b:
        print(f"RATCHET {key}: {c} < baseline {b} (improved; lower the baseline in a later change)")
    else:
        print(f"RATCHET {key}: {c} (baseline {b})")
sys.exit(1 if failed else 0)
PY
