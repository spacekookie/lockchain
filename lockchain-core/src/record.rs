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


use std::collections::BTreeMap;
use chrono::{Local, DateTime};


/// An enum that wraps around all possible data types to store
/// as the value of a vault record.
/// 
/// This doesn't include metadata attached to a field, just the
/// data representation itself (i.e. text, number or sub data-tree)
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Header {
    pub name: String,
    pub category: String,
    pub tags: Vec<String>,
    pub fields: BTreeMap<String, Payload>,
    pub date_created: DateTime<Local>,
    pub date_updated: DateTime<Local>,
}

/// Represents a whole record in memory
/// 
/// The body field can be `None` if it hasn't been cached
/// yet. Calling `body()` will either resolve the data from disk
/// or still return `None` if the current setting doesn't support
/// body loading (such as the  `lockchain-server` which has no
/// cryptocraphy subsystem)
#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub header: Header,
    pub body: Option<BTreeMap<String, Payload>>,
}
