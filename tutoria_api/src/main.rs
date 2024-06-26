use crate::config::config;
use crate::error::Result;
use crate::manager::TutorIAManager;
use crate::web::middlewares::auth::mw_ctx_resolve;
use crate::web::routes;
use axum::{middleware, Router};
use tower::ServiceBuilder;
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use web::middlewares::auth::mw_ctx_require;
use web::middlewares::cors::mw_cors_accept;
use web::middlewares::response_map::mw_response_map;
use web::routes::server_static::server_dir;

pub mod _dev_utils;
mod manager;
mod config;
mod crypt;
mod ctx;
mod error;
mod model;
pub mod utils;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(fmt::Layer::default())
        .with(EnvFilter::from_default_env())
        .try_init()
        .expect("Erro to initialize tracing");

    // -- FOR DEV ONLY
    // _dev_utils::init_dev().await;

    let tutoria_manager = TutorIAManager::new().await?;

    let routes_alunos =
        routes::aluno::routes(tutoria_manager.clone()).route_layer(middleware::from_fn(mw_ctx_require));
    let routes_usuario = routes::usuario::routes(tutoria_manager.clone());
    let routes_professor = routes::professor::router(tutoria_manager.clone());
    let routes_materia =
        routes::materia::router(tutoria_manager.clone()).route_layer(middleware::from_fn(mw_ctx_require));
    let routes_tutor = routes::tutor::routes(tutoria_manager.clone())
        .route_layer(middleware::from_fn(mw_ctx_require));
    let routes_authenticate = routes::auth::routes(tutoria_manager.clone());
    let routes_jwt = routes::jwt::routes().route_layer(middleware::from_fn(mw_ctx_require));

    let mw_cors_accept = mw_cors_accept().await;

    let routes_all = Router::new()
        .merge(routes_alunos)
        .merge(routes_usuario)
        .merge(routes_professor)
        .merge(routes_materia)
        .merge(routes_tutor)
        .merge(routes_authenticate)
        .merge(routes_jwt)
        .layer(
            ServiceBuilder::new()
                .layer(mw_cors_accept)
                // .layer(CookieManagerLayer::new())
                .layer(middleware::from_fn_with_state(tutoria_manager.clone(), mw_ctx_resolve))
                .layer(middleware::map_response(mw_response_map)),
        )
        .fallback_service(server_dir());

    let port = &config().port;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    info!("->> {:<12} - {port}\n", "LISTENING");
    axum::serve(listener, routes_all).await.unwrap();

    Ok(())
}
