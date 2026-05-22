# Cost Budget — Generative HTML Probe 2026-05-22

## LLM Call Budget

**Cap**: ≤ 30 LLM calls  
**Rate**: ≤ ¥3 total

## Actual LLM Calls Made

### Spec synthesis calls (deepseek-v4-pro, meta model, thinking=on)

| Problem | Session | Calls | Token estimate | Status |
|---------|---------|-------|----------------|--------|
| P01 adversarial | p01_xss_v2 | 1 | ~3000 | SUCCESS (spec.md ~5464 bytes) |
| P03 mode_drift | p03_mode_drift_v2 | 1 | ~2500 | SUCCESS |
| P04 impossible | p04_impossible_v2 | 1 | ~3000 | SUCCESS |
| P05 contradictory | p05_contradictory_v2 | 1 | ~3000 | SUCCESS |
| P06 multilingual | p06_multilingual_v2 | 1 | ~2500 | SUCCESS |
| P09 trivial | p09_trivial_v2 | 1 | ~2000 | SUCCESS |

**Subtotal spec synthesis**: 6 calls × ~2500 tokens avg = ~15,000 tokens

### Generate calls (deepseek-v4-pro, blackbox role, thinking=off)

All generate calls failed with HTTP 401 "Api key is invalid". 
Each problem attempted W8 = 3 retries.

| Problem | Session | W8 Attempts | Result |
|---------|---------|-------------|--------|
| P01 adversarial | p01_xss_v2 | 3 | 3x HTTP 401 LlmApiError |
| P04 impossible | p04_impossible_v2 | 3 | 3x HTTP 401 LlmApiError |
| P09 trivial | p09_trivial_v2 | 3 | 3x HTTP 401 LlmApiError |

**Note**: HTTP 401 errors are API-level rejections. Whether the API counted these as billable calls is unknown. The local token usage in GenerationAttempt capsules shows `usage_total_tokens: null` (not recorded on auth failure), suggesting the API rejected before generating tokens.

**Claimed billable**: 0 tokens (all 401, no token usage recorded in capsules)

### Other calls (not billable to this probe)

- Agent A's p01_fifth_grader session: spec synthesis via spec/turn (unknown token count, outside our budget)
- No triage calls (all blocked by C10 promotion guard)

## Total Budget Usage

| Category | LLM Calls | Tokens (estimate) | Cost estimate |
|----------|-----------|-------------------|---------------|
| Spec synthesis (spec/submit) | 6 | ~15,000 | ~¥0.15 (¥0.01/1K for v4-pro) |
| Generate attempts (401) | 9 | 0 (auth rejected) | ¥0.00 |
| **Total** | **6 billable** | **~15,000** | **~¥0.15** |

**Budget used**: 6 / 30 calls (20%), ~¥0.15 / ¥3.00 (5%)

## Notes

1. deepseek-v4-pro usage_total_tokens is populated in SpecCapsule from spec synthesis. Exact values available in `sessions/p0X/spec_transcript.jsonl` per session.
2. The 401 generate calls incurred 0 tokens (auth rejected before completion).
3. Triage calls (blocked by C10) would each consume ~500-1000 tokens if they had run.
4. This probe ran significantly under budget due to: (a) C10 blocking triage path, (b) 401 blocking all generate paths, (c) efficient problem selection (P02/P07/P08 are validation-only, no LLM calls needed).
