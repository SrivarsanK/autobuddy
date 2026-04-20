# Phase 13: The Stimulus - Research

Objective: Research implementation of auto-restart logic for Linux services in Rust.

## 1. Systemctl Integration
- **Command**: `systemctl restart <service_name>`.
- **Rust Execution**: Use `std::process::Command`.
- **Permissions**: `systemctl restart` usually requires `sudo` or root privileges. Since we planned the service to run as `root` in Phase 12, the daemon will have the necessary permissions naturally.

## 2. Configuration Schema
- **Requirement**: We need to know which processes should be "healed".
- **Proposed change**:
```toml
[watchers]
critical_processes = ["postgres", "nginx"]
auto_heal = true # Global flag or per-process?
```
- **Preference**: A global flag `auto_heal` under `watchers` is simpler for a start, but we should eventually allow per-process overrides.

## 3. Detecting Crashes
- Our `ProcessWatcher` already detects when a process in `critical_processes` is no longer in the process list.
- **Workflow**:
    1. `ProcessWatcher` detects missing process.
    2. Emits `ProcessCrash { name }`.
    3. `AlertEngine` or `Main Loop` catches this and checks `config.watchers.auto_heal`.
    4. If true, triggers the heal.

## 4. Risks
- **Infinite Restart Loops**: If a service fails immediately upon start, we might spam `systemctl restart`. 
- **Mitigation**: Use a cooldown period for healing attempts (reuse `alert_cooldown_secs` or dedicated `heal_cooldown`).
