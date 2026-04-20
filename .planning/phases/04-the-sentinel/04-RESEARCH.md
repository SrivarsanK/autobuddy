# Phase 4: The Sentinel - Research

## Objective
Detect new SSH sessions on the host machine.

## Platform Support
1.  **Linux**: Monitor `/var/log/auth.log` for patterns like `Accepted password for ... from [IP]`.
2.  **Windows**: Use Event Viewer API or PowerShell `Get-WinEvent` to look for Event ID 4624 (Logon) with Logon Type 10 (RemoteInteractive).
3.  **Cross-Platform Simulation**: For the viral demo, we will monitor a `auth.log` file in the project root, simulating the Linux pattern.

## Implementation Details
- **SentinelWatcher**: A new watcher that follows a log file.
- **Pattern Matching**: Regex for `Accepted ... from [IP]`.
- **Event Type**: `Event::Custom` or a new `Event::Access`. For now, `Event::Custom` is sufficient to avoid breaking the core loop.

## Branding
- Message: "Sentinel Alert: New connection from [IP] for user [User]. Should I keep an eye on them?"
- Severity: `Severity::Info` or `Severity::Warning` if IP is unknown (Phase 4 will start with Info).
