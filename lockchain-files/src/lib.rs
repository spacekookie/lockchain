//! A persistence layer for lockchain vaults based on files
//!
//! This crate provides a filesystem backend
//! which relies on keeping records in discrete files
//! and folder structures.
//!
//! All further documentation can be found in `FileVault`

extern crate lockchain_core as lcc;
extern crate semver;

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate serde;

use crate::lcc::traits::{Body, LoadRecord, Vault};
use crate::lcc::{
    errors::VaultError,
    users::{Access, Token, UserStore},
    Generator, Header, MetaDomain, Payload, Record, VaultMetadata,
};
use std::collections::HashMap;

mod config;
mod create;
pub mod fs;
mod load;
mod userstore;
mod utils;

pub use crate::config::{ConfigError, VaultConfig};
use crate::fs::Filesystem;

/// Persistence mapper to a folder and file structure
///
/// This implementation tries  to be as efficient
/// as possible, however please note that it is
/// dependant on filesystem operations and is
/// not suited for high-performance applications!
///
/// ---
///
/// Implements the `Vault` API in full,
/// replicating all functionality in memory
/// while providing async operations on-disk.
///
/// Requests on files are debounced!
///
/// The internal layout should not be assumed
/// and isn't stabilised with the crate version
/// (i.e. minor crate bumps can break vault compatibility
/// as long as they remain API compatible).
///
/// The version of a vault is written in it's coniguration
/// which can be read via `json-compat` shims,
/// in case the layout and scheme ever changes.
///
/// The vault folder is safe to copy around –
/// all vault metadata is kept inside it.
pub struct FileVault<T: Body> {
    /// A representation of the cached vault config
    config: VaultConfig,
    /// Filesystem wrapper utility
    fs: Filesystem,
    /// A userstore utility derived from Metadata
    users: UserStore,
    /// A mapping of loaded records
    records: HashMap<String, Record<T>>,
    /// An index of all existing headers
    headers: HashMap<String, Header>,
    /// A map of all metadata files
    metadata: HashMap<String, MetaDomain>,
}

impl<T: Body> LoadRecord<T> for FileVault<T> {}

impl<T: Body> Vault<T> for FileVault<T> {
    fn new(gen: Generator) -> Result<Box<FileVault<T>>, VaultError> {
        Self::create(gen).map(|s| Box::new(s))
    }

    fn load(name: &str, location: &str) -> Result<Box<Self>, VaultError> {
        Self::load(name, location).map(|s| Box::new(s))
    }

    fn create_user(
        &mut self,
        _token: Token,
        _username: &str,
        _secret: Vec<u8>,
        _access: Vec<Access>,
    ) -> Result<(), ()> {
        unimplemented!()
    }

    fn delete_user(&mut self, _token: Token, _username: &str) {
        unimplemented!()
    }

    fn authenticate(&mut self, _username: &str, _secret: &str) -> Token {
        unimplemented!()
    }

    fn deauthenticate(&mut self, _username: &str, _: Token) {
        unimplemented!()
    }

    fn metadata(&self) -> VaultMetadata {
        unimplemented!()
    }

    /// Caches all files from disk to memory
    fn fetch(&mut self) {
        // self.records.clear();
        // self.metadata.clear();

        // self.fs
        //     .fetch::<Record<T>>(FileType::Record)
        //     .unwrap()
        //     .into_iter()
        //     .map(|rec| (rec.header.name.clone(), rec))
        //     .for_each(|x| {
        //         self.records.insert(x.0, x.1);
        //     });

        // self.fs
        //     .fetch::<MetaDomain>(FileType::Metadata)
        //     .unwrap()
        //     .into_iter()
        //     .map(|rec| (rec.name().into(), rec))
        //     .for_each(|x| {
        //         self.metadata.insert(x.0, x.1);
        //     });
        unimplemented!()
    }

    /// Make sure a single record is loaded
    fn pull(&mut self, _name: &str) {
        // self.records.remove(name);
        // self.records.insert(
        //     name.to_owned(),
        //     self.fs.pull::<Record<T>>(FileType::Record, name).unwrap(),
        // );
        unimplemented!()
    }

    fn sync(&mut self) {
        self.fs.sync_vault(&self).unwrap();

        // self.fs
        //     .sync::<Record<T>>(&self.records, FileType::Record)
        //     .unwrap();
        // self.fs
        //     .sync::<MetaDomain>(&self.metadata, FileType::Metadata)
        //     .unwrap();
        // unimplemented!()
    }

    fn get_record(&self, _name: &str) -> Option<&Record<T>> {
        // self.records.get(name)
        unimplemented!()
    }

    fn contains(&self, _name: &str) -> bool {
        // self.records.contains_key(name)
        unimplemented!()
    }

    fn add_record(&mut self, _key: &str, _category: &str, _tags: Vec<&str>) {
        // self.records
        //     .insert(key.to_owned(), Record::new(key, category, tags));
        unimplemented!()
    }

    fn delete_record(&mut self, _record: &str) -> Option<Record<T>> {
        // self.records.remove(record)
        unimplemented!()
    }

    fn add_data(&mut self, _record: &str, _key: &str, _data: Payload) -> Option<()> {
        // self.records.get_mut(record)?.add_data(key, data)
        unimplemented!()
    }

    fn get_data(&self, _record: &str, _key: &str) -> Option<&Payload> {
        // self.records.get(record)?.get_data(key)
        unimplemented!()
    }

    fn meta_add_domain(&mut self, _domain: &str) -> Option<()> {
        // if self.metadata.contains_key(domain) {
        //     None
        // } else {
        //     self.metadata.insert(domain.into(), MetaDomain::new(domain));
        //     Some(())
        // }
        unimplemented!()
    }

    fn meta_pull_domain(&self, _domain: &str) -> Option<&MetaDomain> {
        // self.metadata.get(domain)
        unimplemented!()
    }

    fn meta_push_domain(&mut self, _domain: MetaDomain) -> Option<()> {
        // self.metadata
        //     .insert(domain.name().into(), domain)
        //     .map_or((), |_| ()) // We don't care about `None`
        //     .into()
        unimplemented!()
    }

    fn meta_set(&mut self, _domain: &str, _name: &str, _data: Payload) -> Option<()> {
        // self.metadata.get_mut(domain)?.set_field(name, data)
        unimplemented!()
    }

    fn meta_get(&mut self, _domain: &str, _name: &str) -> Option<Payload> {
        // Some(self.metadata.get(domain)?.get_field(name)?.clone())
        unimplemented!()
    }

    fn meta_exists(&self, _domain: &str) -> bool {
        // self.metadata.contains_key(domain)
        unimplemented!()
    }
}
