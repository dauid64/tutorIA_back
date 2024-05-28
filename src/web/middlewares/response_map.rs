use axum::{http::{Method, Uri}, response::{IntoResponse, Response}, Json};
use serde_json::{json, to_value};
use tracing::debug;
use tracing_subscriber::field::debug;
use uuid::Uuid;

use crate::{ctx::Ctx, web};

pub async fn mw_response_map(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    debug!(" {:<12} - mw_reponse_map", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    let web_error = res.extensions().get::<web::error::Error>();
    let client_status_error = web_error.map(|se| se.client_status_and_error());

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error = to_value(client_error).ok();
            let message = client_error.as_ref().and_then(|v| v.get("message"));
            let detail = client_error.as_ref().and_then(|v| v.get("detail"));

            let client_error_body = json!({
                "error": {
                    "message": message,
                    "data": {
                        "req_uuid": uuid.to_string(),
                        "detail": detail
                    }
                }
            });
            
            debug!("CLIENT ERROR BODY:\n{client_error_body}");

            (*status_code, Json(client_error_body)).into_response()
        });

    debug!("\n");

    error_response.unwrap_or(res)
}