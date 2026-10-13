"""Markers the new starts_at still misses: each `#[cfg(any(`/`#[cfg(all(` attribute
is read to its true end with guard.py's own scanner (rust_attr_end: brackets,
comments and strings), comments and strings are blanked, and it counts as a
marker when a `test` item (after `any(`/`all(`, `,` or a `(` that is not `not(`;
followed by `,` or `)`) is in it. Prints those the new starts_at does not match
at the attribute's start. Usage: misses.py <hooks folder> <folder>…"""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, sys.argv[1])
import rule_ml, guard
SN = re.compile(rule_ml.SA_NEW, re.M)
HEAD = re.compile(r"^[ \t]*#\s*(?:!\s*)?\[(?=\s*cfg\s*\(\s*(?:any|all)\s*\()", re.M)
BLANK = re.compile(r"/\*[\s\S]*?\*/|//[^\n]*|\"(?:\\.|[^\"\\])*\"")
ITEM = re.compile(r"(?:(?:any|all)\s*\(|,|(?<!not)\()\s*test(?=\s*[,)])")
n = miss = 0
for top in sys.argv[2:]:
    for d, _, names in os.walk(top):
        for f in names:
            if not f.endswith(".rs"):
                continue
            p = os.path.join(d, f)
            try:
                t = open(p, encoding="utf-8", newline="").read().replace("\r\n", "\n")
            except (OSError, UnicodeDecodeError):
                continue
            for h in HEAD.finditer(t):
                attr = t[h.start():guard.rust_attr_end(t, h.end())]
                if not ITEM.search(BLANK.sub(lambda m: " " * len(m.group()), attr)):
                    continue
                n += 1
                if not SN.match(t, h.start()):
                    miss += 1
                    print("MISSED", p, t.count("\n", 0, h.start()) + 1, re.sub(r"\s+", " ", attr)[:140])
print("any()/all() markers holding a test item: %d; missed by the new starts_at: %d" % (n, miss))
