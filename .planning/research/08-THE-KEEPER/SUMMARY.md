# Research Summary: The Keeper (systemd & Persistence)

## Executive Summary
This research covers best practices for integrating Rust applications with `systemd` (specifically signaling, logging, and lifecycle) and strategies for configuration persistence without external databases.

## 1. Systemd Integration

### Signaling Readiness (`sd-notify`)
- **Protocol:** Use `Type=notify` in the service unit.
- **Crate:** `sd-notify` (v0.4). It is lightweight and handles environment variable checking (`NOTIFY_SOCKET`).
- **Implementation:** Call `notify(true, &[NotifyState::Ready])` after initialization (config loaded, bot started).

### Logging (`journald`)
- **Facade:** Use the standard `log` crate.
- **Implementation:** `systemd-journal-logger` (v0.4).
- **Benefit:** Maps Rust log levels (`info!`, `warn!`, `error!`) directly to `journalctl` priorities.
- **Fallback:** Use `is_journal_connected()` to detect if we should print to `stderr` instead (useful for development).

### Watchdog
- **Safety:** can use `WatchdogSec=30` in the unit.
- **Rust:** Send `NotifyState::Watchdog` every N seconds in a background tokio task.

## 2. Configuration Persistence

### Strategy: Direct TOML Management
- **Goal:** Update `autobuddy.toml` when settings change via Telegram.
- **Approach:**
    - Represent configuration internally with `serde`.
    - Use `toml::to_string_pretty(&config)` to serialize.
    - Atomically write back to file.
- **Warning:** Must be careful with file locks and racing between `ProcessWatcher` and `main` loop during state changes.

## 3. Installer Strategy

### Components
- **Systemd Unit Template:**
    ```ini
    [Unit]
    Description=autobuddy system daemon
    After=network.target

    [Service]
    Type=notify
    ExecStart=/usr/local/bin/autobuddy
    Restart=always
    WatchdogSec=20

    [Install]
    WantedBy=multi-user.target
    ```
- **Bash Script:**
    1. Build binary (`cargo build --release`).
    2. Copy to `/usr/local/bin/`.
    3. Place `autobuddy.toml` in `/etc/autobuddy/` (or current directory).
    4. Install/Enable systemd unit.

## 4. Proposed Build Order
1. Implement `sd-notify` and `log` integration.
2. Implement Config persistence logic (update fields + write to disk).
3. Create the installer script and service template.
4. Final "Bodyguard" check: Ensure autobuddy can verify its own service status.
