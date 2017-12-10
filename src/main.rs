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
    let j = serde_json::to_string(&record).unwrap();

    // This needs to be a 16byte hash
    let password = "1234567890123456";
    let salt = "lockchain_spacekookie".as_bytes();

    use aesni::{Aes128, BlockCipher};

    let mut start = 0;
    let mut stop = 16;

    let key = GenericArray::from_slice(password.as_bytes()); // [0u8; 16]
    let cipher = Aes128::new(&key);

    loop {
        if j.len() <= stop as usize {
            break;
        }

        let slice = &j[start..stop].as_bytes();
        let mut block = GenericArray::clone_from_slice(&slice);
        cipher.encrypt_block(&mut block);

        println!("Block: {:?}", block);

        start = stop;
        stop += 16;
    }

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

    // let mut key = [0; 32];
    // derive(&HMAC_SHA256, 100, &salt, &password[..], &mut key);

    // let content = b"My content".to_vec();
    // println!("Content to encrypt's size {}", content.len());

    // let additional_data = "some signature here".as_bytes();

    // let mut in_out = content.clone();
    // println!("Tag len {}", CHACHA20_POLY1305.tag_len());
    // for _ in 0..CHACHA20_POLY1305.tag_len() {
    //     in_out.push(0);
    // }

    // // Opening key used to decrypt data
    // let opening_key = OpeningKey::new(&CHACHA20_POLY1305, &key).unwrap();

    // // Sealing key used to encrypt data
    // let sealing_key = SealingKey::new(&CHACHA20_POLY1305, &key).unwrap();

    // // Random data must be used only once per encryption
    // let mut nonce = vec![0; 12];
    // let rand = SystemRandom::new();
    // rand.fill(&mut nonce).unwrap();

    // // Encrypt data into in_out variable
    // let output_size = seal_in_place(
    //     &sealing_key,
    //     &nonce,
    //     &additional_data,
    //     &mut in_out,
    //     CHACHA20_POLY1305.tag_len(),
    // ).unwrap();

    // println!("Encrypted data's size {}", output_size);

    // let decrypted_data = open_in_place(&opening_key, &nonce, &additional_data, 0, &mut in_out)
    //     .unwrap();

    // println!("{:?}", String::from_utf8(decrypted_data.to_vec()).unwrap());
    // assert_eq!(content, decrypted_data);

}
