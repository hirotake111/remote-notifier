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

   # Or run as daemon (background)
   ./target/release/remote-notifier --daemon
   ```

2. **Set up reverse SSH tunnel:**
   ```bash
   # Manual
   ssh -f -N -R 9000:localhost:9000 user@container

   # Or use the helper script
   ./reverse-ssh.sh user@container
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

# Run server
./target/release/remote-notifier

# Or run as daemon (background)
./target/release/remote-notifier --daemon

# Set up tunnel (in another terminal or background)
./reverse-ssh.sh user@container

# Kill daemon when done
kill $(cat /tmp/remote-notifier.pid)

# Kill tunnel when done
pkill -f "ssh.*-R 9000"
```

When running with `--daemon`:
- PID file: `/tmp/remote-notifier.pid`
- Log file: `/tmp/remote-notifier.log`
