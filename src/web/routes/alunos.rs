use axum::{routing::post, Json, Router};
use serde_json::{json, Value};
use crate::web::error::Result;

pub fn routes() -> Router {
    Router::new()
        .route("/api/alunos", post(alunos_handler))
}

async fn alunos_handler() -> Result<Json<Value>> {
    let body = Json(json!({
        "result": {
            "success": true
        }}
    ));

    Ok(body)
}