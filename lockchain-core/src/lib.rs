//! Common library types used in lockchain crates
#![feature(external_doc)]
#![doc(include = "../README.md")]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate chrono;
extern crate bcrypt;
extern crate base64;

pub mod errors;
pub mod traits;
mod users;
mod record;
mod vault;

pub use self::record::{Header, Payload, Record, EncryptedBody};
pub use self::vault::Vault;
pub use self::users::User;