---
phase: 11
plan: 11-01
subsystem: state
tags: ["persistence", "serde", "configuration"]
requires: ["core-integration"]
provides: ["state-persistence"]
affects: ["Cargo.toml", "src/config.rs", "src/main.rs", "src/telegram.rs"]
requirements: ["PER-01", "PER-02", "PER-03", "PER-04"]
---

# Phase 11 Plan 01: Core State Persistence

Implement atomic configuration persistence to allow runtime changes (like buddy mode) to survive restarts.

## Research Findings
- **Atomic Writes:** To prevent config corruption during a crash/power loss, we must use a "Write-To-Temp -> Rename" pattern. The `tempfile` crate is the industry standard for this in Rust.
- **Serialization:** `Config` already derives `Deserialize`. We need to add `Serialize` and move to `toml::to_string_pretty` for clean human-readable output.

## Tasks

<task wave="1" type="execute">
<name>Enable Serialization & Serialization Logic</name>
<description>
Update Cargo.toml with `tempfile` and `serde` features. Update `config.rs` to derive `Serialize` and implement a safe `save()` method.
</description>
<read_first>
- src/config.rs
- Cargo.toml
</read_first>
<action>
1. Add `tempfile = "3.8"` to Cargo.toml.
2. Update `serde` features to `["derive"]` if not already fully there.
3. In `src/config.rs`:
   - Change `use serde::Deserialize` to `use serde::{Deserialize, Serialize}`.
   - Add `Serialize` to all Config structs/enums.
   - Add `use std::io::Write;` and `use tempfile::NamedTempFile;`.
   - Implement `pub fn save<P: AsRef<std::path::Path>>(&self, path: P) -> anyhow::Result<()>`.
     - Use `NamedTempFile::new_in()` (prefer the same directory as the config to ensure same-filesystem rename).
     - Write `toml::to_string_pretty(self)`.
     - `persist(path)`.
</action>
<acceptance_criteria>
- Cargo.toml contains `tempfile`.
- `config.rs` contains `save` method using `tempfile`.
- `cargo check` passes.
</acceptance_criteria>
</task>

<task wave="2" type="execute">
<name>State Accessor in AlertEngine</name>
<description>
Allow `AlertEngine` or a shared state manager to update the mode in memory so it can be saved.
</description>
<read_first>
- src/alert.rs
</read_first>
<action>
1. Add `pub fn set_mode(&mut self, mode: BuddyMode)` to `AlertEngine`.
2. Add `pub fn mode(&self) -> BuddyMode` to `AlertEngine`.
</action>
<acceptance_criteria>
- `AlertEngine` has mode getter/setter.
</acceptance_criteria>
</task>

<task wave="3" type="execute">
<name>Persist Mode Changes via Telegram</name>
<description>
Hook the `/mode` command to update the configuration file.
</description>
<read_first>
- src/main.rs
- src/telegram.rs
</read_first>
<action>
1. In `main.rs`:
   - The `rx.recv()` loop currently handles events.
   - We need a way for the Telegram REPL task to communicate mode changes back to the main thread OR access a thread-safe `Arc<RwLock<Config>>`.
   - *Decision:* Wrap `Config` in `Arc<RwLock<Config>>` and `AlertEngine` in something accessible, OR use a separate command channel.
   - Let's go with `Arc<RwLock<Config>>` for the configuration at minimum.
2. Update `Command::Mode` handler in `telegram.rs` to actually change the state.
   - This requires passing the shared config to `handle_command`.
</action>
<acceptance_criteria>
- `/mode silent` command updates the `autobuddy.toml` file.
</acceptance_criteria>
</task>

## Verification

### Automated Tests
1. Run `cargo check`.
2. Manual integration test: Start daemon, send `/mode silent`, restart daemon, check if it starts in `silent` mode.

### Success Criteria
- [ ] Config file updates atomically on mode change.
- [ ] No data loss on partial writes (crash safety).
- [ ] Log output reflects the updated mode immediately.
