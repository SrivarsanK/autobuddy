# Phase 1: Robust Thresholds & Rules - Context

**Gathered:** 2026-04-20
**Status:** Ready for planning
**Source:** Analyzed from project requirements and current codebase state.

<domain>
## Phase Boundary
Deliver a robust alerting system where thresholds are configurable via `autobuddy.toml`. Add basic lifecycle logging (startup/shutdown).

</domain>

<decisions>
## Implementation Decisions

### Configuration Connection
- [x] **Locked:** `AlertEngine` must use `Config.thresholds` instead of hardcoded values.
- [x] **Locked:** Thresholds should be passed to `AlertEngine` at creation or processing time.

### Lifecycle Logging
- [x] **Locked:** Print "autobuddy daemon starting..." on init (already present in `main.rs`, but ensure it's robust).
- [x] **Locked:** Print "autobuddy daemon shutting down..." on exit (requires handling CTRL+C gracefully).

### the agent's Discretion
- Exact format of the Telegram messages for alerts.
- Whether to use a dedicated logging crate or stick to `println!`.

</decisions>

<canonical_refs>
## Canonical References

### Logic
- `src/alert.rs` — Threshold check logic.
- `src/config.rs` — Config structures.
- `src/main.rs` — Daemon entry point and lifecycle.

### Data
- `autobuddy.toml` — Source of truth for thresholds.

</canonical_refs>

<specifics>
## Specific Ideas
- Use `tokio::signal::ctrl_c` for graceful shutdown logging.

</specifics>

<deferred>
## Deferred Ideas
- Dynamic config reloading (SIGHUP or file watch) — deferred to a future robustness phase.

</deferred>
