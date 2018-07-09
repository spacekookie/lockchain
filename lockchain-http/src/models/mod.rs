//! Data models specific to the lockchain API

pub mod inputs;
pub mod responses;

/// A wrapper model for various API response types
#[derive(Serialize, Deserialize)]
pub enum Response
{
    /// Indicate general success of an operation
    Success,
    /// Indicate a failure of some kind
    Failure(responses::OperationFailed),
    /// Returns a login token
    Token(responses::TokenMessage),
    /// Returns general API information
    Api(responses::ApiInformation),
    /// Returns a list of all vaults
    Vaults(responses::VaultList),
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub struct NoneError;
impl Error for NoneError {}

use std::fmt::{Display, Formatter, Result};

impl Display for NoneError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<None>")
    }
}
