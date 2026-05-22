# turingos tdma run — TDMA-Bounded Production Report

**Model**: deepseek-ai/DeepSeek-V3.2 (temperature 0.7)

**Role**: meta

**Judge**: Nesbitt step verifier (Atom 10)

## Outcome

- Stages completed: **0/8**
- Stages escalated/aborted: ["Step1-Substitute/ZERO_GAIN"]
- Total attempts: **4**
- Total failed attempts: **3**
- Wall clock: **28.4s**

## Per stage

| Stage | Attempts | Final BBS constraints | Outcome |
|---|---|---|---|
| Step1-Substitute | 4 | 1 | escalate-ZERO_GAIN |

## Compression

- Total raw stderr: **41277 bytes** (40.3 KB)
- Total BBS (est): 2808 bytes
- **Compression ratio: 14.7x**
- Distinct judge classes: ["off-stage"]
- Max zero_gain_streak: 2

## Prompt invariance

- Range: **677..677** tokens (variance 0)
- All within B_PROMPT_MAX=5800: **true**

## SiliconFlow tokens consumed

- Prompt: 1451
- Completion: 499

## KILL guards on PRODUCTION LLM traffic

- Raw stderr leak in any prompt: **false** (KILL-tdma-1)
- Prompt always within budget: see above (KILL-tdma-9)

## Evidence integrity

- per_attempt_probes.jsonl sha256: ece1df645feca3d155e936a33184b7f88ae7509b115e80bdc31d72a54297cf6a
- chaintape.jsonl sha256: 431407f832cfb99458779ae4ffcedc0d49ee6381736d7464ba9343a25baa3351
