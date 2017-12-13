//! A version of a record
//!
//! A set of version can be flattened to represent the latest set
//! of changes of a record

use std::collections::BTreeMap;
use super::Payload;

/// An operation that was applied to a version
#[derive(Debug, Serialize, Deserialize)]
pub enum Operation {
    Insert(BTreeMap<String, Payload>),
    Delete(BTreeMap<String, Payload>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    version: u64,
    operation: Operation,
}
