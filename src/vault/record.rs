//! Record module
//!


use std::collections::BTreeMap;
use vault::{Header, Record};
use chrono::{DateTime, Local};


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

impl Record {

    /// Create a new record
    pub fn new(name: &str, category: &str) -> Record {
        return Record {
            header: Header::new(String::from(name), String::from(category)),
            body: BTreeMap::new(),
        };
    }
}