# TB-16.x.2.3 smoke evidence — P11_complete_set_redeem

**Date**: 2026-05-05  
**Charter**: `handover/tracer_bullets/TB-16.x.2_charter_2026-05-04.md` §2 Atom 2.3  
**Risk class**: 2 (env-var-gated arena hook in evaluator.rs + ADDITIVE adapter helper; no sequencer/scheduler change; no economic-semantics mutation; no auth/crypto surface beyond AGENT-signed tx already wired in TB-13)  
**Iteration cap**: 24h (capability signal = real-LLM smoke producing tx_kind_counts.complete_set_redeem ≥ 1)

## Goal

Close the missing 6th system-emitted tx kind in the TB-16 R3 Round 2 chain
union: **CompleteSetRedeem** (raises 11-of-13 → **12-of-13** architect tx
kinds runtime-exercised). Per architect umbrella charter §2 Atom 2.3.

## Run

```
bash handover/tests/scripts/run_tb_16_x_2_3_smoke_2026-05-05.sh
```

Probe env vars:
- `TURINGOS_COMPLETE_SET_SEED=Agent_user_0:1000000` — provider mints 250k YES + 250k NO shares
- `TURINGOS_FORCE_BANKRUPTCY=1` — at MaxTxExhausted, transitions task_markets_t state to Bankrupt (NO wins)
- `TURINGOS_FORCE_REDEEM=Agent_user_0:no:250000` — provider redeems 250k NO shares 1:1 vs collateral

Problem: `aime_1997_p9.lean` (chosen because expected to MaxTxExhaust under
N_SWARM=5 + MAX_TX=20 with `deepseek-chat` thinking-off; same problem as
TB-16.x.2.1 P9_force_expire smoke — proven exhaustion path).

## Chain shape

```
L4 logical_t=1 — TaskOpen (preseed; sponsor=Agent_user_0)
L4 logical_t=2 — EscrowLock (preseed; bounty=200_000 μC)
L4 logical_t=3 — MarketSeed (Agent_user_0; collateral=1_000_000 μC)
L4 logical_t=4 — CompleteSetMint (Agent_user_0; 250_000 YES + 250_000 NO)
L4 logical_t=5 — TerminalSummary (MaxTxExhausted; capsule_id=...)
L4 logical_t=6 — TaskBankruptcy (system-emitted; reason=MaxFailedRunCount)
L4 logical_t=7 — CompleteSetRedeem (Agent_user_0; outcome=No; units=250_000)
```

stderr trace evidence (redeem_trace.txt):
```
[chaintape/tb16-arena] MarketSeedTx submitted by Agent_user_0 (1000000 μC) for event=task-n5_aime_1997_p9_1777963470439
[chaintape/tb16-arena] CompleteSetMintTx submitted by Agent_user_0 (250000 μC YES + 250000 μC NO) for event=task-n5_aime_1997_p9_1777963470439
[chaintape/tb16-arena] TaskBankruptcyTx emitted: emit_id=2 task_id=TaskId("task-n5_aime_1997_p9_1777963470439")
[chaintape/tb16-arena] CompleteSetRedeemTx submitted by Agent_user_0 (units=250000, outcome=No) for event=task-n5_aime_1997_p9_1777963470439
```

## Audit verdict (verdict.json)

```
verdict          = PROCEED
passed           = 34
failed           = 0
halted           = 0
skipped          = 8 (single-problem smoke; multi-problem assertions inherit prior coverage)
tape_root.l4_count            = 7
tape_root.l4e_count           = 2
tape_root.cas_object_count    = 15
tape_root.constitution_hash   = eec695459c71fbef...
tx_kind_counts.market_seed         = 1
tx_kind_counts.complete_set_mint   = 1
tx_kind_counts.task_bankruptcy     = 1
tx_kind_counts.complete_set_redeem = 1   ← TB-16.x.2.3 GREEN
```

| Assertion | Result | Note |
|---|---|---|
| id=22 (Layer D) `conditional_shares_excluded_from_supply` | **Pass** | Pre-existing assertion covers SG-16.x.2.3 per umbrella charter §2 Atom 2.3 ("pre-existing id=22 covers; no new assertion needed") |

`feature_coverage`: TB-1/3/11/13/14/16 GREEN. TB-2/4/5/7/8/9/10/12/15 single-problem-smoke RED (will green in .2.6 combined run; not a regression).

## Ship gates

| SG | Verification | Result |
|---|---|---|
| SG-16.x.2.3 — chain contains CompleteSetRedeemTx with non-zero share_amount | verdict.json `tx_kind_counts.complete_set_redeem = 1` AND stderr submit-trace `units=250000` | ✓ |
| SG (replay determinism — Layer C #16) | `cmp -s verdict.json verdict_replay.json` | ✓ |
| SG (tamper detection 3/3) | tamper_report.json: flip_l4 + flip_cas + remove_l4 all detected | ✓ |
| SG (smoke script fail-closed exit) | python3 JSON guard + trace witness; exits 1 on either count=0 OR positive=0 | ✓ |

## Surfaces shipped

- `experiments/minif2f_v4/src/bin/evaluator.rs` — FORCE_REDEEM env-var hook BEFORE bundle.shutdown (parallel to FORCE_CHALLENGE_RESOLVE; OUTSIDE the MaxTxExhausted EvidenceCapsule conditional so it works on both OMEGA and MaxTxExhausted exit paths). 3-part env var format `<owner>:<outcome>:<share_units>`; event_id auto-derived from `task-{run_id}`.
- `src/runtime/adapter.rs` — ADDITIVE `make_real_complete_set_redeem_signed_by` helper (mirrors `make_real_complete_set_mint_signed_by` shape).
- `genesis_payload.toml` — R-014 rehash:
  - `experiments/minif2f_v4/src/bin/evaluator.rs`: `12489ab4...` → `e1c4d057...`
  - `src/runtime/adapter.rs`: `c1360a73...` → `48da399a...`
- `handover/tests/scripts/run_tb_16_x_2_3_smoke_2026-05-05.sh` — NEW, py3 JSON ship-gate (counts ≥ 1) + py3 stderr trace witness (units > 0). Fail-closed exit per .2.2.fix.r2 Patch F1+F2.

## Deviations from charter (per `feedback_architect_deviation_stance`)

1. **3 env-var parts vs. charter-spec'd 4**: charter §2 Atom 2.3 specified `<owner>:<event_id>:<outcome>:<share_amount>`. Implementation uses 3 (`<owner>:<outcome>:<share_units>`); event_id auto-derived from `task-{run_id}` because `run_id` contains a unix-ms timestamp minted at evaluator entry (run_id.rs:21) and is unpredictable from the smoke script. Mirrors FORCE_BANKRUPTCY's auto-derive pattern (line ~3154). Deviation documented in genesis_payload.toml R-014 annotation chain.

## Forensic carry-over from hygiene #15 (.2.1 evidence re-verify, this session)

The .2.1 smoke ship-gate (`grep '"task_expire"'` on verdict.json) had the same field-name false-positive bug that .2.2.fix Patch B fixed. **Substantive .2.1 smoke is real** (binary contained FORCE_EXPIRE code; stderr `TaskExpire batch: count=1`; verdict.json `task_expire=1`); only the gate logic was broken. Per `feedback_no_retroactive_evidence_rewrite`, historical evidence stays untouched. Forward fix: this `.2.3` script + all subsequent (`.2.5`, `.2.6`) use the python3 JSON count guard pattern + secondary witness (trace log).

## Next

- TB-16.x.2.5 — AutopsyCapsule real-bankruptcy chain (Class 2; same evaluator.rs file; AFTER_ACCEPTED env-var injects real-signed WorkTx so stakes_t populated → autopsy generated on FORCE_BANKRUPTCY).
- TB-16.x.2.4 — Multi-WorkTx + Boltzmann RUNTIME (Class 3; STEP_B_PROTOCOL on sequencer.rs; mandatory dual external audit).
- TB-16.x.2.6 — Combined arena run (Class 2; single chain covers 13-of-13 tx kinds).

## Local-only forensic artifacts (NOT in git history)

- `cas/` (15 CAS objects)
- `runtime_repo/` (7 L4 + 2 L4.E ledger entries)
- `tamper/` (3 corruption test copies)

These are NOT committed (matching prior tb_16_chaintape_smoke_* convention) — the verdict.json + tamper_report.json + dashboard.txt + trace files are the canonical evidence; `cas/` and `runtime_repo/` are reproducible by re-running the smoke script.
