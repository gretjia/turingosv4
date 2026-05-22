# turingos tdma run — TDMA-Bounded Production Report

**Model**: deepseek-ai/DeepSeek-V3.2 (temperature 0.5)

**Role**: meta

**Judge**: Nesbitt step verifier (Atom 10)

## Outcome

- Stages completed: **3/5**
- Stages escalated/aborted: ["Stage4-Counterexample-Construction/ZERO_GAIN"]
- Total attempts: **7**
- Total failed attempts: **3**
- Wall clock: **65.3s**

## Per stage

| Stage | Attempts | Final BBS constraints | Outcome |
|---|---|---|---|
| Stage1-Simplify-2010n | 1 | 0 | passed |
| Stage2-Factor-2010 | 1 | 0 | passed |
| Stage3-Closure-Prime-Containment | 1 | 0 | passed |
| Stage4-Counterexample-Construction | 4 | 1 | escalate-ZERO_GAIN |

## Compression

- Total raw stderr: **41338 bytes** (40.4 KB)
- Total BBS (est): 2976 bytes
- **Compression ratio: 13.9x**
- Distinct judge classes: ["missing-counterexample"]
- Max zero_gain_streak: 2

## Prompt invariance

- Range: **1364..1364** tokens (variance 0)
- All within B_PROMPT_MAX=5800: **true**

## SiliconFlow tokens consumed

- Prompt: 4591
- Completion: 1155

## KILL guards on PRODUCTION LLM traffic

- Raw stderr leak in any prompt: **false** (KILL-tdma-1)
- Prompt always within budget: see above (KILL-tdma-9)

## Evidence integrity

- per_attempt_probes.jsonl sha256: 8c977c8927e0dd354bbc7c9fa9c86af4950ffa37a04c77b60e4dd5c9404223c5
- chaintape.jsonl sha256: 356cd087d4aa23d6821d84d251b368dd67fb68aeb25eec27e6c6e7816192f13b
