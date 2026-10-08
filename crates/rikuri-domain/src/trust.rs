//! Project execution trust (Architecture §4.31, §27.6; Stage Plan S03.04).
//!
//! Reading a project never implies executing it. Promotion and revocation
//! rules (explicit, scoped, rendered consequences) belong to Stage 03; this
//! type fixes the states and their meaning so storage and events can record
//! them from the start.

use crate::wire_enum;

wire_enum! {
    /// Whether project-controlled code may run.
    #[derive(Default)]
    pub enum ProjectTrust {
        /// Default: open, read, index and search only.
        #[default]
        UntrustedRead => "untrusted_read",
        /// Explicitly promoted for scoped execution inside an envelope.
        TrustedScopedExecute => "trusted_scoped_execute",
        /// Trust revoked or invalidated; review required before execution.
        RevokedReviewRequired => "revoked_review_required",
    }
}

impl ProjectTrust {
    /// True only after an explicit scoped promotion.
    pub const fn permits_execution(self) -> bool {
        matches!(self, ProjectTrust::TrustedScopedExecute)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wire::tests::assert_wire_roundtrip;

    #[test]
    fn wire_names_are_stable() {
        assert_wire_roundtrip(ProjectTrust::ALL, ProjectTrust::as_str);
    }

    #[test]
    fn new_projects_cannot_execute() {
        assert_eq!(ProjectTrust::default(), ProjectTrust::UntrustedRead);
        assert!(!ProjectTrust::default().permits_execution());
        assert!(!ProjectTrust::RevokedReviewRequired.permits_execution());
        assert!(ProjectTrust::TrustedScopedExecute.permits_execution());
    }
}
