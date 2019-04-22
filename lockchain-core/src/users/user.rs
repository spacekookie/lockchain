//! User representation module

use super::rights::{Access, Role};
use crate::crypto::{encoding, hashing, random};
use std::collections::HashMap;
use crate::traits::AutoEncoder;
use serde::{Serialize, Deserialize};

/// A generic user representation
///
/// A user has an identify check built in
/// that can verify a passphrase
/// but is ultimately only a metadata item for a API layers.
/// Any layer is free to disregard these access rights
/// (as such, they should not be considered security,
/// only obscurity/ management control)
///
/// A company might not want to allow non-admins
/// to create new vaults or users to delete records.
/// This does not cryptographically stop anyone
/// from breaking into the company server,
/// swapping the source code and
/// changing the rules!
///
/// An user can have multiple role-access pairs.
///
/// A user can be stored in a `UserStore`,
/// along-side authorised keys
#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    #[doc(hidden)]
    pub name: String,
    #[doc(hidden)]
    pub pw_hash: String,
    #[doc(hidden)]
    pub rights: HashMap<Access, Role>,
    #[doc(hidden)]
    pub token: Option<String>,
}

impl User {
    /// Register a new user with a name and password
    pub fn register(name: &str, pw: &str) -> Self {
        Self {
            name: name.into(),
            pw_hash: encoding::base64_encode(&hashing::blake2(pw, name).to_vec()),
            rights: HashMap::new(),
            token: None,
        }
    }
    /// Verify a user password input
    pub fn verify(&self, pw: &str) -> bool {
        self.pw_hash == encoding::base64_encode(&hashing::blake2(pw, &self.name).to_vec())
    }
    /// Provides a hook to use second-factor authentication to authorise
    ///
    /// This is meant to be used with an external Yubikey
    pub fn second_auth_verify(&mut self) -> bool {
        unimplemented!()
    }
    /// Generate a token unique to this user (or return the existing one)
    pub fn token(&mut self) -> String {
        if self.token.is_none() {
            self.token = Some(encoding::base64_encode(&random::bytes(256)));
        }

        self.token.as_ref().unwrap().clone()
    }
    /// Verify that a user is allowed access to a piece of data
    ///
    /// `None` means "no access of any kind"
    pub fn has_access(&self, item: Access) -> Option<Role> {
        self.rights.get(&item).map(|i| i.clone())
    }
    /// Modify access to an item for a role or create a new access entry
    pub fn give_access(&mut self, item: Access, role: Role) {
        self.rights.insert(item, role);
    }
}

impl AutoEncoder for User {}
