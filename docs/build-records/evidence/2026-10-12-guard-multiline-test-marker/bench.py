"""Linear time: one re.search per crafted text, doubling its size; the new
starts_at and RUST_INLINE_TEST should grow about x2 per doubling. Then three
deliberately wrong variants of the multi-line branch, timed on the texts the
suite's linear-time test uses (limit 0.25 s): each must be caught (slower).
Usage: bench.py [k]"""
import re, sys, time, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rule_ml
K = int(sys.argv[1]) if len(sys.argv) > 1 else 4000
SHAPES = {
    "heads, never closed": lambda k: "#[cfg(any(\n" * k,
    "\\ntest, repeated": lambda k: "#[cfg(any(" + "\ntest," * k,
    ",test repeated": lambda k: "#[cfg(any(" + ",test" * k,
    "(test) repeated": lambda k: "#[cfg(any(" + "(test)" * k,
    "(\\n repeated": lambda k: "#[cfg(any(\n" + "(\n" * k,
    "test) repeated": lambda k: "#[cfg(any(\n" + "test)" * k,
    ") ) repeated": lambda k: "#[cfg(any(\ntest" + ") " * k,
    "spaces then test": lambda k: "#[cfg(any(\n" + " " * (8 * k) + "test",
    "not(test), repeated": lambda k: "#[cfg(any(\n" + "not(test),\n" * k,
    "heads with tests, unclosed": lambda k: "#[cfg(any(\n    test,\n" * k,
    "heads test at q": lambda k: "#[cfg(any(test,\n" * k,
    "nested any(test, x)": lambda k: "#[cfg(all(\n" + "    any(test, x)\n" * k,
    "newlines then ]": lambda k: "#[cfg(any(\n" + "\n" * (8 * k) + "]",
    "# and spaces": lambda k: "#" + " " * (8 * k),
    "commas spaces": lambda k: "#[cfg(any(" + ", " * (4 * k),
    "not( newline test": lambda k: "#[cfg(any(\n" + "not(\ntest\n" * k,
}
PATS = {"starts_at": re.compile(rule_ml.SA_NEW, re.M), "RUST_INLINE_TEST": re.compile(rule_ml.RIT_NEW, re.M)}

def t(p, s):
    t0 = time.perf_counter(); p.search(s); return time.perf_counter() - t0

print("%-28s %-17s %s" % ("shape", "pattern", "seconds at k, 2k, 4k, 8k (k=%d)" % K))
worst = 0
for name, mk in SHAPES.items():
    for pn, p in PATS.items():
        ts = [t(p, mk(K * m)) for m in (1, 2, 4, 8)]
        worst = max(worst, ts[-1])
        print("%-28s %-17s %s  x%.1f/doubling" % (name, pn, " ".join("%.4f" % x for x in ts),
                                                  (ts[-1] / max(ts[0], 1e-6)) ** (1 / 3)))
print("slowest single call: %.4f s" % worst)

# The texts the suite's linear-time test adds for this change, and two it had.
TEST_TEXTS = ["#[cfg(any(\n" * 10000, "#[cfg(any(" + "\ntest," * 20000, "#[cfg(any(" + ",test" * 20000]
ML = rule_ml.ML
MUTANTS = {
    "M1 no commit (no lookahead+backreference)": ML.replace("(?=(?P<m>", "(?:(?:").replace("))(?P=m)", "))"),
    "M2 reads across brackets ([\\s\\S])": ML.replace(r"[^\[\]]", r"[\s\S]"),
    "M3 stops at ] only ([^\\]])": ML.replace(r"[^\[\]]", r"[^\]]"),
}
for name, m in MUTANTS.items():
    assert m != ML, name
    sa = rule_ml.SA_NEW.replace(ML, m)
    assert sa != rule_ml.SA_NEW
    p = re.compile(sa, re.M)
    ts = [t(p, s) for s in TEST_TEXTS]
    print("%-45s %s  %s" % (name, " ".join("%.3f" % x for x in ts),
                            "CAUGHT" if max(ts) > 0.25 else "NOT CAUGHT"))
