//! The kernel's crash hooks. The vocabulary (`CrashPoint`, `Crashed`,
//! `Progress`, the `CrashHook` trait) lives in `rikuri_domain::crash` and is
//! re-exported here, so callers keep using `rikurid::crash::…`.
//!
//! The kernel calls a [`CrashHook`] at each point. Production uses
//! [`NoCrash`]. Tests either stop the real daemon at a point so a harness can
//! kill it with SIGKILL ([`HangAtPoint`], `crash-injection` builds only), or
//! use `rikuri-testkit`'s `CrashInjector` in process.

pub use rikuri_domain::crash::{CrashHook, CrashPoint, Crashed, Progress};

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
