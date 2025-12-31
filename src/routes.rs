use crate::startup::AppState;
use crate::configuration::AppPublicConfig; // Import AppPublicConfig
use axum::{
    extract::State,
    response::{IntoResponse, Json, Response},
};
use reqwest::StatusCode;
use serde_json::Value;
use tracing;

pub async fn get_config(State(state): State<AppState>) -> impl IntoResponse {
    let public_config = AppPublicConfig {
        page_title: state.config.page_title.clone(),
        gatus_url: state.config.gatus_url.clone(),
        refresh_interval_ms: state.config.refresh_interval_ms,
    };
    Json(public_config)
}

pub async fn get_statuses(State(state): State<AppState>) -> Response {
    let url = format!("{}/api/v1/endpoints/statuses", state.config.gatus_url);

    match state.client.get(&url).send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<Value>().await {
                    Ok(data) => Json(data).into_response(),
                    Err(e) => {
                        tracing::error!("Failed to parse JSON from Gatus: {}", e);
                        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON".to_string())
                            .into_response()
                    }
                }
            } else {
                tracing::warn!("Received upstream error from Gatus: {}", resp.status());
                (resp.status(), format!("Upstream error: {}", resp.status())).into_response()
            }
        }
        Err(e) => {
            tracing::error!("Failed to fetch from Gatus: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch from Gatus".to_string())
                .into_response()
        }
    }
}
