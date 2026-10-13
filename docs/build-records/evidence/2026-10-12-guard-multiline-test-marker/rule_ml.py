"""The inline_tests rule's starts_at and the tripwire's RUST_INLINE_TEST, before
and after the branch that reads a test marker written over several lines
(rustfmt's form for a long attribute):

    #[cfg(all(
        test,
        feature = "x",
    ))]

The base is the linear-time change (2026-10-12-guard-inline-rule-linear). Both
forms are built from the config RULE_CFG names (default: this checkout's
.claude/hooks/guard-config.json, found from this file's folder or the current
one), whichever form it holds, by exact substring replacement; the old
RUST_INLINE_TEST is written out (ASCII), the new one must equal guard.py's.

Usage as a module: SA_OLD/SA_NEW (starts_at), RIT_OLD/RIT_NEW (RUST_INLINE_TEST), ML.
Run directly: prints them, and which form the config holds."""
import json
import os
import re


def _find(rel):
    for top in (os.environ.get("RULE_ROOT"), os.path.dirname(os.path.abspath(__file__)), os.getcwd()):
        d = top
        while d and d != os.path.dirname(d):
            if os.path.exists(os.path.join(d, rel)):
                return os.path.join(d, rel)
            d = os.path.dirname(d)
    raise FileNotFoundError(rel)


CFG_PATH = os.environ.get("RULE_CFG") or _find(".claude/hooks/guard-config.json")
with open(CFG_PATH, encoding="utf-8") as f:
    (_RULE,) = json.load(f)["inline_tests"]

# The added branch: after `any(`/`all(`, the first `test` (at the start, after
# white space, or after `,` or a `(` that is not `not(`, white space and line
# breaks between) before the attribute's next bracket, committed by lookahead
# and backreference; then the rest up to that bracket, which must be `]` closing
# `))`. Between the first and a later `test` there is no bracket, so the first
# bracket after each is the same one: only the first needs checking. The scan
# never passes a `[` or `]`, so it stops at the next marker's `[` and no text is
# scanned for two markers (linear).
ML = r"(?=(?P<m>(?:[^\[\]]*?(?:,|(?<!not)\())??\s*test(?=\s*[,)])))(?P=m)[^\[\]]*\)"

# (?:any|all)\s*\((?:A|B).*\)  <->  (?:any|all)\s*\((?:(?:A|B).*\)|ML)
_PIECES = [(r"(?:any|all)\s*\((?:", r"(?:any|all)\s*\((?:(?:"),
           (r".*\))\s*\)\s*\]", r".*\)|" + ML + r"))\s*\)\s*\]")]


def _swap(s, pairs):
    for old, new in pairs:
        assert s.count(old) == 1, (old, s)
        s = s.replace(old, new)
    return s


if ML in _RULE["starts_at"]:
    HOLDS = "new"
    SA_NEW = _RULE["starts_at"]
    SA_OLD = _swap(SA_NEW, [(n, o) for o, n in reversed(_PIECES)])
else:
    HOLDS = "old"
    SA_OLD = _RULE["starts_at"]
    SA_NEW = _swap(SA_OLD, _PIECES)
assert _swap(SA_OLD, _PIECES) == SA_NEW

RIT_OLD = r"^[ \t]*#\s*(?:!\s*)?\[\s*cfg\s*\(\s*test\s*\)\s*\]"
RIT_NEW = r"^[ \t]*#\s*(?:!\s*)?\[\s*cfg\s*\(\s*(?:test|(?:any|all)\s*\(" + ML + r")\s*\)\s*\]"

for _p in (SA_OLD, SA_NEW, RIT_OLD, RIT_NEW):
    re.compile(_p, re.M)

if __name__ == "__main__":
    print("config", CFG_PATH, "holds the", HOLDS, "starts_at")
    print("starts_at old", json.dumps(SA_OLD))
    print("starts_at new", json.dumps(SA_NEW))
    print("RUST_INLINE_TEST old", RIT_OLD)
    print("RUST_INLINE_TEST new", RIT_NEW)
