mod config;
mod routes;
mod state;

use std::sync::Arc;
use crate::routes::routes;

#[tokio::main]
async fn main() {
    let config = config::loader::load_config();
    let bind_address = format!("{}:{}", config.address, config.port);
    let config::Config { site_profile, drives, .. } = config;

    tracing_subscriber::fmt()
        .init();

    let state = Arc::new(state::AppState {
        site_profile,
    });

    let app = routes(state);

    let listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}