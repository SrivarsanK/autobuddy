---
wave: 1
depends_on: [Phase 13]
files_modified: [src/event.rs, src/telegram.rs, src/main.rs]
autonomous: true
---

# Plan 14: Safe Hands

Goal: Add interactive confirmation for healing actions via a new `/heal` command.

## Tasks

<task>
<read_first>
- src/event.rs
</read_first>
<action>
Add `HealRequest { name: String }` to the `Event` enum in `src/event.rs`.
</action>
<acceptance_criteria>
- `Event` enum contains `HealRequest`.
</acceptance_criteria>
</task>

<task>
<read_first>
- src/telegram.rs
</read_first>
<action>
1. Add `Heal { pin: String, name: String }` to `Command` enum with a descriptive message.
2. Update `handle_command` to validate the PIN and `tx.send(Event::HealRequest { name })`.
</action>
<acceptance_criteria>
- `/heal` command integrated and PIN-protected.
- `cargo check` passes.
</acceptance_criteria>
</task>

<task>
<read_first>
- src/main.rs
</read_first>
<action>
1. Extract `systemctl restart` logic into an async helper or closure.
2. Update the `ProcessCrash` handler: If `auto_heal` is false, update the alert message to include: *"Use `/heal [pin] [name]` to restart."*
3. Add a match arm for `Event::HealRequest { name }` that triggers the restart logic.
</action>
<acceptance_criteria>
- `main.rs` handles `HealRequest`.
- Manual heal attempts are logged and reported.
</acceptance_criteria>
</task>

## Verification Criteria

### Automated Tests
- None.

### Manual Verification
1. Disable `auto_heal` in config.
2. Kill a critical process.
3. Verify the Telegram alert includes the `/heal` instruction.
4. Send `/heal <pin> <name>` and verify the process restarts and a confirmation message is sent.

## Goals backward verification (must_haves)
- [ ] Users can manually authorize a service restart via Telegram command.
- [ ] Manual healing is PIN-protected.
