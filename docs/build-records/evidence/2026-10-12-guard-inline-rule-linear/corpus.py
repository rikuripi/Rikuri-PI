"""Old vs new inline_tests rule on real Rust: every .rs file under the given
folders, read as the guard reads it (UTF-8, \\r\\n folded to \\n; files that are
not UTF-8 skipped and counted). Compared: every line start where starts_at
begins a match (as differential.py), and guarded_lines on every line. Also
the slowest single file for each rule.
Usage: corpus.py <folder>…"""
import os
import re
import sys
import time

from differential import GL_N, GL_O, SA_N, SA_O, starts


def main():
    files = skipped = sa_files = gl_lines = lines = 0
    slow = {"old": (0.0, ""), "new": (0.0, "")}
    for top in sys.argv[1:]:
        for dirpath, dirnames, names in os.walk(top):
            dirnames.sort()
            for n in sorted(names):
                if not n.endswith(".rs"):
                    continue
                p = os.path.join(dirpath, n)
                try:
                    with open(p, encoding="utf-8", newline="") as f:
                        text = f.read().replace("\r\n", "\n")
                except (OSError, UnicodeDecodeError):
                    skipped += 1
                    continue
                files += 1
                t0 = time.perf_counter()
                o = starts(SA_O, text)
                go = [bool(GL_O.search(line)) for line in text.split("\n")]
                t1 = time.perf_counter()
                w = starts(SA_N, text)
                gw = [bool(GL_N.search(line)) for line in text.split("\n")]
                t2 = time.perf_counter()
                if o != w or go != gw:
                    print("DIFFERENT", p, o, w, [i for i, (a, b) in enumerate(zip(go, gw)) if a != b][:5])
                    sys.exit(1)
                sa_files += bool(o)
                gl_lines += sum(go)
                lines += len(go)
                slow["old"] = max(slow["old"], (t1 - t0, p))
                slow["new"] = max(slow["new"], (t2 - t1, p))
    print("%s: %d .rs files (%d not UTF-8, skipped), %d lines; %d files with a starts_at match, %d guarded lines: "
          "old and new decide the same everywhere" % (" ".join(sys.argv[1:]), files, skipped, lines, sa_files, gl_lines))
    for k, (t, p) in slow.items():
        print("  slowest file, %s rule: %.3f s  %s" % (k, t, p))


if __name__ == "__main__":
    main()
