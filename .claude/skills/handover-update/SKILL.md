---
name: handover-update
description: Update handover/ai-direct/LATEST.md with current session state
user_invocable: true
---

# /handover-update — Session Handover

Mandatory before ending a session.

## Procedure

### 1. Gather State
- Read `handover/ai-direct/LATEST.md`
- `git log --oneline -10`
- `git diff --stat`
- Key decisions from this session

### 2. Draft LATEST.md
```markdown
# TuringOS v4 — Handover State
**Updated**: YYYY-MM-DD
**Session Summary**: [one-line]

## Current State
- [What works]
- [What's broken/incomplete]
- [Active experiments]

## Next Steps
- [Priority tasks]

## Open Questions
- [Unresolved items]
```

### 3. Review
Present draft to user before writing.
