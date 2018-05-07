//! Lockchain record handling module
//! 
//! A record is a set of key-value store values with a header


use std::collections::BTreeMap;
use chrono::{Local, DateTime};
use serde_json;


/// A generic payload for a record
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum Payload {
    Text(String),
    Boolean(bool),
    Number(i64),
    BTreeMap(BTreeMap<String, Payload>),
}

/// The header of a record
/// 
/// Contains easily searchable fields of metadata. Nothing
/// in the Header should ever be considered secure as the
/// headers are kept cached for much longer than the rest
/// of the data.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Header {
    pub name: String,
    pub category: String,
    pub tags: Vec<String>,
    pub fields: BTreeMap<String, Payload>,
    pub date_created: DateTime<Local>,
    pub date_updated: DateTime<Local>,
}

/// 
#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub header: Header,
    pub body: BTreeMap<String, Payload>,
}


impl Header {

    /// Create a new header with a name of a category
    pub fn new(name: String, category: String) -> Header {
        let me = Header {
            name: name,
            category: category,
            tags: Vec::new(),
            fields: BTreeMap::new(),
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
            // fields: BTreeMap::new(),
            body: BTreeMap::new(),
        };
    }

    /// Set a simple key-value pair
    pub fn set_data(&mut self, key: &str, val: Payload) {
        self.body.insert(String::from(key), val);
    }

    /// Contains a cloned value of single data field
    pub fn get_data(&self, key: &str) -> Payload {
        return self.body.get(key).unwrap().clone();
    }

    /// Serialise the entire body into a json tree
    /// 
    /// Contains all secret values in a json tree that you can work with manually.
    pub fn get_json(&self) -> String {
        return serde_json::to_string(&self.body).unwrap();
    }

    /// Add a new tag to this record head. Checks that tags don't already exists
    pub fn add_tag(&mut self, tag: &str) {
        self.header.tags.push(String::from(tag));
    }
}