//!

use lcc::traits::{AutoEncoder, Encryptable, EncryptionHandler};
use lcc::{EncryptedBody, PackedData};

use miscreant::aead::{Aes256Siv, Algorithm};

use super::databody::DataBody;

use lcc::crypto::random;
use lcc::crypto::{Key, KEY_LENGTH};

impl Encryptable for DataBody {}

pub struct AesEngine {
    ctx: Aes256Siv,
    _key: Key,
    iv: Vec<u8>,
}

impl AesEngine {
    /// Generate new key and encryption engine
    pub fn generate() -> Self {
        let key = Key::generate();
        Self {
            ctx: Aes256Siv::new(&key.to_slice()),
            _key: key,
            iv: random::bytes(KEY_LENGTH),
        }
    }
    /// Generate an Aes context from password
    pub fn from_pw(pw: &str, salt: &str) -> Self {
        let key = Key::from_password(pw, salt);
        Self {
            ctx: Aes256Siv::new(&key.to_slice()),
            _key: key,
            iv: random::bytes(KEY_LENGTH),
        }
    }

    /// Load a packed data object which contains an Aes context
    pub fn load(packed: PackedData, pw: &str, salt: &str) -> Option<Self> {
        let mut temp = Self::from_pw(pw, salt);
        let k = Key::decode(&String::from_utf8(temp.decrypt_primitive(&packed)?).ok()?);

        Some(Self {
            ctx: Aes256Siv::new(&k.to_slice()),
            _key: k,
            iv: packed.iv
        })
    }

    /// Serialise the current context to save it somewhere
    pub fn save(&mut self) -> PackedData {
        let k = self._key.to_vec();
        self.encrypt_primitive(&k)
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
        Some(self.ctx
            .open(packed.nonce.as_slice(), iv, packed.data.as_slice())
            .ok()?)
    }
}

impl EncryptionHandler<DataBody> for AesEngine {
    fn encrypt(&mut self, item: DataBody) -> EncryptedBody {
        let ser = item.encode();
        let data = self.encrypt_primitive(&ser.as_bytes().to_vec()).encode();
        EncryptedBody { data }
    }

    fn decrypt(&mut self, item: EncryptedBody) -> Option<DataBody> {
        let packed = PackedData::decode(&item.data);
        Some(DataBody::decode(&String::from_utf8(
            self.decrypt_primitive(&packed)?,
        ).ok()?))
    }
}
