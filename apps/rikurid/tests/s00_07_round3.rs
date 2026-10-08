//! Round 3 of S00.07 (2026-10-08, docs/build-records/2026-10-08-s00-07-rikurid.md):
//! the quarantine false green found by the second independent verification,
//! the owner's decision that a quarantined daemon still replays committed
//! requests, and coverage for mutants that survived the suite.
#![cfg(unix)]

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use rikuri_api::{Request, Response, command};
use rikuri_domain::{IdError, IdSource, SystemClock, SystemIdSource};
use rikurid::crash::NoCrash;
use rikurid::kernel::database_path;
use rikurid::{Idempotency, Kernel, KernelConfig, server};
use serde_json::{Value, json};
use ulid::Ulid;

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

const DEADLINE: Duration = Duration::from_secs(10);

/// System IDs until `failing` is set, then `SourceUnavailable`.
struct SwitchableIds {
    inner: SystemIdSource,
    failing: Arc<AtomicBool>,
}

impl IdSource for SwitchableIds {
    fn next_ulid(&self) -> Result<Ulid, IdError> {
        if self.failing.load(Ordering::SeqCst) {
            Err(IdError::SourceUnavailable(
                "switched off by the test".into(),
            ))
        } else {
            self.inner.next_ulid()
        }
    }
}

fn open_with(home: &Path, ids: Box<dyn IdSource>) -> Result<Kernel, rikurid::KernelError> {
    Kernel::open(
        home,
        KernelConfig {
            ids,
            clock: Box::new(SystemClock),
            crash: Box::new(NoCrash),
            idempotency: Idempotency::Enforced,
        },
    )
}

fn open(home: &Path) -> Result<Kernel, rikurid::KernelError> {
    open_with(home, Box::new(SystemIdSource::new()))
}

fn call(k: &Kernel, command: &str, params: Value) -> Response {
    k.handle(&Request::new(command, params))
}

fn call_keyed(k: &Kernel, command: &str, params: Value, key: &str) -> Response {
    k.handle(&Request::new(command, params).with_idempotency_key(key))
}

fn code(r: &Response) -> String {
    r.error
        .as_ref()
        .map(|e| e.code.as_str().to_owned())
        .unwrap_or_default()
}

fn health(k: &Kernel) -> Value {
    call(k, command::SYSTEM_HEALTH, json!({}))
        .result
        .unwrap_or(Value::Null)
}

fn component<'a>(health: &'a Value, name: &str) -> &'a Value {
    health["components"]
        .as_array()
        .and_then(|cs| cs.iter().find(|c| c["component"] == name))
        .unwrap_or(&Value::Null)
}

fn home(dir: &tempfile::TempDir) -> PathBuf {
    dir.path().join("home")
}

/// Sets every task's `steps_completed` behind the kernel's back, so state
/// and history disagree (value 5) or agree again (value 0).
fn set_task_steps(home: &Path, steps: u32) -> TestResult {
    let raw = rikuri_storage::rusqlite::Connection::open(database_path(home))?;
    raw.execute(
        "UPDATE objects SET body = json_set(body, '$.steps_completed', ?1) WHERE kind = 'task'",
        [steps],
    )?;
    Ok(())
}

/// Important (round 2): while quarantined, health is never green, even
/// after the cause has gone away; the quarantine is sticky.
#[test]
fn quarantine_keeps_health_critical_after_the_cause_clears() {
    let dir = tempfile::tempdir().unwrap();
    let k = open(&home(&dir)).unwrap();
    call(&k, command::SYSTEM_INIT, json!({}));
    let created = call_keyed(
        &k,
        command::TASK_CREATE,
        json!({"title": "a"}),
        "key-create-0001",
    );
    assert!(created.error.is_none(), "{created:?}");

    set_task_steps(&home(&dir), 5).unwrap();
    let h = health(&k);
    assert_eq!(h["state"], "critical", "disagreement not reported: {h}");

    set_task_steps(&home(&dir), 0).unwrap();
    let h = health(&k);
    assert_eq!(
        h["state"], "critical",
        "green while writes are refused: {h}"
    );
    assert!(h["quarantined"].is_string(), "{h}");
    let write = call_keyed(
        &k,
        command::TASK_CREATE,
        json!({"title": "b"}),
        "key-create-0002",
    );
    assert_eq!(
        code(&write),
        "quarantined",
        "quarantine not sticky: {write:?}"
    );
}

/// Owner decision 2026-10-08: a quarantined daemon still answers a retry of
/// a request it already committed (a replay only reads the stored
/// response); a new or changed request is refused with `quarantined`.
#[test]
fn a_quarantined_daemon_still_replays_committed_requests() {
    let dir = tempfile::tempdir().unwrap();
    let k = open(&home(&dir)).unwrap();
    call(&k, command::SYSTEM_INIT, json!({}));
    let first = call_keyed(
        &k,
        command::TASK_CREATE,
        json!({"title": "a"}),
        "key-create-0001",
    );
    assert!(first.error.is_none(), "{first:?}");

    set_task_steps(&home(&dir), 5).unwrap();
    assert_eq!(health(&k)["state"], "critical");

    let replay = call_keyed(
        &k,
        command::TASK_CREATE,
        json!({"title": "a"}),
        "key-create-0001",
    );
    assert!(replay.error.is_none() && replay.replayed, "{replay:?}");
    assert_eq!(replay.result, first.result);
    let changed = call_keyed(
        &k,
        command::TASK_CREATE,
        json!({"title": "b"}),
        "key-create-0001",
    );
    assert_eq!(code(&changed), "quarantined", "{changed:?}");
    let new = call_keyed(
        &k,
        command::TASK_CREATE,
        json!({"title": "c"}),
        "key-create-0003",
    );
    assert_eq!(code(&new), "quarantined", "{new:?}");
}

/// An internal kernel failure inside a write transaction counts against
/// the store's health (it must never stay green while every write fails).
#[test]
fn internal_failures_inside_writes_are_not_green() {
    let dir = tempfile::tempdir().unwrap();
    let failing = Arc::new(AtomicBool::new(false));
    let ids = SwitchableIds {
        inner: SystemIdSource::new(),
        failing: Arc::clone(&failing),
    };
    let k = open_with(&home(&dir), Box::new(ids)).unwrap();
    call(&k, command::SYSTEM_INIT, json!({}));
    failing.store(true, Ordering::SeqCst);
    let r = call_keyed(
        &k,
        command::TASK_CREATE,
        json!({"title": "a"}),
        "key-create-0001",
    );
    assert_eq!(code(&r), "internal", "{r:?}");
    let h = health(&k);
    assert_ne!(component(&h, "store")["state"], "healthy", "{h}");
}

/// A `Write` that keeps what the daemon logs.
#[derive(Clone, Default)]
struct SharedLog(Arc<Mutex<Vec<u8>>>);

impl Write for SharedLog {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Ok(mut v) = self.0.lock() {
            v.extend_from_slice(buf);
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl SharedLog {
    fn text(&self) -> String {
        self.0
            .lock()
            .map(|v| String::from_utf8_lossy(&v).into_owned())
            .unwrap_or_default()
    }
}

/// Serves a fresh kernel on `socket` with `log`; waits for the socket.
fn serve_with_log(home: PathBuf, socket: PathBuf, log: SharedLog) -> TestResult {
    let ready = socket.clone();
    std::thread::spawn(move || {
        let Ok(kernel) = open(&home) else {
            return;
        };
        let Ok(listener) = server::bind(&socket) else {
            return;
        };
        let mut log = log;
        let _ = server::serve(&kernel, &listener, &mut log);
    });
    let start = Instant::now();
    while !ready.exists() {
        if start.elapsed() > DEADLINE {
            return Err("daemon socket never appeared".into());
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    Ok(())
}

/// Sends one raw line and returns the raw response line.
fn exchange(socket: &Path, line: &[u8]) -> TestResult<String> {
    let mut s = UnixStream::connect(socket)?;
    s.set_read_timeout(Some(DEADLINE))?;
    s.write_all(line)?;
    let mut answer = String::new();
    BufReader::new(s).read_line(&mut answer)?;
    Ok(answer)
}

/// The daemon's log cannot be forged through the command name, records
/// requests refused before they parse, and an envelope parse error is
/// bounded.
#[test]
fn the_log_is_one_line_per_request_and_errors_stay_bounded() {
    let dir = tempfile::tempdir().unwrap();
    let socket = server::socket_path(&home(&dir));
    let log = SharedLog::default();
    serve_with_log(home(&dir), socket.clone(), log.clone()).unwrap();

    let forged = br#"{"v":1,"command":"x\nrikurid: task.create -> ok","params":{}}"#;
    let mut line = forged.to_vec();
    line.push(b'\n');
    exchange(&socket, &line).unwrap();
    exchange(&socket, b"not json at all\n").unwrap();
    let mut huge = b"{\"v\":\"".to_vec();
    huge.extend(std::iter::repeat_n(b'9', 60_000));
    huge.extend_from_slice(b"\"}\n");
    let answer = exchange(&socket, &huge).unwrap();
    assert!(
        answer.len() < 1024,
        "envelope error is {} bytes",
        answer.len()
    );

    let text = log.text();
    assert!(
        !text.lines().any(|l| l == "rikurid: task.create -> ok"),
        "a forged log line appeared: {text}"
    );
    assert!(
        text.contains("(unparsed request) -> invalid_request"),
        "refused-before-parsing request not logged: {text}"
    );
    assert_eq!(text.lines().count(), 3, "one line per request: {text}");
}
