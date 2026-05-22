# turingos tdma run — TDMA-Bounded Production Report

**Model**: Qwen/Qwen3-Coder-30B-A3B-Instruct (temperature 0.7)

**Role**: blackbox

**Judge**: Nesbitt step verifier (Atom 10)

## Outcome

- Stages completed: **1/8**
- Stages escalated/aborted: ["Step2-Rewrite/ZERO_GAIN"]
- Total attempts: **5**
- Total failed attempts: **3**
- Wall clock: **14.5s**

## Per stage

| Stage | Attempts | Final BBS constraints | Outcome |
|---|---|---|---|
| Step1-Substitute | 1 | 0 | passed |
| Step2-Rewrite | 4 | 1 | escalate-ZERO_GAIN |

## Compression

- Total raw stderr: **41254 bytes** (40.3 KB)
- Total BBS (est): 2964 bytes
- **Compression ratio: 13.9x**
- Distinct judge classes: ["logical-gap"]
- Max zero_gain_streak: 2

## Prompt invariance

- Range: **991..991** tokens (variance 0)
- All within B_PROMPT_MAX=5800: **true**

## SiliconFlow tokens consumed

- Prompt: 2454
- Completion: 898

## KILL guards on PRODUCTION LLM traffic

- Raw stderr leak in any prompt: **false** (KILL-tdma-1)
- Prompt always within budget: see above (KILL-tdma-9)

## Evidence integrity

- per_attempt_probes.jsonl sha256: 83a7b1b0c4b29c83bdc8d345f89bab8b14a83dc56f29cb97504c00c2cfc281bf
- chaintape.jsonl sha256: b5de351ce703cef76d0b7610aa967f62851af212ae63fc7b1e8d90ed4dd98d8c
