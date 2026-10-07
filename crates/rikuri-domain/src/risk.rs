//! Risk classes for operations (PRD SEC-003).

use crate::wire::wire_enum;

wire_enum! {
    /// How consequential an operation is. Ordered from least to most.
    pub enum RiskClass {
        /// Observes or reads; changes nothing.
        ObserveReadOnly => "observe_read_only",
        /// A known, previously validated repair.
        KnownRepair => "known_repair",
        /// A new modification.
        NovelModification => "novel_modification",
        /// Destructive or otherwise high-consequence.
        DestructiveHighConsequence => "destructive_high_consequence",
        /// Touches money, credentials or signing.
        FinancialCredential => "financial_credential",
    }
}

impl RiskClass {
    /// A consequential operation changes something. It needs an idempotency
    /// key so a retry or a crash replay cannot apply it twice
    /// (Architecture §8).
    pub const fn is_consequential(self) -> bool {
        !matches!(self, RiskClass::ObserveReadOnly)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wire::tests::assert_wire_roundtrip;

    #[test]
    fn wire_names_are_stable() {
        assert_wire_roundtrip(RiskClass::ALL, RiskClass::as_str);
    }

    #[test]
    fn only_read_only_is_inconsequential() {
        for &r in RiskClass::ALL {
            assert_eq!(r.is_consequential(), r != RiskClass::ObserveReadOnly, "{r}");
        }
    }
}
