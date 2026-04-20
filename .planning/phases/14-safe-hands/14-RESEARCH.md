# Phase 14: Safe Hands - Research

Objective: Implement interactive confirmation for healing actions via Telegram.

## 1. Interaction Model
- **Challenge**: Standard Telegram bots can use Inline Keyboards for "Yes/No" interaction. However, managing state (which process to restart) across a stateless webhook/long-polling handler is complex.
- **Alternative**: Command-based confirmation. 
    1. Buddy detects crash: "Postgres crashed! Should I restart it? Reply with `/heal <pin> postgres`"
    2. User sends command.
    3. Buddy executes.
- **Improved Alternative (Contextual)**: Using `teloxide`'s state management or a simple "Pending Heal" queue. Given the simplicity of the current project, a command `/heal <pin> <name>` is the most robust and "audit-friendly" way.

## 2. Requirements for /heal command
- **PIN protection**: Reuse `master_pin`.
- **Validation**: Ensure the requested name is in `critical_processes`.
- **Mechanism**: Reuse the `Command::Mode` pattern in `src/telegram.rs`.

## 3. Workflow
1. Crash detected.
2. If `auto_heal` is **OFF**, send specialized message:
    - *"⚠️ CRASH: {name} is down. I'm standing by. Use `/heal <pin> {name}` to help me bring it back."*
3. User sends `/heal 1234 postgres`.
4. Bot receives, validates PIN, sends `HealAction { name }` event to main loop.
5. Main loop executes restart (refactoring the logic from Phase 13 into a reusable function).

## 4. Risks
- Delay in recovery: Manual healing is slow.
- **Solution**: We now have two modes: "Auto-Heal" (Phase 13) and "Manual-Heal" (Phase 14).
