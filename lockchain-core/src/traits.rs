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

use record::{EncryptedBody, Header, Payload, Record};
use serde::{de::DeserializeOwned, Serialize};
use users::User;

use base64;
use serde_json::{self, Error as SerdeError};

/// A Body trait that can be implemented to hook into the generic Record
/// data module.
///
/// This allows working with both encrypted and cleartext data bodies.
pub trait Body: DeserializeOwned + Serialize {
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

pub trait UserLogin {
    /// Login a user and return it with a token
    fn login(name: &str, password: &str, salt: &str) -> Option<User>;
}

/// A set of utility function that need to be implemented in order
/// for a type to be encryptable or decryptable.
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

/// A trait that abstracts file or record loading for
/// any backend which wants to implement storage functions
pub trait Loading {
    fn load(_path: &str) -> Box<Self> {
        unimplemented!()
    }

    fn save(&mut self, _path: &str) {
        unimplemented!()
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
pub trait Vault<T>
where
    T: Body,
{
    /// A shared constructor for all vault implementations
    fn new(name: &str, location: &str) -> Self;
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
