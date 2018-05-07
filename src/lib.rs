//! Lockchain library core
//! 
//! **Documentation TBD**
//! 
//! In short: this crate handles handling of lockchain vaults. A vault
//! is a collection of secret records that can be searched in an efficient
//! manner, without ever having to keep the entire set in memory 
//! in unencrypted form.
//! 
//! This is primarily used in the lockchain password manager.

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate miscreant;
extern crate base64;
extern crate blake2;
extern crate chrono;
extern crate rand;

pub mod version;
pub mod record;
pub mod vault;
pub mod security;

// Export some commonly used types
pub use vault::{Vault, ErrorType};
pub use record::Payload;