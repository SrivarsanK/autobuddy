# Phase 7: The Controller - Research

## Objective
Enable remote status reports and command verification.

## Implementation Details
1.  **Commands**: Define a `Command` enum using `teloxide::macros::BotCommands`.
    - `/status`: Return uptime, health, and watchers.
    - `/ping`: Check if buddy is awake.
2.  **Uptime**: `sysinfo::System::uptime()` returns uptime in seconds. Format as `Xd Xh Xm`.
3.  **Security**:
    - Add `master_pin` to `autobuddy.toml`. 
    - For this phase, the PIN is mainly for future-proofing and `/mode` switching (Phase 8). `/status` can be unrestricted for the configured `chat_id`.

## Logic Flow
1. Bot receives message.
2. If it matches a command, extract arguments.
3. Process and reply.
