"""Through the REAL hook, lock held, sizes past the timeout.
 S   MultiEdit: test 2->3 + '/*\n#[cfg(any(' + K x '(test)' + '\n*/' above everything (starts_at)
 G0  control: production Edit adding `if cfg!(test) { return 3; }` (guarded_lines must refuse it)
 G   the same + a comment line '// ' + K x 'cfg(' above everything
Usage: rule_stall_hook2.py <hooks dir> <K_S> <K_G>"""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__))); import harness as h
HOOKS, KS, KG = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
root = h.make(HOOKS)
try:
    lib = os.path.join(root, "src/lib.rs")
    print("# %s, lock held" % HOOKS)
    h.show("S  MultiEdit: test 2->3 + '/*#[cfg(any(' + %d '(test)' */' (%d bytes)" % (KS, 6 * KS + 16), h.hook(root, "MultiEdit", {"file_path": lib, "edits": [
        {"old_string": "two(), 2);", "new_string": "two(), 3);"},
        {"old_string": "pub fn two()", "new_string": "/*\n#[cfg(any(" + "(test)" * KS + "\n*/\npub fn two()"}]}))
    h.show("G0 Edit: production code gains `if cfg!(test) { return 3; }`", h.hook(root, "Edit", {"file_path": lib,
        "old_string": "    2\n}", "new_string": "    if cfg!(test) { return 3; }\n    2\n}"}))
    h.show("G  MultiEdit: the same + '// ' + %d 'cfg(' above everything (%d bytes)" % (KG, 4 * KG + 3), h.hook(root, "MultiEdit", {"file_path": lib, "edits": [
        {"old_string": "    2\n}", "new_string": "    if cfg!(test) { return 3; }\n    2\n}"},
        {"old_string": "pub fn two()", "new_string": "// " + "cfg(" * KG + "\npub fn two()"}]}))
finally:
    import shutil; shutil.rmtree(root)
