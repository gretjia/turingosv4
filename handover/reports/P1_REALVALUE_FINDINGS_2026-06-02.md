# P1 Real-Value — Findings (2026-06-02)

> **The corrective primary experiment.** After the forensic retrospective (`SESSION_FORENSIC_RETROSPECTIVE_2026-06-01.md`)
> demoted the flat-allocation / rigged / synthetic emergence experiments, this is the first test of the constitution's
> actual value — **{loss-bearing price} × {non-local tree search}** — on a verified, replay-sound substrate, with both
> routing algorithms brought to equal audit rigor.
> Prereg (SHA-locked): `P1_REALVALUE_PREREG_2026-06-01.json` (`1936f71f` → parity-lock `621d079d`).

## 0. The honest headline

On HARD Lean theorems where a single chain **reliably fails (0/3 seeds, after failing 24/24 in the scan)**:

| arm | hard theorems cracked (of 6) |
|---|---|
| **autonomous** (Path-1: agent reads the full-chain landscape + freely chooses where to build/branch) | **2/6** — lm_deriv1, lm_ineq1 |
| **market** (Path-2: harness forces the node by softmax over the loss-bearing price) | **0/6** |
| **single** (one chain, equal total budget) | **0/6** |

- **Autonomous cracked 2 hard theorems that single *and* forced-softmax both could not** — confound-shielded (the hard floor flattens model/luck), at equal budget + equal repair-depth, every cell replay-clean, with **genuine routing** (0% hallucination across 412 routes).
- **Forced-softmax (market) = single** — both 0/6. At this budget on these theorems, the harness forcing nodes by price adds nothing over a single chain.
- **NOT statistically significant**: pairwise McNemar Holm-p = 0.75 (the cracks are sparse — 1/3 seeds each → only 2 discordant pairs in 18). This is a **directional existence signal, not a significant GO.**

**Verdict A (price-causal efficiency): INCONCLUSIVE — positive direction, not significant.** A real, replay-verified,
confound-shielded demonstration that **agent free-choice tree search (Path-1) cracks hard theorems a single chain and
forced price-routing (Path-2) cannot** — the architect's original thesis — but at 6 theorems × 3 seeds × 24-budget it
does not reach p<0.05. **Verdict B (governance): HOLDS** — all 54 cells `verify_chaintape` replay-clean (economic_state
reconstructed from L4), route telemetry genuine.

This is deliberately **under-claimed**: per AGENTS.md §17 it is a scoped, non-causal statement, not a PROVEN/causal headline.

## 1. Why this experiment is trustworthy (the rigor the prior ones lacked)

- **Substrate verified**: `lean_market_agent.rs` on the real ChainTape — loss-bearing price (WorkTx-Long + Bear
  ChallengeTx-Short → `compute_price_index`) + true softmax over the full live index + arbitrary-parent restart. Smoke:
  13/16 non-local restarts; `verify_chaintape` reconstructs state + economic_state from L4.
- **Real librarian** (`librarian_broadcast.rs`, not a lookalike) wired into every price arm, **held constant** — a
  control, not a confound (workflow-audited PROCEED, commit `05184f60`).
- **Both routing algorithms equal-rigor** (routing-parity-audit, 12-agent adversarial, `EQUAL-RIGOR-PROCEED`,
  commit `329fad1e`): it rejected wiring τ-annealing as a tune-to-win trap, found the *autonomous* arm was the
  crippled one (depth-handicap), and fixed it (equal shielded repair-depth on top-6 nodes); **price kept byte-identical**.
- **Route telemetry** (`route_valid_index_hit / deliberate_fresh_root / hallucinated_out_of_range`): makes "free
  routing helped" falsifiable. Observed 0% hallucination → the autonomous routing is the agent genuinely choosing
  valid/early nodes, not a model naming nonexistent ones.
- **Budget + depth parity** across arms; integer money; no §6 surface; FC1–3 untouched.

## 2. The two cracks (the load-bearing existence claim) + an honest gap

The 2 autonomous cracks are real Lean **kernel-Verified** proofs (omega fires ONLY on `LeanVerdictKind::Verified`):
- **lm_deriv1**: `HasDerivAt (fun x => x^3 - 2x + 5) 10 2` — a derivative proof (HasDerivAt composition).
- **lm_ineq1**: a Cauchy–Schwarz-type inequality over `Real.sqrt` (sum-of-squares nonnegativity).

Neither preview shows `native_decide`/`sorry`, and these analytic theorems are not the decidable-computation kind
`native_decide` solves. **HONEST GAP**: the bin does NOT enforce the `#print axioms ⊆ {propext, Classical.choice,
Quot.sound}` whitelist inline, and the runner reclaimed the CAS on replay-clean — so the specific cracks could not be
re-verified post-hoc. **[AXIOM-CONFIRM RE-RUN IN PROGRESS]** reproduces autonomous cracks (CAS preserved) and runs
`bench_axiom_reverify.py` on them. Until that lands, the cracks are *kernel-Verified but not formally axiom-confirmed*.
Harness fix (recommended): enforce the #print-axioms gate inline on omega + persist the verified proof (§17 G1/G6).

**POST-DATA §17 AUDIT VERDICT: CHALLENGE** (`handover/audits/P1_REALVALUE_AUDIT_2026-06-02.md`). The audit independently
re-ran everything and found the experiment **methodologically sound** — prereg SHA byte-verified, 54/54 replay-clean +
correctly filtered, budget/softmax/repair-depth parity intact, the softmax genuinely DISTRIBUTES (not argmax, §17.3
PASS), route telemetry falsifiable at 0% hallucination, McNemar/Holm correct + honestly non-significant, headline
scoped/non-causal (§17.1 PASS). The **one blocking gap**: the prereg's `axiom_gate` (`#print axioms ⊆ whitelist`) is not
enforced by the binary (a `sorry/native_decide` token-scan only) or the analyzer — so the 2 cracks satisfy the
*kernel-Verified* half but **not yet the *axiom-confirmed* half** of the preregistered SOLVED definition. **Required to
count the cracks:** the in-progress `bench_axiom_reverify.py` re-run must return both reproduced proofs with
`#print axioms ⊆ {propext, Classical.choice, Quot.sound}`. The directional/not-significant/Verdict-B framing is
**defensible as written**; the **2-crack count is provisional** until the axiom re-run lands GREEN. **[AXIOM-CONFIRM RESULT: PENDING]**

## 3. What it means

- **Path-1 (agent free-choice) > Path-2 (forced softmax) ≈ single**, directionally. The decisive mechanism is the
  **agent reading the full-chain price landscape and choosing where to build/branch itself** — not the harness pushing
  nodes by price. This is exactly the architect's "全链路价格开放、agent 自主回到早期节点重启" thesis, and it is the part
  that shows signal; the *forced* softmax routing does not beat a single chain here.
- The **existence** of even 2 confound-shielded cracks is the meaningful result on the architect's hard-problem design
  (the floor shields model/luck; a crack = real evidence the organization, not the model, is stronger). But cracks are
  **sparse and stochastic** (1/3 seeds), so this is a signal to **scale**, not a proven capability.

## 4. Honest limitations + next steps

- **Not significant** at this scale (6 theorems, 3 seeds, 24-budget). To test significance: more hard theorems + more
  seeds + a budget curve (the cracks may be budget-limited; autonomous may crack more at NA/NR↑).
- **Forced-softmax got 0/6** — worth probing whether a higher budget or a more *informative* price (the Bear-doubt
  price is a weak promise-proxy) lets the forced arm contribute; but that is a separate, prereg'd follow-up, not a
  retrofit to win.
- **Axiom-gate the harness** (inline whitelist + proof persistence) so future cracks are formally clean + auditable.
- **External posture stays Verdict B** until a confirmed, axiom-clean, *significant* crack set clears §17 G1–G6.

## 5. Discipline
FC1/FC2/FC3 untouched; no §6 surface; integer money; de-branded channel; prereg SHA-locked before the counted run +
parity-locked; every counted cell replay-clean; both routing algorithms adversarially audited equal-rigor; the headline
is scoped/non-causal per §17. Evidence: `handover/evidence/p1_realvalue_v3_2026-06-02/` (54 cells + replay reports).
