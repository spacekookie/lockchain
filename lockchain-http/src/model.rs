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

/// Fields provided when creating a new vault
#[derive(Serialize, Deserialize)]
pub struct VaultCreate {
    name: String,
    location: String,
}
