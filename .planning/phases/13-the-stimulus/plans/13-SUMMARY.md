# Phase 13 Summary: The Stimulus

Objective: Implement basic auto-restart logic for failing critical processes.

## Key Changes

### Configuration
- **src/config.rs**: Added `auto_heal: bool` to the `Watchers` struct. This allows users to enable/disable self-healing globally via `autobuddy.toml`.

### Orchestration & Healing
- **src/main.rs**: Upgraded the central event loop to intercept `ProcessCrash` events.
- **Auto-Restart Logic**: When a crash is detected for a critical process and `auto_heal` is true:
    - The daemon issues a `systemctl restart <service>` command.
    - Resulting success/failure is logged to the system journal.
    - The user is notified via Telegram about the intervention status ("I've successfully restarted X" or "My attempt to heal X failed").

## Verification Results

### Build & Integrity
- `cargo check`: **PASSED** (all issues related to partial moves resolved).
- **Security**: The binary uses `systemctl` directly, which requires appropriate service permissions (already addressed by the root-running service unit in Phase 12).

### Self-Check: PASSED
- [x] `auto_heal` toggle added.
- [x] Crash detection triggers restart logic.
- [x] Telegram feedback loop integrated for healing events.
