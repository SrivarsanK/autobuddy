---
wave: 1
depends_on: []
files_modified: [src/config.rs, src/watchers/mod.rs, src/watchers/process.rs, src/main.rs, src/alert.rs, autobuddy.toml]
autonomous: true
---

# Plan: Phase 5 - The Bodyguard (Process Monitoring)

Ensure critical applications are always running.

## Tasks

### 1. Update Config for Critical Processes
<task>
<read_first>
- src/config.rs
- autobuddy.toml
</read_first>
<action>
Add `critical_processes: Vec<String>` to the `Watchers` struct in `src/config.rs`.
Update `autobuddy.toml` with a sample list: `critical_processes = ["postgres", "nginx", "redis"]`.
Set `process = true` in toml.
</action>
<acceptance_criteria>
- `config.watchers.critical_processes` exists and is populated from toml.
- `autobuddy.toml` is updated.
- Code compiles.
</acceptance_criteria>
</task>

### 2. Implement ProcessWatcher
<task>
<read_first>
- src/watchers/mod.rs
- src/watchers/syshealth.rs (for sysinfo usage)
</read_first>
<action>
Create `src/watchers/process.rs`.
Implement `ProcessWatcher` that:
1. Iterates through `critical_processes`.
2. Checks `sysinfo` to see if a process with that name exists.
3. If missing, emits `Event::ProcessCrash`.
Add `mod process` to `src/watchers/mod.rs`.
</action>
<acceptance_criteria>
- `src/watchers/process.rs` implemented.
- Polling interval set to 5 seconds.
- Correctly detects missing processes.
</acceptance_criteria>
</task>

### 3. Connect Bodyguard in main.rs
<task>
<read_first>
- src/main.rs
</read_first>
<action>
Update `main.rs` to spawn `ProcessWatcher` if `config.watchers.process` is true, passing the `critical_processes` list.
</action>
<acceptance_criteria>
- `ProcessWatcher` spawned during daemon startup.
- Code compiles.
</acceptance_criteria>
</task>

### 4. Handle Process Failure in AlertEngine
<task>
<read_first>
- src/alert.rs
</read_first>
<action>
Update `AlertEngine::process` to handle `Event::ProcessCrash`.
Add a unit test for process failure alerting.
</action>
<acceptance_criteria>
- `AlertEngine` returns a "Bodyguard Alert" message for missing processes.
- Unit test passes.
</acceptance_criteria>
</task>

## Verification
- [ ] List `postgres` as a critical process.
- [ ] Ensure `postgres` is not running.
- [ ] Verify Telegram notification says: "Bodyguard Alert: postgres is missing from duty!"
