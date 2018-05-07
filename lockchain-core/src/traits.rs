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

use record::{EncryptedBody, Header, Payload};
use serde::{de::DeserializeOwned, Serialize};
use users::User;

use base64;
use serde_json;

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

/// A common vault abstraction trait that deals with
/// data integrety and synchronisation for different
/// platform backends.
pub trait VaultLayer {
    fn fetch(&mut self);
    fn pull(&mut self, name: &str);
    fn sync(&mut self);
}

/// Auto-implement this trait to serialise types to json
pub trait AutoEncoder: Serialize + DeserializeOwned {
    fn encode(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    fn decode(s: &str) -> Self {
        serde_json::from_str(s).unwrap()
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
