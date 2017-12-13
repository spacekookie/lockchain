//! Lockchain record handling module
//! 
//! A record is a set of key-value store values with a header
//! 

mod version;

use std::collections::BTreeMap;
use chrono::{DateTime, Local};


/// A generic payload for a record
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Payload {
    Text(String),
    Boolean(bool),
    Number(i64),
    BTreeMap(BTreeMap<String, Payload>),
}

/// Describes the header of a record file
/// 
/// This part of the record should not be considered safe as it is
/// serialised and cached multiple times.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Header {

    /// The name of this record
    pub name: String,

    /// The primary category used for sorting
    pub category: String,

    /// A dynamic collection of fields. User-configurable
    /// In most cases this is where website URLs can be stored
    pub fields: BTreeMap<String, Payload>,

    /// Dynamic tagging (like categories but not exclusive)
    pub tags: Vec<String>,

    /// Record creation date (fixed)
    pub date_created: DateTime<Local>,

    /// Date of last update
    pub date_updated: DateTime<Local>,
}

/// Represents a record inside lockchain
/// 
/// A record consists of a header and a body. The body has built-in
/// versioning. The different versions are then flattened to create the
/// latest stage of a record which is exposed to the outside.
#[derive(Debug, Serialize, Deserialize)]
pub struct Record {

    /// The header for this record
    pub header: Header,

    /// The versioned record body
    pub body: BTreeMap<String, Payload>,
}

impl Header {

    /// Create a new header with a name of a category
    pub fn new(name: String, category: String) -> Header {
        let me = Header {
            name: name,
            category: category,
            fields: BTreeMap::new(),
            tags: Vec::new(),
            date_created: Local::now(),
            date_updated: Local::now(),
        };

        return me;
    }
}

impl PartialEq for Record {
    fn eq(&self, other: &Record) -> bool {
        self.header == other.header
    }
}

impl Record {

    /// Create a new record
    pub fn new(name: &str, category: &str) -> Record {
        return Record {
            header: Header::new(String::from(name), String::from(category)),
            body: BTreeMap::new(),
        };
    }

    /// Set a simple key-value pair
    pub fn set_data(&mut self, key: &str, val: Payload) {
        self.body.insert(String::from(key), val);
    }

    /// Add a new tag to this record head. Checks that tags don't already exists
    pub fn add_tag(&mut self, tag: &str) {
        self.header.tags.push(String::from(tag));
    }
}