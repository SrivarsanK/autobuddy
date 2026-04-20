# Roadmap: autobuddy

- [v1.0 Milestone: Core Robustness & Viral Features](.planning/milestones/v1.0-ROADMAP.md) - Polished core engine and added "viral" intercepts.
- [v2.0 Milestone: Contextual Intelligence](.planning/milestones/v2.0-ROADMAP.md) - Active system guarding and smart build failure diagnosis.

## Milestone 3: Remote Governance (Complete)
The goal is to move from "Alert only" to "Interact" via Telegram.

- **Status:** Milestone 3 Complete.
- **Current Phase:** None (Ready for Milestone 4).

### Phase 7: The Controller (Remote Commands)
Allow basic machine commands via Telegram.
- [x] Implement command verification PIN/Auth
- [x] Add `/status` command to get real-time health
- [x] **UAT:** Sending `/status` to the bot returns current system usage and uptime.

### Phase 8: The Tactician (Personality)
Implement behavioral filters.
- [x] Add `buddy_mode` to config
- [x] Implement alert level filtering based on mode
- [x] Add `/mode <PIN> <MODE>` command
- [x] **UAT:** Sending `/mode PIN silent` filters out low-priority notifications.
