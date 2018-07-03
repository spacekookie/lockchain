use lockchain::errors::Error as LockError;
use serde::{de::DeserializeOwned, Serialize};

/// A generic container that json/error wraps lockchain-types
///
/// This is heavily used in the lockchain-REST API and can be utilised
/// to send both encrypted and cleartext data via the API endpoint, using
/// the same code.
#[derive(Serialize, Deserialize)]
pub struct CarrierMessage<T>
where
    T: Serialize + DeserializeOwned,
{
    pub error: Result<(), LockError>,
    #[serde(bound(deserialize = "T: Serialize + DeserializeOwned"))]
    pub data: Option<T>,
}

/// A simple message that describes an invalid operation
/// 
/// `explain()` can return a localised string, that provides
/// more details than the error itself.
/// 
/// `LockError` is a generic type provided by `lockchain-core`
/// which is meant to represent any type of error that can
/// occur in the lockchain ecosystem.
#[derive(Serialize, Deserialize)]
pub struct OperationFailed {
    pub explain: String,
    pub error: LockError,
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
