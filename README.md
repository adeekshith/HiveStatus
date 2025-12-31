# HiveStatus

A modern, hexagonal monitoring dashboard frontend for [Gatus](https://github.com/TwiN/gatus). This application acts as a proxy and visualization layer, fetching status data from a Gatus instance and displaying it in a responsive, auto-refreshing honeycomb grid.

## Features

-   **Hexagonal Grid UI:** Visually distinct status indicators (Green for Up, Red for Down).
-   **Grouped Services:** Automatically groups services based on their key prefix (e.g., `core_api` -> Group "Core").
-   **Dark/Light Mode:** Automatically adapts to your system's color scheme.
-   **Responsive Design:** "Honeycomb" packing adapts perfectly to various screen sizes, from mobile to ultra-wide monitors.
-   **Auto-Refresh:** Updates status every 60 seconds.
-   **Rust Backend:** Fast, lightweight proxy using `Axum` to handle CORS and serve static files.

## Prerequisites

-   **Rust:** (For local compilation) v1.70+
-   **Docker & Docker Compose:** (For containerized deployment)

## Configuration

The application is configured via environment variables:

| Variable | Description | Default |
| :--- | :--- | :--- |
| `APP_GATUS_URL` | The base URL of your Gatus instance. | `https://status.twin.sh` |
| `APP_PAGE_TITLE` | The title of the HTML page. | `HiveStatus` |
| `APP_PORT` | The port the server will listen on. | `3000` |
| `APP_HOST` | The host address the server will bind to. | `0.0.0.0` |
| `APP_LOG_LEVEL` | The verbosity of application logs (`info`, `debug`, `warn`, `error`). | `info` |
| `APP_REFRESH_INTERVAL_MS` | The data refresh interval in milliseconds. | `60000` |

## Installation & Running

### Option 1: Docker Compose (Recommended)

You can run HiveStatus easily using the pre-built image from GitHub Container Registry.

1.  Create a `docker-compose.yml` file:
    ```yaml
    version: '3.8'

    services:
      hive-status:
        image: ghcr.io/adeekshith/hivestatus:latest
        ports:
          - "${APP_PORT:-3000}:${APP_PORT:-3000}"
        environment:
          - APP_GATUS_URL=${APP_GATUS_URL:-https://status.twin.sh}
          - APP_PAGE_TITLE=${APP_PAGE_TITLE:-HiveStatus}
          - APP_PORT=${APP_PORT:-3000}
          - APP_HOST=${APP_HOST:-0.0.0.0}
          - APP_LOG_LEVEL=${APP_LOG_LEVEL:-info}
          - APP_REFRESH_INTERVAL_MS=${APP_REFRESH_INTERVAL_MS:-60000}
        restart: unless-stopped
    ```

2.  Start the container:
    ```bash
    docker-compose up -d
    ```

3.  Open `http://localhost:${APP_PORT:-3000}` in your browser.

### Option 2: Run with Docker (Quick Start)

Run the dashboard with a single command:
```bash
docker run -d -p 3000:3000 \
  -e APP_GATUS_URL="https://your-gatus-instance.com" \
  -e APP_PAGE_TITLE="My Custom Status" \
  -e APP_PORT="3000" \
  -e APP_HOST="0.0.0.0" \
  -e APP_LOG_LEVEL="info" \
  -e APP_REFRESH_INTERVAL_MS="30000" \
  ghcr.io/adeekshith/hivestatus:latest
```

### Option 3: Local via Cargo

1.  Navigate to the project directory:
    ```bash
    cd hive-status
    ```

2.  Run the application with your desired configuration:
    ```bash
    # Linux/Mac
    export APP_GATUS_URL="https://your-gatus-url.com"
    export APP_PAGE_TITLE="My Status Page"
    export APP_PORT="8080"
    export APP_LOG_LEVEL="debug"
    export APP_REFRESH_INTERVAL_MS="15000"
    cargo run

    # Windows (PowerShell)
    $env:APP_GATUS_URL="https://your-gatus-url.com"
    $env:APP_PAGE_TITLE="My Status Page"
    $env:APP_PORT="8080"
    $env:APP_LOG_LEVEL="debug"
    $env:APP_REFRESH_INTERVAL_MS="15000"
    cargo run
    ```


## Compilation (Static Binary with musl)

The included `Dockerfile` builds a static binary using `musl` on Alpine Linux.

1.  Build the Docker image:
    ```bash
    docker build -t hive-status .
    ```

2.  Run the container:
    ```bash
    docker run -p 3000:3000 -e GATUS_BASE_URL="https://your-gatus-url.com" hive-status
    ```