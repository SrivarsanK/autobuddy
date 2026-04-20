# Phase 10 Context: Core Integration

Decisions for systemd and logging infrastructure.

## Decisions
- **Watchdog Strategy:** Use `sd-notify` to send a heartbeat every 5 seconds. Timeout in systemd set to 20 seconds.
- **Logging Policy:** Use `systemd-journal-logger` as the primary log source. No local log files. Log levels match Rust's `log` levels (Info/Warn/Error).
- **Ready Signal:** Only call `sd_notify::notify(true, &[NotifyState::Ready])` after the Telegram bot successfully initializes.
- **Signal Handling:** Handle SIGTERM/SIGINT gracefully to signal `NotifyState::Stopping` to systemd before exit.

## Research Findings
- Using `sd-notify` crate (v0.4) for minimal footprint.
- Journal logging supports structured fields (e.g., adding `VERSION` to logs).

## Deferred Ideas
- **Watchdog-Aware Watchers:** Having individual watchers report their health to the heartbeat task (Complexity too high for Phase 10).
