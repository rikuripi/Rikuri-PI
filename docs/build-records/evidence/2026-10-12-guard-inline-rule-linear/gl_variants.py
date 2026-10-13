"""Why guarded_lines is anchored as a whole. Ways of writing it that decide the
same, timed the way the guard calls it (re.search on each line, no flags) on
the lines of one large real file, and on one crafted line (verifier round 3's
'// ' + 20,000 'cfg('). The first `cfg(` is committed in every variant but the
old rule; they differ in how the two branches are anchored.
Usage: gl_variants.py <a large .rs file>"""
import re
import sys
import time

from rule_new import NEW, OLD

X = r"cfg(?:_attr)?\s*(?:!\s*)?\("
MR = r"macro_rules!\s*(?:assert\w*|debug_assert\w*|panic|unreachable|todo)\b"
VARIANTS = [
    ("old rule (no commit, unanchored)", OLD["guarded_lines"]),
    ("cfg( branch anchored, macro_rules! branch not", r"^(?=(?P<c>.*?" + X + r"))(?P=c).*\btest\b|" + MR),
    ("first cfg( by an unrolled loop, macro_rules! not anchored",
     r"^(?=(?P<c>[^c]*(?:c(?!fg(?:_attr)?\s*(?:!\s*)?\()[^c]*)*))(?P=c)" + X + r".*\btest\b|" + MR),
    ("unrolled loop, both anchored",
     r"^(?:(?=(?P<c>[^c]*(?:c(?!fg(?:_attr)?\s*(?:!\s*)?\()[^c]*)*))(?P=c)" + X + r".*\btest\b|.*" + MR + ")"),
    ("both anchored (the rule now)", NEW["guarded_lines"]),
]


def best(fn, n=5):
    ts = []
    for _ in range(n):
        t0 = time.perf_counter()
        fn()
        ts.append(time.perf_counter() - t0)
    return min(ts)


with open(sys.argv[1], encoding="utf-8", newline="") as f:
    lines = f.read().replace("\r\n", "\n").split("\n")
crafted = "// " + "cfg(" * 20000
ref = [bool(re.search(OLD["guarded_lines"], line)) for line in lines]
print("%s: %d lines; %s" % (sys.argv[1], len(lines), sys.version.split()[0]))
for name, pat in VARIANTS:
    rx = re.compile(pat)
    same = [bool(rx.search(line)) for line in lines] == ref and bool(rx.search(crafted)) == bool(
        re.search(OLD["guarded_lines"], crafted))
    t = best(lambda: [rx.search(line) for line in lines])
    t2 = best(lambda: rx.search(crafted), 1)
    print("  %-58s all lines %.4f s (best of 5)   '// ' + 20,000 'cfg(' %.4f s   %s"
          % (name, t, t2, "same decisions" if same else "DIFFERENT"))
