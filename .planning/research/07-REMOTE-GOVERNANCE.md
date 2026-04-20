# Milestone 3: Remote Governance - Research

## Objective
Enable remote interaction via Telegram with secure PIN-based execution.

## Requirements Mapping
- **Security**: PIN-based verification. User must provide a configured PIN to execute sensitive commands (e.g., `/reboot`).
- **Personality**: Frequency-based modes.
    - `Silent`: Only Critical.
    - `Normal`: Warning & Critical (Default).
    - `Chatty`: Info, Warning, & Critical.
- **Status Stats**: Uptime is the primary metric for the `/status` report.

## Technical Feasibility
1.  **Teloxide Commands**: Use the `Command` attribute in Teloxide to define `/status`, `/ping`, and `/mode`.
2.  **PIN Logic**: 
    - Store a hashed or plain-text `master_pin` in `autobuddy.toml`.
    - Verification: `engine` checks if the received token matches config.
3.  **Uptime Calculation**: `sysinfo` provides system uptime in seconds.

## Scope
- [Phase 7]: The Controller (Remote Status & PIN)
- [Phase 8]: The Tactician (Remote Configuration Change)
