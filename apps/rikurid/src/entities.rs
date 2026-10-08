//! The kernel's Stage 00 entities: the owner, the owner's default project,
//! sample tasks and their runs. They live in the object registry
//! (`rikuri-storage`); their history lives in the ledger.

use rikuri_api::command::{Outcome, VerificationStatus};
use rikuri_domain::{
    ActorId, DeletionUnitId, EventId, ExecutionIntent, PrivacyClass, ProjectId, ProjectTrust,
    RunId, SchemaVersion, TaskId, Timestamp, wire_enum,
};
use rikuri_storage::Entity;
use serde::{Deserialize, Serialize};

/// The Personal Intelligence owner, created once by `system.init`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Owner {
    pub id: ActorId,
    pub project_id: ProjectId,
    pub initialized_at: Timestamp,
}

impl Entity for Owner {
    type Id = ActorId;
    const KIND: &'static str = "actor";
    const SCHEMA: SchemaVersion = SchemaVersion::V1;
    fn id(&self) -> ActorId {
        self.id
    }
    fn privacy_class(&self) -> PrivacyClass {
        PrivacyClass::Normal
    }
}

/// The owner's default project. New projects are untrusted for execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub trust: ProjectTrust,
    pub intent: ExecutionIntent,
}

impl Entity for Project {
    type Id = ProjectId;
    const KIND: &'static str = "project";
    const SCHEMA: SchemaVersion = SchemaVersion::V1;
    fn id(&self) -> ProjectId {
        self.id
    }
    fn privacy_class(&self) -> PrivacyClass {
        PrivacyClass::Normal
    }
}

wire_enum! {
    pub enum TaskStatus {
        Created => "created",
        Running => "running",
        Completed => "completed",
    }
}

/// A sample task. Its title is owner content: it carries a privacy class
/// and its own deletion unit, and no event ever copies it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Task {
    pub id: TaskId,
    pub project_id: ProjectId,
    pub title: String,
    pub privacy_class: PrivacyClass,
    pub deletion_unit_id: DeletionUnitId,
    pub status: TaskStatus,
    pub steps_completed: u32,
    pub runs: u32,
    pub current_run: Option<RunId>,
    pub last_run: Option<RunId>,
}

impl Entity for Task {
    type Id = TaskId;
    const KIND: &'static str = "task";
    const SCHEMA: SchemaVersion = SchemaVersion::V1;
    fn id(&self) -> TaskId {
        self.id
    }
    fn privacy_class(&self) -> PrivacyClass {
        self.privacy_class
    }
    fn deletion_unit(&self) -> Option<DeletionUnitId> {
        Some(self.deletion_unit_id)
    }
}

wire_enum! {
    pub enum RunStatus {
        Running => "running",
        Completed => "completed",
    }
}

/// One attempt at a task. Completion (the outcome) and verification are
/// recorded separately, each linked to its own ledger event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Run {
    pub id: RunId,
    pub task_id: TaskId,
    pub attempt: u32,
    pub status: RunStatus,
    pub steps: u32,
    pub outcome: Option<Outcome>,
    pub verification: Option<VerificationStatus>,
    pub started_event_id: EventId,
    pub completion_event_id: Option<EventId>,
    pub verification_event_id: Option<EventId>,
}

impl Entity for Run {
    type Id = RunId;
    const KIND: &'static str = "run";
    const SCHEMA: SchemaVersion = SchemaVersion::V1;
    fn id(&self) -> RunId {
        self.id
    }
    fn privacy_class(&self) -> PrivacyClass {
        PrivacyClass::Normal
    }
}
