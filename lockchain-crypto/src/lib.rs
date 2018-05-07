//!

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate lockchain_core as lcc;

use lcc::{traits::{AutoEncoder, Body},
          Payload};
use std::collections::BTreeMap;

mod databody;
mod engine;

pub use databody::*;
pub use engine::*;
