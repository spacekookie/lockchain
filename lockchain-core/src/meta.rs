//! A cleartext metadata record for a vault metadata store
//!
//! This MetaItem uses a lot of the same traits and mechanisms
//! as the normal Vault Record, without having to rely on encryption
//! or trait base security.

use crate::record::Payload;
use std::collections::HashMap;
use crate::traits::{AutoEncoder, Body};
use serde::{Serialize, Deserialize};

/// A simple representation of metadata for a vault or vault section
pub struct VaultMetadata {
    pub name: String,
    pub location: String,
    pub size: usize,
}

/// A metadomain is a simplified version of a cleartext record.
///
/// It is not encoded in special ways, it is not used in any way
/// for secret information. All data inside a metadata file
/// (for example, living inside the `metadata` folder of a vault)
/// is public to all.
///
/// It can be used for things that need to be stored in encrypted form
/// where the encryption key is never present. Or for simple authentication
/// verification such as:
///
/// - User registry
/// - Per-user encrypted primary keys
/// - Usage statistics shared between clients
///
/// It implements a series of traits which means it's possible to easily
/// interact with to store data.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct MetaDomain {
    /// The name of this meta domain
    name: String,
    body: HashMap<String, Payload>,
}

impl MetaDomain {
    /// Create a new domain space struct
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            body: HashMap::new(),
        }
    }

    /// Return a MetaDomain that is filled with pre-existing data
    pub fn fill(self, new_body: HashMap<String, Payload>) -> Self {
        Self {
            body: new_body,
            ..self
        }
    }

    /// Insert a single value into the body
    pub fn insert<S: Into<String>>(&mut self, _key: S, _value: Payload) -> &mut Self {
        unimplemented!()
    }

    /// Return a read-only reference to the entire body
    pub fn all(&self) -> &HashMap<String, Payload> {
        &self.body
    }

    /// Return the domain name for easy comparison
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the number of items in this domain
    pub fn size(&self) -> usize {
        self.body.len()
    }
}

impl AutoEncoder for MetaDomain {}

impl Body for MetaDomain {
    fn get_field(&self, name: &str) -> Option<&Payload> {
        self.body.get(name)
    }

    fn set_field(&mut self, key: &str, value: Payload) -> Option<()> {
        self.body
            .insert(key.into(), value)
            .map_or(Some(()), |_| Some(()))
    }

    /// Not implemented, always returns None
    fn flatten(&mut self) -> Option<()> {
        None
    }
}
