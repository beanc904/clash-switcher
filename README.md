# Clash Switcher

A Rust-based CLI tool to manage a Docker-containerized Clash proxy (e.g., Clash Verge) and toggle system proxy settings efficiently.

## Features

- **Automatic Proxy Management**: Automatically toggle system proxy settings.
- **Docker Integration**: Manage the lifecycle of the Clash Docker container.
- **CLI Control**: Offers commands to launch, check status, toggle dashboards, and kill the container.

## Usage

The default behavior is:
- **Launch** the Docker container if it is not running.
- **Toggle** the system proxy if the container is already running.

### Commands

| Argument | Description |
| :--- | :--- |
| `--launch`, `-l` | Explicitly launch the Docker container. |
| `--on` | Turn on the system proxy. |
| `--off` | Turn off the system proxy. |
| `--status`, `-s` | Check the status of the system proxy and container. |
| `--dashboard`, `-d` | Open/Close the Clash dashboard. |
| `--kill`, `-k` | Kill the Clash Docker container. |

## Build and Setup

### Build Docker Image
```bash
docker build -t clash-verge-rev:v2.5.1 .
```

### Run
Ensure your `compose.yaml` path matches the one defined in `main.rs` (`/opt/clash-switcher/compose.yaml`).

```bash
# Run via docker compose
docker compose run --rm clash-verge
```

*Note: This tool assumes the container name is `clash-verge` and the image is `clash-verge-rev:v2.5.1`.*
