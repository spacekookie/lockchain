// //! A plug and play http interface layer for various lockchain components
// #![feature(external_doc)]
// #![doc(include = "../README.md")]
// #![feature(non_modrs_mods)]

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate actix_web as actix;
extern crate lockchain_core as lockchain;

use actix::{server, App};
use lockchain::traits::Body;

mod model;
pub use model::CarrierMessage;

mod handlers;

/// Starts a new lockchain server for a certain payload type
///
/// The payload type is defined by the generic parameter provided and can
/// either be just the encrypted message body, or the decrypted message
/// body which is available via the lockchain-crypto crate
pub fn start_server<T: 'static + Body>(iface: &str, port: &str) {
    server::new(|| {
        App::new()
            .resource("/vault", |r| r.f(handlers::create_vault))
            .resource("/vault/{vault-id}", |r| r.f(handlers::update_vault))
            .resource("/vault/{vault-id}", |r| r.f(handlers::delete_vault))
            .resource("/vault/{vault-id}/records/{record-id}", |r| {
                r.f(handlers::get_record::<T>)
            })
            .resource("/vault/{vault-id}/records", |r| {
                r.f(handlers::create_record)
            })
            .resource("/vault/{vault-id}/records/{record-id}", |r| {
                r.f(handlers::update_record)
            })
            .resource("/vault/{vault-id}/records/{record-id}", |r| {
                r.f(handlers::delete_record)
            })
            .resource("/authenticate", |r| r.f(handlers::authenticate))
            .resource("/deauthenticate", |r| r.f(handlers::deauthenticate))
    }).bind(format!("{}:{}", iface, port))
        .expect("Can not bind to port 8000")
        .run();
}
