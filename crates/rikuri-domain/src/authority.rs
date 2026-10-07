//! Provenance and instruction authority (Architecture §4.7, §19.1, §27.4).
//!
//! Instruction authority is derived from where content came from, never
//! stated by the content. Only a live command from the current user carries
//! `current_user` authority. Imported, retrieved, web, tool and model content,
//! including text the user wrote in the past, carries `none`. `trusted_policy`
//! authority comes from Rikuri's policy service (Stage 02), never from content
//! provenance, so a `Provenance` value cannot hold it.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::deletion::ContentRef;
use crate::id::{SourceId, SourceSpanId};
use crate::privacy::PrivacyClass;
use crate::wire::wire_enum;

wire_enum! {
    /// Where a piece of content came from.
    pub enum Origin {
        /// The current authenticated user's live command.
        UserLive => "user_live",
        /// Text the user wrote earlier (notes, past messages).
        UserAuthored => "user_authored",
        /// Imported history or files.
        Imported => "imported",
        /// Web content.
        Web => "web",
        /// Output of a tool.
        ToolOutput => "tool_output",
        /// Output of a model.
        ModelOutput => "model_output",
    }
}

wire_enum! {
    /// Whether content may act as an instruction.
    pub enum InstructionAuthority {
        /// The current authenticated user's command.
        CurrentUser => "current_user",
        /// Rikuri's trusted policy (granted by the policy service only).
        TrustedPolicy => "trusted_policy",
        /// Data only: no authority, whatever the text says.
        None => "none",
    }
}

impl Origin {
    /// The only instruction authority content from this origin can carry.
    pub const fn content_authority(self) -> InstructionAuthority {
        match self {
            Origin::UserLive => InstructionAuthority::CurrentUser,
            Origin::UserAuthored
            | Origin::Imported
            | Origin::Web
            | Origin::ToolOutput
            | Origin::ModelOutput => InstructionAuthority::None,
        }
    }
}

/// A provenance record whose authority disagrees with its origin.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("provenance with origin {origin} cannot carry instruction authority {claimed}")]
pub struct ProvenanceError {
    pub origin: Origin,
    pub claimed: InstructionAuthority,
}

/// Where a content segment came from, what authority it carries and how
/// private it is. Fields are private: authority is computed from origin on
/// construction and validated on deserialization, so no caller and no stored
/// record can raise it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "ProvenanceRecord", into = "ProvenanceRecord")]
pub struct Provenance {
    origin: Origin,
    instruction_authority: InstructionAuthority,
    privacy_class: PrivacyClass,
    source_id: Option<SourceId>,
    source_span_id: Option<SourceSpanId>,
    content_ref: Option<ContentRef>,
}

impl Provenance {
    pub fn new(origin: Origin, privacy_class: PrivacyClass) -> Self {
        Provenance {
            origin,
            instruction_authority: origin.content_authority(),
            privacy_class,
            source_id: None,
            source_span_id: None,
            content_ref: None,
        }
    }

    pub fn with_source(mut self, source_id: SourceId, span: Option<SourceSpanId>) -> Self {
        self.source_id = Some(source_id);
        self.source_span_id = span;
        self
    }

    pub fn with_content(mut self, content_ref: ContentRef) -> Self {
        self.content_ref = Some(content_ref);
        self
    }

    pub fn origin(&self) -> Origin {
        self.origin
    }

    pub fn instruction_authority(&self) -> InstructionAuthority {
        self.instruction_authority
    }

    pub fn privacy_class(&self) -> PrivacyClass {
        self.privacy_class
    }

    pub fn source_id(&self) -> Option<SourceId> {
        self.source_id
    }

    pub fn source_span_id(&self) -> Option<SourceSpanId> {
        self.source_span_id
    }

    pub fn content_ref(&self) -> Option<ContentRef> {
        self.content_ref
    }
}

/// The serialized form. Deserialization goes through `TryFrom`, which
/// refuses any authority other than the one the origin implies.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProvenanceRecord {
    origin: Origin,
    instruction_authority: InstructionAuthority,
    privacy_class: PrivacyClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    source_id: Option<SourceId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    source_span_id: Option<SourceSpanId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    content_ref: Option<ContentRef>,
}

impl TryFrom<ProvenanceRecord> for Provenance {
    type Error = ProvenanceError;

    fn try_from(r: ProvenanceRecord) -> Result<Self, ProvenanceError> {
        if r.instruction_authority != r.origin.content_authority() {
            return Err(ProvenanceError {
                origin: r.origin,
                claimed: r.instruction_authority,
            });
        }
        Ok(Provenance {
            origin: r.origin,
            instruction_authority: r.instruction_authority,
            privacy_class: r.privacy_class,
            source_id: r.source_id,
            source_span_id: r.source_span_id,
            content_ref: r.content_ref,
        })
    }
}

impl From<Provenance> for ProvenanceRecord {
    fn from(p: Provenance) -> Self {
        ProvenanceRecord {
            origin: p.origin,
            instruction_authority: p.instruction_authority,
            privacy_class: p.privacy_class,
            source_id: p.source_id,
            source_span_id: p.source_span_id,
            content_ref: p.content_ref,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wire::tests::assert_wire_roundtrip;

    #[test]
    fn wire_names_are_stable() {
        assert_wire_roundtrip(Origin::ALL, Origin::as_str);
        assert_wire_roundtrip(InstructionAuthority::ALL, InstructionAuthority::as_str);
    }

    #[test]
    fn only_a_live_user_command_has_authority() {
        for &origin in Origin::ALL {
            let p = Provenance::new(origin, PrivacyClass::Normal);
            let expected = if origin == Origin::UserLive {
                InstructionAuthority::CurrentUser
            } else {
                InstructionAuthority::None
            };
            assert_eq!(p.instruction_authority(), expected, "{origin}");
            assert_ne!(
                p.instruction_authority(),
                InstructionAuthority::TrustedPolicy
            );
        }
    }

    #[test]
    fn stored_records_cannot_raise_authority() {
        for &origin in Origin::ALL {
            for &claimed in InstructionAuthority::ALL {
                let json = format!(
                    r#"{{"origin":"{origin}","instruction_authority":"{claimed}","privacy_class":"normal"}}"#
                );
                let parsed = serde_json::from_str::<Provenance>(&json);
                if claimed == origin.content_authority() {
                    assert!(parsed.is_ok(), "{json}");
                } else {
                    assert!(parsed.is_err(), "accepted {json}");
                }
            }
        }
    }

    #[test]
    fn roundtrips_and_rejects_unknown_fields() {
        let p = Provenance::new(Origin::Imported, PrivacyClass::Private);
        let json = serde_json::to_string(&p).unwrap();
        assert_eq!(serde_json::from_str::<Provenance>(&json).unwrap(), p);
        let extra = r#"{"origin":"imported","instruction_authority":"none","privacy_class":"private","grant":"all"}"#;
        assert!(serde_json::from_str::<Provenance>(extra).is_err());
    }
}
