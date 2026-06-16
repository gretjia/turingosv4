# H2_LIVE_MECHANISM_SMOKE_REPORT — 2026-06-16

**Status: LIVE SMOKE PASS (mechanism witness).** The tiny live dynamic-model-budget-router
smoke ran end-to-end on real LLMs + real Lean, reached omega, and replays byte-clean from a
canonical tape that carries per-proposal `model_id`. This is a **SMOKE** (single seed, easy
target, 2 proposals) — a mechanism existence witness, **NOT** confirmatory evidence.
Branch `claude/het-converge-2026-06-16` @ `205fb5d9` (Phase-4 trust-root rehash applied).

## What unblocked it (Phase-4, Class-4, architect-authorized)
The boot trust-root abort was resolved via the §6-Step-4 protocol: 20 mismatched `[trust_root]`
pins recomputed on the **final merged bytes** (de-Lean §8 renames + standing-authorized H-HET-2
modules), Class-4 manifest delta emitted (`TRUSTROOT_REHASH_CLASS4_DELTA_2026-06-16.md`),
**Veto-AI returned PASS**, `constitution.md` pin untouched. Result: `constitution_tc_boot_trust_root_manifest`
**8/8 GREEN**; carrier boots; smoke ran. Commit `205fb5d9`.

## Live run (run_id `smoke_ucb_001`, policy `verify_ucb_price_floor`)
- **omega_reached = true**, omega_node `worktx-lm-node1-smoke_ucb_001-lm`, ttfp 11.94s, wall 11.97s.
- target `calib_core_add_comm` (`rw [Nat.add_comm]`), needs_mathlib=false, axioms ⊆ banked whitelist.
- 2 real proposal LLM calls, 1 Verified / 1 Failed, 362 total model tokens (golden-path 152).
- Lean v4.24.0 (pinned), gateway localhost:8123.

## Mechanism witness — the dynamic router distributed across heterogeneous models
Decoded directly from the **canonical tape** (CAS, via the real `read_from_cas` ProposalTelemetry v2
decoder — not the BIN manifest):

| node | agent | router-selected `model_id` (on tape) | body | verdict |
|------|-------|--------------------------------------|------|---------|
| node0 | Agent_0 | `Qwen/Qwen3-32B` | `induction b with …` | Failed (lean-reject) |
| node1 | Agent_1 | `Qwen/Qwen3.5-397B-A17B` | `rw [Nat.add_comm]` | **Verified (omega)** |

Two agents were routed to **different** models in one run; the larger model produced the winning
proof. Because `model_id` + `token_counts` are both on the tape, per-proposal cost = rate(model)×tokens
is **recomputable from the tape** — the precondition for any future token-economics headline.

## Replay / Tape / DAG status — all green
`verify_chaintape --repo /tmp/smoke_ucb_repo --cas /tmp/smoke_ucb_cas --run-id smoke_ucb_001`
→ 14 L4 entries, `replay_failure: null`, and ALL true:
`ledger_root_verified`, `system_signatures_verified`, `agent_signatures_verified`,
`state_reconstructed`, `economic_state_reconstructed`, `cas_payloads_retrievable`,
`proposal_telemetry_cas_retrievable`. Final state root `fb2bbc0b…`, ledger root `575f7d5a…`.
Registered gates on the tape-canonical / DAG path: `constitution_budget_decision_tape_canonical` **5/5**,
`constitution_h2_dag_reconstructible` **4/4**.

## Engineering findings (this increment)
- **Axiom blocker FIXED** (Step 2): `DEFAULT_ALLOWED_AXIOMS` aligned to banked
  `{propext, Classical.choice, Quot.sound}` → `het_probe_pool` (64s real Lean) + `het_third_bug` (23s)
  RED→GREEN; non-banked axioms still fail-closed (regression `lean_judge_axiom_gate` 9/9).
- **GA-6b landed** (Step 3): `constitution_h2_attempt_decision_source` 6/6 (2 failable).
- **`model_id` on tape** confirmed populated & non-null for this run (decoded above) — H-HET-2 hard-gate #1 met for the smoke.
- cargo check --all-targets 0 errors; manifest+matrix drift 0; 205 gates.

## Honest caveats (do NOT over-read this smoke)
- **SMOKE, not evidence:** n=2 proposals, single seed (42), one easy omega target, no paired stats.
  Coverage/economics claims require the confirmatory pilot (≥12 seeds, deep-chain pool, paired Wilcoxon).
- **`decision_source` / `action_source` are BIN-only and `null` here** (route_llm_calls=0 → the
  UCB routing decision is a deterministic white-box computation, not an LLM call, so no LLM-routing
  decision_source was recorded). Per GA-6b they remain a non-tape field → **tape-canonical promotion
  (ProposalTelemetry/AttemptTelemetry schema v-next) is Class-4 REQUIRED before the paid confirmatory run.**
- **GA-9** (`enable_thinking:false` on pinned `src/drivers/llm_http.rs`, Class-4 §8) is **not** needed
  for this easy-target smoke; it **is** required for a valid hard-target paid run.
- **Prereg freeze** (target pool / policy hash / budget cap / exclusions / ≥12 seeds) still pending.

## Next allowed action (autonomous loop continues)
Step 6 deep-chain target-pool calibration (chain ≥10/≥18, tx ≥ agents×20, axiom-clean) →
Step 7 prereg freeze → Step 8 confirmatory pilot. No paid hard-target confirmatory run until
GA-6b(tape) + GA-9 + prereg are resolved (§11).
