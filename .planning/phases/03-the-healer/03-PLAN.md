---
wave: 1
depends_on: []
files_modified: [src/watchers/mod.rs, src/watchers/build.rs, src/main.rs, src/alert.rs]
autonomous: true
---

# Plan: Phase 3 - The Healer

Implement build failure detection and alerting.

## Tasks

### 1. Implement BuildWatcher
<task>
<read_first>
- src/watchers/mod.rs
</read_first>
<action>
Create `src/watchers/build.rs`. Implement a `BuildWatcher` that monitors `build.log` for error keywords.
Expose it in `src/watchers/mod.rs`.
</action>
<acceptance_criteria>
- `src/watchers/build.rs` exists and implements `Watcher`.
- Trait methods `name` and `run` are implemented.
- `mod build;` is added to `src/watchers/mod.rs`.
</acceptance_criteria>
</task>

### 2. Connect BuildWatcher in main.rs
<task>
<read_first>
- src/main.rs
- src/config.rs
</read_first>
<action>
Update `main.rs` to spawn `BuildWatcher` if `config.watchers.build` is true.
</action>
<acceptance_criteria>
- `BuildWatcher` is spawned in the daemon startup sequence.
- Code compiles.
</acceptance_criteria>
</task>

### 3. Handle BuildFailure events in AlertEngine
<task>
<read_first>
- src/alert.rs
</read_first>
<action>
Update `AlertEngine::process` to handle `Event::BuildFailure`.
Add a unit test to verify build failure alerting.
</action>
<acceptance_criteria>
- `process` match arm for `BuildFailure` returns a descriptive message.
- Unit test passes.
</acceptance_criteria>
</task>

## Verification
- [ ] Create `build.log` with "error: failed to compile".
- [ ] Verify Telegram alert is received.
