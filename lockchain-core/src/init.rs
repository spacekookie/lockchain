use errors::VaultError;
use traits::{Body, Vault};

/// Describes the internal permission layout of a vault
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VaultType {
    /// Create an all-powerful root user which can access everything
    Administrated {
        /// Set a root password
        secret: String,
    },
    /// Similar to `Administrated`
    /// but only allows a single-user for a vault
    SoloUser { username: String, secret: String },
}

/// A shared initialisation generator for vaults
///
/// All vaults, regardless of backends
/// or persistence layer
/// share the same common principles
/// of users and permissions.
///
/// This means that intiailisation is shared,
/// regardless of what backend implements it.
///
/// A `VaultGenerator` takes arguments
/// for a generic backend,
/// calls functions provided by said backend
/// and then returns the actual backend.
pub struct Generator {
    #[doc(hidden)]
    pub name: Option<String>,
    #[doc(hidden)]
    pub location: Option<String>,
    #[doc(hidden)]
    pub user_type: Option<VaultType>,
}

impl Generator {
    /// Start a new generator for a generic type
    pub fn new() -> Self {
        Self {
            name: None,
            location: None,
            user_type: None,
        }
    }

    pub fn path<N, L>(self, name: N, location: L) -> Self
    where
        N: Into<String>,
        L: Into<String>,
    {
        Self {
            name: Some(name.into()),
            location: Some(location.into()),
            ..self
        }
    }

    /// Specify the internal user permission structure for this vault
    ///
    /// If you don't know what this means, please consult
    /// the `VaultType` enum documentation
    pub fn user_type(self, t: VaultType) -> Self {
        Self {
            user_type: Some(t),
            ..self
        }
    }

    /// Finally call this function to construct the vault
    pub fn finalise<V, B>(self) -> Result<V, VaultError>
    where
        V: Vault<B>,
        B: Body,
    {
        V::new(self).map(|b| *b)
    }
}
