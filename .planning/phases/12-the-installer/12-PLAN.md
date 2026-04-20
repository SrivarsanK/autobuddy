---
wave: 1
depends_on: []
files_modified: [autobuddy.service, scripts/install.sh]
autonomous: true
---

# Plan 12: The Installer

Goal: Standardize Linux deployment with a systemd unit and an automation script.

## Tasks

<task>
<read_first>
- .planning/phases/12-the-installer/12-CONTEXT.md
- .planning/phases/12-the-installer/12-RESEARCH.md
</read_first>
<action>
Create `autobuddy.service` in the project root.
Configure it with:
- `Type=notify`
- `ExecStart=/usr/local/bin/autobuddy`
- `WorkingDirectory=/etc/autobuddy`
- `Restart=on-failure`
- `WatchdogSec=10`
- `StandardOutput=journal`
- `StandardError=journal`
</action>
<acceptance_criteria>
- File `autobuddy.service` exists.
- File contains `Type=notify`, `WatchdogSec=10`, and `ExecStart=/usr/local/bin/autobuddy`.
</acceptance_criteria>
</task>

<task>
<read_first>
- .planning/phases/12-the-installer/12-CONTEXT.md
- autobuddy.service
</read_first>
<action>
Create `scripts/install.sh` and make it executable.
The script should:
1. Verify Linux OS.
2. Build binary: `cargo build --release`.
3. Create `/etc/autobuddy` (sudo).
4. Prompt for `TELEGRAM_BOT_TOKEN`, `TELEGRAM_CHAT_ID`, and `MASTER_PIN`.
5. Write initial `autobuddy.toml` to `/etc/autobuddy/`.
6. Copy binary to `/usr/local/bin/autobuddy`.
7. Copy `autobuddy.service` to `/etc/systemd/system/`.
8. Execute `systemctl daemon-reload`, `systemctl enable autobuddy`, and `systemctl start autobuddy`.
</action>
<acceptance_criteria>
- `scripts/install.sh` exists and is executable.
- Script contains build, configuration, and systemctl commands.
</acceptance_criteria>
</task>

## Verification Criteria

### Automated Tests
- None (deployment scripts are verified by inspection and manual test).

### Manual Verification
1. Run `./scripts/install.sh` on a Linux system (mock or real).
2. Check `systemctl status autobuddy`.
3. Check `journalctl -u autobuddy`.

## Goals backward verification (must_haves)
- [ ] Systemd unit file correctly configured for Notify/Watchdog.
- [ ] Installer script automates building, configuration, and service startup.
