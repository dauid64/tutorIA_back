use axum::http::header::{ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_ORIGIN};
use tower_http::cors::CorsLayer;

pub async fn mw_cors_accept() -> CorsLayer {
    let cors = CorsLayer::new()
        .allow_headers([ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_ALLOW_CREDENTIALS]);

    return cors
}