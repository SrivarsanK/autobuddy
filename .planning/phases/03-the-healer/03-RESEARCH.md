# Phase 3: The Healer - Research

## Objective
Monitor for build failures and notify the user with a "fix" suggestion.

## Implementation Details
1.  **BuildWatcher**: A new watcher that simulates monitoring a build process (like `cargo build` or `npm run build`).
2.  **Detection logic**: 
    *   Monitor the project directory for `BUILD_FAILED` indicators or tail a specific log file.
    *   For the viral demo: The watcher will monitor for a `build.log` file.
3.  **Healer Logic**:
    *   When a build failure is detected, the `AlertEngine` or `BuildWatcher` should attempt to extract the error.
    *   Send an `Event::BuildFailure`.
4.  **Telegram Alert**: Include the build error and a playful "fix" recommendation (e.g., "The budget for bugs is over, fix it!").

## Risks
- Real-time build monitoring requires hooking into whatever build system the user uses.
- *Decision:* We will implement a file-tailing watcher that looks for "ERROR" or "FAILURE" in `build.log`.

## Validation Architecture
- **Simulated Test:** `echo "ERROR: compile error in main.rs" > build.log`.
- **Alert:** Verify Telegram notification with the error snippet.
