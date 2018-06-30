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

mod auth;
pub use self::auth::Token;

use crypto::{encoding, hashing};
use std::collections::HashMap;
use {meta::MetaDomain, traits::AutoEncoder};

/// Specifies access to a resource
#[derive(Serialize, Deserialize)]
pub enum Access {
    /// Allows specific access to an entire API
    Api,
    /// Allows access to vault metadata & index files
    Vault(String),
    /// Allows access to a record resource inside a vault
    Record(String, String),
}

/// Specifies the capabilities of a user
#[derive(Serialize, Deserialize)]
pub enum Role {
    Reader,
    Editor,
    Admin,
}

/// A generic user representation
///
/// A user has an identify check built in that can verify a passphrase
/// but is ultimately only a metadata item for a API layer. Any layer is
/// free to disregard these access rights (as such, they should not be
/// considered security, only obscurity/ management control)
///
/// A company might not want allow non-admins to create new vaults or
/// users to delete records. This does not cryptographically stop anyone
/// from breaking into the company server, swapping the source code and
/// changing the rules!
///
/// An user can have multiple role-access pairs
#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    pw_hash: String,
    rights: Vec<(Access, Role)>,
}

impl User {
    /// Register a new user with a name and password
    pub fn register(name: &str, pw: &str) -> Self {
        Self {
            name: name.into(),
            pw_hash: encoding::base64_encode(&hashing::blake2(pw, name).to_vec()),
            rights: Vec::new(),
        }
    }
    /// Verify a user password input
    pub fn verify(&self, pw: &str) -> bool {
        self.pw_hash == encoding::base64_encode(&hashing::blake2(pw, &self.name).to_vec())
    }
}

impl AutoEncoder for User {}

/// A utility structure that loads user data
/// from a metadata store backend
#[derive(Serialize, Deserialize)]
pub struct UserStore {
    /// A map between username – user item
    users: HashMap<String, User>,
}

impl UserStore {
    pub fn get_user(&self, name: &str) -> Option<&User> {
        self.users.get(name)
    }
}

impl AutoEncoder for UserStore {}

/// Allow users to turn MetaDomains
/// that *are* userstores into a UserStore easily
///
/// Will `panic!` if called on a non UserStore
impl From<MetaDomain> for UserStore {
    fn from(md: MetaDomain) -> Self {
        use Payload;
        Self {
            users: md
                .all()
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        match v {
                            Payload::Text(s) => User::decode(s).unwrap(),
                            _ => unreachable!(),
                        },
                    )
                })
                .collect(),
        }
    }
}

impl From<UserStore> for MetaDomain {
    fn from(us: UserStore) -> Self {
        MetaDomain::new("userstore").fill(
            us.users
                .iter()
                .map(|(name, user)| (name.clone(), ::Payload::Text(user.encode().unwrap())))
                .collect(),
        )
    }
}
