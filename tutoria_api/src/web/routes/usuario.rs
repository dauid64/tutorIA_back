use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde_json::{json, Value};
use tracing::debug;

use crate::crypt;
use crate::model::usuario::{UsuarioBmc, UsuarioForCreate};
use crate::manager::TutorIAManager;
use crate::web::error::Result;

pub fn routes(tutoria_manager: TutorIAManager) -> Router {
    Router::new()
        .route("/api/usuario", post(api_create_usuario_handler))
        .with_state(tutoria_manager)
}

async fn api_create_usuario_handler(
    tutoria_manager: State<TutorIAManager>,
    Json(payload): Json<UsuarioForCreate>
) -> Result<Json<Value>> {
    debug!(" {:<12} - api_create_usuario_handler", "HANDLER");

    UsuarioBmc::validate(&payload).await?;

    let crypt_pwd = crypt::pwd::encrypt_pwd(&payload.pwd)?;
    
    let usuario_for_create = UsuarioForCreate{
        username: payload.username,
        pwd: crypt_pwd
    };

    let id = UsuarioBmc::create(&tutoria_manager, usuario_for_create).await?;

    let body = Json(json!({
        "result": {
            "id": id
        }
    }));

    Ok(body)
}