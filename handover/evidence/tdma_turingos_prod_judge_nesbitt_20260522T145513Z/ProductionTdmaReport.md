# turingos tdma run — TDMA-Bounded Production Report

**Model**: deepseek-ai/DeepSeek-V3.2 (temperature 0.5)

**Role**: meta

**Judge**: Nesbitt step verifier (Atom 10)

## Outcome

- Stages completed: **8/8**
- Stages escalated/aborted: []
- Total attempts: **9**
- Total failed attempts: **1**
- Wall clock: **85.3s**

## Per stage

| Stage | Attempts | Final BBS constraints | Outcome |
|---|---|---|---|
| Step1-Substitute | 1 | 0 | passed |
| Step2-Rewrite | 1 | 0 | passed |
| Step3-Expand | 1 | 0 | passed |
| Step4-Group | 1 | 0 | passed |
| Step5-ApplyAMGM | 1 | 0 | passed |
| Step6-Sum | 1 | 0 | passed |
| Step7-Subtract | 1 | 0 | passed |
| Step8-Conclude+Eq | 2 | 1 | passed |

## Compression

- Total raw stderr: **10339 bytes** (10.1 KB)
- Total BBS (est): 964 bytes
- **Compression ratio: 10.7x**
- Distinct judge classes: ["algebra-error"]
- Max zero_gain_streak: 0

## Prompt invariance

- Range: **1716..1716** tokens (variance 0)
- All within B_PROMPT_MAX=5800: **true**

## SiliconFlow tokens consumed

- Prompt: 6680
- Completion: 1484

## KILL guards on PRODUCTION LLM traffic

- Raw stderr leak in any prompt: **false** (KILL-tdma-1)
- Prompt always within budget: see above (KILL-tdma-9)

## Evidence integrity

- per_attempt_probes.jsonl sha256: df9fdf46201afeefa5e5ae77984d24c0933a647a3535fc83ee277e60acf1bd12
- chaintape.jsonl sha256: a645de52b2e58a47e260182634e07ccab7696ac629a236baee0f5c715eb7c7af
