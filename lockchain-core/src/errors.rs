//! Common lockchain error handling
//!
//! When working with a lockchain vault or record set
//! there are a lot of things that can go wrong.
//!
//! This module handles any generic failure condition
//! and logic to escallate from one to the next, e.g.
//! turning a `VaultAlreadyExists` failure to
//! a `FailedInitialise`.

/// A collection of common error codes that can be
/// returned by lockchain API functions
#[derive(Serialize, Deserialize)]
pub enum Error {
    /// Creating a vault where one already exists
    VaultAlreadyExists,
    /// When providing an invalid path
    InvalidPath,
    /// When providing an invalid name (unprintable characters, empty, etc)
    InvalidName,
    /// Provided crypto layer is lacking features or missing
    InvalidCryptoLayer,
    /// Failed to initialise cryptography module
    FailedCrypto,
    /// Failed the internal self-test
    FailedSelfTest,
    /// Failed loading a file or vault
    FailedLoading,
    /// Failed to initialise a module
    FailedInitalise,
    /// Failed to create a new record, entry or vault
    FailedCreation,
    /// An unknown error occured =/
    UnknownFailure,
    // #[hidden_docs]
    __NonExhaustive,
}
