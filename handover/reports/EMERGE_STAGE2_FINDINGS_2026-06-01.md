# EMERGE Stage-2 Findings — the sampling control kills "combination = capability" (honest negative)

**Date:** 2026-06-01 · **Prereg:** EMERGE_STAGE2_PREREG_2026-06-01.json + _ADDENDUM_ (both locked before any
result was read) · Frozen cells: `emerge_stage2_qwen_cells` (k4), `emerge_stage2_v4pro_k8_cells`,
`emerge_stage2_qwen_k12_cells`.

## Verdicts against the locked prereg

| question | verdict |
|---|---|
| **A — verifier coverage** | **VALIDATED** (Stage-1): sampling + Lean ≈ 1.5–2× single-shot for strong models |
| **Q2 — cross-family helps at equal PER-MODEL budget** | **YES**: union(3)@k4 = 30/44 > best single = 23/44; qwen3.7-max adds 4 it alone-misses |
| **Q1 — is the combination REAL capability or just more draws?** | **JUST_SAMPLING (decisive)** |

## The decisive control (Q1) — equal TOTAL compute

The honest test of "does heterogeneity buy CAPABILITY" is: best single model at the SAME total draws as the swarm.

| arm | draws/theorem | solved |
|---|---|---|
| union(v4-pro@k4, V3.2@k4, qwen3.7-max@k4) — 3-model swarm | 12 (3×4) | **30/44** |
| **qwen3.7-max @ k12 — ONE strong model** | 12 | **31/44** |
| v4-pro @ k8 (secondary control) | 8 | 24/44 (vs 2-model union 26) |

**A single strong model with 12 draws SOLVES MORE (31) than the 3-model heterogeneous union at equal compute
(30).** The swarm does not just fail to beat one strong model — it slightly *underperforms* it (diversity
coordination overhead with no capability payoff). Rule was: REAL if qwen@k12 ≤ 27, JUST_SAMPLING if ≥ 29.
Result 31 ⇒ **JUST_SAMPLING, unambiguously**.

## What this means (no spin)

- **Heterogeneous strong-model combination is NOT a capability lever on real Lean.** The Stage-1 "+8" and the
  Stage-2 Q2 "+4 cross-family" were real at equal *per-model* budget but are **artifacts of more total draws** —
  the equal-*total*-budget control erases them. The architect predicted this; the data confirms it.
- **Verifier-coverage IS real** (Stage-1 A) — sampling + a hard verifier rescues p>0 theorems. The rational use
  of extra compute is *more draws of the strongest single model*, not a diverse swarm.
- **The hard ceiling stands**: 13/44 theorems no configuration solved (qwen@k12 none-solved = 13). Breaking
  these needs DEPTH (thinking-mode / tree search), not breadth — out of scope here.

## Consequence for the Market-Mission

This is a **de-risking negative**: it kills the wrong direction before it cost a counted sweep.

- **TP-4 (effective-independent-N) is DEMOTED/PARKED** exactly as the execution plan pre-committed for the
  JUST_SAMPLING branch — there is no diversity-coverage headline to chase; if pursued at all it becomes a
  reputation + cost-accounting study on an untrusted pool, not an N_eff capability claim.
- It **sharpens the mission reframe**: TuringOS's defensible value is NOT capability-combination (dead) and is
  unlikely to be raw efficiency (the repo's H0 prior + now this). It is **Verdict B — auditable, Sybil-resistant,
  Goodhart-shielded, replayable agent governance** under untrusted/heterogeneous agents. The T2 experiment
  should be read primarily through Verdict B, with Verdict A (price-causal efficiency) as the falsifiable upside.

## Data quality

Q1 ran on the per-theorem-resumable harness (commit 103b239), #print-axioms whitelist enforced per solve,
wall 4180s, replay-honest. v4-pro@k8 (24) and qwen@k4 (23) cross-check the trend. No harness-bug artifact: the
single-model @ high-k climbs smoothly (v4-pro 18→24 at k4→k8; qwen 23→31 at k4→k12) — pure, clean sampling.
