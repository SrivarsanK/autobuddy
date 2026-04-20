# Milestone 4 Requirements: The Keeper

Grouped requirements for system integration and state persistence.

## 1. System Integration (SYS)
- [ ] **SYS-01**: Implement `sd-notify` to signal `READY` state to systemd after bot initialization.
- [ ] **SYS-02**: Integrate `systemd-journal-logger` for structured logging with priority levels.
- [ ] **SYS-03**: Implement a background "Heartbeat" task that pings systemd's watchdog.
- [ ] **SYS-04**: Generate a standard `autobuddy.service` unit file template.
- [ ] **SYS-05**: Create a `setup.sh` installer script for Linux environments.

## 2. State Persistence (PER)
- [ ] **PER-01**: Implement `Config::save` method to serialize current state back to TOML.
- [ ] **PER-02**: Update `/mode` command to automatically trigger a config save on change.
- [ ] **PER-03**: Persist `chat_id` and `buddy_mode` across daemon restarts.
- [ ] **PER-04**: Atomic file writing to prevent config corruption during unexpected shut-downs.

## Future Requirements (Backlog)
- [ ] **HIST-01**: Local SQLite storage for event history.
- [ ] **GUI-01**: Web dashboard for remote config management.

## Out of Scope
- Windows Service integration (Focusing on Linux/Systemd for this milestone).
- Multiple Telegram bot support.

## Traceability
*To be filled by roadmap*
