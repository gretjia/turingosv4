# Pre-registration — Hard Lean Market Go/No-Go

**Phase:** TuringOS Hard Lean Market Go/No-Go — price-routed collective proof search under equal budget
**Date registered:** 2026-05-30
**Risk class:** 0 (this document) — the experiment it governs is Class 1-2
**Plan of record:** approved plan `fuzzy-snacking-pascal` (this session)
**Status:** REGISTERED — variables below are LOCKED before any counted run. No edit-while-running.

> This charter is a deliverable in its own right. It exists to convert a prior belief
> ("a price-routed market should beat a single agent on hard problems") into a
> **falsifiable, pre-registered** result. Locking arms / metrics / theorem-set / thresholds
> *before* unblinding is the only defense against goalpost-moving and confirmation bias.

---

## 1. The single falsifiable question

> **At equal model, equal token budget, equal wall-time, equal verifier — is the
> price-routed market significantly stronger than (a) a single agent, (b) no-price
> multi-agent, and (c) majority-vote / best-first sampling — on hard Lean theorems whose
> proofs are kernel-verified?**

- **Thesis A (this phase, equal model):** MARKET > {single, no-price-multi, majority, best-first}.
  Isolates the *market mechanism*.
- **Thesis B (A-greenlit fast-follow, cross model):** a Flash-swarm market beats a single
  large deep-thinking model on **cost-per-solved** and **time-to-first-proof**. B depends on
  A for attribution (else "swarm wins" confounds with "Flash is just cheaper").

A NO-GO on A is not a failure: it re-positions TuringOS from "the market thinks" to
"auditable AGI-OS / governance substrate."

## 2. Locked parameters

| Field | Decision |
|---|---|
| **Main verifier** | Lean kernel (toolchain v4.24.0, Mathlib via the minif2f lake project). OMEGA only on full `Verified`. |
| **Side verifier** | SWE-bench hidden-test harness — **shadow-run only**, scoped to the 4 locally pre-built Docker images; never a headline, never PR'd. |
| **Base model** | ONE base model across all Thesis-A arms (DeepSeek `deepseek-chat` via the local proxy `:8123`). Not mixed. A big deep-thinking arm is added ONLY in the Thesis-B fast-follow. |
| **Budget parity** | every arm gets equal total token budget, equal wall-time cap, equal tool access. Reported per cell. |
| **Headline metrics** | resolve-rate@budget · cost-per-solved (USD) · time-to-first-proof · the price-routing ablation contrast |
| **Secondary metrics** | PPUT (`golden_path_tokens/(total_tokens×wall_s)`) · price entropy · branching factor · failed-branch reuse · Lean-error-recovery rate |
| **Hard gate** | every counted cell replays clean via `verify_chaintape` (state + economic_state from ChainTape L4 alone) or it does not enter any headline. |
| **Forbidden** | weak-judge OMEGA · human/LLM subjective scoring · best-of-one reporting · goalpost-moving after unblinding. |

## 3. Verifier integrity (non-negotiable — stronger than `run_lean_checker`)

The reusable in-tree verifier `run_lean_checker` (`src/top_white/predicates/registry.rs:1220`)
treats Lean exit-0 as pass. That is **insufficient** for this experiment:

- **`sorry` / `admit` / `native_decide`:** Lean exits 0 on a `sorry` (it is a warning, not an
  error). A `sorry`-bearing proof exiting 0 would be a FALSE `Verified`. `LeanJudge` MUST
  reject if the proof **source** contains `sorry`/`admit`/`native_decide` (token scan) OR the
  Lean **output** contains a "declaration uses 'sorry'" warning. `Verified` ⟺ exit 0 **and**
  source-clean **and** output-clean. This maps to `LeanVerdictKind::SorryBlocked → no OMEGA`.
- **Mathlib resolution:** `lean <file>` resolves `import Mathlib` only with the correct cwd /
  `LEAN_PATH` against the prebuilt `.lake/` cache. This MUST be validated by a real run
  (verify a known-good `e1_proofs` proof returns `Verified`, and a `sorry`-mutated copy
  returns `SorryBlocked`) **before** any counted cell. If Mathlib cannot be resolved in this
  environment, the phase halts until it can.
- **Verdict mapping:** use the canonical `LeanResult::derive_verdict_kind_from_legacy_fields`
  (`attempt_telemetry.rs:691`); never invent a parallel mapping.

OMEGA fires **only** on `LeanVerdictKind::Verified`. `Failed` / `PartialAccepted` /
`SorryBlocked` / `ParseFailed` → no OMEGA, and each becomes a `verified=false` tape node.

## 4. Arms

| arm | definition |
|---|---|
| **MARKET** (treatment) | the G1 price-routed loop: `boltzmann_select_parent_v2` over `compute_price_index`, WorkTx-Long (Bull) / ChallengeTx-Short (Bear), integer CPMM, Lean-verified, OMEGA settles. |
| **A0 SHUFFLED-PRICE** (decisive ablation) | byte-identical to MARKET except the price vector fed to parent-selection is randomly permuted each round. Same tape, trades, compute, replay — only the routing signal is destroyed. |
| **B1 single-agent** | one agent, same model, same total budget as the market's N agents summed; best of its own chain. |
| **B2 parallel-sampling** | N independent agents, no shared tape, no prices; first kernel-`Verified` wins. The "is it just best-of-N" control. |
| **B3 no-price multi-agent** | N agents share the work-DAG but parent selection is uniform-random (prices stripped). Isolates *price* routing from *shared-tape* collaboration. |
| **B4 majority-vote** | N independent proofs; accept only if ≥k agree (verifier still gates). |
| **B5 best-first** | shared tape, parent = argmax(progress heuristic), no betting / no short / no CPMM. Isolates the priced-adversarial mechanism from greedy frontier search. |

## 5. Theorem set protocol

| suite | count | role |
|---|---|---|
| Calibration | 10-20 | confirm LeanJudge / failure feedback / tape / baselines all work end-to-end |
| **Hard Pilot** | 30-50 | the market-uplift read |
| Shadow SWE | 5-10 | bind the Docker verifier to the market loop (4 pre-built images); integration-seam only |

Hard-Pilot selection rules (locked):
- **single-agent pass@1 ∈ ~10-40%** (measured in P0; too easy → no signal, too hard → all PPUT=0);
- **branching / decomposable** proof search (multiple plausible lemma paths) — the market's edge
  is exploration + dead-branch pruning, NOT single-thread depth; pure-depth problems are out;
- **exclude** the already-solved `handover/evidence/e1_proofs/*` lineage (no headroom);
- **not** MiniF2F / plain IMO-Putnam (saturated → a model arms race);
- **no** open conjectures / unproven theorems (no clean go/no-go).
- A **held-out** split is fixed in advance; tuning is permitted only on the calibration/pilot
  split, and the held-out split decides the headline.

## 6. Statistics

- Unit of analysis = the (theorem × seed) **cell**, blocked by theorem.
- ~8-12 hard theorems × 8 seeds per arm (≈64-96 cells/arm). Seeds varied via the existing
  per-agent seed (`0xB01 + round*31 + ai`) exposed as a CLI arg.
- Primary test: paired permutation (sign-flip) on per-cell `Verified` indicators + a
  per-theorem-stratified bootstrap of the resolve-rate difference; cluster by theorem.
- Time-to-first-proof: stratified log-rank / paired Wilcoxon with right-censoring at the cap.
- Multiplicity: Holm-Bonferroni across the baseline family (6 comparisons) at family α = 0.05.

## 7. Pre-registered decision rule (binding)

```json
{
  "decision_rule": "hard-lean-market-gonogo/v1",
  "registered": "2026-05-30",
  "GO_requires_all": [
    "MARKET resolve-rate >= strongest_baseline + 15pp  OR  cost_adjusted_success >= 1.5x",
    "MARKET strictly beats A0_SHUFFLED_PRICE AND B3_no_price_multi on resolve OR time-to-first-proof (paired p<0.05, Holm-Bonferroni)",
    "MARKET >= B4_majority_vote AND >= B5_best_first on resolve",
    "100% of counted cells replay-green; every OMEGA a real Lean Verified (no sorry)",
    "mechanism corroboration (>=2 of): price-entropy collapse in MARKET but not A0/B3; failed-branch-reuse>0 and > best-first; short-pressure concentrated on abandoned branches",
    "advantage holds as a CURVE across difficulty AND >=2 budget points (not one operating point)"
  ],
  "WEAK_GO_if": "resolve not significantly higher, BUT time-to-first-proof OR cost-per-proof clearly beats baseline -> 'cheaper/faster auditable substrate, not a stronger solver'; triggers the Thesis-B cross-model fast-follow",
  "NO_GO_if_any": [
    "A0_SHUFFLED_PRICE and/or B3_no_price_multi tie MARKET within noise (p>=0.05 or effect < threshold) -> parallel-sampling verdict",
    "MARKET fails to beat B2_parallel_sampling by the threshold at equal N+budget+walltime",
    "MARKET fails to beat B1_single_agent at equal total budget",
    "MARKET loses to B4_majority_vote or B5_best_first",
    "resolve rises but mechanism signals absent (uplift not attributable to price-routing)",
    "wins only after tuning and fails on the held-out split"
  ],
  "INVALID_if_any": [
    "any counted cell fails verify_chaintape replay",
    "any OMEGA is not a genuine Lean Verified (sorry/admit/native_decide leaked)",
    "the task proves easy (single-agent near ceiling) -> re-run on harder theorems"
  ]
}
```

## 8. Out of scope this phase (cuts)

No 90-agent scale run before N=4/8/16 shows a difference; no `sequencer` / monetary-invariant
edits before the L4.E trace (P0-F) proves a real invariant bug; no MiniF2F / plain IMO-Putnam
as the main benchmark; no open conjectures; no human/LLM-judge result called OMEGA; no strong
external PR of the first SWE-bench results; open-world PCP / world-predicate deferred to phase 2
(architect to supply research).

## 9. Amendment policy

Any change to §2-§7 after a counted run begins requires a NEW versioned pre-registration file
(`..._PREREG_<date>_vN.md`) stating what changed, why, and a timestamp — committed BEFORE the
next counted run and BEFORE looking at outcomes. The original remains in the record.
