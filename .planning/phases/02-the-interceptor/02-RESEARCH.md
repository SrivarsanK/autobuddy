# Phase 2: The Interceptor - Research

## Objective
Detect dangerous shell commands and trigger alerts.

## Research Findings
Monitoring terminal commands in real-time is OS-specific and non-trivial without shell extensions (like a `.zshrc` hook). For our "Viral" demo, we want to simulate or implement a lightweight mechanism.

### Options:
1.  **Tail Shell History**: Simple but delayed.
2.  **Process Monitoring**: Watch for `sh`, `bash`, `cmd` children. Hard to get full arguments reliably on all OS without `ptrace` or similar.
3.  **Local Socket/Pipe**: `autobuddy` opens a pipe/socket, and a shell function `alias` or `preexec` hook sends the command to it.
    *   *Decision:* For this phase, we will implement a "mock" intercept mechanism that listens for commands sent via a simple trigger file or local socket, as full system-wide shell-agnostic interception is out of scope for a single phase.
    *   *Refined Goal:* Implement a watcher that monitors a specific "command stream" (e.g., a named pipe or a standard history file location based on common shell defaults).

### Matching Logic
- The `Rules` config in `autobuddy.toml` contains `dangerous_commands` (regex or substring).
- `AlertEngine` already has `Event::DangerousCommand` variant.
- We need to connect the `Rules` config to `TerminalWatcher` or `AlertEngine`.

### Implementation Detail
- Update `Rules` to be passed where needed.
- `TerminalWatcher` should use `Rules` to filter detected commands.

## Validation Architecture
- **Simulated Test:** Create a script that "sends" a dangerous command to the watcher.
- **Alert:** Ensure `Severity::Critical` is used.
