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
pub struct Error {
    VaultAlreadyExists,
    InvalidPath,
    InvalidName,
    InvalidCryptoLayer,
    FailedCrypto,
    FailedSelfTest,
    FailedLoading,
    FailedInitalise,
    FailedCreation,
    UnknownFailure,
    #[hidden_docs]
    __NonExhaustive,
}