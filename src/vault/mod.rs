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