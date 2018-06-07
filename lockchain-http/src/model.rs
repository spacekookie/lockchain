//! API data models

use lockchain::errors::Error as LockError;
use serde::{Serialize, de::DeserializeOwned};

#[derive(Serialize, Deserialize)]
pub struct CarrierMessage<T: Serialize + DeserializeOwned> {
    pub error: Result<(), LockError>,
    #[serde(bound(deserialize = "T: Serialize + DeserializeOwned"))]
    pub data: Option<T>,
}