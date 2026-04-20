---
name: gsd-sketch-wrap-up
description: Package sketch design findings into a persistent project skill for future build conversations
---

<objective>
Curate sketch design findings and package them into a persistent project skill that the agent
auto-loads when building the real UI. Also writes a summary to `.planning/sketches/` for
project history. Output skill goes to `./.agent/skills/sketch-findings-[project]/` (project-local).
</objective>

<execution_context>
@.agent/get-shit-done/workflows/sketch-wrap-up.md
@.agent/get-shit-done/references/ui-brand.md
</execution_context>

<runtime_note>
**Copilot (VS Code):** Use `vscode_askquestions` wherever this workflow calls `AskUserQuestion`.
</runtime_note>

<process>
Execute the sketch-wrap-up workflow from @.agent/get-shit-done/workflows/sketch-wrap-up.md end-to-end.
Preserve all curation gates (per-sketch review, grouping approval, GEMINI.md routing line).
</process>
