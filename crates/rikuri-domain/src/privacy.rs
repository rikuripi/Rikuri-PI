//! Data privacy classes and execution intent (Architecture §19.1, §27.14;
//! PRD §10.1). These are two different types: a privacy class describes
//! data; an execution intent is the user's routing choice for a project or
//! task. "Private" exists in both, with different meanings.

use crate::wire_enum;

wire_enum! {
    /// How restricted a piece of data is. Ordered from least to most
    /// restrictive, so `max` of two classes is the stricter one.
    pub enum PrivacyClass {
        /// Ordinary data.
        Normal => "normal",
        /// Personal data; external processing only under admission rules.
        Private => "private",
        /// Never enters a cloud request; only a proven no-network envelope.
        LocalOnly => "local_only",
        /// Never enters any model context, local or cloud.
        Secret => "secret",
    }
}

impl PrivacyClass {
    /// The class applied to content nobody has classified yet. Unknown
    /// content gets conservative local treatment until an authorized
    /// classification decision (threat model §2).
    pub const UNCLASSIFIED: PrivacyClass = PrivacyClass::LocalOnly;

    /// The class of material derived from both inputs: derived data inherits
    /// the strictest source restriction (PRD SEC-023). Only a separate,
    /// authorized declassification may lower it; text never can.
    pub fn combine(self, other: PrivacyClass) -> PrivacyClass {
        self.max(other)
    }
}

wire_enum! {
    /// The user's routing choice for work (Architecture §4.30).
    #[derive(Default)]
    pub enum ExecutionIntent {
        /// Rikuri chooses within policy (the default).
        #[default]
        Auto => "auto",
        /// Prefer private paths for this work.
        Private => "private",
        /// Local execution only, in a proven no-network envelope.
        LocalOnly => "local_only",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wire::tests::assert_wire_roundtrip;

    #[test]
    fn wire_names_are_stable() {
        assert_wire_roundtrip(PrivacyClass::ALL, PrivacyClass::as_str);
        assert_wire_roundtrip(ExecutionIntent::ALL, ExecutionIntent::as_str);
        let names: Vec<_> = PrivacyClass::ALL.iter().map(|c| c.as_str()).collect();
        assert_eq!(names, ["normal", "private", "local_only", "secret"]);
    }

    #[test]
    fn combining_never_lowers_restriction() {
        for &a in PrivacyClass::ALL {
            for &b in PrivacyClass::ALL {
                let c = a.combine(b);
                assert!(c >= a && c >= b, "{a} + {b} gave {c}");
                assert!(c == a || c == b);
            }
        }
        assert_eq!(
            PrivacyClass::Normal.combine(PrivacyClass::Secret),
            PrivacyClass::Secret
        );
    }

    #[test]
    fn unclassified_is_conservative() {
        assert!(PrivacyClass::UNCLASSIFIED > PrivacyClass::Private);
        assert_eq!(ExecutionIntent::default(), ExecutionIntent::Auto);
    }

    #[test]
    fn unknown_values_are_refused() {
        assert!("public".parse::<PrivacyClass>().is_err());
        assert!("confidential".parse::<PrivacyClass>().is_err());
        assert!("LocalOnly".parse::<PrivacyClass>().is_err());
    }
}
