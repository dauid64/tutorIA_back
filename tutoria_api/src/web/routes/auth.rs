use axum::routing::{post};
use axum::{extract::State, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;

use crate::crypt::{jwt, pwd};
use crate::manager::TutorIAManager;
use crate::model::usuario::{UsuarioBmc, UsuarioForLogin};
use crate::web::error::{Error, Result};

pub fn routes(tutoria_manager: TutorIAManager) -> Router {
    Router::new()
        .route("/api/login", post(api_login_handler))
        .with_state(tutoria_manager)
}

async fn api_login_handler(
    tutoria_manager: State<TutorIAManager>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
    debug!(" {:<12} - api_login_handler", "HANDLER");

    let LoginPayload {
        username,
        pwd: pwd_clear,
    }: LoginPayload = payload;

    let user: UsuarioForLogin = UsuarioBmc::first_by_username(&tutoria_manager, &username)
        .await?
        .ok_or(Error::LoginFailUsernameNotFound)?;
    let user_id = user.id;
    let Some(pwd) = user.pwd else {
        return Err(Error::LoginFailUserHasNoPwd { user_id });
    };

    pwd::validate_pwd(&pwd_clear, &pwd).map_err(|_| Error::LoginFailPwdNotMatching { user_id })?;

    let jwt = jwt::encode_jwt(&user.username, user_id)?;

    let body = Json(json!({
        "result": {
            "success": true,
            "jwt": jwt,
            "user_username": user.username,
            "user_id": user_id
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}