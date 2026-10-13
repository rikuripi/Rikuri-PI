"""The inline_tests rule before and after this change, built from the base commit's
guard-config.json by exact substring replacement (each old piece must occur once).
Record: docs/build-records/2026-10-12-guard-inline-rule-linear.md

Usage as a module: OLD, NEW are dicts with starts_at / attached_above / guarded_lines.
Run directly: prints both (JSON-escaped) and whether NEW is the rule the
repository's guard-config.json holds now."""
import json
import os
import subprocess

BASE = "dc7e327"  # Rikuri-PI fix/guard-rust-test-layouts-sync: the rule before this change
REPO = subprocess.check_output(["git", "-C", os.path.dirname(os.path.abspath(__file__)), "rev-parse",
                                "--show-toplevel"], text=True).strip()

_cfg = json.loads(subprocess.check_output(["git", "-C", REPO, "show", BASE + ":.claude/hooks/guard-config.json"]))
(OLD,) = _cfg["inline_tests"]

# starts_at: the any()/all() branch. Old: every `test` the line offers is tried
# against every `)` after it. New: the first `test` on the line is committed
# (lookahead + backreference = an atomic group, which Python 3.9 lacks); a
# later one on the same line can only have fewer `)` after it. A `test` on a
# later line (reached through `[(,]` and white space that spans a line break)
# is the one other candidate, tried on its own (white space precedes it, so
# the `not(` lookbehind always holds there and is left out).
SA_OLD = r"(?:.*[(,]\s*)?(?<!not\()test(?=\s*[,)]).*\)"
SA_NEW = (r"(?:(?=(?P<t>(?:.*?[(,][^\S\n]*)??(?<!not\()test(?=\s*[,)])))(?P=t)"
          r"|.*[(,][^\S\n]*\n\s*test(?=\s*[,)])).*\)")

# guarded_lines (the guard calls it on one line at a time): the cfg(...)
# branch. Old: every `cfg(` on the line scans the rest of the line for
# `test`. New: the first `cfg(` is committed; a later one can only have less
# of the line after it. Both branches are anchored at the line's start (the
# macro_rules! one through `.*`), so a search tries the line once, from its
# start, instead of at every position.
GL_OLD = (r"cfg(?:_attr)?\s*(?:!\s*)?\(.*\btest\b"
          r"|macro_rules!\s*(?:assert\w*|debug_assert\w*|panic|unreachable|todo)\b")
GL_NEW = (r"^(?:(?=(?P<c>.*?cfg(?:_attr)?\s*(?:!\s*)?\())(?P=c).*\btest\b"
          r"|.*macro_rules!\s*(?:assert\w*|debug_assert\w*|panic|unreachable|todo)\b)")


def _swap(s, old, new):
    assert s.count(old) == 1, (old, s)
    return s.replace(old, new)


NEW = dict(OLD, starts_at=_swap(OLD["starts_at"], SA_OLD, SA_NEW),
           guarded_lines=_swap(OLD["guarded_lines"], GL_OLD, GL_NEW))

if __name__ == "__main__":
    for k in ("starts_at", "attached_above", "guarded_lines"):
        print(k, "changed" if OLD[k] != NEW[k] else "unchanged")
        print("  old", json.dumps(OLD[k]))
        print("  new", json.dumps(NEW[k]))
    with open(os.path.join(REPO, ".claude/hooks/guard-config.json"), encoding="utf-8") as f:
        now = json.load(f)["inline_tests"]
    print("guard-config.json now holds", "NEW" if now == [NEW] else "OLD" if now == [OLD] else "neither")
