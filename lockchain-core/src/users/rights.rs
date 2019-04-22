//! Permission and access system for lockchain

use crate::traits::AutoEncoder;
use serde::{Deserialize, Serialize};

/// Specifies access to a resource
#[derive(Hash, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Access {
    /// A key that is only used to re-encrypt sub-keys
    Root,
    /// Allows access to vault metadata & index files
    Vault(Role),
    /// Allows access to a record resource inside a vault
    Record(Role, String),
}

impl AutoEncoder for Access {}

/// Specifies the capabilities of a user
#[derive(Hash, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Role {
    /// Only has read access
    Reader,
    /// Can edit any field in a record
    Editor,
    /// Can modify base structure, squash and delete records
    Admin,
}

impl AutoEncoder for Role {}
