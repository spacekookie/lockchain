//! Common vault data module
//!
//! A vault represents a collection of records of sensitive data. Each record
//! is a versioned history of key-value store entries inside a body. By default
//! only the headers of records are available as they don't contain secret
//! information.
//!
//! In the future, Vaults will have user and rights management that determines
//! how data can be written and read from records.

use record::{Payload, Record};
use std::collections::HashMap;
use traits::{Body, VaultLayer};

/// In-memory representation of a lockchain vault.
///
/// By itself it represents vault metadata (name, users, location)
/// as well as a list of record headers.
///
/// To provide on-disk functionality it requires the `-storage`
/// trait library and for encrypted file access the `-crypto`
/// crate.
///
/// The body backend is being being generic with the `Body` trait.
#[allow(dead_code)]
pub struct Vault<T: Body> {
    /// The name of this vault
    name: String,
    /// The location of this vault (either local or remote)
    location: String,
    ///A list of known records in this vault
    records: HashMap<String, Record<T>>,
}

impl<T: Body> VaultLayer for Vault<T> {
    fn fetch(&mut self) {
        unimplemented!()
    }
    fn pull(&mut self, _name: &str) {
        unimplemented!()
    }
    fn sync(&mut self) {
        unimplemented!()
    }
}

impl<T: Body> Vault<T> {
    pub fn new(name: &str, location: &str) -> Vault<T> {
        Vault {
            name: name.to_owned(),
            location: location.to_owned(),
            records: HashMap::<String, Record<T>>::new(),
        }
    }

    /// Try to retrieve a record from this vault.
    ///
    /// Returns `None` if it doesn't exist or can't be retrieved
    /// from the selected storage backend for this vault.
    pub fn get_record(&self, name: &str) -> Option<&Record<T>> {
        self.records.get(name)
    }

    /// Checks if a record is present in this vault without
    /// ever trying to load it.
    ///
    /// This is good to check the in-memory metadata without causing
    /// load on a central backend store that might not be located
    /// on the same machine as the front-end client.
    pub fn contains(&self, name: &str) -> bool {
        self.records.contains_key(name)
    }

    /// Adds a new (empty) record to the vault
    pub fn add_record(&mut self, key: &str, category: &str, tags: Vec<&str>) {
        self.records
            .insert(String::from(key), Record::new(key, category, tags));
    }

    /// Fill an existing record with data
    pub fn add_data(&mut self, record: &str, key: &str, data: Payload) -> Option<()> {
        (self.records.get_mut(record)?).add_data(key, data)
    }
}
