"""Time of ONE regex call of each inline_tests pattern, old vs new, on crafted
texts at two sizes (k and 2k). Linear: doubling k about doubles the time;
quadratic: about four times. Calls are made the way the guard makes them:
starts_at and attached_above with re.search(…, re.M) on the whole text
(the guard's upward scan uses re.match per line, never slower), guarded_lines
with re.search on each line. Old calls stop being timed past CAP seconds.
Usage: bench.py [k]"""
import re
import sys
import time

from rule_new import NEW, OLD

K = int(sys.argv[1]) if len(sys.argv) > 1 else 10000
CAP = 30.0

SHAPES = {
    "starts_at": [
        ("'#[cfg(any(' + k '(test)'   (verifier I5)", lambda k: "#[cfg(any(" + "(test)" * k),
        ("'#[cfg(any(' + k ',test'", lambda k: "#[cfg(any(" + ",test" * k),
        ("'#[cfg(any(test' + k ', test' + ')'", lambda k: "#[cfg(any(test" + ", test" * k + ")"),
        ("'#[cfg(any(' + k '(test))'", lambda k: "#[cfg(any(" + "(test))" * k),
        ("'#[cfg(all(' + k '(test) ' + ')' * k", lambda k: "#[cfg(all(" + "(test) " * k + ")" * k),
        ("'#[cfg(any(test' + k ') '", lambda k: "#[cfg(any(test" + ") " * k),
        ("'#[cfg(any(,' + k newlines + 'test' + k ')'", lambda k: "#[cfg(any(," + "\n" * k + "test" + ")" * k),
        ("'#[cfg(any(' + k '(test' + spaces", lambda k: "#[cfg(any(" + "(test   " * k),
        ("k/50 lines of '#[cfg(any(' + 50 '(test)'", lambda k: ("#[cfg(any(" + "(test)" * 50 + "\n") * (k // 50)),
        ("sqrt lines: '#[cfg(any(' + m '(test)', m lines",
         lambda k: ("#[cfg(any(" + "(test)" * int(k ** 0.5 * 4) + "\n") * int(k ** 0.5 / 4)),
        ("'#' + k spaces            (round 5)", lambda k: "#" + " " * (4 * k)),
        ("'# !' + k spaces          (round 5)", lambda k: "# !" + " " * (4 * k)),
        ("'#[cfg(any(' + k ', '      (round 5)", lambda k: "#[cfg(any(" + ", " * (2 * k)),
        ("k '#\\n'", lambda k: "#\n" * (2 * k)),
        ("'#[cfg(any(' + k 'not(test),'", lambda k: "#[cfg(any(" + "not(test)," * k),
        ("'#[cfg(any(' + k '(\\ntest)'", lambda k: "#[cfg(any(" + "(\ntest)" * k),
    ],
    "guarded_lines": [
        ("'// ' + k 'cfg('           (verifier I5)", lambda k: "// " + "cfg(" * k),
        ("k 'cfg_attr('", lambda k: "cfg_attr(" * k),
        ("k 'cfg!('", lambda k: "cfg!(" * k),
        ("k 'cfg ! ('", lambda k: "cfg ! (" * k),
        ("'test ' + k 'cfg('", lambda k: "test " + "cfg(" * k),
        ("k 'cfg(tests '", lambda k: "cfg(tests " * k),
        ("'cfg' + k spaces + '!' + k spaces", lambda k: "cfg" + " " * (2 * k) + "!" + " " * (2 * k)),
        ("'cfg(' + k 'atest'", lambda k: "cfg(" + "atest" * k),
        ("k 'macro_rules!'", lambda k: "macro_rules!" * k),
        ("'macro_rules!' + k spaces", lambda k: "macro_rules!" + " " * (4 * k)),
        ("'macro_rules!assert' + k 'x'", lambda k: "macro_rules!assert" + "x" * (4 * k)),
    ],
    "attached_above": [
        ("k spaces + '#' + k spaces", lambda k: " " * (2 * k) + "#" + " " * (2 * k)),
        ("'#' + k spaces + '!' + k spaces", lambda k: "#" + " " * (2 * k) + "!" + " " * (2 * k)),
        ("k U+200B", lambda k: "​" * (4 * k)),
        ("k '\\n#'", lambda k: "\n#" * (2 * k)),
    ],
}


def call(key, pat, text):
    if key == "guarded_lines":
        return any(re.search(pat, line) for line in text.split("\n"))
    return bool(re.search(pat, text, re.M))


def timed(key, pat, text):
    t0 = time.perf_counter()
    r = call(key, pat, text)
    return time.perf_counter() - t0, r


def main():
    print("one call each; k = %d and %d; %s" % (K, 2 * K, sys.version.split()[0]))
    worst_new = 0.0
    for key, shapes in SHAPES.items():
        print("\n%s%s" % (key, "" if OLD[key] != NEW[key] else "  (unchanged: old = new)"))
        for name, make in shapes:
            row = []
            for rule in (OLD, NEW):
                ts, res = [], None
                for k in (K, 2 * K):
                    if rule is OLD and ts and ts[-1] > CAP / 4:
                        ts.append(float("nan"))
                        continue
                    dt, res = timed(key, rule[key], make(k))
                    ts.append(dt)
                row.append((ts, res))
            (o1, o2), ro = row[0]
            (n1, n2), rn = row[1]
            worst_new = max(worst_new, n1, n2)
            print("  %-46s %7d chars  old %8.4fs %8.4fs (x%5.1f)  new %7.4fs %7.4fs (x%4.1f)  %s" % (
                name, len(make(2 * K)), o1, o2, o2 / o1 if o1 > 1e-4 else float("nan"), n1, n2,
                n2 / n1 if n1 > 1e-4 else float("nan"), "same" if ro == rn else "DIFFERENT %s %s" % (ro, rn)))
    print("\nslowest new call: %.4f s" % worst_new)


if __name__ == "__main__":
    main()
