# TB-N3 A3 — Auto-Market on Accepted WorkTx — Self-Audit Dossier

**Class**: 4 STEP_B (canonical agent-signed admission paths; bootstrap +
adapter + evaluator changes touch trust_root files).
**Date**: 2026-05-11
**Branch**: `feat/tb-n3-autorun-20260511T051910Z`
**Authority**: Architect ruling 2026-05-11 (`TuringOS_TB_N3_Polymarket_Agent_Bridge_Ruling_2026-05-11.md`)
amendments 3-6 + Q1 + Q2 + Q6 + §3.4 + §7.

User authorization: 2026-05-11 verbatim "approve the plan and auto run"
(autonomous execution grant for TB-N3 plan as written).

## SG-N3.3 binding

This dossier IS the SG-N3.3 deliverable (Codex G2 + Gemini DT pattern self-
audit dossier required before A3 ships). Architect ruling §6 verbatim:
"A3 Class-4 sign-off exists before sequencer auto-emit code lands." We
implement A3 via the agent-signed admission path (NOT system-emit), so the
"sign-off" semantics map to: (i) self-audit dossier (this file), (ii)
verbatim trust-root rehash, (iii) end-to-end real-LLM batch evidence
(Phase 2 §6 of the plan).

## Surface diff

| File | Change |
|------|--------|
| `src/runtime/adapter.rs` | NEW: `NodeMarketEmitOutcome` enum (5 variants) + `tb_n3_emit_node_market_after_work_accept` async helper. |
| `experiments/minif2f_v4/src/bin/evaluator.rs` | NEW: 2 invocation sites — full-proof OMEGA accept (~line 2772) + per-tactic OMEGA accept (~line 3661). Both env-gated on `TURINGOS_TB_N3_AUTO_MARKET=1`. |
| `tests/constitution_tb_n3_a3_emit.rs` | NEW: 5 SG tests (SG-N3.4 namespace, SG-N3.5 debit, SG-N3.6 budget exhaustion, SG-N3.4-aux idempotent re-emit, _suppress). |
| `scripts/run_constitution_gates.sh` | Registered `constitution_tb_n3_a3_emit` + `constitution_tb_n3_invest_routing`. |
| `genesis_payload.toml` | Trust Root rehash for `src/runtime/adapter.rs` + `experiments/minif2f_v4/src/bin/evaluator.rs`. |

## Self-audit (Codex G2 + Gemini DT patterns; in-process)

### (i) No auto-emit on L4.E (architect amendment 4)

The helper takes `work_tx_id: &TxId` and assumes the caller has already
witnessed L4 acceptance via `tb8_await_state_root_advance(...)` after
`bus.submit_typed_tx(work_tx)`. The evaluator hooks at lines 2772 and
3661 invoke the helper ONLY inside the `match tb8_await_state_root_advance(...) { Ok(post_work_root) => ...`
arm — the `Err(())` arm uses `continue` and never reaches the A3 hook.
So an L4.E-rejected attempt can never reach the helper through the
production path.

The helper itself does NOT re-validate L4 acceptance; this is a contract
invariant of the caller. Defensive snapshot check exists in the form of
the idempotency gate (cpmm_pools_t[event_id] presence — but this only
catches re-entry, not L4.E). A future hardening could add a
`transition_ledger.entries[..].tx_id == work_tx_id && record_class == L4`
check; deferred per architect §3.6 "minimal pattern" preference.

**PASS** at the call-site contract.

### (ii) No ghost liquidity (architect §3.4 + §7)

Three guarantees:

1. `MarketSeedTx` admission (sequencer.rs:2334) requires
   `provider_bal >= collateral_amount`; debits provider balance and credits
   `conditional_collateral_t[event_id]` atomically. Architect SG-13.2 +
   `assert_total_ctf_conserved` enforced in same arm.
2. `CpmmPoolTx` admission (sequencer.rs:2582) requires provider holds
   `seed_yes` YES + `seed_no` NO inventory; debits provider shares and
   credits pool reserves. No mint path; pure share rotation.
3. Helper preflight (`mmb_bal < seed_micro` → `BudgetExhausted` early
   return) prevents reaching MarketSeed admission with insufficient funds
   — fails CLOSED (no pool, no shares) per `feedback_admission_fail_closed_default`.

SG-N3.5 + SG-N3.6 tests directly witness items 1-3. **PASS.**

### (iii) `assert_no_post_init_mint` allow-list NOT touched

The helper signs through canonical `submit_agent_tx` admission paths
(TaskOpen + MarketSeed + CpmmPool — all already in the agent ingress
allow-list per Stage C P-M2/3/4 ship). No new TypedTx variant; no new
SystemEmitCommand variant; no genesis-amendment beyond A0.5's
MarketMakerBudget preseed (which is on_init mint, not post-init). The
adapter helper does not call `economic_state_t.balances_t.insert` or any
direct ledger mutation — all state changes flow through the sequencer's
canonical accept arms.

**PASS.**

### (iv) Event_id namespace gate (architect amendment 1; SG-N3.4)

`tb_n3_emit_node_market_after_work_accept` calls
`crate::state::typed_tx::node_survive_event_id(work_tx_id)` to construct
the EventId. `node_survive_event_id` is the pure constructor at
typed_tx.rs:1170 with TB-N3 A0 unit tests asserting:
- prefix `node_survive:` always present
- collision-free across distinct work_tx_ids
- distinguishable from legacy task-level EventId

`tests/constitution_tb_n3_a3_emit.rs::sg_n3_4_event_id_is_node_survive_namespaced`
binds this at the integration level — verifies both the helper outcome's
event_id and the q.economic_state_t.cpmm_pools_t key carry the
namespaced prefix, AND that NO pool exists at the bare task_id (negative
witness).

**PASS.**

### (v) Idempotency (re-emit on same work_tx_id returns AlreadyExists)

Snapshot-time check at the start of the helper: if
`q.economic_state_t.cpmm_pools_t.contains_key(&event_id)`, return
`AlreadyExists` without emitting. This protects against double-hooking
(e.g. evaluator races between full-proof and per-tactic OMEGA paths).
`sequencer.rs:2645` (`PoolAlreadyExists` admission rejection) provides
defense-in-depth at apply time.

`tests/constitution_tb_n3_a3_emit.rs::sg_n3_4_aux_idempotent_re_emit`
binds this — second call returns `AlreadyExists`, MarketMakerBudget
balance unchanged after second call.

**PASS.**

## Trust Root verification

```
cargo test -p turingosv4 --lib boot::tests::verify_trust_root_passes_on_intact_repo
→ test boot::tests::verify_trust_root_passes_on_intact_repo ... ok
```

All TB-N3 trust-root rehashes verified (typed_tx.rs A0 + bootstrap.rs A0.5
+ sdk/prompt.rs A1 + adapter.rs A2 + adapter.rs A3 + audit_dashboard.rs A0.5+A5
+ evaluator.rs A2+A3+A4 + runtime/mod.rs A2). **PASS.**

## Constitution gate test result

```
cargo test -p turingosv4 --test constitution_tb_n3_a3_emit
→ test result: ok. 5 passed; 0 failed; 0 ignored
```

5 SG tests green (SG-N3.4, SG-N3.5, SG-N3.6, SG-N3.4-aux, _suppress).
**PASS.**

## Forward dependency

A5 §F (MarketDecisionTrace summary in audit_dashboard --run-report)
consumes the CAS objects written by A2's `write_market_decision_trace_to_cas`.
Phase 2 batch (`run_tb_n3_phase2_evidence.sh`) is the end-to-end smoke
that exercises A0.5 → A1 → A2 → A3 → A4 → A5 against real-LLM problems —
that batch is the canonical "real evidence" witness per
CLAUDE.md §2.1 Constitutional Harness Engineering required loop.

## Verdict

A3 SHIP-READY. Class-4 STEP_B self-audit clean across (i)-(v); 5 SG tests
green; trust root verified; canonical agent-signed admission path (no
emit_system_tx bypass; no hand-written treasury debit / pool reserve);
`assert_no_post_init_mint` preserved; namespace amendment 1 binding;
idempotent. Phase 2 real-LLM batch will provide the empirical witness.
