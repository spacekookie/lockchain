//! Athentication errors

use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result};

/// Common errors that can occur when authenticating users
#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    /// Forking an authentication task failed
    FailedFork,
    /// Failed to authenticate via PAM due to a PAM related issue
    FailedPAM,
    /// The provided user either doesn't exist or is not authorised
    UserNotAuthorised,
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
