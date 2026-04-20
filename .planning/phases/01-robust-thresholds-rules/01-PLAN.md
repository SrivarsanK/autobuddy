---
wave: 1
depends_on: []
files_modified: [src/config.rs, src/alert.rs, src/main.rs]
autonomous: true
---

# Plan: Phase 1 - Robust Thresholds & Rules

Connect the alert engine to the config file and add lifecycle logging.

## Tasks

### 1. Update AlertEngine to accept Thresholds
<task>
<read_first>
- src/alert.rs
- src/config.rs
</read_first>
<action>
Modify `AlertEngine` struct and `new` method in `src/alert.rs` to store `Thresholds` from `src/config.rs`.
Update the `process` method to compare against `self.thresholds` instead of the hardcoded `80.0` and `90.0` values.
</action>
<acceptance_criteria>
- `AlertEngine` struct has a `thresholds: Thresholds` field.
- `AlertEngine::process` uses `self.thresholds.cpu_pct` and `self.thresholds.ram_pct`.
- Code compiles.
</acceptance_criteria>
</task>

### 2. Update main.rs to pass Thresholds and handle Shutdown
<task>
<read_first>
- src/main.rs
</read_first>
<action>
Update `main.rs` to pass `config.thresholds.clone()` to `AlertEngine::new`.
Wrap the main loop or use `tokio::select!` to listen for `tokio::signal::ctrl_c()`.
Print "autobuddy daemon shutting down..." when the signal is received.
</action>
<acceptance_criteria>
- `AlertEngine::new` call in `main.rs` includes `config.thresholds`.
- Pressing CTRL+C during execution prints the shutdown message.
</acceptance_criteria>
</task>

### 3. Verification of Alert Logic
<task>
<read_first>
- src/alert.rs
</read_first>
<action>
Add a unit test module at the end of `src/alert.rs` to verify that `AlertEngine` correctly identifies alert conditions based on varying thresholds.
</action>
<acceptance_criteria>
- `cargo test` passes for `alert::tests`.
- Test covers both "below threshold" (no alert) and "above threshold" (alert) scenarios.
</acceptance_criteria>
</task>

## Verification
- [ ] Run `cargo run`. Verify "autobuddy daemon starting..." is visible.
- [ ] Temperately set `cpu_pct = 1.0` in `autobuddy.toml`. Verify Telegram alert is received.
- [ ] Stop with CTRL+C. Verify "autobuddy daemon shutting down..." is visible.
