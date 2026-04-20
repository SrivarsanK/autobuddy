# Phase 5: The Bodyguard - Research

## Objective
Monitor critical processes and alert if they are not running.

## Domain Knowledge
- **sysinfo**: Already used in `SysHealthWatcher`. We can use it to list all processes and check for matches.
- **Process List**: Users need a way to specify which processes are "critical".
- **Alert Trigger**: If a process in the `critical_processes` list is missing from the system process tree, emit `Event::ProcessCrash`.

## Refinement
Instead of just "crash", we should call it "missing" or "not running" to cover both clean stops and crashes.
Branding: "Bodyguard Alert: [Process] is missing from duty! Checking the perimeter."

## Performance
Checking the full process list every second might be overhead. A 5-second interval for the Bodyguard is safer.
