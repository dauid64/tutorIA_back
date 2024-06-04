use axum::{middleware, Router};
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use web::middlewares::auth::mw_ctx_require;
use web::middlewares::response_map::mw_response_map;
use web::routes::server_static::server_dir;
use crate::error::Result;
use crate::config::config;
use crate::model::ModelManager;
use crate::web::middlewares::auth::mw_ctx_resolve;
use crate::web::routes;

mod error;
mod config;
mod web;
mod model;
mod crypt;
mod ctx;
pub mod _dev_utils;
pub mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(fmt::Layer::default())
        .with(EnvFilter::from_default_env())
        .try_init()
        .expect("Erro to initialize tracing");

    // -- FOR DEV ONLY
    _dev_utils::init_dev().await;

    let mm = ModelManager::new().await?;

    let routes_alunos = routes::aluno::routes(mm.clone()).route_layer(middleware::from_fn(mw_ctx_require));
    let routes_usuario = routes::usuario::routes(mm.clone());
    let routes_professor = routes::professor::router(mm.clone()).route_layer(middleware::from_fn(mw_ctx_require));
    let routes_materia = routes::materia::router(mm.clone()).route_layer(middleware::from_fn(mw_ctx_require));
    let routes_authenticate = routes::authenticate::routes(mm.clone());

    let routes_all = Router::new()
        .merge(routes_alunos)
        .merge(routes_usuario)
        .merge(routes_professor)
        .merge(routes_materia)
        .merge(routes_authenticate)
        .layer(middleware::map_response(mw_response_map))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
        .layer(CookieManagerLayer::new())
        .fallback_service(server_dir());


    let port = &config().port;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    info!("->> {:<12} - {port}\n", "LISTENING");
    axum::serve(listener, routes_all).await.unwrap();

    Ok(())
}
