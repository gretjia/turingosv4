---
name: lesson-to-rule
description: Convert violations into executable rules — Living Harness amplifier
user_invocable: true
---

# /lesson-to-rule — Violation → Rule Conversion

## Input
`/lesson-to-rule <V-xxx>` or `/lesson-to-rule` (auto-detect recent unprotected violations)

## Stages

### 1. Read Context
- `incidents/V-xxx_*/meta.yaml` — pattern, axiom, severity
- `incidents/V-xxx_*/trace.md` — execution record
- `incidents/V-xxx_*/root_cause.md` — causal chain
- `traces/sessions/*.jsonl` — recent related triggers

### 2. Classify Pattern
Map to enforcement type: grep / grep_inverse / compound

### 3. Search Existing Rules
Check if `rules/active/` already covers this pattern. Avoid duplicates.

### 4. Generate YAML
Draft new rule per `rules/SCHEMA.yaml`. Assign next R-xxx ID.

### 5. Create Case (判例)
Draft case per `cases/SCHEMA.md`:
- Link to constitutional clause(s)
- Facts, ruling, and precedent
- Assign next C-xxx ID

### 6. Await Confirmation
Present rule + case to user. Add to `rules/active/` and `cases/` only after approval.
Hard cap: 30 active rules maximum.
