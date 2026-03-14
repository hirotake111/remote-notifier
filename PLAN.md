## Problem

I use GitHub Codespaces for work. And I use coding agents side the dev container.
And inside the container there is no way to notify the agents (say, claude code) when it's done its task.
I don't watch agents thinking along the way all the time - so I need it to have a way to ping me when it's done.

## Expected usecase

Me (macos) + TMUX | -- SSH --> | dev container + claude code |

## Solution

Use SSH reverse port forwarding to send notifications from the container to macOS without an intermediate server.

## Architecture

- **macOS**: runs a local server that listens for notifications and shows system notifications
- **SSH**: reverse port forward (`-R 9000:localhost:9000`)
- **Container**: sends HTTP POST request to localhost:9000

## Steps

- [x] **Initialize Rust project**
  - `cargo init --bin`

- [x] **Implement server**
  - Add dependencies (axum for HTTP, notify-rust for macOS notifications)
  - Create HTTP server that listens on localhost:9000
  - Handle POST requests and show system notification
  - Use tokio for async runtime

- [x] **Test locally**
  - Run server
  - Send test request with curl

- [x] **Build and release**
  - Configure Cargo.toml for release build
  - Build binary

## Future enhancements (out of scope for v1)

- CLI client for sending notifications
- Support for different notification sounds
- Configuration file for port/customization
