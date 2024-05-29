use axum::routing::get;
use axum::{extract::State, routing::post, Json, Router};
use serde_json::{json, Value};
use crate::{model::ModelManager, web::error::Result};
use crate::model::aluno::{AlunoBmc, AlunoForCreate};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/aluno", post(api_create_aluno_handler))
        .route("/api/aluno", get(api_search_aluno_handler))
        .with_state(mm)
}

async fn api_create_aluno_handler(
    State(mm): State<ModelManager>,
    Json(payload): Json<AlunoForCreate>
) -> Result<Json<Value>> {
    AlunoBmc::validate(&payload).await?;

    let id = AlunoBmc::create(&mm, payload).await?;

    let body = Json(json!({
        "result": {
            "id": id
        }}
    ));

    Ok(body)
}

async fn api_search_aluno_handler(State(mm): State<ModelManager>) -> Result<Json<Value>> {
    let alunos = AlunoBmc::search_with_join_user(&mm).await?;
    
    let body_response = json!({
        "alunos": alunos
    });

    Ok(Json(body_response))
}