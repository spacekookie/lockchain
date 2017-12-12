//! Record module
//!


use std::collections::BTreeMap;
use vault::{Header, Record, Payload};
use chrono::{Local};


impl Header {

    /// Create a new header with a name of a category
    pub fn new(name: String, category: String) -> Header {
        let me = Header {
            name: name,
            category: category,
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