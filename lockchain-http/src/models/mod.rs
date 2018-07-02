//! Data models specific to the lockchain API

pub mod inputs;
pub mod responses;

use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;

/// A wrapper model for various API response types
#[derive(Serialize, Deserialize)]
pub enum Response{
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
