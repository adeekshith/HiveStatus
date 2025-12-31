use serde::{Deserialize, Serialize};
use std::env;
use std::str::FromStr;
use tracing::Level;

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

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::Level;

    // Use a helper to set/unset env vars safely for tests
    struct TestEnvVarGuard {
        name: &'static str,
        original_value: Option<String>,
    }

    impl TestEnvVarGuard {
        fn new(name: &'static str, value: &str) -> Self {
            let original_value = env::var(name).ok();
            unsafe {
                env::set_var(name, value);
            }
            TestEnvVarGuard { name, original_value }
        }

        fn remove(name: &'static str) -> Self {
            let original_value = env::var(name).ok();
            unsafe {
                env::remove_var(name);
            }
            TestEnvVarGuard { name, original_value }
        }
    }

    impl Drop for TestEnvVarGuard {
        fn drop(&mut self) {
            unsafe {
                if let Some(value) = &self.original_value {
                    env::set_var(self.name, value);
                } else {
                    env::remove_var(self.name);
                }
            }
        }
    }

    #[test]
    fn app_config_new_uses_defaults_when_no_env_vars() {
        // Ensure no relevant env vars are set before test
        let _guard_host = TestEnvVarGuard::remove("APP_HOST");
        let _guard_port = TestEnvVarGuard::remove("APP_PORT");
        let _guard_gatus_url = TestEnvVarGuard::remove("APP_GATUS_URL");
        let _guard_page_title = TestEnvVarGuard::remove("APP_PAGE_TITLE");
        let _guard_refresh = TestEnvVarGuard::remove("APP_REFRESH_INTERVAL_MS");
        let _guard_log_level = TestEnvVarGuard::remove("APP_LOG_LEVEL");

        let config = AppConfig::new();

        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3000);
        assert_eq!(config.gatus_url, "https://status.twin.sh");
        assert_eq!(config.page_title, "HiveStatus");
        assert_eq!(config.refresh_interval_ms, 60000);
        assert_eq!(config.log_level, Level::INFO);
    }

    #[test]
    fn app_config_new_reads_env_vars() {
        let _guard_host = TestEnvVarGuard::new("APP_HOST", "127.0.0.1");
        let _guard_port = TestEnvVarGuard::new("APP_PORT", "8080");
        let _guard_gatus_url = TestEnvVarGuard::new("APP_GATUS_URL", "http://test.gatus.com");
        let _guard_page_title = TestEnvVarGuard::new("APP_PAGE_TITLE", "Custom Title");
        let _guard_refresh = TestEnvVarGuard::new("APP_REFRESH_INTERVAL_MS", "10000");
        let _guard_log_level = TestEnvVarGuard::new("APP_LOG_LEVEL", "debug");

        let config = AppConfig::new();

        assert_eq!(config.host, "127.0.0.1"); // Error here, should be 127.0.0.1
        assert_eq!(config.port, 8080);
        assert_eq!(config.gatus_url, "http://test.gatus.com");
        assert_eq!(config.page_title, "Custom Title");
        assert_eq!(config.refresh_interval_ms, 10000);
        assert_eq!(config.log_level, Level::DEBUG);
    }

    #[test]
    fn app_config_new_handles_invalid_numeric_env_vars() {
        let _guard_port = TestEnvVarGuard::new("APP_PORT", "invalid");
        let _guard_refresh = TestEnvVarGuard::new("APP_REFRESH_INTERVAL_MS", "not_a_number");
        let _guard_log_level = TestEnvVarGuard::new("APP_LOG_LEVEL", "invalid_level");

        let config = AppConfig::new();

        assert_eq!(config.port, 3000);
        assert_eq!(config.refresh_interval_ms, 60000);
        assert_eq!(config.log_level, Level::INFO);
    }
}