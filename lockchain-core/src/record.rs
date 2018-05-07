//! Common record representation inside of vaults.
//!
//! A record is a collection of data stored in a vault. It's
//! structured into a publicly known, unencrypted header and
//! a securely saved, encrypted body.
//!
//! While the `lockchain-server` never has access to the body data,
//! the header is stored and cached for make search requests faster.
//!
//! **No secret information should ever be stored in the header**

use chrono::{DateTime, Local};
use std::collections::BTreeMap;
use serde::{de::DeserializeOwned, Serialize};

// use traits::Body;

/// An enum that wraps around all possible data types to store
/// as the value of a vault record.
///
/// This doesn't include metadata attached to a field, just the
/// data representation itself (i.e. text, number or sub data-tree)
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum Payload {
    Text(String),
    Boolean(bool),
    Number(i64),
    BTreeMap(BTreeMap<String, Payload>),
}

/// The public header of a record
///
/// A header consists of always-available fields that
/// are hard-defined in the lockchain file format as well
/// as custom fields that can be declared by each application
/// specifically.
///
/// You should never rely on the presence of custom fields as
/// older version of the software might not support them or
/// know about them!
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Header {
    /// The name of this record
    pub name: String,
    /// The primary category the record is in
    pub category: String,
    /// A collection of custom tags
    pub tags: Vec<String>,
    /// Custom fields to query by. **Do not store secure data in fields!
    pub fields: BTreeMap<String, Payload>,
    /// Timestamp when the record was created
    pub date_created: DateTime<Local>,
    /// Timestamp when the record was last updated
    pub date_updated: DateTime<Local>,
}

/// Represents a whole record in memory
///
/// The body field can be `None` if it hasn't been cached
/// yet. Calling `body()` will either resolve the data from disk
/// or still return `None` if the current setting doesn't support
/// body loading (such as the  `lockchain-server` which has no
/// cryptocraphy subsystem)
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Record<T: Body> {
    pub header: Header,
    #[serde(bound(deserialize = "T: Body"))]
    pub body: Option<T>,
}

/// A Body trait that can be implemented to hook into the generic Record
/// data module.
///
/// This allows working with both encrypted and cleartext data bodies.
pub trait Body: DeserializeOwned + Serialize {
    ///Get the value of a field from this body
    fn get_field(&self, key: &str) -> Option<Payload>;
    /// Set the value of a field
    fn set_field(&mut self, key: &str, value: &Payload);
    /// Remove versioning and flatten the data tree to a single level.
    fn flatten(&mut self);
}
