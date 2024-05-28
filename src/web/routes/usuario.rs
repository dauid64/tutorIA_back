use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::Cookies;
use tracing::debug;

use crate::crypt::{self, pwd};
use crate::ctx::Ctx;
use crate::model::usuario::{UsuarioBmc, UsuarioForCreate, UsuarioForLogin};
use crate::model::ModelManager;
use crate::web;
use crate::web::error::{Result, Error};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/login", post(api_login_handler))
        .route("/api/usuario", post(api_create_usuario_handler))
        .with_state(mm)
}

async fn api_login_handler(
    mm: State<ModelManager>,
    cookies: Cookies, 
    Json(payload): Json<LoginPayload>
) -> Result<Json<Value>> {
    debug!(" {:<12} - api_login_handler", "HANDLER");

    let LoginPayload {
        username,
        pwd: pwd_clear,
    }: LoginPayload = payload;
    let root_ctx = Ctx::root_ctx();

    let user: UsuarioForLogin = UsuarioBmc::first_by_username(&root_ctx, &mm, &username)
        .await?
        .ok_or(Error::LoginFailUsernameNotFound)?;
    let user_id = user.id;
    let Some(pwd) = user.pwd else {
        return Err(Error::LoginFailUserHasNoPwd { user_id })
    };

    pwd::validate_pwd(&pwd_clear, &pwd)
        .map_err(|_| Error::LoginFailPwdNotMatching { user_id })?;

    web::set_token_cookie(&cookies, &user.username)?;

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

async fn api_create_usuario_handler(
    mm: State<ModelManager>,
    Json(payload): Json<UsuarioForCreate>
) -> Result<Json<Value>> {
    debug!(" {:<12} - api_create_usuario_handler", "HANDLER");
    
    UsuarioBmc::validate(&payload).await?;

    let crypt_pwd = crypt::pwd::encrypt_pwd(&payload.pwd)?;
    
    let usuario_for_create = UsuarioForCreate{
        username: payload.username,
        pwd: crypt_pwd
    };

    let id = UsuarioBmc::create(&mm, usuario_for_create).await?;

    let body = Json(json!({
        "result": {
            "success": true,
            "id": id
        }
    }));

    Ok(body)
}