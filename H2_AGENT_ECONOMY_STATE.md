# H2_AGENT_ECONOMY_STATE — 2026-06-16

**Branch:** `claude/het-converge-2026-06-16` (≥9 commits ahead of `origin/main`, NOT pushed).
**Tree:** cargo check --all-targets → **exit 0**.
**Latest:** Phase-4 trust-root rehash applied (`205fb5d9`, boot gate 8/8); live mechanism smoke
`smoke_ucb_001` PASS (omega + heterogeneous routing on replay-green tape, `ee006e54`); 6-auditor
adversarial QC done (QC-CONCERNS, no VIOLATION/no blocker, CONTINUE_STEP6) — claims rescoped. See
`H2_LIVE_MECHANISM_SMOKE_REPORT.md`.

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

## DONE this increment
- ✅ Classical.choice axiom whitelist aligned (het reds RED→GREEN; non-banked still fail-closed; audit dim-B PASS).
- ✅ GA-6b attempt-level decision_source (logic-witness; tape-canonical promotion still Class-4).
- ✅ Phase-4 trust-root rehash (Class-4 §6-Step-4, Veto-AI PASS, constitution.md untouched; boot 8/8).
- ✅ Live mechanism smoke `smoke_ucb_001` (omega + heterogeneous routing on canonical tape; replay byte-clean).
- ✅ 6-auditor adversarial QC + recursive audit (QC-CONCERNS, no VIOLATION; overstatements corrected).

## Required fixes BEFORE the paid confirmatory run (audit-derived)
1. **served_model provenance** (dim C) — on-tape model_id is the REQUESTED label, not served-model;
   proxy echoes requested label + discards upstream resp.model. Fix: return/record served_model + assert + test.
2. **MODEL_RATES → CAS at genesis** (dim D) — rate table is a compile-time const, not on tape; cost is
   only recomputable given the external table. Fix: write rates to CAS, CID in GenesisPin.
3. **Run-path budget CONSERVATION** (dim F) — `allocated_token_budget` hardcoded 900 vs balance −1
   (`lean_market_agent.rs:2139-2142`); conservation gate fires only on a synthetic fixture.
4. **BudgetAllocationTelemetry replay reconstruction** (dim E/F) — `verify.rs` reconstructs ProposalTelemetry
   but not allocation; add so replay witnesses allocation == derive_from_tape.
5. **binary-hash + HEAD in manifest** (dim E) — no binary↔source binding for the run; emit sha256(binary)+HEAD.
- **decision_source/action_source tape-canonical promotion** (Class-4) + **GA-9** (`enable_thinking:false`,
  pinned llm_http.rs, Class-4 §8) + **prereg freeze** (target pool / policy hash / budget cap / ≥12 seeds).
- Non-blocking bookkeeping (dim B): populate `axioms` from `parse_axiom_set` on Verified (now `[]`); remove dead `axiom_gate()`.

## Next allowed action (adjudicator: CONTINUE_STEP6)
Step 6 deep-chain target-pool calibration (chain ≥10/≥18, tx ≥ agents×20, axiom-clean; long-run discipline:
inner-unit checkpoint + --resume + binding-budget pilot) → Step 7 prereg freeze → Step 8 confirmatory pilot.
No paid hard-target run until the 5 fixes + GA-9 + prereg resolve (§11).
