---
wave: 1
depends_on: []
files_modified: [src/config.rs, src/main.rs]
autonomous: true
---

# Plan 13: The Stimulus

Goal: Implement basic auto-restart logic for failing critical processes.

## Tasks

<task>
<read_first>
- src/config.rs
</read_first>
<action>
Update `Watchers` struct in `src/config.rs` to include `pub auto_heal: bool`.
Update `src/config.rs` to handle default values if necessary (though TOML will typically provide them).
</action>
<acceptance_criteria>
- `Watchers` struct has `auto_heal` field.
- Code compiles with `cargo check`.
</acceptance_criteria>
</task>

<task>
<read_first>
- src/main.rs
- src/event.rs
</read_first>
<action>
Modify the event loop in `main.rs` to handle `Event::ProcessCrash`.
If `current_config.watchers.auto_heal` is true, invoke `systemctl restart <name>`.
Log the attempt using `info!` and `error!`.
</action>
<acceptance_criteria>
- `main.rs` has logic for `Event::ProcessCrash` that checks `auto_heal`.
- Restart command uses `std::process::Command`.
</acceptance_criteria>
</task>

## Verification Criteria

### Automated Tests
- None (systemctl integration is hard to test in isolation).

### Manual Verification
1. Enable `auto_heal = true` in `autobuddy.toml`.
2. Ensure a service (e.g. `nginx`) is in `critical_processes`.
3. Stop the service manually: `sudo systemctl stop nginx`.
4. Observe autobuddy logs for "Attempting to restart nginx..." and wait for the recovery notification.

## Goals backward verification (must_haves)
- [ ] Daemon can automatically trigger `systemctl restart` for failing processes.
- [ ] Auto-heal behavior is configurable via the toml file.
