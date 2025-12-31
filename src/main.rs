use axum::{
    extract::State,
    response::{IntoResponse, Response, Json},
    routing::get,
    Router,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use tower_http::services::ServeDir;
use tracing::Level;
use tracing_subscriber::{filter::Targets, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone, Serialize, Deserialize)]
struct AppConfig {
    page_title: String,
    gatus_url: String,
    refresh_interval_ms: u64,
}

#[derive(Clone)]
struct AppState {
    client: Client,
    config: AppConfig,
}

#[tokio::main]
async fn main() {
    // --- Configuration ---
    let host = env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("APP_PORT").unwrap_or_else(|_| "3000".to_string());
    let log_level_str = env::var("APP_LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    
    let refresh_interval_ms = env::var("APP_REFRESH_INTERVAL_MS")
        .unwrap_or_else(|_| "60000".to_string())
        .parse::<u64>()
        .unwrap_or(60000);

    let app_config = AppConfig {
        gatus_url: env::var("APP_GATUS_URL")
            .unwrap_or_else(|_| "https://status.twin.sh".to_string()),
        page_title: env::var("APP_PAGE_TITLE").unwrap_or_else(|_| "HiveStatus".to_string()),
        refresh_interval_ms,
    };

    // --- Tracing/Logging (Initialized with config values) ---
    let log_level = Level::from_str(&log_level_str).unwrap_or(Level::INFO);
    let filter = Targets::new()
        .with_default(log_level)
        .with_target("tower_http", log_level);
        
    tracing_subscriber::registry().with(tracing_subscriber::fmt::layer()).with(filter).init();

    tracing::info!("Configuration loaded:");
    tracing::info!("-> Gatus URL: {}", app_config.gatus_url);
    tracing::info!("-> Page Title: {}", app_config.page_title);
    tracing::info!("-> Refresh Interval: {}ms", app_config.refresh_interval_ms);
    tracing::info!("-> Log Level: {}", log_level_str);
    
    // --- App State ---
    let state = AppState {
        client: Client::new(),
        config: app_config,
    };

    // --- Router ---
    let static_files = ServeDir::new("static").append_index_html_on_directories(true);

    let app = Router::new()
        .route("/api/config", get(get_config))
        .route("/api/statuses", get(get_statuses))
        .fallback_service(static_files)
        .with_state(state);

    // --- Server ---
    let addr = SocketAddr::new(
        IpAddr::from_str(&host).unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
        port.parse::<u16>().unwrap_or(3000),
    );
    tracing::info!("Listening on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_config(State(state): State<AppState>) -> Json<AppConfig> {
    Json(state.config)
}

async fn get_statuses(State(state): State<AppState>) -> Response {
    let url = format!("{}/api/v1/endpoints/statuses", state.config.gatus_url);
    
    match state.client.get(&url).send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<Value>().await {
                    Ok(data) => Json(data).into_response(),
                    Err(e) => {
                        tracing::error!("Failed to parse JSON from Gatus: {}", e);
                        (
                            axum::http::StatusCode::INTERNAL_SERVER_ERROR, 
                            "Failed to parse JSON from Gatus".to_string()
                        ).into_response()
                    }
                }
            } else {
                tracing::warn!("Received upstream error from Gatus: {}", resp.status());
                (
                    resp.status(),
                    format!("Upstream error: {}", resp.status())
                ).into_response()
            }
        },
        Err(e) => {
            tracing::error!("Failed to fetch from Gatus: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch from Gatus".to_string()
            ).into_response()
        }
    }
}