//! The local socket server: one JSON request line per connection, one JSON
//! response line back (`rikuri-api`). The socket lives in an owner-only
//! directory and is itself owner-only. Requests are logged by command and
//! outcome only, never by parameters, which can hold owner content.

use std::io::{self, BufRead, BufReader, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::time::Duration;

use rikuri_api::{ApiError, ErrorCode, MAX_REQUEST_BYTES, Request, Response};

use crate::Kernel;

/// How long the server waits for a client to send its request line.
pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

/// The socket path inside a Rikuri home directory.
pub fn socket_path(home: &Path) -> PathBuf {
    home.join("run").join("rikurid.sock")
}

/// The longest socket path accepted. Unix socket addresses hold about 104
/// bytes on macOS and 108 on Linux; the smaller limit keeps both working.
pub const MAX_SOCKET_PATH_BYTES: usize = 103;

/// Binds the socket. Refuses if another daemon answers on it; replaces a
/// stale socket file left by a daemon that died.
pub fn bind(socket: &Path) -> io::Result<UnixListener> {
    let len = socket.as_os_str().len();
    if len > MAX_SOCKET_PATH_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "socket path is {len} bytes; Unix sockets allow at most {MAX_SOCKET_PATH_BYTES} here. \
                 Pass --socket with a shorter path (for example under $XDG_RUNTIME_DIR)"
            ),
        ));
    }
    if let Some(dir) = socket.parent() {
        let mut builder = std::fs::DirBuilder::new();
        builder.recursive(true);
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }
        builder.create(dir)?;
    }
    if socket.exists() {
        if UnixStream::connect(socket).is_ok() {
            return Err(io::Error::new(
                io::ErrorKind::AddrInUse,
                format!("another rikurid is already serving {}", socket.display()),
            ));
        }
        std::fs::remove_file(socket)?;
    }
    let listener = UnixListener::bind(socket)?;
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(socket, std::fs::Permissions::from_mode(0o600))?;
    }
    Ok(listener)
}

/// Serves connections one at a time until the listener fails.
pub fn serve(kernel: &Kernel, listener: &UnixListener, log: &mut dyn Write) -> io::Result<()> {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_connection(kernel, stream, log) {
                    let _ = writeln!(log, "rikurid: connection error: {e}");
                }
            }
            Err(e) => {
                let _ = writeln!(log, "rikurid: accept failed: {e}");
            }
        }
    }
    Ok(())
}

fn handle_connection(kernel: &Kernel, stream: UnixStream, log: &mut dyn Write) -> io::Result<()> {
    stream.set_read_timeout(Some(REQUEST_TIMEOUT))?;
    let mut writer = stream.try_clone()?;
    let limit = u64::try_from(MAX_REQUEST_BYTES + 1).unwrap_or(u64::MAX);
    let mut reader = BufReader::new(stream.take(limit));
    let mut line = String::new();
    let read = reader.read_line(&mut line);
    let response = match read {
        Ok(_) if !line.ends_with('\n') => refuse(format!(
            "request must be one JSON line of at most {MAX_REQUEST_BYTES} bytes"
        )),
        Ok(_) => match serde_json::from_str::<Request>(line.trim_end()) {
            Ok(req) => {
                let response = kernel.handle(&req);
                let outcome = response.error.as_ref().map_or("ok", |e| e.code.as_str());
                let _ = writeln!(
                    log,
                    "rikurid: {} -> {outcome}",
                    req.command.chars().take(64).collect::<String>()
                );
                response
            }
            Err(e) => refuse(format!("malformed request envelope: {e}")),
        },
        Err(e) => refuse(format!("could not read the request: {e}")),
    };
    let mut out = serde_json::to_string(&response).map_err(io::Error::other)?;
    out.push('\n');
    writer.write_all(out.as_bytes())?;
    writer.flush()
}

fn refuse(message: String) -> Response {
    Response::failure(None, ApiError::new(ErrorCode::InvalidRequest, message))
}
