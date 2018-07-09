//! Definition of the core lockchain API

use actix_web::{HttpRequest, Json, Responder};
use lockchain::{
    traits::{Body, Vault}, Record,
};

use models::{inputs::*, responses::*, NoneError, Response};
use state::ApiState;

use std::intrinsics;
use std::sync::{Arc, Mutex};

type HttpRequestState<T> = HttpRequest<Arc<Mutex<T>>>;

// /// GET /vault
// ///
// /// Check the documentation for more information about how to provide payloads
// pub fn get_vaults<B, V>(req: HttpRequestState<ApiState<B, V>>) -> impl Responder
// where
//     B: Body,
//     V: Vault<B>,
// {
//     let state = req.state().lock().unwrap();
//     Json(VaultList {
//         vaults: state.vaults().iter().map(|s| s.to_string()).collect(),
//         count: state.count(),
//     })
// }

// /// PUT /vault
// ///
// /// Check the documentation for more information about how to provide payloads
// pub fn create_vault<B, V>(
//     (item, req): (Json<VaultCreate>, HttpRequestState<ApiState<B, V>>),
// ) -> impl Responder
// where
//     B: Body,
//     V: Vault<B>,
// {
//     let mut state = req.state().lock().unwrap();
//     let location = if state.bound_scope {
//         state.working_dir.clone().join(&item.location)
//     } else {
//         (&item.location).into()
//     };

//     state.add_vault(&item.name, V::new(&item.name, location.to_str().unwrap()));
//     Json(Response::Success::<NoneError>)
// }

// pub fn delete_vault<B, V>(
//     (item, req): (Json<VaultCreate>, HttpRequestState<ApiState<B, V>>),
// ) -> impl Responder
// where
//     B: Body,
//     V: Vault<B>,
// {
//     use lockchain::errors::Error as LockError;

//     Json(OperationFailed::<LockError> {
//         reason: "Not implemented".into(),
//         error: LockError::UnknownFailure.into(),
//     })
// }

// pub fn scope_vault<B, V>(
//     (item, req): (Json<VaultCreate>, HttpRequestState<ApiState<B, V>>),
// ) -> impl Responder
// where
//     B: Body,
//     V: Vault<B>,
// {
//     use lockchain::errors::Error as LockError;

//     Json(OperationFailed::<LockError> {
//         reason: "Not implemented".into(),
//         error: LockError::UnknownFailure.into(),
//     })
// }

// pub fn unscope_vault<B, V>(
//     (item, req): (Json<VaultCreate>, HttpRequestState<ApiState<B, V>>),
// ) -> impl Responder
// where
//     B: Body,
//     V: Vault<B>,
// {
//     use lockchain::errors::Error as LockError;

//     Json(Response::Failure::<LockError>(OperationFailed {
//         reason: "Not implemented".into(),
//         error: LockError::UnknownFailure.into(),
//     }))

//     // Json()
// }

// /// POST /vault/{vault-id}
// pub fn update_vault<B, V>(_req: HttpRequestState<ApiState<B, V>>) -> impl Responder
// where
//     B: Body,
//     V: Vault<B>,
// {
//     use lockchain::errors::Error as LockError;

//     Json(OperationFailed::<LockError> {
//         reason: "Not implemented".into(),
//         error: LockError::UnknownFailure.into(),
//     })
// }

// pub fn get_all_records<B, V>(_req: HttpRequestState<ApiState<B, V>>) -> impl Responder
// where
//     B: Body,
//     V: Vault<B>,
// {
//     use lockchain::errors::Error as LockError;

//     Json(OperationFailed::<LockError> {
//         reason: "Not implemented".into(),
//         error: LockError::UnknownFailure.into(),
//     })
// }

// /// PUT /vault/{vault-id}/records
// pub fn create_record<B, V>(_req: HttpRequestState<ApiState<B, V>>) -> impl Responder
// where
//     B: Body,
//     V: Vault<B>,
// {
//     use lockchain::errors::Error as LockError;

//     Json(OperationFailed::<LockError> {
//         reason: "Not implemented".into(),
//         error: LockError::UnknownFailure.into(),
//     })
// }

// /// GET /vault/{vault-id}/records/{record-id}
// pub fn get_record<B, V>(
//     (item, req): (Json<VaultCreate>, HttpRequestState<ApiState<B, V>>),
// ) -> impl Responder
// where
//     B: Body,
//     V: Vault<B>,
// {
//     let mut state = req.state().lock().unwrap();
//     let vault = state.get_vault("");

//     use lockchain::errors::Error as LockError;

//     Json(OperationFailed::<LockError> {
//         reason: "Not implemented".into(),
//         error: LockError::UnknownFailure.into(),
//     })
// }

// /// POST /vault/{vault-id}/records/{record-id}
// pub fn update_record<B, V>(_req: HttpRequestState<ApiState<B, V>>) -> impl Responder
// where
//     B: Body,
//     V: Vault<B>,
// {
//     use lockchain::errors::Error as LockError;

//     Json(OperationFailed::<LockError> {
//         reason: "Not implemented".into(),
//         error: LockError::UnknownFailure.into(),
//     })
// }

// /// DELETE /vault/{vault-id}/records/{record-id}
// pub fn delete_record<B, V>(_req: HttpRequestState<ApiState<B, V>>) -> impl Responder
// where
//     B: Body,
//     V: Vault<B>,
// {
//     use lockchain::errors::Error as LockError;

//     Json(OperationFailed::<LockError> {
//         reason: "Not implemented".into(),
//         error: LockError::UnknownFailure.into(),
//     })
// }

#[allow(dead_code)]
pub fn foo<B, V>(_req: HttpRequestState<ApiState<B, V>>) -> impl Responder
where
    B: Body,
    V: Vault<B>,
{
    use lockchain::errors::Error as LockError;
    use models::{NoneError, Response};
    let a = 5;

    match a {
        5 => Json(Response::Failure::<LockError>(OperationFailed {
            explain: "BOOOOM!".into(),
            error: LockError::FailedSelfTest,
        })),
        _ => Json(Response::Success::<LockError>),
    }

    // } else {
    // Json(Response::Success::<NoneError>)
    // }
}

// /// PUT /authenticate
// pub fn authenticate<B, V>(
//     (item, req): (Json<Authenticate>, HttpRequestState<ApiState<B, V>>),
// ) -> impl Responder
// where
//     B: Body,
//     V: Vault<B>,
// {
//     use lockchain::users::*;
//     let Authenticate { username, password } = item.into_inner();

//     Json(match pam_authenticate(&username, &password) {
//         Ok(()) => {
//             /* Store the token for auth later */
//             let state = req.state().lock().unwrap();
//             let token = String::new();
//             state.tokens.insert(token.clone());

//             Response::Token(TokenMessage {
//                 username,
//                 token: token,
//             })
//         }
//         Err(e) => Response::Failure(OperationFailed {
//             reason: "Failed to authenticate user".into(),
//             error: e.into(),
//         }),
//     })
// }

// /// PUT /de-authenticate
// pub fn deauthenticate<B, V>(
//     (item, req): (Json<Deauthenticate>, HttpRequestState<ApiState<B, V>>),
// ) -> impl Responder
// where
//     B: Body,
//     V: Vault<B>,
// {
//     use lockchain::errors::Error as LockError;

//     Json(OperationFailed::<LockError> {
//         reason: "Not implemented".into(),
//         error: LockError::UnknownFailure.into(),
//     })
// }

// /// GET /api
// ///
// /// Check the documentation for more information about how to provide payloads
// pub fn api_data<B: Body, V: Vault<B>>(_: HttpRequestState<ApiState<B, V>>) -> impl Responder
// where
//     B: Body,
//     V: Vault<B>,
// {
//     Json(ApiInformation {
//         version: "1.0".into(),
//         providers: vec![
//             unsafe { intrinsics::type_name::<V>() }.into(),
//             unsafe { intrinsics::type_name::<B>() }.into(),
//         ],
//         hostname: None,
//         supported: "1.0".into(),
//     })
// }
