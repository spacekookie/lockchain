use traits::AutoEncoder;

/// Specifies access to a resource
#[derive(Hash, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Access {
    /// Allows access to vault metadata & index files
    Vault(Role),
    /// Allows access to a record resource inside a vault
    Record(Role, String),
}

impl AutoEncoder for Access {}

/// Specifies the capabilities of a user
#[derive(Hash, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Role {
    Reader,
    Editor,
    Admin,
}

impl AutoEncoder for Role {}
