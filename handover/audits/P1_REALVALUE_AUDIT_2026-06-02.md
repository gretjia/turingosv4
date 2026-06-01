# P1 Real-Value — Post-Data §17 Clean-Context Audit (2026-06-02)

**Role:** independent witness, clean context, no implementation transcript (AGENTS.md §9, §17 G5).
**Audited run:** `handover/evidence/p1_realvalue_v3_2026-06-02/` (54 cells = {autonomous,market,single}×6 hard theorems×3 seeds).
**Prereg (verified byte-identical to its `.sha256`):** `P1_REALVALUE_PREREG_2026-06-01.json` sha256
`621d079d782d92367310d37844063f6e98670ae96e95f0a0b4a2cb993094a703`.
**Report:** `handover/reports/P1_REALVALUE_FINDINGS_2026-06-02.md`.

## Verdict: **CHALLENGE** (one blocking gap; verdict framing otherwise defensible)

### PASS (independently re-run)
- **Prereg integrity** — SHA byte-identical, locked, unmodified.
- **G1 replay** — 54/54 `.replay.json` `economic_state_reconstructed=true`; analyzer counts ONLY replay-clean cells
  (analyze_p1_hardproblem.py:46-47). Full CAS-recompute impossible (CAS reclaimed) — matches the report's disclosure; the
  warrant is the reconstruction flag, not a byte badge (§17.2 respected).
- **G3 fair baseline** — single auto-compensated (`effective_rounds=n_rounds*n_agents`, lean_market_agent.rs:655) →
  24 proposals each arm. autonomous vs market share the SAME `boltzmann_softmax_select_parent` (actor.rs:115, a genuine
  DISTRIBUTING softmax, NOT argmax — §17.3 PASS) over the SAME price index + identical FEEDBACK_MAX=240 repair depth.
  actor.rs/price_index.rs differ from main = the shared TRUE-softmax substrate fix applied EQUALLY to both price arms,
  not a per-arm advantage. No crippled/force-suicided rival.
- **Cracks reproduced exactly** — single 0/6, market 0/6, autonomous 2/6 (lm_deriv1 s3, lm_ineq1 s2). McNemar by hand:
  exact one-sided p(b=2,c=0)=0.25, Holm×3=0.75 ≫ 0.05 → correctly NOT significant.
- **Route honesty** — `route_hallucinated_out_of_range=0` across 412 routes; the counter genuinely CAN fire
  (lean_market_agent.rs:795-801, a distinct observable bucket) → "free routing genuine" is sound.
- **§17.1 no-PROVEN** — headline is scoped/non-causal (INCONCLUSIVE + Verdict-B); no PROVEN/DEFINITIVE/X>Y.

### CHALLENGE — the blocking gap
The prereg `/metric/axiom_gate` requires every counted crack to pass `#print axioms ⊆ {propext, Classical.choice,
Quot.sound}`. NEITHER the binary (lean_judge.rs:50 = a `sorry/admit/native_decide` SOURCE-TOKEN scan only, then
`lean -DwarningAsError` → Verified iff exit 0; no `#print axioms` emitted) NOR the analyzer (omega/verified only)
enforces the positive whitelist assertion. So the 2 cracks satisfy the *kernel-Verified* half but NOT the
*axiom-confirmed* half of the preregistered SOLVED definition. The token-scan blocks the common native_decide vector and
the cracks are analytic (HasDerivAt / Real.sqrt — not decidable-computation), so the risk is LOW, but the prereg DEFINED
the crack to require the whitelist, and that check has not been run on these cells (CAS reclaimed → not post-hoc-runnable
on this run's artifacts).

### Required before the cracks count (and before any posture past Verdict B)
The in-progress `bench_axiom_reverify.py` re-run (autonomous on lm_deriv1 + lm_ineq1, CAS preserved) must return both
reproduced proofs with `#print axioms ⊆ {propext, Classical.choice, Quot.sound}`. If GREEN → the directional 2-crack
signal stands as written. If either pulls an off-whitelist axiom → that crack does not count, autonomous tally drops.
**Durable fix (concur with report line 78):** enforce the whitelist inline on omega + persist the verified proof body.

### Disposition
The **directional / not-significant / Verdict-B framing is fully defensible.** The **"2 cracks" count is provisional**
(kernel-Verified, not yet axiom-confirmed) and is correctly self-gated by the report on the in-progress axiom re-run.
