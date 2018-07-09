//! Definition of the core lockchain API

use actix_web::{HttpRequest, Json, Responder};

use lockchain::errors::{Error as LockError, *};
use lockchain::traits::{Body, Vault};
use lockchain::Record;

use models::{inputs::*, responses::*, Response};
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
    Json(Response::Success)
}

pub fn delete_vault<B, V>(
    (item, req): (Json<VaultCreate>, HttpRequestState<ApiState<B, V>>),
) -> impl Responder
where
    B: Body,
    V: Vault<B>,
{
    Json(OperationFailed {
        explain: "Not implemented".into(),
        error: LockError::Unknown,
    })
}

pub fn scope_vault<B, V>(
    (item, req): (Json<VaultCreate>, HttpRequestState<ApiState<B, V>>),
) -> impl Responder
where
    B: Body,
    V: Vault<B>,
{
    Json(OperationFailed {
        explain: "Not implemented".into(),
        error: LockError::Unknown,
    })
}

pub fn unscope_vault<B, V>(
    (item, req): (Json<VaultCreate>, HttpRequestState<ApiState<B, V>>),
) -> impl Responder
where
    B: Body,
    V: Vault<B>,
{
    Json(OperationFailed {
        explain: "Not implemented".into(),
        error: LockError::Unknown,
    })
}

/// POST /vault/{vault-id}
pub fn update_vault<B, V>(_req: HttpRequestState<ApiState<B, V>>) -> impl Responder
where
    B: Body,
    V: Vault<B>,
{
    Json(OperationFailed {
        explain: "Not implemented".into(),
        error: LockError::Unknown,
    })
}

pub fn get_all_records<B, V>(_req: HttpRequestState<ApiState<B, V>>) -> impl Responder
where
    B: Body,
    V: Vault<B>,
{
    Json(OperationFailed {
        explain: "Not implemented".into(),
        error: LockError::Unknown,
    })
}

/// PUT /vault/{vault-id}/records
pub fn create_record<B, V>(_req: HttpRequestState<ApiState<B, V>>) -> impl Responder
where
    B: Body,
    V: Vault<B>,
{
    Json(OperationFailed {
        explain: "Not implemented".into(),
        error: LockError::Unknown,
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
        explain: "Not implemented".into(),
        error: LockError::Unknown,
    })
}

/// POST /vault/{vault-id}/records/{record-id}
pub fn update_record<B, V>(_req: HttpRequestState<ApiState<B, V>>) -> impl Responder
where
    B: Body,
    V: Vault<B>,
{
    Json(OperationFailed {
        explain: "Not implemented".into(),
        error: LockError::Unknown,
    })
}

/// DELETE /vault/{vault-id}/records/{record-id}
pub fn delete_record<B, V>(_req: HttpRequestState<ApiState<B, V>>) -> impl Responder
where
    B: Body,
    V: Vault<B>,
{
    Json(OperationFailed {
        explain: "Not implemented".into(),
        error: LockError::Unknown,
    })
}

/// PUT /authenticate
pub fn authenticate<B, V>(
    (item, req): (Json<Authenticate>, HttpRequestState<ApiState<B, V>>),
) -> impl Responder
where
    B: Body,
    V: Vault<B>,
{
    use lockchain::users::*;
    let Authenticate { username, password } = item.into_inner();

    Json(match pam_authenticate(&username, &password) {
        Ok(()) => {
            /* Store the token for auth later */
            let mut state = req.state().lock().unwrap();
            let token = String::new();
            state.tokens.insert(token.clone());

            Response::Token(TokenMessage {
                username,
                token: token,
            })
        }
        Err(e) => Response::Failure(OperationFailed {
            explain: "Failed to authenticate user".into(),
            error: LockError::Auth(AuthError::UserNotAuthorised),
        }),
    })
}

/// PUT /de-authenticate
pub fn deauthenticate<B, V>(
    (item, req): (Json<Deauthenticate>, HttpRequestState<ApiState<B, V>>),
) -> impl Responder
where
    B: Body,
    V: Vault<B>,
{
    Json(OperationFailed {
        explain: "Not implemented".into(),
        error: LockError::Unknown,
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
