# P01 Capsule Chain — p01_xss_v2

CAS path: sessions/p01_xss_v2/cas/

| # | CID prefix | Schema | Details |
|---|-----------|--------|---------|
| 1 | 9d2d4a07 | turingos-spec-capsule-v1 | SpecCapsule from first spec/submit attempt, size=5464 |
| 2 | fc8607a5 | turingos-spec-capsule-v1 | SpecCapsule from second spec/submit attempt (same session), size=5408 |
| 3 | 97828a83 | turingos-generation-attempt-v1 | attempt retry_index=0, outcome=LlmApiError, parent=null |
| 4 | 09379de4 | turingos-generate-rejection-v1 | rejection for attempt 3, reject_class=LlmApiError, retryable=true |
| 5 | 6f33ba25 | turingos-generation-attempt-v1 | attempt retry_index=1, parent=6acc5b3b..., outcome=LlmApiError |
| 6 | de82a8c5 | turingos-generate-rejection-v1 | rejection for attempt 5, retryable=true |
| 7 | a08dd029 | turingos-generation-attempt-v1 | attempt retry_index=2, parent=6b03fef6..., outcome=LlmApiError |
| 8 | b1bb0ea8 | turingos-generate-rejection-v1 | rejection for attempt 7, retryable=true |

Referential closure: 
- Each GenerationAttempt references spec_capsule_cid=525cf139... (second SpecCapsule)
- Attempt chain: 0→1→2 via parent_attempt_cid
- Each GenerateRejection references both spec_capsule_cid and generation_attempt_cid
- No BuildSessionView (correct: no accepted artifact)
