//! Data models specific to the lockchain API

use lockchain::errors::Error as LockError;
use serde::{de::DeserializeOwned, Serialize};

/// A generic container that json/error wraps lockchain-types
///
/// This is heavily used in the lockchain-REST API and can be utilised
/// to send both encrypted and cleartext data via the API endpoint, using
/// the same code.
#[derive(Serialize, Deserialize)]
pub struct CarrierMessage<T: Serialize + DeserializeOwned> {
    pub error: Result<(), LockError>,
    #[serde(bound(deserialize = "T: Serialize + DeserializeOwned"))]
    pub data: Option<T>,
}

/// A simple message that describes an invalid operation
#[derive(Serialize, Deserialize)]
pub struct OperationFailed {
    pub reason: String,
    pub code: u32,
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

/// Fields provided when creating a new vault
#[derive(Serialize, Deserialize)]
pub struct VaultCreate {
    pub name: String,
    pub location: String,
}

/// Response to creating a new vault
#[derive(Serialize, Deserialize)]
pub struct VaultCreateResponse {
    pub name: String,
    pub created: bool,
    pub error: Option<String>,
}
