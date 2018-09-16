use crypto::{Key, KeyType};
use std::borrow::Cow;

/// An authentication token that can be compared in constant time
///
/// ```
/// use lockchain_core::users::Token;
/// let t1 = Token::new();
/// let t2 = Token::new();
///
/// // Will fail, but no expose failure length
/// assert!(t1 != t2);
/// ```
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Token {
    inner: Key,
}

impl Token {
    pub fn new() -> Self {
        Self {
            inner: Key::new(KeyType::Aes128),
        }
    }
}

/// A request wrapper around a username and token
///
/// This structure is accepted by most Vault-trait
/// functions to reduce the number of paramters required.auth
///
/// Because `Request` objects are short-lived and numerous,
/// they only deal with references to the original
/// username and token data.
#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct Request<'outer> {
    username: &'outer str,
    token: Cow<'outer, Token>,
}
