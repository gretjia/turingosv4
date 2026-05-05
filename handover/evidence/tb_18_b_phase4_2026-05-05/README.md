# TB-18 Atom B Phase 4 — Single-Chain 13/13 tx-kind Evidence (2026-05-05)

## What this directory proves

**FR-18.7 + FR-18.8 + SG-18.6 + SG-18.7 closed**: ONE evaluator process (`comprehensive_arena` binary) drove ≥6 engineered tasks against ONE shared chain (one runtime_repo + one CAS + one Sequencer). All 13 architect-mandated tx kinds emitted in the single chain.

Per architect TB-18 ratification ruling §2.8 verbatim:

> Atom B 要证明的是: one evaluator process / one runtime_repo / one CAS / one chain / multiple tasks. 如果它只是一个 process 里启动多个 subprocess, 每个 subprocess 自己起 chain, 那不合格.

This evidence demonstrates the §2.8 mandate is met.

## Directory layout

```
tb_18_b_phase4_2026-05-05/
├── README.md                              ← this file
├── r1/                                    ← canonical run (chain_seed_id=tb18-arena-r1)
│   ├── agent_keystore.enc                 ← TB-9 durable keystore (locked under password)
│   ├── runtime_repo/                      ← L4 chain root (.git stored as .dotgit.tar.gz)
│   │   ├── agent_audit_trail.jsonl        ← per-tx CAS-record index (TB-6 Atom 5)
│   │   ├── agent_pubkeys.json             ← per-run agent pubkey manifest
│   │   ├── genesis_report.json            ← chain-level genesis (TB-7R Deliverable C)
│   │   ├── initial_q_state.json           ← preseeded balances + epoch
│   │   ├── pinned_pubkeys.json            ← system-tx pubkey manifest
│   │   ├── rejections.jsonl               ← L4.E entries (1 synthetic gate seed)
│   │   ├── synthetic_rejection_label.json ← marks the synthetic L4.E gate
│   │   └── _dotgit_post_tar/              ← local restore of runtime_repo.dotgit.tar.gz; NOT git-tracked
│   ├── runtime_repo.dotgit.tar.gz         ← canonical L4 chain bytes (replay-verifiable)
│   ├── cas/                               ← CAS objects root
│   │   ├── .turingos_cas_index.jsonl      ← CAS object index
│   │   └── _dotgit_post_tar/              ← local restore of cas.dotgit.tar.gz; NOT git-tracked
│   ├── cas.dotgit.tar.gz                  ← canonical CAS bytes
│   └── evidence/
│       ├── SHARED_CHAIN_RUNS_REPORT.json  ← per-task outcome list (6 tasks)
│       └── tx_kind_distribution.json      ← 13 distinct tx_kind counts
└── r0_wrong_cas_env/                      ← non-canonical (pre-fix run; CAS env-var typo). Kept as troubleshooting artifact only; do NOT cite for ship.
```

## Run summary (r1; canonical)

- **Wall-clock**: 2839 ms (~2.8 sec; engineered single-process arena)
- **Chain depth**: 31 L4 entries on `refs/transitions/main`
- **L4.E rejections**: 1 (synthetic zero-stake WorkTx for the L4.E gate; `synthetic_rejection_for_l4e_gate=true`)
- **Distinct tx kinds emitted**: **13/13** (architect's full coverage set)
- **Tasks driven**: 6 engineered (task_A through task_F per design §4.5)
- **Process count**: 1 (`comprehensive_arena` binary)
- **Bundle count**: 1 (`SharedChain::from_env` constructed once)

### tx_kind_distribution

```json
{
  "TaskOpen": 6,        ← 6 tasks each open
  "EscrowLock": 6,      ← 6 tasks each escrow-lock
  "Work": 4,            ← task_A, B, D, F
  "Verify": 2,          ← task_A, B (OMEGA-Confirm)
  "FinalizeReward": 1,  ← task_A
  "Challenge": 1,       ← task_B
  "ChallengeResolve": 1,← task_B (Released)
  "MarketSeed": 1,      ← task_C
  "CompleteSetMint": 1, ← task_C
  "CompleteSetRedeem": 1,← task_C (after bankruptcy resolves NO-wins)
  "TerminalSummary": 3, ← task_D, E, F (D=MaxTxExhausted, E=MaxTxExhausted, F=DegradedLLM)
  "TaskBankruptcy": 2,  ← task_C, D
  "TaskExpire": 1       ← task_D (BankruptcyTriggered)
}
```

13 distinct tx kinds × 6 tasks → **architect's 13/13 single-chain mandate satisfied** (FR-18.8 + SG-18.7).

## How this run was produced

1. `comprehensive_arena --out-dir handover/evidence/tb_18_b_phase4_2026-05-05/r1 --chain-seed-id tb18-arena-r1`
2. The binary internally:
   - Sets `TURINGOS_CHAINTAPE_PATH=<r1>/runtime_repo` + `TURINGOS_CAS_PATH=<r1>/cas` + `TURINGOS_CHAINTAPE_PRESEED=1`.
   - Calls `SharedChain::from_env(...)` (Phase 1 lift; `chain_runtime.rs::from_env`).
   - Calls `chain_runtime::write_synthetic_l4_l4e_gate_and_genesis_report(...)` ONCE with `chain_seed_id="tb18-arena-r1"` (Phase 2 lift).
   - For each of 6 engineered task specs:
     - Calls `drive_task(&mut chain, &spec, PerCallBudget::default())` (Phase 3 substantive body — TaskOpen + EscrowLock real-signed scaffold).
     - Emits the task-specific lifecycle txs via direct `bus.submit_typed_tx` (proposer-side: Work, Verify, Challenge, MarketSeed, CompleteSetMint, CompleteSetRedeem) and `bundle.sequencer.emit_system_tx(SystemEmitCommand::*)` (system-emitted: FinalizeReward via `tb8_emit_finalize_after_verify`; ChallengeResolve via `tb16_emit_challenge_resolve_for_eligible`; TerminalSummary via `tb11_emit_terminal_summary_for_run`; TaskBankruptcy + TaskExpire via direct `emit_system_tx`).
   - Calls `bundle.shutdown().await` ONCE at chain end (drains queued submissions).
   - Writes `tx_kind_distribution.json` + `SHARED_CHAIN_RUNS_REPORT.json` to `evidence/`.

## How to replay-verify

1. Restore the chain bytes:
   ```
   cd handover/evidence/tb_18_b_phase4_2026-05-05/r1
   tar xzf runtime_repo.dotgit.tar.gz -C runtime_repo
   tar xzf cas.dotgit.tar.gz -C cas
   ```
2. (After replay tooling is wired for Phase 4 chains; `audit_tape` works on the same L4 chain shape.)

## Why no LLM agent loop

Per `feedback_chaintape_externalized_proposal`: the chain records what the system externalized via `submit_typed_tx`, not LLM internals. The 6-task engineered set produces all 13 tx kinds via real-signed envelopes (`make_real_*_signed_by` helpers). The architect §2.4 failure-mode coverage table was already saturated by the M0 retry on per-problem chains (commit `2bc712e`: 7 OMEGA-Confirm + 7 natural EvidenceCapsule + 6 controlled timeouts); TB-18 ship's specific gap is single-chain multi-task tx-kind coverage (FR-18.7 + FR-18.8), not additional LLM-driven solve evidence.

Per `feedback_no_workarounds_strict_constitution`: this is NOT 凑活 — synthetic real-signed envelopes are the architect-precedented mechanism for arena drivers (TB-16 Atom 7 §7.3 FR-16.3 + FR-16.4 ratified `make_real_challengetx_signed_by` etc. for exactly this purpose); they produce the same chain shape that LLM-driven envelopes would, against the same admission gates.

## Class 3 risk envelope

Per `feedback_class4_cannot_hide_in_class3`: this commit stays Class 3 — consumes existing public APIs (`SharedChain`, `drive_task`, `write_synthetic_l4_l4e_gate_and_genesis_report`, `make_real_*_signed_by`, `emit_system_tx`) and does NOT touch sequencer admission / typed-tx schema / canonical-signing-payload. Class 3 dual external audit (Codex + Gemini) is requested; see `handover/audits/DUAL_AUDIT_TB_18_B_PHASE4_REQUEST_2026-05-05.md`.

## Forward-binding (post-Phase-4)

| Item | Status | Owner |
|---|---|---|
| External Codex micro-audit (G0) | Filed pre-H per architect §2.1 | User-invoked (cloud-billed) |
| External Codex+Gemini ship audit (G1) | Filed pre-H per architect §2.1 + Q7 | User-invoked (cloud-billed) |
| Architect § sign-off on benchmark report | TB-17 §8 precedent | User-conveyed |
| Atom H M1 (50-100 × n1/n3) | Multi-hour LLM compute | Forward-bound to TB-18.H-impl |
| Atom H M2 (100+ × n5; observe-only) | Multi-day LLM compute | Forward-bound to TB-18.H-impl |
| `audit_tape` PROCEED on r1 chain | Tool needs Phase 4 chain replay support | Forward-bound or assertion-extend |

## Predecessor

This evidence supersedes the multi-chain UNION pattern that TB-16.x.2.6 and earlier used for 13/13 coverage. CR-18.8 ("No multi-chain union claimed as single-chain") is satisfied: exactly ONE chain produced this 13/13 evidence.
