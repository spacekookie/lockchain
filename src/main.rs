//! This will become the lockchain library crate at some point
//!
//! For now it's a hybrid between a library and a Gtk+ UI

extern crate chrono;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod vault;
use vault::*;

extern crate base64;
extern crate aesni;
extern crate generic_array;
use generic_array::GenericArray;

fn main() {

    let record = Record::new("facebook", "web");
    let mut j = serde_json::to_string(&record).unwrap();
    println!("{}", j);

    // This needs to be a 16byte hash
    let password = "1234567890123456";
    let salt = "lockchain_spacekookie".as_bytes();

    use aesni::{Aes128, BlockCipher};

    let mut start = 0;
    let mut stop = 16;

    let key = GenericArray::from_slice(password.as_bytes()); // [0u8; 16]
    let cipher = Aes128::new(&key);

    let mut recovered = String::from("");

    let mut padded = false;
    loop {

        if j.len() < stop as usize {
            let mut diff = stop - j.len();  
            padded = true;
            for _ in 0..diff {
                j.push(' ');
            }
        }


        let slice = &j[start..stop].as_bytes();
        let mut block = GenericArray::clone_from_slice(&slice);
        cipher.encrypt_block(&mut block);

        cipher.decrypt_block(&mut block);
        recovered.push_str(std::str::from_utf8(&block).unwrap());

        start = stop;
        stop += 16;

        if padded {
            break;
        }
    }

    println!("Are equals? {}", recovered == j);

    let after_record: Record = serde_json::from_str(&recovered).unwrap();
    println!("{:?}", serde_json::to_string(&after_record).unwrap());

    // let mut block = GenericArray::clone_from_slice(&[0u8; 16]);
    // let mut block8 = GenericArray::clone_from_slice(&[block; 8]);

    // Initialize cipher

    // let block_copy = block.clone();
    // // Encrypt block in-place
    // cipher.encrypt_block(&mut block);
    // // And decrypt it back
    // cipher.decrypt_block(&mut block);
    // assert_eq!(block, block_copy);

    // // We can encrypt 8 blocks simultaneously using
    // // instruction-level parallelism
    // let block8_copy = block8.clone();
    // cipher.encrypt_blocks(&mut block8);
    // cipher.decrypt_blocks(&mut block8);
}
