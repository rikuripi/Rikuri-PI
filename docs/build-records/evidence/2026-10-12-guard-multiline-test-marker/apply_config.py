"""Apply the multi-line marker branch to guard-config.json by exact text
replacement: each old piece must occur once; afterwards the parsed rule's
starts_at must equal rule_ml.SA_NEW and every other key must be unchanged."""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rule_ml
path = sys.argv[1]
raw = open(path, encoding="utf-8", newline="").read()
before = json.loads(raw)
def j(s):  # a pattern as it is written inside a JSON string (ASCII parts only)
    return json.dumps(s)[1:-1]
pieces = [(j(r"(?:any|all)\s*\((?:"), j(r"(?:any|all)\s*\((?:(?:")),
          (j(r".*\))\s*\)\s*\]") + '"', j(r".*\)|" + rule_ml.ML + r"))\s*\)\s*\]") + '"')]
for old, new in pieces:
    assert raw.count(old) == 1, (old, raw.count(old))
    raw = raw.replace(old, new)
doc_old = ("they match exactly what they matched before (verifier round 3, I5; "
           "docs/build-records/2026-10-12-guard-inline-rule-linear.md)")
doc_new = doc_old + (", and a branch added the same day reads a marker rustfmt writes over several lines "
                     "(`#[cfg(all(` alone on its line, `test,` on the next), which neither the rule nor the "
                     "tripwire saw: from `any(`/`all(` it commits to the first `test` before the attribute's "
                     "next bracket and needs that bracket to be the `]` closing `))`, so it never reads past "
                     "another marker's `[` and stays linear (owner, 2026-10-12; "
                     "docs/build-records/2026-10-12-guard-multiline-test-marker.md)")
assert raw.count(doc_old) == 1
raw = raw.replace(doc_old, doc_new)
after = json.loads(raw)
assert after["inline_tests"][0]["starts_at"] == rule_ml.SA_NEW
assert {k: v for k, v in after["inline_tests"][0].items() if k != "starts_at"} == \
       {k: v for k, v in before["inline_tests"][0].items() if k != "starts_at"}
assert {k: v for k, v in after.items() if k not in ("inline_tests", "_inline_tests_doc")} == \
       {k: v for k, v in before.items() if k not in ("inline_tests", "_inline_tests_doc")}
old_raw = open(path, encoding="utf-8", newline="").read()
assert [c for c in old_raw if ord(c) > 127] == [c for c in raw if ord(c) > 127]
open(path, "w", encoding="utf-8", newline="").write(raw)
print("ok")
