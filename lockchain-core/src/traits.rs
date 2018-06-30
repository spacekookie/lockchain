//! Common vault traits for plugin-crates
//!
//! The core of this crate has no functionality and is dependant
//! on other libraries to fill those holes. To make this easer
//! (and sometimes possible), we defined a few common behaviours
//! in traits to expand on in implementations specific to the
//! library.
//!
//! Each trait is documented in more detail and provides default
//! implementations with `unimplemented!` macros to make
//! compilation work without external crates but not calling
//! functions at runtime.

use meta::{MetaDomain, VaultMetadata};
use record::{EncryptedBody, Header, Payload, Record};
use serde::{de::DeserializeOwned, Serialize};

use base64;
use serde_json::{self, Error as SerdeError};
use std::error::Error;

/// A Body trait that can be implemented to hook into the generic Record
/// data module.
///
/// This allows working with both encrypted and cleartext data bodies.
pub trait Body: DeserializeOwned + Serialize + Send {
    ///Get the value of a field from this body
    fn get_field(&self, key: &str) -> Option<&Payload>;
    /// Set the value of a field
    fn set_field(&mut self, key: &str, value: Payload) -> Option<()>;
    /// Remove versioning and flatten the data tree to a single level.
    fn flatten(&mut self) -> Option<()>;
}

/// A simple trait that allows libraries to hook into the
/// `body()` and `record()` hooks for vault records.
pub trait LoadRecord<T: Body> {
    fn header() -> Header {
        unimplemented!()
    }

    fn body() -> T {
        unimplemented!()
    }
}

/// This is purely a marker trait for encryptable types
///
/// Indicates that a type should be handlable by an encryption
/// engine, also relying on the auto encoder functionality.
///
/// Additional functions might be added to this trait further down
/// the road but for now, it's really just a marker that you can easily
/// implement for any type that's also `AutoEncoder`
///
/// ```rust, norun
/// impl Encryptable for YourSpecialType {}
/// ```
pub trait Encryptable: AutoEncoder {}

/// A base trait that describes the basic functionality of
/// an encryption backend which handles encrypted files.
///
/// Encryption is never done directly on the bodies, only via
/// this scheduler type with the help of the [[Encryptable]] trait.
pub trait EncryptionHandler<T>
where
    T: Encryptable + AutoEncoder + Body,
{
    fn encrypt(&mut self, item: T) -> EncryptedBody;
    fn decrypt(&mut self, item: EncryptedBody) -> Option<T>;
}

/// An abstract file loading utility trait
///
/// Any type that implements `FileIO` also has to be
/// `AutoEncoder` in order to be storable. This trait implements
/// common file I/O operations, assuming that any type using it
/// will then provide the required utility functions.
pub trait FileIO: AutoEncoder {
    /// Load a type from a file path
    fn load(path: &str) -> Result<Self, Box<Error>> {
        use std::fs;
        fs::read_to_string(path)
            .and_then(|s| Self::decode(&s).map_err(|e| e.into()))
            .map_err(|e| e.into())
    }

    /// Store a type to a file path
    fn save(&self, path: &str) -> Result<(), Box<Error>> {
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut file = OpenOptions::new().write(true).create(true).open(path)?;
        let content = self.encode()?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }
}

/// Trait for an in-memory representation of a lockchain vault.
///
/// By itself it represents vault metadata (name, users, location)
/// as well as a list of record headers.
///
/// To provide on-disk functionality it requires the `-storage`
/// trait library and for encrypted file access the `-crypto`
/// crate.
///
/// The body backend is being being generic with the `Body` trait.
pub trait Vault<T>: Send
where
    T: Body,
{
    /// A shared constructor for all vault implementations
    fn new(name: &str, location: &str) -> Self;
    /// Get basic vault metadata
    fn metadata(&self) -> VaultMetadata;
    /// Fetch metadata headers for all records
    fn fetch(&mut self);
    /// Pull a specific record from the backend
    fn pull(&mut self, name: &str);
    /// Sync all changes back to the backend
    fn sync(&mut self);
    /// Get a complete record from this vault
    fn get_record(&self, name: &str) -> Option<&Record<T>>;
    /// Probe if a record is contained
    fn contains(&self, name: &str) -> bool;
    /// Add a new record to this vault
    fn add_record(&mut self, key: &str, category: &str, tags: Vec<&str>);
    /// Delete a record from this vault
    fn delete_record(&mut self, record: &str) -> Option<Record<T>>;
    /// Add data to an existing record, overwriting existing fields
    fn add_data(&mut self, record: &str, key: &str, data: Payload) -> Option<()>;
    /// Get the (latest) value of a specific record data field
    fn get_data(&self, record: &str, key: &str) -> Option<&Payload>;
    /// Adds a domain space to the metadata store inside the vault
    ///
    /// A domain is a collection metadata files that can be
    /// returned with a single pull request
    fn meta_add_domain(&mut self, domain: &str) -> Option<()>;
    /// Returns all records from a meta domain
    fn meta_pull_domain(&mut self, domain: &str) -> Option<&MetaDomain>;
    /// Entirely replace a meta domain in the store
    fn meta_push_domain(&mut self, domain: MetaDomain) -> Option<()>;
    /// Set the value of a field inside a domain. Field names **must not** collide
    fn meta_set(&mut self, domain: &str, name: &str, data: Payload) -> Option<()>;
    /// Get the value of a (unique) field inside a domain
    fn meta_get(&mut self, domain: &str, name: &str) -> Option<Payload>;
}

/// Auto-implement this trait to serialise types to json
pub trait AutoEncoder: Serialize + DeserializeOwned {
    fn encode(&self) -> Result<String, SerdeError> {
        serde_json::to_string_pretty(self)
    }

    fn decode(s: &str) -> Result<Self, SerdeError> {
        serde_json::from_str(s)
    }
}

/// Include this trait to monkey-patch base64 functions onto String types
pub trait Base64AutoEncoder {
    fn to_base64(&self) -> String;
    fn from_base64(base64: &str) -> String;
}

impl Base64AutoEncoder for String {
    /// Automatically encode this string to base64
    fn to_base64(&self) -> String {
        base64::encode(self.as_bytes())
    }

    /// Craft a string from an existing base64 string slice
    fn from_base64(base64: &str) -> String {
        String::from_utf8(base64::decode(base64).unwrap()).unwrap()
    }
}
