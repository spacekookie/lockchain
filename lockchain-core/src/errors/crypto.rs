//! I sell crypto and crypto accessories (errors)

use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    /// Provided crypto layer is lacking features or missing
    InvalidCryptoLayer,
    /// Failed to initialise cryptography module
    FailedCrypto,
    /// Invalid key or user identity
    FailedKey,
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match self {
                _ => "Unknown failure",
            }
        )
    }
}
