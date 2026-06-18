# H2_ECONOMY_SMOKE_REPORT — dynamic model-budget market (no-LLM)

**Date:** 2026-06-16 · **Branch:** `claude/het-converge-2026-06-16` (off `origin/main`, not pushed).
**Scope:** no-LLM simulation smoke of `VERIFY_UCB_PRICE_PRIOR_FLOOR_V1` + the minimum Phase-2
economic-claim gates. **No paid run. No trust-root pin / GA-9 / llm_http.rs touched.**

The real track (not "heterogeneous roster"): **dynamic model-budget allocation** — budget flows to
higher-expected-value models while preserving exploration, achieving more union coverage and/or
better token-pure PPUT at equal-or-lower budget.

---

## 1. Verification checkpoint
- `cargo check --all-targets` → **exit 0**.
- Manifest cross-check (discovered ⇄ manifest): **0 orphans both ways** → **manifest drift = 0**.
- `cargo test --test constitution_matrix_drift` → **3/3 PASS** → **matrix drift = 0** (after
  registering the 7 H-HET-2 gates in `CONSTITUTION_EXECUTION_MATRIX.md`).
- `cargo test --all-targets` NOT run in full: pulls the 46-min broad-AGI `true_suite_*` runners
  (need real Lean/LLM/network). Ran the H-HET-2 + merge-relevant gate subset green instead.
- `scripts/run_constitution_gates.sh` → **total=201, failed=21 → 20 after matrix fix.** All 20
  reds classified, **none are de-Lean/merge code regressions**:
  - **15** `constitution_true_suite_*` (boot_cli/cybench/fc3_governance/gaia/generate_artifact/
    gpqa/market_external_agent/math/mind2web/osworld/replay_cas_tamper/swebench/tdma/toolbench/
    webarena) — **environment reds** (need real Lean/LLM/network/benchmark; red on any non-resourced box).
  - **1** `tc_boot_trust_root_manifest` — **EXPECTED Phase-4-pending**: trust-root intentionally
    un-rehashed (genesis pins held at main's; merged source differs) until the Class-4 §8 rehash.
  - **1** `het_tape_reconstructibility` — merge-induced: main's `AgentManifestRequired` admission
    postdates het's Gate-D bootstrap (documented; fix = register manifest + sign TaskOpen).
  - **2** `obl005_final_closure_witness` — OBLIGATIONS union truthfully has OBL-018 in_progress /
    OBL-021 blocked (true state; het-baseline, not a regression).
  - **1** `production_module_liveness` + **1** `script_liveness_inventory` — pre-existing documented
    liveness reds (script/module inventory).
  - macOS note: the runner uses GNU `grep -P`; no `ggrep` here — the awk cross-check is the portable
    equivalent. Full-suite green is Linux/CI-deferred.

## 2. GA-6 claim-boundary correction (per architect)
The earlier GA-6 over-claimed. Corrected to **GA-6a — ALLOCATION-provenance completeness**: it
witnesses that every `BudgetAllocationTelemetry.selected_model_id` ⊆ scored `candidates` (no
ghost/untraced allocation), at the **lib level only**. It does **NOT** witness every
`AttemptNode.action_source/decision_source` in the actual carrier tape (those live in the
`lean_market_agent` bin, not importable by an integration test). **GA-6b (attempt-level
decision_source completeness) is REQUIRED before any paid confirmatory run** — by making the carrier
telemetry importable/testable or via a tape-replay witness. Recorded in the gate header + manifest authority.

## 3. Minimum Phase-2 economic gates — status (all GREEN, all failable, registered)
| gate | status |
|---|---|
| GA-0 `constitution_h2_policy_hash_frozen` | 🟢 2/2 — frozen pin == prereg `9fb0f612…`; mutated param diverges |
| GA-5 `constitution_h2_budget_conservation` | 🟢 3/3 — before−allocated==after, Σ≤B_target; leak caught |
| GA-6a `constitution_h2_decision_source_complete` | 🟢 2/2 — allocation ⊆ scored candidates; ghost caught |
| GA-2 `constitution_h2_headline_recompute` | 🟢 6/6 — union_delta/share/token/PPUT recompute from fixture tape; tamper caught |
| GA-3 `constitution_h2_dag_reconstructible` | 🟢 4/4 — BudgetDecision→Proposal→Verification→OMEGA; missing funding/edge → red |
| GA-7 `constitution_h2_router_name_lie` | 🟢 5/5 — UCB bonus varies w/ pull_count; ε-floor fires; not argmax |
| GA-8 `constitution_h2_arm_parity` | 🟢 6/6 — treatment ≤ controls' token/microUSD/proposal budget |

GA-4 (serial-PPUT-primary) deferred — wants the Phase-2 reporter schema. GA-9 (carrier
non-thinking) deferred — Class-4 §8 on pinned `llm_http.rs`.

## 4. Dynamic router simulation results (no-LLM, deterministic, seed 0x48455432)
`tests/h2_economy_sim.rs` drives the REAL `routing_policy::score_and_select` over 3 equal-budget
arms (B_TARGET = 60,000 tok each). Fixture = H-HET-1 complementary coverage: deepseek uniquely
solves {det_zero, det_3x3}; qwen397 uniquely {det_2x2}; det_mul common; glm/qwen32 weak.

| arm | UNION coverage | total tokens | PPUT (tok/solve) |
|---|---|---|---|
| **UCB-MARKET (treatment)** | **4** {det_mul,det_2x2,det_zero,det_3x3} | **7,200** | **1,800** |
| round-robin (control-1) | 4 | 8,400 | 2,100 |
| best-single = deepseek (control-2) | **3** (misses det_2x2) | 15,600 | 5,200 |

UCB per-model budget share: deepseek 66.7%, qwen397 13.9%, glm 11.1%, qwen32 8.3%. ε-floor
activity in the main sim: **0** (the count-bonus explores efficiently enough that the deadline
floor never binds; a separate test `ucb_floor_fires_when_budget_tight` proves the real floor IS
reachable via `score_and_select`, returning `SelectionReason::Floor`).

**Independently re-verified** (re-ran the test; byte-identical) and **code-audited for rigging**:
non-vacuous claim assertion (asserts UCB≥both ONLY when it holds, else "CLAIM NOT MET" — no fixture
rigging), equal-budget invariant on all arms, real `score_and_select` (treatment) vs honest
baselines (`max_by_key` verify-rate for best-single; cycle for round-robin), anti-collapse (≥3
distinct models funded), deterministic (no RNG).

## 5. Allocation trace (UCB-MARKET arm, abridged)
```
r0  det_mul  -> deepseek  spent=1200  solved=true  [TieBreak]   (cold-start tie → tie-break)
r1  det_2x2  -> deepseek  spent=1200  solved=false [TieBreak]
r4  det_2x2  -> qwen397   spent=1000  solved=true  [UcbScore]   ← market re-routes off the failing
                                                                  model to the unique solver
r5  det_zero -> deepseek  spent=1200  solved=true  [TieBreak]
r6  det_3x3  -> deepseek  spent=1200  solved=true  [TieBreak]
```
The value-add moment is `r4`: after deepseek hard-fails det_2x2, the UCB score routes budget to
qwen397 (the unique solver), instead of best-single burning 10 wasted deepseek calls on det_2x2.

## 6. Counterfactual comparison
- vs **round-robin**: SAME union coverage (4=4) at **lower spend** (7,200 < 8,400) — the market
  stops paying a target once solved and avoids diluting onto non-verifying models.
- vs **best-single(deepseek)**: **higher** union coverage (4 > 3) at **much lower spend**
  (7,200 vs 15,600) — best-single structurally cannot cover det_2x2 and wastes budget retrying it.
- So the market converts complementary coverage into better coverage AND better token-PPUT at
  ≤ budget — the H-HET-2 economic thesis, demonstrated at smoke scale.

## 7. DAG reconstructed from tape
7 `BudgetAllocationTelemetry` records (UCB arm) round-trip through a real `CasStore`; a test-local
reconstruct walks them into `BudgetDecision → (model→target, solved) → OMEGA` edges, e.g.
`qwen397 → det_2x2 (solved, reason=UcbScore)`, `deepseek → det_zero (solved)`. The reconstructed
union-solved set {det_2x2,det_3x3,det_mul,det_zero} is asserted **byte-equal to the live arm
coverage** (`derive_from_tape(tape) == allocation_view`, Art 0.2), plus per-record
Σ candidate-pulls == header-total.

## 8. HONEST scope / what this is NOT
- **Smoke, not scientific evidence** (architect's framing). The outcome table is **synthetic**
  (no real model + real verifier — §17 G2 not satisfied; by design for no-LLM).
- **B_TARGET does not bind** (arms spend 7–15k of 60k) → this is a *spend-efficiency + coverage-vs-
  best-single* result, not a "fixed binding budget → more coverage" result.
- Small deterministic fixture (4 targets); a single seed. No paired-seed statistics (G4).

## 9. Blockers before live smoke / full paid run (smallest blocker → smallest repair)
1. **Lean axiom whitelist rejects `Classical.choice`** (from `het_probe_pool` real-Lean run:
   "non-whitelisted axioms: Classical.choice"). Live det-family verification would reject proofs
   using it. *Smallest repair:* confirm `Classical.choice` belongs in `allowed_axioms` (it is in the
   standard banked set {propext, Classical.choice, Quot.sound}) and add it, OR confirm the reference
   bodies must avoid it. **This blocks live solving.** (Investigate before the tiny live smoke.)
2. **GA-6b attempt-level decision_source** completeness — required before any paid confirmatory run.
3. **Phase-4 trust-root rehash** (Class-4 §8 on merged bytes) — `tc_boot_trust_root_manifest` stays
   red until done; required before ship/replay-trusted run.
4. **GA-9 carrier non-thinking** (`enable_thinking:false`, pinned `llm_http.rs`, Class-4 §8) —
   required for a *valid* hard-target run.
5. **Prereg freeze** (theorem pool / exclusion / budget cap / stopping rule / ≥N seeds) — before the
   confirmatory paid run.
6. **het_tape_reconstructibility** bootstrap (AgentManifestRequired) — bring het's Gate-D to main's
   admission model (register manifest + sign TaskOpen).

**Infra status for the tiny live smoke (step 5):** gateway `localhost:8123` reachable; Lean
toolchain present (`~/.elan`). The smoke is *infra-ready* but **gated on blocker #1** (axiom
whitelist) for any actual solve; the mechanism wiring (router → BudgetDecision tape → DAG) is
already proven in §4–§7 no-LLM. Recommend: resolve #1, then run one tiny live smoke (deep-chain
target, strict cap, dynamic router, replay clean) as **mechanism smoke only**.

## Branch commits (this increment)
```
f1e73f10  no-LLM VERIFY_UCB_PRICE_PRIOR_FLOOR_V1 simulation (smoke)
6362e5b6  GA-2/GA-3/GA-7/GA-8 + GA-6a boundary correction + matrix rows
95ab82bc  GA-5 + GA-6
f10742c1  GA-0
2665ffbd  converge(H-HET) re-apply (de-Lean, cargo-green)
```
