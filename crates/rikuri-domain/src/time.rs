//! Timestamps and the clock seam. Production code reads [`SystemClock`];
//! tests use a fixed or stepped clock (`rikuri-testkit` `FixtureClock`)
//! through the same trait, so recorded times are deterministic under test.

use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

/// Milliseconds since the Unix epoch, UTC.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Timestamp(pub i64);

impl Timestamp {
    pub const fn millis(self) -> i64 {
        self.0
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}ms", self.0)
    }
}

/// A source of the current time.
pub trait Clock: Send + Sync {
    fn now(&self) -> Timestamp;
}

/// The operating system's wall clock. A clock before 1970 or beyond the
/// `i64` millisecond range reads as the nearest bound rather than panicking.
#[derive(Debug, Default, Clone, Copy)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> Timestamp {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| i64::try_from(d.as_millis()).unwrap_or(i64::MAX))
            .unwrap_or(0);
        Timestamp(millis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_clock_is_after_2026_and_serializes_as_a_number() {
        let now = SystemClock.now();
        assert!(now.millis() > 1_767_225_600_000, "{now}"); // 2026-01-01
        assert_eq!(serde_json::to_string(&Timestamp(5)).unwrap(), "5");
    }
}
