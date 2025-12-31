use hive_status::configuration::{AppConfig, AppPublicConfig};
use hive_status::startup;
use once_cell::sync::Lazy;
use tokio::net::TcpListener;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
use axum::http::StatusCode; // Import StatusCode

// Lazy static to ensure the tracing subscriber is only initialized once
static TRACING: Lazy<()> = Lazy::new(|| {
    // Use try_init() to avoid panicking if a subscriber is already set (e.g., in parallel tests)
    tracing_subscriber::fmt().compact().try_init().ok();
});

/// Helper function to spawn the application in the background for testing.
async fn spawn_app() -> (String, MockServer) {
    Lazy::force(&TRACING); // Ensure tracing is initialized at least once

    // 1. Setup a mock Gatus server
    let mock_server = MockServer::start().await;

    // 2. Configure the application with test environment variables
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();

    let config = AppConfig {
        host: "127.0.0.1".to_string(),
        port,
        gatus_url: mock_server.uri(),
        page_title: "Test Title".to_string(),
        refresh_interval_ms: 5000,
        log_level: tracing::Level::DEBUG, // Use DEBUG for test logs
    };

    // 3. Run our app in a background thread using the provided listener
    let server_address = format!("http://127.0.0.1:{}", port);
    tokio::spawn(async move {
        startup::run_with_listener(config, listener).await;
    });

    // 4. Wait for the server to be ready by polling /api/config
    let client = reqwest::Client::new();
    loop {
        if let Ok(response) = client.get(format!("{}/api/config", server_address)).send().await {
            if response.status().is_success() {
                break;
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(10)).await; // Small delay to prevent busy-waiting
    }

    // 5. Return the address and the mock server instance
    (server_address, mock_server)
}

#[tokio::test]
async fn config_endpoint_returns_correct_configuration() {
    // Arrange
    let (app_address, _mock_server) = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/api/config", app_address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    let config: AppPublicConfig = response.json().await.expect("Failed to parse JSON");

    assert_eq!(config.page_title, "Test Title");
    assert_eq!(config.refresh_interval_ms, 5000);
}

#[tokio::test]
async fn statuses_endpoint_proxies_gatus_request() {
    // Arrange
    let (app_address, mock_server) = spawn_app().await;
    let client = reqwest::Client::new();

    // Setup the mock Gatus API response
    let mock_response_body = serde_json::json!([
        {
            "name": "Mock Service",
            "key": "mock_service",
            "results": [
                { "success": true }
            ]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response_body))
        .mount(&mock_server)
        .await;

    // Act
    let response = client
        .get(format!("{}/api/statuses", app_address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    let response_body: serde_json::Value = response.json().await.expect("Failed to parse JSON");
    assert_eq!(response_body, mock_response_body);
}

#[tokio::test]
async fn statuses_endpoint_handles_upstream_non_200_response() {
    // Arrange
    let (app_address, mock_server) = spawn_app().await;
    let client = reqwest::Client::new();

    // Setup the mock Gatus API to return a 404
    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;

    // Act
    let response = client
        .get(format!("{}/api/statuses", app_address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = response.text().await.expect("Failed to get response body");
    assert!(body.contains("Upstream error: 404 Not Found"));
}

#[tokio::test]
async fn statuses_endpoint_handles_upstream_invalid_json() {
    // Arrange
    let (app_address, mock_server) = spawn_app().await;
    let client = reqwest::Client::new();

    // Setup the mock Gatus API to return invalid JSON
    Mock::given(method("GET"))
        .and(path("/api/v1/endpoints/statuses"))
        .respond_with(ResponseTemplate::new(200).set_body_string("this is not json"))
        .mount(&mock_server)
        .await;

    // Act
    let response = client
        .get(format!("{}/api/statuses", app_address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let body = response.text().await.expect("Failed to get response body");
    assert!(body.contains("Failed to parse JSON"));
}

#[tokio::test]
async fn static_files_are_served() {
    // Arrange
    let (app_address, _mock_server) = spawn_app().await;
    let client = reqwest::Client::new();

    // Act: Request a known static file
    let response = client
        .get(format!("{}/style.css", app_address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(
        response.headers()["content-type"],
        "text/css"
    );
}