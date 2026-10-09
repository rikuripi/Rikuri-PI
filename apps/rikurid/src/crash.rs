//! Crash points (Test Strategy §5). Stage 00 owns points 1, 2, 3 and 6;
//! tool, budget and backup points arrive with those systems.
//!
//! The kernel calls a [`CrashHook`] at each point. Production uses
//! [`NoCrash`]. Tests either stop the real daemon at a point so a harness can
//! kill it with SIGKILL ([`HangAtPoint`], `crash-injection` builds only), or
//! implement [`CrashHook`] themselves to fail in-process (T00.06's hook
//! returns [`Crashed`] once at an armed point). A one-shot `FailAtPoint`
//! lived here until 2026-10-09; nothing used it, and S00.08's testkit owns
//! test injectors.

use rikuri_domain::wire_enum;

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

/// Production: never crashes.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoCrash;

impl CrashHook for NoCrash {
    fn at(&self, _: CrashPoint, _: &str, _: Progress) -> Result<(), Crashed> {
        Ok(())
    }
    fn is_armed(&self) -> bool {
        false
    }
}

/// Real-process crash: at `point` of `command`, announce the point and the
/// command's [`Progress`] on stderr and stop forever, so a harness can
/// check where it stopped and SIGKILL the daemon there with its transaction
/// state exactly as the point describes.
#[cfg(feature = "crash-injection")]
#[derive(Debug)]
pub struct HangAtPoint {
    point: CrashPoint,
    command: String,
}

#[cfg(feature = "crash-injection")]
impl HangAtPoint {
    /// The marker a harness waits for before killing the daemon.
    pub const MARKER: &'static str = "RIKURI_CRASH_POINT";

    /// Parses `RIKURI_CRASH_AT=<point>@<command>`.
    pub fn from_spec(spec: &str) -> Result<HangAtPoint, String> {
        let (point, command) = spec
            .split_once('@')
            .ok_or_else(|| format!("RIKURI_CRASH_AT must be <point>@<command>, got {spec:?}"))?;
        // crash points sit on the write path, so only a command that writes
        // can reach one; anything else would never fire
        if !rikuri_api::command::is_consequential(command) {
            return Err(format!(
                "RIKURI_CRASH_AT must name a command that writes, got {command:?}"
            ));
        }
        Ok(HangAtPoint {
            point: point.parse().map_err(|e| format!("{e}"))?,
            command: command.to_owned(),
        })
    }
}

#[cfg(feature = "crash-injection")]
impl CrashHook for HangAtPoint {
    fn at(&self, point: CrashPoint, command: &str, progress: Progress) -> Result<(), Crashed> {
        use std::io::Write;
        if point == self.point && command == self.command {
            let mut err = std::io::stderr();
            let _ = writeln!(err, "{} {} {} {}", Self::MARKER, point, command, progress);
            let _ = err.flush();
            loop {
                std::thread::sleep(std::time::Duration::from_secs(3600));
            }
        }
        Ok(())
    }
    fn is_armed(&self) -> bool {
        true
    }
}
