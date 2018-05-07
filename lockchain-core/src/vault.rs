//! Vault data module
//!
//! A vault represents a collection of records of sensitive data. Each record
//! is encrypted before being written to disk.
//!
//! A vault can have multiple users which allows login-information to be
//! shared between multiple people. By default only one (root) user
//! is enabled though.
//!

use std::collections::{BTreeMap, HashMap};
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use record::{Payload, Record};
use security::{CryptoEngine, Key};

use serde_json;

/// This should be made pretty with actual Errors at some point
#[derive(Debug)]
pub enum ErrorType {
    VaultAlreadyExists,
    DirectoryAlreadyExists,
    FailedToInitialise,
    Success,
}

pub struct Vault {
    name: String,
    path: String,
    engine: CryptoEngine,
    pub records: HashMap<String, Record>,
}

impl Vault {
    /// Attempt to create a new vault
    pub fn new(name: &str, path: &str, password: &str) -> Result<Vault, ErrorType> {
        let mut buffer = PathBuf::new();
        buffer.push(path);
        buffer.push(format!("{}.vault", name));

        let mut me = Vault {
            name: String::from(name),
            path: buffer.to_str().unwrap().to_owned(),
            engine: CryptoEngine::generate(Key::generate()),
            records: HashMap::new(),
        };

        /* Create relevant files */
        match me.create_dirs() {
            ErrorType::Success => {
                let mut buffer = buffer.clone();
                buffer.push("primary.key");
                me.engine
                    .save(buffer.to_str().unwrap(), password, &me.name)
                    .unwrap();
            }
            val => return Err(val),
        }

        return Ok(me);
    }

    pub fn load(name: &str, path: &str, password: &str) -> Result<Vault, ErrorType> {
        /* Construct the base path */
        let mut pathbuf = PathBuf::new();
        pathbuf.push(path);
        pathbuf.push(format!("{}.vault", name));

        /* Load the primary key */
        pathbuf.push("primary.key");
        let mut engine = match CryptoEngine::load(pathbuf.to_str().unwrap(), password, name) {
            Ok(e) => e,
            Err(e) => return Err(ErrorType::FailedToInitialise),
        };
        pathbuf.pop();

        /* Load all existing records */
        pathbuf.push("records");
        let records = fs::read_dir(pathbuf.as_path()).unwrap();
        let mut record_map = HashMap::new();
        pathbuf.pop();

        /* Decrypt and map all existing records */
        for entry in records {
            let mut encrypted = String::new();
            let record = entry.unwrap();
            let mut file = File::open(record.path().as_os_str()).unwrap();
            file.read_to_string(&mut encrypted).unwrap();

            /* Decrypt and decode the data */
            let a_record: Record = engine.decrypt(encrypted).unwrap();

            let name = a_record.header.name.clone();
            record_map.insert(name, a_record);
        }

        return Ok(Vault {
            name: String::from(name),
            path: "".to_owned(),
            engine: engine,
            records: record_map,
        });
    }

    /// Adds a new (empty) record to the vault
    pub fn add_record(&mut self, name: &str, category: &str, tags: Vec<&str>) {
        let mut record = Record::new(name, category);
        for tag in tags {
            record.add_tag(&tag);
        }

        self.records.insert(String::from(name), record);
    }

    /// Fill an existing record with data
    pub fn add_data(&mut self, record: &str, key: &str, data: Payload) {
        let r: &mut Record = self.records.get_mut(record).unwrap();
        r.set_data(key, data);
    }

}
