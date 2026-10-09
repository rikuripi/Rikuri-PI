//! Defects found when S00.07 was first run (2026-10-08,
//! docs/build-records/2026-10-08-s00-07-rikurid.md). Calls that could hang
//! run on their own thread with a deadline, so a deadlock fails the test
//! instead of hanging the suite.
#![cfg(unix)]

use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use rikuri_api::{Request, Response, command};
use rikuri_domain::{SystemClock, SystemIdSource};
use rikurid::crash::NoCrash;
use rikurid::{Idempotency, Kernel, KernelConfig, server};
use serde_json::json;

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

const DEADLINE: Duration = Duration::from_secs(10);
const MARKER: &str = "Marker title 7f3a";

fn config() -> KernelConfig {
    KernelConfig {
        ids: Box::new(SystemIdSource::new()),
        clock: Box::new(SystemClock),
        crash: Box::new(NoCrash),
        idempotency: Idempotency::Enforced,
    }
}

/// init → task.create (with the marker title) → state.show.
fn slice() -> Vec<Request> {
    vec![
        Request::new(command::SYSTEM_INIT, json!({})),
        Request::new(command::TASK_CREATE, json!({ "title": MARKER }))
            .with_idempotency_key("key-create-0001"),
        Request::new(command::STATE_SHOW, json!({})),
    ]
}

/// Runs `requests` against a fresh kernel on its own thread. `None` if not
/// every response arrived by the deadline.
fn in_process(home: PathBuf, requests: Vec<Request>) -> Option<Vec<Response>> {
    let n = requests.len();
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let Ok(kernel) = Kernel::open(&home, config()) else {
            return;
        };
        for req in &requests {
            if tx.send(kernel.handle(req)).is_err() {
                return;
            }
        }
    });
    (0..n).map(|_| rx.recv_timeout(DEADLINE).ok()).collect()
}

/// Serves a fresh kernel on `socket` from its own thread and waits until the
/// socket exists.
fn start_daemon(home: PathBuf, socket: PathBuf) -> TestResult {
    let ready = socket.clone();
    std::thread::spawn(move || {
        let Ok(kernel) = Kernel::open(&home, config()) else {
            return;
        };
        let Ok(listener) = server::bind(&socket) else {
            return;
        };
        let _ = server::serve(&kernel, &listener, &mut std::io::sink());
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

/// Sends `requests` in order through the real client. `None` if not every
/// response arrived by the deadline.
fn through_socket(socket: PathBuf, requests: Vec<Request>) -> Option<Vec<Response>> {
    let n = requests.len();
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        for req in &requests {
            if tx.send(rikuri_api::client::send(&socket, req)).is_err() {
                return;
            }
        }
    });
    (0..n).map(|_| rx.recv_timeout(DEADLINE).ok()).collect()
}

fn assert_slice_answers(responses: Option<Vec<Response>>, how: &str) {
    let Some(responses) = responses else {
        panic!("{how}: a request got no response within {DEADLINE:?} (deadlock?)");
    };
    for r in &responses {
        assert!(r.error.is_none(), "{how}: {r:?}");
    }
    let shown = responses
        .last()
        .and_then(|r| r.result.as_ref())
        .map(|v| v.to_string())
        .unwrap_or_default();
    assert!(
        shown.contains(MARKER),
        "{how}: state.show lacks the task: {shown}"
    );
    assert!(shown.contains("\"initialized\":true"), "{how}: {shown}");
}

fn mode(p: &Path) -> TestResult<u32> {
    Ok(std::fs::metadata(p)?.permissions().mode() & 0o777)
}

fn set_mode(p: &Path, m: u32) -> TestResult {
    std::fs::set_permissions(p, std::fs::Permissions::from_mode(m))?;
    Ok(())
}

/// Defect A: state.show re-entered the store's connection lock and hung the
/// daemon.
#[test]
fn state_show_answers_in_process() {
    let dir = tempfile::tempdir().unwrap();
    assert_slice_answers(in_process(dir.path().join("home"), slice()), "in-process");
}

/// Defect A through the real entry point: socket server and client.
#[test]
fn state_show_answers_through_the_socket() {
    let dir = tempfile::tempdir().unwrap();
    let home = dir.path().join("home");
    let socket = server::socket_path(&home);
    start_daemon(home, socket.clone()).unwrap();
    assert_slice_answers(through_socket(socket.clone(), slice()), "socket");
    // the daemon still answers afterwards
    let again = through_socket(
        socket,
        vec![Request::new(command::SYSTEM_HEALTH, json!({}))],
    );
    assert!(
        again.is_some_and(|r| r[0].error.is_none()),
        "daemon stopped answering after state.show"
    );
}

/// Defect B: an existing socket directory that others can access is
/// refused, as an existing data directory is.
#[test]
fn a_shared_socket_directory_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let run = dir.path().join("shared-run");
    std::fs::create_dir(&run).unwrap();
    set_mode(&run, 0o755).unwrap();
    let socket = run.join("rikurid.sock");
    match server::bind(&socket) {
        Err(e) => assert_eq!(e.kind(), std::io::ErrorKind::PermissionDenied, "{e}"),
        Ok(_) => panic!("bound a socket in a directory others can access"),
    }
    assert!(
        !socket.exists(),
        "nothing may be created in a refused directory"
    );
}

/// Legitimate use: a fresh socket directory is created owner-only and the
/// socket itself is owner-only.
#[test]
fn socket_and_its_directory_are_owner_only() {
    let dir = tempfile::tempdir().unwrap();
    let run = dir.path().join("run");
    let socket = run.join("rikurid.sock");
    let _listener = server::bind(&socket).unwrap();
    assert_eq!(mode(&run).unwrap(), 0o700, "socket directory");
    assert_eq!(mode(&socket).unwrap() & 0o077, 0, "socket");
}
