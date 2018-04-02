//! A version of a record
//!
//! A set of version can be flattened to represent the latest set
//! of changes of a record

use std::collections::BTreeMap;
use record::Payload;


/// An operation that was applied to a version
///
/// An operation is either an insert or a delete.
/// It also carries a string key and a payload value inside
/// a tuple. These are then summed together as a vector.
///
/// This means that if data contradicts itself in the same
/// version the later edit (call) will override the previous
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Operation {
    Insert(String, Payload),
    Delete(String),
}

use self::Operation::{Insert, Delete};

/// Represents a series of operations done in sequence
/// that are applied to a record to preserve history of state
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Version {
    version: u64,
    ops: Vec<Operation>,
}

impl Version {
    /// Create a simple new version
    pub fn new(ver: u64) -> Version {
        return Version {
            version: ver,
            ops: Vec::new(),
        };
    }

    /// A simple utility to add an INSERT operation
    ///
    /// Checks if the same operation already exists
    pub fn insert(&mut self, key: &str, val: Payload) {
        let i = Insert(String::from(key), val);
        if self.ops.contains(&i) {
            return;
        }

        self.ops.push(i);
    }

    /// A simple utility to add a DELETE operation
    ///
    /// Checks if the same operation already exists. Also checks
    /// if an insert operation for the same key already exists,
    /// removing it from the vector if it does.
    pub fn delete(&mut self, key: &str) {
        let d = Delete(String::from(key));

        /* Search for the insert key */
        let mut inserted = false;
        let mut ctr = 0;
        for op in &self.ops {
            match op {
                &Insert(ref k, _) => {
                    if key == k {
                        inserted = true;
                        break;
                    }
                }
                _ => {}
            }

            ctr += 1;
        }

        /* Remove the insert then */
        if inserted {
            self.ops.remove(ctr);
            return;
        }

        /* Search for existing deletion */
        if self.ops.contains(&d) {
            return;
        }

        /* It's safe to insert! */
        self.ops.push(d);
    }

    /// Take a version full of operations and flatten it to  a single
    /// binary search tree that can be included into an embedded record
    ///
    /// Non-mutable on the version itself
    pub fn flatten(&self) -> BTreeMap<String, Payload> {
        let mut map = BTreeMap::new();

        /* For all operations, process them in order */
        for op in &self.ops {

            /* Match the operation */
            match op {
                &Insert(ref key, ref payload) => {
                    map.insert(key.clone(), payload.clone());
                }
                &Delete(ref key) => {
                    map.remove(key);
                }
            }
        }

        /* Return the map */
        return map;
    }

    /// A utility function which merges two versions onto &self
    ///
    /// - If a key is present in `other`, `self.key` is overwritten
    /// - If a key is missing in `other`, `self.key` is deleted
    ///
    pub fn merge(&mut self, other: &Version) {
        for op in &other.ops {
            self.ops.push(op.clone());
        }
    }
}
