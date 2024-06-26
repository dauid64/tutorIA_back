use axum::{extract::State, routing::post, Json, Router};
use serde_json::{json, Value};
use crate::manager::TutorIAManager;
use crate::model::professor::ProfessorBmc;
use crate::web::error::Result;
use crate::model::professor::ProfessorForCreate;

pub fn router(tutoria_manager: TutorIAManager) -> Router {
    Router::new()
        .route("/api/professor", post(api_create_professor_handler))
        .with_state(tutoria_manager)
}

async fn api_create_professor_handler(
    State(tutoria_manager): State<TutorIAManager>,
    Json(payload): Json<ProfessorForCreate>
) -> Result<Json<Value>> {

    let id = ProfessorBmc::create(&tutoria_manager, payload).await?;

    let body = Json(json!({
        "result": {
            "id": id
        }
    }));

    Ok(body) 
}