//! The CLI's documented exit codes that T00.09's parity run never reaches
//! (2026-10-08): 2 for a usage error, 4 when no daemon answers, 5 when the
//! daemon refuses to write (here: quarantined).
#![cfg(unix)]

use std::path::PathBuf;
use std::process::{Command, Output};
use std::time::{Duration, Instant};

use rikuri_api::{Request, command};
use rikuri_domain::{SystemClock, SystemIdSource};
use rikurid::crash::NoCrash;
use rikurid::{Idempotency, Kernel, KernelConfig, server};
use serde_json::{Value, json};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn config(idempotency: Idempotency) -> KernelConfig {
    KernelConfig {
        ids: Box::new(SystemIdSource::new()),
        clock: Box::new(SystemClock),
        crash: Box::new(NoCrash),
        idempotency,
    }
}

fn rikuri(args: &[&str]) -> TestResult<Output> {
    Ok(Command::new(env!("CARGO_BIN_EXE_rikuri"))
        .args(args)
        .output()?)
}

fn envelope(out: &Output) -> TestResult<Value> {
    Ok(serde_json::from_slice(&out.stdout)?)
}

#[test]
fn a_usage_error_exits_2() {
    let out = rikuri(&["task", "create"]).unwrap(); // --title is required
    assert_eq!(out.status.code(), Some(2), "{out:?}");
}

#[test]
fn no_daemon_exits_4_with_daemon_unavailable() {
    let dir = tempfile::tempdir().unwrap();
    let socket = dir.path().join("nobody-listens.sock");
    let socket = socket.to_str().unwrap();
    let out = rikuri(&["--socket", socket, "state", "show"]).unwrap();
    assert_eq!(out.status.code(), Some(4), "{out:?}");
    assert_eq!(
        envelope(&out).unwrap()["error"]["code"],
        "daemon_unavailable"
    );
}

/// Serves a home whose state and history disagree, so the daemon starts
/// quarantined; waits for the socket.
fn serve_quarantined(home: PathBuf, socket: PathBuf) -> TestResult {
    {
        // the T00.08 shape: idempotency off, the same key applied twice
        let k = Kernel::open(&home, config(Idempotency::DisabledForRegressionTest))?;
        k.handle(&Request::new(command::SYSTEM_INIT, json!({})));
        for _ in 0..2 {
            k.handle(
                &Request::new(command::TASK_CREATE, json!({"title": "a"}))
                    .with_idempotency_key("key-dup-000001"),
            );
        }
    }
    let ready = socket.clone();
    std::thread::spawn(move || {
        let Ok(kernel) = Kernel::open(&home, config(Idempotency::Enforced)) else {
            return;
        };
        let Ok(listener) = server::bind(&socket) else {
            return;
        };
        let _ = server::serve(&kernel, &listener, &mut std::io::sink());
    });
    let start = Instant::now();
    while !ready.exists() {
        if start.elapsed() > Duration::from_secs(10) {
            return Err("daemon socket never appeared".into());
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    Ok(())
}

#[test]
fn a_refused_write_exits_5() {
    let dir = tempfile::tempdir().unwrap();
    let home = dir.path().join("home");
    let socket = server::socket_path(&home);
    serve_quarantined(home, socket.clone()).unwrap();
    let socket = socket.to_str().unwrap();
    let out = rikuri(&[
        "--socket",
        socket,
        "task",
        "create",
        "--title",
        "b",
        "--idempotency-key",
        "key-create-0002",
    ])
    .unwrap();
    assert_eq!(out.status.code(), Some(5), "{out:?}");
    assert_eq!(envelope(&out).unwrap()["error"]["code"], "quarantined");
}
