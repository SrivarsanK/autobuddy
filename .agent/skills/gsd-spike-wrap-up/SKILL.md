---
name: gsd-spike-wrap-up
description: Package spike findings into a persistent project skill for future build conversations
---

<objective>
Curate spike experiment findings and package them into a persistent project skill that the agent
auto-loads in future build conversations. Also writes a summary to `.planning/spikes/` for
project history. Output skill goes to `./.agent/skills/spike-findings-[project]/` (project-local).
</objective>

<execution_context>
@.agent/get-shit-done/workflows/spike-wrap-up.md
@.agent/get-shit-done/references/ui-brand.md
</execution_context>

<runtime_note>
**Copilot (VS Code):** Use `vscode_askquestions` wherever this workflow calls `AskUserQuestion`.
</runtime_note>

<process>
Execute the spike-wrap-up workflow from @.agent/get-shit-done/workflows/spike-wrap-up.md end-to-end.
Preserve all curation gates (per-spike review, grouping approval, GEMINI.md routing line).
</process>
