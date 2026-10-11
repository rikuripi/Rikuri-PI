"""E1 evidence: planted defects driven through the real rikurid and rikuri CLI
on a scratch home (written by verifier round 4, 2026-10-10; adopted here).
Usage: drive-daemon.py <bin-dir> <home> <scenario> [SWITCH=1 ...]
Run inside a sandbox with no network and no access to /run; the binaries come
from a scratch copy with write-set-defects-and-cost.patch applied."""
import json, os, signal, sqlite3, subprocess, sys, time

bins, home, scenario = sys.argv[1], sys.argv[2], sys.argv[3]
switches = dict(a.split("=", 1) for a in sys.argv[4:])
env = dict(os.environ, **switches)
os.makedirs(home, exist_ok=True)
log = open(os.path.join(home, "..", os.path.basename(home) + ".daemon.log"), "a")
daemon = None


def cli(*args):
    out = subprocess.run([f"{bins}/rikuri", "--home", home, *args], capture_output=True, text=True)
    try:
        return json.loads(out.stdout)
    except Exception:
        return {"ok": False, "raw": out.stdout + out.stderr, "exit": out.returncode}


def start():
    global daemon
    daemon = subprocess.Popen([f"{bins}/rikurid", "--home", home], env=env, stdout=log, stderr=log)
    for _ in range(100):
        if cli("system", "health").get("ok"):
            return
        time.sleep(0.1)
    raise SystemExit("daemon did not answer")


def stop():
    global daemon
    if daemon:
        daemon.send_signal(signal.SIGTERM)
        daemon.wait(timeout=10)
        daemon = None


def said(r):
    if r.get("ok"):
        return "ok"
    e = r.get("error") or {}
    return f"refused ({e.get('code')}): {e.get('message', r.get('raw'))}"[:300]


def res(r):
    return r.get("result") or {}


def health():
    h = res(cli("system", "health"))
    return f"state={h.get('state')} quarantined={h.get('quarantined')!r}"


def tasks():
    return {t["title"]: t for t in res(cli("state", "show")).get("tasks", [])}


def show(label):
    for title, t in sorted(tasks().items()):
        print(f"  {label} task {title}: status={t['status']} steps={t['steps_completed']} "
              f"runs={t['runs']} current={t['current_run']} privacy={t['privacy_class']}")


def db(sql):
    con = sqlite3.connect(f"file:{home}/state/rikuri.db?mode=ro", uri=True)
    try:
        return con.execute(sql).fetchall()
    finally:
        con.close()


n = [0]


def key(tag):
    n[0] += 1
    return f"key-{tag}-{n[0]:06d}"


def create(title, privacy=None, k=None):
    args = ["task", "create", "--title", title, "--idempotency-key", k or key("create")]
    if privacy:
        args += ["--privacy", privacy]
    return cli(*args)


def start_task(t, v, k=None):
    return cli("task", "start", "--task", t, "--expected-version", str(v),
               "--idempotency-key", k or key("start"))


def step(t, r, v, k=None):
    return cli("task", "step", "--task", t, "--run", r, "--expected-version", str(v),
               "--idempotency-key", k or key("step"))


def complete(t, r, v, k=None):
    return cli("task", "complete", "--task", t, "--run", r, "--expected-version", str(v),
               "--outcome", "succeeded", "--verification", "passed",
               "--idempotency-key", k or key("done"))


try:
    start()
    print(f"[{scenario}] switches={switches or 'none'}")
    init = cli("init")
    print("  init:", said(init))
    if scenario == "foreign-run":
        # N1: task a running, task b running; a step of a names b's run
        a = res(create("a")); b = res(create("b", "private"))
        sa = res(start_task(a["task_id"], a["version"])); sb = res(start_task(b["task_id"], b["version"]))
        print("  step a naming b's run:", said(step(a["task_id"], sb["run_id"], sa["version"])))
        show("after")
        print("  health:", health())
    elif scenario == "foreign-run-created":
        # N1: task a never started; a step of a names b's run
        a = res(create("a")); b = res(create("b", "private"))
        sb = res(start_task(b["task_id"], b["version"]))
        print("  step created a naming b's run:", said(step(a["task_id"], sb["run_id"], a["version"])))
        show("after")
        print("  health:", health())
    elif scenario == "foreign-complete-running":
        # N2: task a running; a completion of a names b's run
        a = res(create("a")); b = res(create("b", "private"))
        sa = res(start_task(a["task_id"], a["version"])); sb = res(start_task(b["task_id"], b["version"]))
        print("  complete a naming b's run:", said(complete(a["task_id"], sb["run_id"], sa["version"])))
        show("after")
        print("  health:", health())
    elif scenario == "foreign-complete-created":
        # N2: task a never started; a completion of a names b's run
        a = res(create("a")); b = res(create("b", "private"))
        sb = res(start_task(b["task_id"], b["version"]))
        print("  complete created a naming b's run:", said(complete(a["task_id"], sb["run_id"], a["version"])))
        show("after")
        print("  health:", health())
    elif scenario == "shared-deletion-unit":
        # N5: the second task gets the first one's deletion unit
        print("  create a:", said(create("a", "private")))
        print("  create b:", said(create("b", "local_only")))
        for r in db("SELECT json_extract(body,'$.title'), deletion_unit_id FROM objects "
                    "WHERE kind='task' ORDER BY rowid"):
            print(f"  task {r[0]}: deletion unit {r[1]}")
        print("  health:", health())
    elif scenario == "no-idempotency-record":
        # N6: no idempotency record, so a retry would run the command again
        k = "key-create-dup0001"
        print("  create a:", said(create("a", k=k)))
        print("  retry create a (same key):", said(create("a", k=k)))
        print("  tasks titled a:", len(db("SELECT 1 FROM objects WHERE kind='task' "
                                          "AND json_extract(body,'$.title')='a'")))
        print("  health:", health())
    elif scenario == "wrong-idempotency-command":
        # N9: a step's record under another command, so its retry is refused
        a = res(create("a")); sa = res(start_task(a["task_id"], a["version"]))
        k = "key-step-wrongcmd1"
        print("  step:", said(step(a["task_id"], sa["run_id"], sa["version"], k=k)))
        print("  retry step (same key, same request):", said(step(a["task_id"], sa["run_id"], sa["version"], k=k)))
        print("  health:", health())
    elif scenario == "stored-response":
        # X2: a create's record stores another task's ID, so its retry
        # answers with that task
        print("  create a:", said(create("a")))
        k = "key-create-stored01"
        c = create("c", "private", k=k)
        print("  create c:", said(c))
        again = create("c", "private", k=k)
        print("  retry create c (same key):", said(again),
              "| same task as the first answer:", res(again).get("task_id") == res(c).get("task_id"))
        print("  health:", health())
    elif scenario == "extra-record":
        # X3: a create also writes a record under "<key>-x", which then
        # refuses a later, different command under that key
        k = "key-create-extra01"
        print("  create a:", said(create("a", k=k)))
        print("  create d under key-x:", said(create("d", k=k + "-x")))
        print("  health:", health())
    elif scenario == "create-answer":
        # R1: a create answers, and records, another task's ID
        print("  create a:", said(create("a")))
        b = create("b secret", "private", k="key-create-answer1")
        print("  create b:", said(b))
        titles = {t["task_id"]: title for title, t in tasks().items()}
        print("  b's answer names the task titled:", titles.get(res(b).get("task_id")))
        print("  health:", health())
    elif scenario == "init-answer":
        # R3: init answers a project ID that is not the Inbox's
        inbox = (res(cli("state", "show")).get("project") or {}).get("project_id")
        print("  init's answer names the Inbox:", inbox is not None and res(init).get("project_id") == inbox)
        print("  health:", health())
    elif scenario == "event-schema":
        # S1: the ledger stamps events with another schema version
        print("  create a:", said(create("a")))
        print("  event schema versions:", sorted({r[0] for r in db("SELECT schema_version FROM events")}))
        print("  health:", health())
    elif scenario == "init-trust":
        # N8: the Inbox is created trusted for execution
        print("  project:", res(cli("state", "show")).get("project"))
        print("  health:", health())
    else:
        raise SystemExit(f"unknown scenario {scenario}")
finally:
    stop()
