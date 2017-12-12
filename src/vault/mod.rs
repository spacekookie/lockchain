//! Vault data module
//! 
//! A vault represents a collection of records of sensitive data. Each record
//! is encrypted before being written to disk.
//! 
//! A vault can have multiple users which allows login-information to be
//! shared between multiple people. By default only one (root) user
//! is enabled though.
//! 

mod management;
mod version;
mod record;
mod user;
pub mod crypto;


use std::collections::BTreeMap;
use chrono::{DateTime, Local};


/// A generic payload for a record
#[derive(Debug, Serialize, Deserialize)]
pub enum Payload {
    String,
    bool,
    i64,
    BTreeMap,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Header {
    pub name: String,
    pub category: String,
    pub tags: Vec<String>,
    pub date_created: DateTime<Local>,
    pub date_updated: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub header: Header,
    body: BTreeMap<String, Payload>
}


/// A vault that represents a collection of records of sensitive data.
/// Each record is encrypted before being written to disk.
/// 
/// A vault can have multiple users which allows login-information to be
/// shared between multiple people. By default only one (root) user
/// is enabled though.
pub struct Vault {

    
}