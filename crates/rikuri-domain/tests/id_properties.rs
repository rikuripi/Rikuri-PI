//! Property tests for stable IDs (ADR 0001).

use proptest::prelude::*;
use rikuri_domain::id::PREFIXES;
use rikuri_domain::{AnyId, EventId, IdError, IdSource, TaskId};
use ulid::Ulid;

struct Fixed(Ulid);
impl IdSource for Fixed {
    fn next_ulid(&self) -> Result<Ulid, IdError> {
        Ok(self.0)
    }
}

proptest! {
    /// Parsing arbitrary text never panics, and anything accepted prints back
    /// to exactly the same text: one text form per ID.
    #[test]
    fn parsing_is_total_and_canonical(text in "\\PC{0,60}") {
        if let Ok(id) = text.parse::<AnyId>() {
            prop_assert_eq!(id.to_string(), text.clone());
        }
        if let Ok(id) = text.parse::<TaskId>() {
            prop_assert_eq!(id.to_string(), text);
        }
    }

    /// Every non-nil ULID makes a valid ID that round-trips through text and
    /// JSON as its own kind and is refused as any other kind.
    #[test]
    fn issued_ids_roundtrip_only_as_their_kind(raw in 1u128..=u128::MAX) {
        let task = TaskId::generate(&Fixed(Ulid(raw))).unwrap();
        let text = task.to_string();
        prop_assert_eq!(text.parse::<TaskId>().unwrap(), task);
        prop_assert!(text.parse::<EventId>().is_err());
        let json = serde_json::to_string(&task).unwrap();
        prop_assert_eq!(serde_json::from_str::<TaskId>(&json).unwrap(), task);
        let any: AnyId = text.parse().unwrap();
        prop_assert_eq!(any.prefix(), "task");
    }

    /// ID order equals text order, so ULID time-sorting survives storage as text.
    #[test]
    fn text_order_matches_id_order(a in 1u128..=u128::MAX, b in 1u128..=u128::MAX) {
        let x = TaskId::generate(&Fixed(Ulid(a))).unwrap();
        let y = TaskId::generate(&Fixed(Ulid(b))).unwrap();
        prop_assert_eq!(x.cmp(&y), x.to_string().cmp(&y.to_string()));
    }

    /// A registered prefix with a valid ULID always parses as AnyId.
    #[test]
    fn every_registered_prefix_parses(idx in 0usize..PREFIXES.len(), raw in 1u128..=u128::MAX) {
        let text = format!("{}_{}", PREFIXES[idx], Ulid(raw));
        let any: AnyId = text.parse().unwrap();
        prop_assert_eq!(any.prefix(), PREFIXES[idx]);
    }
}
