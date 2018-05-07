//! A commonly used user-abstraction for the lockchain ecosystem

use bcrypt::{self, DEFAULT_COST};

/// Simple user authentication abstraction
#[allow(dead_code)]
pub struct User {
    name: String,
    pw_hash: String,
    pw_salt: String,
    token: Option<String>,
}

impl User {
    ///
    pub fn register(name: &str, password: &str, salt: &str) -> Option<User> {
        Some(User {
            name: name.to_owned(),
            pw_hash: bcrypt::hash(&format!("{}{}", password, salt), DEFAULT_COST).ok()?,
            pw_salt: salt.to_owned(),
            token: None,
        })
    }
}
