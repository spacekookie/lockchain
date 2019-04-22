//! Provides an authentication module backed by PAM
//!
//! The way a user is authenticated is via the `lockchain` group
//! and a simple writing/ deleting of a lock file.

use crate::errors::AuthError;

/// Simple way to authenticate a user for administrative actions
///
/// Attempts to open a PAM session for the provided user/pw combination
/// then attempts to write to a tmpfile in the lockchain config directory.
/// If this action is successful the user is either the same running the
/// lockchain server *or* has access to the file via group permissions.
///
/// This does rely on `lockchain` being properly configured on the server
/// i.e. not using public permissions for the configuration/ state directory.
///
/// **Note** as of `lockchain v0.9.0` this function has not been implemented
/// yet due to issues in the `pam-auth` dependency.
#[allow(unused_variables)]
pub fn pam_authenticate(username: &str, password: &str) -> Result<(), AuthError> {
    unimplemented!()
}

// pub fn yubikey_authenticate(username: &str, yubi_id: &str) -> Result<(), AuthError> {
//     unimplemented!()
// }
