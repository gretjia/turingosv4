# H2_AGENT_ECONOMY_STATE — 2026-06-16

**Branch:** `claude/het-converge-2026-06-16` @ `d288a63e` (6 commits ahead of `origin/main`, NOT pushed).
**Tree:** 0 tracked-dirty; 4 untracked (reports/scratch + this file). cargo check --all-targets → **exit 0**.

## Constitutional Veto Findings
- Art-0.2: H-HET-2 routing/budget DECISIONS are tape-canonical (GA-2/3/5/6a green). **Open gap:**
  `decision_source` (parse_fallback/llm_error classification) is a BIN field on `AttemptNode`, NOT
  on any tape record → cannot be a PRIMARY-metric basis until promoted to a versioned tape schema
  (GA-6b logic-witness now; tape-canonical promotion = Class-4 before confirmatory run).
- Art-0.4: NOT claimed closed (system-level Q_t debt remains). H-HET-2 records ride existing schema.
- Trust-root: deferred (Phase-4). genesis pins held at main's; merged source differs → `tc_boot_trust_root_manifest` red is EXPECTED until the Class-4 §8 rehash.

## Engineering Findings
- Gates: **205 registered**, manifest drift 0, matrix drift 0.
- H-HET-2 economic gates (all GREEN, failable, registered): GA-0 policy_hash_frozen, GA-5
  budget_conservation, GA-6a decision_source_complete (allocation-level), GA-2 headline_recompute,
  GA-3 dag_reconstructible, GA-7 router_name_lie, GA-8 arm_parity.
- No-LLM `VERIFY_UCB_PRICE_PRIOR_FLOOR_V1` sim (tests/h2_economy_sim.rs): UCB 4cov/7200tok/1800PPUT
  vs round-robin 4/8400/2100 vs best-single 3/15600/5200. SMOKE only (synthetic, single seed,
  non-binding budget) — audited non-rigged (real score_and_select, equal budget, non-vacuous asserts).

## Known reds (20; classified — none are de-Lean/merge code regressions)
- 15 `true_suite_*` — environment (need real Lean/LLM/network/benchmark).
- 1 `tc_boot_trust_root_manifest` — expected Phase-4-pending (trust-root deferred).
- 1 `het_tape_reconstructibility` — merge-induced (`AgentManifestRequired` vs het bootstrap).
- 2 `obl005_final_closure_witness` — OBLIGATIONS union truthfully has OBL-018/021 open (het-baseline).
- 1 `production_module_liveness` + 1 `script_liveness_inventory` — pre-existing documented liveness.

## Blockers before live-smoke / confirmatory (status this increment)
1. **Classical.choice axiom whitelist** — `DEFAULT_ALLOWED_AXIOMS=[propext,Quot.sound]` excludes the
   banked `Classical.choice` (in `AXIOM_WHITELIST`); het det-family proofs rejected. → **fixing now**
   (Step 2, surgical, §11.5-compliant). Gates live verification.
2. **GA-6b** attempt-level decision_source completeness — **implementing now** (Step 3, logic-witness;
   tape-canonical promotion flagged Class-4 before confirmatory).
3. **Phase-4 trust-root rehash** (Class-4 §8 on merged bytes) — needed for ship/replay-trusted run.
4. **GA-9** (`enable_thinking:false`, pinned llm_http.rs, Class-4 §8) — needed for valid hard-target run.
5. **Prereg freeze** (target pool / policy hash / budget cap / exclusions / ≥12 seeds) — before confirmatory.

## Next allowed action
Integrate Step-2 (axiom) + Step-3 (GA-6b) → run ONE tiny live dynamic-router mechanism smoke
(gateway localhost:8123 reachable, Lean ~/.elan present) → `H2_LIVE_MECHANISM_SMOKE_REPORT.md`.
Then deep-chain target-pool calibration → prereg freeze → confirmatory pilot. No paid hard-target
confirmatory run until GA-6b(tape)/GA-9/trust-root/prereg resolved (§11).
