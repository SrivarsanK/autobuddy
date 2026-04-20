---
wave: 1
depends_on: []
files_modified: [src/watchers/terminal.rs, src/main.rs, src/alert.rs]
autonomous: true
---

# Plan: Phase 2 - The Interceptor

Implement dangerous command detection logic.

## Tasks

### 1. Update TerminalWatcher to monitor commands
<task>
<read_first>
- src/watchers/terminal.rs
- src/config.rs
</read_first>
<action>
Update `TerminalWatcher` to accept `Rules` in its constructor (or `run` method if necessary).
Implement logic to "detect" commands. For this version, we'll implement a simple polling of the shell history file or a simulated stream for demo purposes.
Specifically: If a command contains any string from `rules.dangerous_commands`, send a `DangerousCommand` event.
</action>
<acceptance_criteria>
- `TerminalWatcher` is updated to use config rules.
- Logic for detecting substrings in commands is implemented.
</acceptance_criteria>
</task>

### 2. Update main.rs to pass Rules to Watcher
<task>
<read_first>
- src/main.rs
</read_first>
<action>
Modify `main.rs` to pass `config.rules.clone()` when spawning `TerminalWatcher`.
</action>
<acceptance_criteria>
- `TerminalWatcher` instantiation in `main.rs` includes the `rules` parameter.
- Build passes.
</acceptance_criteria>
</task>

### 3. Verification of Dangerous Command Alert
<task>
<read_first>
- src/alert.rs
</read_first>
<action>
Ensure `AlertEngine::process` handles `Event::DangerousCommand` correctly (it already does, but verify logic against `Severity::Critical`).
Add a unit test to `src/alert.rs` specifically for dangerous command events.
</action>
<acceptance_criteria>
- Unit test for `DangerousCommand` in `alert.rs` passes.
- Alert message includes the command string.
</acceptance_criteria>
</task>

## Verification
- [ ] Set `dangerous_commands = ["test_danger"]` in `autobuddy.toml`.
- [ ] Simulate the "test_danger" command (via history injection or mock trigger).
- [ ] Verify a CRITICAL alert is received on Telegram.
