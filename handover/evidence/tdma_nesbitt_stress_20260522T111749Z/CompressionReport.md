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
| Step2-Rewrite | 4 | 2 |
| Step3-Expand | 4 | 2 |
| Step4-Group | 3 | 1 |
| Step5-ApplyAMGM | 3 | 1 |
| Step6-Sum | 3 | 1 |
| Step7-Subtract | 2 | 1 |
| Step8-Conclude+Eq | 2 | 1 |

## Compression evidence

- Total attempts: **25** (17 failed + 8 successful + 0 escalated)
- Total raw stderr bytes ingested: **210807** (205.9 KB)
- Total BBS bytes (estimated): **16756** (16.4 KB)
- **Compression ratio: 12.6x** (raw stderr / BBS)

## Prompt size invariance

- Range: **399..686** tokens (variance 287)
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

- per_attempt_probes.jsonl sha256: 983ea80b0d5c908a7c50cd89f781a4ac966a8565ae8c5e43e58146f9ce52a78d
- chaintape.jsonl sha256: 0b5aa3c7f1a94353d73f58e24a9fc6ae5854b3b02f16602cc72b6bae776126b8
