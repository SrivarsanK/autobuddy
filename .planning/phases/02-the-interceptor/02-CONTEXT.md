# Phase 2: The Interceptor - Context

**Gathered:** 2026-04-20
**Status:** Ready for planning
**Source:** Phase 2 Roadmap & Research.

<domain>
## Phase Boundary
Implement the "Dangerous Command" watcher. It must detect restricted strings in commands and trigger critical alerts via Telegram.

</domain>

<decisions>
## Implementation Decisions

### Interception Method
- [x] **Locked:** For the demo, `TerminalWatcher` will read from a "command buffer" or tail the user's shell history (e.g., `~/.bash_history` or `~/.zsh_history`).
- [x] **Locked:** Use `autobuddy.toml`'s `dangerous_commands` list for matching.

### Alerting
- [x] **Locked:** Use `Severity::Critical` for dangerous commands.
- [x] **Locked:** Optionally support `block_on_match` (though blocking is harder, we will at least flag it in the alert).

### the agent's Discretion
- Which shell history file to prioritize (auto-detect based on env).
- Polling frequency for the history file.

</decisions>

<canonical_refs>
## Canonical References
- `src/watchers/terminal.rs` — Implementation of the watcher.
- `src/config.rs` — `Rules` structure.
- `src/event.rs` — `DangerousCommand` variant.

</canonical_refs>

<specifics>
## Specific Ideas
- Allow a "test command" to be injected via a command line argument or a specific trigger to demonstrate the feature without actually running `rm -rf /`.

</specifics>

<deferred>
## Deferred Ideas
- Pre-execution blocking (requires kernel probes or deep shell integration).
- Multi-shell support (pwsh, fish, etc) if significantly different.

</deferred>
