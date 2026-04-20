# Phase 12: The Installer - Research

Objective: Research implementation details for the autobuddy Linux installer and systemd integration.

## 1. Systemd Service Requirements
- **Type**: `notify` is preferred since we already implemented `sd-notify` signaling in Phase 10/11.
- **Security**:
    - `DynamicUser=yes` is great for simple daemons, but might complicate access to log files or persistent config in the home dir.
    - `CapabilityBoundingSet=` and `RestrictAddressFamilies=` for hardening.
- **Lifecycle**:
    - `WatchdogSec=10` (matching the 5s heartbeat we established in `main.rs`).
    - `Restart=on-failure`.

## 2. Installer Script (scripts/install.sh)
- **Environment Checks**: Ensure `cargo` (or pre-built binary if available) and `systemd` exist.
- **Workflow**:
    1. Build the release binary.
    2. Move binary to `/usr/local/bin/autobuddy`.
    3. Create config directory `/etc/autobuddy` or use user home.
    4. Install `autobuddy.service` to `/etc/systemd/system/`.
    5. Prompt user for Telegram Bot Token and Chat ID.
    6. Generate initial `autobuddy.toml`.
    7. `systemctl daemon-reload`, `enable`, and `start`.

## 3. Configuration Path Strategy
- Since the daemon is designed as a "buddy" for the user, it might be better to run as a **User Service** (`systemd --user`) or a system service that refers to a specific config file.
- Research suggest using `/etc/autobuddy/autobuddy.toml` for system-wide or `~/.config/autobuddy.toml` for user-specific. Given the "daemon" requirement in PROJECT.md, `/etc/` is more standard for a permanent "Keeper".

## 4. Dependencies
- Shell: `bash`.
- Tools: `cargo`, `systemctl`, `sed`.
