# Phase 3: The Healer - Context

**Gathered:** 2026-04-20
**Status:** Ready for planning
**Source:** Phase 3 Roadmap.

<domain>
## Phase Boundary
Implement a Build Watcher that detects failures and alerts via Telegram.

</domain>

<decisions>
## Implementation Decisions

### Build Monitoring
- [x] **Locked:** Create `src/watchers/build.rs`.
- [x] **Locked:** Watch for the existence or modification of `build.log`.
- [x] **Locked:** If "error" or "failed" is found in the content, trigger an event.

### Alert Content
- [x] **Locked:** Include a snippet of the error in the Telegram message.
- [x] **Locked:** Play into the brand (The Healer).

</decisions>

<canonical_refs>
## Canonical References
- `src/event.rs` — `BuildFailure` variant.
- `src/watchers/build.rs` — New watcher.

</canonical_refs>
