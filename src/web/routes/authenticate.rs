use axum::routing::post;
use axum::{extract::State, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::Cookies;
use tracing::debug;

use crate::crypt::pwd;
use crate::ctx::Ctx;
use crate::model::usuario::{UsuarioBmc, UsuarioForLogin};
use crate::model::ModelManager;
use crate::web::{self, remove_token_cookie};
use crate::web::error::{Result, Error};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/login", post(api_login_handler))
        .route("/api/logoff", post(api_logoff_handler))
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

    let user: UsuarioForLogin = UsuarioBmc::first_by_username(&mm, &username)
        .await?
        .ok_or(Error::LoginFailUsernameNotFound)?;
    let user_id = user.id;
    let Some(pwd) = user.pwd else {
        return Err(Error::LoginFailUserHasNoPwd{ user_id })
    };

    pwd::validate_pwd(&pwd_clear, &pwd).map_err(|_| Error::LoginFailPwdNotMatching { user_id })?;

    web::set_token_cookie(&cookies, &user.username, user_id)?;

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));


    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

async fn api_logoff_handler(
    cookies: Cookies,
    Json(payload): Json<LogoffPayload>
) -> Result<Json<Value>> {
    debug!(" {:<12} - api_logoff_handler", "HANDLER");
    let should_logoff = payload.logoff;

    if should_logoff {
        remove_token_cookie(&cookies)?;
    }

    let body = Json(json!({
        "result": {
            "logged_off": should_logoff
        }
    }));

    Ok(body)
}


#[derive(Debug, Deserialize)]
struct LogoffPayload {
    logoff: bool,
}
