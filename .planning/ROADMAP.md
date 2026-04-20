# Roadmap: autobuddy

- [v1.0 Milestone: Core Robustness & Viral Features](.planning/milestones/v1.0-ROADMAP.md) - Polished core engine and added "viral" intercepts.
- [v2.0 Milestone: Contextual Intelligence](.planning/milestones/v2.0-ROADMAP.md) - Active system guarding and smart build failure diagnosis.

## Milestone 3: Remote Governance (Planned)
The goal is to move from "Alert only" to "Interact" via Telegram.

### Phase 7: The Controller (Remote Commands)
Allow basic machine commands via Telegram.
- [ ] Implement command verification PIN/Auth
- [ ] Add `/status` command to get real-time health
- [ ] **UAT:** Sending `/cpu` to the bot returns current system usage.

### Phase 8: The Tactician (Personality)
Implement behavioral filters.
- [ ] Add `buddy_mode` to config
- [ ] Implement event filtering in `AlertEngine`
- [ ] **UAT:** In `silent` mode, INFO alerts are not sent to Telegram.
