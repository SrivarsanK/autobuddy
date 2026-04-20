---
wave: 1
depends_on: []
files_modified: [src/alert.rs]
autonomous: true
---

# Plan: Phase 6 - The Oracle (Smart Fixes)

Provide repair suggestions for build failures.

## Tasks

### 1. Implement Oracle Logic in AlertEngine
<task>
<read_first>
- src/alert.rs
</read_first>
<action>
Add an `oracle_diagnose(log: &str) -> Option<String>` private method to `AlertEngine`.
Implement a basic mapping of error patterns to suggestions (as identified in research).
Update the `Event::BuildFailure` match arm in `process` to append the suggestion if found.
</action>
<acceptance_criteria>
- `AlertEngine` has diagnosis logic.
- Messages for build failures now include a "Oracle Suggestion" section if diagnostic matches.
</acceptance_criteria>
</task>

### 2. Verify with Unit Tests
<task>
<read_first>
- src/alert.rs
</read_first>
<action>
Update `test_alert_on_build_failure` or add a new test `test_oracle_suggestion` to verify that specific log snippets trigger the correct suggestions.
</action>
<acceptance_criteria>
- Test passes.
- Suggestions are accurate to the input logs.
</acceptance_criteria>
</task>

## Verification
- [ ] Simulate a build fail log with "unresolved import".
- [ ] Verify Telegram alert contains: "Oracle Suggestion: Check your imports...".
