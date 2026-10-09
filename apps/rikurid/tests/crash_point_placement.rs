//! W1: each crash point sits where Test Strategy §5 puts it. Recovery is
//! proven per point by T00.06 and T00.07, but only where the point actually
//! is: points 1 and 2 both expect "no change", so moving point 2 before the
//! state write, or point 1 into the transaction, left every test green
//! (PR #21 verifier, mutants W03–W05). The kernel now tells the hook what
//! the command has done so far, read from facts when the point is reached,
//! and this test holds each point to its definition. The real daemon's stop marker carries the
//! same progress, and the crash matrix checks it per cell.
#![cfg(unix)]

use rikuri_api::{Request, command};
use rikuri_domain::{SystemClock, SystemIdSource};
use rikuri_testkit::CrashInjector;
use rikurid::crash::{CrashPoint, Progress};
use rikurid::{Idempotency, Kernel, KernelConfig};
use serde_json::{Value, json};

/// Test Strategy §5, written out here rather than taken from the kernel:
/// point 1 before the transaction; point 2 after the state write, before
/// any event; point 3 after the commit; point 6 after the idempotency
/// record, before the response.
fn expected(point: CrashPoint, idempotency: bool) -> Progress {
    match point {
        CrashPoint::BeforeTransaction => Progress::default(),
        CrashPoint::AfterStateWrite => Progress {
            transaction_open: true,
            state_written: true,
            ..Progress::default()
        },
        CrashPoint::AfterCommit | CrashPoint::BeforeResponse => Progress {
            transaction_open: false,
            state_written: true,
            events_appended: true,
            idempotency_recorded: idempotency,
            committed: true,
        },
    }
}

fn ok(k: &Kernel, req: &Request) -> Value {
    let r = k.handle(req);
    assert!(r.error.is_none(), "{} failed: {r:?}", req.command);
    r.result.unwrap_or(Value::Null)
}

/// Every consequential command once, in order.
fn lifecycle(k: &Kernel) {
    ok(k, &Request::new(command::SYSTEM_INIT, json!({})));
    let t = ok(
        k,
        &Request::new(command::TASK_CREATE, json!({"title": "t"}))
            .with_idempotency_key("key-create-0001"),
    );
    let t = ok(
        k,
        &Request::new(
            command::TASK_START,
            json!({"task_id": t["task_id"], "expected_version": t["version"]}),
        )
        .with_idempotency_key("key-start-00001"),
    );
    let t = ok(
        k,
        &Request::new(
            command::TASK_STEP,
            json!({"task_id": t["task_id"], "run_id": t["run_id"],
                   "expected_version": t["version"]}),
        )
        .with_idempotency_key("key-step-000001"),
    );
    ok(
        k,
        &Request::new(
            command::TASK_COMPLETE,
            json!({"task_id": t["task_id"], "run_id": t["run_id"],
                   "expected_version": t["version"],
                   "outcome": "succeeded", "verification": "passed"}),
        )
        .with_idempotency_key("key-done-000001"),
    );
}

#[test]
fn every_crash_point_sees_what_test_strategy_section_5_says() {
    for (idempotency, enforced) in [
        (Idempotency::Enforced, true),
        (Idempotency::DisabledForRegressionTest, false),
    ] {
        let dir = tempfile::tempdir().unwrap();
        let recorder = CrashInjector::new();
        let k = Kernel::open(
            &dir.path().join("home"),
            KernelConfig {
                ids: Box::new(SystemIdSource::new()),
                clock: Box::new(SystemClock),
                crash: Box::new(recorder.clone()),
                idempotency,
            },
        )
        .unwrap();
        lifecycle(&k);

        let seen = recorder.seen();
        let mut want = vec![];
        for cmd in [
            command::SYSTEM_INIT,
            command::TASK_CREATE,
            command::TASK_START,
            command::TASK_STEP,
            command::TASK_COMPLETE,
        ] {
            for &point in CrashPoint::ALL {
                want.push((point, cmd.to_owned(), expected(point, enforced)));
            }
        }
        assert_eq!(seen, want, "idempotency enforced: {enforced}");
    }
}

/// A replay reaches only the points after the transaction, and says it
/// wrote nothing: the retry of a committed request is not a second effect.
#[test]
fn a_replay_reports_no_write() {
    let dir = tempfile::tempdir().unwrap();
    let recorder = CrashInjector::new();
    let k = Kernel::open(
        &dir.path().join("home"),
        KernelConfig {
            ids: Box::new(SystemIdSource::new()),
            clock: Box::new(SystemClock),
            crash: Box::new(recorder.clone()),
            idempotency: Idempotency::Enforced,
        },
    )
    .unwrap();
    ok(&k, &Request::new(command::SYSTEM_INIT, json!({})));
    let create = Request::new(command::TASK_CREATE, json!({"title": "t"}))
        .with_idempotency_key("key-create-0001");
    ok(&k, &create);
    recorder.clear_seen();
    let again = k.handle(&create);
    assert!(again.replayed, "{again:?}");

    let committed_nothing = Progress {
        committed: true,
        ..Progress::default()
    };
    let seen = recorder.seen();
    assert_eq!(
        seen,
        vec![
            (
                CrashPoint::BeforeTransaction,
                command::TASK_CREATE.to_owned(),
                Progress::default()
            ),
            (
                CrashPoint::AfterCommit,
                command::TASK_CREATE.to_owned(),
                committed_nothing
            ),
            (
                CrashPoint::BeforeResponse,
                command::TASK_CREATE.to_owned(),
                committed_nothing
            ),
        ]
    );
}
