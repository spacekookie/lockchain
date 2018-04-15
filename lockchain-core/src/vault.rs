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

    /// Sync current records to disk, overwriting existing files
    pub fn sync(&mut self) {
        let mut buffer = PathBuf::new();
        buffer.push(&self.path);
        buffer.push("records");

        for (name, record) in &self.records {
            let encrypted = self.engine.encrypt(&record).unwrap();

            /* <vault>/records/<name>.data */
            {
                buffer.push(&format!("{}.data", name));
                let file = buffer.as_path();
                // println!("Saving file '{}' to '{}'", name, file.to_str().unwrap());

                let mut handle = match file.exists() {
                    true => match File::open(file.as_os_str()) {
                        Ok(k) => k,
                        Err(e) => panic!("Failed to open file: {}", e),
                    },
                    false => match File::create(file.as_os_str()) {
                        Ok(k) => k,
                        Err(e) => panic!("Failed to create file ({:?}): {}", file.as_os_str(), e),
                    },
                };

                /* Write to disk */
                match handle.write_all(encrypted.as_bytes()) {
                    Err(e) => println!("An error was encountered while writing '{}': {}", name, e),
                    _ => {}
                }
            }

            buffer.pop();
        }
    }

    /**************************/

    /// Create all relevant directories
    fn create_dirs(&mut self) -> ErrorType {
        let mut path = PathBuf::new();
        path.push(&self.path);

        /* Check if the directories already exist */
        if path.as_path().exists() {
            return ErrorType::DirectoryAlreadyExists;
        }

        /* Create the directory */
        match fs::create_dir_all(path.as_path()) {
            Err(_) => return ErrorType::FailedToInitialise,
            _ => {}
        };

        /* Create a few other directories */
        path.push("records");
        fs::create_dir_all(path.as_path()).unwrap();
        return ErrorType::Success;
    }
}
