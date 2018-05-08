//! A module that enables file management for vaults
//!
//!
#![feature(non_modrs_mods)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate lockchain_core as lcc;

use lcc::traits::{Body, Vault};
use lcc::{Payload, Record};
use std::collections::HashMap;

mod fs;
use fs::Filesystem;

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

    fn fetch(&mut self) {
        unimplemented!()
    }

    fn pull(&mut self, name: &str) {
        unimplemented!()
    }

    fn sync(&mut self) {
        unimplemented!()
    }

    fn get_record(&self, name: &str) -> Option<&Record<T>> {
        unimplemented!()
    }

    fn contains(&self, name: &str) -> bool {
        unimplemented!()
    }

    fn add_record(&mut self, key: &str, category: &str, tags: Vec<&str>) {
        unimplemented!()
    }

    fn delete_record(&mut self, record: &str) -> Option<Record<T>> {
        unimplemented!()
    }

    fn add_data(&mut self, record: &str, key: &str, data: Payload) -> Option<()> {
        unimplemented!()
    }

    fn get_data(&self, record: &str, key: &str) -> Option<&Payload> {
        unimplemented!()
    }
}
