# Project: autobuddy

A Rust-based system monitoring daemon that alerts via Telegram.

## Goal
A "viral" system buddy that watches your back, alerting you to high resource usage, dangerous shell commands, and build failures.

## Context
- **Runtime:** Rust (tokio, teloxide, sysinfo)
- **Status:** Phase 1 complete (Basic daemon, Telegram alerts, SysHealth watcher).
- **Next Up:** Proper thresholding from config, full DangerousCommand interception, and BuildFailure detection.

## Requirements

### Validated
- ✓ Basic daemon structure — existing
- ✓ Telegram alert delivery — existing
- ✓ CPU/RAM monitoring — existing

### Active
- [ ] Connect thresholds in `AlertEngine` to `autobuddy.toml` config
- [ ] Implement `DangerousCommand` detection in `TerminalWatcher`
- [ ] Implement `BuildFailure` watcher for common build tools (cargo, npm)
- [ ] Add `ProcessCrash` watcher

### Out of Scope
- [ ] Remote dashboard — focusing on Telegram for now
- [ ] Multi-user support — single local buddy for the user

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Telegram for Alerts | Low friction, mobile notifications out of the box | ✓ Implemented |
| Rust/Tokio | High performance, safe concurrency for multiple watchers | ✓ Implemented |
