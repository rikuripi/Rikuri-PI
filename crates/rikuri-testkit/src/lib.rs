//! Shared test fixtures (Stage Plan S00.08). Tests across the workspace
//! take their clocks, IDs and crash hooks from here instead of defining
//! their own, so a deterministic run means the same thing in every suite.
//!
//! - [`FixtureClock`]: a fixed or stepping [`Clock`].
//! - [`FixtureIds`]: sequential ULIDs at a fixed time, switchable to fail.
//! - [`CrashInjector`]: a [`CrashHook`] that records every crash point it
//!   reaches, with the command's [`Progress`], and crashes once at an armed
//!   point.
//!
//! Test-only: no runtime crate depends on this one except as a
//! dev-dependency. `FakeModelAdapter` and `FakeTool` arrive with the
//! interfaces they fake (S02.07, S04.01).

use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError};

use rikuri_domain::{
    Clock, CrashHook, CrashPoint, Crashed, IdError, IdSource, Progress, Timestamp,
};
use ulid::Ulid;

/// The fixed time the fixtures use unless told otherwise
/// (2026-09-21T14:13:20Z, in milliseconds), as the ULID time `FixtureIds`
/// takes.
pub const T0: u64 = 1_790_000_000_000;

/// [`T0`] as the [`Timestamp`] `FixtureClock` takes, so a suite's clock and
/// IDs share one time.
pub const T0_TIME: Timestamp = Timestamp(1_790_000_000_000);

/// A [`Clock`] that reads a fixed time, or moves forward a fixed step on
/// every reading, so "when" each event happened is distinguishable.
#[derive(Debug)]
pub struct FixtureClock {
    next: AtomicI64,
    step: i64,
}

impl FixtureClock {
    /// Always `at`.
    pub fn fixed(at: Timestamp) -> FixtureClock {
        FixtureClock::stepping(at, 0)
    }

    /// `from`, then `from + step`, `from + 2 * step`, … one per reading.
    pub fn stepping(from: Timestamp, step: i64) -> FixtureClock {
        FixtureClock {
            next: AtomicI64::new(from.0),
            step,
        }
    }
}

impl Clock for FixtureClock {
    fn now(&self) -> Timestamp {
        Timestamp(self.next.fetch_add(self.step, Ordering::SeqCst))
    }
}

/// An [`IdSource`] giving ULIDs 1, 2, 3, … at a fixed time, so the same
/// requests give the same IDs. It can be switched to fail, to test what a
/// caller does when IDs run out.
#[derive(Debug)]
pub struct FixtureIds {
    time_ms: u64,
    next: AtomicU64,
    failing: Arc<AtomicBool>,
}

impl FixtureIds {
    /// IDs 1, 2, 3, … at `time_ms`.
    pub fn sequential(time_ms: u64) -> FixtureIds {
        FixtureIds {
            time_ms,
            next: AtomicU64::new(1),
            failing: Arc::new(AtomicBool::new(false)),
        }
    }

    /// A switch shared with this source: while it is on, every request
    /// fails with [`IdError::SourceUnavailable`].
    pub fn failure_switch(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.failing)
    }
}

impl IdSource for FixtureIds {
    fn next_ulid(&self) -> Result<Ulid, IdError> {
        if self.failing.load(Ordering::SeqCst) {
            return Err(IdError::SourceUnavailable(
                "switched off by the test".into(),
            ));
        }
        Ok(Ulid::from_parts(
            self.time_ms,
            u128::from(self.next.fetch_add(1, Ordering::SeqCst)),
        ))
    }
}

/// One crash point the kernel reached.
pub type Reached = (CrashPoint, String, Progress);

#[derive(Debug, Default)]
struct Injector {
    armed: Option<(CrashPoint, Option<String>)>,
    seen: Vec<Reached>,
}

/// A [`CrashHook`] for in-process tests. It records every crash point it
/// reaches, with the command and its [`Progress`], and once armed it fails
/// the next matching point with [`Crashed`] (a simulated crash), then
/// disarms. Clones share one state, so a test keeps a clone while the
/// kernel owns another.
#[derive(Debug, Clone, Default)]
pub struct CrashInjector(Arc<Mutex<Injector>>);

impl CrashInjector {
    pub fn new() -> CrashInjector {
        CrashInjector::default()
    }

    /// Crash once at `point` of any command; `None` disarms. Arming again
    /// replaces any point still pending.
    pub fn arm(&self, point: Option<CrashPoint>) {
        self.state().armed = point.map(|p| (p, None));
    }

    /// Crash once at `point`, only of `command`.
    pub fn arm_for(&self, point: CrashPoint, command: &str) {
        self.state().armed = Some((point, Some(command.to_owned())));
    }

    /// Every crash point reached so far, in order.
    pub fn seen(&self) -> Vec<Reached> {
        self.state().seen.clone()
    }

    /// Forgets the points reached so far.
    pub fn clear_seen(&self) {
        self.state().seen.clear();
    }

    /// None of the injector's own methods panic while holding the lock, so
    /// it is not poisoned through this API; recovering instead of
    /// unwrapping keeps the kit free of panics on its own lock.
    fn state(&self) -> std::sync::MutexGuard<'_, Injector> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

impl CrashHook for CrashInjector {
    fn at(&self, point: CrashPoint, command: &str, progress: Progress) -> Result<(), Crashed> {
        let mut s = self.state();
        s.seen.push((point, command.to_owned(), progress));
        let fires = match &s.armed {
            Some((p, None)) => *p == point,
            Some((p, Some(c))) => *p == point && c == command,
            None => false,
        };
        if fires {
            s.armed = None;
            Err(Crashed { point })
        } else {
            Ok(())
        }
    }

    /// Always armed: a kernel built with an injector is a test kernel, and
    /// its health says so.
    fn is_armed(&self) -> bool {
        true
    }
}
