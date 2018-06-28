//! User and access regulation module
//!
//! Access can be given for a vault or file (based on id)
//! as well as an entire Api endpoint. By default all
//! Rust APIs assume `{ Api, Admin }` access, for other
//! APIs crates (http, ...), a user with access rights
//! has to be specified.
//!
//! A user for an API endpoint is not the same as a user for
//! a vault. An API admin could have access to a vault where
//! they can only read a single file!
//!
//! `User` is also a serialisable struct which contains important
//! data to load and store them into a metadata store.

mod auth;
pub use self::auth::Token;

use crypto::{encoding, hashing};
use traits::AutoEncoder;

/// Specifies access to a resource
#[derive(Serialize, Deserialize)]
pub enum Access {
    /// Allows specific access to an entire API
    Api,
    /// Allows access to vault metadata & index files
    Vault(String),
    /// Allows access to a record resource inside a vault
    Record(String, String),
}

/// Specifies the capabilities of a user
#[derive(Serialize, Deserialize)]
pub enum Role {
    Reader,
    Editor,
    Admin,
}

/// A generic user representation
#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    pw_hash: String,
    role: Role,
    access: Vec<Access>,
}

impl User {
    /// Register a new user with a name and password
    pub fn register(name: &str, pw: &str) -> Self {
        Self {
            name: name.into(),
            pw_hash: encoding::base64_encode(&hashing::blake2(pw, name).to_vec()),
            role: Role::Reader,
            access: Vec::new(),
        }
    }
}

impl AutoEncoder for User {}
