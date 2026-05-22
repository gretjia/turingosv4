# turingos tdma run — TDMA-Bounded Production Report

**Model**: deepseek-ai/DeepSeek-V3.2 (temperature 0.5)

**Role**: meta

**Judge**: Nesbitt step verifier (Atom 10)

## Outcome

- Stages completed: **2/8**
- Stages escalated/aborted: ["Stage3-n=2-mod3/ZERO_GAIN"]
- Total attempts: **6**
- Total failed attempts: **3**
- Wall clock: **67.7s**

## Per stage

| Stage | Attempts | Final BBS constraints | Outcome |
|---|---|---|---|
| Stage1-Witness-n=1 | 1 | 0 | passed |
| Stage2-WLOG-gcd=1 | 1 | 0 | passed |
| Stage3-n=2-mod3 | 4 | 1 | escalate-ZERO_GAIN |

## Compression

- Total raw stderr: **41186 bytes** (40.2 KB)
- Total BBS (est): 2928 bytes
- **Compression ratio: 14.1x**
- Distinct judge classes: ["wrong-mod-analysis"]
- Max zero_gain_streak: 2

## Prompt invariance

- Range: **1234..1234** tokens (variance 0)
- All within B_PROMPT_MAX=5800: **true**

## SiliconFlow tokens consumed

- Prompt: 4251
- Completion: 1291

## KILL guards on PRODUCTION LLM traffic

- Raw stderr leak in any prompt: **false** (KILL-tdma-1)
- Prompt always within budget: see above (KILL-tdma-9)

## Evidence integrity

- per_attempt_probes.jsonl sha256: 2a094be6f814638afe6e81b06ba01c63592968dc0e23908b3cf7ef36bee283ab
- chaintape.jsonl sha256: 830f0181c59b4ad292f8f5507265e652b0a8c51ba38c32a994408054b718fd06
