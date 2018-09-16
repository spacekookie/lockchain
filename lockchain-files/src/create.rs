//! A small submodule which handles all aspects of vault creation
#![allow(unused_imports)]

use lcc::errors::VaultError;
use lcc::{
    crypto::{Key, KeyType},
    traits::{Body, Vault},
    users::UserStore,
    Generator, VaultType,
};
use std::collections::HashMap;

use config::{ConfigError, VaultConfig};
use fs::Filesystem;
use FileVault;

impl<T: Body> FileVault<T> {
    /// A small utility to create a new file vault
    pub(crate) fn create(gen: Generator) -> Result<Self, VaultError> {
        let (name, location) = Self::get_path(&gen)?;
        let vault_type = gen
            .user_type
            .as_ref()
            .ok_or(VaultError::IncompleteGenerator)?;

        let fs = Filesystem::new(location, name);
        fs.scaffold().map_err(|_| VaultError::FailedCreation)?;

        let config = VaultConfig::new(&vault_type)?;
        let mut users = UserStore::new();

        /* At this point we'll have to create some user */
        use self::VaultType::*;
        match vault_type {
            SoloUser { username, secret } => users.add_user(username.clone(), secret.clone()),
            Administrated { secret } => users.add_user("Admin".into(), secret.clone()),
        }

        let mut me = Self {
            config,
            fs,
            users,
            records: HashMap::new(),
            headers: HashMap::new(),
            metadata: HashMap::new(),
        };

        /* Make sure to sync all changes made after scaffold */
        me.sync();

        Ok(me)
    }

    fn get_path(gen: &Generator) -> Result<(&str, &str), VaultError> {
        match gen {
            &Generator {
                name: Some(ref n),
                location: Some(ref l),
                ..
            } => Ok((&n, &l)),
            _ => Err(VaultError::IncompleteGenerator),
        }
    }
}
