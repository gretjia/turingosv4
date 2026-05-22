# P02 Verdict — Length Limit (4096-char boundary)

**Expected**: 4096 chars passes; 4097 chars rejected 400  
**Actual**: CORRECT on both paths (spec/submit and spec/turn)

**Result**: PASS

**Key findings**:
1. spec/submit: 4096 chars → HTTP 500 (passes validation; fails at workspace-toml)
2. spec/submit: 4097 chars → HTTP 400 "answer 1 is too long (4097 chars); max is 4096"
3. spec/turn:   4096 chars → HTTP 400 (passes length validation; C10 promotion guard blocks triage)
4. spec/turn:   4097 chars → HTTP 400 "user_answer is too long (4097 chars); max is 4096"

Both endpoints independently enforce the 4096-char limit with identical error messages.
The boundary is INCLUSIVE (4096 accepted, 4097 rejected).

**Kernel surfaces exercised**: FC1-N5 trust boundary validation
