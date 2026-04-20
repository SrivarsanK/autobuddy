# Project: autobuddy

A Rust-based system monitoring daemon that alerts via Telegram.

## Goal
A "viral" system buddy that watches your back, alerting you to high resource usage, dangerous shell commands, and build failures.

## Context
- **Current Version:** v4.0
- **Status:** Milestone 4 complete. System integration active.
- **Next Up:** Milestone 5: The Healer (Auto-recovery & Governance).

<details>
<summary>Archived Milestones</summary>

- **Milestone 1:** Core Robustness & Viral Features complete.
- **Milestone 2:** Contextual Intelligence complete.
- **Milestone 3:** Remote Governance complete.
</details>

## Current Milestone: v4.0 The Keeper

**Goal:** Transform autobuddy into a permanent system service with persistent state.

**Target features:**
- Systemd integration (Service unit, auto-start on boot)
- Configuration persistence (Save `/mode` and `/ping` state changes)
- Installer script (One-liner install/setup)
- Logging to journald

## Requirements (v4.0)

### Validated
- ✅ System Health Monitoring
- ✅ Terminal Command Guarding
- ✅ Build Failure Healing
- ✅ Access & Process Guarding
- ✅ PIN-protected Telegram Commands

### Upcoming
- [ ] Systemd service file template and generation
- [ ] Automatic configuration writing on state change
- [ ] Journald logging integration for service readability
- [x] Post-install setup wizard via Telegram

## Current Milestone: v5.0 The Healer

**Goal:** Bridge the gap between alerting and resolving by implementing auto-recovery.

**Target features:**
- Automatic `systemctl restart` for failing critical processes.
- Active feedback loops on Telegram for interactive healing.
- Recovery verification reporting.

### Upcoming Requirements (v5.0)
- [ ] Implement `heal` logic in `ProcessWatcher`.
- [ ] Add `confirm_restart` interaction in Telegram.
- [ ] Post-recovery status check.

## Current Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Telegram for Alerts | Low friction, mobile notifications | ✅ Shipped |
| systemd for Service | Native Linux standard for background daemons | ✅ Planned |
| Persistent TOML | Simplest way to persist mode changes without a DB | ✅ Planned |
