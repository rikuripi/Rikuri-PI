//! The documented CLI exit code and retryable flag of every error code,
//! pinned one by one (2026-10-08: mapping every failure to 3 passed the
//! whole suite, which only checked the 3–5 range).

use rikuri_api::ErrorCode;

#[test]
fn every_error_code_has_its_documented_exit_code_and_retry_flag() {
    let documented: &[(&str, u8, bool)] = &[
        ("invalid_request", 3, false),
        ("unsupported_envelope_version", 3, false),
        ("unknown_command", 3, false),
        ("not_initialized", 3, false),
        ("not_found", 3, false),
        ("version_conflict", 3, false),
        ("idempotency_conflict", 3, false),
        ("invalid_state", 3, false),
        ("storage_failure", 5, true),
        ("internal", 5, true),
        ("daemon_unavailable", 4, true),
        ("quarantined", 5, false),
    ];
    assert_eq!(
        ErrorCode::ALL.len(),
        documented.len(),
        "a code is missing here"
    );
    for &(name, exit, retryable) in documented {
        let code: ErrorCode = name.parse().unwrap();
        assert_eq!(code.exit_code(), exit, "{name} exit code");
        assert_eq!(code.retryable(), retryable, "{name} retryable");
    }
}
