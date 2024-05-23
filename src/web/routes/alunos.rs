use axum::{extract::State, routing::post, Json, Router};
use serde_json::{json, Value};
use crate::{model::ModelManager, web::error::Result};
use crate::model::aluno::{AlunoBmc, AlunoForCreate};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/alunos", post(alunos_handler))
        .with_state(mm)
}

async fn alunos_handler(
    State(mm): State<ModelManager>,
    Json(payload): Json<AlunoForCreate>
) -> Result<Json<Value>> {
    let id = AlunoBmc::create(&mm, payload).await?;

    let body = Json(json!({
        "result": {
            "id": id
        }}
    ));

    Ok(body)
}