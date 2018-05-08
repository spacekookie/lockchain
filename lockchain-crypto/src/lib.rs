//! A shim-layer crate for lockchain encryption
//! 
//! To get going with encrypted lockchain files, just initialise an
//! AesEngine type and start working with encrypted types provided by
//! some backend.

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate miscreant;

extern crate lockchain_core as lcc;


mod databody;
mod engine;
mod keys;
mod utils;
mod data;

pub use databody::*;
pub use engine::AesEngine;
