# Strong-model probe findings — empirical basis for the verifier-coverage thrust

> 2026-05-31. Live probes on the real proxy (:8123) + real Lean, measuring strong-model behavior on the
> 44-theorem pool, to ground the architect's "coverage vs combination, is high-temp lucky-guess worth it"
> question with DATA before designing the experiment. These findings change the experiment design.

## Finding 1 — single-shot p is NOT 0 for strong models; it has high VARIANCE (sampling luck is real)
On the easiest pool theorem `lm_smoke : n + 0 = n` (reference proof = just `simp`):
- deepseek-reasoner, single-shot run #1: emitted `induction n with k ih ...` — **Lean4-outdated syntax**
  (the old combinator form; Lean4 now needs `induction n with | zero => | succ k ih =>`) → REJECTED.
- deepseek-reasoner, single-shot run #2: emitted `simp` → **SOLVED in round 0.**
Same model, same trivial theorem, two draws → one wrong-syntax miss, one clean solve. **p ≈ 0.5 with high
variance, not p = 0.** My first batch probe (0/5 for ALL models incl reasoner) under-measured p because it
was single-shot, no feedback — it caught the BAD draws.

## Finding 2 — the misses are largely SYNTAX/VERSION variance, exactly what a verifier+retry fixes
The failures are not "the model can't do the math" — they are wrong Lean4 syntax (`λ h,` Lean3 form,
`induction ... with k ih` old combinator, wrong Mathlib lemma name). These are precisely the errors a
**Lean error-feedback loop** or **multi-sample coverage** corrects: the model knows the proof, it just needs
the verifier to reject the bad surface form and either retry or have another sample land the right one.

## Finding 3 — this IS the first-principles answer to "is high-temp lucky-guess worth it"
Because the Lean verifier is ~free (sub-3s) and picks ANY correct sample:
- pass@k = 1 − (1−p)^k. With p≈0.5 and the observed variance, k=2-4 samples ≈ certain solve on easy theorems.
- **Lucky-guess (high-temp coverage) is RATIONAL whenever a cheap verifier exists AND p>0** — the verifier
  converts the system from vote-limited (Condorcet ½ ceiling) to coverage-limited (no ceiling). This is the
  Monkeys / inference-scaling result, observed live on our substrate.
- The HARD limit stands: for a theorem genuinely beyond the model (p≈0), no k rescues it — that residual is
  the real frontier, and the only way past it is COMBINATION (a stronger/complementary model raising p>0).

## Implications for the experiment design (what changes)
1. **The harness MUST be multi-sample + Lean-feedback, NOT single-shot.** Single-shot under-measures every
   model's true capability (catches bad-syntax draws). The market's bank-if-any-passes (coverage) primitive
   is the right shape; a per-attempt Lean-error-feedback retry is the stronger one.
2. **Measure p properly at k samples**, not k=1, or the whole capability picture is wrong (and the prediction
   model would be mis-fit — this is also the Schaeffer anti-Mirage point: fit the smooth p, not the jumpy 0/1).
3. **The pool's "hard residual" must be recomputed at k≥4 with feedback** — at k=1 it looked like ~0 for
   strong models, which is a measurement artifact, not the real headroom band.
4. **Coverage (one strong model × big k + verifier) is the FIRST capability to demonstrate** — the probes
   show it directly converts strong-model variance into solves. Combination (heterogeneous strong models)
   is the second, for the p≈0 residual that coverage cannot reach.

## Substrate notes
Strong models live + reachable: deepseek-reasoner, DeepSeek-V3.2, Qwen3-32B, Qwen2.5-72B. The bin's LLM
client speaks to :8123 with any model string; per-agent model assignment is a ~1-flag change. Lean
v4.24.0 toolchain verifies; note the bench_axiom_reverify.py path may need a v4.30.0 update (flagged).
FC1/2/3 untouched; these are read-only probes.
