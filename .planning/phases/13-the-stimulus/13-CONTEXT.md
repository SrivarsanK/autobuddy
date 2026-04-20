# Phase 13: The Stimulus - Context

**Gathered:** 2026-04-20
**Status:** Ready for planning

<domain>
## Phase Boundary
This phase implements the "Stimulus" — the actual action of restarting a failing service. It transitions autobuddy from a monitoring tool to an orchestration tool.
</domain>

<decisions>
## Implementation Decisions

### Auto-Heal Configuration
- Add `auto_heal: bool` to the `Watchers` struct in `src/config.rs`.
- Default to `false` in the code, but the installer can set it to `true` or user can enable it via `/mode`.

### Restart Mechanism
- Implementation will live in a new module or a helper in `watchers`.
- It will use `std::process::Command::new("systemctl").arg("restart").arg(name).output()`.
- We will log the output (stdout/stderr) of the restart attempt to journald for debugging.

### Integration Path
- The main event loop in `main.rs` will handle the `Event::ProcessCrash` specifically by checking the `auto_heal` flag.
</decisions>

<canonical_refs>
## Canonical References
- `ROADMAP.md` — Phase 13 goal.
- `src/config.rs` — Config structure to modify.
- `src/main.rs` — Event loop to modify.
</canonical_refs>
