//!

use databody::DataBody;
use lcc::{traits::{Encryptable, EncryptionHandler},
          EncryptedBody};

impl Encryptable for DataBody {}

pub struct AesEngine {}

impl EncryptionHandler<DataBody> for AesEngine {
    fn encrypt(&mut self, item: DataBody) -> EncryptedBody {
        unimplemented!()
    }

    fn decrypt(&mut self, item: EncryptedBody) -> DataBody {
        unimplemented!()
    }
}
