---
wave: 1
depends_on: []
files_modified: [src/alert.rs, src/config.rs, src/telegram.rs, autobuddy.toml]
autonomous: true
---

# Plan: Phase 8 - The Tactician (Personality)

Implement alert filtering based on buddy mode.

## Tasks

### 1. Update Config with Buddy Mode
<task>
<read_first>
- src/config.rs
- autobuddy.toml
</read_first>
<action>
1. Add `BuddyMode` enum with `Silent`, `Normal`, `Chatty`.
2. Add `buddy_mode: BuddyMode` to `Config`.
3. Default to `Normal` in `autobuddy.toml`.
</action>
<acceptance_criteria>
- `config.buddy_mode` is available.
- `autobuddy.toml` has `buddy_mode = "normal"`.
</acceptance_criteria>
</task>

### 2. Implement Filtering in AlertEngine
<task>
<read_first>
- src/alert.rs
</read_first>
<action>
1. Add `mode: BuddyMode` to `AlertEngine`.
2. In `AlertEngine::process`, check `severity` against `mode`.
    - `silent`: Only `Severity::Critical`.
    - `normal`: `Severity::Warning` and `Severity::Critical`.
    - `chatty`: All.
3. Return `None` if the severity is below the threshold.
</action>
<acceptance_criteria>
- `AlertEngine` rejects Info alerts when in `silent` or `normal` mode.
- Unit tests verify filtering logic.
</acceptance_criteria>
</task>

### 3. Add /mode Command (Preview)
<task>
<read_first>
- src/telegram.rs
</read_first>
<action>
1. Add `/mode [PIN] [NAME]` to `Command` enum.
2. For this phase, the bot should just acknowledge the request and verify the PIN. 
    - *Note*: Changing the mode *live* across threads is complex; we will prioritize the filtering logic first and check for simple ways to update the engine if possible.
</action>
<acceptance_criteria>
- `/mode` command is recognized.
- In-place acknowledge: "Mode update requested...".
</acceptance_criteria>
</task>

## Verification
- [ ] Set `buddy_mode = "silent"` in toml.
- [ ] Run `autobuddy` and verify Info events (e.g. Sentinel) are not sent.
- [ ] Verify unit tests for `AlertEngine` severity filtering.
