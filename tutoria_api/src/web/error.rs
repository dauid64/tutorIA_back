use axum::{http::StatusCode, response::{IntoResponse, Response}};
use serde::Serialize;
use tracing::debug;
use derive_more::From;
use uuid::Uuid;

use crate::{crypt, model, web};

use super::middlewares;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, From, Clone)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    Model(model::Error),
    Utils(crypt::Error),
    CtxExt(middlewares::auth::CtxExtError),
    LoginFailUsernameNotFound,
    ParamsNotFound,
    #[from(ignore)]
    LoginFailUserHasNoPwd{ user_id: Uuid },
    #[from(ignore)]
    LoginFailPwdNotMatching { user_id: Uuid },
    #[from(ignore)]
    Router(&'static str),
    #[from(ignore)]
    Unauthorized(&'static str),
    #[from(ignore)]
    InvalidUuid(String),
    #[from(ignore)]
    TutorIAAgentError(String)
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!(" {:<12} - model::Error {self:?}", "INTO_RES");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        response
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        use web::error::Error::*;

        match self {
            LoginFailUsernameNotFound
            | LoginFailUserHasNoPwd { .. }
            | LoginFailPwdNotMatching { .. } => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            }
            Unauthorized(err) => {
                (StatusCode::FORBIDDEN, ClientError::INVALID_DATA(err))
            }
            CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
            Model(model::Error::ValidateFail(err)) => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_DATA(err))
            }
            Model(model::Error::EntityNotFound { entity, id }) => {
                (StatusCode::BAD_REQUEST, ClientError::ENTITY_NOT_FOUND { entity, id: *id })
            }
            Router(err) => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_DATA(err))
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
    NO_AUTH,
    SERVICE_ERROR,
    ENTITY_NOT_FOUND { entity: &'static str, id: Uuid},
    INVALID_DATA(&'static str)
}