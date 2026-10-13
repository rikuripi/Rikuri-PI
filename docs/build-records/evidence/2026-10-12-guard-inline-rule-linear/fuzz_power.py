"""Does the differential fuzzer see a wrong rewrite? Each mutant below changes
the NEW patterns in one way that changes what they match; the fuzzer must
report a difference for every one within the time given.
Usage: fuzz_power.py [seconds per mutant]"""
import random
import re
import sys
import time

import differential as d
from rule_new import NEW, OLD

MUTANTS = [
    ("starts_at: no later-line branch",
     "starts_at", r"|.*[(,][^\S\n]*\n\s*test(?=\s*[,)])", ""),
    ("starts_at: later `test` before `test` right after any(",
     "starts_at", r"[^\S\n]*)??(?<!not", r"[^\S\n]*)?(?<!not"),
    ("starts_at: last `test` on the line committed, not the first",
     "starts_at", r"(?:.*?[(,]", r"(?:.*[(,]"),
    ("starts_at: no not( lookbehind on the first line",
     "starts_at", r"[^\S\n]*)??(?<!not\()test", r"[^\S\n]*)??test"),
    ("starts_at: a later line may start with not(",
     "starts_at", r"\n\s*test", r"\n\s*(?:not\()?test"),
    ("starts_at: later line without the line break",
     "starts_at", r"[^\S\n]*\n\s*test", r"[^\S\n]*test"),
    ("starts_at: `)` scan crosses lines",
     "starts_at", r"(?=\s*[,)])).*\)", r"(?=\s*[,)]))[\s\S]*\)"),
    ("guarded_lines: last cfg( committed, not the first",
     "guarded_lines", r"(?=(?P<c>.*?cfg", r"(?=(?P<c>.*cfg"),
    ("guarded_lines: macro_rules! only at the line's start",
     "guarded_lines", r"|.*macro_rules!", r"|macro_rules!"),
    ("guarded_lines: test before the first cfg( counts",
     "guarded_lines", r"(?P=c).*\btest\b", r".*\btest\b"),
    ("guarded_lines: cfg_attr dropped",
     "guarded_lines", r"cfg(?:_attr)?\s*", r"cfg\s*"),
]


def main():
    secs = float(sys.argv[1]) if len(sys.argv) > 1 else 10
    bad = 0
    for name, key, old, new in MUTANTS:
        pat = NEW[key]
        assert pat.count(old) == 1, (name, old)
        mut = pat.replace(old, new)
        sa = re.compile(mut if key == "starts_at" else NEW["starts_at"], re.M)
        gl = re.compile(mut if key == "guarded_lines" else NEW["guarded_lines"])
        r = random.Random(7)
        t_end, n, found = time.time() + secs, 0, None
        while time.time() < t_end and found is None:
            text = d.shaped(r) if r.random() < 0.6 else d.rand_text(r)
            n += 1
            if d.starts(d.SA_O, text) != d.starts(sa, text):
                found = text
            for line in text.split("\n"):
                if found is None and bool(d.GL_O.search(line)) != bool(gl.search(line)):
                    found = line
        print("%-62s %s after %d texts%s" % (name, "KILLED" if found is not None else "SURVIVED", n,
                                             (": " + repr(found)) if found is not None else ""))
        bad += found is None
    sys.exit(bad)


if __name__ == "__main__":
    main()
