use crate::manager::TutorIAManager;
use crate::model::aluno::{AlunoBmc, AlunoForCreate};
use crate::web::error::Result;
use axum::routing::get;
use axum::{extract::State, routing::post, Json, Router};
use serde_json::{json, Value};

pub fn routes(tutoria_manager: TutorIAManager) -> Router {
    Router::new()
        .route("/api/aluno", get(api_find_aluno_handler))
        .route("/api/aluno", post(api_create_aluno_handler))
        .with_state(tutoria_manager.clone())
}

async fn api_create_aluno_handler(
    tutoria_manager: State<TutorIAManager>,
    Json(payload): Json<AlunoForCreate>,
) -> Result<Json<Value>> {
    AlunoBmc::validate(&payload).await?;

    let id = AlunoBmc::create(&tutoria_manager, payload).await?;

    let body = Json(json!({
        "result": {
            "id": id
        }}
    ));

    Ok(body)
}

async fn api_find_aluno_handler(State(tutoria_manager): State<TutorIAManager>) -> Result<Json<Value>> {
    let alunos = AlunoBmc::search_with_join_user(&tutoria_manager).await?;

    let body_response = json!({
        "result": {
            "alunos": alunos
        }
    });

    Ok(Json(body_response))
}
