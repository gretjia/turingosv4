# P08 Verdict — Empty / Whitespace

**Expected**: All-whitespace answers rejected (400 invalid_input)  
**Actual**: Empty ("") correctly rejected; whitespace ("   ") passes validation

**Result**: PARTIAL — whitespace bypass is a validation gap

**Key findings**:
1. `validate_answers` at line 420: `if answer.is_empty()` — correct for ""
2. "   " (spaces only) is NOT empty → passes validate_answers → reaches shellout
3. Shellout then fails at workspace-toml bug (separate issue)
4. Fix: change to `if answer.trim().is_empty()`
5. spec/turn also passes whitespace (hits triage, blocked by C10 promotion guard)

**Kernel surfaces exercised**: FC1-N5 trust boundary validation
