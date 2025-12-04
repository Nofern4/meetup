use std::{net::SocketAddr, sync::Arc, time::Duration};

use anyhow::{Ok, Result};
use axum::{
    Router,
    routing::get,
    http::{Method, StatusCode, header::{AUTHORIZATION, CONTENT_TYPE}},
    response::Html,
};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    services::{ServeDir, ServeFile},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::info;

use crate::{
    config::config_model::DotEnvyConfig,
    infrastructure::database::postgresql_connection::PgPoolSquad,
};

fn static_serve() -> Router {
    let static_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("static");

    let service = ServeDir::new(&static_dir)
        .not_found_service(ServeFile::new(static_dir.join("index.html")));

    Router::new().fallback_service(service)
}

fn api_serve() -> Router {
    Router::new().fallback(|| async { (StatusCode::NOT_FOUND, "API not found") })
}

async fn hello_world() -> Html<&'static str> {
    Html("<h1>Hello ja</h1>")
}

pub async fn start(config: Arc<DotEnvyConfig>, _db_pool: Arc<PgPoolSquad>) -> Result<()> {
    let app = Router::new()
        .route("/", get(hello_world))     // ← This makes "/" show "Hello World"

        .merge(static_serve())
        .nest("/api", api_serve())

        .layer(TimeoutLayer::new(Duration::from_secs(config.server.timeout)))
        .layer(RequestBodyLimitLayer::new(
            (config.server.body_limit * 1024 * 1024).try_into()?,
        ))
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                    Method::OPTIONS,
                ])
                .allow_origin(Any)
                .allow_headers([AUTHORIZATION, CONTENT_TYPE]),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    let listener = TcpListener::bind(addr).await?;

    info!("Server start on port {}", config.server.port);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async { tokio::signal::ctrl_c().await.expect("Fail ctrl + c") };
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Receive ctrl + c signal"),
        _ = terminate => info!("Receive terminate signal"),
    }
}
