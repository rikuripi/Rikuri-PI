//! The crash-point vocabulary (Test Strategy §5): where in a consequential
//! command a crash can be injected, what the command has done there, and
//! the hook the kernel calls at each point. It lives here, beside the other
//! injection seams [`Clock`](crate::Clock) and [`IdSource`](crate::IdSource),
//! so `rikurid` and `rikuri-testkit` share one copy of it (owner decision
//! 2026-10-09; a testkit depending on `rikurid` would compile two). The
//! kernel's own hooks (`NoCrash`, `HangAtPoint`) stay in `rikurid::crash`.
//! Stage 00 owns points 1, 2, 3 and 6; tool, budget and backup points
//! arrive with those systems.

use crate::wire_enum;

wire_enum! {
    /// Where in a consequential command a crash is injected.
    pub enum CrashPoint {
        /// Matrix point 1: before the canonical transaction begins.
        BeforeTransaction => "before_transaction",
        /// Matrix point 2: after the state write, before the event append
        /// (the transaction is open and must roll back).
        AfterStateWrite => "after_state_write",
        /// Matrix point 3: after the transaction commits.
        AfterCommit => "after_commit",
        /// Matrix point 6: after the idempotency record committed, before the
        /// response reaches the client.
        BeforeResponse => "before_response",
    }
}

/// A simulated crash: the caller stops as if the process had died here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Crashed {
    pub point: CrashPoint,
}

/// What a consequential command has done when it reaches a crash point.
/// The kernel reads each field from facts when the point is reached (the
/// store's open transaction, SQLite's change count, the ledger's newest
/// event, the idempotency write's result, the store's write returning), so
/// a point that moves sees different progress: that is how tests prove each
/// point sits where Test Strategy §5 puts it (2026-10-09; points moved
/// before the state write or into the transaction had passed every recovery
/// test, and flags kept by hand let a point move unnoticed).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Progress {
    /// The canonical transaction is open.
    pub transaction_open: bool,
    /// The command's state change is written (inside the transaction).
    pub state_written: bool,
    /// Its events are appended to the ledger.
    pub events_appended: bool,
    /// Its idempotency record is written.
    pub idempotency_recorded: bool,
    /// The transaction committed.
    pub committed: bool,
}

impl std::fmt::Display for Progress {
    /// `tx=0 state=1 events=0 idem=0 committed=0`, as the stop marker
    /// prints it for a harness to check.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = u8::from;
        write!(
            f,
            "tx={} state={} events={} idem={} committed={}",
            b(self.transaction_open),
            b(self.state_written),
            b(self.events_appended),
            b(self.idempotency_recorded),
            b(self.committed)
        )
    }
}

/// Called by the kernel at each crash point of each consequential command,
/// with what the command has done so far.
pub trait CrashHook: Send + Sync {
    fn at(&self, point: CrashPoint, command: &str, progress: Progress) -> Result<(), Crashed>;

    /// True when this hook can stop the kernel (test builds); reported in
    /// `system.health` so a test build is never mistaken for a real one.
    fn is_armed(&self) -> bool;
}
