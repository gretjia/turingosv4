# TDMA-Bounded-RC1 Atom 9 — Distiller Compression Stress Report

Synthetic stress test of the distiller / BBS / zero_gain / eviction stack.
Verdicts are scripted via `InjectedJudge` so failure patterns are deterministic.

## S1 — info retention (4 distinct-signature failures + 1 success)

- Final BBS constraint count: **4** (expected 4)
- Information retention rate: **100%**
- Raw stderr leak in any prompt: **false**

## S2 — zero_gain triggering (same-signature repeat)

- Escalated: **true**
- Escalated at attempt: **4**
- ZERO_GAIN_K threshold: **3** (escalation expected at attempt 4 or via MAX_RETRIES)

## S3 — long chain (10 tasks × 3 distinct-sig failures + 1 success)

- Prompt size range: **362..501** (variance 139)
- All prompts within B_PROMPT_MAX=5800: **true**
- Tasks completed: **10/10**
- Total raw stderr bytes ingested: **410600** (401.0 KB)
- Total BBS bytes (token-estimated): **33012** (32.2 KB)
- **Compression ratio: 12.4x** (raw_stderr / BBS)
- Per-task final constraint counts: [3, 3, 3, 3, 3, 3, 3, 3, 3, 3]
- Raw stderr leak in any prompt across full run: **false**

## S4 — mixed (alternating signature repeat + change)

- Max zero_gain_streak observed: **1**
- Zero_gain_streak reset events: **2**
- Final BBS constraint count (3 distinct signatures expected): **3**
- Escalated via MAX_RETRIES: **true**

## Evidence integrity

- per_step_probes.jsonl sha256: b0289a20a82adbd16c0d988a155935c31ed499987d1d4b7e4d215b41d840c3f9
