//! Implements an Aes256Siv encryption engine
//!
//! Can be initialised from scratch or with a pw/salt
//! combintaion which derives a key via the `keybob` crate.

use crate::lcc::crypto::{random, Key};
use crate::lcc::traits::{AutoEncoder, Encryptable, EncryptionHandler};
use crate::lcc::{EncryptedBody, PackedData};

use super::databody::DataBody;
use miscreant::{Aead, Aes256SivAead};

impl Encryptable for DataBody {}

pub struct AesEngine {
    ctx: Aes256SivAead,
    _key: Option<Key>,
    iv: Vec<u8>,
}

impl AesEngine {
    /// Initialise an AesEngine and take ownership of a raw key
    pub fn new(key: Key) -> Self {
        assert!(key.len() == 64);

        Self {
            ctx: Aes256SivAead::new(&key.as_slice()),
            _key: Some(key),
            iv: random::bytes(64),
        }
    }

    /// Generate new key and encryption engine
    #[deprecated]
    pub fn generate() -> Self {
        // let key = Key::new(KeyType::Aes256);
        // let len = key.len();
        // Self {
        //     ctx: Aes256Siv::new(&key.as_slice()),
        //     _key: key,
        //     new_key: None,
        //     iv: random::bytes(len),
        // }
        unimplemented!()
    }

    /// Generate an Aes context from password
    #[deprecated]
    pub fn from_pw(_pw: &str, _salt: &str) -> Self {
        // let key = Key::from_pw(KeyType::Aes256, pw, salt);
        // let len = key.len();
        // Self {
        //     ctx: Aes256Siv::new(&key.as_slice()),
        //     _key: key,
        //     new_key: None,
        //     iv: random::bytes(len),
        // }
        unimplemented!()
    }

    /// Load a packed data object which contains an Aes context
    #[deprecated]
    pub fn load(_packed: PackedData, _pw: &str, _salt: &str) -> Option<Self> {
        // let mut temp = Self::from_pw(pw, salt);
        // let k: Key = Key::decode(&String::from_utf8(temp.decrypt_primitive(&packed)?).ok()?).ok()?;

        // Some(Self {
        //     ctx: Aes256Siv::new(&k.as_slice()),
        //     _key: k,
        //     new_key: None,
        //     iv: packed.iv,
        // })
        unimplemented!()
    }

    /// Serialise the current context to save it somewhere
    #[deprecated]
    pub fn save(&mut self) -> PackedData {
        // let k = self._key.as_slice().into();
        // self.encrypt_primitive(&k)
        unimplemented!()
    }

    fn encrypt_primitive(&mut self, data: &Vec<u8>) -> PackedData {
        let nonce = random::bytes(64);
        let iv = &self.iv.as_slice();
        let encrypted = self.ctx.seal(nonce.as_slice(), iv, data.as_slice());

        PackedData {
            iv: self.iv.clone(),
            data: encrypted,
            nonce: nonce,
        }
    }

    fn decrypt_primitive(&mut self, packed: &PackedData) -> Option<Vec<u8>> {
        let iv = &self.iv.as_slice();
        Some(
            self.ctx
                .open(packed.nonce.as_slice(), iv, packed.data.as_slice())
                .ok()?,
        )
    }
}

impl EncryptionHandler<DataBody> for AesEngine {
    fn encrypt(&mut self, item: DataBody) -> EncryptedBody {
        let ser = item.encode().unwrap();
        let data = self
            .encrypt_primitive(&ser.as_bytes().to_vec())
            .encode()
            .unwrap();
        EncryptedBody { data }
    }

    fn decrypt(&mut self, item: EncryptedBody) -> Option<DataBody> {
        let packed = PackedData::decode(&item.data).ok()?;
        Some(DataBody::decode(&String::from_utf8(self.decrypt_primitive(&packed)?).ok()?).ok()?)
    }
}
