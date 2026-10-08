//! `rikurid [--home DIR] [--socket PATH]`: open the canonical store under
//! DIR and serve the local socket PATH (default `DIR/run/rikurid.sock`).
//! DIR defaults to `$RIKURI_HOME`, then `~/.local/share/rikuri`.

use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use rikuri_domain::{SystemClock, SystemIdSource};
use rikurid::crash::{CrashHook, NoCrash};
use rikurid::{Idempotency, Kernel, KernelConfig, server};

fn main() -> ExitCode {
    let mut err = std::io::stderr();
    match run(&mut err) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            let _ = writeln!(err, "rikurid: {message}");
            ExitCode::FAILURE
        }
    }
}

fn run(log: &mut dyn Write) -> Result<(), String> {
    let (home, socket) = args()?;
    let (crash, idempotency) = test_hooks(log)?;
    let kernel = Kernel::open(
        &home,
        KernelConfig {
            ids: Box::new(SystemIdSource::new()),
            clock: Box::new(SystemClock),
            crash,
            idempotency,
        },
    )
    .map_err(|e| format!("cannot open {}: {e}", home.display()))?;
    let report = kernel.check_consistency();
    if !report.problems.is_empty() {
        let _ = writeln!(
            log,
            "rikurid: WARNING ledger and state disagree: {}",
            report.problems.join("; ")
        );
    }
    let socket = socket.unwrap_or_else(|| server::socket_path(&home));
    let listener =
        server::bind(&socket).map_err(|e| format!("cannot bind {}: {e}", socket.display()))?;
    let _ = writeln!(log, "rikurid: ready on {}", socket.display());
    server::serve(&kernel, &listener, log).map_err(|e| e.to_string())
}

const USAGE: &str = "usage: rikurid [--home DIR] [--socket PATH]";

fn args() -> Result<(PathBuf, Option<PathBuf>), String> {
    let (mut home, mut socket) = (None, None);
    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        let value = args.next().ok_or_else(|| USAGE.to_owned())?;
        match flag.as_str() {
            "--home" if home.is_none() => home = Some(PathBuf::from(value)),
            "--socket" if socket.is_none() => socket = Some(PathBuf::from(value)),
            _ => return Err(USAGE.to_owned()),
        }
    }
    let home = match home {
        Some(h) => h,
        None => match std::env::var_os("RIKURI_HOME") {
            Some(dir) => PathBuf::from(dir),
            None => std::env::var_os("HOME")
                .map(|h| PathBuf::from(h).join(".local/share/rikuri"))
                .ok_or_else(|| "set --home, RIKURI_HOME or HOME".to_owned())?,
        },
    };
    Ok((home, socket))
}

/// Test hooks exist only in `crash-injection` builds. A production build
/// that sees the test variables says so instead of silently ignoring them.
#[cfg(not(feature = "crash-injection"))]
fn test_hooks(log: &mut dyn Write) -> Result<(Box<dyn CrashHook>, Idempotency), String> {
    for var in ["RIKURI_CRASH_AT", "RIKURI_TEST_DISABLE_IDEMPOTENCY"] {
        if std::env::var_os(var).is_some() {
            let _ = writeln!(
                log,
                "rikurid: {var} is set but this build has no crash injection; ignored"
            );
        }
    }
    Ok((Box::new(NoCrash), Idempotency::Enforced))
}

#[cfg(feature = "crash-injection")]
fn test_hooks(log: &mut dyn Write) -> Result<(Box<dyn CrashHook>, Idempotency), String> {
    let crash: Box<dyn CrashHook> = match std::env::var("RIKURI_CRASH_AT") {
        Ok(spec) => {
            let _ = writeln!(log, "rikurid: TEST BUILD: will stop at {spec}");
            Box::new(rikurid::crash::HangAtPoint::from_spec(&spec)?)
        }
        Err(_) => Box::new(NoCrash),
    };
    let idempotency = if std::env::var_os("RIKURI_TEST_DISABLE_IDEMPOTENCY").is_some() {
        let _ = writeln!(
            log,
            "rikurid: TEST BUILD: idempotency DISABLED for the T00.08 regression"
        );
        Idempotency::DisabledForRegressionTest
    } else {
        Idempotency::Enforced
    };
    Ok((crash, idempotency))
}
