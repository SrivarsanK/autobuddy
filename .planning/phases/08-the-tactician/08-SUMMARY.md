## Phase 8 Summary: The Tactician

### Objective
Implement buddy personality modes that control alert frequency.

### Key Changes
- **Personality Modes**: Added `BuddyMode` enum (`silent`, `normal`, `chatty`).
- **Alert Filtering**:
    - `silent`: Responds only to `Critical` events.
    - `normal`: Responds to `Warning` and `Critical`.
    - `chatty`: Responds to all events (`Info` level and above).
- **Core Improvements**:
    - Enhanced `Severity` enum with `PartialOrd` to allow efficient threshold comparisons.
    - Updated `AlertEngine` to enforce filtering logic based on the configured mode.
- **Remote Integration**:
    - Added `/mode <pin> <name>` command preview.
    - Updated `BotCommands` parsing logic to support multi-argument commands.

### Verification Results
- **Unit Tests**: Passed (8/8). Verified that `silent` mode correctly blocks `Warning` events and `normal` mode correctly blocks `Info` events.
- **Commit History**:
    - `4b7e8b1` feat(config): add BuddyMode enum to Config
    - `67e4449` feat(config): set default buddy_mode to normal in autobuddy.toml
    - `3f984e2` feat(alert): implement buddy_mode filtering in AlertEngine
    - `8d9b4a2` feat(telegram): add /mode command preview
    - `3367fe8` feat(personality): finalize BuddyMode filtering and /mode preview

## 🏁 Phase 8 Complete
`autobuddy` now has a personality.
