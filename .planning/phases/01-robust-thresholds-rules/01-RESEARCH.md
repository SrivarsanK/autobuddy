# Phase 1: Robust Thresholds & Rules - Research

## Implementation Strategy
The goal is to move from hardcoded alert thresholds (currently 80% CPU/90% RAM in `alert.rs`) to values loaded from `autobuddy.toml`.

### Component Analysis

1.  **`Config` (`config.rs`)**:
    *   Already contains `Thresholds` struct with `cpu_pct`, `ram_pct`, `disk_pct`.
    *   Need to ensure these fields are accessible and potentially passed to the `AlertEngine`.

2.  **`AlertEngine` (`alert.rs`)**:
    *   Currently takes `cooldown_secs`.
    *   Needs to be updated to either store the `Thresholds` config or take them as parameters during `process`.
    *   Recommendation: Pass `Thresholds` to `AlertEngine::new` or `process` to decouple logic from global config.

3.  **`SysHealthWatcher` (`syshealth.rs`)**:
    *   Polls every 30s.
    *   Sends `Event::SysHealth`.
    *   No changes strictly needed here if `AlertEngine` handles the comparison.

4.  **Logging**:
    *   Use `println!` or `eprintln!` for basic phase requirement of startup/shutdown logging, or integrate `log` crate if preferred (sticking to `println` for simplicity as per current patterns).

### Risks & Side Effects
*   **Cooldown logic**: Ensure that updating thresholds doesn't reset cooldown timers unexpectedly.
*   **Precision**: `sysinfo` returns f32, config uses f32. Exact matches should be avoided in favor of "greater than".

## Verification Strategy (Nyquist Dimension 8)
1.  **Unit Test**: Update `AlertEngine` tests to verify different thresholds trigger correctly.
2.  **Integration**: Run daemon with a very low threshold (e.g., 1.0% CPU) to verify Telegram alert triggers immediately.
3.  **Config**: Verify that changing `autobuddy.toml` and restarting the daemon changes the alert behavior.

## Validation Architecture
*   **Tests:** `alert.rs` should have a test module.
*   **Manual:** `cargo run` with `cpu_pct = 1.0` in `autobuddy.toml`.
