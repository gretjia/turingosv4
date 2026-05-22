# turingos tdma run — TDMA-Bounded Production Report

**Model**: Qwen/Qwen3-Coder-30B-A3B-Instruct (temperature 0.7)

**Role**: blackbox

**Judge**: Nesbitt step verifier (Atom 10)

## Outcome

- Stages completed: **8/8**
- Stages escalated/aborted: []
- Total attempts: **9**
- Total failed attempts: **1**
- Wall clock: **80.4s**

## Per stage

| Stage | Attempts | Final BBS constraints | Outcome |
|---|---|---|---|
| Step1-Substitute | 1 | 0 | passed |
| Step2-Rewrite | 1 | 0 | passed |
| Step3-Expand | 1 | 0 | passed |
| Step4-Group | 1 | 0 | passed |
| Step5-ApplyAMGM | 2 | 1 | passed |
| Step6-Sum | 1 | 0 | passed |
| Step7-Subtract | 1 | 0 | passed |
| Step8-Conclude+Eq | 1 | 0 | passed |

## Compression

- Total raw stderr: **10313 bytes** (10.1 KB)
- Total BBS (est): 996 bytes
- **Compression ratio: 10.4x**
- Distinct judge classes: ["off-stage"]
- Max zero_gain_streak: 0

## Prompt invariance

- Range: **1549..1549** tokens (variance 0)
- All within B_PROMPT_MAX=5800: **true**

## SiliconFlow tokens consumed

- Prompt: 8325
- Completion: 1889

## KILL guards on PRODUCTION LLM traffic

- Raw stderr leak in any prompt: **false** (KILL-tdma-1)
- Prompt always within budget: see above (KILL-tdma-9)

## Evidence integrity

- per_attempt_probes.jsonl sha256: b37c875ff2ede4546c69e47e5d3a7dd8382fec19fbb2c3bf304b58d7a83b81f4
- chaintape.jsonl sha256: f7510226307d7ecdf2a9800dc18e71efafdf844a9a276f575e07b5a527094b47
