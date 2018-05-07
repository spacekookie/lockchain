//! Common library types used in lockchain crates

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate chrono;

pub mod errors;
pub mod traits;
mod record;
mod vault;

pub use self::record::{Record, Header, Body, Payload};
pub use self::vault::Vault;