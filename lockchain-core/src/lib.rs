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
extern crate blake2;
extern crate rand;
extern crate keybob;
extern crate nix;
extern crate pam_auth;

pub mod errors;
pub mod traits;
pub mod crypto;
pub mod users;
mod meta;
mod record;

pub use self::crypto::PackedData;
pub use self::record::{Header, Payload, Record, EncryptedBody};
pub use self::meta::{MetaDomain, VaultMetadata};
