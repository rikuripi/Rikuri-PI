//! Health evidence counts and thresholds (Architecture §27.8): a component
//! counts consecutive failures and resets on success, the store is judged on
//! reads and writes separately, and it turns Degraded after one or two
//! failures in a row and Critical from the third. cargo-mutants (2026-10-08,
//! crash matrix record) found none of these asserted; verifier round 5 found
//! the success path and the kernel's "could not run" path unasserted.
#![cfg(unix)]

use std::path::Path;
use std::sync::atomic::{AtomicI64, Ordering};

use rikuri_api::{Request, command};
use rikuri_domain::{Clock, SystemClock, SystemIdSource, Timestamp};
use rikuri_storage::{OpHealth, StoreHealth};
use rikurid::crash::NoCrash;
use rikurid::health::{ComponentHealth, HealthState};
use rikurid::kernel::database_path;
use rikurid::{Idempotency, Kernel, KernelConfig};
use serde_json::{Value, json};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn open(home: &Path) -> Result<Kernel, rikurid::KernelError> {
    Kernel::open(
        home,
        KernelConfig {
            ids: Box::new(SystemIdSource::new()),
            clock: Box::new(SystemClock),
            crash: Box::new(NoCrash),
            idempotency: Idempotency::Enforced,
        },
    )
}

/// init → create → start → complete.
fn completed_run(home: &Path) -> TestResult<Kernel> {
    let k = open(home)?;
    k.handle(&Request::new(command::SYSTEM_INIT, json!({})));
    let created = keyed(
        &k,
        command::TASK_CREATE,
        json!({"title": "t"}),
        "key-create-0001",
    )?;
    let started = keyed(
        &k,
        command::TASK_START,
        json!({"task_id": created["task_id"], "expected_version": created["version"]}),
        "key-start-00001",
    )?;
    keyed(
        &k,
        command::TASK_COMPLETE,
        json!({"task_id": started["task_id"], "run_id": started["run_id"],
               "expected_version": started["version"],
               "outcome": "succeeded", "verification": "passed"}),
        "key-done-000001",
    )?;
    Ok(k)
}

fn keyed(k: &Kernel, command: &str, params: Value, key: &str) -> TestResult<Value> {
    let r = k.handle(&Request::new(command, params).with_idempotency_key(key));
    match (r.error, r.result) {
        (None, Some(v)) => Ok(v),
        (e, _) => Err(format!("{command} failed: {e:?}").into()),
    }
}

fn raw(home: &Path, sql: &str) -> TestResult {
    let raw = rikuri_storage::rusqlite::Connection::open(database_path(home))?;
    raw.execute_batch(sql)?;
    Ok(())
}

/// `system.health`'s component named `name`.
fn component(k: &Kernel, name: &str) -> TestResult<Value> {
    let h = k
        .handle(&Request::new(command::SYSTEM_HEALTH, json!({})))
        .result
        .ok_or("system.health failed")?;
    h["components"]
        .as_array()
        .and_then(|cs| cs.iter().find(|c| c["component"] == name))
        .cloned()
        .ok_or_else(|| format!("no {name} component in {h}").into())
}

/// The consistency check counts its consecutive failures (each
/// `system.health` reruns it).
#[test]
fn the_consistency_component_counts_consecutive_failures() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    assert_eq!(
        component(&k, "ledger_consistency")?["consecutive_failures"],
        0
    );
    raw(
        &home,
        "UPDATE objects SET body = json_set(body, '$.outcome', 'failed') WHERE kind = 'run'",
    )?;
    assert_eq!(
        component(&k, "ledger_consistency")?["consecutive_failures"],
        1
    );
    let c = component(&k, "ledger_consistency")?;
    assert_eq!(c["consecutive_failures"], 2, "{c}");
    assert_eq!(c["state"], "critical", "{c}");
    Ok(())
}

/// Reads that fail while writes succeed make the store unhealthy, then
/// Critical as they keep failing. (The exact thresholds are pinned below
/// through `from_store`, so this does not depend on how many reads
/// `system.health` itself performs.)
#[test]
fn failing_reads_turn_the_store_unhealthy_then_critical() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    // a task body that no longer decodes: every read that lists tasks fails
    raw(
        &home,
        "UPDATE objects SET body = json_set(body, '$.status', 'unknown') WHERE kind = 'task'",
    )?;
    let first = component(&k, "store")?;
    assert_ne!(first["state"], "healthy", "{first}");
    assert!(
        first["detail"]
            .as_str()
            .is_some_and(|d| d.contains("read transaction(s) failed")),
        "{first}"
    );
    let mut last = first;
    for _ in 0..5 {
        last = component(&k, "store")?;
    }
    assert_eq!(last["state"], "critical", "{last}");
    Ok(())
}

fn failing(n: u64) -> OpHealth {
    OpHealth {
        consecutive_failures: n,
        last_error: Some(format!("failure {n}")),
        ..OpHealth::default()
    }
}

/// The thresholds, pinned directly: Degraded after one or two failures in a
/// row, Critical from the third, for reads and for writes; when both fail,
/// writes are judged (a failing write must never hide behind reads).
#[test]
fn store_thresholds_are_degraded_then_critical_at_three() {
    let ok = Some(Timestamp(1));
    for (n, want) in [
        (1, HealthState::Degraded),
        (2, HealthState::Degraded),
        (3, HealthState::Critical),
        (4, HealthState::Critical),
    ] {
        for (reads, writes, side) in [
            (failing(n), OpHealth::default(), "read"),
            (OpHealth::default(), failing(n), "write"),
        ] {
            let h = StoreHealth {
                last_success: ok,
                reads,
                writes,
                ..StoreHealth::default()
            };
            let c = ComponentHealth::from_store("store", &h);
            assert_eq!(c.state, want, "{n} {side}: {c:?}");
            assert_eq!(c.consecutive_failures, n, "{c:?}");
            assert!(
                c.detail.contains(&format!("{n} {side} transaction(s)")),
                "{c:?}"
            );
        }
    }
    // both failing: the write side is reported, even with fewer failures
    let h = StoreHealth {
        last_success: ok,
        reads: failing(3),
        writes: failing(1),
        ..StoreHealth::default()
    };
    let c = ComponentHealth::from_store("store", &h);
    assert!(c.detail.contains("1 write transaction(s)"), "{c:?}");
}

/// A clock that moves forward one millisecond per reading, so "when" each
/// check happened is distinguishable.
struct Stepping(AtomicI64);

impl Clock for Stepping {
    fn now(&self) -> Timestamp {
        Timestamp(self.0.fetch_add(1, Ordering::SeqCst))
    }
}

/// After a failure is repaired, the next successful check resets the
/// count, records this success (not an older one) and the work it covered.
#[test]
fn a_successful_check_after_a_failure_resets_the_evidence() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = Kernel::open(
        &home,
        KernelConfig {
            ids: Box::new(SystemIdSource::new()),
            clock: Box::new(Stepping(AtomicI64::new(1_000))),
            crash: Box::new(NoCrash),
            idempotency: Idempotency::Enforced,
        },
    )?;
    // the open-time check covered an empty ledger
    k.handle(&Request::new(command::SYSTEM_INIT, json!({})));
    let created = keyed(
        &k,
        command::TASK_CREATE,
        json!({"title": "t"}),
        "key-create-0001",
    )?;
    keyed(
        &k,
        command::TASK_START,
        json!({"task_id": created["task_id"], "expected_version": created["version"]}),
        "key-start-00001",
    )?;
    let set = |status: &str| {
        raw(
            &home,
            &format!(
                "UPDATE objects SET body = json_set(body, '$.status', '{status}') \
                 WHERE kind = 'task'"
            ),
        )
    };
    set("completed")?;
    let failed = component(&k, "ledger_consistency")?;
    assert_eq!(failed["consecutive_failures"], 1, "{failed}");
    set("running")?;
    let c = component(&k, "ledger_consistency")?;
    assert_eq!(c["state"], "healthy", "{c}");
    assert_eq!(c["consecutive_failures"], 0, "{c}");
    assert_eq!(c["last_success"], c["last_attempt"], "{c}");
    assert!(
        c["last_success"].as_i64() > failed["last_attempt"].as_i64(),
        "this success, not an older one: {c} after {failed}"
    );
    assert_eq!(c["work_completed"], 3, "init, created, started: {c}");
    Ok(())
}

/// When the replay itself cannot run (a stored task that no longer
/// decodes), the kernel reports it and stops writes: a check that could not
/// run is never a pass.
#[test]
fn a_check_that_cannot_run_quarantines_writes() -> TestResult {
    let dir = tempfile::tempdir()?;
    let home = dir.path().join("home");
    let k = completed_run(&home)?;
    raw(
        &home,
        "UPDATE objects SET body = json_set(body, '$.status', 'unknown') WHERE kind = 'task'",
    )?;
    let report = k.check_consistency();
    assert!(
        report
            .problems
            .iter()
            .any(|p| p.contains("consistency check could not run")),
        "{:?}",
        report.problems
    );
    assert!(k.quarantined().is_some());
    let refused = k.handle(
        &Request::new(command::TASK_CREATE, json!({"title": "after"}))
            .with_idempotency_key("key-create-0002"),
    );
    assert_eq!(
        refused.error.map(|e| e.code.as_str().to_owned()),
        Some("quarantined".to_owned())
    );
    Ok(())
}
