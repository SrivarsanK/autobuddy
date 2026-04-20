# Roadmap - Milestone 4: The Keeper

**3 phases** | **9 requirements mapped** | 100% Coverage

## Timeline

| Phase | Name | Goal | Requirements | Status |
|-------|------|------|--------------|--------|
| 10 | **Core Integration** | Logging and systemd signaling | SYS-01, SYS-02, SYS-03 | ⚡ In Queue |
| 11 | **Memory & State** | Persistence for runtime config changes | PER-01, PER-02, PER-03, PER-04 | 💤 Pending |
| 12 | **The Installer** | Standardized Linux deployment | SYS-04, SYS-05 | 💤 Pending |

---

## Phase Details

### Phase 10: Core Integration
**Goal:** Hook the daemon into the Linux service lifecycle.
- **Tasks:**
    - Add `log`, `sd-notify`, and `systemd-journal-logger` to `Cargo.toml`.
    - Initialize `JournalLog` in `main.rs`.
    - Signal `NotifyState::Ready` after bot startup.
    - Implement background watchdog heartbeat task.
- **Success Criteria:**
    - `journalctl -u autobuddy` shows correctly leveled logs.
    - `systemctl status autobuddy` shows `active (running)` and `Ready` status.

### Phase 11: Memory & State
**Goal:** Allow the buddy to "remember" mode changes.
- **Tasks:**
    - Implement `serde` serialization for `Config`.
    - Add `Config::save()` with atomic file writing using `tempfile`.
    - Trigger `save()` when `/mode` command is received.
- **Success Criteria:**
    - Changing mode to `silent` via Telegram persists after restarting the process.
    - `autobuddy.toml` is updated automatically.

### Phase 12: The Installer
**Goal:** Easy deployment.
- **Tasks:**
    - Create `autobuddy.service` template.
    - Write `scripts/install.sh` to automate setup.
- **Success Criteria:**
    - A clean install takes less than 30 seconds.
    - Systemd unit correctly spawns the binary with the right config path.
