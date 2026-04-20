# Phase 14: Safe Hands - Context

**Gathered:** 2026-04-20
**Status:** Ready for planning

<domain>
## Phase Boundary
This phase introduces "Active Governance" to the healing process. It ensures that the user is always in control when auto-healing is disabled, or when a manual override is needed.
</domain>

<decisions>
## Implementation Decisions

### The /heal Command
- Add `Heal { pin: String, name: String }` to the `Command` enum in `src/telegram.rs`.
- This command will dispatch a new `Event::HealAction { name: String }`.

### Internal Refactoring
- Move the `systemctl restart` logic from the `main.rs` match arm into a helper function `fn attempt_heal(name: &str, bot: &TelegramBot, config: &Config)`.
- This function will be called by both:
    1. The auto-heal logic (if enabled).
    2. The manual `/heal` command.

### User Experience
- Update the crash alert message to include instructions for the `/heal` command if `auto_heal` is disabled.
</decisions>

<canonical_refs>
## Canonical References
- `ROADMAP.md` — Phase 14 goals.
- `src/telegram.rs` — Command definition.
- `src/main.rs` — Event handling logic.
</canonical_refs>
