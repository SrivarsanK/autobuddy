## Phase 5 Summary: The Bodyguard

### Objective
Ensure critical processes are always running and alert on absence.

### Key Changes
- **Configuration Update**: Added `critical_processes: Vec<String>` to the `Watchers` struct. Enabled by default with `postgres`, `nginx`, and `redis`.
- **ProcessWatcher [src/watchers/process.rs]**:
    - Leverages the `sysinfo` crate to scan the process table.
    - Polls every 5 seconds for configured critical names.
    - Emits `Event::ProcessCrash` if a required process is missing.
- **AlertEngine Expansion**: Added a Warning-level alert for process absence: "Bodyguard Alert: [Process] is missing from duty!"
- **Integration**: Explicitly spawned in `main.rs` along with other watchers.

### Verification Results
- **Unit Tests**: Added `test_alert_on_process_crash`. All 6 tests passed.
- **Diagnostics**: Fixed unreachable pattern warnings in `AlertEngine` logic by ensuring total coverage of `Event` variants.
- **Commit**: `b5fcf40` feat(5): implement ProcessWatcher for critical services

### Manual Verification Guide
1. Start `autobuddy`: `cargo run`
2. Stop a critical process (e.g. `redis`) or ensure it's not running.
3. Check Telegram: You will receive "Bodyguard Alert: redis is missing from duty! Checking the perimeter."

## 🏁 Phase 5 Complete
Bodyguard is now on duty.
