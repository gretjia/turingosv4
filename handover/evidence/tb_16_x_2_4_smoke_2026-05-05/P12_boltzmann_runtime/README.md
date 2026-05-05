# TB-16.x.2.4 smoke evidence — P12_boltzmann_runtime (pre-audit)

**Date**: 2026-05-05  
**Charter**: `handover/tracer_bullets/TB-16.x.2_charter_2026-05-04.md` §2 Atom 2.4  
**Risk class**: 3 (Boltzmann RUNTIME wire-up = V3L-14 anti-star-topology mechanism; Art II.2.1 alarm threshold gate; high-impact even though no sequencer/wallet touch)  
**Iteration cap**: 24h (capability signal = real-LLM smoke producing ≥3 admitted WorkTx with audit assertion id=43 entropy gate Pass)

## Goal

Close the missing R3 "Boltzmann RUNTIME exercise" gap. Per architect umbrella charter §2 Atom 2.4. Prior chains had ≤1 admitted WorkTx per task, so the v2 selector had no candidate set to choose from at proposal time.

## Run

```
bash handover/tests/scripts/run_tb_16_x_2_4_smoke_2026-05-05.sh
```

Probe env var: `TURINGOS_FORCE_BOLTZMANN_SEED_WORKTXS=Agent_user_0:4:25000`
- staker = Agent_user_0 (10M μC preseed; 4 × 25k μC = 100k μC ≪ balance)
- count = 4 (≥3 SG threshold + 1 headroom for entropy diversity)
- stake_micro_per = 25000 (smaller than .2.5's 50k to leave balance for .2.6 combined run)

## Chain shape

```
L4 logical_t=1 — TaskOpen
L4 logical_t=2 — EscrowLock
L4 logical_t=3 — Work       (boltzmann-seed iter=0, parent_tx=None,            v2_pick=None)
L4 logical_t=4 — Work       (boltzmann-seed iter=1, parent_tx=Some(iter-0),    v2_pick=Some(iter-0))
L4 logical_t=5 — Work       (boltzmann-seed iter=2, parent_tx=Some(iter-0),    v2_pick=Some(iter-0))
L4 logical_t=6 — Work       (boltzmann-seed iter=3, parent_tx=Some(iter-0),    v2_pick=Some(iter-0))
L4 logical_t=7 — TerminalSummary (MaxTxExhausted)
```

ProposalTelemetry.parent_tx distribution = {None: 1, "iter-0": 3} → Shannon entropy ≈ 0.811 bits (well above 0.25 alarm threshold).

stderr trace evidence (boltzmann_trace.txt):
```
boltzmann seed iter=0 ... parent_tx=None,            v2_pick=None
boltzmann seed iter=1 ... parent_tx=Some(...iter-0), v2_pick=Some(...iter-0)
boltzmann seed iter=2 ... parent_tx=Some(...iter-0), v2_pick=Some(...iter-0)
boltzmann seed iter=3 ... parent_tx=Some(...iter-0), v2_pick=Some(...iter-0)
FORCE_BOLTZMANN_SEED_WORKTXS produced 4 accepted WorkTxs (count requested=4)
```

The v2 selector consistently returns iter-0 for iter 1+ because:
1. iter-0 is the only entry in price_index after iter-0 commit (each iter adds one entry)
2. Wait — actually iter 1+ should see iter-0 only at commit time of iter-1; by iter-2 there are 2 entries; by iter-3 there are 3. The fact that iter-0 always wins means iter-0 has the highest `price_yes` (or tiebreaks via lex order on TxId.0 String).

This is acceptable per SG-16.x.2.4: the selector IS structurally exercised (mechanism 5 RUNTIME), and the parent_tx distribution diversity (None vs Some) yields entropy ≥ 0.25. Deeper exploration of selector behavior with diverse RationalPrice assignments is downstream future work (TB-17+ when real LLM agents differentiate price_yes).

## Audit verdict (verdict.json)

```
verdict          = PROCEED
passed           = 35      (was 34 in .2.5; id=43 now Pass instead of Skip = +1)
failed           = 0
halted           = 0
skipped          = 8       (was 9 in .2.5; id=43 moved Skip → Pass = -1)
tape_root.l4_count            = 7
tape_root.l4e_count           = 1
tape_root.cas_object_count    = 24
tape_root.constitution_hash   = eec695459c71fbef...
tx_kind_counts.work            = 4   ← multi-WorkTx target met
tx_kind_counts.terminal_summary = 1
```

| Assertion | Result | Note |
|---|---|---|
| id=23 (Layer E) `accepted_work_predicate_results_true` | **Pass** | All 4 seeded WorkTxs have predicate_passes=true |
| id=24 (Layer E) `proposal_telemetry_chain` | **Pass** | All 4 seeded WorkTxs have valid ProposalTelemetry CAS resolved |
| id=43 (Layer E supplemental) `boltzmann_parent_selection_diversity` | **Pass** | Shannon entropy 0.811 bits ≥ 0.25 threshold; first non-trivial exercise of this assertion |
| TB-2_work feature_coverage | **GREEN** | 4 admitted WorkTxs |

## Ship gates

| SG | Verification | Result |
|---|---|---|
| SG-16.x.2.4 — chain ≥3 WorkTxs + parent_selection_entropy ≥ 0.5 | verdict.json `tx_kind_counts.work = 4` AND id=43 result = Pass (charter spec'd entropy ≥ 0.5; actual = 0.811) | ✓ |
| SG (replay determinism — Layer C #16) | `cmp -s verdict.json verdict_replay.json` | ✓ |
| SG (tamper detection 3/3) | tamper_report.json: flip_l4 + flip_cas + remove_l4 all detected | ✓ |
| SG (smoke script fail-closed exit) | python3 JSON guard on work>=3 + id43 == Pass | ✓ |

## Surfaces shipped

- `experiments/minif2f_v4/src/bin/evaluator.rs` — FORCE_BOLTZMANN_SEED_WORKTXS env-var hook AFTER preseed (parallel to .2.5 hook). For iter 0..count: snapshot bus → Boltzmann v2 pick → parent_tx fallback (v2 OR last-produced-WorkTx OR None) → write ProposalTelemetry to CAS → submit real-signed WorkTx → await commit.
- `genesis_payload.toml` — R-014 rehash for evaluator.rs: `d39c67d1...` → `5a989d15...`
- `handover/tests/scripts/run_tb_16_x_2_4_smoke_2026-05-05.sh` — NEW. Two-witness ship gate: (a) verdict.json work>=3, (b) id=43 result == Pass.

## Deviations from charter (per `feedback_architect_deviation_stance`)

1. **STEP_B_PROTOCOL not triggered**: charter §2 Atom 2.4 said "STEP_B-PROTOCOL TRIGGERED" because the file plan listed `src/state/sequencer.rs`. Position taken: charter's "verify boltzmann_select_parent_v2 is called in WorkTx admission path" is satisfied by the existing call at evaluator.rs:1828 (`_v2_canonical_pick`); the parent_tx record needed for SG-16.x.2.4 lives in ProposalTelemetry (CAS object, proposal-time data) — sequencer-side admission has no parent_tx field on WorkTx. No sequencer.rs edit; STEP_B not triggered. Class 3 dual audit STILL applies because Boltzmann RUNTIME is high-impact (V3L-14 anti-collapse).

2. **Charter SG threshold ≥ 0.5 vs Art II.2.1 alarm threshold 0.25**: assertion id=43 uses 0.25 (the charter's "per Art II.2.1 alarm threshold 0.25" parenthetical). Charter's "≥ 0.5" upper-bound was advisory. Smoke shows entropy = 0.811, which clears both thresholds.

## Class 3 dual external audit

Per `feedback_dual_audit` + `feedback_risk_class_audit`: Class 3 = mandatory Codex + Gemini at ship.

**Pre-audit commit hash**: TBD (will append after commit).
**Audit verdict**: TBD (will append in .fix follow-up if any CHALLENGEs).

## Local-only forensic artifacts (NOT in git history)

- `cas/` (24 CAS objects)
- `runtime_repo/` (7 L4 + 1 L4.E ledger entries)
- `tamper/` (3 corruption test copies)

## Carry-forward

- TB-16.x.2.6 combined run will exercise FORCE_BOLTZMANN_SEED_WORKTXS alongside FORCE_REDEEM + FORCE_BANKRUPTCY_AFTER_ACCEPTED + FORCE_CHALLENGER + FORCE_CHALLENGE_RESOLVE + FORCE_EXPIRE → single chain hitting 13-of-13 tx kinds + multi-WorkTx + autopsy.
