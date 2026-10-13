"""The texts of GenericPolicy.test_inline_rules_take_linear_time, read from the
test file itself, timed one call each (as the test calls them) against the rule
before (dc7e327) and after (rule_new.NEW). The test stops at its first slow
call; this shows every added text is slow on the old rule on its own.
Usage: test_texts_table.py <test_guard.py>"""
import ast
import re
import sys
import time

from rule_new import NEW, OLD

src = open(sys.argv[1], encoding="utf-8").read()
fn = next(n for n in ast.walk(ast.parse(src)) if isinstance(n, ast.FunctionDef)
          and n.name == "test_inline_rules_take_linear_time")
texts_node = next(n.value for n in ast.walk(fn) if isinstance(n, ast.Assign) and n.targets[0].id == "texts")
texts = eval(compile(ast.Expression(texts_node), "texts", "eval"))
print("%d texts; one re.search(pattern, text, re.M) each, as the test makes it; limit 0.25 s; %s"
      % (len(texts), sys.version.split()[0]))
for text in texts:
    for key in ("starts_at", "attached_above", "guarded_lines"):
        row = []
        for rule in (OLD, NEW):
            t0 = time.perf_counter()
            re.search(rule[key], text, re.M)
            row.append(time.perf_counter() - t0)
        flag = "  <-- old over the limit" if row[0] >= 0.25 else ""
        print("  %-26r %7d chars  %-14s old %8.4f s  new %7.4f s%s" % (text[:24], len(text), key, row[0], row[1], flag))
