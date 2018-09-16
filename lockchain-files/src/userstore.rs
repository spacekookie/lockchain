//! Implements serialization, desrialization for UserStore

use lcc::{errors::DataError, traits::AutoEncoder, users::UserStore};

pub trait DiskMirror {
    fn to_disk(&self) -> Vec<u8>;
    fn from_disk(Vec<u8>) -> Result<Box<Self>, DataError>;
}

impl DiskMirror for UserStore {
    fn to_disk(&self) -> Vec<u8> {
        self.encode().unwrap().into_bytes()
    }

    fn from_disk(vec: Vec<u8>) -> Result<Box<Self>, DataError> {
        Self::decode(::std::str::from_utf8(vec.as_slice()).map_err(|_| DataError::FailedDecode)?)
            .map(|s| Box::new(s))
            .map_err(|_| DataError::FailedDecode)
    }
}
