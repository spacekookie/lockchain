//! A shared initialisation block for vaults
//!
//! All vaults, regardless of backends
//! or persistence layer
//! share the same common principles
//! of users and permissions.
//!
//! This means that intiailisation is shared,
//! regardless of what backend implements it.
//!
//! A `VaultGenerator` takes arguments
//! for a generic backend,
//! calls functions provided by said backend
//! and then returns the actual backend.

use traits::{Body, Vault};

/// A generator is initialised with a generic backend
/// which can then chain-call functions to setup the
/// base functionality of a Vault, and then yield
/// a working and initialised instance of the
/// generic vault backend.
pub struct Generator {
    #[doc(hidden)]
    pub name: Option<String>,
    #[doc(hidden)]
    pub location: Option<String>,
}

impl Generator {
    /// Start a new generator for a generic type
    pub fn new() -> Self {
        Self {
            name: None,
            location: None,
        }
    }

    pub fn path<S: Into<String>>(self, name: S, location: S) -> Self {
        Self {
            name: Some(name.into()),
            location: Some(location.into()),
            ..self
        }
    }

    /// Finally call this function to construct the vault
    pub fn finalise<V, B>(self) -> V
    where
        V: Vault<B>,
        B: Body,
    {
        V::new(self)
    }
}
