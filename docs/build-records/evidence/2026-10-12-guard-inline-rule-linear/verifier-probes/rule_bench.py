"""Single regex calls of a repository's inline_tests rule (and the guard's fixed patterns) on crafted
lines other than runs of spaces. Usage: rule_bench.py <hooks dir>"""
import json, os, re, sys, time
sys.path.insert(0, sys.argv[1]); import guard
cfg = guard.load_config(os.path.join(sys.argv[1], "guard-config.json"))
def t(label, fn):
    t0 = time.time(); r = fn(); dt = time.time() - t0
    print("%-70s %7.3fs -> %s" % (label, dt, r if not isinstance(r, str) else len(r)), flush=True)
for r in cfg.get("inline_tests", []):
    sa, aa, gl = r["starts_at"], r.get("attached_above", ""), r.get("guarded_lines", "")
    for k in (2000, 4000, 8000):
        s = "#[cfg(any(" + ",test" * k + ")" * k + "x"
        t("starts_at  '#[cfg(any(' + %d ',test' + %d ')' + 'x' (%d chars)" % (k, k, len(s)), lambda: bool(re.search(sa, s, re.M)))
        s2 = "// " + "cfg(" * (5 * k)
        t("guarded_lines  '// ' + %d 'cfg(' (%d chars)" % (5 * k, len(s2)), lambda: bool(re.search(gl, s2)))
        s3 = "#[cfg(all(" + "(" * (5 * k)
        t("starts_at  '#[cfg(all(' + %d '(' " % (5 * k), lambda: bool(re.search(sa, s3, re.M)))
    for k in (20000, 40000):
        s4 = "#[cfg(any(" + "(test)" * k
        t("starts_at  '#[cfg(any(' + %d '(test)'" % k, lambda: bool(re.search(sa, s4, re.M)))
for k in (20000, 40000):
    s = "'\\u{" + "0" * (k * 5)
    t("RUST_SCAN on '\\u{' + %d zeros" % (5 * k), lambda: len(guard.rust_file_modules(s)))
    s = "r" + "#" * (5 * k)
    t("RUST_SCAN on 'r' + %d '#'" % (5 * k), lambda: len(guard.rust_file_modules(s)))
    s = 'r#"' + '"' * (5 * k)
    t("rust_file_modules on 'r#\"' + %d '\"'" % (5 * k), lambda: len(guard.rust_file_modules(s)))
    s = "#[" + "[" * (5 * k)
    t("rust_file_attrs / rust_file_modules on '#[' + %d '['" % (5 * k), lambda: (len(guard.rust_file_attrs("#!" + s[1:])), len(guard.rust_file_modules(s))))
    s = "/*" * (5 * k)
    t("rust_file_attrs on %d '/*'" % (5 * k), lambda: len(guard.rust_file_attrs(s)))
    s = "#![a]" * (5 * k)
    t("rust_file_attrs on %d '#![a]'" % (5 * k), lambda: len(guard.rust_file_attrs(s)))
    s = "#!" + " " * (5 * k) + "x"
    t("rust_file_attrs on '#!' + %d spaces" % (5 * k), lambda: len(guard.rust_file_attrs(s)))
    s = "#![cfg(test)]" + "​" * (5 * k) + "#![x]"
    t("rust_file_attrs on '#![cfg(test)]' + %d U+200B" % (5 * k), lambda: len(guard.rust_file_attrs(s)))
    s = "mod" + "/**/" * (5 * k) + "x;"
    t("rust_file_modules on 'mod' + %d '/**/' + 'x;'" % (5 * k), lambda: len(guard.rust_file_modules(s)))
    s = "#[path=\"" + "\\a" * (5 * k)
    t("RUST_PATH.match on '#[path=\"' + %d '\\\\a'" % (5 * k), lambda: bool(guard.RUST_PATH.match(s)))
    s = "#[path=\"x\"" + " " * (5 * k) + "x"
    t("RUST_PATH.match on '#[path=\"x\"' + %d spaces + 'x'" % (5 * k), lambda: bool(guard.RUST_PATH.match(s)))
    s = "#[path = " + '"' + "x" * (5 * k) + '"' + "]" + " " * 10
    t("RUST_PATH.match long path + trailing spaces", lambda: bool(guard.RUST_PATH.match(s)))
