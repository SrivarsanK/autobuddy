# Roadmap: autobuddy

## Milestone 1: Core Robustness & Viral Features
The goal is to polish the existing features and add the "viral" intercepts that make autobuddy a true companion.

### Phase 1: Robust Thresholds & Rules
Connect the engine to the config so users can actually customize their buddy.
- [x] Fix `AlertEngine` to use `Config` thresholds
- [x] Log daemon startup/shutdown events
- [x] **UAT:** High CPU (simulated) triggers Telegram alert based on `autobuddy.toml` value.

### Phase 2: The Interceptor (Dangerous Commands)
Wait for shell events and alert on danger.
- [x] Implement `TerminalWatcher` logic
- [x] Add substring matching for `dangerous_commands`
- [x] **UAT:** Running a command on the "naughty list" triggers a CRITICAL alert.

### Phase 3: Build & Crash Watchers
Watch the dev loop.
- [x] Implement `BuildWatcher` (logfile tailing)
- [x] Implement error parsing for Telegram alerts
- [x] **UAT:** Build error (simulated via log) sends a Telegram notification.

## Milestone 2: Contextual Intelligence
Move beyond threshold monitoring into proactive "buddy" behaviors.

### Phase 4: The Sentinel (SSH & Network)
Implement network/access monitoring.
- [ ] Implement SSH connection watcher
- [ ] **UAT:** New SSH login triggers an immediate Informational alert.

### Phase 5: The Bodyguard (Process Monitoring)
Protect critical components of the system.
- [ ] Add `ProcessCrash` watcher for configured critical processes
- [ ] **UAT:** Stopping a critical process (e.g. `postgres`) triggers a WARNING alert.

### Phase 6: The Oracle (Smart Fixes)
Enhanced healing.
- [ ] Implement "Fix Suggestions" based on build errors
- [ ] **UAT:** Build failure alert includes a "Try this:..." repair suggestion.
