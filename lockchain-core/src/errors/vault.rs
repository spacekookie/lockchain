//! Vault workflow and I/O errors

use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    /// Indicates that **mandatory** fields in the
    /// vault generator haven't been set – vault
    /// constructor was never called.
    IncompleteGenerator,
    /// A vault already exists with that path-id
    AlreadyExists,
    /// The provided path is invalid
    InvalidPath,
    /// The provided name is invalid
    /// 
    /// This usually means the backing storage doesn't support some
    /// character in the name which can sometimes occur if the name
    /// contains special unicode characters that a filesystem doesn't
    /// recognise as valid characters.
    InvalidName,
    /// The combination of selected components threw a runtime incompatibility error
    InvalidCompoents {
        /// Optionally the type that is incompatible (if it can be determined)
        tt: Option<String>,
    },
    /// Vault failed it's checksum self-test
    /// 
    /// This is problematic because it also means the vault was unable to correct
    /// any errors. Either the backing storage has some serious issues or maybe
    /// an external sync process that lockchain can't detect is still working.
    FailedSelfTest,
    /// Failed to initialise lockchain vault handler
    FailedInitalise,
    /// Failed to create a vault for an unknown reason
    FailedCreation,
    /// Failed to load a vault for an unknown reason
    FailedLoading,
    /// Failed to close the vault properly.
    /// 
    /// This could be because the backing storage is no longer available
    /// or permisions to write have been revoked.
    FailedClosing,
    /// Make sure we don't break user code with new options
    #[doc(hidden)]
    __NonExhaustive,
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
