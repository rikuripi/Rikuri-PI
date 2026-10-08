//! Crash points (Test Strategy §5). Stage 00 owns points 1, 2, 3 and 6;
//! tool, budget and backup points arrive with those systems.
//!
//! The kernel calls a [`CrashHook`] at each point. Production uses
//! [`NoCrash`]. Tests either stop the real daemon at a point so a harness can
//! kill it with SIGKILL ([`HangAtPoint`], `crash-injection` builds only), or
//! fail in-process to simulate a crash ([`FailAtPoint`]).

use std::sync::atomic::{AtomicBool, Ordering};

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

/// Called by the kernel at each crash point of each consequential command.
pub trait CrashHook: Send + Sync {
    fn at(&self, point: CrashPoint, command: &str) -> Result<(), Crashed>;

    /// True when this hook can stop the kernel (test builds); reported in
    /// `system.health` so a test build is never mistaken for a real one.
    fn is_armed(&self) -> bool;
}

/// Production: never crashes.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoCrash;

impl CrashHook for NoCrash {
    fn at(&self, _: CrashPoint, _: &str) -> Result<(), Crashed> {
        Ok(())
    }
    fn is_armed(&self) -> bool {
        false
    }
}

/// In-process simulation: fails once, at `point` of `command`.
#[derive(Debug)]
pub struct FailAtPoint {
    point: CrashPoint,
    command: String,
    fired: AtomicBool,
}

impl FailAtPoint {
    pub fn new(point: CrashPoint, command: &str) -> FailAtPoint {
        FailAtPoint {
            point,
            command: command.to_owned(),
            fired: AtomicBool::new(false),
        }
    }
}

impl CrashHook for FailAtPoint {
    fn at(&self, point: CrashPoint, command: &str) -> Result<(), Crashed> {
        if point == self.point
            && command == self.command
            && !self.fired.swap(true, Ordering::SeqCst)
        {
            Err(Crashed { point })
        } else {
            Ok(())
        }
    }
    fn is_armed(&self) -> bool {
        true
    }
}

/// Real-process crash: at `point` of `command`, announce the point on
/// stderr and stop forever, so a harness can SIGKILL the daemon there with
/// its transaction state exactly as the point describes.
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
    fn at(&self, point: CrashPoint, command: &str) -> Result<(), Crashed> {
        use std::io::Write;
        if point == self.point && command == self.command {
            let mut err = std::io::stderr();
            let _ = writeln!(err, "{} {} {}", Self::MARKER, point, command);
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
