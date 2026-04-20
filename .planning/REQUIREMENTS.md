# Milestone 3 Requirements: Remote Governance

## 1. Remote Controller (The Controller)
- **Status Inquiry**: Bot responds to `/status` with System Uptime, CPU/RAM, and a list of active Bodyguards.
- **PIN Verification**: Sensitive commands (to be defined in MS3) require a matching PIN from `autobuddy.toml`.

## 2. Personality Engine (The Tactician)
- **Frequency Regulation**: AlertEngine filters events based on `buddy_mode`:
    - `silent`: Severity >= Critical.
    - `normal`: Severity >= Warning.
    - `chatty`: All events.
- **Dynamic Switching**: User can change mode via `/mode [PIN] [NAME]`.

## 3. Success Metrics (UAT)
- [ ] User sends `/status`. Bot returns uptime and health.
- [ ] User sends `/mode secret silent`. Bot confirms mode change if PIN matches.
- [ ] In `silent` mode, Informational Sentinel alerts are suppressed.
