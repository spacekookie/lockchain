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


    // use blake2::Blake2s;
    // use blake2::digest::{Input, VariableOutput};
    // use std::fmt::Write;

    // let mut hasher = Blake2s::new(16).unwrap();
    // // instead of `input` method here we should use `process`
    // hasher.process(b"hello world");
    // let mut buf = [0u8; 16];
    // hasher.variable_result(&mut buf).unwrap();



// use blake2::{Blake2b, Digest};

// let record = Record::new("facebook", "web");
// let mut j = serde_json::to_string(&record).unwrap();
// println!("{}", j);

// // create a Blake2b object
// let mut hasher = Blake2b::new();

// // write input message
// hasher.input(j.as_bytes());

// // read hash digest and consume hasher
// let output = hasher.result();
// println!("{:x}", output);