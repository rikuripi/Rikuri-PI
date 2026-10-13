"""Places only the new RUST_INLINE_TEST matches (all folders given) whose
committed `test` sits inside a not(…): the heuristic the old rule already had.
Usage: not_nested.py <folder>…"""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rule_ml
RO, RN = re.compile(rule_ml.RIT_OLD, re.M), re.compile(rule_ml.RIT_NEW, re.M)
n = under_not = 0
for top in sys.argv[1:]:
    for d, _, names in os.walk(top):
        for f in names:
            if not f.endswith(".rs"):
                continue
            try:
                t = open(os.path.join(d, f), encoding="utf-8", newline="").read().replace("\r\n", "\n")
            except (OSError, UnicodeDecodeError):
                continue
            for m in RN.finditer(t):
                if RO.match(t, m.start()):
                    continue
                n += 1
                pre = m.group()[:m.start("m") - m.start() + len(m.group("m"))]  # up to the committed test
                stack = []
                for tok in re.finditer(r"\bnot\s*\(|\(|\)", pre):
                    if tok.group() == ")":
                        if stack:
                            stack.pop()
                    else:
                        stack.append(tok.group() != "(")
                if any(stack):
                    under_not += 1
                    print("  ", os.path.join(d, f), re.sub(r"\s+", " ", m.group())[:100])
print("new-only tripwire matches:", n, "with the committed test inside not(...):", under_not)
