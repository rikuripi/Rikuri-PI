"""How often real Rust writes a test marker over several lines, rustfmt-style:
`#[cfg(any(` alone on its line, `test,` on a line of its own below. Neither the
inline_tests rule (old or new) nor the tripwire's RUST_INLINE_TEST sees it.
Usage: survey_multiline.py <folder>…"""
import os
import re
import sys

HEAD = re.compile(r"^[ \t]*#!?\[cfg\((?:any|all)\(\s*$")
TEST = re.compile(r"test\s*[,)]*\]?")
for top in sys.argv[1:]:
    n = files = 0
    ex = []
    for d, _, names in os.walk(top):
        for f in names:
            if not f.endswith(".rs"):
                continue
            files += 1
            try:
                with open(os.path.join(d, f), encoding="utf-8") as fh:
                    lines = fh.read().split("\n")
            except (OSError, UnicodeDecodeError):
                continue
            for i, line in enumerate(lines):
                if not HEAD.match(line):
                    continue
                body = []
                for l2 in lines[i + 1:i + 12]:
                    body.append(l2.strip())
                    if l2.strip().endswith("]"):
                        break
                if any(TEST.fullmatch(b) for b in body):
                    n += 1
                    if len(ex) < 3:
                        ex.append("%s:%d" % (os.path.join(d, f), i + 1))
                    break
    print("%s: %d .rs files, %d with a multi-line any()/all() marker holding `test`" % (top, files, n))
    for e in ex:
        print("  e.g.", e)
