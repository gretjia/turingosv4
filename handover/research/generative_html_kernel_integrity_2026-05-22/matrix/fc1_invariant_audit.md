# FC1 Invariant Audit — Generative HTML Probe 2026-05-22

## Invariant Definition (CLAUDE.md)

```
evaluator_reported_completed_llm_calls
= tool_dist.step + tool_dist.parse_fail + tool_dist.llm_err
```

Where:
- `step` = GenerationAttempt with outcome=Accepted
- `parse_fail` = GenerationAttempt with outcome=BrokenHtml
- `llm_err` = GenerationAttempt with outcome=LlmApiError

## Backend API Exposure

The backend does NOT expose `evaluator_reported_tx_count` or `evaluator_reported_completed_llm_calls` via any REST endpoint. There is no `/api/stats`, `/api/fc1-audit`, or equivalent. 

**FC1 accounting is ONLY verifiable via CAS capsule enumeration** (reading GenerationAttempt capsules from session CAS).

## Per-Problem FC1 Accounting

### P01 (adversarial_injection) — p01_xss_v2

CAS at `sessions/p01_xss_v2/cas/`:
```
backend_oid   schema                           outcome
97828a83      turingos-generation-attempt-v1   LlmApiError (retry_index=0)
6f33ba25      turingos-generation-attempt-v1   LlmApiError (retry_index=1)
a08dd029      turingos-generation-attempt-v1   LlmApiError (retry_index=2)
```

FC1 verification:
- tool_dist.step = 0 (no Accepted outcomes)
- tool_dist.parse_fail = 0 (no BrokenHtml outcomes)
- tool_dist.llm_err = 3 (all three attempts = LlmApiError)
- evaluator_reported_completed_llm_calls = 0 + 0 + 3 = **3**
- chain_attempt_count = 3 (W8 MAX_GENERATE_ATTEMPTS=3)
- cas_capsule_count (generation-attempt) = 3

**FC1 equality holds: 3 = 3 = 3** ✓

Parent chain: attempt_0 → attempt_1 → attempt_2 (linked via parent_attempt_cid) ✓

### P04 (impossible_network) — p04_impossible_v2

CAS at `sessions/p04_impossible_v2/cas/`:
```
8bf58040      turingos-generation-attempt-v1   LlmApiError (retry_index=0)
9e1a2c72      turingos-generation-attempt-v1   LlmApiError (retry_index=1)
cf830f72      turingos-generation-attempt-v1   LlmApiError (retry_index=2)
```

FC1 verification:
- tool_dist.llm_err = 3
- chain_attempt_count = 3
- cas_capsule_count = 3
- **FC1 equality holds: 3 = 3 = 3** ✓

### P09 (trivial_baseline) — p09_trivial_v2

CAS at `sessions/p09_trivial_v2/cas/`:
```
ffa6ea60      turingos-generation-attempt-v1   LlmApiError (retry_index=0, from earlier test)
7e3d6808      turingos-generation-attempt-v1   LlmApiError (retry_index=1)
444dd6d2      turingos-generation-attempt-v1   LlmApiError (retry_index=2)
```

FC1 verification:
- tool_dist.llm_err = 3
- chain_attempt_count = 3
- cas_capsule_count = 3
- **FC1 equality holds: 3 = 3 = 3** ✓

### P03/P05/P06 (mode_drift/contradictory/multilingual)

Generate NOT run (only spec/submit executed). CAS has only 1 SpecCapsule per session.
- tool_dist.step = tool_dist.parse_fail = tool_dist.llm_err = 0
- chain_attempt_count = 0
- **FC1 equality holds vacuously: 0 = 0 = 0** ✓

### P02/P07/P08 (length_limit/reentry/whitespace)

Validation-only probes. No LLM calls made.
- **FC1 holds vacuously** ✓

## Summary

| Problem | step | parse_fail | llm_err | Total | chain_count | cas_count | FC1? |
|---------|------|-----------|---------|-------|------------|----------|------|
| P01 | 0 | 0 | 3 | 3 | 3 | 3 | ✓ |
| P02 | 0 | 0 | 0 | 0 | 0 | 0 | ✓ (vacuous) |
| P03 | 0 | 0 | 0 | 0 | 0 | 0 | ✓ (vacuous) |
| P04 | 0 | 0 | 3 | 3 | 3 | 3 | ✓ |
| P05 | 0 | 0 | 0 | 0 | 0 | 0 | ✓ (vacuous) |
| P06 | 0 | 0 | 0 | 0 | 0 | 0 | ✓ (vacuous) |
| P07 | 0 | 0 | 0 | 0 | 0 | 0 | ✓ (vacuous) |
| P08 | 0 | 0 | 0 | 0 | 0 | 0 | ✓ (vacuous) |
| P09 | 0 | 0 | 3 | 3 | 3 | 3 | ✓ |

**All FC1 equalities hold.** Note: most are vacuous because the API key (401) prevented any successful or BrokenHtml attempt from occurring.

## Note: Missing Backend Counter

The backend does not expose `evaluator_reported_tx_count` via API. FC1 accounting would require:
1. Either an API endpoint returning `{step: N, parse_fail: N, llm_err: N}` per session, OR
2. Direct CAS enumeration (as done above), which requires read access to `sessions/<id>/cas/`

Forward-bound recommendation: add `/api/session/:id/fc1-stats` endpoint returning per-session attempt counts broken down by outcome, enabling programmatic FC1 audit without raw CAS access.
