# P09 Verdict — Trivial Baseline (todo list)

**Expected**: Clean spec → accepted artifact (fast path)  
**Actual**: Spec synthesis OK (~47s); generate 3x LlmApiError (401)

**Result**: PARTIAL — spec path verified; generate path blocked by API key issue

**Key findings**:
1. Spec synthesis completed in ~47 seconds via deepseek-v4-pro (meta model)
2. SpecCapsule correctly written to session-local CAS with schema_id=turingos-spec-capsule-v1
3. W8 retry chain correct: 3 attempts, each linked via parent_attempt_cid
4. All 3 generate attempts got HTTP 401 from SiliconFlow (deepseek-v4-pro as blackbox)
5. GenerateRejection capsules correctly written with retryable=true, reject_class=LlmApiError
6. No BuildSessionView (correct: no accepted artifact)
7. Tape-relay works: attempt 2+ includes "feeding prior rejection diagnostics into LLM prompt"

**Capsule chain**: 1 SpecCapsule + 3 GenerationAttempt + 3 GenerateRejection = 7 capsules
**FC1**: step=0, parse_fail=0, llm_err=3; chain_count=3; cas_count=3 → FC1 holds ✓
**Kernel surfaces exercised**: FC1 full chain (spec→generate→reject), W8 retry, tape-relay
