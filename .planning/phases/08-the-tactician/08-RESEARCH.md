# Phase 8: The Tactician - Research

## Objective
Implement buddy personality modes that control alert frequency.

## Requirements
- Modes: `silent`, `normal` (default), `chatty`.
- `silent`: Severity >= Critical.
- `normal`: Severity >= Warning.
- `chatty`: Severity >= Info.

## Implementation Details
1.  **Config**: Add `buddy_mode: String` to the main `Config` struct.
2.  **AlertEngine**:
    - Add a `mode` field to `AlertEngine`.
    - In `process`, check the event's severity against the active `mode`.
    - If severity is below the mode's threshold, return `None`.
3.  **Telegram Command**:
    - Add `/mode` command to `telegram.rs`.
    - Syntax: `/mode [PIN] [SILENT/NORMAL/CHATTY]`.
    - Requires `PIN` verification against `master_pin`.

## Workflow
1. User sends `/mode secret silent`.
2. Bot verifies `secret`.
3. Bot updates internal state (will need a way to communicate back to the engine or reload config).
    - *Note*: For now, we'll implement the configuration part and the Engine filtering. Updating state live might require an `Arc<RwLock>` or similar if we want truly dynamic switching without restart.
