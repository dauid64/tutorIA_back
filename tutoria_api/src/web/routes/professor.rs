use axum::{extract::State, routing::post, Json, Router};
use serde_json::{json, Value};
use crate::model::professor::ProfessorBmc;
use crate::web::error::Result;
use crate::model::{professor::ProfessorForCreate, ModelManager};

pub fn router(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/professor", post(api_create_professor_handler))
        .with_state(mm)
}

async fn api_create_professor_handler(
    State(mm): State<ModelManager>,
    Json(payload): Json<ProfessorForCreate>
) -> Result<Json<Value>> {

    let id = ProfessorBmc::create(&mm, payload).await?;

    let body = Json(json!({
        "result": {
            "id": id
        }
    }));

    Ok(body) 
}