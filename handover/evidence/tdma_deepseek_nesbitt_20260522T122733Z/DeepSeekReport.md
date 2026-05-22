# TDMA-Bounded-RC1 Atom 12 — Real DeepSeek LLM Nesbitt Stress

**Model**: deepseek-chat (temperature 0.7)

**Problem**: Nesbitt's inequality (real LLM worker vs deterministic IneqMath-style judge)

## Outcome

- Stages completed: **8/8**
- Stages escalated/aborted: []
- Total attempts: **12**
- Total failed attempts: **4**
- Wall clock: **23.1s**

## Per stage

| Stage | Attempts | Final BBS constraints | Outcome |
|---|---|---|---|
| Step1-Substitute | 1 | 0 | passed |
| Step2-Rewrite | 2 | 1 | passed |
| Step3-Expand | 1 | 0 | passed |
| Step4-Group | 1 | 0 | passed |
| Step5-ApplyAMGM | 1 | 0 | passed |
| Step6-Sum | 1 | 0 | passed |
| Step7-Subtract | 1 | 0 | passed |
| Step8-Conclude+Eq | 4 | 1 | passed |

## Compression

- Total raw stderr: **41236 bytes** (40.3 KB)
- Total BBS (est): 3940 bytes
- **Compression ratio: 10.5x**
- Distinct judge classes: ["algebra-error", "logical-gap"]
- Max zero_gain_streak: 2

## Prompt invariance

- Range: **713..1286** tokens (variance 573)
- All within B_PROMPT_MAX=5800: **true**

## DeepSeek tokens consumed

- Prompt: 8058
- Completion: 1594

## KILL guards on REAL LLM traffic

- Raw stderr leak in any prompt: **false** (KILL-tdma-1)
- Prompt always within budget: see above (KILL-tdma-9)

## Evidence integrity

- per_attempt_probes.jsonl sha256: 3d751d8519a6a4c0edc11c5c141fbc80411acc0fdc2dd8d242230d6bc122b0e2
- chaintape.jsonl sha256: 6481588d3db87283aa9f8cdf0c6618a412d5d9e0f7c376611dd8230d102ed79c
