//! Stage 00 commands and their parameters. Every parameter struct refuses
//! unknown fields, so a misspelled or injected field is an error rather than
//! silently ignored.
//!
//! | Command | Consequential | Parameters |
//! |---|---|---|
//! | `system.init` | yes (built-in key) | none |
//! | `system.health` | no | none |
//! | `task.create` | yes | [`TaskCreate`] |
//! | `task.start` | yes | [`TaskStart`] |
//! | `task.step` | yes | [`TaskStep`] |
//! | `task.complete` | yes | [`TaskComplete`] |
//! | `state.show` | no | none |
//! | `ledger.tail` | no | [`LedgerTail`] |

use rikuri_domain::{PrivacyClass, RunId, TaskId, wire_enum};
use serde::{Deserialize, Serialize};

pub const SYSTEM_INIT: &str = "system.init";
pub const SYSTEM_HEALTH: &str = "system.health";
pub const TASK_CREATE: &str = "task.create";
pub const TASK_START: &str = "task.start";
pub const TASK_STEP: &str = "task.step";
pub const TASK_COMPLETE: &str = "task.complete";
pub const STATE_SHOW: &str = "state.show";
pub const LEDGER_TAIL: &str = "ledger.tail";

/// Every command, in the order above.
pub const ALL: &[&str] = &[
    SYSTEM_INIT,
    SYSTEM_HEALTH,
    TASK_CREATE,
    TASK_START,
    TASK_STEP,
    TASK_COMPLETE,
    STATE_SHOW,
    LEDGER_TAIL,
];

/// Commands that change canonical state and therefore need an idempotency
/// key. `system.init` uses a fixed built-in key: initialization happens once.
pub fn is_consequential(command: &str) -> bool {
    matches!(
        command,
        SYSTEM_INIT | TASK_CREATE | TASK_START | TASK_STEP | TASK_COMPLETE
    )
}

/// Parameters of commands that take none.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NoParams {}

/// `task.create`: a sample task in the owner's default project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskCreate {
    /// 1–200 characters. Stored as task content, never copied into events.
    pub title: String,
    /// Defaults to `normal` for this owner-entered sample task.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy_class: Option<PrivacyClass>,
}

/// `task.start`: begin a new run (attempt) of a task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskStart {
    pub task_id: TaskId,
    pub expected_version: u64,
}

/// `task.step`: record one unit of work on the running run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskStep {
    pub task_id: TaskId,
    pub run_id: RunId,
    pub expected_version: u64,
}

wire_enum! {
    /// How a run ended. Completion, not proof of success.
    pub enum Outcome {
        Succeeded => "succeeded",
        Failed => "failed",
        Cancelled => "cancelled",
    }
}

wire_enum! {
    /// What independent verification found. Separate from the outcome: a
    /// run can report success that verification did not confirm
    /// (Architecture §4 invariant 10).
    pub enum VerificationStatus {
        Passed => "passed",
        Failed => "failed",
        NotRun => "not_run",
    }
}

/// `task.complete`: end the run with an outcome and a verification result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskComplete {
    pub task_id: TaskId,
    pub run_id: RunId,
    pub expected_version: u64,
    pub outcome: Outcome,
    pub verification: VerificationStatus,
}

/// `ledger.tail`: the most recent events.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LedgerTail {
    /// 1–1000; default 20.
    #[serde(default = "default_tail")]
    pub limit: u32,
}

fn default_tail() -> u32 {
    20
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parameters_refuse_unknown_fields() {
        assert!(serde_json::from_value::<TaskCreate>(json!({"title": "t"})).is_ok());
        assert!(
            serde_json::from_value::<TaskCreate>(json!({"title": "t", "approve": true})).is_err()
        );
        assert!(serde_json::from_value::<NoParams>(json!({"x": 1})).is_err());
        assert_eq!(
            serde_json::from_value::<LedgerTail>(json!({}))
                .unwrap()
                .limit,
            20
        );
    }

    #[test]
    fn only_state_changing_commands_are_consequential() {
        let consequential: Vec<_> = ALL.iter().filter(|c| is_consequential(c)).collect();
        assert_eq!(
            consequential,
            [
                &SYSTEM_INIT,
                &TASK_CREATE,
                &TASK_START,
                &TASK_STEP,
                &TASK_COMPLETE
            ]
        );
    }

    #[test]
    fn outcome_and_verification_are_distinct_vocabularies() {
        assert_eq!(
            serde_json::to_value(Outcome::Succeeded).unwrap(),
            json!("succeeded")
        );
        assert_eq!(
            serde_json::to_value(VerificationStatus::NotRun).unwrap(),
            json!("not_run")
        );
        assert!("passed".parse::<Outcome>().is_err());
    }
}
