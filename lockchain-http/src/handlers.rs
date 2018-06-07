//! Definition of the core lockchain API

use actix::{HttpRequest, Json, Responder, Result};
use lockchain::{traits::Body, Record};

use model::CarrierMessage;

/// PUT /vault
pub fn create_vault(_req: HttpRequest) -> impl Responder {
    format!("Unimplemented!")
}

/// POST /vault/{vault-id}
pub fn update_vault(_req: HttpRequest) -> impl Responder {
    format!("Unimplemented!")
}

/// DELETE /vault/{vault-id}
pub fn delete_vault(_req: HttpRequest) -> impl Responder {
    format!("Unimplemented!")
}

/// GET /vault/{vault-id}/records/{record-id}
pub fn get_record<T: Body>(_req: HttpRequest) -> Result<Json<CarrierMessage<Record<T>>>> {
    unimplemented!()
    // Ok(Json(CarrierMessage {
    //     error: Ok(()),
    //     data: Some(Record::new("name", "category", vec!["test", "foo"])),
    // }))
}

/// PUT /vault/{vault-id}/records
pub fn create_record(_req: HttpRequest) -> impl Responder {
    format!("Unimplemented!")
}

/// POST /vault/{vault-id}/records/{record-id}
pub fn update_record(_req: HttpRequest) -> impl Responder {
    format!("Unimplemented!")
}

/// DELETE /vault/{vault-id}/records/{record-id}
pub fn delete_record(_req: HttpRequest) -> impl Responder {
    format!("Unimplemented!")
}

/// PUT /authenticate
pub fn authenticate(_req: HttpRequest) -> impl Responder {
    format!("Unimplemented!")
}

/// PUT /de-authenticate
pub fn deauthenticate(_req: HttpRequest) -> impl Responder {
    format!("Unimplemented!")
}
