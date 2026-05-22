# turingos tdma run — TDMA-Bounded Production Report

**Model**: deepseek-ai/DeepSeek-V3.2 (temperature 0.7)

**Role**: meta

**Judge**: Nesbitt step verifier (Atom 10)

## Outcome

- Stages completed: **2/8**
- Stages escalated/aborted: ["Step3-Expand/ZERO_GAIN"]
- Total attempts: **6**
- Total failed attempts: **3**
- Wall clock: **49.7s**

## Per stage

| Stage | Attempts | Final BBS constraints | Outcome |
|---|---|---|---|
| Step1-Substitute | 1 | 0 | passed |
| Step2-Rewrite | 1 | 0 | passed |
| Step3-Expand | 4 | 1 | escalate-ZERO_GAIN |

## Compression

- Total raw stderr: **41281 bytes** (40.3 KB)
- Total BBS (est): 2976 bytes
- **Compression ratio: 13.9x**
- Distinct judge classes: ["off-stage"]
- Max zero_gain_streak: 2

## Prompt invariance

- Range: **941..941** tokens (variance 0)
- All within B_PROMPT_MAX=5800: **true**

## SiliconFlow tokens consumed

- Prompt: 2793
- Completion: 892

## KILL guards on PRODUCTION LLM traffic

- Raw stderr leak in any prompt: **false** (KILL-tdma-1)
- Prompt always within budget: see above (KILL-tdma-9)

## Evidence integrity

- per_attempt_probes.jsonl sha256: 4c8aa780f44325b563d4864ae28ac2be2cf3bd1c68422b8a90418b630f41efb1
- chaintape.jsonl sha256: 4b843cb0868ca2ffadc9117022c22782505bf2c92725c0253daac9460150846c
