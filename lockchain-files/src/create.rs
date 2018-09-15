//! A small submodule which handles all aspects of vault creation
#![allow(unused_imports)]

use lcc::errors::VaultError;
use lcc::{traits::Body, Generator};
use std::collections::HashMap;

use ::FileVault;
use ::config::{VaultConfig, ConfigError};
use ::fs::{Filesystem, FileType};


impl<T: Body> FileVault<T> {
    /// A small utility to create a new file vault
    pub(crate) fn create(gen: Generator) -> Result<Self, VaultError> {
        let (name, location) = Self::get_path(&gen)?;

        let fs = Filesystem::new(location, name);
        fs.scaffold().map_err(|_| VaultError::FailedCreation)?;
        
        let cfg = VaultConfig::new(&gen);

        // Ok(Box::new(
        //     Self {
        //         meta_info: (
        //             gen.name.clone().unwrap().into(),
        //             gen.location.clone().unwrap().into(),
        //         ),
        //         records: HashMap::new(),
        //         config: VaultConfig::new(),
        //         metadata: HashMap::new(),
        //         fs: Filesystem::new(&gen.location.unwrap(), &gen.name.unwrap()),
        //         users: UserStoreMapper::new(),
        //     }.initialize(),
        // ))
        unimplemented!()
    }

    fn get_path(gen: &Generator) -> Result<(&str, &str), VaultError> {
        

        Err(VaultError::IncompleteGenerator)
    }
}
