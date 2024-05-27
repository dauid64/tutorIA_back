use axum::{middleware, Router};
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
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

    let routes_alunos = routes::aluno::routes(mm.clone());
    let routes_usuario = routes::usuario::routes(mm.clone());

    let routes_all = Router::new()
        .merge(routes_alunos)
        .merge(routes_usuario)
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
        .layer(CookieManagerLayer::new());


    let port = &config().port;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    info!("->> {:<12} - {port}\n", "LISTENING");
    axum::serve(listener, routes_all).await.unwrap();

    Ok(())
}
