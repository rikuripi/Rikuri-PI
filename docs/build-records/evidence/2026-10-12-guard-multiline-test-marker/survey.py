"""What widening starts_at and RUST_INLINE_TEST would change on real Rust:
per folder, files whose first starts_at match moves (old vs new), files the
old/new tripwire marker finds (the gap guarded_lines keeps: survey_gl.py).
Usage: survey.py <folder>…"""
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rule_ml  # noqa: E402

SA_OLD = re.compile(rule_ml.SA_OLD, re.M)
SA_NEW = re.compile(rule_ml.SA_NEW, re.M)
RIT_OLD = re.compile(rule_ml.RIT_OLD, re.M)
RIT_NEW = re.compile(rule_ml.RIT_NEW, re.M)


def first(p, text):
    m = p.search(text)
    return None if m is None else m.start()


for top in sys.argv[1:]:
    files = moved = new_only = rit_old = rit_new = 0
    ex = {"moved": [], "rit": []}
    for d, _, names in os.walk(top):
        for f in names:
            if not f.endswith(".rs"):
                continue
            path = os.path.join(d, f)
            try:
                with open(path, encoding="utf-8", newline="") as fh:
                    text = fh.read().replace("\r\n", "\n")
            except (OSError, UnicodeDecodeError):
                continue
            files += 1
            a, b = first(SA_OLD, text), first(SA_NEW, text)
            if a != b:
                moved += 1
                new_only += a is None
                if len(ex["moved"]) < 3:
                    ex["moved"].append("%s:%d" % (path, text.count("\n", 0, b) + 1))
                assert b is not None and (a is None or b < a), path  # only ever earlier
            ro, rn = first(RIT_OLD, text), first(RIT_NEW, text)
            rit_old += ro is not None
            rit_new += rn is not None
            if rn is not None and ro != rn and len(ex["rit"]) < 3:
                ex["rit"].append("%s:%d" % (path, text.count("\n", 0, rn) + 1))
            if rn is not None and (b is None or b > rn):
                print("  TRIPWIRE NOT COVERED BY NEW RULE:", path)
    print("%s: %d .rs files" % (top, files))
    print("  starts_at: first match moves in %d files (%d had none before)" % (moved, new_only))
    print("  tripwire marker: %d files before, %d after" % (rit_old, rit_new))
    for k, v in ex.items():
        for e in v:
            print("   e.g. [%s] %s" % (k, e))
