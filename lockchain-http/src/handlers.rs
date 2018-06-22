//! Definition of the core lockchain API

use actix_web::{HttpRequest, Json, Responder, Result};
use lockchain::{
    traits::{Body, Vault}, Record,
};

use model::*;
use state::ApiState;

use std::intrinsics;
use std::sync::{Arc, Mutex};

type HttpRequestState<T> = HttpRequest<Arc<Mutex<T>>>;

/// GET /vault
///
/// Check the documentation for more information about how to provide payloads
pub fn get_vaults<B, V>(req: HttpRequestState<ApiState<B, V>>) -> impl Responder
where
    B: Body,
    V: Vault<B>,
{
    let state = req.state().lock().unwrap();
    Json(VaultList {
        vaults: state.vaults().iter().map(|s| s.to_string()).collect(),
        count: state.count(),
    })
}

/// PUT /vault
///
/// Check the documentation for more information about how to provide payloads
pub fn create_vault<B, V>(
    (item, req): (Json<VaultCreate>, HttpRequestState<ApiState<B, V>>),
) -> impl Responder
where
    B: Body,
    V: Vault<B>,
{
    let mut state = req.state().lock().unwrap();
    let location = if state.bound_scope {
        state.working_dir.clone().join(&item.location)
    } else {
        (&item.location).into()
    };

    state.add_vault(&item.name, V::new(&item.name, location.to_str().unwrap()));
    Json(VaultCreateResponse {
        name: item.name.clone(),
        created: true,
        error: None,
    })
}

/// POST /vault/{vault-id}
pub fn update_vault<B, V>(_req: HttpRequestState<ApiState<B, V>>) -> impl Responder
where
    B: Body,
    V: Vault<B>,
{
    Json(OperationFailed {
        reason: "Not implemented".into(),
        code: 255,
    })
}

/// DELETE /vault/{vault-id}
pub fn delete_vault<B, V>(_req: HttpRequestState<ApiState<B, V>>) -> impl Responder
where
    B: Body,
    V: Vault<B>,
{
    Json(OperationFailed {
        reason: "Not implemented".into(),
        code: 255,
    })
}

/// GET /vault/{vault-id}/records/{record-id}
pub fn get_record<B, V>(
    (item, req): (Json<VaultCreate>, HttpRequestState<ApiState<B, V>>),
) -> impl Responder
where
    B: Body,
    V: Vault<B>,
{
    let mut state = req.state().lock().unwrap();
    let vault = state.get_vault("");

    Json(OperationFailed {
        reason: "Not implemented".into(),
        code: 255,
    })

    // Ok(Json(CarrierMessage {
    //     error: Ok(()),
    //     data: Some(Record::new("name", "category", vec!["test", "foo"])),
    // }))
}

/// PUT /vault/{vault-id}/records
pub fn create_record<B, V>(_req: HttpRequestState<ApiState<B, V>>) -> impl Responder
where
    B: Body,
    V: Vault<B>,
{
    Json(OperationFailed {
        reason: "Not implemented".into(),
        code: 255,
    })
}

/// POST /vault/{vault-id}/records/{record-id}
pub fn update_record<B, V>(_req: HttpRequestState<ApiState<B, V>>) -> impl Responder
where
    B: Body,
    V: Vault<B>,
{
    Json(OperationFailed {
        reason: "Not implemented".into(),
        code: 255,
    })
}

/// DELETE /vault/{vault-id}/records/{record-id}
pub fn delete_record<B, V>(_req: HttpRequestState<ApiState<B, V>>) -> impl Responder
where
    B: Body,
    V: Vault<B>,
{
    Json(OperationFailed {
        reason: "Not implemented".into(),
        code: 255,
    })
}

/// PUT /authenticate
pub fn authenticate<B, V>(_req: HttpRequestState<ApiState<B, V>>) -> impl Responder
where
    B: Body,
    V: Vault<B>,
{
    Json(OperationFailed {
        reason: "Not implemented".into(),
        code: 255,
    })
}

/// PUT /de-authenticate
pub fn deauthenticate<B, V>(_req: HttpRequestState<ApiState<B, V>>) -> impl Responder
where
    B: Body,
    V: Vault<B>,
{
    Json(OperationFailed {
        reason: "Not implemented".into(),
        code: 255,
    })
}

/// GET /api
///
/// Check the documentation for more information about how to provide payloads
pub fn api_data<B: Body, V: Vault<B>>(_: HttpRequestState<ApiState<B, V>>) -> impl Responder
where
    B: Body,
    V: Vault<B>,
{
    Json(ApiInformation {
        version: "1.0".into(),
        providers: vec![
            unsafe { intrinsics::type_name::<V>() }.into(),
            unsafe { intrinsics::type_name::<B>() }.into(),
        ],
        hostname: None,
        supported: "1.0".into(),
    })
}
