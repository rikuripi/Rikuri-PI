//! The text form of `Progress` is a contract: the real daemon prints it in
//! its stop marker and the crash matrix harness parses it (W1 placement).
//! Each field has its own place and its own 0/1, in a fixed order.

use rikuri_domain::{CrashPoint, Progress};

#[test]
fn progress_prints_each_field_in_its_place() {
    assert_eq!(
        Progress::default().to_string(),
        "tx=0 state=0 events=0 idem=0 committed=0"
    );
    let one_each = [
        (
            Progress {
                transaction_open: true,
                ..Progress::default()
            },
            "tx=1 state=0 events=0 idem=0 committed=0",
        ),
        (
            Progress {
                state_written: true,
                ..Progress::default()
            },
            "tx=0 state=1 events=0 idem=0 committed=0",
        ),
        (
            Progress {
                events_appended: true,
                ..Progress::default()
            },
            "tx=0 state=0 events=1 idem=0 committed=0",
        ),
        (
            Progress {
                idempotency_recorded: true,
                ..Progress::default()
            },
            "tx=0 state=0 events=0 idem=1 committed=0",
        ),
        (
            Progress {
                committed: true,
                ..Progress::default()
            },
            "tx=0 state=0 events=0 idem=0 committed=1",
        ),
    ];
    for (progress, text) in one_each {
        assert_eq!(progress.to_string(), text);
    }
}

#[test]
fn crash_points_keep_their_wire_names() {
    let names: Vec<&str> = CrashPoint::ALL.iter().map(|p| p.as_str()).collect();
    assert_eq!(
        names,
        [
            "before_transaction",
            "after_state_write",
            "after_commit",
            "before_response"
        ]
    );
}
