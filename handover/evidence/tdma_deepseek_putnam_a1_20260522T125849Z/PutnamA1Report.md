# TDMA-Bounded-RC1 Atom 13 — Putnam 2024 A1 (Real DeepSeek, EXTREME stress)

**Model**: deepseek-chat (temperature 0.7)

**Problem**: Putnam 2024 A1 (number theory; mod-3 + 2-adic infinite descent)

## Outcome

- Stages completed: **8/8**
- Stages escalated/aborted: []
- Total attempts: **10**
- Total failed attempts: **2**
- Wall clock: **30.5s**

## Per stage

| Stage | Attempts | Final BBS constraints | Outcome |
|---|---|---|---|
| Stage1-Witness-n=1 | 1 | 0 | passed |
| Stage2-WLOG-gcd=1 | 1 | 0 | passed |
| Stage3-n=2-mod3 | 1 | 0 | passed |
| Stage4-n=2-b-mult-of-3 | 3 | 1 | passed |
| Stage5-n>=3-b-even | 1 | 0 | passed |
| Stage6-n>=3-a-even | 1 | 0 | passed |
| Stage7-n>=3-c-even | 1 | 0 | passed |
| Stage8-Conclude-n=1-only | 1 | 0 | passed |

## Compression

- Total raw stderr: **20645 bytes** (20.2 KB)
- Total BBS (est): 1992 bytes
- **Compression ratio: 10.4x**
- Distinct judge classes: ["missing-descent-step"]
- Max zero_gain_streak: 1

## Prompt invariance

- Range: **1532..1532** tokens (variance 0)
- All within B_PROMPT_MAX=5800: **true**

## DeepSeek tokens consumed

- Prompt: 12083
- Completion: 2528

## KILL guards on REAL LLM traffic

- Raw stderr leak in any prompt: **false** (KILL-tdma-1)
- Prompt always within budget: see above (KILL-tdma-9)

## Evidence integrity

- per_attempt_probes.jsonl sha256: 624b6e97ea19a6bbae20851fcf4a84fd3a066962d9673f7d0566807f5a97e9c4
- chaintape.jsonl sha256: a5196d2b6930fb86f187f35579353d17def6aae71634b02d3e7d50f73c653346
