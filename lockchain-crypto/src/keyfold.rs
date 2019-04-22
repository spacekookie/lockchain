//! Keyfolds map keys to encrypted keys

use crate::lcc::crypto::{Key, KeyType};
use crate::lcc::traits::EncryptionHandler;
use crate::lcc::EncryptedBody;

use crate::AesEngine;

/// Transparent key-encrypter utility
///
/// This structure acts as a mapper between the
/// encrypted keys that are stored in a vault and
/// the decrypted keys that need to exist in order
/// for the `AesEngine` (and similar) to work.
///
/// This means that it is initialised with a
/// user passphrase (and name for salt purposes)
/// and is subsequently able to encrypt keys
/// to be stored in a vault persistence medium
/// or decrypt keys that are retrieved via a
/// Vault metadata API.
pub struct Keyfold {
    engine: Option<AesEngine>,
}

impl Keyfold {
    /// Take ownership of the AesEngine for transactions
    pub fn begin(&mut self, engine: AesEngine) {
        self.engine = Some(engine);
    }

    /// Return ownership o the AesEngine
    pub fn end(mut self) -> AesEngine {
        let engine = self.engine.unwrap();
        self.engine = None;
        engine
    }
}

impl EncryptionHandler<Key> for Keyfold {
    fn encrypt(&mut self, _item: Key) -> EncryptedBody {
        unimplemented!()
    }

    fn decrypt(&mut self, _item: EncryptedBody) -> Option<Key> {
        unimplemented!()
    }
}
