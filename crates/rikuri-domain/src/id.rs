//! Stable typed identifiers (ADR 0001).
//!
//! Every identity is `<prefix>_<ULID>`: a registered lowercase prefix naming
//! the kind, then 26 canonical (uppercase Crockford base32) ULID characters.
//! IDs are opaque, never reused, never derived from a path, title, provider or
//! device, and never a per-table counter (Architecture §4.2, §22). Exactly one
//! text form is accepted for each ID, so idempotency keys and ledger
//! references cannot fork on letter case.

use std::fmt;
use std::str::FromStr;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use ulid::Ulid;

use crate::wire::bounded;

/// Why an ID string or ID generation was rejected. Rejected input is held
/// bounded and escaped (at most 64 bytes plus its length), never in full.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum IdError {
    #[error("ID \"{0}\" has no `<prefix>_<ulid>` separator")]
    Malformed(String),
    #[error("ID \"{value}\" has prefix \"{found}\", expected \"{expected}\"")]
    WrongKind {
        value: String,
        expected: &'static str,
        found: String,
    },
    #[error("ID \"{0}\" has an unregistered prefix")]
    UnknownPrefix(String),
    #[error("ID \"{0}\" does not contain a valid ULID")]
    InvalidUlid(String),
    #[error("ID \"{0}\" is not in canonical form (uppercase ULID)")]
    NonCanonical(String),
    #[error("ID \"{0}\" uses the nil ULID, which is never issued")]
    Nil(String),
    #[error("ID source unavailable: {0}")]
    SourceUnavailable(String),
}

/// Produces fresh ULIDs. Production code uses [`SystemIdSource`]; tests use a
/// deterministic source (`rikuri-testkit` `FixtureIds`) through this trait.
pub trait IdSource: Send + Sync {
    fn next_ulid(&self) -> Result<Ulid, IdError>;
}

/// Wall-clock ULIDs, monotonic within one process: IDs issued later in the
/// same millisecond still sort after earlier ones.
#[derive(Debug, Default)]
pub struct SystemIdSource {
    generator: Mutex<ulid::Generator>,
}

impl SystemIdSource {
    pub fn new() -> Self {
        Self::default()
    }
}

impl IdSource for SystemIdSource {
    fn next_ulid(&self) -> Result<Ulid, IdError> {
        let mut generator = self
            .generator
            .lock()
            .map_err(|_| IdError::SourceUnavailable("generator lock poisoned".into()))?;
        generator.generate().map_err(|_| {
            IdError::SourceUnavailable("ULID space for this millisecond exhausted".into())
        })
    }
}

/// Splits `<prefix>_<ulid>` and validates the ULID part. Returns the prefix.
fn parse_parts(value: &str) -> Result<(&str, Ulid), IdError> {
    let shown = || bounded(value);
    let (prefix, encoded) = value
        .rsplit_once('_')
        .ok_or_else(|| IdError::Malformed(shown()))?;
    if prefix.is_empty() || !prefix.bytes().all(|b| b.is_ascii_lowercase()) {
        return Err(IdError::Malformed(shown()));
    }
    let ulid = Ulid::from_string(encoded).map_err(|_| IdError::InvalidUlid(shown()))?;
    if ulid.to_string() != encoded {
        return Err(IdError::NonCanonical(shown()));
    }
    if ulid == Ulid::nil() {
        return Err(IdError::Nil(shown()));
    }
    Ok((prefix, ulid))
}

/// Defines a typed ID and registers its prefix in [`PREFIXES`].
macro_rules! define_ids {
    ($( $(#[$meta:meta])* $name:ident => $prefix:literal ),+ $(,)?) => {
        /// Every registered prefix. A new kind is an additive change; a
        /// prefix is never renamed or reused (ADR 0001).
        pub const PREFIXES: &[&str] = &[$($prefix),+];

        $(
            $(#[$meta])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
            #[serde(try_from = "String", into = "String")]
            pub struct $name(Ulid);

            impl $name {
                pub const PREFIX: &'static str = $prefix;

                /// Issues a new ID from `source`.
                pub fn generate(source: &dyn IdSource) -> Result<Self, IdError> {
                    let ulid = source.next_ulid()?;
                    if ulid == Ulid::nil() {
                        return Err(IdError::Nil(ulid.to_string()));
                    }
                    Ok(Self(ulid))
                }

                /// The underlying ULID (time-sortable; carries no meaning beyond order).
                pub fn ulid(self) -> Ulid {
                    self.0
                }
            }

            impl fmt::Display for $name {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "{}_{}", $prefix, self.0)
                }
            }

            impl FromStr for $name {
                type Err = IdError;

                fn from_str(value: &str) -> Result<Self, IdError> {
                    let (prefix, ulid) = parse_parts(value)?;
                    if prefix != $prefix {
                        return Err(IdError::WrongKind {
                            value: bounded(value),
                            expected: $prefix,
                            found: bounded(prefix),
                        });
                    }
                    Ok(Self(ulid))
                }
            }

            impl TryFrom<String> for $name {
                type Error = IdError;

                fn try_from(value: String) -> Result<Self, IdError> {
                    value.parse()
                }
            }

            impl From<$name> for String {
                fn from(id: $name) -> String {
                    id.to_string()
                }
            }

            impl From<$name> for AnyId {
                fn from(id: $name) -> AnyId {
                    AnyId { prefix: $prefix, ulid: id.0 }
                }
            }
        )+
    };
}

define_ids! {
    /// A project: a persistent goal and work container.
    ProjectId => "project",
    /// A task: an executable unit of work.
    TaskId => "task",
    /// One execution attempt of a task.
    RunId => "run",
    /// An append-only ledger event.
    EventId => "evt",
    /// A human, device, worker or service that acted.
    ActorId => "actor",
    /// The smallest set of content forgotten together (see `deletion`).
    DeletionUnitId => "dunit",
    /// Referenced content (blob or external resource) held outside events.
    ContentId => "content",
    /// An imported or captured source.
    SourceId => "source",
    /// An exact provenance pointer within a source.
    SourceSpanId => "span",
    /// A recorded capability, privacy or approval decision.
    SecurityDecisionId => "secdec",
    /// A recorded cost or budget entry.
    CostRecordId => "cost",
}

/// An ID of any registered kind, for fields such as an event's subject list
/// that may reference several kinds. It keeps its prefix, so it never loses
/// what it refers to.
///
/// `Deserialize` is written by hand: the prefix is held as `&'static str`,
/// and a derived impl would accept only `'static` input, so an `AnyId` could
/// not be read from a stored row (regression test below).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
#[serde(into = "String")]
pub struct AnyId {
    prefix: &'static str,
    ulid: Ulid,
}

impl AnyId {
    /// The registered prefix naming this ID's kind.
    pub fn prefix(self) -> &'static str {
        self.prefix
    }

    /// Converts to a specific kind, failing if the kind differs.
    pub fn downcast<T: FromStr<Err = IdError>>(self) -> Result<T, IdError> {
        self.to_string().parse()
    }
}

impl fmt::Display for AnyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}_{}", self.prefix, self.ulid)
    }
}

impl FromStr for AnyId {
    type Err = IdError;

    fn from_str(value: &str) -> Result<Self, IdError> {
        let (prefix, ulid) = parse_parts(value)?;
        let prefix = PREFIXES
            .iter()
            .copied()
            .find(|p| *p == prefix)
            .ok_or_else(|| IdError::UnknownPrefix(bounded(value)))?;
        Ok(AnyId { prefix, ulid })
    }
}

impl<'de> Deserialize<'de> for AnyId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let text = <std::borrow::Cow<'de, str>>::deserialize(d)?;
        text.parse().map_err(serde::de::Error::custom)
    }
}

impl TryFrom<String> for AnyId {
    type Error = IdError;

    fn try_from(value: String) -> Result<Self, IdError> {
        value.parse()
    }
}

impl From<AnyId> for String {
    fn from(id: AnyId) -> String {
        id.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fixed(u128);
    impl IdSource for Fixed {
        fn next_ulid(&self) -> Result<Ulid, IdError> {
            Ok(Ulid(self.0))
        }
    }

    #[test]
    fn display_is_prefix_then_canonical_ulid() {
        let id = TaskId::generate(&Fixed(0x0191_2345_6789_abcd_ef01_2345_6789_abcd)).unwrap();
        let text = id.to_string();
        assert!(text.starts_with("task_"));
        assert_eq!(text.len(), "task_".len() + 26);
        assert_eq!(text.parse::<TaskId>().unwrap(), id);
    }

    #[test]
    fn wrong_kind_is_rejected() {
        let id = TaskId::generate(&Fixed(42)).unwrap();
        let err = id.to_string().parse::<EventId>().unwrap_err();
        assert!(
            matches!(
                err,
                IdError::WrongKind {
                    expected: "evt",
                    ..
                }
            ),
            "{err:?}"
        );
    }

    #[test]
    fn non_canonical_and_malformed_text_is_rejected() {
        let id = TaskId::generate(&Fixed(42)).unwrap().to_string();
        assert!(matches!(
            id.to_lowercase().parse::<TaskId>(),
            Err(IdError::Malformed(_)) | Err(IdError::NonCanonical(_))
        ));
        let lower_ulid = format!("task_{}", id["task_".len()..].to_lowercase());
        assert!(matches!(
            lower_ulid.parse::<TaskId>(),
            Err(IdError::NonCanonical(_))
        ));
        for bad in [
            "",
            "task",
            "task_",
            "_01ARZ3NDEKTSV4RRFFQ69G5FAV",
            "Task_01ARZ3NDEKTSV4RRFFQ69G5FAV",
            "task_01ARZ3NDEKTSV4RRFFQ69G5FA",
            "task_01ARZ3NDEKTSV4RRFFQ69G5FAVV",
            "task_U1ARZ3NDEKTSV4RRFFQ69G5FAV",
        ] {
            assert!(bad.parse::<TaskId>().is_err(), "{bad:?} accepted");
        }
    }

    #[test]
    fn nil_ulid_is_never_an_id() {
        assert!(matches!(TaskId::generate(&Fixed(0)), Err(IdError::Nil(_))));
        assert!(matches!(
            "task_00000000000000000000000000".parse::<TaskId>(),
            Err(IdError::Nil(_))
        ));
    }

    #[test]
    fn prefixes_are_unique_lowercase_words() {
        let mut seen = std::collections::HashSet::new();
        for p in PREFIXES {
            assert!(seen.insert(*p), "duplicate prefix {p}");
            assert!(
                !p.is_empty() && p.bytes().all(|b| b.is_ascii_lowercase()),
                "bad prefix {p}"
            );
        }
    }

    #[test]
    fn any_id_keeps_kind_and_downcasts_only_to_it() {
        let evt = EventId::generate(&Fixed(7)).unwrap();
        let any: AnyId = evt.into();
        assert_eq!(any.prefix(), "evt");
        assert_eq!(any.to_string(), evt.to_string());
        assert_eq!(any.downcast::<EventId>().unwrap(), evt);
        assert!(any.downcast::<TaskId>().is_err());
        assert!(matches!(
            "nope_01ARZ3NDEKTSV4RRFFQ69G5FAV".parse::<AnyId>(),
            Err(IdError::UnknownPrefix(_))
        ));
    }

    #[test]
    fn serde_uses_the_text_form_and_validates() {
        let id = RunId::generate(&Fixed(99)).unwrap();
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, format!("\"{id}\""));
        assert_eq!(serde_json::from_str::<RunId>(&json).unwrap(), id);
        assert!(serde_json::from_str::<TaskId>(&json).is_err());
    }

    #[test]
    fn system_source_is_monotonic() {
        let source = SystemIdSource::new();
        let mut previous = TaskId::generate(&source).unwrap();
        for _ in 0..10_000 {
            let next = TaskId::generate(&source).unwrap();
            assert!(next > previous, "{next} not after {previous}");
            assert!(
                next.to_string() > previous.to_string(),
                "text order differs from ID order"
            );
            previous = next;
        }
    }

    /// Regression (found building the ledger, 2026-10-05): `AnyId` could be
    /// deserialized only from `'static` text, so reading it from a stored
    /// row did not compile.
    #[test]
    fn any_id_deserializes_from_runtime_text() {
        let id: AnyId = EventId::generate(&Fixed(7)).unwrap().into();
        let text: String = serde_json::to_string(&vec![id]).unwrap();
        let parsed: Vec<AnyId> = serde_json::from_str(&text).unwrap();
        assert_eq!(parsed, vec![id]);
        let bad = String::from("[\"nope_01ARZ3NDEKTSV4RRFFQ69G5FAV\"]");
        assert!(serde_json::from_str::<Vec<AnyId>>(&bad).is_err());
    }

    /// Regression (PR #7 review, 2026-10-05): a rejected ID echoed the whole
    /// untrusted input into the error, so a 1 MiB input made a 1 MiB error.
    #[test]
    fn errors_do_not_echo_unbounded_input() {
        let huge = format!("task_{}", "A".repeat(1 << 20));
        for err in [
            huge.parse::<TaskId>().unwrap_err(),
            huge.parse::<AnyId>().unwrap_err(),
            "x".repeat(1 << 20).parse::<TaskId>().unwrap_err(),
        ] {
            let shown = err.to_string();
            assert!(shown.len() < 200, "error is {} bytes", shown.len());
        }
    }
}
