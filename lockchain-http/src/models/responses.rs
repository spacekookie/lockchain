use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;

/// A generic container that json/error wraps lockchain-types
///
/// This is heavily used in the lockchain-REST API and can be utilised
/// to send both encrypted and cleartext data via the API endpoint, using
/// the same code.
#[derive(Serialize, Deserialize)]
pub struct CarrierMessage<T, E>
where
    T: Serialize + DeserializeOwned,
    E: Error + Serialize + DeserializeOwned,
{
    #[serde(bound(deserialize = "E: Serialize + DeserializeOwned"))]
    pub error: Result<(), E>,
    #[serde(bound(deserialize = "T: Serialize + DeserializeOwned"))]
    pub data: Option<T>,
}

/// A simple message that describes an invalid operation
#[derive(Serialize, Deserialize)]
pub struct OperationFailed {
    pub reason: String,
    pub code: u32,
}

/// Message that returns a token
#[derive(Serialize, Deserialize)]
pub struct TokenMessage {
    pub username: String,
    pub token: String,
}

/// **Returns** Api information
#[derive(Serialize, Deserialize)]
pub struct ApiInformation {
    pub version: String,
    pub providers: Vec<String>,
    pub hostname: Option<String>,
    pub supported: String,
}

/// **Returns** List existing vaults
#[derive(Serialize, Deserialize)]
pub struct VaultList {
    pub vaults: Vec<String>,
    pub count: usize,
}

/// Response to creating a new vault
#[derive(Serialize, Deserialize)]
pub struct VaultCreateResponse {
    pub name: String,
    pub created: bool,
    pub error: Option<String>,
}
