// //! A plug and play http interface layer for various lockchain components
// //!
// //! The way this is done is by shimming a common REST interface (via actix-web) in
// //! between common `lockchain-core` types and the `lockchain-client` library which
// //! is a base wrapper around `reqwest` which uses this API.
// //!
// //! You can of course also use whatever other library, in whatever language you want
// //! to access this API. Doing so via the *official* client gives you the ability to
// //! negotiate version numbers and have more advanced error handling built-in.
// //!
// //! Ideally this shim-layer version should be the same as the `lockchain-core` it binds
// //! against, however especially during development this won't always be the case.

#[macro_use]
extern crate serde_derive;
extern crate env_logger;
extern crate serde;

extern crate actix_web;
extern crate lockchain_core as lockchain;

mod handlers;
mod model;
// pub use model::CarrierMessage;


use actix_web::{server, App};
use lockchain::traits::{Body, Vault};
use std::sync::{Arc, Mutex};

/// Starts a new lockchain server for a certain payload type
///
/// The payload type is defined by the generic parameter provided and can
/// either be just the encrypted message body, or the decrypted message
/// body which is available via the lockchain-crypto crate
/// 
/// Create a new lockchain-http server for a vault state
/// 
/// 
pub fn create_server<B: Body + 'static>(
    state: impl Vault<B> + 'static,
) -> server::HttpServer<App<Arc<Mutex<impl Vault<B> + 'static>>>> {
    let state = Arc::new(Mutex::new(state));

    server::new(move || {
        vec![App::with_state(Arc::clone(&state))
        
            .resource("/vault", |r| r.f(handlers::create_vault))
            .resource("/vault/{vaultid}", |r| r.f(handlers::update_vault))
            .resource("/vault/{vaultid}", |r| r.f(handlers::delete_vault))
            .resource("/vault/{vaultid}/records/{recordid}", |r| { r.f(handlers::get_record) })
            .resource("/vault/{vaultid}/records", |r| r.f(handlers::create_record))
            .resource("/vault/{vaultid}/records/{recordid}", |r| { r.f(handlers::update_record) })
            .resource("/vault/{vaultid}/records/{recordid}", |r| { r.f(handlers::delete_record) })
            .resource("/authenticate", |r| r.f(handlers::authenticate))
            .resource("/deauthenticate", |r| r.f(handlers::deauthenticate))
        
        ]
    }).bind("localhost:8080")
        .expect("Oh no!")
}
