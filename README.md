# remote-notifier

Notify your local machine when a task running in a remote dev container (e.g., GitHub Codespaces) is complete.

## Problem

You use GitHub Codespaces for work with coding agents (like Claude Code) inside the dev container. There's no way to get notified when the agent finishes its task since you don't watch it constantly.

## Solution

A simple client/server setup that works through SSH reverse port forwarding - no intermediate server needed.

## Architecture

```
┌─────────────────────────────────────┐
│  macOS                              │
│                                     │
│  ┌─────────────────────────────┐   │
│  │ notifier-server             │   │
│  │ - listens on localhost:9000 │   │
│  │ - shows notification        │   │
│  └─────────────────────────────┘   │
└─────────────────────────────────────┘
            ▲
            │ SSH reverse tunnel (-R)
            │ container:9000 → macOS:9000
            │
┌─────────────────────────────────────┐
│  dev container                      │
│                                     │
│  curl -X POST localhost:9000       │
│     -d "task done!"                │
└─────────────────────────────────────┘
```

## Usage

1. **Start the server on your local machine:**
   ```bash
   ./target/release/remote-notifier
   ```

2. **SSH into the container with reverse port forwarding:**
   ```bash
   ssh -R 9000:localhost:9000 user@container
   ```

3. **Send a notification from the container when done:**
   ```bash
   curl -X POST localhost:9000 \
     -H "Content-Type: application/json" \
     -d '{"message":"Claude Code finished!"}'
   ```

## Requirements

- **macOS**: No additional dependencies
- **Linux**: Install `libnotify-bin` for `notify-send`
  ```bash
  sudo apt install libnotify-bin  # Debian/Ubuntu
  sudo dnf install libnotify       # Fedora
  ```

## Installation

```bash
# Build
cargo build --release

# Run
cargo run --release
```
