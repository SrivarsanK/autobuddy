# Project: autobuddy

A Rust-based system monitoring daemon that alerts via Telegram.

## Goal
A "viral" system buddy that watches your back, alerting you to high resource usage, dangerous shell commands, and build failures.

## Context
- **Runtime:** Rust (tokio, teloxide, sysinfo)
- **Status:** Milestone 1 complete. Core watchers (SysHealth, Terminal, Build) active.
- **Next Up:** Milestone 2: Contextual Intelligence (Sentinels, Bodyguards, and Oracles).

## Requirements

### Validated
- ✓ Basic daemon structure — existing
- ✓ Telegram alert delivery — existing
- ✓ CPU/RAM monitoring — existing

### Active
- [x] Connect thresholds in `AlertEngine` to `autobuddy.toml` config
- [x] Implement `DangerousCommand` detection in `TerminalWatcher`
- [x] Implement `BuildFailure` watcher for common build tools (cargo, npm)
- [ ] Implement SSH Watcher (The Sentinel)
- [ ] Implement `ProcessCrash` watcher (The Bodyguard)
- [ ] Implement Smart Diagnosis (The Oracle)

### Out of Scope
- [ ] Remote dashboard — focusing on Telegram for now
- [ ] Multi-user support — single local buddy for the user

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Telegram for Alerts | Low friction, mobile notifications out of the box | ✓ Implemented |
| Rust/Tokio | High performance, safe concurrency for multiple watchers | ✓ Implemented |
