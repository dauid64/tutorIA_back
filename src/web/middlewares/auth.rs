use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

use crate::model::usuario::{UsuarioBmc, UsuarioForAuth};
use crate::web::set_token_cookie;
use crate::{ctx::Ctx, model::ModelManager, crypt::jwt::decode_jwt, web::{error::{Result, Error}, AUTH_TOKEN}};

pub async fn mw_ctx_require (
    ctx: Result<Ctx>,
    req: Request<Body>,
    next: Next
) -> Result<Response>{
    debug!(" {:<12} - mw_ctx_require - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolve(
    mm: State<ModelManager>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    debug!(" {:<12} - mw_ctx_resolve", "MIDDLEWARE");

    let ctx_ext_result = _ctx_resolve(mm, &cookies).await;

    if ctx_ext_result.is_err() 
        && !matches!(ctx_ext_result, Err(CtxExtError::TokenNotInCookie))
    {
        cookies.remove(Cookie::from(AUTH_TOKEN))
    }

    req.extensions_mut().insert(ctx_ext_result);

    Ok(next.run(req).await)
} 

async fn _ctx_resolve(mm: State<ModelManager>, cookies: &Cookies) -> CtxExtResult {
    let token = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(CtxExtError::TokenNotInCookie)?;

    let claim = decode_jwt(token).map_err(|_| CtxExtError::TokenWrongFormat)?.claims;

    let user: UsuarioForAuth = UsuarioBmc::first_by_username( &mm, &claim.username)
        .await
        .map_err(|err| CtxExtError::ModelAccessError(err.to_string()))?
        .ok_or(CtxExtError::UserNotFound)?;

    set_token_cookie(cookies, &user.username)
        .map_err(|_| CtxExtError::CannotSetTokenCookie)?;

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
    TokenNotInCookie,
    TokenWrongFormat,
    ModelAccessError(String),
    UserNotFound,
    CannotSetTokenCookie,
    CtxCreateFail(String),
    CtxNotInRequestExt
}