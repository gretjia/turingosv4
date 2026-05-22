# turingos tdma run — TDMA-Bounded Production Report

**Model**: deepseek-ai/DeepSeek-V3.2 (temperature 0.7)

**Role**: meta

**Judge**: Nesbitt step verifier (Atom 10)

## Outcome

- Stages completed: **7/8**
- Stages escalated/aborted: ["Step8-Conclude+Eq/MAX_RETRIES"]
- Total attempts: **12**
- Total failed attempts: **4**
- Wall clock: **117.7s**

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
| Step8-Conclude+Eq | 5 | 2 | escalate-MAX_RETRIES |

## Compression

- Total raw stderr: **51597 bytes** (50.4 KB)
- Total BBS (est): 4192 bytes
- **Compression ratio: 12.3x**
- Distinct judge classes: ["algebra-error", "missing-equality-case"]
- Max zero_gain_streak: 2

## Prompt invariance

- Range: **1664..1728** tokens (variance 64)
- All within B_PROMPT_MAX=5800: **true**

## SiliconFlow tokens consumed

- Prompt: 9672
- Completion: 2181

## KILL guards on PRODUCTION LLM traffic

- Raw stderr leak in any prompt: **false** (KILL-tdma-1)
- Prompt always within budget: see above (KILL-tdma-9)

## Evidence integrity

- per_attempt_probes.jsonl sha256: 0effbe85f38e033a84211f0078fb4d8f46c76d0c577a06a173b69c3ac63b4a9c
- chaintape.jsonl sha256: 88b94469572844faf660a89061ab84a82b620266b30d31ad2491b37fa36dc951
