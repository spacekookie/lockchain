//! A plug and play http interface layer for various lockchain components
//!
//! The way this is done is by shimming a common REST interface (via actix-web) in
//! between common `lockchain-core` types and the `lockchain-client` library which
//! is a base wrapper around `reqwest` which uses this API.
//!
//! You can of course also use whatever other library, in whatever language you want
//! to access this API. Doing so via the *official* client gives you the ability to
//! negotiate version numbers and have more advanced error handling built-in.
//!
//! Ideally this shim-layer version should be the same as the `lockchain-core` it binds
//! against, however especially during development this won't always be the case.

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate actix_web as actix;
extern crate lockchain_core as lockchain;

use actix::{server, App};
use lockchain::traits::Body;

mod handlers;
mod model;

pub use model::CarrierMessage;

/// Starts a new lockchain server for a certain payload type
///
/// The payload type is defined by the generic parameter provided and can
/// either be just the encrypted message body, or the decrypted message
/// body which is available via the lockchain-crypto crate
pub fn start_server<T: 'static + Body>(iface: &str, port: &str) {
    server::new(|| {
        App::new()
            .resource("/vault", |r| r.f(handlers::create_vault))
            .resource("/vault/{vaultid}", |r| r.f(handlers::update_vault))
            .resource("/vault/{vaultid}", |r| r.f(handlers::delete_vault))
            .resource("/vault/{vaultid}/records/{recordid}", |r| {
                r.f(handlers::get_record::<T>)
            })
            .resource("/vault/{vaultid}/records", |r| {
                r.f(handlers::create_record)
            })
            .resource("/vault/{vaultid}/records/{recordid}", |r| {
                r.f(handlers::update_record)
            })
            .resource("/vault/{vaultid}/records/{recordid}", |r| {
                r.f(handlers::delete_record)
            })
            .resource("/authenticate", |r| r.f(handlers::authenticate))
            .resource("/deauthenticate", |r| r.f(handlers::deauthenticate))
    }).bind(format!("{}:{}", iface, port))
        .expect("Can not bind to port 8000")
        .run();
}
