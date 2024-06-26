use tower_http::cors::CorsLayer;

pub async fn mw_cors_accept() -> CorsLayer {
    let cors = CorsLayer::new().allow_credentials(true);

    return cors;
}
