"""Old vs new on real Rust, at every line start that holds `#`: starts_at and
RUST_INLINE_TEST must match wherever they matched before (superset); every
position only the new ones match must be a marker written over several lines;
and wherever the new RUST_INLINE_TEST matches, the new starts_at matches too
(the tripwire never names a file the rule covers). Usage: corpus.py <folder>…"""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rule_ml
SO, SN = re.compile(rule_ml.SA_OLD, re.M), re.compile(rule_ml.SA_NEW, re.M)
RO, RN = re.compile(rule_ml.RIT_OLD, re.M), re.compile(rule_ml.RIT_NEW, re.M)
HASH = re.compile(r"^[^\S\n]*#|^[​-‏⁠﻿ \t\f\v\r]*#", re.M)
files = lines = sa_new_only = rit_new_only = bad = 0
ex = []
for top in sys.argv[1:]:
    for d, _, names in os.walk(top):
        for f in names:
            if not f.endswith(".rs"):
                continue
            p = os.path.join(d, f)
            try:
                text = open(p, encoding="utf-8", newline="").read().replace("\r\n", "\n")
            except (OSError, UnicodeDecodeError):
                continue
            files += 1
            lines += text.count("\n") + 1
            for h in HASH.finditer(text):
                i = h.start()
                so, sn, ro, rn = (bool(x.match(text, i)) for x in (SO, SN, RO, RN))
                if (so and not sn) or (ro and not rn) or (rn and not sn):
                    bad += 1
                    print("BAD", p, text.count("\n", 0, i) + 1, so, sn, ro, rn)
                for old, new, tag in ((so, sn, "starts_at"), (ro, rn, "tripwire")):
                    if new and not old:
                        m = (SN if tag == "starts_at" else RN).match(text, i)
                        attr = m.group()
                        if tag == "starts_at":
                            sa_new_only += 1
                            if "\n" not in attr:
                                bad += 1
                                print("SINGLE-LINE NEW-ONLY", p, text.count("\n", 0, i) + 1, repr(attr[:120]))
                            elif len(ex) < 6:
                                ex.append("%s:%d %r" % (p, text.count("\n", 0, i) + 1, attr[:90]))
                        else:
                            rit_new_only += 1
print("%d .rs files, %d lines" % (files, lines))
print("starts_at: %d positions only the new one matches (all multi-line: %s)" % (sa_new_only, bad == 0))
print("RUST_INLINE_TEST: %d positions only the new one matches" % rit_new_only)
print("superset and tripwire-within-rule violations: %d" % bad)
for e in ex:
    print("  e.g.", e)
