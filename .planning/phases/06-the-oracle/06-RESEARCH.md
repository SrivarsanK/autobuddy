# Phase 6: The Oracle - Research

## Objective
Enhanced "The Healer" with smart diagnosis and repair suggestions.

## Requirements
- When a `BuildFailure` occurs, analyze the `log_tail`.
- Map specific error strings to suggested fixes.
- Append "Oracle Suggestion: [Fix]" to the Telegram alert.

## Mapping Database (Draft)
| Error Snippet | Suggested Fix |
|---|---|
| `unresolved import` | "Check your imports or Cargo.toml for missing dependencies." |
| `cannot find value` | "Is the variable defined in this scope? Check for typos." |
| `expected ..., found ...` | "Type mismatch detected. Use `.into()` or check function signatures." |
| `failed to resolve: use of undeclared crate` | "Run `cargo update` or check if the dependency is in Cargo.toml." |

## Implementation
- Update `AlertEngine` to include a `suggest_fix` helper.
- Enhance the `Event::BuildFailure` arm in `process` to call this helper.
- Severity remains Critical.
