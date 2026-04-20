---
phase: 1
name: robust-thresholds-rules
date: 2026-04-20
---

# Validation Strategy - Phase 1

This phase is successful if thresholds from `autobuddy.toml` are respected and the daemon logs its lifecycle.

## Dimension 8: Verification Loop

### 1. Automated Tests
- **AlertEngine Logic:** Create unit tests in `src/alert.rs` that use custom `Thresholds` to ensure correct `Option<(String, Severity)>` is returned.
- **Config Parsing:** Ensure `autobuddy.toml` is read correctly into the structures.

### 2. Manual Verification (UAT)
- **Startup Log:** Run `cargo run` and verify "autobuddy daemon starting..." appears.
- **Shutdown Log:** Press CTRL+C and verify "autobuddy daemon shutting down..." appears.
- **Threshold Alert:** 
    1. Set `cpu_pct = 1.0` in `autobuddy.toml`.
    2. Run daemon.
    3. Verify Telegram alert "CPU high" is received.
    4. Reset `cpu_pct` to `85.0`.

### 3. Build & Lint
- `cargo check` must pass with no new warnings.
- `cargo clippy` should be clean.
