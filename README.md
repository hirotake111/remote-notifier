# remote-notifier

Notify your local machine when a task running in a remote dev container (e.g., GitHub Codespaces) is complete.

## Problem

You use GitHub Codespaces for work with coding agents (like Claude Code) inside the dev container. There's no way to get notified when the agent finishes its task since you don't watch it constantly.

## Solution

A simple client/server setup that works through SSH reverse port forwarding - no intermediate server needed.

## Architecture

```
┌─────────────────────────────────────┐
│  Local Machine (macOS/Linux)        │
│                                     │
│  ┌─────────────────────────────┐   │
│  │ remote-notifier             │   │
│  │ - listens on localhost:9000 │   │
│  │ - shows notification + sound│   │
│  └─────────────────────────────┘   │
└─────────────────────────────────────┘
            ▲
            │ SSH reverse tunnel (-f -N -R)
            │ container:9000 → local:9000
            │
┌─────────────────────────────────────┐
│  Dev Container                      │
│                                     │
│  curl -X POST localhost:9000       │
│     -d '{"message":"done!"}'      │
└─────────────────────────────────────┘
```

## Usage

1. **Start the server on your local machine:**
   ```bash
   # Run in foreground
   ./target/release/remote-notifier

   # Run as daemon (background)
   ./target/release/remote-notifier --daemon

   # Or run with tunnel (foreground)
   ./target/release/remote-notifier --tunnel user@container

   # Or run as daemon with tunnel
   ./target/release/remote-notifier --daemon --tunnel user@container
   ```

2. **Send a notification from the container when done:**
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

# Run server
./target/release/remote-notifier

# Run with tunnel (all-in-one)
./target/release/remote-notifier --tunnel user@container

# Run as daemon with tunnel
./target/release/remote-notifier --daemon --tunnel user@container

# Stop the tunnel
./target/release/remote-notifier --kill-tunnel

# Stop the daemon
kill $(cat /tmp/remote-notifier.pid)
```

## Flags

| Flag | Description |
|------|-------------|
| `--daemon` | Run in background |
| `--tunnel <user@host>` | Start SSH reverse tunnel |
| `--kill-tunnel` | Stop the tunnel |

## Troubleshooting

### "remote port forwarding failed for listen port 9000"

The remote port 9000 is already in use (stale tunnel from a previous run).

**Solution:** Kill the process on the remote server:

```bash
ssh <user@host> "sudo fuser -k 9000/tcp"
```

### Binary hangs at "Testing SSH connection..."

Ensure your SSH key is set up for passwordless authentication:

```bash
ssh-copy-id <user@host>
```
