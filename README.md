# Gatus Frontend

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
| `GATUS_BASE_URL` | The base URL of your Gatus instance. | `https://status.deekshith.in` |

## Installation & Running

### Option 1: Local via Cargo

1.  Navigate to the project directory:
    ```bash
    cd gatus-frontend
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

3.  Open `http://localhost:3000` in your browser.

### Option 2: Docker Compose

1.  Navigate to the project directory:
    ```bash
    cd gatus-frontend
    ```

2.  Build and start the container:
    ```bash
    docker-compose up -d --build
    ```
    *(To change the Gatus URL, edit the `docker-compose.yml` file or pass it inline: `GATUS_BASE_URL=... docker-compose up ...`)*

3.  Open `http://localhost:3000` in your browser.

## Compilation (Static Binary with musl)

The included `Dockerfile` builds a static binary using `musl` on Alpine Linux.

1.  Build the Docker image:
    ```bash
    docker build -t gatus-frontend .
    ```

2.  Run the container:
    ```bash
    docker run -p 3000:3000 -e GATUS_BASE_URL="https://your-gatus-url.com" gatus-frontend
    ```
