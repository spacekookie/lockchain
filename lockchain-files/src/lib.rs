//! A module that enables file management for vaults
//!
//!
#![feature(non_modrs_mods)]

extern crate lockchain_core as lcc;

use lcc::traits::{Body, Vault};
use lcc::{Payload, Record};
use std::collections::HashMap;

mod fs;
use fs::{FileType, Filesystem};

/// Represents a vault on disk
pub struct DataVault<T: Body> {
    records: HashMap<String, Record<T>>,
    fs: Filesystem,
}

impl<T: Body> DataVault<T> {
    /// Small utility function to setup file structure
    fn initialize(self) -> Self {
        self.fs.scaffold();
        self
    }
}

impl<T: Body> Vault<T> for DataVault<T> {
    fn new(name: &str, location: &str) -> Self {
        Self {
            records: HashMap::new(),
            fs: Filesystem::create(location, name),
        }.initialize()
    }

    /// Caches all files from disk to memory
    fn fetch(&mut self) {
        self.records.clear();
        self.fs
            .fetch::<Record<T>>(FileType::Record)
            .unwrap()
            .into_iter()
            .map(|rec| (rec.header.name.clone(), rec))
            .for_each(|x| {
                self.records.insert(x.0, x.1);
            });
    }

    /// Make sure a single record is loaded
    fn pull(&mut self, name: &str) {
        self.records.remove(name);
        self.records.insert(
            name.to_owned(),
            self.fs.pull::<Record<T>>(FileType::Record, name).unwrap(),
        );
    }

    fn sync(&mut self) {
        self.fs.sync::<Record<T>>(FileType::Record).unwrap();
    }

    fn get_record(&self, name: &str) -> Option<&Record<T>> {
        self.records.get(name)
    }

    fn contains(&self, name: &str) -> bool {
        self.records.contains_key(name)
    }

    fn add_record(&mut self, key: &str, category: &str, tags: Vec<&str>) {
        self.records
            .insert(key.to_owned(), Record::new(key, category, tags));
    }

    fn delete_record(&mut self, record: &str) -> Option<Record<T>> {
        self.records.remove(record)
    }

    fn add_data(&mut self, record: &str, key: &str, data: Payload) -> Option<()> {
        self.records.get_mut(record)?.add_data(key, data)
    }

    fn get_data(&self, record: &str, key: &str) -> Option<&Payload> {
        self.records.get(record)?.get_data(key)
    }
}
