//! User and access regulation module
//!
//!

mod auth;
pub use self::auth::Token;

use traits::AutoEncoder;

/// Specifies access to a resource
#[derive(Serialize, Deserialize)]
pub enum Access {
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
    id: u64,
    name: String,
    pw_hash: String,
    pw_salt: String,
    role: Role,
    access: Vec<Access>,
}

impl AutoEncoder for User {}
