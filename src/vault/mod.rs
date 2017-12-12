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

pub mod crypto;
use crypto::CryptoEngine;

use std::collections::{HashMap, BTreeMap};
use std::path::PathBuf;
use std::error::Error;
use chrono::{DateTime, Local};

use std::fs;
use std::fs::File;
use std::io::prelude::*;

use serde_json;

/// This should be made pretty with actual Errors at some point
pub enum ErrorType {
    VAULT_ALREADY_EXISTS,
    DIRECTORY_ALREADY_EXISTS,
    FAILED_TO_INITIALISE,
    SUCCESS,
}

/// A generic payload for a record
#[derive(Debug, Serialize, Deserialize)]
pub enum Payload {
    String,
    bool,
    i64,
    BTreeMap,
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
    body: BTreeMap<String, Payload>,
}

/// A vault that represents a collection of records of sensitive data.
/// Each record is encrypted before being written to disk.
///
/// A vault can have multiple users which allows login-information to be
/// shared between multiple people. By default only one (root) user
/// is enabled though.
pub struct Vault {
    name: String,
    path: PathBuf,
    crypto: CryptoEngine,
    records: HashMap<String, Record>,
}

impl Vault {
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
            ErrorType::SUCCESS => {}
            val => return Err(val),
        }

        println!("{:?}", me.path);
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

        /* Decrypt and map all existing records */
        for entry in records {
            let mut encrypted = String::new();
            let record = entry.unwrap();
            let mut file = File::open(record.path().as_os_str()).unwrap();
            file.read_to_string(&mut encrypted).unwrap();

            /* Make the encrypted data a vector */
            let record_bytes = encrypted.as_bytes();
            let mut record_vector: Vec<u8> = Vec::new();
            for byte in record_bytes {
                record_vector.push(*byte);
            }
            let decrypted = crypto.decrypt(&record_vector);
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

    /**************************/

    /// Create all relevant directories
    fn create_dirs(&mut self) -> ErrorType {

        /* Check if the directories already exist */
        if self.path.as_path().exists() {
            return ErrorType::DIRECTORY_ALREADY_EXISTS;
        }

        /* Create the directory */
        match fs::create_dir_all(self.path.as_path()) {
            Err(_) => return ErrorType::FAILED_TO_INITIALISE,
            _ => {}
        };

        /* Create configs */
        let key = match self.crypto.dump_encrypted_key() {
            Some(k) => k,
            None => return ErrorType::FAILED_TO_INITIALISE,
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
        }

        return ErrorType::SUCCESS;
    }
}