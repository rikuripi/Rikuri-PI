//! T00.09 (Stage Plan): CLI → kernel → repository → ledger as one real
//! path, and the CLI and in-process adapters pass the same JSON and error
//! contract. Two kernels get identical deterministic seams (sequential IDs,
//! a fixed clock): one is served over the real socket to the real `rikuri`
//! binary, the other is called in-process. Every step must give a
//! byte-identical response envelope and the same exit code.
#![cfg(unix)]

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use rikuri_api::{Request, command};
use rikuri_domain::{Clock, IdError, IdSource, Timestamp};
use rikurid::crash::NoCrash;
use rikurid::{Idempotency, Kernel, KernelConfig, server};
use serde_json::{Value, json};
use ulid::Ulid;

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

const T0: u64 = 1_790_000_000_000;

/// IDs 1, 2, 3, … at a fixed time: the same requests give the same IDs.
struct SequentialIds(AtomicU64);
impl IdSource for SequentialIds {
    fn next_ulid(&self) -> Result<Ulid, IdError> {
        Ok(Ulid::from_parts(
            T0,
            u128::from(self.0.fetch_add(1, Ordering::SeqCst)),
        ))
    }
}

struct FixedClock;
impl Clock for FixedClock {
    fn now(&self) -> Timestamp {
        Timestamp(i64::try_from(T0).unwrap_or(i64::MAX))
    }
}

fn deterministic() -> KernelConfig {
    KernelConfig {
        ids: Box::new(SequentialIds(AtomicU64::new(1))),
        clock: Box::new(FixedClock),
        crash: Box::new(NoCrash),
        idempotency: Idempotency::Enforced,
    }
}

/// Serves a deterministic kernel on `socket` from its own thread.
fn start_daemon(home: PathBuf, socket: PathBuf) -> TestResult {
    let ready = socket.clone();
    std::thread::spawn(move || {
        let Ok(kernel) = Kernel::open(&home, deterministic()) else {
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

/// One step: the CLI arguments, the request the CLI should send for them,
/// and optional standard input (for `rikuri raw`).
struct Step {
    args: Vec<String>,
    request: Request,
    stdin: Option<String>,
}

fn step(args: &[&str], request: Request) -> Step {
    Step {
        args: args.iter().map(|a| (*a).to_owned()).collect(),
        request,
        stdin: None,
    }
}

fn raw(envelope: Value) -> TestResult<Step> {
    Ok(Step {
        args: vec!["raw".into()],
        request: serde_json::from_value(envelope.clone())?,
        stdin: Some(envelope.to_string()),
    })
}

/// Runs the real `rikuri` binary; returns its stdout line and exit code.
fn run_cli(socket: &Path, s: &Step) -> TestResult<(String, i32)> {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_rikuri"));
    cmd.arg("--socket").arg(socket).args(&s.args);
    let output = match &s.stdin {
        None => cmd.output()?,
        Some(text) => {
            use std::io::Write;
            let mut child = cmd
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()?;
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(text.as_bytes())?;
            }
            child.wait_with_output()?
        }
    };
    let out = String::from_utf8(output.stdout)?.trim_end().to_owned();
    Ok((out, output.status.code().unwrap_or(-1)))
}

/// Compares one step through both adapters; returns the CLI's response.
fn same(socket: &Path, inproc: &Kernel, s: &Step) -> TestResult<Value> {
    let (cli_out, cli_code) = run_cli(socket, s)?;
    let response = inproc.handle(&s.request);
    let expected = serde_json::to_string(&response)?;
    assert_eq!(
        cli_out,
        expected,
        "envelopes differ for `rikuri {}`",
        s.args.join(" ")
    );
    assert_eq!(
        cli_code,
        i32::from(response.exit_code()),
        "exit codes differ for `rikuri {}`",
        s.args.join(" ")
    );
    Ok(serde_json::from_str(&cli_out)?)
}

fn str_at<'a>(v: &'a Value, field: &str) -> &'a str {
    v["result"][field].as_str().unwrap_or_default()
}

fn u64_at(v: &Value, field: &str) -> u64 {
    v["result"][field].as_u64().unwrap_or_default()
}

#[test]
fn cli_and_in_process_adapters_give_the_same_envelopes() {
    let dir = tempfile::tempdir().unwrap();
    let cli_home = dir.path().join("cli-home");
    let socket = server::socket_path(&cli_home);
    start_daemon(cli_home, socket.clone()).unwrap();
    let inproc = Kernel::open(&dir.path().join("inproc-home"), deterministic()).unwrap();
    let title = "T00.09 marker 5c1e";

    // before init: a read works, a write is refused
    same(
        &socket,
        &inproc,
        &step(
            &["state", "show"],
            Request::new(command::STATE_SHOW, json!({})),
        ),
    )
    .unwrap();
    let early = same(
        &socket,
        &inproc,
        &step(
            &[
                "task",
                "create",
                "--title",
                title,
                "--idempotency-key",
                "key-create-0001",
            ],
            Request::new(command::TASK_CREATE, json!({"title": title}))
                .with_idempotency_key("key-create-0001"),
        ),
    )
    .unwrap();
    assert_eq!(early["error"]["code"], "not_initialized", "{early}");

    same(
        &socket,
        &inproc,
        &step(&["init"], Request::new(command::SYSTEM_INIT, json!({}))),
    )
    .unwrap();

    let create = |t: &str| {
        step(
            &[
                "task",
                "create",
                "--title",
                t,
                "--idempotency-key",
                "key-create-0001",
            ],
            Request::new(command::TASK_CREATE, json!({"title": t}))
                .with_idempotency_key("key-create-0001"),
        )
    };
    let created = same(&socket, &inproc, &create(title)).unwrap();
    assert_eq!(created["ok"], true, "{created}");
    let replay = same(&socket, &inproc, &create(title)).unwrap();
    assert_eq!(replay["replayed"], true, "{replay}");
    let conflict = same(&socket, &inproc, &create("another title")).unwrap();
    assert_eq!(
        conflict["error"]["code"], "idempotency_conflict",
        "{conflict}"
    );
    // `--privacy` reaches the request as `privacy_class`
    let private = same(
        &socket,
        &inproc,
        &step(
            &[
                "task",
                "create",
                "--title",
                "a private one",
                "--privacy",
                "private",
                "--idempotency-key",
                "key-create-0002",
            ],
            Request::new(
                command::TASK_CREATE,
                json!({"title": "a private one", "privacy_class": "private"}),
            )
            .with_idempotency_key("key-create-0002"),
        ),
    )
    .unwrap();
    assert_eq!(private["ok"], true, "{private}");

    let task = str_at(&created, "task_id").to_owned();
    let version = u64_at(&created, "version").to_string();
    let started = same(
        &socket,
        &inproc,
        &step(
            &[
                "task",
                "start",
                "--task",
                &task,
                "--expected-version",
                &version,
                "--idempotency-key",
                "key-start-00001",
            ],
            Request::new(
                command::TASK_START,
                json!({"task_id": task, "expected_version": u64_at(&created, "version")}),
            )
            .with_idempotency_key("key-start-00001"),
        ),
    )
    .unwrap();
    assert_eq!(started["ok"], true, "{started}");
    let run = str_at(&started, "run_id").to_owned();

    let stale = same(
        &socket,
        &inproc,
        &step(
            &[
                "task",
                "start",
                "--task",
                &task,
                "--expected-version",
                "1",
                "--idempotency-key",
                "key-start-00002",
            ],
            Request::new(
                command::TASK_START,
                json!({"task_id": task, "expected_version": 1}),
            )
            .with_idempotency_key("key-start-00002"),
        ),
    )
    .unwrap();
    assert!(stale["ok"] == false, "{stale}");

    let v = u64_at(&started, "version");
    let stepped = same(
        &socket,
        &inproc,
        &step(
            &[
                "task",
                "step",
                "--task",
                &task,
                "--run",
                &run,
                "--expected-version",
                &v.to_string(),
                "--idempotency-key",
                "key-step-000001",
            ],
            Request::new(
                command::TASK_STEP,
                json!({"task_id": task, "run_id": run, "expected_version": v}),
            )
            .with_idempotency_key("key-step-000001"),
        ),
    )
    .unwrap();
    assert_eq!(stepped["ok"], true, "{stepped}");

    let v = u64_at(&stepped, "version");
    let completed = same(
        &socket,
        &inproc,
        &step(
            &[
                "task",
                "complete",
                "--task",
                &task,
                "--run",
                &run,
                "--expected-version",
                &v.to_string(),
                "--outcome",
                "succeeded",
                "--verification",
                "passed",
                "--idempotency-key",
                "key-done-000001",
            ],
            Request::new(
                command::TASK_COMPLETE,
                json!({"task_id": task, "run_id": run, "expected_version": v,
                       "outcome": "succeeded", "verification": "passed"}),
            )
            .with_idempotency_key("key-done-000001"),
        ),
    )
    .unwrap();
    assert_eq!(completed["ok"], true, "{completed}");

    // the CLI path reached the repository and the ledger
    let shown = same(
        &socket,
        &inproc,
        &step(
            &["state", "show"],
            Request::new(command::STATE_SHOW, json!({})),
        ),
    )
    .unwrap();
    assert!(
        shown.to_string().contains(title),
        "state.show lacks the task: {shown}"
    );
    // a limit below the history's length, so `--limit` must reach the request
    let last3 = same(
        &socket,
        &inproc,
        &step(
            &["ledger", "tail", "--limit", "3"],
            Request::new(command::LEDGER_TAIL, json!({"limit": 3})),
        ),
    )
    .unwrap();
    assert_eq!(
        last3["result"]["events"].as_array().map(Vec::len),
        Some(3),
        "{last3}"
    );
    // the default limit (20) covers the whole history
    let tail = same(
        &socket,
        &inproc,
        &step(
            &["ledger", "tail"],
            Request::new(command::LEDGER_TAIL, json!({"limit": 20})),
        ),
    )
    .unwrap();
    let events = tail["result"]["events"]
        .as_array()
        .map(Vec::len)
        .unwrap_or_default();
    assert!(
        events >= 7,
        "expected init, two creates, start, step, completion and verification events: {tail}"
    );
    assert!(
        !tail.to_string().contains(title),
        "the title leaked into the ledger: {tail}"
    );
    let health = same(
        &socket,
        &inproc,
        &step(
            &["system", "health"],
            Request::new(command::SYSTEM_HEALTH, json!({})),
        ),
    )
    .unwrap();
    assert_eq!(health["result"]["state"], "healthy", "{health}");

    // the error contract through `rikuri raw`
    for envelope in [
        json!({"v": 1, "command": "no.such.command", "params": {}}),
        json!({"v": 2, "command": "state.show", "params": {}}),
        json!({"v": 1, "command": "ledger.tail", "params": {"limit": 5, "grant": "all"}}),
        json!({"v": 1, "command": "task.create", "params": {"title": "no key"}}),
    ] {
        let r = same(&socket, &inproc, &raw(envelope.clone()).unwrap()).unwrap();
        assert_eq!(r["ok"], false, "{envelope} was accepted: {r}");
    }
}
