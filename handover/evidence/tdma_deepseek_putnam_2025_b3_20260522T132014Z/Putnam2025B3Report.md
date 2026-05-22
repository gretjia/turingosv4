# TDMA-Bounded-RC1 Atom 14 — Putnam 2025 B3 (Real DeepSeek, POST-CUTOFF EXTREME stress)

**Model**: deepseek-chat (temperature 0.7)

**Problem**: Putnam 2025 B3 (Dec 6, 2025 — post-DeepSeek-chat training cutoff)

## Outcome

- Stages completed: **5/5**
- Stages escalated/aborted: []
- Total attempts: **6**
- Total failed attempts: **1**
- Wall clock: **18.6s**

## Per stage

| Stage | Attempts | Final BBS constraints | Outcome |
|---|---|---|---|
| Stage1-Simplify-2010n | 1 | 0 | passed |
| Stage2-Factor-2010 | 1 | 0 | passed |
| Stage3-Closure-Prime-Containment | 1 | 0 | passed |
| Stage4-Counterexample-Construction | 1 | 0 | passed |
| Stage5-Conclude-NO | 2 | 1 | passed |

## Compression

- Total raw stderr: **10326 bytes** (10.1 KB)
- Total BBS (est): 988 bytes
- **Compression ratio: 10.5x**
- Distinct judge classes: ["wrong-final-answer"]
- Max zero_gain_streak: 0

## Prompt invariance

- Range: **1804..1804** tokens (variance 0)
- All within B_PROMPT_MAX=5800: **true**

## DeepSeek tokens consumed

- Prompt: 4271
- Completion: 1150

## KILL guards on REAL LLM traffic

- Raw stderr leak in any prompt: **false** (KILL-tdma-1)
- Prompt always within budget: see above (KILL-tdma-9)

## Evidence integrity

- per_attempt_probes.jsonl sha256: 6820a3b7097c31f175af227515e6235c4112bf486adb3abd2c336ecf94762822
- chaintape.jsonl sha256: 0444e2c4b97e8c347b7aa0b63a0e209d65a1526485fee7a247d21691b7114be7
