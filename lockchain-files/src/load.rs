//! A small submodule which handles all aspects of vault creation
#![allow(unused_imports)]

use lcc::errors::VaultError;
use lcc::{traits::Body, Generator};
use std::collections::HashMap;

use ::FileVault;
use ::config::{VaultConfig, ConfigError};
use ::fs::Filesystem;


impl<T: Body> FileVault<T> {
    /// A small utility to load an existing file vault
    pub(crate) fn load(name: &str, location: &str) -> Result<Self, VaultError> {
        unimplemented!()

        // self.config = match VaultConfig::load(&self.fs.root) {
        //     Ok(cfg) => cfg,
        //     _ => return Err(VaultError::FailedLoading),
        // };
        // Ok(Box::new(self))

        // Self {
        //     meta_info: (name.into(), location.into()),
        //     records: HashMap::new(),
        //     config: VaultConfig::new(Gene),
        //     metadata: HashMap::new(),
        //     fs: Filesystem::new(location, name),
        //     users: UserStoreMapper::new(),
        // }.load()
    }

}
