# TB-16.x.2.1 — TaskExpire env-var trigger smoke

**Date**: 2026-05-04
**Sub-atom**: TB-16.x.2.1 of umbrella `TB-16.x.2_charter_2026-05-04.md`
**Class**: 2 (env-var-gated arena hook in `evaluator.rs`; no economic semantics change)
**Profile**: `P9_force_expire` — `aime_1997_p9.lean` × `TURINGOS_FORCE_EXPIRE=1` × N=5 × MAX_TX=20

## Ship gate

**SG-16.x.2.1** = "TaskExpire tx kind in arena-produced tx kinds" — **✓ PASSED**.

| Signal | Value | Path |
|---|---|---|
| evaluator rc | 0 | `evaluator.stderr` |
| evaluator wall | 411s | (timing) |
| TaskExpire emit | `count=1 total_refunded_micro=200000 current_logical_t=3` | `expire_trace.txt` |
| accepted_tx_ids includes | `system-task-expire-1-4` | `runtime_repo/run_summary.json` |
| audit_tape verdict | `PROCEED` 34/0/0/7 | `verdict.json` |
| `tx_kind_counts.task_expire` | `1` | `verdict.json` |
| replay byte-identity | ✓ identical | `verdict.json` vs `verdict_replay.json` |
| tamper detection | **3/3** | `tamper_report.json` |
| dashboard L4 entry | `4 \| TaskExpire \| Agent_user_0` | `dashboard.txt` |

Architect tx-kind union: 9-of-13 → **10-of-13** runtime-exercised.

## Reproducer

```sh
bash handover/tests/scripts/run_tb_16_x_2_1_smoke_2026-05-04.sh
```

The runner uses a non-existent markov-pointer path (`/tmp/tb16x21_no_markov_pointer.txt`)
so Layer G (markov) assertions Skip — this matches R3 Round 2's recorded
behavior (`markov_constitution_hash_matches = "Skipped"`). The global
`handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt` currently points at a
Cid whose payload bytes live in TB-16 R3 Round 2's per-problem CAS, so a
fresh isolated CAS cannot resolve it. This is a **pre-existing infra gap
independent of sub-atom 2.1** (deferred to umbrella sub-atom 2.6).

To run the FORCE_EXPIRE + FORCE_BANKRUPTCY combined-path:

```sh
COMBINE_BANKRUPTCY=1 bash handover/tests/scripts/run_tb_16_x_2_1_smoke_2026-05-04.sh
```

## Forbidden honored

- (a) no f64 added — helper uses i64 micro-units throughout
- (b) L4 vs L4.E split honored — TaskExpire emits to L4 accepted (system-emitted)
- (c) no retroactive evidence rewrite — historical R3 Round 2 untouched
- (d) system-emitted via `emit_system_tx` (not agent-submitted)
- (e) no AMM/CPMM/price-as-truth introduction
- (f) no `prediction_market.rs` import
- (g) all 38+3 supplemental assertions retained
- (h) no `agent_id` outside `sandbox_prefix` — only `Agent_user_0` (preseed owner)

## TRACE_MATRIX

`FC2` — capital-flow expiry: `TaskExpireTx` releases escrow back to provider per
`tb11_emit_expire_for_eligible` policy (`ExpireReason::Deadline` solo;
`ExpireReason::BankruptcyTriggered` when chained with `FORCE_BANKRUPTCY`).
