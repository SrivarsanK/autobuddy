# Phase 15: Post-Op - Research

Objective: Implement verification of service recovery after healing.

## 1. Goal
Ensure that when we say a service has "recovered", it actually stays up for a few seconds and returns to a healthy state.

## 2. Verification Mechanism
- **Polling**: After a restart, wait ~5 seconds.
- **Check**: Check the process list again or use `systemctl is-active <name>`.
- **Success Result**: Notify user of actual success.
- **Failure Result**: Notify user if the service crashed again immediately.

## 3. Implementation in Rust
- We can spawn a background task (`tokio::spawn`) after a restart command.
- The task sleeps for N seconds, checks status, and emits a `RecoveryStatus { name, success }` event or sends a direct Telegram message.

## 4. Edge Cases
- Service takes 10+ seconds to start (e.g. heavy DB). 
- **Solution**: Check incrementally or use a longer delay. For the buddy, a 5-second "Is it still there?" check is usually sufficient for a "viral" watchdog.
