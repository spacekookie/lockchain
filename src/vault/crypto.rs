//! Crypto submodule for vaults
//! 
//! Provides stateless functions to encrypt and decrypt vault data
//! 

use aesni::{Aes256, BlockCipher};

pub fn aes256_encrypt() {
    let cipher = Aes256::new(&Default::default());
    let mut input = Default::default();

    let a = "something";
    let mut aa = a.as_bytes();
    println!("Before: {:?}", input);

    cipher.encrypt_block(&mut input);
    println!("After: {:?}", input);
    
    // test::black_box(&input);
    
    // });
    // bh.bytes = input.len() as u64;
}

// pub fn aes256_decrypt(bh: &mut test::Bencher) {
//     let cipher = Aes256::new(&Default::default());
//     let mut input = Default::default();

//     bh.iter(|| {
//         cipher.decrypt_block(&mut input);
//         test::black_box(&input);
//     });
//     bh.bytes = input.len() as u64;
// }

