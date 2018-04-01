//! Hashing submodule
//! 

use blake2::Blake2s;
use blake2::digest::{Input, VariableOutput};

/* To make sure I don't typo all over this */
const BLAKE_16_LENGTH: usize = 16;

pub fn blake2_16(data: &str, salt: &str) -> [u8; BLAKE_16_LENGTH] {
    
    let mut hasher = match Blake2s::new(BLAKE_16_LENGTH) {
        Ok(res) => res,
        Err(some) => panic!(some),
    };

    let to_hash = format!("{}{}", data, salt);
    hasher.process(to_hash.as_bytes());

    let mut buffer = [0u8; BLAKE_16_LENGTH];
    match hasher.variable_result(&mut buffer) {
        Ok(res) => res,
        Err(e) => panic!(e),
    };

    return buffer;
}
