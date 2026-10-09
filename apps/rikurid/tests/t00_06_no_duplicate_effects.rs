//! T00.06 (Stage Plan): no duplicate consequential effect across randomized
//! command sequences with injected crashes. Each command may crash in-process
//! at one of the kernel's crash points (1, 2, 3, 6); the client then retries
//! with the same idempotency key, and sometimes re-sends a request that
//! already succeeded. Afterwards there is exactly one task per create key that
//! succeeded, as many `task.created` events as tasks, and the
//! ledger-consistency replay (W1's oracle) finds nothing wrong.
#![cfg(unix)]

use std::collections::BTreeSet;
use std::path::Path;
use std::sync::{Arc, Mutex};

use proptest::prelude::*;
use rikuri_api::{Request, Response, command};
use rikuri_domain::{SystemClock, SystemIdSource};
use rikurid::crash::{CrashHook, CrashPoint, Crashed, Progress};
use rikurid::{Idempotency, Kernel, KernelConfig};
use serde_json::{Value, json};

/// Crashes the next consequential command at the armed point, once.
#[derive(Clone, Default)]
struct ArmedOnce(Arc<Mutex<Option<CrashPoint>>>);

impl ArmedOnce {
    fn arm(&self, point: Option<CrashPoint>) {
        if let Ok(mut p) = self.0.lock() {
            *p = point;
        }
    }
}

impl CrashHook for ArmedOnce {
    fn at(&self, point: CrashPoint, _command: &str, _: Progress) -> Result<(), Crashed> {
        match self.0.lock() {
            Ok(mut armed) if *armed == Some(point) => {
                *armed = None;
                Err(Crashed { point })
            }
            _ => Ok(()),
        }
    }
    fn is_armed(&self) -> bool {
        true
    }
}

fn open(
    home: &Path,
    crash: ArmedOnce,
    idempotency: Idempotency,
) -> Result<Kernel, rikurid::KernelError> {
    Kernel::open(
        home,
        KernelConfig {
            ids: Box::new(SystemIdSource::new()),
            clock: Box::new(SystemClock),
            crash: Box::new(crash),
            idempotency,
        },
    )
}

#[derive(Debug, Clone)]
enum Op {
    /// `system.init` (its own built-in key; only the every-point test uses it,
    /// since a home is initialized once).
    Init,
    /// `task.create` with key `key` and title `title` (a reused key with
    /// another title is a conflict, not a new effect).
    Create {
        key: u8,
        title: u8,
    },
    Start {
        task: u8,
    },
    Step {
        task: u8,
    },
    Complete {
        task: u8,
    },
}

fn op() -> impl Strategy<Value = Op> {
    prop_oneof![
        3 => (0u8..4, 0u8..2).prop_map(|(key, title)| Op::Create { key, title }),
        2 => any::<u8>().prop_map(|task| Op::Start { task }),
        2 => any::<u8>().prop_map(|task| Op::Step { task }),
        2 => any::<u8>().prop_map(|task| Op::Complete { task }),
    ]
}

fn crash_point() -> impl Strategy<Value = Option<CrashPoint>> {
    prop_oneof![
        2 => Just(None),
        1 => proptest::sample::select(CrashPoint::ALL.to_vec()).prop_map(Some),
    ]
}

fn is_simulated_crash(r: &Response) -> bool {
    r.error
        .as_ref()
        .is_some_and(|e| e.code.as_str() == "daemon_unavailable")
}

fn state(k: &Kernel) -> Value {
    k.handle(&Request::new(command::STATE_SHOW, json!({})))
        .result
        .unwrap_or(Value::Null)
}

/// The request for `op`, given current state; `None` if it cannot be built
/// (for example, no task exists yet).
fn request(op: &Op, state: &Value, serial: usize) -> Option<Request> {
    if let Op::Init = op {
        return Some(Request::new(command::SYSTEM_INIT, json!({})));
    }
    let tasks = state["tasks"].as_array()?;
    let pick = |i: u8| -> Option<&Value> {
        (!tasks.is_empty()).then(|| &tasks[usize::from(i) % tasks.len()])
    };
    let key = |kind: &str| format!("key-{kind}-{serial:06}");
    Some(match op {
        Op::Init => return None,
        Op::Create { key: k, title } => Request::new(
            command::TASK_CREATE,
            json!({"title": format!("title {title}")}),
        )
        .with_idempotency_key(&format!("key-create-{k:04}")),
        Op::Start { task } => {
            let t = pick(*task)?;
            Request::new(
                command::TASK_START,
                json!({"task_id": t["task_id"], "expected_version": t["version"]}),
            )
            .with_idempotency_key(&key("start"))
        }
        Op::Step { task } => {
            let t = pick(*task)?;
            Request::new(
                command::TASK_STEP,
                json!({"task_id": t["task_id"], "run_id": t["current_run"],
                       "expected_version": t["version"]}),
            )
            .with_idempotency_key(&key("step"))
        }
        Op::Complete { task } => {
            let t = pick(*task)?;
            Request::new(
                command::TASK_COMPLETE,
                json!({"task_id": t["task_id"], "run_id": t["current_run"],
                       "expected_version": t["version"],
                       "outcome": "succeeded", "verification": "passed"}),
            )
            .with_idempotency_key(&key("done"))
        }
    })
}

/// Runs the sequence; returns (create keys that succeeded, final state,
/// consistency problems, task.created events in the ledger).
fn run_sequence(
    k: &Kernel,
    crash: &ArmedOnce,
    steps: &[(Op, Option<CrashPoint>, bool)],
) -> (BTreeSet<String>, Value, Vec<String>, usize) {
    let mut created_keys = BTreeSet::new();
    for (serial, (op, point, resend)) in steps.iter().enumerate() {
        let Some(req) = request(op, &state(k), serial) else {
            continue;
        };
        crash.arm(*point);
        let mut response = k.handle(&req);
        crash.arm(None);
        if is_simulated_crash(&response) {
            // the client never saw an answer: retry with the same key
            response = k.handle(&req);
        }
        if *resend && response.error.is_none() {
            // a duplicate delivery of a request that already succeeded
            let again = k.handle(&req);
            assert!(
                again.replayed && again.result == response.result,
                "{again:?}"
            );
        }
        if response.error.is_none() && req.command == command::TASK_CREATE {
            created_keys.insert(req.idempotency_key.clone().unwrap_or_default());
        }
    }
    let report = k.check_consistency();
    let tail = k
        .handle(&Request::new(command::LEDGER_TAIL, json!({"limit": 1000})))
        .result
        .unwrap_or(Value::Null);
    let created_events = tail["events"]
        .as_array()
        .map(|es| {
            es.iter()
                .filter(|e| e["event_type"] == "task.created")
                .count()
        })
        .unwrap_or_default();
    (created_keys, state(k), report.problems, created_events)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    #[test]
    fn crashes_and_retries_never_duplicate_an_effect(
        steps in proptest::collection::vec((op(), crash_point(), any::<bool>()), 1..30)
    ) {
        let dir = tempfile::tempdir().unwrap();
        let crash = ArmedOnce::default();
        let k = open(&dir.path().join("home"), crash.clone(), Idempotency::Enforced).unwrap();
        k.handle(&Request::new(command::SYSTEM_INIT, json!({})));

        let (keys, state, problems, created_events) = run_sequence(&k, &crash, &steps);
        let tasks = state["tasks"].as_array().map(Vec::len).unwrap_or_default();
        prop_assert!(problems.is_empty(), "consistency problems: {:?}", problems);
        prop_assert_eq!(tasks, keys.len(), "one task per create key that succeeded");
        prop_assert_eq!(created_events, tasks, "one task.created event per task");
    }
}

/// Every crash point of every consequential command (`system.init` and the
/// task lifecycle), deterministically: the
/// retry with the same key succeeds, runs the command afresh when the crash
/// came before the commit (points 1 and 2) and replays it after (points 3 and
/// 6), and the replay finds nothing wrong.
#[test]
fn every_point_of_every_command_recovers_with_one_effect() {
    let lifecycle = [
        (command::SYSTEM_INIT, Op::Init),
        (command::TASK_CREATE, Op::Create { key: 0, title: 0 }),
        (command::TASK_START, Op::Start { task: 0 }),
        (command::TASK_STEP, Op::Step { task: 0 }),
        (command::TASK_COMPLETE, Op::Complete { task: 0 }),
    ];
    for &point in CrashPoint::ALL {
        let dir = tempfile::tempdir().unwrap();
        let crash = ArmedOnce::default();
        let k = open(
            &dir.path().join("home"),
            crash.clone(),
            Idempotency::Enforced,
        )
        .unwrap();
        for (serial, (name, op)) in lifecycle.iter().enumerate() {
            let req = request(op, &state(&k), serial).unwrap();
            crash.arm(Some(point));
            let crashed = k.handle(&req);
            assert!(
                is_simulated_crash(&crashed),
                "{name} at {point}: {crashed:?}"
            );
            let retried = k.handle(&req);
            assert!(retried.error.is_none(), "{name} at {point}: {retried:?}");
            let after_commit =
                matches!(point, CrashPoint::AfterCommit | CrashPoint::BeforeResponse);
            assert_eq!(retried.replayed, after_commit, "{name} at {point}");
            let problems = k.check_consistency().problems;
            assert!(problems.is_empty(), "{name} at {point}: {problems:?}");
        }
        let s = state(&k);
        assert_eq!(s["initialized"], true, "at {point}");
        let tasks = s["tasks"].as_array().unwrap();
        assert_eq!(tasks.len(), 1, "at {point}");
        assert_eq!(tasks[0]["steps_completed"], 1, "at {point}");
        assert_eq!(tasks[0]["runs"], 1, "at {point}");
        assert_eq!(tasks[0]["last_run"]["outcome"], "succeeded", "at {point}");
    }
}

/// The oracle bites (the in-process twin of T00.08): with idempotency off, a
/// crash after commit and a retry apply the create twice, and the same
/// checks report it.
#[test]
fn without_idempotency_the_oracle_reports_the_duplicate() {
    let dir = tempfile::tempdir().unwrap();
    let crash = ArmedOnce::default();
    let k = open(
        &dir.path().join("home"),
        crash.clone(),
        Idempotency::DisabledForRegressionTest,
    )
    .unwrap();
    k.handle(&Request::new(command::SYSTEM_INIT, json!({})));
    let steps = [(
        Op::Create { key: 0, title: 0 },
        Some(CrashPoint::AfterCommit),
        false,
    )];
    let (keys, state, problems, created_events) = run_sequence(&k, &crash, &steps);
    let tasks = state["tasks"].as_array().map(Vec::len).unwrap_or_default();
    assert_eq!(keys.len(), 1);
    assert_eq!(tasks, 2, "the retry applied the create again");
    assert_eq!(created_events, 2);
    assert!(
        problems.iter().any(|p| p.contains("duplicate effect")),
        "the oracle missed the duplicate: {problems:?}"
    );
}

/// A kernel with idempotency off is a regression-test build and can never
/// pass for production: its build component is Critical.
#[test]
fn a_kernel_without_idempotency_reports_its_build_critical() {
    let dir = tempfile::tempdir().unwrap();
    let k = open(
        &dir.path().join("home"),
        ArmedOnce::default(),
        Idempotency::DisabledForRegressionTest,
    )
    .unwrap();
    let h = k
        .handle(&Request::new(command::SYSTEM_HEALTH, json!({})))
        .result
        .unwrap();
    let build = h["components"]
        .as_array()
        .and_then(|cs| cs.iter().find(|c| c["component"] == "build"))
        .cloned()
        .unwrap();
    assert_eq!(build["state"], "critical", "{build}");
    assert!(
        build["detail"]
            .as_str()
            .is_some_and(|d| d.contains("idempotency disabled")),
        "{build}"
    );
}
