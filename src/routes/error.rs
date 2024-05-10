use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use derive_more::From;
use serde::Serialize;
use serde_with::serde_as;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From, strum_macros::AsRefStr, Clone)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    DatabaseError,
    CredentialNotMatch,
    IdAlreadyUsed,
    HashFail,
    NotFound,
    Unauthenticated,
    Unauthorized,
}

impl Error {
    pub fn to_client(&self) -> (StatusCode, &str) {
        match self {
            Error::DatabaseError | Error::HashFail => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            Error::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            Error::CredentialNotMatch => (StatusCode::UNAUTHORIZED, "Credential not match"),
            Error::IdAlreadyUsed => (StatusCode::BAD_REQUEST, "Id already exist"),
            Error::Unauthorized | Error::Unauthenticated => {
                (StatusCode::UNAUTHORIZED, "Unauthorized")
            }
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // Create a placeholder Axum reponse.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the reponse.
        response.extensions_mut().insert(Arc::new(self));

        response
    }
}
