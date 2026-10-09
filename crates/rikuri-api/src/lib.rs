//! The agent-facing contract of the Rikuri kernel (Stage Plan S00.07,
//! Architecture §25.4, §42): a versioned JSON envelope with typed success and
//! error states, stable error codes and documented CLI exit codes. Human
//! rendering is a layer above this; machines read these shapes.
//!
//! One request per line in, one response per line out. The same envelope
//! goes through the daemon's socket and the in-process adapter, and both
//! must give the same answers (T00.09).

#[cfg(unix)]
pub mod client;
pub mod command;

use rikuri_domain::wire_enum;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The envelope version this build speaks.
pub const ENVELOPE_VERSION: u32 = 1;

/// The largest request line the daemon reads, in bytes.
pub const MAX_REQUEST_BYTES: usize = 64 * 1024;

/// A command request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    /// Envelope version; must equal [`ENVELOPE_VERSION`].
    pub v: u32,
    /// Dotted command name, e.g. `task.create` (see [`command`]).
    pub command: String,
    /// Client correlation ID, echoed in the response. At most 64 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Required for consequential commands: a retry with the same key and
    /// the same request returns the original response and changes nothing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    /// Command parameters, an object.
    #[serde(default = "empty_object")]
    pub params: Value,
}

fn empty_object() -> Value {
    Value::Object(serde_json::Map::new())
}

impl Request {
    pub fn new(command: &str, params: Value) -> Request {
        Request {
            v: ENVELOPE_VERSION,
            command: command.to_owned(),
            request_id: None,
            idempotency_key: None,
            params,
        }
    }

    pub fn with_idempotency_key(mut self, key: &str) -> Request {
        self.idempotency_key = Some(key.to_owned());
        self
    }
}

/// A command response: exactly one of `result` and `error` is present.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Response {
    pub v: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    pub ok: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
    /// True when the response was served from an idempotency record: the
    /// command had already been applied and nothing changed this time.
    #[serde(default)]
    pub replayed: bool,
}

impl Response {
    pub fn success(request_id: Option<String>, result: Value) -> Response {
        Response {
            v: ENVELOPE_VERSION,
            request_id,
            ok: true,
            result: Some(result),
            error: None,
            replayed: false,
        }
    }

    pub fn failure(request_id: Option<String>, error: ApiError) -> Response {
        Response {
            v: ENVELOPE_VERSION,
            request_id,
            ok: false,
            result: None,
            error: Some(error),
            replayed: false,
        }
    }

    /// The process exit code a CLI returns for this response.
    pub fn exit_code(&self) -> u8 {
        match &self.error {
            None if self.ok => 0,
            None => ErrorCode::Internal.exit_code(),
            Some(e) => e.code.exit_code(),
        }
    }
}

/// A typed error.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApiError {
    pub code: ErrorCode,
    /// Human-readable explanation. Programs branch on `code`, never on this.
    pub message: String,
    /// Whether the same request may succeed if retried later unchanged.
    pub retryable: bool,
    /// Structured fields for the code, e.g. `{"expected": 1, "actual": 2}`.
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub details: Value,
}

impl ApiError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> ApiError {
        ApiError {
            code,
            message: message.into(),
            retryable: code.retryable(),
            details: Value::Null,
        }
    }

    pub fn with_details(mut self, details: Value) -> ApiError {
        self.details = details;
        self
    }
}

wire_enum! {
    /// Stable error codes. A code is never renamed or reused.
    pub enum ErrorCode {
        /// Malformed envelope or parameters.
        InvalidRequest => "invalid_request",
        /// The envelope version is not one this build speaks.
        UnsupportedEnvelopeVersion => "unsupported_envelope_version",
        /// No such command.
        UnknownCommand => "unknown_command",
        /// `system.init` has not been run.
        NotInitialized => "not_initialized",
        /// A referenced entity does not exist.
        NotFound => "not_found",
        /// The entity changed since the version the caller named.
        VersionConflict => "version_conflict",
        /// The idempotency key was used for a different request.
        IdempotencyConflict => "idempotency_conflict",
        /// The command is not valid in the entity's current state.
        InvalidState => "invalid_state",
        /// The canonical store failed; nothing was committed.
        StorageFailure => "storage_failure",
        /// An unexpected kernel failure; nothing was committed.
        Internal => "internal",
        /// The daemon could not be reached or the connection broke before
        /// a response. Produced by clients, and by the in-process adapter
        /// only for a simulated crash in a test build. Whether the command
        /// was applied is unknown: retry with the same idempotency key.
        DaemonUnavailable => "daemon_unavailable",
        /// The ledger-consistency check failed (current state and ledger
        /// history disagree, or the check could not run), so the daemon
        /// serves reads, health and replays of committed requests but refuses
        /// every write until the owner repairs it. Nothing was committed.
        Quarantined => "quarantined",
    }
}

impl ErrorCode {
    /// Documented CLI exit codes:
    ///
    /// | Exit | Meaning |
    /// |---|---|
    /// | 0 | success |
    /// | 2 | command-line usage error (reported by the CLI's argument parser) |
    /// | 3 | request refused: fix the request or the state, then retry |
    /// | 4 | daemon unavailable: outcome unknown, retry with the same key |
    /// | 5 | daemon failure: nothing committed; check `system.health` |
    pub const fn exit_code(self) -> u8 {
        match self {
            ErrorCode::InvalidRequest
            | ErrorCode::UnsupportedEnvelopeVersion
            | ErrorCode::UnknownCommand
            | ErrorCode::NotInitialized
            | ErrorCode::NotFound
            | ErrorCode::VersionConflict
            | ErrorCode::IdempotencyConflict
            | ErrorCode::InvalidState => 3,
            ErrorCode::DaemonUnavailable => 4,
            ErrorCode::StorageFailure | ErrorCode::Internal | ErrorCode::Quarantined => 5,
        }
    }

    /// Whether the unchanged request may succeed if retried.
    pub const fn retryable(self) -> bool {
        matches!(
            self,
            ErrorCode::DaemonUnavailable | ErrorCode::StorageFailure | ErrorCode::Internal
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn error_codes_are_stable_and_map_to_documented_exit_codes() {
        let names: Vec<_> = ErrorCode::ALL.iter().map(|c| c.as_str()).collect();
        assert_eq!(
            names,
            [
                "invalid_request",
                "unsupported_envelope_version",
                "unknown_command",
                "not_initialized",
                "not_found",
                "version_conflict",
                "idempotency_conflict",
                "invalid_state",
                "storage_failure",
                "internal",
                "daemon_unavailable",
                "quarantined"
            ]
        );
        for &c in ErrorCode::ALL {
            assert!(matches!(c.exit_code(), 3..=5), "{c}");
            assert_eq!(c.as_str().parse::<ErrorCode>().unwrap(), c);
        }
    }

    #[test]
    fn envelope_round_trips_and_refuses_unknown_fields() {
        let req =
            Request::new("task.create", json!({"title": "x"})).with_idempotency_key("k-12345678");
        let text = serde_json::to_string(&req).unwrap();
        assert_eq!(serde_json::from_str::<Request>(&text).unwrap(), req);
        assert!(serde_json::from_str::<Request>(r#"{"v":1,"command":"x","grant":"all"}"#).is_err());

        let ok = Response::success(Some("r1".into()), json!({"a": 1}));
        assert_eq!(ok.exit_code(), 0);
        let err = Response::failure(None, ApiError::new(ErrorCode::NotFound, "no task"));
        assert_eq!(err.exit_code(), 3);
        let text = serde_json::to_string(&err).unwrap();
        assert!(text.contains(r#""code":"not_found""#), "{text}");
        assert_eq!(serde_json::from_str::<Response>(&text).unwrap(), err);
    }

    #[test]
    fn missing_params_default_to_an_empty_object() {
        let req: Request = serde_json::from_str(r#"{"v":1,"command":"state.show"}"#).unwrap();
        assert_eq!(req.params, json!({}));
    }
}
