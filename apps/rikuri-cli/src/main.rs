//! `rikuri`: the command-line client of `rikurid` (Stage Plan S00.07).
//!
//! A thin, untrusted client: it builds a request envelope, sends it over the
//! daemon's socket and prints the response envelope as one JSON line. It
//! holds no state and makes no decisions; the daemon validates everything.
//! The exit code is the response's documented exit code (`rikuri-api`):
//! 0 success, 2 usage, 3 request refused, 4 daemon unavailable, 5 daemon
//! failure.

use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use rikuri_api::command;
use rikuri_api::{ApiError, ErrorCode, Request, Response};
use rikuri_domain::{IdSource, SystemIdSource};
use serde_json::{Value, json};

#[derive(Parser)]
#[command(name = "rikuri", version, about = "Rikuri Pi command-line client")]
struct Cli {
    /// Rikuri home directory (default: $RIKURI_HOME, then ~/.local/share/rikuri).
    #[arg(long, global = true)]
    home: Option<PathBuf>,
    /// Daemon socket (default: HOME/run/rikurid.sock).
    #[arg(long, global = true)]
    socket: Option<PathBuf>,
    /// Print the response indented.
    #[arg(long, global = true)]
    pretty: bool,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create the owner and the default project (once).
    Init,
    /// Sample tasks.
    #[command(subcommand)]
    Task(TaskCommand),
    /// Current state.
    #[command(subcommand)]
    State(StateCommand),
    /// Ledger history.
    #[command(subcommand)]
    Ledger(LedgerCommand),
    /// Daemon and component health.
    #[command(subcommand)]
    System(SystemCommand),
    /// Send a request envelope read from standard input, unchanged.
    Raw,
}

#[derive(Subcommand)]
enum TaskCommand {
    Create {
        #[arg(long)]
        title: String,
        #[arg(long)]
        privacy: Option<String>,
        #[arg(long)]
        idempotency_key: Option<String>,
    },
    Start {
        #[arg(long)]
        task: String,
        #[arg(long)]
        expected_version: u64,
        #[arg(long)]
        idempotency_key: Option<String>,
    },
    Step {
        #[arg(long)]
        task: String,
        #[arg(long)]
        run: String,
        #[arg(long)]
        expected_version: u64,
        #[arg(long)]
        idempotency_key: Option<String>,
    },
    Complete {
        #[arg(long)]
        task: String,
        #[arg(long)]
        run: String,
        #[arg(long)]
        expected_version: u64,
        #[arg(long)]
        outcome: String,
        #[arg(long)]
        verification: String,
        #[arg(long)]
        idempotency_key: Option<String>,
    },
}

#[derive(Subcommand)]
enum StateCommand {
    Show,
}

#[derive(Subcommand)]
enum LedgerCommand {
    Tail {
        #[arg(long, default_value_t = 20)]
        limit: u32,
    },
}

#[derive(Subcommand)]
enum SystemCommand {
    Health,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let socket = match &cli.socket {
        Some(s) => Ok(s.clone()),
        None => rikuri_home(cli.home.clone()).map(|h| h.join("run").join("rikurid.sock")),
    };
    let response = match (socket, request(&cli.command)) {
        (Ok(socket), Ok(req)) => rikuri_api::client::send(&socket, &req),
        (Err(message), _) | (_, Err(message)) => {
            Response::failure(None, ApiError::new(ErrorCode::InvalidRequest, message))
        }
    };
    let text = if cli.pretty {
        serde_json::to_string_pretty(&response)
    } else {
        serde_json::to_string(&response)
    };
    let mut out = std::io::stdout();
    match text {
        Ok(t) if writeln!(out, "{t}").is_ok() => ExitCode::from(response.exit_code()),
        _ => ExitCode::from(ErrorCode::Internal.exit_code()),
    }
}

fn rikuri_home(flag: Option<PathBuf>) -> Result<PathBuf, String> {
    flag.or_else(|| std::env::var_os("RIKURI_HOME").map(PathBuf::from))
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share/rikuri")))
        .ok_or_else(|| "set --home, RIKURI_HOME or HOME".to_owned())
}

/// A key for a consequential command. A generated key is reported on
/// stderr so the owner can retry the same command safely with it.
fn key(given: &Option<String>) -> Result<String, String> {
    if let Some(k) = given {
        return Ok(k.clone());
    }
    let ulid = SystemIdSource::new()
        .next_ulid()
        .map_err(|e| e.to_string())?;
    let k = format!("cli-{ulid}");
    let _ = writeln!(
        std::io::stderr(),
        "rikuri: idempotency key {k} (to retry safely, rerun with --idempotency-key {k})"
    );
    Ok(k)
}

fn request(cmd: &Command) -> Result<Request, String> {
    let plain = |name: &str, params: Value| Ok(Request::new(name, params));
    let keyed = |name: &str, params: Value, k: &Option<String>| -> Result<Request, String> {
        Ok(Request::new(name, params).with_idempotency_key(&key(k)?))
    };
    match cmd {
        Command::Init => plain(command::SYSTEM_INIT, json!({})),
        Command::State(StateCommand::Show) => plain(command::STATE_SHOW, json!({})),
        Command::Ledger(LedgerCommand::Tail { limit }) => {
            plain(command::LEDGER_TAIL, json!({"limit": limit}))
        }
        Command::System(SystemCommand::Health) => plain(command::SYSTEM_HEALTH, json!({})),
        Command::Task(TaskCommand::Create {
            title,
            privacy,
            idempotency_key,
        }) => {
            let mut params = json!({"title": title});
            if let Some(p) = privacy {
                params["privacy_class"] = json!(p);
            }
            keyed(command::TASK_CREATE, params, idempotency_key)
        }
        Command::Task(TaskCommand::Start {
            task,
            expected_version,
            idempotency_key,
        }) => keyed(
            command::TASK_START,
            json!({"task_id": task, "expected_version": expected_version}),
            idempotency_key,
        ),
        Command::Task(TaskCommand::Step {
            task,
            run,
            expected_version,
            idempotency_key,
        }) => keyed(
            command::TASK_STEP,
            json!({"task_id": task, "run_id": run, "expected_version": expected_version}),
            idempotency_key,
        ),
        Command::Task(TaskCommand::Complete {
            task,
            run,
            expected_version,
            outcome,
            verification,
            idempotency_key,
        }) => keyed(
            command::TASK_COMPLETE,
            json!({
                "task_id": task, "run_id": run, "expected_version": expected_version,
                "outcome": outcome, "verification": verification
            }),
            idempotency_key,
        ),
        Command::Raw => {
            let mut text = String::new();
            std::io::stdin()
                .read_to_string(&mut text)
                .map_err(|e| format!("cannot read the request from stdin: {e}"))?;
            serde_json::from_str(text.trim())
                .map_err(|e| format!("malformed request envelope: {e}"))
        }
    }
}
