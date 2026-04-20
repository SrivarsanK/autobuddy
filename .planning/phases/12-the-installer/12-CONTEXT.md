# Phase 12: The Installer - Context

**Gathered:** 2026-04-20
**Status:** Ready for planning

<domain>
## Phase Boundary
This phase delivers the standardized Linux installation experience. It bridges the gap between a "developer project" and a "system utility".

Key deliverables:
- A systemd unit file that leverages our `sd-notify` / `Watchdog` implementation.
- An interactive `install.sh` script for zero-friction setup.
</domain>

<decisions>
## Implementation Decisions

### Service Configuration
- **User:** The service should run as root or a dedicated `autobuddy` user. For simplicity in this version, we will default to `root` to allow easy reading of terminal history/processes, but provide a commented-out non-privileged alternative.
- **Paths:** 
    - Binary: `/usr/local/bin/autobuddy`
    - Config: `/etc/autobuddy/autobuddy.toml`
- **Type:** `notify` (leverages the code from Phase 10).

### Installer Behavior
- The script must detect if it's running on Linux.
- It should build the project in `--release` mode.
- It must guide the user through setting up the initial `autobuddy.toml` if it doesn't exist.
- It should handle `systemctl` commands to enable and start the service.

### the agent's Discretion
- The script name should be `install.sh` in the root or a `scripts/` directory.
- The systemd file should be a template replaced by the script during installation.
</decisions>

<canonical_refs>
## Canonical References
- `ROADMAP.md` — Phase 12 goals.
- `PROJECT.md` — Requirement SYS-04, SYS-05.
- `src/main.rs` — Reference for watchdog heartbeat timing.
</canonical_refs>

<specifics>
## Specific Ideas
- Add a "Test Message" step in the installer to verify Telegram connectivity before starting the daemon.
</specifics>
