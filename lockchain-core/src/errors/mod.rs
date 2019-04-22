//! Common lockchain error handling
//!
//! `Error` is a wrapper type around many different errors
//! that are provided by different parts of
//! the `lockchain-core` crate.
//!
//! It also re-exports those error types so that a user can use
//! the error handling module simply by including `lockchain_core::errors::*`

mod auth;
mod crypto;
mod data;
mod vault;

pub use self::auth::Error as AuthError;
pub use self::crypto::Error as CryptoError;
pub use self::data::Error as DataError;
pub use self::vault::Error as VaultError;

use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    /// A common "unknown" type for errors
    /// that can't be associated with a type or
    /// simply need to be stubbed until more
    /// information is available.
    Unknown,
    /// A basic vault operation error
    Vault(VaultError),
    /// Errors occuring during authentication
    Auth(AuthError),
    /// Cryptographic errors
    Crypto(CryptoError),
    /// Data integrity or retrieval errors
    Data(DataError),
    /// Make sure we don't break user code with new options
    #[doc(hidden)]
    __NonExhaustive,
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use self::Error::*;
        write!(
            f,
            "{}",
            match self {
                Vault(ee) => format!("VaultError: {}", ee),
                Auth(ee) => format!("AuthError: {}", ee),
                Crypto(ee) => format!("CryptoError: {}", ee),
                Data(ee) => format!("DataError: {}", ee),
                _ => "Unknown error".into(),
            }
        )
    }
}
