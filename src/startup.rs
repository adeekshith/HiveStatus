use crate::configuration::AppConfig;
use crate::routes::{get_config, get_statuses};
use axum::{
    routing::get,
    Router,
};
use reqwest::Client;
use std::net::SocketAddr;
use tokio::net::TcpListener; // Use tokio's TcpListener
use std::str::FromStr;
use tower_http::services::ServeDir;
use tracing_subscriber::{filter::Targets, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    pub config: AppConfig,
}

pub async fn run_with_listener(config: AppConfig, listener: TcpListener) -> SocketAddr {
    // --- Tracing/Logging ---
    let log_level = config.log_level;
    let filter = Targets::new()
        .with_default(log_level)
        .with_target("tower_http", log_level);
        
    tracing_subscriber::registry().with(tracing_subscriber::fmt::layer()).with(filter).try_init().ok(); // Use try_init to avoid panics in tests

    tracing::info!("Configuration loaded:");
    tracing::info!("-> Gatus URL: {}", config.gatus_url);
    tracing::info!("-> Page Title: {}", config.page_title);
    tracing::info!("-> Refresh Interval: {}ms", config.refresh_interval_ms);
    tracing::info!("-> Log Level: {}", config.log_level);
    
    // --- App State ---
    let state = AppState {
        client: Client::new(),
        config: config.clone(), // Clone config for AppState
    };

    // --- Router ---
    let static_files = ServeDir::new("static").append_index_html_on_directories(true);

    let app = Router::new()
        .route("/api/config", get(get_config))
        .route("/api/statuses", get(get_statuses))
        .fallback_service(static_files)
        .with_state(state);

    // --- Server ---
    let addr = listener.local_addr().unwrap();
    tracing::info!("Listening on http://{}", addr);
    
    axum::serve(listener, app)
        .await
        .expect("Failed to start Axum server");
    addr
}
