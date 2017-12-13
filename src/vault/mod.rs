//! Vault data module
//!
//! A vault represents a collection of records of sensitive data. Each record
//! is encrypted before being written to disk.
//!
//! A vault can have multiple users which allows login-information to be
//! shared between multiple people. By default only one (root) user
//! is enabled though.
//!

mod management;
mod version;
mod record;
mod user;

use std::collections::{HashMap, BTreeMap};
use std::io::prelude::*;
use std::path::PathBuf;
use std::fs::File;
use std::fs;

use crypto::engine::CryptoEngine;

use chrono::{DateTime, Local};
use serde_json;


/// This should be made pretty with actual Errors at some point
#[derive(Debug)]
#[allow(unused)]
pub enum ErrorType {
    VaultAlreadyExists,
    DirectoryAlreadyExists,
    FailedToInitialise,
    Success,
}

/// A generic payload for a record
#[derive(Debug, Serialize, Deserialize)]
pub enum Payload {
    Text(String),
    Boolean(bool),
    Number(i64),
    BTreeMap(BTreeMap<String, Payload>),
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Header {
    pub name: String,
    pub category: String,
    pub tags: Vec<String>,
    pub date_created: DateTime<Local>,
    pub date_updated: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub header: Header,
    pub body: BTreeMap<String, Payload>,
}

/// A vault that represents a collection of records of sensitive data.
/// Each record is encrypted before being written to disk.
///
/// A vault can have multiple users which allows login-information to be
/// shared between multiple people. By default only one (root) user
/// is enabled though.
#[allow(unused)]
pub struct Vault {
    name: String,
    path: PathBuf,
    crypto: CryptoEngine,
    pub records: HashMap<String, Record>,
}

impl Vault {

    /// Attempt to create a new vault
    pub fn new(name: &str, path: &str, password: &str) -> Result<Vault, ErrorType> {
        let mut me = Vault {
            name: String::from(name),
            path: PathBuf::new(),
            crypto: CryptoEngine::new(password, ""),
            records: HashMap::new(),
        };

        me.path.push(path);
        me.path.push(format!("{}.vault", name));

        /* Create relevant files */
        match me.create_dirs() {
            ErrorType::Success => {}
            val => return Err(val),
        }

        return Ok(me);
    }

    pub fn load(name: &str, path: &str, password: &str) -> Vault {

        /* Construct the base path */
        let mut pathbuf = PathBuf::new();
        pathbuf.push(path);
        pathbuf.push(format!("{}.vault", name));

        /* Load the secret key */
        let mut key = String::new();
        {
            pathbuf.push("primary.key");
            let key_path = pathbuf.as_os_str();
            let mut key_file = File::open(key_path).unwrap();
            key_file.read_to_string(&mut key).expect(
                "Failed to load primary key file!",
            );
        };

        let crypto = CryptoEngine::load_existing(&key, password);

        /* Load all existing records */
        pathbuf.pop();
        pathbuf.push("records");
        let records = fs::read_dir(pathbuf.as_path()).unwrap();
        let mut record_map: HashMap<String, Record> = HashMap::new();
        pathbuf.pop();

        /* Decrypt and map all existing records */
        for entry in records {
            let mut encrypted = String::new();
            let record = entry.unwrap();
            let mut file = File::open(record.path().as_os_str()).unwrap();
            file.read_to_string(&mut encrypted).unwrap();

            /* Decrypt and decode the data */
            let decrypted = crypto.decrypt(&encrypted);
            let a_record: Record = serde_json::from_str(&decrypted).unwrap();

            let name = a_record.header.name.clone();
            record_map.insert(name, a_record);
        }

        return Vault {
            name: String::from(name),
            path: PathBuf::new(),
            crypto: crypto,
            records: record_map,
        };
    }

    /// Adds a new record to the vault
    pub fn add_record(&mut self, name: &str, category: &str, tags: Vec<&str>) {
        let mut record = Record::new(name, category);
        for tag in tags {
            record.add_tag(&tag);
        }

        self.records.insert(String::from(name), record);
    }

    pub fn add_data(&mut self, record: &str, key: &str, data: Payload) {
        let r: &mut Record = self.records.get_mut(record).unwrap();
        r.set_data(key, data);
    }

    /// Sync current records to disk, overwriting existing files
    pub fn sync(&self) {

        let mut path = self.path.clone();
        path.push("records");
        println!("Syncing records in: {:?}", path.as_os_str());

        for (name, record) in &self.records {
            let serialised = serde_json::to_string(&record).unwrap();
            let encrypted = self.crypto.encrypt(&serialised);

            /* <vault>/records/<name>.data */
            {
                path.push(format!("{}.data", name));
                let file = path.as_path();
                println!("File exists: {}", file.exists());

                let mut handle = match file.exists() {
                    true => {
                        match File::open(file.as_os_str()) {
                            Ok(k) => k,
                            Err(e) => panic!("Failed to open file: {}", e),
                        }
                    }
                    false => {
                        match File::create(file.as_os_str()) {
                            Ok(k) => k,
                            Err(e) => panic!("Failed to create file ({:?}): {}", file.as_os_str(), e),
                        }
                    }
                };

                /* Write to disk */
                match handle.write_all(encrypted.as_bytes()) {
                    Err(e) => println!("An error was encountered while writing '{}': {}", name, e),
                    _ => {}
                }
            }

            path.pop();
        }
    }

    /**************************/

    /// Create all relevant directories
    fn create_dirs(&mut self) -> ErrorType {

        /* Check if the directories already exist */
        if self.path.as_path().exists() {
            return ErrorType::DirectoryAlreadyExists;
        }

        /* Create the directory */
        match fs::create_dir_all(self.path.as_path()) {
            Err(_) => return ErrorType::FailedToInitialise,
            _ => {}
        };

        /* Create configs */
        let key = match self.crypto.dump_encrypted_key() {
            Some(k) => k,
            None => return ErrorType::FailedToInitialise,
        };

        /* Write encrypted key to disk */
        {
            self.path.push("primary.key");
            let key_path = self.path.as_os_str();
            let mut key_file = File::create(key_path).unwrap();
            println!("Creating key file at {:?}", key_file);
            key_file.write_all(key.as_bytes()).unwrap();
        }

        /* Create a few other directories */
        {
            self.path.pop();
            self.path.push("records");
            fs::create_dir_all(self.path.as_path()).unwrap();
            self.path.pop();
        }

        return ErrorType::Success;
    }
}