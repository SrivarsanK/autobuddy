---
wave: 1
depends_on: [Phase 14]
files_modified: [src/main.rs]
autonomous: true
---

# Plan 15: Post-Op

Goal: Verify service recovery after a restart attempt.

## Tasks

<task>
<read_first>
- src/main.rs
</read_first>
<action>
Modify the restart logic in `main.rs` (both auto-heal and manual) to trigger a background verification.
The logic should:
1. After a successful `systemctl restart` command:
2. `tokio::spawn` a task that:
    - Sleeps for 5 seconds.
    - Runs `systemctl is-active <name>`.
    - Sends a confirmation or failure alert to the user based on the result.
</action>
<acceptance_criteria>
- Verification task is spawned after restart.
- Task provides follow-up notification to Telegram.
- `cargo check` passes.
</acceptance_criteria>
</task>

<task>
<read_first>
- src/main.rs
</read_first>
<action>
Clean up any duplicate logic between the `ProcessCrash` (auto) and `HealRequest` (manual) match arms if possible.
</action>
<acceptance_criteria>
- No significant code duplication for restart/verify logic.
</acceptance_criteria>
</task>

## Verification Criteria

### Automated Tests
- None.

### Manual Verification
1. Trigger a manual or auto-heal.
2. Verify you get the "Restarted" message immediately.
3. Wait 5 seconds.
4. Verify you get the "Recovery Verified" (stable) or "Recovery Failed" (crash loop) message.

## Goals backward verification (must_haves)
- [ ] Autobuddy verifies that healed services remain active.
- [ ] Users are notified of the final recovery status.
