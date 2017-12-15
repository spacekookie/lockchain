//! Random number utility module for lockchain
//! 
//! Provides stateless secure random number and byte generation

use rand::{thread_rng, Rng};


/// Generate a random number with an upper bound
#[allow(unused)]
pub fn number(bound: u64) -> u64 {
    return thread_rng().next_u64() % bound;
}


/// Generate a sequence of random bytes that are returned
/// as a vector.
/// 
/// Can at most allocate 2048 bytes at a time
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