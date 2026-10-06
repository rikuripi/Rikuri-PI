//! Pure domain types for the Rikuri Pi kernel (Stage Plan S00.04).
//!
//! This crate owns vocabulary, not behavior: stable IDs, privacy and risk
//! classes, provenance with instruction authority, deletion units, project
//! trust, schema versions, and the ID and clock seams. It performs no I/O and depends on no other
//! Rikuri crate (Architecture §45.1, "`rikuri-domain` stays pure"). See
//! `README.md` for the Module Contract.

pub mod authority;
pub mod deletion;
pub mod id;
pub mod privacy;
pub mod risk;
pub mod schema;
pub mod time;
pub mod trust;
mod wire;

pub use authority::{InstructionAuthority, Origin, Provenance, ProvenanceError};
pub use deletion::ContentRef;
pub use id::{
    ActorId, AnyId, ContentId, CostRecordId, DeletionUnitId, EventId, IdError, IdSource, ProjectId,
    RunId, SecurityDecisionId, SourceId, SourceSpanId, SystemIdSource, TaskId,
};
pub use privacy::{ExecutionIntent, PrivacyClass};
pub use risk::RiskClass;
pub use schema::SchemaVersion;
pub use time::{Clock, SystemClock, Timestamp};
pub use trust::ProjectTrust;
pub use wire::UnknownVariant;
