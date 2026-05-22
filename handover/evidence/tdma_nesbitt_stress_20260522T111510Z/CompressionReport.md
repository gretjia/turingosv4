# TDMA-Bounded-RC1 Atom 10 — Nesbitt's Inequality Stress Report

**Problem**: Prove a/(b+c) + b/(a+c) + c/(a+b) ≥ 3/2 for a, b, c > 0.

**Real-world basis**: IneqMath benchmark (arxiv.org/abs/2506.07927). LLM step-level accuracy on this problem class is ≤ 10% per the paper.

**Judge backend**: NesbittStepJudge — 5-category IneqMath-style judge (direction-reversal, bad-substitution, algebra-error, logical-gap, missing-equality).

## Proof progress

- Canonical stages: 8
- Stages completed: **8**
- Stages escalated: []

## Per-stage attempt counts

| Stage | Attempts | Final BBS constraints |
|---|---|---|
| Step1-Substitute | 4 | 3 |
| Step2-Rewrite | 1 | 0 |
| Step3-Expand | 1 | 0 |
| Step4-Group | 2 | 1 |
| Step5-ApplyAMGM | 2 | 1 |
| Step6-Sum | 3 | 1 |
| Step7-Subtract | 2 | 1 |
| Step8-Conclude+Eq | 2 | 1 |

## Compression evidence

- Total attempts: **17** (9 failed + 8 successful + 0 escalated)
- Total raw stderr bytes ingested: **111598** (109.0 KB)
- Total BBS bytes (estimated): **8904** (8.7 KB)
- **Compression ratio: 12.5x** (raw stderr / BBS)

## Prompt size invariance

- Range: **399..649** tokens (variance 250)
- All within B_PROMPT_MAX=5800: **true**

## BBS / distiller machinery

- Max constraints ever in any single BBS: **3**
- Max zero_gain_streak observed: **1**
- Distinct reject_classes observed: **["algebra-error", "bad-substitution", "direction-reversal", "logical-gap", "missing-equality-case", "off-stage"]**

## KILL guard surface

- Raw stderr leak in any prompt: **false** (KILL-tdma-1)
- Prompt within B_PROMPT_MAX in every retry: see above (KILL-tdma-9)
- verified_head never advanced on failure: structurally enforced by the kernel (KILL-tdma-5)

## Evidence integrity

- per_attempt_probes.jsonl sha256: f55eee581bc69c66ec24a9db63a9e3cb372569683f042768dbdb992f830cab9d
- chaintape.jsonl sha256: 2be1d1e409dfb887874c29755aff6bbc848c8e4fd2fb0620933191743eaee832
