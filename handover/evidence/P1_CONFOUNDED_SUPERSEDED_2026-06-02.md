# P1 evidence — SUPERSEDED for the autonomous-vs-market causal contrast (2026-06-02)

> **Annotation, not mutation** (AGENTS.md §8): this note ADDS context to existing evidence;
> the cells under `p1_realvalue_v3_2026-06-02/` and `p1_scaleup_2026-06-02/` are NOT rewritten.

## What two independent external auditors found (both VETO)

The P1 binary at HEAD `1a812204` (`src/bin/lean_market_agent.rs`) claimed `autonomous` and
`market` differ **only in who picks the node**. That claim is **false at the code level** — the
fatal confound (auditor B, ratified by auditor 2 as the authoritative report):

- `market` / `single` build the proof prompt with `build_prompt()` → **one** parent body + **one**
  shielded Lean error + librarian digest.
- `autonomous` built it with `build_autonomous_prompt()` → the **full frontier**: top-6 nodes'
  **full** bodies + **full** shielded errors + all-node snippets, **in the same LLM call that
  generates the proof**.

So `autonomous`'s cracks could be caused by a **~6× richer proof-generation context (global
failure-context synthesis)**, NOT by **free-choice routing**. The two arms are therefore **not** a
single-variable contrast, and the headline "Path-1 free-choice cracks what single + Path-2 cannot"
is **not causally warranted** by this data.

Secondary findings (both auditors): the prereg's control family (`parallel`, `shuffled_price`,
`no_price`) was **not run** in the counted sweep; the prereg PRIMARY subject is `market` (which
solved **0/18**), not `autonomous`; price arms make an extra skeptic (`bear`) LLM call that the
Bulls-only `single` baseline does not (compute asymmetry); the `#print axioms` whitelist gate was
**post-hoc**, not inline in the verifier.

## What is SUPERSEDED

- ❌ The **autonomous-vs-market** comparison in `p1_realvalue_v3_2026-06-02/` (54 cells) and the
  partial `p1_scaleup_2026-06-02/` (35 cells written before the run was stopped). The causal
  attribution to "free-choice routing" is confounded by the prompt-context advantage. **Do not cite
  these as a market/organization causal result.**

## What still HOLDS (independently re-checked, both auditors PASS)

- ✅ **Economic replay (Verdict B)**: all counted v3 cells `verify_chaintape` replay-clean
  (economic_state reconstructed from L4). Auditors flagged that *causal-cost* replay (LLM calls,
  prompt hashes, token counts, route decisions, price snapshots, axiom lists) is **incomplete** —
  addressed by the F2/telemetry correction.
- ✅ **The `lm_ineq1` proof is real**: `ineq_amgm_concrete` (3a²+5b² ≥ 2√15·ab) compiles under
  Lean+Mathlib; `#print axioms = {propext, Classical.choice, Quot.sound}` ⊆ whitelist (axiom-clean).
  Both auditors verified the math; neither could independently recompile (no repo/Mathlib in their
  sandbox) — so this stands as **mathematically sound + documented axiom-clean**, an **existence**
  result, not a causal one.
- ✅ **Statistics honesty**: McNemar/Holm `p_holm = 0.75` correctly NOT significant; the rarity
  (6-seed re-run: lm_deriv1 0/6, lm_ineq1 1/6) honestly disclosed; no PROVEN/causal language.

## The correction (in flight)

Workflow `wf8a015uq` (`p1-audit-corrections`) implements the second report's required fixes:

1. **F1 — decouple** `autonomous` into two LLM calls: Stage-1 route on a **compact** frontier
   summary (index/price/conf/error-class/depth/hash only — **no bodies, no errors**) → parent index;
   Stage-2 proof via the **byte-identical** `build_prompt()` market uses. A prompt-parity self-test
   (SHA-256 equal) is the load-bearing check that the confound is gone.
2. **F5 — inline `#print axioms` gate** in `LeanJudge::verify`; OMEGA fires only on axiom-clean.
3. **F2 — compute telemetry**: per-arm proposal/route/bear LLM-call + token counts in the manifest.
4. **F3 — topology baselines**: `single_restart`, `single_tree_no_price`, `parallel_restart`.

The **corrected re-run** (P1-RERUN) runs the full prereg control family + decoupled `autonomous`,
reports the prereg **primary** metric (market vs controls) **and** a separate decoupled-autonomous
prereg, and **replaces** the superseded comparison above.

## Honest claim going forward (auditor-recommended, adopted)

Not: *"the market organization solves hard problems a single model cannot, due to loss-bearing price
routing."* Instead, pending the corrected re-run: *autonomous (full-landscape free-choice) produced a
rare, mathematically-sound, documented-axiom-clean Lean proof of a single-0 hard theorem that single
and market did not — an **existence** result that is not statistically significant, did not reproduce
robustly, did not satisfy the preregistered market-vs-controls primary metric, and was **causally
confounded** by autonomous's broader prompt context until the F1 decouple.*
