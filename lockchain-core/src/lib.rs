//! Common library types used in lockchain crates
#![feature(external_doc)]
#![doc(include = "../README.md")]

#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate serde;

pub mod errors;
mod record;
pub mod traits;
mod vault;

pub use self::record::{Header, Payload, Record};
pub use self::vault::Vault;
