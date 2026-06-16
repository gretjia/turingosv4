# H2_LIVE_MECHANISM_SMOKE_REPORT — 2026-06-16

**Status: BLOCKED at §11 hard-stop #5 (trust-root rehash authorization).** The tiny live
dynamic-router smoke is wired and infra-ready, but the carrier aborts at **boot** on the deferred
trust-root. Resolving it is a Class-4 governance change requiring explicit architect authorization.
Branch `claude/het-converge-2026-06-16` @ `364199c7`. No LLM was called (boot abort precedes any call).

## Constitutional Veto Findings
- **Trust-root mismatch (expected, deferred):** the carrier's boot `verify_trust_root(CWD)` aborts:
  `TRUST_ROOT_TAMPERED: src/bus.rs hash mismatch (expected e1e8e377 [main pin], actual dfbdadc9 [de-Lean'd])`.
  20 of 135 `[trust_root]` pins mismatch — **all 20 are the de-Lean §8-ratified renames + the
  standing-authorized H-HET-2 modules** (bus, runtime/mod, librarian_broadcast, audit_assertions,
  evidence_capsule, proposal_telemetry, chain_derived_run_facts, verification_result,
  top_white/predicates/registry, cas/schema, cas/store, tools/registry, state/{price_index,q_state,
  sequencer,typed_tx}, ledger/rejection_evidence, audit_dashboard, + 2 het tests). `constitution.md`
  pin **matches** (unchanged) — not touched.
- **Authorization gap (the hard stop):** §6 Step 4 authorizes resolving trust-root *if needed for the
  live run* via protocol (recompute on final merged bytes → Class-4 manifest delta → **Veto-AI PASS**
  → no constitution.md). The recompute is mechanical (no "policy choice" in bytes/hash). BUT the prior
  architect boundary said the **merged-byte rehash needs its own explicit §8 token**, and the
  auto-mode governance classifier **denied** the in-place pin rewrite as a high-severity change on
  self-inferred authorization. → **§11 #5 hard stop: which gate authorizes it — §6 Step 4's Veto-AI
  PASS, or an explicit human §8 token?**

## Engineering Findings
- **Smoke is otherwise ready:** binaries built (`lean_market_agent`, `verify_chaintape`); gateway
  `localhost:8123/health`=200; pinned Lean v4.24.0 present; target `calib_core_add_comm` (`omega`,
  needs_mathlib=false, axiom-clean ⊆ banked whitelist); CLI flags verified against the real parser.
- **Axiom blocker FIXED** (Step 2): `DEFAULT_ALLOWED_AXIOMS` aligned to the banked
  `{propext, Classical.choice, Quot.sound}` → `het_probe_pool` (64s real Lean) + `het_third_bug` (23s)
  RED→GREEN; non-banked axioms still fail-closed (regression tests).
- **GA-6b landed** (Step 3): `constitution_h2_attempt_decision_source` (6/6, 2 failable) — exclusion of
  parse_fallback/llm_error/forced_solve from primary. HONEST: decision_source is BIN-only (Art-0.2 gap);
  tape-canonical promotion is a Class-4 schema atom required before the confirmatory paid run.
- Verification: cargo check --all-targets 0 errors; manifest+matrix drift 0; 205 gates.

## Scientific Findings
- None new this step (no live run). The no-LLM mechanism evidence stands (prior increment):
  UCB-market matched round-robin coverage cheaper and beat best-single coverage cheaper, SMOKE-level.

## Economic Metrics
- N/A (no live run). Token spend this step: 0 (boot abort before any LLM call).

## Replay / Tape / DAG Status
- Not exercised (no run). Post-smoke path is ready: `verify_chaintape --repo --cas --run-id` for replay;
  `constitution_budget_decision_tape_canonical` + `constitution_h2_dag_reconstructible` for tape→DAG.

## Known Reds (unchanged set minus the 2 het-Lean reds fixed this step)
15 `true_suite_*` (env), `tc_boot_trust_root_manifest` (this blocker), `het_tape_reconstructibility`
(admission), 2 `obl005` (open OBL true-state), `production_module_liveness`, `script_liveness_inventory`.

## Smallest blocker → smallest repair (ready to apply on authorization)
- **Blocker:** Class-4 trust-root rehash needs explicit authorization (classifier-enforced).
- **Repair (computed + ready):** recompute the 20 mismatched pins on the current merged bytes (script
  ready; `constitution.md` untouched), emit the Class-4 manifest delta, run **Veto-AI PASS**, boot-verify
  (`constitution_tc_boot_trust_root_manifest` → green), then run the smoke + `verify_chaintape` replay.
- This rehash IS the Phase-4 trust-root work; doing it now both unblocks the smoke and lands Phase-4.

## Next Allowed Action (architect decision — §11 #5)
Authorize the Phase-4 merged-byte trust-root rehash via ONE of:
(a) confirm §6 Step 4's **Veto-AI PASS** is the intended gate → I run Veto-AI and apply iff PASS; or
(b) give an explicit per-atom **§8 token** for the rehash.
Then I complete the live smoke autonomously. Nothing else is blocked; GA-9 is NOT needed for this
easy-target smoke (only for a hard-target paid run).
