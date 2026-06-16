# H2_LIVE_MECHANISM_SMOKE_REPORT — 2026-06-16 (rev. post adversarial QC)

**Status: LIVE SMOKE PASS as a MECHANISM-EXISTENCE WITNESS — claims rescoped after a 6-auditor
adversarial QC (verdict QC-CONCERNS, no VIOLATION, no blocker).** The tiny live router smoke ran
end-to-end on real LLMs + real Lean, reached omega, and replays byte-clean from a canonical tape.
This is a **SMOKE**, NOT confirmatory evidence. Branch `claude/het-converge-2026-06-16` @ `205fb5d9`.

## Three core claims — independently re-verified at the byte level (HOLD)
1. **omega is a real Lean 4.24.0 verification.** node1 CAS VerifierResult decodes to `exit_code=0,
   verified=true, error_class=None, verdict_kind=Verified`; `rw [Nat.add_comm]` on `calib_core_add_comm`.
2. **`model_id` is genuinely on the canonical tape.** 2 ProposalTelemetry.v2 + 2
   BudgetAllocationTelemetry.v1 objects decode byte-exact (model roster + winners).
3. **Replay is byte-clean.** `replay.json` all-true, `replay_failure=null`, 14 L4 entries.

## What unblocked it (Phase-4, Class-4, architect-authorized)
§6-Step-4 protocol: 20 mismatched `[trust_root]` pins recomputed on final merged bytes, Class-4
manifest delta (`TRUSTROOT_REHASH_CLASS4_DELTA_2026-06-16.md`), **Veto-AI PASS**, `constitution.md`
untouched → `constitution_tc_boot_trust_root_manifest` **8/8 GREEN**. Commit `205fb5d9`.

## Live run (run_id `smoke_ucb_001`, policy `verify_ucb_price_floor`)
omega_reached=true, omega_node `…node1…`, ttfp 11.94s. Target `calib_core_add_comm`, axioms ⊆ banked
whitelist. 2 real proposal LLM calls, 1 Verified / 1 Failed, 362 total model tokens. Lean v4.24.0.

## Mechanism witness — heterogeneous-model routing on a replay-green tape (TIE-BREAK leg, NOT value-driven)
Decoded from the canonical tape (BudgetAllocationTelemetry.v1 + ProposalTelemetry.v2):

| node | agent | router-REQUESTED `model_id` (on tape) | `selection_reason` (on tape) | body | verdict |
|------|-------|---------------------------------------|------------------------------|------|---------|
| node0 | Agent_0 | `Qwen/Qwen3-32B` | **TieBreak** (4 arms tied @42500) | `induction b with …` | Failed |
| node1 | Agent_1 | `Qwen/Qwen3.5-397B-A17B` | **TieBreak** (3 arms tied @43525) | `rw [Nat.add_comm]` | **Verified (omega)** |

**What this smoke licenses (precisely):** `VerifyUcbPriceFloor` routed 2 proposals to 2 distinct
heterogeneous models and recorded those allocation decisions on a canonical, replay-green tape.
**What it does NOT license:** for this 2-tick omega-halted run, **both selections were the policy's
deterministic lexicographic TIE-BREAK leg** (`selection_reason=TieBreak` on tape; all candidate
scores tied because no verify/price signal had accumulated in 2 of 16 budgeted ticks). The UCB +
price-floor + ε-floor machinery is implemented, exercised, and tape-recorded (a live `price_bps=7500`
cold-start prior rode on the failed Qwen3-32B), but it was **non-decisive** here — the winner was a
tie-break, not a value differentiation. Substantiating "value-driven" needs a longer/non-halting run
where `selection_reason=UcbScore/ColdStart` appears on tape.

**Provenance caveat:** the on-tape `model_id` is the router-**REQUESTED** label (`tick_model`), NOT a
served-model confirmation. The proxy echoes the client's requested label and discards the upstream
provider's `resp.model`; the Rust driver parses but never consumes `response.model`; all 4 roster
labels route to one provider. So heterogeneity is proven at the **routing-decision** layer, not the
**execution** layer.

**Cost caveat:** per-proposal cost = rate(model_id)×tokens is recomputable **given the tape AND the
pinned `MODEL_RATES` compile-time constant** (`src/market_tape_shared.rs:146-159`) — the rate table is
NOT itself on the tape / not hash-pinned. Tape carries the inputs (model_id + tokens); the rate is a
source-code side-input. Art-0.2 gap on the cost axis only (the routing decision itself IS tape-canonical
via the SHA-pinned `RoutingPolicyConfig`).

## Replay / Tape / DAG status — all green
`verify_chaintape` → 14 L4, `replay_failure: null`, ALL reconstruction flags true. Registered gates:
`constitution_budget_decision_tape_canonical` **5/5**, `constitution_h2_dag_reconstructible` **4/4**
(both genuinely failable but **fixture-local** — neither fires on the live smoke tape; see caveats).

## Engineering findings (this increment)
- Axiom blocker FIXED: `DEFAULT_ALLOWED_AXIOMS` aligned to banked `{propext, Classical.choice,
  Quot.sound}`; non-banked axioms still fail-closed (audit dim-B PASS; `native_decide` double-gated
  by source-scan + whitelist). `het_probe_pool` (64s real Lean) + `het_third_bug` RED→GREEN; 9/9 regression.
- GA-6b landed: `constitution_h2_attempt_decision_source` 6/6 (failable within its declared
  BIN-only/logic-witness scope).
- cargo check --all-targets 0 errors; manifest+matrix drift 0; 205 gates.

## Honest caveats (do NOT over-read this smoke)
- **SMOKE, not evidence:** n=2 proposals, single seed (42), one easy omega target, no paired stats.
- **Routing was tie-break, not value-driven** at this run length (see above) — no PROVEN/value-driven headline.
- **model_id = requested label, not served model** (no upstream served-model witness).
- **Cost needs the external MODEL_RATES table** (not on tape).
- **Run-level budget CONSERVATION is NOT closed:** `allocated_token_budget` hardcoded 900 vs balance −1
  (`lean_market_agent.rs:2139-2142`); the conservation gate passes only on a synthetic fixture; replay
  (`verify.rs`) reconstructs ProposalTelemetry but NOT BudgetAllocationTelemetry. This smoke witnesses
  dynamic model-budget **routing**, not run-level **conservation**.
- **Binary freshness unverifiable:** manifest records no binary-hash / no source_commit; the on-disk
  binary postdates the run. Circumstantial case strong (only a docs-only commit separates 205fb5d9 from
  HEAD; `lean_market_agent.rs` unchanged in 205fb5d9) but not artifact-bound.
- `decision_source/action_source` are BIN-only and null (route_llm_calls=0) → tape-canonical promotion is Class-4.

## Required fixes BEFORE the paid confirmatory run (audit-derived, tracked)
1. **served_model provenance** — proxy returns upstream `resp.model` (or a `served_model` field); driver
   records it on ProposalTelemetry + assert/flag served≠requested + regression test. (dim C)
2. **MODEL_RATES → CAS at genesis** — write the rate table to CAS, CID in GenesisPin, so cost is
   tape-recomputable without a source side-input. (dim D)
3. **Run-path budget conservation** — fix `allocated_token_budget` (hardcoded 900) vs balance accounting
   so `constitution_h2_budget_conservation` fires on real-run records, not just a fixture. (dim F)
4. **BudgetAllocationTelemetry replay reconstruction** — `verify.rs` reconstructs allocation from tape so
   `replay.json` witnesses allocation == derive_from_tape. (dim E/F)
5. **Binary-hash + HEAD in manifest** — emit `sha256(binary)` + git HEAD at run-start for source binding. (dim E)
- Plus (non-blocking bookkeeping, dim B): populate the `axioms` field from `parse_axiom_set` on Verified
  (currently emits `[]`), and remove/route the dead `axiom_gate()` method.
- Plus `decision_source/action_source` tape-canonical promotion (Class-4) + GA-9 (`enable_thinking:false`,
  Class-4 §8) + prereg freeze.

## Next allowed action (autonomous loop continues — adjudicator: CONTINUE_STEP6)
Step 6 deep-chain target-pool calibration (chain ≥10/≥18, tx ≥ agents×20, axiom-clean) → Step 7 prereg
freeze → Step 8 confirmatory pilot. No paid hard-target run until the 5 fixes + GA-9 + prereg resolve (§11).
