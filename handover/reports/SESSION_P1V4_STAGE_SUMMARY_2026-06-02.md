# Stage Summary — P1 v4 corrective experiment + verdict (2026-06-02)

Branch `claude/p1-realvalue` (HEAD `841c08e9`, 44 ahead / 5 behind `origin/main`). Not pushed. PR-only.

## The arc this session

1. **Two external audits VETO'd the v3 P1 causal claim.** Authoritative (2nd) report's fatal finding —
   **confound B**: `autonomous`'s proof prompt fed the full search landscape (top-6 full bodies + errors)
   while `market` saw one parent, so the v3 "autonomous crack" could be 6× richer context, not free-choice
   routing. Plus: prereg controls never run; axiom gate post-hoc; compute asymmetry hidden.

2. **Implemented the corrections** (workflow `wf8a015uq`, research→implement→LLM-free sandbox→2× adversarial
   audit→repair; both PROCEED). Commit **`cb89a5d6`** (Class 2, `lean_market_agent.rs` + `lean_judge.rs` only):
   - **F1 decouple** — `autonomous` → Stage-1 route on a COMPACT summary (`build_route_summary`:
     idx/price/conf/error-CLASS/age/hash, no body/error/librarian) → Stage-2 proof via the ONE shared
     `stage2_proof_prompt` = the *same* `build_prompt` market uses. Byte-identical proof prompt ENFORCED by
     `--self-test` (`sha(stage2)==sha(market) && != confound_control`, mutation-tested). Confound B removed.
   - **F5 inline axiom gate** — `LeanJudge::verify` routes a clean exit-0 compile through `axiom_gate`
     (`#print axioms`); Verified IFF axiom set ⊆ {propext, Classical.choice, Quot.sound}; fail-closed.
   - **F2 telemetry** — manifest carries proposal/route/bear LLM-calls + tokens; invariant
     `total = proof+route+bear+completion`; route counted (no autonomous discount).
   - **F3 baselines** — `single_restart`, `single_tree_no_price`, `parallel_restart`.
   - Verified 真跑: self-test PROMPT-PARITY-OK; bin 13/0; judge 12/0 real Lean; constitution gates 164/166
     (2 reds = pre-existing OBLIGATIONS.md + script-inventory drift, attributed); §6 clean; integer money.

3. **Found a flaw DEEPER than the auditors':** the v3 "hard" theorems weren't hard. `single` gets 24
   attempts/cell; the v4 floor shows **`single` SOLVES `lm_ineq1` axiom-clean** (the v3 flagship "crack",
   same AM-GM route). v3's "single 0/3" was a ~5% small-sample fluke. **Re-established the hard floor at
   `single` 0/6** → 13 of 18 candidates robustly hard (5 excluded, incl. lm_ineq1 1/6, lm_det_zero 6/6).

4. **Ran the corrected 9-arm experiment** (prereg `d80ba8cd`, SHA-locked; `run_p1_v4_parallel.sh`, JOBS=6):
   STAGE A floor (single ×18×6) → STAGE B crack (5 arms ×13×6) → STAGE C topology (3 arms ×13×6). **732
   cells, all `verify_chaintape` replay-clean, every crack axiom-clean** (inline gate).

5. **VERDICT (`31faaeb0` report, `841c08e9` audited):** see `P1_REALVALUE_V4_FINDINGS_2026-06-02.md`.
   - **PREREG_1 (market-Hayek): NOT confirmed.** `market` (real price) LOSES to `no_price` (random) 2 vs 6
     and `shuffled_price` 2 vs 5; `p_holm=1.0`. The loss-bearing price-routing claim is not supported —
     real price does WORSE than destroyed/random price.
   - **PREREG_2 (autonomous-freechoice): NOT confirmed.** `autonomous` uniquely cracks 1 theorem
     (`lm_deriv1`, real+axiom-clean), `p_holm=1.0`, and is the most expensive (3.09M tok/solve) / least
     productive (1/78) arm. Free choice genuine (0% route hallucination), just doesn't help.
   - **The lever is NON-LOCAL TREE SEARCH, not price.** `single_tree_no_price` (1 agent, restart from any
     own node, no price) = 5.1% @ 329K tok/solve — beats market/autonomous/parallel. Adding the REAL price
     signal is the biggest negative delta (`shuffled_price`→`market` −3.8pp). **Directional, NOT
     significant** (sparse cracks; all `p_holm=1.0`).
   - **Honest framing:** multi-agent beats single chain (existence: 24 axiom-clean cracks; 6/13 cracked by
     nobody) but the gain is **sampling + non-locality, not the constitution's price mechanism**.

6. **Clean-context audit (§17 G5):** `handover/audits/P1_REALVALUE_V4_AUDIT_2026-06-02.md` —
   **`NO-VIOLATION` + `PROCEED`**. Independent auditor recompiled 4 cracks under real Lean+Mathlib (all
   axiom-clean), re-derived McNemar+Holm by hand, ran the self-test, confirmed equal-budget + token
   invariant on all 732 cells + softmax distributes, checked the reverse (p=0.96 → NO-GO not over-stated).

7. **Constitutional-compliance check (in flight, workflow `wf7b6ms3m`):** does the market faithfully
   instantiate FC1/FC2/FC3 with broadcast/shielding/veto/predicates LIT? → `MARKET_FC_COMPLIANCE_2026-06-02.md`.

## Honest scope of the NO-GO

P1 tested price as a **router** of a homogeneous-agent, abundant-budget proof-tree search. The NO-GO is
for *price-routing in this regime*. It does **not** refute price **allocating a scarce shared budget across
complementary specialists** (LEAN-ALLOC / PROBE-ALLOC — a different mechanism, untested here, and the
steelman of the constitution's price claim).

## Open decisions (architect)

- **PR:** branch is 44-commit (v3+v4), 5 behind main. Recommend a clean v4-only PR off latest `origin/main`.
- **Next line:** (A) power-up non-locality to significance on the crackable band; (B) genuinely harder
  theorems (6/13 cracked by nobody → narrow band); (C) **price × scarce-budget-allocation** (the steelman);
  (D) accept NO-GO, advance Mission Stage-2. (My lean: C — test price where it should have the best shot.)

## Key artifacts

- Binary/judge: `src/bin/lean_market_agent.rs`, `src/judges/lean_judge.rs` (committed `cb89a5d6`).
- Prereg: `handover/preregistration/P1_REALVALUE_V4_CORRECTED_PREREG_2026-06-02.json` (+`.sha256`).
- Runner: `scripts/run_p1_v4_parallel.sh`; analyzer: `scripts/analyze_p1_v4.py`.
- Evidence: `handover/evidence/p1_v4_2026-06-02/` (732 manifests + replay; raw tape/CAS gitignored, pruned
  to the 35 crack cells, 132 MB).
- Report: `handover/reports/P1_REALVALUE_V4_FINDINGS_2026-06-02.md`. Audit: `…/audits/P1_REALVALUE_V4_AUDIT_2026-06-02.md`.
- Superseded note: `handover/evidence/P1_CONFOUNDED_SUPERSEDED_2026-06-02.md`. Audit-VETO record:
  `…/audits/P1_EXTERNAL_AUDIT_VETO_AND_CORRECTION_2026-06-02.md`.
