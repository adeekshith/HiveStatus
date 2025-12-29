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
| `GATUS_BASE_URL` | The base URL of your Gatus instance. | `https://status.twin.sh` |

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
          - "3000:3000"
        environment:
          - GATUS_BASE_URL=https://status.deekshith.in
        restart: unless-stopped
    ```

2.  Start the container:
    ```bash
    docker-compose up -d
    ```

3.  Open `http://localhost:3000` in your browser.

### Option 2: Run with Docker (Quick Start)

Run the dashboard with a single command:
```bash
docker run -d -p 3000:3000 -e GATUS_BASE_URL="https://status.twin.sh" ghcr.io/adeekshith/hivestatus:latest
```

### Option 3: Local via Cargo

1.  Navigate to the project directory:
    ```bash
    cd hive-status
    ```

2.  Run the application:
    ```bash
    # Linux/Mac
    export GATUS_BASE_URL="https://your-gatus-url.com"
    cargo run

    # Windows (PowerShell)
    $env:GATUS_BASE_URL="https://your-gatus-url.com"
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