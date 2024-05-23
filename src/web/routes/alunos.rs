use axum::{extract::State, routing::post, Json, Router};
use serde_json::{json, Value};
use crate::{model::ModelManager, web::error::Result};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/alunos", post(alunos_handler))
        .with_state(mm)
}

async fn alunos_handler(
    State(mm): State<ModelManager>
) -> Result<Json<Value>> {
    let body = Json(json!({
        "result": {
            "success": true
        }}
    ));

    Ok(body)
}