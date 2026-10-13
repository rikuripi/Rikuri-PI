"""The gap left after widening starts_at: multi-line #[cfg…(…)] / #[cfg_attr(…)]
attributes holding the word `test` in which no single line matches guarded_lines
(the guard checks guarded_lines one line at a time) and which are no starts_at
marker. Usage: survey_gl.py <folder>…"""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rule_ml
with open(rule_ml.CFG_PATH, encoding="utf-8") as f:
    GL = re.compile(json.load(f)["inline_tests"][0]["guarded_lines"])
SA = re.compile(rule_ml.SA_NEW, re.M)
ATTR = re.compile(r"^[ \t]*#!?\[\s*cfg(?:_attr)?\s*\([^\]]*\]", re.M)
for top in sys.argv[1:]:
    n = files = 0; kinds = {}; ex = []
    for d, _, names in os.walk(top):
        for f in names:
            if not f.endswith(".rs"):
                continue
            p = os.path.join(d, f)
            try:
                text = open(p, encoding="utf-8", newline="").read().replace("\r\n", "\n")
            except (OSError, UnicodeDecodeError):
                continue
            hit = False
            for m in ATTR.finditer(text):
                a = m.group()
                if "\n" not in a or not re.search(r"\btest\b", a):
                    continue
                if any(GL.search(l) for l in a.split("\n")) or SA.match(text, m.start()):
                    continue
                k = "cfg_attr" if "cfg_attr" in a.split("(")[0] else "cfg"
                kinds[k] = kinds.get(k, 0) + 1
                hit = True
                if len(ex) < 4:
                    ex.append("%s:%d" % (p, text.count("\n", 0, m.start()) + 1))
            n += hit
    print("%s: %d files with such attributes %s" % (top, n, kinds))
    for e in ex:
        print("  e.g.", e)
