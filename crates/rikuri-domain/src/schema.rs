//! Schema versions for stored records, events and wire envelopes
//! (Architecture §42–43). Each record carries its own version so a reader can
//! tell which shape it holds and migrations can be tested from every
//! supported prior version.

use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A schema version. Versions start at 1; 0 never names a schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "u32", into = "u32")]
pub struct SchemaVersion(u32);

/// Version 0 was supplied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("schema version 0 is not valid; versions start at 1")]
pub struct ZeroSchemaVersion;

impl SchemaVersion {
    pub const V1: SchemaVersion = SchemaVersion(1);

    pub const fn new(version: u32) -> Option<SchemaVersion> {
        if version == 0 {
            None
        } else {
            Some(SchemaVersion(version))
        }
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for SchemaVersion {
    type Error = ZeroSchemaVersion;

    fn try_from(v: u32) -> Result<Self, ZeroSchemaVersion> {
        SchemaVersion::new(v).ok_or(ZeroSchemaVersion)
    }
}

impl From<SchemaVersion> for u32 {
    fn from(v: SchemaVersion) -> u32 {
        v.0
    }
}

impl fmt::Display for SchemaVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_refused_everywhere() {
        assert_eq!(SchemaVersion::new(0), None);
        assert!(serde_json::from_str::<SchemaVersion>("0").is_err());
        assert_eq!(
            serde_json::from_str::<SchemaVersion>("1").unwrap(),
            SchemaVersion::V1
        );
        assert_eq!(serde_json::to_string(&SchemaVersion::V1).unwrap(), "1");
    }
}
