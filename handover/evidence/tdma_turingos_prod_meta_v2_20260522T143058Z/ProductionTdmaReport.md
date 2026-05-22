# turingos tdma run — TDMA-Bounded Production Report

**Model**: deepseek-ai/DeepSeek-V3.2 (temperature 0.7)

**Role**: meta

**Judge**: Nesbitt step verifier (Atom 10)

## Outcome

- Stages completed: **1/8**
- Stages escalated/aborted: ["Step2-Rewrite/ZERO_GAIN"]
- Total attempts: **5**
- Total failed attempts: **3**
- Wall clock: **53.7s**

## Per stage

| Stage | Attempts | Final BBS constraints | Outcome |
|---|---|---|---|
| Step1-Substitute | 1 | 0 | passed |
| Step2-Rewrite | 4 | 1 | escalate-ZERO_GAIN |

## Compression

- Total raw stderr: **41167 bytes** (40.2 KB)
- Total BBS (est): 2988 bytes
- **Compression ratio: 13.8x**
- Distinct judge classes: ["off-stage"]
- Max zero_gain_streak: 2

## Prompt invariance

- Range: **799..799** tokens (variance 0)
- All within B_PROMPT_MAX=5800: **true**

## SiliconFlow tokens consumed

- Prompt: 2041
- Completion: 833

## KILL guards on PRODUCTION LLM traffic

- Raw stderr leak in any prompt: **false** (KILL-tdma-1)
- Prompt always within budget: see above (KILL-tdma-9)

## Evidence integrity

- per_attempt_probes.jsonl sha256: 46a952eab28882014e4495750cbd3ab36ae9bc6f9bd77c68903d1e92055653d7
- chaintape.jsonl sha256: 061555bc070644037d684b410597a7582000e3fbbb2ab69f505fd0229118ef49
