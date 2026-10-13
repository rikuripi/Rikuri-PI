"""Shared harness: a scratch crate with the hooks copied in, the fix-lock held by session S1,
and the REAL hook process (guard.py reading PreToolUse JSON on stdin)."""
import json, os, shutil, subprocess, sys, tempfile, time
LIB = ("pub fn two() -> u8 {\n    2\n}\n\n#[cfg(test)]\nmod chk;\n\n#[cfg(test)]\nmod tests {\n"
       "    #[test]\n    fn t() {\n        assert_eq!(super::two(), 2);\n    }\n}\n")
CHK = "#[test]\nfn c() {\n    assert_eq!(super::two(), 2);\n}\n"
def make(hooks, profile=None, lock=True):
    root = tempfile.mkdtemp(prefix="v3-", dir=os.environ.get("PROBE_TMP"))
    os.makedirs(os.path.join(root, "src"))
    open(os.path.join(root, "src/lib.rs"), "w").write(LIB); open(os.path.join(root, "src/chk.rs"), "w").write(CHK)
    shutil.copytree(hooks, os.path.join(root, ".claude/hooks"), ignore=shutil.ignore_patterns("__pycache__", "test_guard.py"))
    if profile:
        p = os.path.join(root, ".claude/hooks/guard-config.json"); c = json.load(open(p)); c["profile"] = profile; json.dump(c, open(p, "w"))
    if lock:
        os.makedirs(os.path.join(root, ".claude/state/fix-locks")); open(os.path.join(root, ".claude/state/fix-locks/S1.json"), "w").write("{}")
    return root
def hook(root, tool, tin, timeout=180):
    ev = {"session_id": "S1", "hook_event_name": "PreToolUse", "tool_name": tool, "tool_input": tin, "cwd": root}
    env = {k: v for k, v in os.environ.items() if not k.startswith("GIT_")}
    t = time.time()
    try:
        p = subprocess.run([sys.executable, os.path.join(root, ".claude/hooks/guard.py")], input=json.dumps(ev),
                           capture_output=True, text=True, env=env, timeout=timeout)
    except subprocess.TimeoutExpired:
        return "TIMEOUT>%ds" % timeout, float(timeout), ""
    if not p.stdout.strip():
        return "allow", time.time() - t, ""
    o = json.loads(p.stdout)["hookSpecificOutput"]
    return o["permissionDecision"], time.time() - t, o["permissionDecisionReason"][:80]
def show(label, r):
    print("  %-78s %-7s %7.2fs%s  %s" % (label, r[0], r[1], "  <-- past the hook's 10 s timeout" if r[1] > 10 else "", r[2]), flush=True)
