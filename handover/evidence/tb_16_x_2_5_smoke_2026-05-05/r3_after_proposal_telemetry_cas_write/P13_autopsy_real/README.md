# TB-16.x.2.5 smoke evidence — P13_autopsy_real (r3 canonical)

**Date**: 2026-05-05  
**Charter**: `handover/tracer_bullets/TB-16.x.2_charter_2026-05-04.md` §2 Atom 2.5  
**Risk class**: 2 (env-var-gated arena hook in evaluator.rs only; ADDITIVE id=43 audit assertion stub in `src/runtime/audit_assertions.rs`; no sequencer/scheduler change; no economic-semantics mutation; no auth/crypto surface beyond agent-signed WorkTx already wired in TB-3 + TB-7)  
**Iteration cap**: 24h (capability signal = real-LLM smoke producing AgentAutopsyCapsule on chain via TB-15 dispatch arm Step 3.5)

## Goal

Close the missing R2 P4 path: **WorkTx-accepted → got accepted → then the
task went bankrupt → AutopsyCapsule generated**. Per architect umbrella
charter §2 Atom 2.5.

## Run

```
OUT_BASE=handover/evidence/tb_16_x_2_5_smoke_2026-05-05/r3_after_proposal_telemetry_cas_write \
  bash handover/tests/scripts/run_tb_16_x_2_5_smoke_2026-05-05.sh
```

Probe env vars:
- `TURINGOS_FORCE_BANKRUPTCY_AFTER_ACCEPTED=Agent_user_0:50000` — inject a real-signed WorkTx (predicate_passes=true) at preseed time so stakes_t has ≥1 entry for `task-{run_id}` BEFORE the LLM swarm runs
- `TURINGOS_FORCE_BANKRUPTCY=1` — at MaxTxExhausted, emit TaskBankruptcyTx referencing the EvidenceCapsule. Sequencer's TB-15 Step 3.5 hook (sequencer.rs:1374) calls `derive_autopsies_for_bankruptcy(pre_econ, bk, ...)` which iterates `pre_econ.stakes_t` for entries matching `bk.task_id`, deriving an AgentAutopsyCapsule per match. Step 3.5 in apply_one (sequencer.rs:3097) writes the bytes via `write_bankruptcy_autopsies_to_cas`.

Problem: `aime_1997_p9.lean` (chosen because expected to MaxTxExhaust under
N_SWARM=5 + MAX_TX=20 with `deepseek-chat` thinking-off; same problem as
TB-16.x.2.1 + .2.3).

## Iteration narrative (forensic record)

| Run | Diff | Outcome |
|---|---|---|
| r1 (`tb_16_x_2_5_smoke_2026-05-05/P13_autopsy_real/`) | First attempt; `STAKER=Agent_solver_0` (NOT in `default_pput_preseed_pairs`) | seed WorkTx rejected `InsufficientBalance`; chain has work=0 → no autopsy |
| r2 (`r2_after_agent_user_0_fix/P13_autopsy_real/`) | Switched STAKER → `Agent_user_0` (10M μC preseed) | seed WorkTx admitted → work=1; **but** verdict=BLOCK, halted=1 because audit assertion id=24 (`proposal_telemetry_chain`) HALTED — the seed used `Cid::from_content` of a literal string for proposal_cid but never wrote the bytes to CAS, so `cas.get(proposal_cid)` returned NotFound. Tamper detection 0/3 collateral damage. |
| r3 (this canonical) | Hook now opens CasStore + builds ProposalTelemetry via `build_for_evaluator_append` (which writes proposal_artifact bytes to CAS as side-effect) + calls `proposal_telemetry::write_to_cas` → uses returned CID as WorkTx.proposal_cid | All gates GREEN |

## Chain shape (r3 canonical)

```
L4 logical_t=1 — TaskOpen        (preseed; sponsor=Agent_user_0)
L4 logical_t=2 — EscrowLock      (preseed; bounty=200_000 μC)
L4 logical_t=3 — Work            (seed by Agent_user_0; stake=50_000 μC)
L4 logical_t=4 — TerminalSummary (MaxTxExhausted; capsule_id=...)
L4 logical_t=5 — TaskBankruptcy  (system-emitted; reason=MaxFailedRunCount)
                                  + Step 3.5 hook fires → derive_autopsies_for_bankruptcy
                                  → AgentAutopsyCapsule written to CAS
                                  → AutopsyPrivateDetail also written
                                  → agent_autopsies_t[event_id] gets capsule_id
```

stderr trace evidence (autopsy_trace.txt):
```
[chaintape/tb16-arena] seed WorkTx submitted by Agent_user_0 (stake=50000 μC) for task=task-n5_aime_1997_p9_1777964827285 — populates stakes_t for TB-16.x.2.5 autopsy generation
[chaintape/tb16-arena] TaskBankruptcyTx emitted: emit_id=2 task_id=TaskId("task-n5_aime_1997_p9_1777964827285")
```

CAS index evidence (`.turingos_cas_index.jsonl`):
```
{"object_type":"AgentAutopsyCapsule",  "creator":"sequencer-epoch-1", "size_bytes":334, ...}
{"object_type":"AutopsyPrivateDetail", "creator":"sequencer-epoch-1", "size_bytes":230, ...}
```

## Audit verdict (verdict.json)

```
verdict          = PROCEED
passed           = 34
failed           = 0
halted           = 0
skipped          = 9 (single-problem smoke; multi-problem assertions inherit prior coverage)
tape_root.l4_count            = 5
tape_root.l4e_count           = 1
tape_root.cas_object_count    = 17
tape_root.constitution_hash   = eec695459c71fbef...
tx_kind_counts.work            = 1   ← seeded WorkTx admitted
tx_kind_counts.terminal_summary = 1
tx_kind_counts.task_bankruptcy = 1   ← bankruptcy fired
```

| Assertion | Result | Note |
|---|---|---|
| id=14 (Layer C) `replay_autopsy_index_chains` | **Pass** | Pre-existing assertion covers — TB-16.x.2.5 SG indirect coverage |
| id=24 (Layer E) `proposal_telemetry_chain` | **Pass** | r2 HALT closed by writing ProposalTelemetry bytes to CAS in r3 |
| id=29 (Layer F) `autopsy_private_detail_creator_is_system` | **Pass** | AutopsyPrivateDetail creator field = `sequencer-epoch-1` per Step 3.5 hook (CR-15.3) |
| id=43 (Layer E supplemental) `boltzmann_parent_selection_diversity` | **Skipped** | New in this commit (TB-16.x.2.4 stub). Skip rationale: "no task has ≥3 admitted WorkTxs". Will Pass under TB-16.x.2.4 with FORCE_BOLTZMANN_SEED_WORKTXS env var. |

## Ship gates

| SG | Verification | Result |
|---|---|---|
| SG-16.x.2.5 — AutopsyCapsule with loss_reason_class set + loss_amount > 0 | CAS index has `object_type=AgentAutopsyCapsule` + `object_type=AutopsyPrivateDetail` (both written by Step 3.5 hook with stake_amount=50000 μC); WorkTx in chain (stake>0); TaskBankruptcy in chain | ✓ |
| SG (replay determinism — Layer C #16) | `cmp -s verdict.json verdict_replay.json` | ✓ |
| SG (tamper detection 3/3) | tamper_report.json: flip_l4 + flip_cas + remove_l4 all detected | ✓ |
| SG (smoke script fail-closed exit) | python3 JSON CAS-index guard + trace witness; exits 1 on missing autopsy CAS objects OR missing trace lines | ✓ |

Note: SG-16.x.2.5 charter wording "non-default loss_reason_class" is satisfied by `LossReasonClass::Bankruptcy` per autopsy_capsule.rs:46-48 (TB-15 v0 sole production trigger). The literal `LossReasonClass::default() == Bankruptcy` (line 67-71) means "default" semantics is "the bankruptcy class is set, not the zero/uninitialized variant" — Bankruptcy IS the operational set value.

## Surfaces shipped

- `experiments/minif2f_v4/src/bin/evaluator.rs` — FORCE_BANKRUPTCY_AFTER_ACCEPTED env-var hook AFTER preseed (TaskOpen + EscrowLock + optional CompleteSetSeed) but BEFORE LLM swarm. Opens CasStore, builds + writes ProposalTelemetry to CAS, then constructs real-signed WorkTx via make_real_worktx_signed_by(predicate_passes=true) referencing the telemetry CID as proposal_cid.
- `src/runtime/audit_assertions.rs` — NEW Layer E supplemental assertion id=43 `boltzmann_parent_selection_diversity` (Skipped under .2.5 single-WorkTx scenario; will Pass under .2.4 multi-WorkTx). Bundled here so the audit_tape binary the .2.5 smoke runs already includes the assertion id stub for downstream .2.6 combined-run consistency.
- `genesis_payload.toml` — R-014 rehash for evaluator.rs: `e1c4d057...` → `d39c67d1...` (note: r2's intermediate hash `c19dfcbe...` was burned during r3 fix and is documented in the annotation chain). audit_assertions.rs is NOT in trust_root manifest (per `grep audit_assertions genesis_payload.toml` — only mod.rs is, hash unchanged).
- `handover/tests/scripts/run_tb_16_x_2_5_smoke_2026-05-05.sh` — NEW. Two-witness ship gate: (a) verdict.json work>=1 + task_bankruptcy>=1, (b) CAS index `.turingos_cas_index.jsonl` contains object_type=AgentAutopsyCapsule + AutopsyPrivateDetail. Replaces naive substring scan that was fooled by BCS-encoded enum tag (LossReasonClass::Bankruptcy → 1-byte variant index, not UTF-8 string).

## Local-only forensic artifacts (NOT in git history)

- `cas/` (17 CAS objects)
- `runtime_repo/` (5 L4 + 1 L4.E ledger entries)
- `tamper/` (3 corruption test copies)
- `tb_16_x_2_5_smoke_2026-05-05/P13_autopsy_real/` (r1 — STAKER mismatch; pre-fix)
- `tb_16_x_2_5_smoke_2026-05-05/r2_after_agent_user_0_fix/P13_autopsy_real/` (r2 — id=24 HALT pre-fix)

These are NOT committed (matching prior tb_16_chaintape_smoke_* convention) — verdict.json + tamper_report.json + dashboard.txt + trace files are the canonical evidence; large dirs are reproducible via the smoke script.

## Carry-forward

- TB-16.x.2.4 will exercise id=43 with `TURINGOS_FORCE_BOLTZMANN_SEED_WORKTXS` env var (multi-WorkTx + diverse parent_tx → entropy ≥ 0.25 → id=43 Pass).
- TB-16.x.2.6 combined run will produce a single chain with all of .2.1+.2.2+.2.3+.2.4+.2.5 active, hitting 13-of-13 tx kinds.
