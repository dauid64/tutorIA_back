use std::sync::Arc;

use axum::{http::StatusCode, response::{IntoResponse, Response}};
use serde::Serialize;
use tracing::debug;
use derive_more::From;

use crate::model;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, From)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    Model(model::Error),
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

        // FIXME - Analisar se Arc é uma boa prática
        response.extensions_mut().insert(Arc::new(self));

        response
    }
}

impl std::error::Error for Error {}