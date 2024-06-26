use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use axum_extra::{headers, TypedHeader};
use serde::Serialize;
use tracing::debug;

use crate::manager::TutorIAManager;
use crate::model::usuario::{UsuarioBmc, UsuarioForAuth};
use crate::{
    crypt::jwt::decode_jwt,
    ctx::Ctx,
    web::error::{Error, Result},
};

pub async fn mw_ctx_require(ctx: Result<Ctx>, req: Request<Body>, next: Next) -> Result<Response> {
    debug!(" {:<12} - mw_ctx_require - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolve(
    tutoria_manager: State<TutorIAManager>,
    authorization: Option<TypedHeader<headers::Authorization<headers::authorization::Bearer>>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    debug!(" {:<12} - mw_ctx_resolve", "MIDDLEWARE");
    if let Some(TypedHeader(authorization)) = authorization {
        let token = authorization.token();

        let ctx_ext_result = _ctx_resolve(tutoria_manager, token).await;
    
        req.extensions_mut().insert(ctx_ext_result);
    } else {
        let ctx_ext_result = _ctx_resolve(tutoria_manager, "").await;

        req.extensions_mut().insert(ctx_ext_result);
    };

    Ok(next.run(req).await)
}

async fn _ctx_resolve(
    tutoria_manager: State<TutorIAManager>,
    token: &str,
) -> CtxExtResult {
    if token.is_empty() {
        return Err(CtxExtError::TokenNotInHeader)
    }

    let claim = decode_jwt(token.to_string())
        .map_err(|_| CtxExtError::TokenWrongFormat)?
        .claims;

    let user: UsuarioForAuth = UsuarioBmc::first_by_username(&tutoria_manager, &claim.username)
        .await
        .map_err(|err| CtxExtError::ModelAccessError(err.to_string()))?
        .ok_or(CtxExtError::UserNotFound)?;

    Ctx::new(user.id).map_err(|err| CtxExtError::CtxCreateFail(err.to_string()))
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!(" {:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<CtxExtResult>()
            .ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
            .clone()
            .map_err(Error::CtxExt)
    }
}

type CtxExtResult = core::result::Result<Ctx, CtxExtError>;

#[derive(Clone, Serialize, Debug)]
pub enum CtxExtError {
    TokenNotInHeader,
    TokenWrongFormat,
    ModelAccessError(String),
    UserNotFound,
    CtxCreateFail(String),
    CtxNotInRequestExt,
}
