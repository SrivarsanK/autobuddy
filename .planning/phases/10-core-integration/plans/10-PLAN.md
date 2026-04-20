---
wave: 1
depends_on: []
files_modified: ["Cargo.toml", "src/main.rs"]
autonomous: true
requirements: ["SYS-01", "SYS-02", "SYS-03"]
---

# Plan: Core Systemd & Journald Integration

Implement the "plumbing" required for autobuddy to run as a native Linux service with heartbeat monitoring and structured logging.

## Tasks

<task>
<read_first>
- c:\Users\Arunavo\Desktop\autobuddy\Cargo.toml
</read_first>
<action>
Add the following dependencies to `Cargo.toml`:
- `log = "0.4"`
- `systemd-journal-logger = "0.4"`
- `sd-notify = "0.4"`
</action>
<acceptance_criteria>
- `Cargo.toml` contains `log`, `systemd-journal-logger`, and `sd-notify`.
- `cargo check` passes without dependency conflicts.
</acceptance_criteria>
</task>

<task>
<read_first>
- c:\Users\Arunavo\Desktop\autobuddy\src\main.rs
</read_first>
<action>
Refactor `main.rs` logger initialization:
1. Initialize `JournalLog` using `systemd_journal_logger::JournalLog::new()`.
2. Map log levels to journal priorities.
3. Add a fallback to simple console logging if not connected to journald (optional but recommended for development).
4. Remove any existing simple `println!` blocks used for debugging in favor of `log::info!`, `log::warn!`, and `log::error!`.
</action>
<acceptance_criteria>
- `src/main.rs` uses `JournalLog` for initialization.
- `cargo run` prints logs (if on Linux, verify with `journalctl -f`).
</acceptance_criteria>
</task>

<task>
<read_first>
- c:\Users\Arunavo\Desktop\autobuddy\src\main.rs
- c:\Users\Arunavo\Desktop\autobuddy\src\telegram.rs
</read_first>
<action>
Implement Readiness Signal:
1. In `main.rs`, after the Telegram bot client has successfully initialized (handshake complete), call `sd_notify::notify(true, &[NotifyState::Ready])`.
2. Ensure this only happens once.
</action>
<acceptance_criteria>
- The binary only signals "Ready" after the bot is connected.
- Verification via `systemctl status autobuddy` (once service is set up in later phase).
</acceptance_criteria>
</task>

<task>
<read_first>
- c:\Users\Arunavo\Desktop\autobuddy\src\main.rs
</read_first>
<action>
Implement Heartbeat Watchdog:
1. Spawn a background `tokio` task in `main.rs` (start of the app).
2. Every 5 seconds, send `sd_notify::notify(true, &[NotifyState::Watchdog])`.
3. Ensure the task handles the loop lifetime of the app.
</action>
<acceptance_criteria>
- A background task exists in `main.rs` for heartbeats.
- Heartbeat is sent exactly every 5 seconds.
</acceptance_criteria>
</task>

## Verification
- Running `cargo run` (local) shouldn't crash when systemd is missing (sd-notify handles missing socket).
- Logs show up in standard output during development.
- Code uses `log::*` macros instead of `println!`.

## must_haves
1. Readiness signal sent AFTER telegram init.
2. 5s Heartbeat task running globally.
3. Journald logger installed.
