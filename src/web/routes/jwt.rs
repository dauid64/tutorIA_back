use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use crate::web::error::Result;

pub fn routes() -> Router {
    Router::new()
        .route("/api/validate", get(api_validate_jwt))
}

// Essa função servirá somente para o frontend chamar ela e checar se o jwt está valido
async fn api_validate_jwt() -> Result<Json<Value>> {
    let body = Json(json!({
        "result": {
            "success": true,
            "is_valid": true
        }
    }));

    Ok(body)
}