//! A persistence layer for lockchain vaults based on files
//!
//! This crate provides a filesystem backend
//! which relies on keeping records in discrete files
//! and folder structures.
//!
//! This is great if a vault needs to be easily syncable
//! or indexable by another tool.
//! No clear-text secrets are ever written to disk.
//! But might be held in memory cache
//! for a period of time.
//!
//! This backend is comparibly slow
//! and should be avoided
//! for performance critical applications.
//! For such applications
//! the blockstore is much more suited
//! which represents a vault
//! in a binary blob
//! independant of the used filesystem.
//!
//! Part of the performance problems
//! comes from locking the entire vault
//! when doing operations,
//! meaning that only
//! one instance
//! of a lockchain library
//! can operate on it
//! at the time
//!
//! ```
//! my_vault/
//!   config.toml
//!   Lockfile
//!   metadata/
//!     userstore.meta
//!     registry.meta
//!   records/
//!     <base64 hash 1>.rec
//!     <base64 hash 2>.rec
//!     <base64 hash 3>.rec
//!   hashsums/
//!     <base64 hash 1>.sum
//!     <base64 hash 2>.sum
//!     <base64 hash 3>.sum
//! ```
#![feature(non_modrs_mods)]

extern crate lockchain_core as lcc;
extern crate semver;
extern crate toml;

#[macro_use]
extern crate serde_derive;
extern crate serde;

use lcc::traits::{Body, LoadRecord, Vault};
use lcc::{
    errors::VaultError,
    users::{Access, Token, UserStore},
    Generator, MetaDomain, Payload, Record, VaultMetadata,
};
use std::collections::HashMap;

mod config;
mod fs;
mod userstore;
mod utils;

pub use config::{ConfigError, VaultConfig};
use fs::{FileType, Filesystem};
use userstore::UserStoreMapper;

/// Persistence mapper to a folder and file structure
///
/// Implements the `Vault` API in full,
/// replicating all functionality in memory
/// and never writing clear text data to disk.
///
/// The internal layout should not be assumed
/// and isn't stabilised with the crate version
/// (i.e. minor crate bumps can break vault compatibility
/// as long as they remain API compatible).
///
/// The version of a vault is written in it's coniguration
/// (which won't change – ever).
///
/// The vault folder is safe to copy around –
/// all vault metadata is kept inside it.
pub struct DataVault<T: Body> {
    meta_info: (String, String),
    config: VaultConfig,
    records: HashMap<String, Record<T>>,
    metadata: HashMap<String, MetaDomain>,
    fs: Filesystem,
    users: UserStoreMapper,
}

impl<T: Body> DataVault<T> {
    /// Small utility function to setup file structure
    fn initialize(self) -> Self {
        self.fs.scaffold();
        self.config.save(&self.fs.root).unwrap();
        self
    }

    fn load(mut self) -> Result<Box<Self>, VaultError> {
        self.config = match VaultConfig::load(&self.fs.root) {
            Ok(cfg) => cfg,
            _ => return Err(VaultError::FailedLoading),
        };

        Ok(Box::new(self))
    }
}

impl<T: Body> LoadRecord<T> for DataVault<T> {}

impl<T: Body> Vault<T> for DataVault<T> {
    fn new(gen: Generator) -> Result<Box<DataVault<T>>, VaultError> {
        Ok(Box::new(
            Self {
                meta_info: (
                    gen.name.clone().unwrap().into(),
                    gen.location.clone().unwrap().into(),
                ),
                records: HashMap::new(),
                config: VaultConfig::new(),
                metadata: HashMap::new(),
                fs: Filesystem::new(&gen.location.unwrap(), &gen.name.unwrap()),
                users: UserStoreMapper::new(UserStore::new()),
            }.initialize(),
        ))
    }

    fn create_user(
        &mut self,
        token: Token,
        username: &str,
        secret: &str,
        access: Vec<Access>,
    ) -> Result<(), ()> {
        unimplemented!()
    }

    fn delete_user(&mut self, token: Token, username: &str) {}

    // Checking if a vault exists is basically checking it's config
    // against the compatible version of this library.
    //
    // If it's compatible we can open the vault into memory
    // (loading all required paths into the struct), then return it
    fn load(name: &str, location: &str) -> Result<Box<Self>, VaultError> {
        Self {
            meta_info: (name.into(), location.into()),
            records: HashMap::new(),
            config: VaultConfig::new(),
            metadata: HashMap::new(),
            fs: Filesystem::new(location, name),
            users: UserStoreMapper::new(UserStore::new()),
        }.load()
    }

    fn authenticate(&mut self, username: &str, secret: &str) -> Token {
        unimplemented!()
    }

    fn deauthenticate(&mut self, username: &str, _: Token) {
        unimplemented!()
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

    fn meta_pull_domain(&self, domain: &str) -> Option<&MetaDomain> {
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

    fn meta_exists(&self, domain: &str) -> bool {
        self.metadata.contains_key(domain)
    }
}
