# Phase 15: Post-Op - Context

**Gathered:** 2026-04-20
**Status:** Ready for planning

<domain>
## Phase Boundary
This phase closes the healing loop. It ensures "Mission Accomplished" is only said when the service is actually stable.
</domain>

<decisions>
## Implementation Decisions

### Non-blocking Verification
- When a restart is triggered (automatic or manual), we will spawn a background check.
- The check will wait 5 seconds and then verify the service status via `systemctl is-active`.

### Notification Strategy
- Success: *"🛡️ Recovery Verified: {name} is stable and active."*
- Failure: *"🚨 Recovery Failed: {name} went down again. It might be in a crash loop!"*

### Refactoring
- To keep the code clean, we will move the restart execution logic into a dedicated module or struct `HealEngine`.
</decisions>

<canonical_refs>
## Canonical References
- `ROADMAP.md` — Phase 15 goals.
- `src/main.rs` — Event loop where logic currently lives.
</canonical_refs>
