//! A module that enables file management for vaults
//!
//!
#![feature(non_modrs_mods)]

extern crate lockchain_core as lcc;

use lcc::traits::{Body, Vault};
use lcc::{MetaDomain, Payload, Record, VaultMetadata};
use std::collections::HashMap;

mod fs;
use fs::{FileType, Filesystem};

/// Represents a vault on disk
#[derive(Debug)]
pub struct DataVault<T: Body> {
    meta_info: (String, String),
    records: HashMap<String, Record<T>>,
    metadata: HashMap<String, MetaDomain>,
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
    fn new(name: &str, location: &str) -> DataVault<T> {
        Self {
            meta_info: (name.into(), location.into()),
            records: HashMap::new(),
            metadata: HashMap::new(),
            fs: Filesystem::create(location, name),
        }.initialize()
    }

    fn metadata(&self) -> VaultMetadata {
        VaultMetadata {
            name: self.meta_info.0.clone(),
            location: self.meta_info.1.clone(),
            size: self.records.len(),
        }
    }

    /// Caches all files from disk to memory
    fn fetch(&mut self) {
        self.records.clear();
        self.metadata.clear();

        self.fs
            .fetch::<Record<T>>(FileType::Record)
            .unwrap()
            .into_iter()
            .map(|rec| (rec.header.name.clone(), rec))
            .for_each(|x| {
                self.records.insert(x.0, x.1);
            });

        self.fs
            .fetch::<MetaDomain>(FileType::Metadata)
            .unwrap()
            .into_iter()
            .map(|rec| (rec.name().into(), rec))
            .for_each(|x| {
                self.metadata.insert(x.0, x.1);
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
        self.fs
            .sync::<Record<T>>(&self.records, FileType::Record)
            .unwrap();
        self.fs
            .sync::<MetaDomain>(&self.metadata, FileType::Metadata)
            .unwrap();
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

    fn meta_add_domain(&mut self, domain: &str) -> Option<()> {
        if self.metadata.contains_key(domain) {
            None
        } else {
            self.metadata.insert(domain.into(), MetaDomain::new(domain));
            Some(())
        }
    }

    fn meta_pull_domain(&mut self, domain: &str) -> Option<&MetaDomain> {
        self.metadata.get(domain)
    }

    fn meta_push_domain(&mut self, domain: MetaDomain) -> Option<()> {
        self.metadata
            .insert(domain.name().into(), domain)
            .map_or((), |_| ()) // We don't care about `None`
            .into()
    }

    fn meta_set(&mut self, domain: &str, name: &str, data: Payload) -> Option<()> {
        self.metadata.get_mut(domain)?.set_field(name, data)
    }

    fn meta_get(&mut self, domain: &str, name: &str) -> Option<Payload> {
        Some(self.metadata.get(domain)?.get_field(name)?.clone())
    }
}
