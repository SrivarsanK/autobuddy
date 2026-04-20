# Roadmap - Milestone 5: The Healer

**3 phases** | **3 requirements mapped** | 100% Coverage

## Timeline

| Phase | Name | Goal | Requirements | Status |
|-------|------|------|--------------|--------|
| 13 | **The Stimulus** | Implement basic auto-restart logic | HEAL-01 | ✅ Complete |
| 14 | **Safe Hands** | Interactive Telegram confirmation | HEAL-02 | ✅ Complete |
| 15 | **Post-Op** | Recovery verification and reporting | HEAL-03 | ✅ Complete |

---

## Phase Details

### Phase 13: The Stimulus
**Goal:** Add a `heal` flag to critical processes and implement the logic to attempt restarts.
- **Tasks:**
    - Update `Config` to include `auto_heal: bool` for critical processes.
    - Implement `systemctl restart` call using `std::process::Command`.
- **Success Criteria:**
    - Killing `postgres` (monitored) results in an automatic system restart attempt.

### Phase 14: Safe Hands
**Goal:** Prevent unintended restarts by adding a confirmation step.
- **常规:**
    - Add `HealChoice` event.
    - Implement Telegram "Yes/No" callback buttons (if supported by context) or simple command reply.
- **Success Criteria:**
    - Daemon pauses after crash and waits for user's command before restarting.

### Phase 15: Post-Op
**Goal:** Close the feedback loop.
- **Tasks:**
    - Verify process is running after restart attempt.
    - Notify user of success or failure.
- **Success Criteria:**
    - User receives "Postgres recovered successfully!" message.
