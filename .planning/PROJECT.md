# Project: autobuddy

A Rust-based system monitoring daemon that alerts via Telegram.

## Goal
A "viral" system buddy that watches your back, alerting you to high resource usage, dangerous shell commands, and build failures.

## Context
- **Current Version:** v2.0
- **Status:** Milestone 2 complete. Proactive guarding (SSH, Process, Oracle) active.
- **Next Up:** Milestone 3: Remote Governance (PIN-protected commands via Telegram).

<details>
<summary>Archived Milestone 1 & 2 Context</summary>

- **Milestone 1:** Core Robustness & Viral Features complete.
- **Milestone 2:** Contextual Intelligence complete.
- **Key Decisons:**
    - Telegram for Alerts (High friction, mobile notifications).
    - Rust/Tokio (Performance and concurrency).
    - Sentinel logic via log-tailing.
    - Bodyguard logic via sysinfo.
</details>

## Requirements (v3.0)

### Validated
- ✓ System Health Monitoring
- ✓ Terminal Command Guarding
- ✓ Build Failure Healing
- ✓ Access & Process Guarding

### Upcoming
- [ ] Command verification PIN structure
- [ ] Remote system status fetching
- [ ] Buddy "Personality" modes (Silent, Chatty, Nervous)

## Current Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Telegram for Alerts | Low friction, mobile notifications | ✓ Shipped |
| Custom Event Refactor | Faster integration of new watchers | ✓ Shipped |
| Smart Suggestions | User "wow" factor + utility | ✓ Shipped |
