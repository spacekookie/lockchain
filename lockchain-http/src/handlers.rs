//! Definition of the core lockchain API

use actix_web::{HttpRequest, Json, Responder, Result};
use lockchain::{
    traits::{Body, Vault}, Record,
};

use model::CarrierMessage;

use std::sync::{Arc, Mutex};

type HttpRequestState<T> = HttpRequest<Arc<Mutex<T>>>;

/// PUT /vault
/// 
/// Check the documentation for more information about how to provide payloads
pub fn create_vault<B: Body>(_req: HttpRequestState<impl Vault<B>>) -> impl Responder {
    format!("Unimplemented!")
}

/// POST /vault/{vault-id}
pub fn update_vault<B: Body>(_req: HttpRequestState<impl Vault<B>>) -> impl Responder {
    format!("Unimplemented!")
}

/// DELETE /vault/{vault-id}
pub fn delete_vault<B: Body>(_req: HttpRequestState<impl Vault<B>>) -> impl Responder {
    format!("Unimplemented!")
}

/// GET /vault/{vault-id}/records/{record-id}
pub fn get_record<B: Body>(
    _req: HttpRequestState<impl Vault<B>>,
) -> Result<Json<CarrierMessage<Record<B>>>> {
    unimplemented!()

    // Ok(Json(CarrierMessage {
    //     error: Ok(()),
    //     data: Some(Record::new("name", "category", vec!["test", "foo"])),
    // }))
}

/// PUT /vault/{vault-id}/records
pub fn create_record<B: Body>(_req: HttpRequestState<impl Vault<B>>) -> impl Responder {
    format!("Unimplemented!")
}

/// POST /vault/{vault-id}/records/{record-id}
pub fn update_record<B: Body>(_req: HttpRequestState<impl Vault<B>>) -> impl Responder {
    format!("Unimplemented!")
}

/// DELETE /vault/{vault-id}/records/{record-id}
pub fn delete_record<B: Body>(_req: HttpRequestState<impl Vault<B>>) -> impl Responder {
    format!("Unimplemented!")
}

/// PUT /authenticate
pub fn authenticate<B: Body>(_req: HttpRequestState<impl Vault<B>>) -> impl Responder {
    format!("Unimplemented!")
}

/// PUT /de-authenticate
pub fn deauthenticate<B: Body>(_req: HttpRequestState<impl Vault<B>>) -> impl Responder {
    format!("Unimplemented!")
}
