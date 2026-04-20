---
wave: 1
depends_on: []
files_modified: [src/config.rs, src/watchers/mod.rs, src/watchers/sentinel.rs, src/main.rs, src/alert.rs, autobuddy.toml]
autonomous: true
---

# Plan: Phase 4 - The Sentinel (SSH & Network)

Implement access monitoring with a focus on SSH login detection.

## Tasks

### 1. Update Config Schema
<task>
<read_first>
- src/config.rs
- autobuddy.toml
</read_first>
<action>
Add `sentinel: bool` to the `Watchers` struct in `src/config.rs`.
Update `autobuddy.toml` to include the `sentinel` key.
</action>
<acceptance_criteria>
- `Watchers` struct has a `sentinel` field.
- `autobuddy.toml` contains `sentinel = true`.
- Code compiles.
</acceptance_criteria>
</task>

### 2. Implement SentinelWatcher
<task>
<read_first>
- src/watchers/mod.rs
- src/watchers/build.rs (as reference)
</read_first>
<action>
Create `src/watchers/sentinel.rs`.
Implement a `SentinelWatcher` that monitors an `auth.log` file in the project root.
Look for the pattern: `Accepted password for [USER] from [IP]`.
Emit `Event::Custom`.
Add `mod sentinel` to `src/watchers/mod.rs`.
</action>
<acceptance_criteria>
- `src/watchers/sentinel.rs` implemented.
- Correctly parses username and IP from a sample line.
- Sends an `Event::Custom` to the transmitter.
</acceptance_criteria>
</task>

### 3. Setup Sentinel in main.rs
<task>
<read_first>
- src/main.rs
</read_first>
<action>
Update `main.rs` to import `SentinelWatcher` and spawn it if enabled in config.
</action>
<acceptance_criteria>
- `SentinelWatcher` is spawned during daemon startup.
- Code compiles.
</acceptance_criteria>
</task>

### 4. Handle Sentinel Alerts in AlertEngine
<task>
<read_first>
- src/alert.rs
</read_first>
<action>
Add a match arm for `Event::Custom` where `watcher == "sentinel"` in `AlertEngine::process`.
Add a unit test for sentinel alerts.
</action>
<acceptance_criteria>
- `AlertEngine` returns a properly formatted message for sentinel events.
- Unit test passes.
</acceptance_criteria>
</task>

## Verification
- [ ] Create `auth.log`.
- [ ] Append `Apr 20 19:45:00 host sshd[123]: Accepted password for root from 192.168.1.1 port 22`.
- [ ] Verify Telegram notification displays the Sentinel alert.
