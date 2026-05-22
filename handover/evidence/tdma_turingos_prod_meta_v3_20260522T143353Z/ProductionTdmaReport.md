# turingos tdma run — TDMA-Bounded Production Report

**Model**: deepseek-ai/DeepSeek-V3.2 (temperature 0.7)

**Role**: meta

**Judge**: Nesbitt step verifier (Atom 10)

## Outcome

- Stages completed: **7/8**
- Stages escalated/aborted: ["Step8-Conclude+Eq/MAX_RETRIES"]
- Total attempts: **12**
- Total failed attempts: **4**
- Wall clock: **108.5s**

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

- Total raw stderr: **51438 bytes** (50.2 KB)
- Total BBS (est): 4448 bytes
- **Compression ratio: 11.6x**
- Distinct judge classes: ["algebra-error", "missing-equality-case"]
- Max zero_gain_streak: 1

## Prompt invariance

- Range: **1640..1704** tokens (variance 64)
- All within B_PROMPT_MAX=5800: **true**

## SiliconFlow tokens consumed

- Prompt: 9921
- Completion: 1988

## KILL guards on PRODUCTION LLM traffic

- Raw stderr leak in any prompt: **false** (KILL-tdma-1)
- Prompt always within budget: see above (KILL-tdma-9)

## Evidence integrity

- per_attempt_probes.jsonl sha256: 2a10bb147d90d195343391a565b87109c84ae31602e46b4316648abc153931a6
- chaintape.jsonl sha256: 8ded70285eb7f6310aea92c030d0c51dc042853a3cdde76b2c2b923c94e66597
