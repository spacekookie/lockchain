//! A common set of functions & types for the `lockchain` ecosystem.
//!
//! This crate by itself doesn't do much. It is meant to be used as the central
//! adapter between a variety of other crates from the `lockchain` ecosystem,
//! that all plug and interact with types and functions defined in this library.
//!
//! This documentation is meant as an outline of what the core crate does and how
//! you can use it, in combination with other libraries, in your application.
//!
//! ## Overview
//!
//! At it's centre, `lockchain-core` defines storage traits. These come in a few
//! forms, from `Vault`, being a very generic interface over what is meant as a
//! secure storage collection, to `Record`, being an individual entry in such a
//! system. This means that both the nature of `Vault` and `Record` are generic
//! implementation details, left to you to pick for your application, depending on
//! what fits your needs.
//!
//! Additionally there are of course crypto primitives. `lockchain-core` exposes the
//! `keybob` API for generation and verification of clear text secrets that can be
//! padded to generate AES encryption keys. It adds a user management layer that provides
//! login, permissions as well as second-factor authentication (such as a yubikey). And it
//! provides an easy to use keystore, which binds encrypted keys to user identities, so that
//! decryption never has to be done outside of the users scope.
//!
//! ---
//!
//! ## Usage
//!
//! This means that there's no one way of using `lockchain-core`, instead there are other crates
//! that plug into it. Following is a list of crates, maintained by the `lockchain` team that
//! were designed to work seemlessly with `lockchain-core`.
//!
//! - `lockchain-crypto` is an adapter layer that adds the ability to stream-decrypt records from
//! any kind of vault
//! - `lockchain-files` is a storage adapter which implements a file-storage layer for a vault
//! - `lockchain-memory` is a storage adapter which implements a vault only in memory
//! - `lockchain-client` provides a shim layer between a common client interface and several server-facing
//! communication interfaces, such as `http` or `unix-sockets`
//! - `lockchain-http` provides an http shim on top of the core lockchain API's
//! - `lockchain-unix` provides a unix socket API shim, similar to the http layer
//!
//! The core principle behind lockchain's design was that the server can store encrypted files, without
//! having the capability of being made to decrypt them. That means that the code required
//! is physically not contained in the binary.
//!
//! Primarily this means that crypto is always done on the "client side", however this is up to _you_ to define.
//! Your application might have different needs than were envisioned for lockchain, and as such you can pick
//! and choose from features across the entire `lockchain` ecosystem to which fit your usecase best.
//!
//! ## Something missing?
//!
//! This crate ecosystem is still in active development. There are several projects that aim to use
//! the `lockchain` ecosystem for secure storage needs. As such, we hope to have covered most use cases
//! already.
//!
//! If we missed something, please let us know!

#[macro_use]
extern crate serde_derive;
extern crate base64;
extern crate bcrypt;
extern crate blake2;
extern crate chrono;
extern crate keybob;
extern crate nix;
extern crate pam_auth;
extern crate rand;
extern crate serde;
extern crate serde_json;

pub mod crypto;
pub mod errors;
mod meta;
mod record;
pub mod traits;
pub mod users;

mod init;

pub use self::crypto::PackedData;
pub use self::meta::{MetaDomain, VaultMetadata};
pub use self::record::{EncryptedBody, Header, Payload, Record};
pub use self::init::{VaultType, Generator};

/// Export commonly used types via the prelude
pub mod prelude {
    pub use super::crypto::PackedData;
    pub use super::meta::{MetaDomain, VaultMetadata};
    pub use super::record::{EncryptedBody, Header, Payload, Record};
    pub use super::init::{VaultType, Generator};
}
