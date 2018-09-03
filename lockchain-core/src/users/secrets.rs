//! A secrets type module that wraps around some user content with metadata

use traits::AutoEncoder;

/// Specifies the type of secret that's used to derive a vault user secret
#[derive(Serialize, Deserialize)]
pub enum SecretType {
    /// A simple password
    Plain,
    /// A keyfile that allows asymetric trust operations
    Keyfile,
    /// Signing a user password with the id of a yubikey
    Combine,
}

impl AutoEncoder for SecretType {}

/// The backing secret for user authentication
/// 
/// This is _always_ in a non-recoverable form, i.e. a hash
/// and salted password. **However** it does reveal something
/// about the user setup, i.e. the type of secret used.
/// 
/// Depending on what secret is used, there are other operations that
/// might be supported to verify operations. For example, a `Keyfile`
/// secret can deposit the entire public key in the `content` field, 
/// then use asymmetric operations to verify operations more thoroughly.
#[derive(Serialize, Deserialize)]
pub struct UserSecret {
    tt: SecretType,
    content: String,
}

impl AutoEncoder for UserSecret {}
