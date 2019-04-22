//! A shim-layer crate for lockchain encryption
//!
//! To get going with encrypted lockchain files, just initialise an
//! AesEngine type and start working with encrypted types provided by
//! some backend.

extern crate lockchain_core as lcc;

mod databody;
mod engine;
mod keyfold;

pub use crate::databody::DataBody;
pub use crate::engine::AesEngine;
pub use crate::keyfold::Keyfold;