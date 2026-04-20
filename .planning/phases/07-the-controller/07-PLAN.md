---
wave: 1
depends_on: []
files_modified: [src/telegram.rs, src/config.rs, autobuddy.toml]
autonomous: true
---

# Plan: Phase 7 - The Controller (Remote Status)

Implement secure status reporting via Telegram.

## Tasks

### 1. Update Config with Master PIN
<task>
<read_first>
- src/config.rs
- autobuddy.toml
</read_first>
<action>
Add `master_pin: String` to `Config` struct in `src/config.rs`.
Update `autobuddy.toml` with `master_pin = "buddy123"`.
</action>
<acceptance_criteria>
- `config.master_pin` is available at runtime.
- `autobuddy.toml` is updated.
</acceptance_criteria>
</task>

### 2. Implement Telegram Commands
<task>
<read_first>
- src/telegram.rs
</read_first>
<action>
1. Define `Command` enum using `teloxide::macros::BotCommands`.
2. Implement `/ping` (replies "I'm awake and watching your back!").
3. Implement `/status` (calculates uptime via `sysinfo` and returns health summary).
4. Update `TelegramBot::new` and the dispatch loop to handle commands.
</action>
<acceptance_criteria>
- Bot responds to `/ping`.
- Bot responds to `/status` with formatted uptime (e.g., "Uptime: 1h 23m").
- Non-command messages are ignored (to prevent spam).
</acceptance_criteria>
</task>

## Verification
- [ ] Run `autobuddy`.
- [ ] Send `/ping` to bot. Verify message.
- [ ] Send `/status` to bot. Verify uptime is displayed.
