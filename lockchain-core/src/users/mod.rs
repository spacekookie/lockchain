//! User and access regulation module
//!
//! Access can be given for a vault or file (based on id)
//! as well as an entire Api endpoint. By default all
//! Rust APIs assume `{ Api, Admin }` access, for other
//! APIs crates (http, ...), a user with access rights
//! has to be specified.
//!
//! A user for an API endpoint is not the same as a user for
//! a vault. An API admin could have access to a vault where
//! they can only read a single file!
//!
//! `User` is also a serialisable struct which contains important
//! data to load and store them into a metadata store.
#![allow(deprecated)]

mod auth;
mod user;
mod rights;
mod tokens;
mod keystore;

mod secrets;
mod userstore;

pub use self::auth::pam_authenticate;
pub use self::keystore::KeyStore;
pub use self::tokens::Token;
pub use self::user::User;

pub use errors::AuthError;
pub use self::rights::{Access, Role};

use crypto::{encoding, hashing, random};
use std::collections::HashMap;
use {
    meta::MetaDomain,
    traits::{AutoEncoder, Base64AutoEncoder},
};

/// A utility structure that manages users and can be derived
/// from/into a metadata object. By default this process uses
/// base64 encoding.
///
/// The workflow for this is to create a new `UserStore`, add
/// users and then use `meta_push_domain` and give it the
/// `UserStore::into()` which is then encoded automatically.
/// The reverse action works the same way
#[deprecated(since="0.10.0", note="Use the `userstore::UserStore` structure instead")]
#[allow(deprecated)]
#[derive(Serialize, Deserialize)]
pub struct UserStore {
    /// A map between username – user item
    users: HashMap<String, User>,
    registry: HashMap<String, Vec<Access>>,
}

impl UserStore {
    /// Generate a sign-up token for a new user which needs to be
    /// provided in order for them to create an account.
    pub fn get_token(&mut self, access: Vec<Access>) -> String {
        let token = ::crypto::encoding::base64_encode(&::crypto::random::bytes(128));
        self.registry.insert(
            token.clone(),
            if access.is_empty() {
                vec![Access::Vault(Role::Reader)]
            } else {
                access
            },
        );

        token
    }

    pub fn get_user(&self, name: &str) -> Option<&User> {
        self.users.get(name)
    }

    pub fn get_all(&self) -> &HashMap<String, User> {
        &self.users
    }

    pub fn add(&mut self, user: User) -> Option<()> {
        self.users.insert(user.name.clone(), user);
        Some(())
    }
}

impl Default for UserStore {
    fn default() -> Self {
        Self {
            users: HashMap::new(),
            registry: HashMap::new(),
        }
    }
}

impl AutoEncoder for UserStore {}

/// Allow users to turn MetaDomains
/// that *are* userstores into a UserStore easily
///
/// Will most likely `panic!` if called on a non UserStore
impl From<(MetaDomain, MetaDomain)> for UserStore {
    fn from((users, registry): (MetaDomain, MetaDomain)) -> Self {
        Self {
            users: users
                .all()
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        match v {
                            ::Payload::Text(s) => User::decode(&String::from_base64(s)).unwrap(),
                            _ => unreachable!(),
                        },
                    )
                })
                .collect(),
            registry: registry
                .all()
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        match v {
                            ::Payload::List(regs) => regs
                                .iter()
                                .map(|reg| {
                                    Access::decode(&String::from_base64(match reg {
                                        ::Payload::Text(s) => s,
                                        _ => unreachable!(),
                                    })).unwrap()
                                })
                                .collect(),
                            _ => unreachable!(),
                        },
                    )
                })
                .collect(),
        }
    }
}

impl From<UserStore> for (MetaDomain, MetaDomain) {
    fn from(us: UserStore) -> Self {
        (
            MetaDomain::new("userstore").fill(
                us.users
                    .iter()
                    .map(|(name, user)| {
                        (
                            name.clone(),
                            ::Payload::Text(user.encode().unwrap().to_base64()),
                        )
                    })
                    .collect(),
            ),
            MetaDomain::new("registry").fill(
                us.registry
                    .iter()
                    .map(|(name, reg)| {
                        (
                            name.clone(),
                            ::Payload::List(
                                reg.iter()
                                    .map(|reg| ::Payload::Text(reg.encode().unwrap().to_base64()))
                                    .collect(),
                            ),
                        )
                    })
                    .collect(),
            ),
        )
    }
}
