# TB-G G3.2 — Architect §8 Ratification

> **Status**: RATIFIED. Architect provided multi-clause Class-4 §8 sign-off
> on 2026-05-12 (session #46), naming the act (`I ratify G3.2 as a Class-4
> STEP_B atom under the following decisions`) and authorizing scope subject
> to all conditions enumerated in §3. Structurally equivalent to canonical
> Class-4 §8 forms (`好，确认可以 ship` / `同意 sign-off` / `签字，同意后续执行`)
> per CLAUDE.md §10 multi-clause analysis.
>
> **Cross-reference**: `handover/directives/2026-05-12_TB_G_G3_2_§8_PACKET.md`
> (DRAFT packet authored 2026-05-12 session #46; this ratification clears the
> packet's §6 BLANK and §8 HALT). Resolves all Q1..Q6 + adds five §7
> supplementary requirements not anticipated by the packet.

## §0. Header

- **TB**: TB-G atom **G3.2** (Solvency emitter + sequencer-side risk-cap admission + Gap-A reputation accumulation + Gap-B verifier bond return)
- **Class**: **4 STEP_B** — `src/state/sequencer.rs` (4 admission arms) + `src/state/typed_tx.rs` (RejectionClass tail-append) + `src/bottom_white/cas/autopsy_capsule.rs` reuse (per-task-end emit)
- **Ratification commit**: this file (authored at HEAD `4d4412b` session #46 boot)
- **Phase-id**: P3-G (RSP Economy Generative Arena per TB-G charter §0)
- **FC-trace**: FC1-Runtime-Loop (admission predicate; predicate-fail → L4.E `BankruptcyRiskCapExceeded`) + FC3-Meta (AgentAutopsyCapsule derived-from-tape-and-CAS per-task-end emit)
- **Bundle scope**: Gap-A reputation accumulation + Gap-B verifier bond return INCLUDED in this §8 (Q4 verdict). `OBS_G2P_VERIFY_PEER_REWARD` flips 🟡 → 🟢 CLOSED on G3.2 ship.

## §1. Architect verbatim §8 sign-off

```
I ratify G3.2 as a Class-4 STEP_B atom under the following decisions:

Q1: risk cap = per-agent initial_balance_micro / 10, no new EconomicState risk-cap table.
Q2: reputation accumulation = uniform +1.
Q3: verifier bond return uses existing FinalizeRewardTx extension, not new BondReturnTx.
Q4: Gap-A reputation accumulation and Gap-B verifier bond return are bundled into G3.2.
Q5: bankruptcy risk-cap admission fires before per-arm balance/stake/router gates.
Q6: AutopsyCapsule emits per-task-end / per-bankruptcy-event.

Allowed:
- sequencer admission changes for WorkTx, BuyWithCoinRouterTx, ChallengeTx, VerifyTx;
- RejectionClass tail-append BankruptcyRiskCapExceeded;
- FinalizeRewardTx dispatch extension for verifier bond return;
- reputations_t +1 update;
- AgentAutopsyCapsule emit.

Forbidden:
- new TypedTx system tx variant;
- new BondReturnTx;
- new EconomicState bankruptcy_risk_cap_t table;
- WorkTx/VerifyTx/ChallengeTx/BuyRouter struct schema changes;
- batching with G4.2 or any other Class-4 atom;
- bypassing dual audit;
- bypassing minimal real-LLM smoke.

Ship only after:
- dual audit PASS under VETO > CHALLENGE > PASS;
- Trust Root verify passes;
- minimal 3-problem real-LLM smoke passes;
- at least one bankrupt/low-balance AutopsyCapsule path is witnessed or a deterministic fixture proves the path.
```

**Multi-clause verdict** (per CLAUDE.md §10):
- Clause 1 names act: `I ratify G3.2 as a Class-4 STEP_B atom`
- Clause 2 enumerates 6 Q1..Q6 decisions
- Clause 3 enumerates allowed scope (5 surfaces)
- Clause 4 enumerates forbidden scope (7 prohibitions)
- Clause 5 enumerates ship conditions (4 gates)
- Structurally equivalent to canonical Class-4 §8 forms per §10 multi-clause analysis.

## §2. Q1..Q6 verdicts (canonical)

| Q | Decision | Rationale (architect-verbatim summary) |
|---|----------|----------------------------------------|
| Q1 | per-agent `initial_balance_micro / 10`; NO new `bankruptcy_risk_cap_t` table | matches G3.1 SHIPPED `classify_solvency` boundary; read-view ↔ write-view consistency |
| Q2 | uniform `+1` per accepted VerifyTx | G3.2 lands the surface; complex verifier-reward theory deferred to forward TB |
| Q3 | B1 — extend `FinalizeRewardTx`; NO new `BondReturnTx` | reuses existing system-tx authorization; no new TxKind / signing domain |
| Q4 | bundle Gap-A + Gap-B into G3.2 | same sequencer surface; same Class-4 boundary; one architect review |
| Q5 | risk-cap fires FIRST (before per-arm gates) | more general failure subsumes more specific; Information Loom clustering |
| Q6 | per-task-end (per `TerminalSummaryTx` boundary) | fine-grained causal evidence; not run-end aggregate |

## §3. Supplementary requirements (architect §7; beyond Q1..Q6)

These five requirements were added by the architect after the packet's Q1..Q6 scope; they are part of the §8 sign-off conditions.

| # | Requirement | Surface | Test binding |
|---|-------------|---------|--------------|
| 7.1 | `RiskCapImpactReport` audit-output: risk_cap_rejections + agent_id + balance_before + risk_cap + tx_kind + task_id + whether_another_agent_continued + solve_outcome. Wire into audit dashboard so post-G3.2 solve-rate analysis can attribute regression to risk-cap suppression vs other causes. | NEW `src/runtime/risk_cap_impact_report.rs` + audit_dashboard render section | `tests/constitution_g3_risk_cap_impact_report.rs` |
| 7.2 | Below-risk-cap agents can still **read / observe / receive autopsy**; ONLY risky/staked actions (WorkTx stake, BuyRouter purchase, ChallengeTx stake, VerifyTx bond) blocked. Avoid epistemic disappearance. | Risk-cap precondition scope (4 arms only; NOT read-side `agent_visible_universe` / `compute_agent_pnl` / capsule read). | `tests/constitution_g3_below_cap_agent_can_still_read.rs` |
| 7.3 | AutopsyCapsule **private scoped read view** — latest Markov capsule only, NOT historical-failure prompt stuffing. Avoid context poisoning. | `src/sdk/snapshot.rs::UniverseSnapshot` autopsy field is the LATEST scoped capsule per viewer, not a list of all autopsies. | `tests/constitution_g3_autopsy_markov_not_history.rs` |
| 7.4 | Reputation +1 unique per `(verifier, target_work_tx)` pair — Sybil-spam guard. If existing system allows repeated VerifyTx against same target, reputation increments only on first unique pair. | `agent_verifications_t` uniqueness key. | `tests/constitution_g3_reputation_sybil_guard.rs::same_verifier_same_target_second_verify_no_extra_reputation` |
| 7.5 | `FinalizeRewardTx` result MUST separate `solver_reward_delta` + `verifier_bond_return_delta` (+ `other_settlement_delta` if present). Audit traceability: payout_sum ≤ escrow + bond return, no double credit. | `FinalizeRewardTx` post-dispatch payout view + audit_dashboard render. | `tests/constitution_g3_finalize_reward_payout_breakdown.rs` |

## §4. Authorized scope (§6.1 of packet — now LIVE)

**Allowed path**:
- Cut branch `feat/g3-2-risk-cap-admission` from `origin/main` HEAD `4d4412b`
- Implement atoms G3.2-A through G3.2-G as enumerated in §5 below
- Run shipgate: `cargo check --workspace` + `cargo test --workspace` + `bash scripts/run_constitution_gates.sh`
- Trust Root rehash (sequencer.rs + typed_tx.rs at minimum; autopsy/finalize_reward only if STEP_B-protected by manifest)
- PRE-§8 dual audit: Codex G2 + Gemini DT round-cap=2 covering Q1..Q12 in packet §6.3 + §3.7.1..7.5 of this ratification
- Minimal real-LLM 3-problem smoke OR deterministic fixture proving at least one bankrupt/low-balance AutopsyCapsule path
- Matrix flip §R G3 🟡 → 🟢 on ship-gates-all-PASS
- Handover update via `/handover-update`

**Forbidden path** (architect §1 clause 4 verbatim + packet §2.5 defended invariants):
- NO new `TypedTx` system tx variant
- NO new `BondReturnTx`
- NO new `EconomicState.bankruptcy_risk_cap_t` table
- NO struct-schema change to `WorkTx` / `VerifyTx` / `ChallengeTx` / `BuyWithCoinRouterTx`
- NO batching with G4.2 or any other Class-4 atom
- NO bypass of dual audit
- NO bypass of minimal real-LLM smoke
- NO new signing domain prefix
- NO new TxKind id
- NO `f64` in money path
- NO predicate change that reads price / market / trace data (risk-cap predicate reads `balances_t` only)
- NO `genesis_payload.toml` Trust Root manifest rotation beyond the 2 STEP_B file rehashes (sequencer + typed_tx)

**Risk class**: **4 STEP_B**.

**Audit required**: yes — Codex G2 + Gemini DT round-cap=2 PRE-§8.

**Ship authorized**: yes, conditional on (a) dual audit PASS under VETO > CHALLENGE > PASS, (b) shipgate (cargo + constitution gates) all GREEN, (c) Trust Root verify passes, (d) minimal real-LLM 3-problem smoke OR deterministic fixture witnesses ≥1 bankrupt/low-balance AutopsyCapsule path.

## §5. Atom sequencing (architect-mandated; per §8 clause 5)

Per `feedback_no_batch_class4_signoff` strict reading — atoms within a single Class-4 §8 may be implemented as one atomic STEP_B commit (G1.1 / P-M6 precedent). Implementation order respects dependency chain:

| Atom | Surface | Tests (architect-verbatim binding) |
|------|---------|------------------------------------|
| **G3.2-A** | `RejectionClass::BankruptcyRiskCapExceeded` tail-append in `src/state/typed_tx.rs` + display string + `transition_error_to_rejection_class` mapping + `transition_error_to_public_summary_token` token | `rejection_class_tail_appended` + `sg_g3_12_display_under_64_bytes` + `bankruptcy_risk_cap_display_low_pollution` |
| **G3.2-B** | `bankruptcy_risk_cap_micro(agent_id, q) -> i64` helper in `src/runtime/agent_pnl.rs` (reuse `initial_balance_micro_from_default_preseed / 10`) | `agent_1m_cap_100k` + `marketmaker_5m_cap_500k` + `sponsor_10m_cap_1m` + `no_new_state_table` + `risk_cap_threshold_matches_g3_1_near_insolvent` |
| **G3.2-C** | 4 admission arms (WorkTx + BuyWithCoinRouter + Challenge + Verify) gain risk-cap precondition FIRST; rejected attempts make zero state mutation | `sg_g3_10_*_below_cap_rejects_l4e` × 4 + `risk_cap_fires_before_per_arm_specific_check` + `rejected_attempts_make_no_state_mutation` + `above_cap_original_gate_still_fires` |
| **G3.2-D** | Gap-A reputation accumulation: `reputations_t[verifier] += 1` on accepted VerifyTx; Sybil guard via `(verifier, target_work_tx)` uniqueness | `verifytx_increments_reputation_uniform_plus_one` + `worktx_does_not_increment_verifier_reputation` + `challenge_does_not_increment_verifier_reputation` + `same_verifier_same_target_second_verify_no_extra_reputation` |
| **G3.2-E** | Gap-B verifier bond return: extend `FinalizeRewardTx` dispatch to credit verifier bonds; separate `solver_reward_delta` + `verifier_bond_return_delta` in result | `finalize_reward_returns_verifier_bond` + `bond_return_idempotent` + `bond_return_does_not_double_pay` + `bond_return_requires_system_signature` + `agent_cannot_submit_finalize_reward` + `payout_sum_le_escrow_plus_bond` |
| **G3.2-F** | `AgentAutopsyCapsule` per-task-end emit via NEW `emit_bankrupt_autopsies_at_problem_end` helper; reuses TB-15 `write_autopsy_capsule` + `is_autopsy_active_at` gate; private scoped Markov view (latest only, NOT history) | `bankrupt_agent_at_problem_end_receives_autopsy` + `solvent_agent_at_problem_end_no_autopsy` + `autopsy_capsule_id_content_addressable` + `autopsy_emit_replay_deterministic` + `autopsy_emit_skipped_when_activation_gate_off` + `autopsy_private_detail_not_agent_visible` + `autopsy_public_summary_bounded` + `autopsy_markov_latest_not_history` |
| **G3.2-G** | `RiskCapImpactReport` audit-output surface + dashboard wire-up | `risk_cap_impact_report_renders_rejections` + `risk_cap_impact_report_attributes_solve_outcome` |
| **G3.2-H** | Shipgate: cargo + constitution gates + Trust Root rehash | (validation gate — no new test) |
| **G3.2-I** | Real-LLM 3-problem smoke OR deterministic fixture | smoke evidence at `handover/evidence/g_phase_g3_2_smoke_2026-05-12T<ts>Z/` |
| **G3.2-J** | Dual audit dispatch (Codex G2 + Gemini DT) | dual audit verdicts at `handover/audits/CODEX_G2_TB_G_G3_2_VERDICT.md` + `handover/audits/GEMINI_DT_TB_G_G3_2_VERDICT.md` |
| **G3.2-K** | Matrix flip §R G3 🟡 → 🟢 + LATEST.md handover update | matrix diff + LATEST.md diff |

## §6. Ship gates (binding; from §8 clause 5)

| ID | Gate | Validation |
|----|------|------------|
| SHIP-G3.2.1 | Dual audit PASS under VETO > CHALLENGE > PASS | Codex G2 + Gemini DT verdict files both PROCEED |
| SHIP-G3.2.2 | Trust Root verify passes | `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` PASS |
| SHIP-G3.2.3 | Minimal 3-problem real-LLM smoke passes | `chain_invariant verdict=Ok delta=0 audit_proceed=true` on smoke run |
| SHIP-G3.2.4 | ≥1 bankrupt/low-balance AutopsyCapsule path witnessed | Real smoke witness OR deterministic fixture |
| SHIP-G3.2.5 (architect §7.1) | RiskCapImpactReport renders in audit dashboard | `risk_cap_impact_report_renders_rejections` test PASS + dashboard render section present |
| SHIP-G3.2.6 (architect §7.2) | Below-cap agents can still read/observe | `below_cap_agent_can_still_read` test PASS |
| SHIP-G3.2.7 (architect §7.3) | AutopsyCapsule scoped Markov view (latest only) | `autopsy_markov_latest_not_history` test PASS |
| SHIP-G3.2.8 (architect §7.4) | Sybil-guard reputation uniqueness | `same_verifier_same_target_second_verify_no_extra_reputation` test PASS |
| SHIP-G3.2.9 (architect §7.5) | FinalizeRewardTx payout breakdown separates solver + verifier deltas | `finalize_reward_payout_breakdown` test PASS |

## §7. Rollback plan (per packet §6.2)

Per Stage C P-M2..P-M9 rollback precedent (HEAD `01dd825` 2026-05-09) + G1.1 / P-M6 STEP_B revert-able shape:

1. G3.2 lands as 1 atomic STEP_B commit (collapsed-rehash pattern: sequencer + typed_tx + autopsy emit + finalize_reward + tests + Trust Root + matrix bundled).
2. Rollback: `git revert <G3.2-commit-sha>` produces clean inverse commit.
3. State preserved on rollback: G3.1 / G3.3 / G3.4 / G1.1 / all prior STEP_B atoms intact.
4. `RejectionClass::BankruptcyRiskCapExceeded` variant disappears (no historical L4.E records reference it).
5. Matrix re-revert: §R G3 🟢 → 🟡 (back to current state).
6. OBS_G2P_VERIFY_PEER_REWARD reverts to 🟡 (Gap-A/B re-opens).
7. 3+ new test files removed; existing tests unaffected.
8. Trust Root re-rehash: revert sequencer + typed_tx SHA256s in genesis_payload.toml.

## §8. Forward items NOT covered by this §8

Per `feedback_no_batch_class4_signoff` + packet §8:

- **G4.2** (Multi-LLM mix + No-Hidden-Model-Switch detector) — SEPARATE Class-4 §8 packet required; INDEPENDENT of G3.2 (can draft in parallel; sequencing-only, not dependency).
- **G2P.4** (PromptCapsule swarm-write) — Class 2-3; ship-eligible under parent §8 G-Phase autonomous-forward.
- **G5.1 / G5.2 / G5.3** (Opportunity scheduler + 7-action menu + role classifier) — Class 1-2 autonomous after G3.2 + G4.2 ship.
- **G6.1 / G6.2 / G6.3** (Epistemic pricing feedback) — Class 1-2 autonomous after G3.2 + G4.2 ship.
- **G7** (Structural smoke) — Class 1-2 autonomous after G5 + G6 ship.
- **TB-GD** (Gardener Agent / stale artifact scanner) — forward charter for the only RED row; report-only, no ChainTape mutation.

---

**End of architect §8 ratification. Implementation cleared to proceed at `feat/g3-2-risk-cap-admission` branch.**
