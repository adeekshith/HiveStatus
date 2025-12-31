use std::env;
use std::str::FromStr;
use tracing::Level;
use serde::{Deserialize, Serialize}; // Keep for PublicConfig

#[derive(Clone)] // AppConfig used internally by the server
pub struct AppConfig {
    pub page_title: String,
    pub gatus_url: String,
    pub refresh_interval_ms: u64,
    pub host: String,
    pub port: u16,
    pub log_level: Level, // This field doesn't need to be serialized/deserialized
}

impl AppConfig {
    pub fn new() -> Self {
        let host = env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("APP_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .unwrap_or(3000);
        let log_level_str = env::var("APP_LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
        let log_level = Level::from_str(&log_level_str).unwrap_or(Level::INFO);

        let refresh_interval_ms = env::var("APP_REFRESH_INTERVAL_MS")
            .unwrap_or_else(|_| "60000".to_string())
            .parse::<u64>()
            .unwrap_or(60000);

        let gatus_url = env::var("APP_GATUS_URL")
            .unwrap_or_else(|_| "https://status.twin.sh".to_string());
        
        let page_title = env::var("APP_PAGE_TITLE").unwrap_or_else(|_| "HiveStatus".to_string());

        Self {
            page_title,
            gatus_url,
            refresh_interval_ms,
            host,
            port,
            log_level,
        }
    }
}

// This struct is used for the public API to the frontend
#[derive(Clone, Serialize, Deserialize)]
pub struct AppPublicConfig {
    pub page_title: String,
    pub gatus_url: String,
    pub refresh_interval_ms: u64,
}