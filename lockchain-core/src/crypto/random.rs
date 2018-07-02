//! A small convenience wrapper around `rand`

use rand::{thread_rng, Rng};

/// Generate a random number with an upper bound
pub fn number(bound: u64) -> u64 {
    return thread_rng().next_u64() % bound;
}

/// Generate a sequence of random bytes that are returned
/// as a vector.
///
/// Can at most allocate 2048 bytes at a time
/// FIXME: That shouldn't have a limit!
pub fn bytes(length: usize) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();

    if length > 2048 {
        return vec;
    }

    let mut random_data = [0u8; 2048];
    thread_rng().fill_bytes(&mut random_data);

    for i in 0..length {
        vec.push(random_data[i]);
    }

    return vec;
}

/// A small utility wraper around bcrypt to allow
/// easy password checking.
pub mod passwd {
    use bcrypt::{self, DEFAULT_COST};

    /// Create a new password, returning a hash
    pub fn create(pw: &str) -> Option<String> {
        Some(bcrypt::hash(pw, DEFAULT_COST).ok()?)
    }

    /// Verify a password against it's stored hash
    pub fn verify(pw: &str, hash: &str) -> Option<bool> {
        bcrypt::verify(pw, hash).ok()
    }
}
