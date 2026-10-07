//! Deletion units and referenced content (Architecture §4.11, §8, §21.3).
//!
//! A deletion unit is the smallest set of content forgotten together. Every
//! piece of sensitive content belongs to exactly one unit; Stage 01 gives each
//! unit its own data-encryption key so `forget` can destroy it. Events never
//! embed content: they hold a `ContentRef`, so the append-only ledger can keep
//! the fact that something happened after its content is forgotten.

use serde::{Deserialize, Serialize};

use crate::id::{ContentId, DeletionUnitId};

/// A reference to content held outside the ledger, with the deletion unit
/// that governs it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContentRef {
    pub content_id: ContentId,
    pub deletion_unit_id: DeletionUnitId,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::id::{IdError, IdSource};
    use ulid::Ulid;

    struct Counter(std::sync::atomic::AtomicU64);
    impl IdSource for Counter {
        fn next_ulid(&self) -> Result<Ulid, IdError> {
            let n = self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Ok(Ulid::from_parts(1_700_000_000_000, u128::from(n)))
        }
    }

    #[test]
    fn content_ref_serializes_ids_only() {
        let ids = Counter(1.into());
        let r = ContentRef {
            content_id: ContentId::generate(&ids).unwrap(),
            deletion_unit_id: DeletionUnitId::generate(&ids).unwrap(),
        };
        let json = serde_json::to_value(r).unwrap();
        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2);
        assert!(obj["content_id"].as_str().unwrap().starts_with("content_"));
        assert!(
            obj["deletion_unit_id"]
                .as_str()
                .unwrap()
                .starts_with("dunit_")
        );
        assert_eq!(serde_json::from_value::<ContentRef>(json).unwrap(), r);
    }
}
