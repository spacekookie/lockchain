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
//!
//! **Note**: API endpoint documentation can be found
//! [here](https://github.com/spacekookie/lockchain/tree/master/lockchain-http#api-reference)

#![feature(core_intrinsics)]

#[macro_use]
extern crate serde_derive;
extern crate env_logger;
extern crate serde;

extern crate actix_web;
extern crate lockchain_core as lockchain;

mod handlers;
pub mod models;
pub mod state;

use actix_web::{http, server, App};
use lockchain::traits::{Body, Vault};
use state::ApiState;
use std::error::Error;
use std::sync::{Arc, Mutex};

/// A simple rename of the long generic types that are returned for a new server
pub type HttpApi<V> = server::HttpServer<App<Arc<Mutex<V>>>>;

/// Create a new lockchain-http server for a vault state
///
/// Lifetime wise, vault needs to long as long as the server, which is returned to
/// call `run()` on. Make sure this is done in a thread, to not block your
/// mainapplication context
///
/// Additionally, provide a address bind and port string to bind to.
///
/// ## Example
///
/// ```norun
/// use lockchain_core::{traits::*, EncryptedBody};
/// use lockchain_http::create_server;
/// use lockchain_files::FileVault;
///
/// let server = create_server(
///     "localhost",
///     "8080",
///     FileVault::<EncryptedBody>::new("name", "some-location"),
/// ).run();
/// ```
pub fn create_server<B, V>(
    bind: &str,
    port: &str,
    state: ApiState<B, V>,
) -> Result<HttpApi<ApiState<B, V>>, Box<Error>>
where
    B: Body + 'static,
    V: Vault<B> + 'static,
{
    let state = Arc::new(Mutex::new(state));

    server::new(move || {
        vec![
            App::with_state(Arc::clone(&state))
                .resource("/vaults", |r| {
                    // Get existing vaults
                    r.method(http::Method::GET).with(handlers::get_vaults);
                    // Create new vault (if authorised)
                    r.method(http::Method::PUT).with(handlers::create_vault);
                    // Delete entire vault (if authorised)
                    r.method(http::Method::DELETE).with(handlers::delete_vault);
                })
                .resource("/vaults/scope", |r| {
                    // Bring an existing vault into scope (if authorised)
                    r.method(http::Method::PUT).with(handlers::scope_vault);
                    // Remove an existing vault from API scope (if authorised)
                    r.method(http::Method::DELETE).with(handlers::unscope_vault);
                })
                .resource("/vaults/{vaultid}", |r| {
                    // Update vault metadata (access rights, users, indices, etc)
                    r.method(http::Method::POST).with(handlers::update_vault)
                })
                .resource("/vaults/{vaultid}/records", |r| {
                    // Get the vault record index (omits records without access)
                    r.method(http::Method::GET).with(handlers::get_all_records);
                    // Create a new record (if authorised) in the vault
                    r.method(http::Method::PUT).with(handlers::create_record);
                })
                .resource("/vaults/{vaultid}/records/{recordid}", |r| {
                    // Get a specific record from a vault
                    r.method(http::Method::GET).with(handlers::get_record);
                    // Update a specific record
                    r.method(http::Method::POST).with(handlers::update_record);
                    // Delete a specific record from a vault
                    r.method(http::Method::DELETE).with(handlers::delete_record);
                })
                .resource("/vaults/{vaultid}/metadata", |r| {
                    // Get a metadata index of things available. To retrieve an item
                    // a vault registry token needs to be provided which is checked
                    r.method(http::Method::GET).with(handlers::get_all_metadata);
                    // Create a new metadata domain (requires admin rights)
                    r.method(http::Method::PUT).with(handlers::put_metadata);
                })
                .resource("/vaults/{vaultid}/metadata/{metaid}", |r| {
                    // Get a specific metadata field
                    r.method(http::Method::GET).with(handlers::get_metadata);
                    // Update a specific metadata field
                    r.method(http::Method::POST).with(handlers::update_metadata);
                })
                .resource("/vaults/{vaultid}/register", |r| {
                    // Register a new user in a vault (with a registry token)
                    r.method(http::Method::POST).with(handlers::vault_register)
                })
                .resource("/vaults/{vaultid}/login", |r| {
                    // Login as a user for this vault
                    r.method(http::Method::POST).with(handlers::vault_login)
                })
                .resource("/vaults/{vaultid}/logout", |r| {
                    // Logout, closing an existing session
                    r.method(http::Method::POST).with(handlers::vault_logout)
                })
                .resource("/users/login", |r| {
                    // Request a new auth token
                    r.method(http::Method::POST).with(handlers::login)
                })
                .resource("/users/logout", |r| {
                    // Hand-in active auth token
                    r.method(http::Method::POST).with(handlers::logout)
                })
                .resource("/api", |r| {
                    // Get information about this API endpoint
                    r.method(http::Method::GET).with(handlers::api_data);
                }),
        ]
    }).bind(format!("{}:{}", bind, port))
        .map_err(|e| e.into())
}
