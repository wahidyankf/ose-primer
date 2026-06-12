use std::sync::Arc;

use anyhow::Context;
use crud_be_rust_axum::{app, config::Config, db::pool::create_pool, state::AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::from_env().context("Failed to load configuration")?;
    let pool = create_pool(&config.database_url)
        .await
        .context("Failed to create database pool")?;

    let state = Arc::new(AppState::new(pool, config.jwt_secret));
    let app = app::router(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .context("Failed to bind port")?;

    tracing::info!("Listening on port {}", config.port);
    axum::serve(listener, app).await.context("Server error")?;

    Ok(())
}
