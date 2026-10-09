//! The local-socket client: one request line out, one response line in.
//! Used by the CLI and by test harnesses, so every caller speaks the
//! protocol the same way. The daemon is reached through a Unix domain socket
//! in the owner-only data directory.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::Duration;

use crate::{ApiError, ErrorCode, MAX_REQUEST_BYTES, Request, Response};

/// How long a client waits for the daemon's answer.
pub const RESPONSE_TIMEOUT: Duration = Duration::from_secs(30);

/// Sends `request` to the daemon listening on `socket` and returns its
/// response. Any failure to connect, write or read a complete response is a
/// client-side `daemon_unavailable` response: whether the command was applied
/// is then unknown, and the caller retries with the same idempotency key.
pub fn send(socket: &Path, request: &Request) -> Response {
    match exchange(socket, request) {
        Ok(response) => response,
        Err(reason) => Response::failure(
            request.request_id.clone(),
            ApiError::new(
                ErrorCode::DaemonUnavailable,
                format!("no response from rikurid at {}: {reason}", socket.display()),
            ),
        ),
    }
}

fn exchange(socket: &Path, request: &Request) -> Result<Response, String> {
    let mut line = serde_json::to_string(request).map_err(|e| e.to_string())?;
    if line.len() > MAX_REQUEST_BYTES {
        return Err(format!(
            "request is {} bytes; the limit is {MAX_REQUEST_BYTES}",
            line.len()
        ));
    }
    line.push('\n');
    let mut stream = UnixStream::connect(socket).map_err(|e| e.to_string())?;
    stream
        .set_read_timeout(Some(RESPONSE_TIMEOUT))
        .map_err(|e| e.to_string())?;
    stream
        .write_all(line.as_bytes())
        .map_err(|e| e.to_string())?;
    stream.flush().map_err(|e| e.to_string())?;
    let mut reader = BufReader::new(stream);
    let mut answer = String::new();
    let n = reader.read_line(&mut answer).map_err(|e| e.to_string())?;
    if n == 0 || !answer.ends_with('\n') {
        return Err("connection closed before a complete response".into());
    }
    serde_json::from_str(answer.trim_end()).map_err(|e| format!("malformed response: {e}"))
}
