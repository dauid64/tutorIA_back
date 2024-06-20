use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde_json::{json, Value};
use tutoria_agent::{create_tutoria_assistant, TutorIAContext};
use uuid::Uuid;

use crate::model::materia::MateriaBmc;
use crate::model::tutor::{TutorBmc, TutorForCreate};
use crate::model::ModelManager;
use crate::web::error::{Error, Result};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/tutor", post(api_create_tutor_handler))
        .with_state(mm)
}

async fn api_create_tutor_handler(
    State(mm): State<ModelManager>,
    Json(payload): Json<TutorForCreate>,
) -> Result<Json<Value>> {
    let materia = MateriaBmc::find_by_id(&mm, payload.materia_id).await?;
    let assistant_name = format!("{} {}", payload.nome, Uuid::new_v4());

    let assitant_ctx = TutorIAContext {
        materia: materia.nome,
    };

    let tutoria = create_tutoria_assistant(assistant_name, assitant_ctx)
        .await
        .map_err(|err| Error::TutorIAAgentError(err.to_string()))?;

    let tutor_c = TutorForCreate {
        nome: payload.nome,
        assistant_id: tutoria.assistant_id.as_str().to_string(),
        materia_id: payload.materia_id,
    };

    let tutor_id = TutorBmc::create(&mm, tutor_c).await?;

    let body = Json(json!({
        "result": {
            "id": tutor_id
        }
    }));

    Ok(body)
}
