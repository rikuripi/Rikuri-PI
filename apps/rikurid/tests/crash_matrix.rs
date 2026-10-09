//! The kernel crash matrix against the real daemon (Stage Plan W1):
//!
//! - **T00.07**: a `crash-injection` `rikurid` stops at each Stage 00 crash
//!   point (1 before the transaction, 2 after the state write, 3 after
//!   commit, 6 before the response) of each consequential command
//!   (`system.init`, `task.create`, `task.start`, `task.step`,
//!   `task.complete`: 20 cells) and is killed with SIGKILL there. The same
//!   binary restarted on the same home without `RIKURI_CRASH_AT` (unarmed)
//!   shows either no change (points 1, 2) or the whole change (3, 6), with a
//!   clean consistency replay, and the client's retry with the same key
//!   gives exactly one effect: run afresh before the commit, replayed after.
//!   The cells cycle through outcomes and verifications, not only success.
//!   Each stop also checks the progress the daemon printed, so a crash
//!   point that moved fails here (W1 placement, 2026-10-09).
//! - **T00.08**: with idempotency off, the same kill and retry apply the
//!   create twice, and the W1 oracle (the consistency replay) reports it.
//! - **T00.12**: across the crashes and replays, task, run and event IDs in
//!   responses are stable and resolve in state and the ledger, run links
//!   point at the right typed events, the owner and project are the ones
//!   `system.initialized` recorded, owner content stays out of the ledger,
//!   and a missing reference is refused without an effect. (The harness
//!   speaks through `rikuri_api::client`, the CLI's own transport; T00.09
//!   proves the `rikuri` binary gives the same envelopes.)
//!
//! Run by the full gate's `crash-matrix` step, in its own target directory
//! so the test build never replaces the production `target/debug/rikurid`:
//! `cargo test -p rikurid --features crash-injection --test crash_matrix
//! --target-dir target/crash-injection`.
#![cfg(unix)]

use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{self, Receiver};
use std::time::{Duration, Instant};

use rikuri_api::{Request, Response, client, command};
use rikuri_domain::{RunId, SystemIdSource, TaskId};
use rikurid::crash::{CrashPoint, HangAtPoint};
use rikurid::server;
use serde_json::{Value, json};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

const TITLE: &str = "crash matrix owner content";

/// A real `rikurid` process; killed with SIGKILL when dropped.
struct Daemon {
    child: Child,
    lines: Receiver<String>,
    socket: PathBuf,
    /// What it logged up to and including "ready".
    startup: Vec<String>,
}

impl Daemon {
    /// Starts the daemon on `home` with only the given test variables set,
    /// and waits until it says it is ready.
    fn start(home: &Path, env: &[(&str, &str)]) -> TestResult<Daemon> {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_rikurid"));
        cmd.arg("--home")
            .arg(home)
            .env_remove("RIKURI_CRASH_AT")
            .env_remove("RIKURI_TEST_DISABLE_IDEMPOTENCY")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped());
        for (k, v) in env {
            cmd.env(k, v);
        }
        let mut child = cmd.spawn()?;
        let stderr = child.stderr.take().ok_or("no stderr pipe")?;
        let (tx, lines) = mpsc::channel();
        std::thread::spawn(move || {
            for line in BufReader::new(stderr).lines().map_while(Result::ok) {
                if tx.send(line).is_err() {
                    break;
                }
            }
        });
        let mut daemon = Daemon {
            child,
            lines,
            socket: server::socket_path(home),
            startup: vec![],
        };
        daemon.startup = daemon.wait_for("rikurid: ready on")?;
        Ok(daemon)
    }

    /// Waits for a stderr line containing `needle`; returns the lines seen.
    fn wait_for(&self, needle: &str) -> TestResult<Vec<String>> {
        let deadline = Instant::now() + Duration::from_secs(20);
        let mut seen = vec![];
        loop {
            let left = deadline.saturating_duration_since(Instant::now());
            match self.lines.recv_timeout(left) {
                Ok(line) => {
                    let found = line.contains(needle);
                    seen.push(line);
                    if found {
                        return Ok(seen);
                    }
                }
                Err(e) => {
                    return Err(
                        format!("rikurid never logged {needle:?} ({e}); saw {seen:?}").into(),
                    );
                }
            }
        }
    }

    fn send(&self, req: &Request) -> Response {
        client::send(&self.socket, req)
    }

    /// SIGKILL (`Child::kill` on Unix) and reap.
    fn kill9(&mut self) -> TestResult {
        self.child.kill()?;
        self.child.wait()?;
        Ok(())
    }
}

impl Drop for Daemon {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// A scratch home under /tmp: a Unix socket path must stay short (about 104
/// bytes on macOS), and $TMPDIR there is long.
fn scratch() -> TestResult<tempfile::TempDir> {
    Ok(tempfile::Builder::new()
        .prefix("rikuri-crash.")
        .tempdir_in("/tmp")?)
}

fn ok(r: &Response, what: &str) -> TestResult<Value> {
    match (&r.error, &r.result) {
        (None, Some(v)) => Ok(v.clone()),
        _ => Err(format!("{what} failed: {r:?}").into()),
    }
}

fn code(r: &Response) -> String {
    r.error
        .as_ref()
        .map(|e| e.code.as_str().to_owned())
        .unwrap_or_default()
}

fn state(d: &Daemon) -> TestResult<Value> {
    ok(
        &d.send(&Request::new(command::STATE_SHOW, json!({}))),
        "state.show",
    )
}

fn ledger(d: &Daemon) -> TestResult<Vec<Value>> {
    let tail = ok(
        &d.send(&Request::new(command::LEDGER_TAIL, json!({"limit": 1000}))),
        "ledger.tail",
    )?;
    Ok(tail["events"].as_array().cloned().unwrap_or_default())
}

fn health(d: &Daemon) -> TestResult<Value> {
    ok(
        &d.send(&Request::new(command::SYSTEM_HEALTH, json!({}))),
        "system.health",
    )
}

/// The ledger-consistency component of `system.health` (which reruns the
/// replay), and whether writes are quarantined.
fn consistency(d: &Daemon) -> TestResult<(Value, bool)> {
    let h = health(d)?;
    let c = h["components"]
        .as_array()
        .and_then(|cs| cs.iter().find(|c| c["component"] == "ledger_consistency"))
        .cloned()
        .ok_or("system.health has no ledger_consistency component")?;
    Ok((c, h.get("quarantined").is_some()))
}

fn assert_clean(d: &Daemon, at: &str) -> TestResult {
    let (c, quarantined) = consistency(d)?;
    if c["state"] != "healthy" || quarantined {
        return Err(format!("{at}: the consistency replay is not clean: {c}").into());
    }
    Ok(())
}

fn the_task(state: &Value) -> TestResult<Value> {
    match state["tasks"].as_array().map(Vec::as_slice) {
        Some([t]) => Ok(t.clone()),
        other => Err(format!("expected exactly one task, got {other:?}").into()),
    }
}

/// The lifecycle the matrix walks: every consequential command, in order.
const LIFECYCLE: [&str; 5] = [
    command::SYSTEM_INIT,
    command::TASK_CREATE,
    command::TASK_START,
    command::TASK_STEP,
    command::TASK_COMPLETE,
];

/// How a cell's run ends: (outcome, verification).
type Finish = (&'static str, &'static str);

const SUCCESS: Finish = ("succeeded", "passed");

/// The matrix cycles through these so the links are checked for every kind
/// of ending, not only success.
const FINISHES: [Finish; 4] = [
    SUCCESS,
    ("failed", "failed"),
    ("cancelled", "not_run"),
    ("succeeded", "failed"),
];

/// The request for `cmd` given current state, with a fixed key per command
/// so a retry is the same request (`system.init` has its own built-in key).
fn request(cmd: &str, state: &Value, (outcome, verification): Finish) -> TestResult<Request> {
    if cmd == command::SYSTEM_INIT {
        return Ok(Request::new(cmd, json!({})));
    }
    let key = format!("crash-matrix-{}", cmd.replace('.', "-"));
    let t = state["tasks"].get(0).cloned().unwrap_or(Value::Null);
    let params = match cmd {
        command::TASK_CREATE => json!({"title": TITLE}),
        command::TASK_START => json!({"task_id": t["task_id"], "expected_version": t["version"]}),
        command::TASK_STEP => json!({"task_id": t["task_id"], "run_id": t["current_run"],
                                     "expected_version": t["version"]}),
        command::TASK_COMPLETE => json!({"task_id": t["task_id"], "run_id": t["current_run"],
                                         "expected_version": t["version"],
                                         "outcome": outcome, "verification": verification}),
        other => return Err(format!("not a lifecycle command: {other}").into()),
    };
    Ok(Request::new(cmd, params).with_idempotency_key(&key))
}

fn after_commit(point: CrashPoint) -> bool {
    matches!(point, CrashPoint::AfterCommit | CrashPoint::BeforeResponse)
}

/// What the command must have done when the daemon stops at `point`, as
/// the stop marker prints it (Test Strategy §5, written out here rather
/// than taken from the daemon): point 1 nothing; point 2 the state written
/// inside the open transaction, no event; points 3 and 6 committed with its
/// events and, unless idempotency is off, its idempotency record.
fn expected_progress(point: CrashPoint, idempotency: bool) -> String {
    let idem = u8::from(idempotency);
    match point {
        CrashPoint::BeforeTransaction => "tx=0 state=0 events=0 idem=0 committed=0".to_owned(),
        CrashPoint::AfterStateWrite => "tx=1 state=1 events=0 idem=0 committed=0".to_owned(),
        CrashPoint::AfterCommit | CrashPoint::BeforeResponse => {
            format!("tx=0 state=1 events=1 idem={idem} committed=1")
        }
    }
}

/// Sends `req` to a daemon that will stop at `point`, checks that it stopped
/// where the point is defined (its progress), kills it there with SIGKILL,
/// and returns the response the client saw.
fn send_and_kill(
    d: &mut Daemon,
    req: &Request,
    point: CrashPoint,
    idempotency: bool,
) -> TestResult<Response> {
    let socket = d.socket.clone();
    let sent = req.clone();
    let client = std::thread::spawn(move || client::send(&socket, &sent));
    let marker = format!("{} {point} {}", HangAtPoint::MARKER, req.command);
    let seen = d.wait_for(&marker)?;
    d.kill9()?;
    let line = seen.last().cloned().unwrap_or_default();
    let progress = line
        .split_once(&marker)
        .map(|(_, rest)| rest.trim().to_owned())
        .unwrap_or_default();
    let want = expected_progress(point, idempotency);
    if progress != want {
        return Err(format!(
            "{point}@{}: stopped with progress {progress:?}, expected {want:?} (crash point moved?)",
            req.command
        )
        .into());
    }
    client
        .join()
        .map_err(|_| "the client thread panicked".into())
}

/// One matrix cell: the lifecycle with a SIGKILL at `point` of `target`.
/// Returns the final daemon and every response the client kept.
fn crash_cell(
    home: &Path,
    point: CrashPoint,
    target: &str,
    finish: Finish,
) -> TestResult<(Daemon, Vec<Value>)> {
    let at = format!("{point}@{target}");
    let mut d = Daemon::start(home, &[("RIKURI_CRASH_AT", &at)])?;
    let mut results = vec![];
    for cmd in LIFECYCLE {
        let req = request(cmd, &state(&d)?, finish)?;
        if cmd != target {
            results.push(ok(&d.send(&req), cmd)?);
            continue;
        }
        let before_state = state(&d)?;
        let before_events = ledger(&d)?.len();
        let lost = send_and_kill(&mut d, &req, point, true)?;
        if code(&lost) != "daemon_unavailable" {
            return Err(format!("{at}: the client should have lost the answer: {lost:?}").into());
        }

        // restart unarmed (no RIKURI_CRASH_AT): no change, or the whole change
        d = Daemon::start(home, &[])?;
        assert_clean(&d, &format!("{at} after restart"))?;
        let after_events = ledger(&d)?.len();
        let changed = state(&d)? != before_state || after_events != before_events;
        if changed != after_commit(point) {
            return Err(format!(
                "{at}: after the kill, change applied = {changed} (events {before_events} → \
                 {after_events}); expected {}",
                after_commit(point)
            )
            .into());
        }

        // the retry with the same key gives exactly one effect
        let retried = d.send(&req);
        let result = ok(&retried, &format!("{at} retry"))?;
        if retried.replayed != after_commit(point) {
            return Err(format!(
                "{at}: retry replayed = {}, expected {}",
                retried.replayed,
                after_commit(point)
            )
            .into());
        }
        // T00.12: a further duplicate delivery is a stable replay
        let again = d.send(&req);
        if !again.replayed || again.result.as_ref() != Some(&result) {
            return Err(format!("{at}: a second retry is not the same replay: {again:?}").into());
        }
        results.push(result);
    }
    assert_clean(&d, &format!("{at} at the end"))?;
    Ok((d, results))
}

/// T00.12: every ID the client kept resolves in state and the ledger, and
/// the run's links point at the right typed events.
fn check_references(
    d: &Daemon,
    results: &[Value],
    (outcome_given, verification_given): Finish,
    at: &str,
) -> TestResult {
    let s = state(d)?;
    let task = the_task(&s)?;
    let events = ledger(d)?;
    let event = |id: &Value| events.iter().find(|e| &e["event_id"] == id).cloned();
    let [init, created, started, stepped, completed] = results else {
        return Err(format!("{at}: expected five results, got {}", results.len()).into());
    };
    let run = &task["last_run"];
    let fail = |what: &str| -> TestResult { Err(format!("{at}: {what}; task {task}").into()) };

    if task["status"] != "completed" || task["steps_completed"] != 1 || task["runs"] != 1 {
        return fail("the lifecycle did not end with one completed run and one step");
    }
    let initialized = event(&init["event_id"]);
    if s["owner"]["actor_id"] != init["actor_id"]
        || s["owner"]["project_id"] != init["project_id"]
        || s["project"]["project_id"] != init["project_id"]
        || !initialized.is_some_and(|e| {
            e["event_type"] == "system.initialized"
                && e["actor_id"] == init["actor_id"]
                && e["project_id"] == init["project_id"]
        })
    {
        return fail(&format!(
            "the owner and project are not the ones init returned and recorded: {init}; state {s}"
        ));
    }
    for r in &results[1..] {
        if r["task_id"] != task["task_id"] {
            return fail(&format!("a response names another task: {r}"));
        }
    }
    for r in [started, stepped, completed] {
        if r["run_id"] != run["id"] {
            return fail(&format!("a response names another run: {r}"));
        }
    }
    let typed = |id: &Value, kind: &str| {
        event(id).is_some_and(|e| e["event_type"] == kind && e["task_id"] == task["task_id"])
    };
    if !typed(&created["event_id"], "task.created")
        || !typed(&started["event_id"], "run.started")
        || !typed(&stepped["event_id"], "task.stepped")
        || !typed(&completed["event_ids"][0], "run.completed")
        || !typed(&completed["event_ids"][1], "run.verified")
    {
        return fail("a response's event ID does not resolve to its typed ledger event");
    }
    if run["started_event_id"] != started["event_id"]
        || run["completion_event_id"] != completed["event_ids"][0]
        || run["verification_event_id"] != completed["event_ids"][1]
    {
        return fail("the run's links do not point at the events the client was given");
    }
    let outcome = event(&run["completion_event_id"]).map(|e| e["metadata"]["outcome"].clone());
    let verification =
        event(&run["verification_event_id"]).map(|e| e["metadata"]["verification"].clone());
    if run["outcome"] != outcome_given
        || outcome != Some(json!(outcome_given))
        || run["verification"] != verification_given
        || verification != Some(json!(verification_given))
    {
        return fail("outcome or verification differs between the run and the ledger");
    }
    // one event per effect: nothing was applied twice
    let count = |kind: &str| events.iter().filter(|e| e["event_type"] == kind).count();
    for kind in [
        "system.initialized",
        "task.created",
        "run.started",
        "task.stepped",
        "run.completed",
        "run.verified",
    ] {
        if count(kind) != 1 {
            return fail(&format!("{} {kind} events", count(kind)));
        }
    }
    if serde_json::to_string(&events)?.contains(TITLE) {
        return fail("owner content (the task title) reached the ledger");
    }
    Ok(())
}

#[test]
fn t00_07_kill_9_at_every_point_of_every_command_then_one_effect() -> TestResult {
    for (p, &point) in CrashPoint::ALL.iter().enumerate() {
        for (t, target) in LIFECYCLE.into_iter().enumerate() {
            let finish = FINISHES[(p + t) % FINISHES.len()];
            let dir = scratch()?;
            let home = dir.path().join("home");
            let (d, results) = crash_cell(&home, point, target, finish)?;
            check_references(&d, &results, finish, &format!("{point}@{target}"))?;
        }
    }
    Ok(())
}

#[test]
fn t00_08_without_idempotency_the_same_crash_duplicates_and_w1_fails() -> TestResult {
    let dir = scratch()?;
    let home = dir.path().join("home");
    let off = ("RIKURI_TEST_DISABLE_IDEMPOTENCY", "1");
    let point = CrashPoint::AfterCommit;
    let mut d = Daemon::start(
        &home,
        &[("RIKURI_CRASH_AT", &format!("{point}@task.create")), off],
    )?;
    ok(
        &d.send(&Request::new(command::SYSTEM_INIT, json!({}))),
        "init",
    )?;
    let req = request(command::TASK_CREATE, &state(&d)?, SUCCESS)?;
    let lost = send_and_kill(&mut d, &req, point, false)?;
    assert_eq!(code(&lost), "daemon_unavailable", "{lost:?}");

    let d = Daemon::start(&home, &[off])?;
    let retried = d.send(&req);
    ok(&retried, "retry")?;
    assert!(!retried.replayed, "nothing can replay without idempotency");
    let tasks = state(&d)?["tasks"].as_array().map(Vec::len);
    assert_eq!(tasks, Some(2), "the retry applied the create a second time");

    // the oracle reports it, and writes stop
    let (c, quarantined) = consistency(&d)?;
    assert_ne!(c["state"], "healthy", "{c}");
    assert!(
        c["last_error"]
            .as_str()
            .is_some_and(|e| e.contains("duplicate effect")),
        "{c}"
    );
    assert!(quarantined);
    drop(d);

    // and a restarted daemon says so on its log and refuses writes
    let d = Daemon::start(&home, &[])?;
    assert!(
        d.startup.iter().any(|l| l.contains("duplicate effect")),
        "{:?}",
        d.startup
    );
    let refused = d.send(&request(command::TASK_START, &state(&d)?, SUCCESS)?);
    assert_eq!(code(&refused), "quarantined", "{refused:?}");
    Ok(())
}

#[test]
fn t00_12_missing_references_are_refused_without_an_effect() -> TestResult {
    let dir = scratch()?;
    let home = dir.path().join("home");
    let d = Daemon::start(&home, &[])?;
    ok(
        &d.send(&Request::new(command::SYSTEM_INIT, json!({}))),
        "init",
    )?;
    ok(
        &d.send(&request(command::TASK_CREATE, &state(&d)?, SUCCESS)?),
        "create",
    )?;
    ok(
        &d.send(&request(command::TASK_START, &state(&d)?, SUCCESS)?),
        "start",
    )?;
    let task = the_task(&state(&d)?)?;
    let events = ledger(&d)?.len();

    let ids = SystemIdSource::new();
    let no_task = TaskId::generate(&ids)?;
    let no_run = RunId::generate(&ids)?;
    let refused = [
        (
            command::TASK_START,
            json!({"task_id": no_task, "expected_version": 1}),
            "not_found",
        ),
        (
            command::TASK_STEP,
            json!({"task_id": task["task_id"], "run_id": no_run,
                   "expected_version": task["version"]}),
            "not_found",
        ),
        (
            command::TASK_COMPLETE,
            json!({"task_id": task["task_id"], "run_id": no_run,
                   "expected_version": task["version"],
                   "outcome": "succeeded", "verification": "passed"}),
            "not_found",
        ),
        (
            command::TASK_STEP,
            json!({"task_id": no_task, "run_id": task["current_run"], "expected_version": 1}),
            "not_found",
        ),
        (
            command::TASK_COMPLETE,
            json!({"task_id": no_task, "run_id": task["current_run"], "expected_version": 1,
                   "outcome": "succeeded", "verification": "passed"}),
            "not_found",
        ),
        (
            command::TASK_STEP,
            json!({"task_id": task["task_id"], "expected_version": task["version"]}),
            "invalid_request",
        ),
    ];
    for (i, (cmd, params, expected)) in refused.into_iter().enumerate() {
        let r =
            d.send(&Request::new(cmd, params).with_idempotency_key(&format!("missing-ref-{i:04}")));
        assert_eq!(code(&r), expected, "{cmd}: {r:?}");
    }
    assert_eq!(ledger(&d)?.len(), events, "a refused command left an event");
    assert_eq!(
        the_task(&state(&d)?)?,
        task,
        "a refused command changed state"
    );
    assert_clean(&d, "after refusals")?;
    Ok(())
}

/// A `crash-injection` build is a test build even when nothing is armed:
/// health must never call it the production configuration (Module Contract;
/// verifier round 2 found the unarmed build reporting "production
/// configuration", and the gate's crash-matrix step had left that build as
/// `target/debug/rikurid`).
#[test]
fn an_unarmed_crash_injection_build_is_not_reported_as_production() -> TestResult {
    let dir = scratch()?;
    let d = Daemon::start(&dir.path().join("home"), &[])?;
    let h = health(&d)?;
    let build = h["components"]
        .as_array()
        .and_then(|cs| cs.iter().find(|c| c["component"] == "build"))
        .cloned()
        .ok_or("system.health has no build component")?;
    assert_ne!(build["state"], "healthy", "{build}");
    assert!(
        build["detail"]
            .as_str()
            .is_some_and(|d| d.contains("crash-injection")),
        "{build}"
    );
    assert_ne!(h["state"], "healthy", "{h}");
    Ok(())
}

/// Idempotency off outranks crash injection: such a build is Critical,
/// never merely a Degraded test build.
#[test]
fn a_crash_injection_build_without_idempotency_is_critical() -> TestResult {
    let dir = scratch()?;
    let d = Daemon::start(
        &dir.path().join("home"),
        &[("RIKURI_TEST_DISABLE_IDEMPOTENCY", "1")],
    )?;
    let h = health(&d)?;
    let build = h["components"]
        .as_array()
        .and_then(|cs| cs.iter().find(|c| c["component"] == "build"))
        .cloned()
        .ok_or("system.health has no build component")?;
    assert_eq!(build["state"], "critical", "{build}");
    assert!(
        build["detail"]
            .as_str()
            .is_some_and(|d| d.contains("idempotency disabled")),
        "{build}"
    );
    Ok(())
}

/// An armed crash-injection daemon says so: armed outranks "no crash point
/// armed", and both are test builds, never production.
#[test]
fn an_armed_crash_injection_build_says_it_is_armed() -> TestResult {
    let dir = scratch()?;
    let d = Daemon::start(
        &dir.path().join("home"),
        &[("RIKURI_CRASH_AT", "after_commit@task.create")],
    )?;
    let h = health(&d)?;
    let build = h["components"]
        .as_array()
        .and_then(|cs| cs.iter().find(|c| c["component"] == "build"))
        .cloned()
        .ok_or("system.health has no build component")?;
    assert_eq!(build["state"], "degraded", "{build}");
    assert!(
        build["detail"]
            .as_str()
            .is_some_and(|d| d.contains("crash injection armed")),
        "{build}"
    );
    Ok(())
}
