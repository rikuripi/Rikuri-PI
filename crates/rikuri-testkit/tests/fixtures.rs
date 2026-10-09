//! The kit's own contract: each fixture does exactly what the suites that
//! use it rely on.

use std::sync::atomic::Ordering;

use rikuri_domain::{
    AnyId, Clock, CrashHook, CrashPoint, Crashed, EventId, IdError, IdSource, Progress, TaskId,
    Timestamp,
};
use rikuri_testkit::{CrashInjector, FixtureClock, FixtureIds, T0, T0_TIME};
use ulid::Ulid;

#[test]
fn a_fixed_clock_never_moves_and_a_stepping_one_moves_by_its_step() {
    let fixed = FixtureClock::fixed(Timestamp(7));
    assert_eq!([fixed.now(), fixed.now()], [Timestamp(7), Timestamp(7)]);

    let stepping = FixtureClock::stepping(Timestamp(1_000), 5);
    assert_eq!(
        [stepping.now(), stepping.now(), stepping.now()],
        [Timestamp(1_000), Timestamp(1_005), Timestamp(1_010)]
    );
}

#[test]
fn ids_are_sequential_at_a_fixed_time_and_can_be_switched_off() {
    let ids = FixtureIds::sequential(T0);
    let first = ids.next_ulid().unwrap();
    let second = ids.next_ulid().unwrap();
    assert_eq!(first, Ulid::from_parts(T0, 1));
    assert_eq!(second, Ulid::from_parts(T0, 2));

    let switch = ids.failure_switch();
    switch.store(true, Ordering::SeqCst);
    assert!(matches!(
        ids.next_ulid(),
        Err(IdError::SourceUnavailable(_))
    ));
    switch.store(false, Ordering::SeqCst);
    assert_eq!(ids.next_ulid().unwrap(), Ulid::from_parts(T0, 3));
}

fn progress(n: u8) -> Progress {
    Progress {
        transaction_open: n & 1 != 0,
        state_written: n & 2 != 0,
        events_appended: n & 4 != 0,
        idempotency_recorded: n & 8 != 0,
        committed: n & 16 != 0,
    }
}

#[test]
fn the_injector_records_every_point_it_reaches_in_order() {
    let injector = CrashInjector::new();
    let kernel_side = injector.clone();
    assert!(kernel_side.is_armed(), "an injector is always a test hook");
    for (n, point) in CrashPoint::ALL.iter().enumerate() {
        let n = u8::try_from(n).unwrap();
        assert_eq!(kernel_side.at(*point, "task.create", progress(n)), Ok(()));
    }
    let seen = injector.seen();
    assert_eq!(seen.len(), CrashPoint::ALL.len());
    for (n, (point, command, p)) in seen.iter().enumerate() {
        assert_eq!(*point, CrashPoint::ALL[n]);
        assert_eq!(command, "task.create");
        assert_eq!(*p, progress(u8::try_from(n).unwrap()));
    }
    injector.clear_seen();
    assert!(injector.seen().is_empty());
}

#[test]
fn an_armed_point_crashes_once_for_any_command_then_disarms() {
    let injector = CrashInjector::new();
    let at = |point, command| injector.at(point, command, Progress::default());
    injector.arm(Some(CrashPoint::AfterCommit));
    assert_eq!(at(CrashPoint::BeforeTransaction, "task.create"), Ok(()));
    assert_eq!(
        at(CrashPoint::AfterCommit, "task.step"),
        Err(Crashed {
            point: CrashPoint::AfterCommit
        })
    );
    assert_eq!(
        at(CrashPoint::AfterCommit, "task.step"),
        Ok(()),
        "once only"
    );

    injector.arm(Some(CrashPoint::AfterStateWrite));
    injector.arm(None);
    assert_eq!(
        at(CrashPoint::AfterStateWrite, "task.create"),
        Ok(()),
        "disarmed"
    );

    // the point it crashed at is recorded too: every point reached
    let reached: Vec<CrashPoint> = injector.seen().into_iter().map(|(p, _, _)| p).collect();
    assert_eq!(
        reached,
        [
            CrashPoint::BeforeTransaction,
            CrashPoint::AfterCommit,
            CrashPoint::AfterCommit,
            CrashPoint::AfterStateWrite
        ]
    );
}

#[test]
fn a_point_armed_for_one_command_ignores_the_others() {
    let injector = CrashInjector::new();
    let at = |point, command| injector.at(point, command, Progress::default());
    injector.arm_for(CrashPoint::BeforeResponse, "task.complete");
    assert_eq!(at(CrashPoint::BeforeResponse, "task.create"), Ok(()));
    assert_eq!(at(CrashPoint::AfterCommit, "task.complete"), Ok(()));
    assert_eq!(
        at(CrashPoint::BeforeResponse, "task.complete"),
        Err(Crashed {
            point: CrashPoint::BeforeResponse
        })
    );
    assert_eq!(at(CrashPoint::BeforeResponse, "task.complete"), Ok(()));
}

#[test]
fn t0_is_one_time_for_clocks_and_ids() {
    assert_eq!(i64::try_from(T0).ok(), Some(T0_TIME.0));
    assert_eq!(FixtureClock::fixed(T0_TIME).now(), T0_TIME);
}

/// ADR 0001 follow-up: fixture IDs keep the stable-ID properties the domain
/// proves for arbitrary ULIDs (`rikuri-domain/tests/id_properties.rs`):
/// one canonical text form, round-trips through text and JSON as their own
/// kind only, parse as `AnyId`, and text order equals ID order (so issue
/// order survives storage as text). Distinct and increasing as issued.
#[test]
fn fixture_ids_keep_the_adr_0001_id_properties() {
    let ids = FixtureIds::sequential(T0);
    let issued: Vec<TaskId> = (0..1_000)
        .map(|_| TaskId::generate(&ids).unwrap())
        .collect();
    for task in &issued {
        let text = task.to_string();
        assert_eq!(text.parse::<TaskId>().unwrap(), *task);
        assert_eq!(text.parse::<TaskId>().unwrap().to_string(), text);
        assert!(text.parse::<EventId>().is_err());
        let json = serde_json::to_string(task).unwrap();
        assert_eq!(serde_json::from_str::<TaskId>(&json).unwrap(), *task);
        assert_eq!(text.parse::<AnyId>().unwrap().prefix(), "task");
    }
    for pair in issued.windows(2) {
        assert!(pair[0] < pair[1], "issued in increasing order");
        assert!(
            pair[0].to_string() < pair[1].to_string(),
            "text order equals ID order"
        );
    }
}

/// Arming again replaces a point still pending: a stale point never fires.
#[test]
fn arming_again_replaces_a_pending_point() {
    let injector = CrashInjector::new();
    let at = |point, command| injector.at(point, command, Progress::default());
    injector.arm(Some(CrashPoint::BeforeTransaction));
    injector.arm(Some(CrashPoint::AfterCommit));
    assert_eq!(at(CrashPoint::BeforeTransaction, "task.create"), Ok(()));
    assert_eq!(
        at(CrashPoint::AfterCommit, "task.create"),
        Err(Crashed {
            point: CrashPoint::AfterCommit
        })
    );
    injector.arm_for(CrashPoint::AfterStateWrite, "task.step");
    injector.arm(Some(CrashPoint::BeforeResponse));
    assert_eq!(at(CrashPoint::AfterStateWrite, "task.step"), Ok(()));
}
