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

## Markov capsule semantic (constitutional, not workaround)

The smoke runner passes `--markov-pointer /tmp/tb16x21_no_markov_pointer.txt`
(a non-existent path) to `audit_tape`. This is **not** a workaround for
infrastructure brokenness; it is the public API for "this chain has no
inherited Markov capsule" per `src/runtime/audit_assertions.rs:421-425`'s
`if pointer.exists() else None` branch.

**Why this is constitutionally correct, not "凑活":**

1. MarkovEvidenceCapsule is NOT a flowchart node (FC1/FC2/FC3 contain no
   Markov node). Per CR-15.5 it is a 派生视图 — evidence compression, not
   source of truth. (`constitution.md:455-509` FC1, `571-660` FC2,
   `826-870` FC3; `src/runtime/markov_capsule.rs:1-50` CR-15.5)
2. Markov chain genesis is `previous_capsule_cid: None`
   (`src/runtime/markov_capsule.rs:60+111`). A fresh isolated chain has no
   inherited Markov by definition.
3. TB-16.x.2.1's smoke run is fresh `runtime_repo` + fresh `cas`, with
   no `previous_capsule_cid` claim in its bytes — i.e. it is
   constitutionally a **genesis chain** per FC2 Boot semantic.
4. Therefore `markov_capsule = None` is the unique correct state, and
   the 7 Layer G `Skipped` assertions are CORRECT, not bypassed.

The deeper Art. 0.2 violation surfaced — `handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt`
as a global parallel-ledger sidecar — is **out of sub-atom 2.1 scope** and
filed as `handover/alignment/OBS_R022_GLOBAL_LATEST_MARKOV_PARALLEL_LEDGER_2026-05-04.md`
for architect ratification. Sub-atom 2.x continues with absent-pointer
semantics (constitutionally correct) until ruling lands.

To run the FORCE_EXPIRE + FORCE_BANKRUPTCY combined-path:

```sh
COMBINE_BANKRUPTCY=1 bash handover/tests/scripts/run_tb_16_x_2_1_smoke_2026-05-04.sh
```

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
