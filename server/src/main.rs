use axum::{Router, middleware::from_fn};
use shared::middlewares::{cors_middleware::get_cors_layer, logger_middleware::logger_middleware};
use tracing_subscriber::prelude::*;

mod config;
mod features;
mod shared;

use config::Config;
use features::auth::AuthHandlers;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env().expect("Failed to load configuration");
    let app = Router::new()
        .nest("/api", AuthHandlers::routes(config.clone()))
        .layer(from_fn(logger_middleware))
        .layer(get_cors_layer());

    let server_address = config.server_address();
    let listener = tokio::net::TcpListener::bind(&server_address)
        .await
        .unwrap();

    info!("Server listening on {}", server_address);
    axum::serve(listener, app).await.unwrap();
}
