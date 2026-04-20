# Milestone 2: Contextual Intelligence

The goal is to move beyond simple threshold monitoring into more proactive and intelligent "buddy" behaviors.

## User Questions
1.  **SSH Detection**: When a new SSH session starts, should we just notify or also block if from an unknown IP?
    *   *Implicit Decision:* Just notify for now to avoid locking out the user.
2.  **AI Fixes**: Should we actually integrate an LLM at this stage to "fix" build errors?
    *   *Implicit Decision:* We will implement a "Smart Diagnosis" feature that proposes a fix based on common error patterns and logs.
3.  **Process Monitoring**: Which processes are considered "critical"?
    *   *Implicit Decision:* Allow users to define a `critical_processes` list in the config.

## Requirements

### R1: SSH Watcher
- [ ] Monitor `/var/log/auth.log` or similar on Linux, or Event Viewer on Windows for new SSH connections.
- [ ] Send an alert containing the remote IP and username.

### R2: Critical Process Watcher
- [ ] Monitor a user-defined list of process names.
- [ ] Alert immediately if any process on the list is not running (crash or stopped).

### R3: Smart Diagnosis (The Healer V2)
- [ ] When a `BuildFailure` occurs, analyze the log snippet.
- [ ] Compare against a local database of "Common Fixes" or use an LLM API if configured.
- [ ] Alert includes: "I think I know what's wrong. Try running `cargo update`?".

## Roadmap Update (Milestone 2)

### Phase 4: The Sentinel (SSH & Network)
Implement network/access monitoring.
- [ ] SSH connection watcher.
- [ ] (Optional) Outbound request spikes.

### Phase 5: The Bodyguard (Process Monitoring)
Protect critical components of the system.
- [ ] Process existence checks for `critical_processes`.
- [ ] Memory leak detection for specific apps.

### Phase 6: The Oracle (Smart Fixes)
Enhanced healing.
- [ ] Fix database implementation (mapping error messages to fixes).
- [ ] Integration of "Repair Proposals" in Telegram.
