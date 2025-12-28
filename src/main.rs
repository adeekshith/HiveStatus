use axum::{
    extract::State,
    response::{IntoResponse, Response, Json},
    routing::get,
    Router,
};
use reqwest::Client;
use serde_json::Value;
use std::env;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState {
    client: Client,
    gatus_base_url: String,
}

#[tokio::main]
async fn main() {
    // Initialize tracing (optional, but good practice)
    tracing_subscriber::fmt::init();

    let gatus_base_url = env::var("GATUS_BASE_URL")
        .unwrap_or_else(|_| "https://status.deekshith.in".to_string());
    
    // Remove trailing slash if present to avoid double slashes
    let gatus_base_url = if gatus_base_url.ends_with('/') {
        gatus_base_url[..gatus_base_url.len()-1].to_string()
    } else {
        gatus_base_url
    };

    println!("Using Gatus Base URL: {}", gatus_base_url);

    let state = AppState {
        client: Client::new(),
        gatus_base_url,
    };

    let app = Router::new()
        .route("/api/statuses", get(get_statuses))
        .nest_service("/", ServeDir::new("static").append_index_html_on_directories(true))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_statuses(State(state): State<AppState>) -> Response {
    let url = format!("{}/api/v1/endpoints/statuses", state.gatus_base_url);
    
    match state.client.get(&url).send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<Value>().await {
                    Ok(data) => Json(data).into_response(),
                    Err(e) => (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR, 
                        format!("Failed to parse JSON: {}", e)
                    ).into_response(),
                }
            } else {
                (
                    resp.status(),
                    format!("Upstream error: {}", resp.status())
                ).into_response()
            }
        },
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch from Gatus: {}", e)
        ).into_response(),
    }
}