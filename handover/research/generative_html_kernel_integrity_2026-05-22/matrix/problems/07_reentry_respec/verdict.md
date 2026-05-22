# P07 Verdict — Reentry / Re-spec

**Expected**: Second submit detected/rejected; session state protected  
**Actual**: spec/submit silently overwrites; spec/turn prevents null-answer reinit

**Result**: FAIL (spec/submit path) + PASS (spec/turn path)

**Key findings**:
1. spec/submit: second call with same session_id overwrites answers.json silently (no 409)
2. spec/submit: no check if session already has spec.md before re-running synthesis
3. spec/turn: correctly rejects null answer for existing session ("session already exists")
4. Cross-mode: spec/turn session → spec/submit on same session_id writes answers.json alongside turn capsules (no conflict, but no guard)
5. Session namespace is shared between spec/submit and spec/turn with no coordination protocol

**Kernel surfaces exercised**: FC1-N5 (session state management)
