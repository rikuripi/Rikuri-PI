"""Old vs new inline_tests rule: do they decide the same?

starts_at: the guard uses the start of the first match (re.search, re.M) and,
for attributes, whether there is one. Compared here: EVERY line start where a
match begins (search, then search again from the next line), on whole texts.
guarded_lines: the guard calls re.search(pattern, line) on each line of
text.split("\\n"). Compared here: that boolean, on every line of every text.

Usage: differential.py [seconds] [seed]"""
import random
import re
import sys
import time

from rule_new import NEW, OLD

SA_O, SA_N = re.compile(OLD["starts_at"], re.M), re.compile(NEW["starts_at"], re.M)
GL_O, GL_N = re.compile(OLD["guarded_lines"]), re.compile(NEW["guarded_lines"])


def starts(rx, text):
    out, pos = [], 0
    while True:
        m = rx.search(text, pos)
        if m is None:
            return out
        out.append(m.start())
        nl = text.find("\n", m.start())
        if nl < 0:
            return out
        pos = nl + 1


TOKENS = ["#", "!", "[", "]", "(", ")", ",", " ", " ", "\n", "\n", "\t", "\r", "\x0b", "​", "\xa0", "\x1c",
          " ", "cfg", "cfg", "_attr", "test", "test", "test", "tests", "atest", "any", "all", "not", "not(",
          "x", "=", '"', "// ", "macro_rules!", "assert", "panic", "_", "cfg(", "any(", "(test)", ",test", "#[",
          "#![", "cfg!", "))]", ")]", "é"]


def rand_text(r):
    n = r.randint(0, 40)
    return "".join(r.choice(TOKENS) for _ in range(n))


def shaped(r):
    """Attribute-like texts, more likely to come near a match."""
    head = r.choice(["#[cfg(", "#![cfg(", "# ! [ cfg (", "  #[cfg(", "#[cfg(\n", "#\n[cfg(", "‎#[cfg("])
    body = r.choice(["test", "any(", "all(", "any (", "all(\n", "not(", "any(not(", "any(\n"])
    mid = "".join(r.choice(["test", ", ", ",", "(", ")", " ", "\n", "\n ", "not(", "unix", 'feature = "test"',
                            "testx", "x", "\t", " ", "))", "]", ",", "(test", ",test", "test)", "test,",
                            "\n test", "not(\ntest", "not( test", ",\n", "(\n", "))]", "\n))]"])
                  for _ in range(r.randint(0, 12)))
    tail = r.choice(["))]", ")]", ") )\n]", "", "))", ")]\n", " ) ) ] ", "))\n\n]"])
    pre = r.choice(["", "fn a() {}\n", "x\n\n", "// c\n"])
    return pre + head + body + mid + tail + r.choice(["", "\nmod tests {}", "\n"])


def main():
    secs = float(sys.argv[1]) if len(sys.argv) > 1 else 20
    seed = int(sys.argv[2]) if len(sys.argv) > 2 else 1
    r = random.Random(seed)
    t_end = time.time() + secs
    n = sa_hits = gl_hits = lines = 0
    while time.time() < t_end:
        text = shaped(r) if r.random() < 0.6 else rand_text(r)
        n += 1
        o, w = starts(SA_O, text), starts(SA_N, text)
        if o != w:
            print("STARTS_AT DIFFERS", repr(text), o, w)
            sys.exit(1)
        sa_hits += bool(o)
        for line in text.split("\n"):
            lines += 1
            a, b = bool(GL_O.search(line)), bool(GL_N.search(line))
            if a != b:
                print("GUARDED_LINES DIFFERS", repr(line), a, b)
                sys.exit(1)
            gl_hits += a
    print("seed %d, %.0f s: %d texts (%d with a starts_at match), %d lines (%d guarded): no difference"
          % (seed, secs, n, sa_hits, lines, gl_hits))


if __name__ == "__main__":
    main()
