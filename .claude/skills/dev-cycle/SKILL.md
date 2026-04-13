---
name: dev-cycle
description: Full development cycle for TuringOS — plan, audit, code, validate, summarize
user_invocable: true
---

# /dev-cycle — Development Cycle

## Stages

### 1. PLAN
- Read `handover/ai-direct/LATEST.md` for current state
- Read `handover/bible.md` for philosophy constraints
- Draft implementation plan, present to user

### 2. AUDIT PLAN
- Invoke `auditor` agent to review plan against constitution.md
- Check Layer 1 invariants and bible.md alignment

### 3. FIX PLAN
- Revise if audit found issues

### 3.5. SPRINT CONTRACT
- Max 5 files per sprint. Decompose larger plans into sequential sprints.
- Define "done" criteria before coding.

### 4. CODE
- Implement changes per approved plan
- Follow sprint scope strictly

### 4.5. MIGRATION SCAN
- `grep -r` experiments/ for patterns affected by this change
- Run 6 lesson: economic engine changes MUST scan experiment SKILLs

### 5. VALIDATE
- Invoke `/validate` skill

### 6. EXTERNAL AUDIT
- Mandatory: code author cannot be sole auditor (Rule 23)
- Mathematical audit → Gemini. Code audit → Codex.
- Present external findings to user verbatim, unedited.

### 7. SUMMARY
- Report changes, validation results, external audit findings
