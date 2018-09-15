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

        let fs = Filesystem::new(location, name);
        fs.scaffold().map_err(|_| VaultError::FailedCreation)?;

        let config = VaultConfig::new(&gen)?;
        let mut users = UserStore::new();

        /* At this point we'll have to create some user */
        use self::VaultType::*;
        match &config.vault_type {
            SoloUser { username, secret } => users.add_user(
                username.clone(),
                Key::from_pw(KeyType::Aes256, &secret, &username),
            ),
            Administrated { secret } => users.add_user(
                "Admin".into(),
                Key::from_pw(KeyType::Aes256, &secret, "admin"),
            ),
        }

        let mut me = Self {
            config,
            fs,
            users,
            ..Default::default()
        };

        /* Make sure to sync all made changes after the scaffold */
        me.sync();

        Ok(me)
    }

    fn get_path(gen: &Generator) -> Result<(&str, &str), VaultError> {
        Err(VaultError::IncompleteGenerator)
    }
}
