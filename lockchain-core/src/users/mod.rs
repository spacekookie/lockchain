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
mod user;
mod rights;
mod tokens;

mod secrets;
mod userstore;

pub use self::auth::pam_authenticate;
pub use self::tokens::Token;
pub use self::user::User;
pub use self::userstore::UserStore;

pub use errors::AuthError;
pub use self::rights::{Access, Role};
