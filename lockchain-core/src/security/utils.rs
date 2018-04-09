//! A collection of utility submodules

/// Encoding module
pub mod encoding {
    use base64;
    use std::fmt::Write;

    /// Encode a piece of arbitary data into a bse64 string
    pub fn base64_encode(data: &Vec<u8>) -> String {
        return base64::encode(data);
    }

    /// Decode a base64 string into arbitrary data
    pub fn base64_decode(data: &String) -> Vec<u8> {
        return base64::decode(data).unwrap();
    }

    /// Simply encode a byte-string as hexadecimal symbols
    pub fn encode_hex(data: &str) -> String {
        let mut s = String::new();
        for &byte in data.as_bytes() {
            write!(&mut s, "{:X}", byte).expect("Unable to HEX encode!");
        }

        return s;
    }
}

/// A hashing utility module
pub mod hashing {
    use blake2::Blake2s;
    use blake2::digest::{Input, VariableOutput};

    const BLAKE_16_LENGTH: usize = 16;

    /// Hash a value with blake2
    pub fn blake2(data: &str, salt: &str) -> [u8; BLAKE_16_LENGTH] {
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
}

/// Random number utility module for lockchain
/// 
/// Provides stateless secure random number and byte generation
pub mod random {
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
}
