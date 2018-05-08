//! A plug and play http interface layer for various lockchain components

extern crate gotham_serde_json_body_parser as goth_json;
extern crate gotham;

/// Contains API internal state and metadata
pub struct Server {

}

pub enum ApiFeature {
    BasicIo
}