//! A plug and play http interface layer for various lockchain components
#![feature(external_doc)]
#![doc(include = "../README.md")]
#![feature(non_modrs_mods)]

extern crate serde;

#[macro_use]
extern crate serde_derive;

extern crate actix_web as actix;
extern crate lockchain_core as lockchain;

use actix::{http::Method, server, App, HttpRequest, Json, Responder, Result};
use lockchain::{errors::Error as LockError, traits::Body, Record};
use serde::{Serialize, de::DeserializeOwned};

#[derive(Serialize)]
struct CarrierMessage<T: Serialize + DeserializeOwned> {
    error: Result<(), LockError>,
    data: Option<T>,
}

/// PUT /vault
fn create_vault(_req: HttpRequest) -> impl Responder {
    format!("Not Implemented!")
}

fn index<T: Body>(_req: HttpRequest) -> Result<Json<CarrierMessage<T>>> {
    unimplemented!()
}

/// POST /vault/{vault-id}
fn update_vault(_req: HttpRequest) -> impl Responder {
    format!("Not Implemented!")
}

/// DELETE /vault/{vault-id}
fn delete_vault(_req: HttpRequest) -> impl Responder {
    format!("Not Implemented!")
}

/// GET /vault/{vault-id}/records/{record-id}
fn get_record(_req: HttpRequest) -> impl Responder {
    format!("Not Implemented!")
}

/// PUT /vault/{vault-id}/records
fn create_record(_req: HttpRequest) -> impl Responder {
    format!("Not Implemented!")
}

/// POST /vault/{vault-id}/records/{record-id}
fn update_record(_req: HttpRequest) -> impl Responder {
    format!("Not Implemented!")
}

/// DELETE /vault/{vault-id}/records/{record-id}
fn delete_record(_req: HttpRequest) -> impl Responder {
    format!("Not Implemented!")
}

/// PUT /authenticate
fn authenticate(_req: HttpRequest) -> impl Responder {
    format!("Not Implemented!")
}

/// PUT /de-authenticate
fn deauthenticate(_req: HttpRequest) -> impl Responder {
    format!("Not Implemented!")
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/vault", |r| r.f(create_vault))
            .resource("/vault/{vault-id}", |r| r.f(update_vault))
            .resource("/vault/{vault-id}", |r| r.f(delete_vault))
            .resource("/vault/{vault-id}/records/{record-id}", |r| {
                r.f(get_record)
            })
            .resource("/vault/{vault-id}/records", |r| r.f(create_record))
            .resource("/vault/{vault-id}/records/{record-id}", |r| {
                r.f(update_record)
            })
            .resource("/vault/{vault-id}/records/{record-id}", |r| {
                r.f(delete_record)
            })
            .resource("/authenticate", |r| r.f(authenticate))
            .resource("/deauthenticate", |r| r.f(deauthenticate))
    }).bind("127.0.0.1:8000")
        .expect("Can not bind to port 8000")
        .run();
}

/// The REST API only have major/ incompatible versions
const API_VERSION: &'static str = "v1";

/// Contains API internal state and metadata
pub struct Server {}

impl Server {}

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
