//! A plug and play http interface layer for various lockchain components
#![feature(external_doc)]
#![doc(include = "../README.md")]
#![feature(non_modrs_mods)]


extern crate gotham_serde_json_body_parser as goth_json;
extern crate gotham;

/// Contains API internal state and metadata
pub struct Server {

}

impl Server {
    
}

/// An enum that represents optional features. At least
/// one flag needs to be provided to initialise [[Server]]
/// in order to make a working lockchain-http interface.
pub enum ApiFeature {
    /// Basic functionality for record I/O
    Base,
    /// Enables user access management
    Users,
    /// Allows management of user identities
    UserManagement,
    /// Allows management of filestorage scopes & loading
    VaultManagement,        
}