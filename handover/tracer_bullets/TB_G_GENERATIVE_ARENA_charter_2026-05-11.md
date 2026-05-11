# TB-G — Constitution-Gated Generative Arena — Charter (2026-05-11)

> **Class 0 charter** drafted autonomously per architect 2026-05-11 G-Phase
> verdict (ARCHIVED at `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`,
> committed at `2c110dc` pre-teleport sweep). Binding directive: **enter
> generative phase G**. Substrate (TB-N3) is accepted as landed; next
> deliverable is persistent multi-agent market collaboration on the same
> tape — not more CPMM mechanics.
>
> **Origin**: web ultraplan session `01QqSehGhpsts18AC5qExyAS` approved by
> user verbatim "plan approved, returned to terminal and execute your plan"
> on 2026-05-11.

---

## §0. Tags (mandatory per `feedback_tb_phase_tag_required`)

- **phase_id**: P3-G — RSP Economy Generative Arena (post-TB-N3 substrate
  landing; agent-side market dynamics on the persistent tape).
- **roadmap_exit_criteria_addressed**:
  1. P3 Exit "persistent multi-agent market collaboration on tape" — agents
     live across problems, accumulate balance / positions / reputation /
     autopsy, and produce post-accept survival/reuse market activity.
  2. P3 Exit "constitution as arena boundary, not product" — generative
     market activity emerges **without** loosening any of the 6 constitutional
     gates: tape-first / no-ghost-liquidity / no-price-as-truth /
     dashboard-derived-only / no-real-funds / no-public-chain.
  3. v3 run6 structural equivalence (architect §G7) — Minimum-tier 12 items
     witnessed structurally on tape; Mid + Late tier deferred to TB-G+1.
- **kill_criteria_tested**:
  1. If post-G-Phase batch evidence on the 9-problem set shows per-problem
     genesis reset (balances reset, positions cleared, reputation zeroed)
     between problems, reject — G1 ship-gate violation.
  2. If any G-Phase atom changes the Predicate to read price / market /
     trace data, reject — Art. I.1.1 PCP violation (architect SG-G6.4).
  3. If any bankrupt agent successfully stakes / pays / bonds above the
     `BANKRUPTCY_RISK_CAP_MICRO` cap, reject — architect SG-G3.4 violation
     (G3.2 risk-cap admission).
  4. If any agent's resolved `model_name` in `AttemptTelemetry` differs from
     its genesis `agent_model_assignment` without a ChainTape record, reject
     — architect SG-G4.5 violation (G4.4 no-hidden-model-switch).
  5. If end-of-batch `assert_total_ctf_conserved` /
     `assert_no_post_init_mint` / `assert_complete_set_balanced` flips RED,
     reject — economy conservation violation under generative load.
- **Authority basis**: architect verdict 2026-05-11 verbatim (archived in
  `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`) +
  CLAUDE.md §13 economy laws + §19 no-manipulation-by-sequencing +
  `feedback_no_batch_class4_signoff`. Class-0 charter authority autonomous;
  Class-3+ atoms require per-atom architect §8 (G1.1, G3.2, G4.2 are
  Class-4 STEP_B and HALT for sign-off).

## §0.6. Architect ruling 2026-05-11 — binding amendments

Verbatim source at `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`.
Five binding amendments override any prior TB-N* assumption that conflicts:

- **Amendment G-1**: persistence priority is non-negotiable. G1 (cross-problem
  persistence) MUST land before G4 (multi-LLM). Architect §3 verbatim:
  "如果没有 persistence，multi-LLM 也只是一群模型轮流重置状态".
- **Amendment G-2**: peer verification bridge is a **parallel priority**
  with G1/G2, not a forward step. Architect §8.2: "verify_peer=0 比 invest=0
  更危险". G2P module is mandated.
- **Amendment G-3**: post-accept alpha is borne by `WorkTx.stake`. Plan
  must not force router-buy on every problem; G7.1 SG-G7.3 is batch-wide,
  not per-problem. Empty-market on a single problem is permitted **and**
  must be classified by an explicit `NoTradeReason` (e.g., `TooFastSolve`).
- **Amendment G-4**: round-robin `agent_idx = tx % n_agents` is pseudo
  multi-agent. G5.1 must replace it with an opportunity scheduler exposing
  a 7-action menu: `{propose_proof, verify_peer, challenge_node,
  invest_long, invest_short, abstain, bid_task}`.
- **Amendment G-5**: empty-market is a valid empirical result (§8.5). G7.2
  §K renders a clean-negative explanation + forward-TB stub when minimum-tier
  is not met; this is the deliverable, not a failure.

## §0.7. Non-objectives (architect §10 verbatim)

This phase is NOT:
- More substrate compliance work (TB-N3 already landed it).
- A public benchmark / public chain / real-funds deployment.
- DeFi expansion / new market mechanics (no new CPMM kernel atoms).
- A constitution-loosening exercise. The 6 constitutional gates remain
  in force as **arena boundary**.

---

## §1. Modules (G0..G7)

| Module | Goal | Class peak | §8 packet required |
|--------|------|-----------|--------------------|
| G0 | Charter + verdict archive reference + matrix §R rows | 0 | no |
| G1 | Cross-Problem Persistence (one runtime_repo + CAS + HEAD_t across N problems) | **4** (G1.1 resume mode) | **yes** (G1.1) |
| G2 | MarketDecisionTrace audit + NoTradeReason extension + L4.E failed-invest binding | 2 | no |
| G2P | Peer Verification Bridge (architect §8.2 parallel priority) | 2 | no |
| G3 | Persistent PnL / Solvency / Bankruptcy risk-cap admission | **4** (G3.2 sequencer admission) | **yes** (G3.2) |
| G4 | Multi-LLM Mix + No-Hidden-Model-Switch detector | **4** (G4.2 genesis schema) | **yes** (G4.2) |
| G5 | Opportunity Scheduler + 7-action menu + Role Classifier | 3 | no |
| G6 | Epistemic Pricing Feedback (observe-only) + Unresolved-Challenged filter | 2 | no |
| G7 | Structural Run6-Equivalent Smoke (13 Minimum-tier sub-gates) + Mid-tier `--mid-tier` flag + Late-tier TB-G+1 stub | 2 | no |

### Module G0 — Charter Reset (Class 0; landing now)

| Atom | Class | Code surface | Ship gate |
|------|-------|--------------|-----------|
| **G0.1** This charter | 0 | `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md` | charter cites architect directive verbatim + names G1..G7 atoms + lists kill criteria |
| **G0.2** Architect verdict archive | 0 | (already canonical at `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md` 586 lines, committed at `2c110dc`) | no duplicate archive — single source of truth |
| **G0.3** Matrix §R row block | 0 | `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` §R (NEW; rows for each SG-G* gate, RED until atom test lands) | matrix row count grows by 9 (one per Module G1..G7 + G2P aggregate + SG-G overall); rows stay RED until corresponding test exists |

### Module G1 — Cross-Problem Persistence (HIGHEST priority)

| Atom | Class | Code surface | Ship gate |
|------|-------|--------------|-----------|
| **G1.1** Resume-mode genesis branch — `RuntimeChaintapeConfig.resume_existing_chain: bool` flag (env `TURINGOS_CHAINTAPE_RESUME=1`); `build_chaintape_sequencer` learns a `resume` branch: (a) opens existing `refs/transitions/main` instead of fail-closing on `NonEmptyRuntimeRepo`; (b) calls `reconstruct_from_chaintape_refs` (exists at `src/state/head_t_witness.rs`) to rebuild `QState` from L4 + L4.E + CAS; (c) sets `Sequencer.next_logical_t = chain_length` so strict `len + 1` invariant holds on next commit. | **4 STEP_B** | `src/runtime/mod.rs:407` + `src/state/sequencer.rs` (`Sequencer::new_at_logical_t` constructor) + `src/state/head_t_witness.rs` (reuse `reconstruct_from_chaintape_refs`) + `tests/constitution_g1_resume.rs` (NEW; 5+ gates) | SG-G1.1..SG-G1.5: empty-repo byte-equal genesis / N-entry resume `next_logical_t == N` / balances reconstruction matches forward replay / `NonEmptyRuntimeRepo` only fires when resume=false / pinned_pubkeys preserved across resume |
| **G1.2** Batch driver binary | 3 | NEW `experiments/minif2f_v4/src/bin/batch_evaluator.rs` + NEW `experiments/minif2f_v4/src/swarm_one_problem.rs` (extract from `evaluator.rs:829..1700`) | SG-G1.6..SG-G1.8: ONE `runtime_repo` across N problems / legacy `evaluator` binary unchanged tests / failure mid-batch yields `RunExhaustedTx` terminal state |
| **G1.3** Persistent batch wrapper script | 2 | NEW `scripts/run_g_phase_batch.sh` (sibling of `run_stage_b3.sh:364`) | SG-G1.9..SG-G1.10: one evidence dir shape `<run>/<model>/seed<S>/rep<R>/{runtime_repo,cas,PROBLEMS.txt,batch_summary.json}` / `audit_tape` PROCEED across full batch |
| **G1.4** Persistence-evidence binding test (6 architect-required persisted fields) | 2 | `tests/constitution_g1_persistence_evidence_binding.rs` (NEW) | SG-G1.11..SG-G1.15: ≥1 Agent_i balance trajectory non-flat / replay byte-equality / market-history walk non-empty / proof-performance trajectory ≥1 accept+reject for some agent / autopsy index monotone-add |

### Module G2 — Market Decision Observability (parallel with G1; Turing lens)

| Atom | Class | Code surface | Ship gate |
|------|-------|--------------|-----------|
| **G2.1** NoTradeReason audit + 2 net-new variants | 2 | `src/runtime/market_decision_trace.rs:38` (existing 11 variants + 2 new: `NoPerceivedEdge`, `PromptBudgetExceeded`; doc-alias `AmountExceedsBalance` ↔ architect's `InsufficientBalance`) + `src/runtime/adapter.rs:1315` + `evaluator.rs:3120..` | SG-G2.1..SG-G2.6: trace-or-tx for every market-bearing turn / failed invest in L4.E / source-grep covers each variant / 13-variant exhaustive name check |
| **G2.2** `audit_dashboard --run-report` §F NoTradeReason rows | 2 | `src/bin/audit_dashboard.rs::render_tb_n3_run_report` (extend) | SG-G2.4: §F renders per-`NoTradeReason` count + submitted/traced ratio |
| **G2.3** Failed-invest L4.E binding test | 2 | `tests/constitution_g2_failed_invest_l4e.rs` (NEW) | SG-G2.5: router-rejected `BuyWithCoinRouterTx` lands in L4.E with matching `RejectionClass` |

### Module G2P — Peer Verification Bridge (architect §8.2 PARALLEL priority; Nakamoto lens)

| Atom | Class | Code surface | Ship gate |
|------|-------|--------------|-----------|
| **G2P.1** `=== Pending Peer Reviews ===` prompt block | 2 | `src/sdk/prompt.rs` + `evaluator.rs:2098..` | SG-G2P.1..G2P.2: per-viewer (no peer CoT leak) / fixture renders pending-review row |
| **G2P.2** Peer-verify-coverage §F.X + walker | 2 | `src/bin/audit_dashboard.rs::render_tb_n3_run_report` | SG-G2P.3..G2P.5: walker emits per-agent `peer_verify_count` / §F.X renders coverage % / persistent-batch ≥1 non-solver VerifyTx OR explicit bottleneck explanation |
| **G2P.3** Verifier reward / bond return audit | 1 | `src/state/sequencer.rs` (existing VerifyTx arm) | SG-G2P.6: existing TB-N1 A4 gates GREEN OR `OBS_G2P_VERIFY_PEER_REWARD` filed |

### Module G3 — Persistent PnL / Solvency / Bankruptcy (Drucker lens)

| Atom | Class | Code surface | Ship gate |
|------|-------|--------------|-----------|
| **G3.1** `compute_agent_pnl` derived view (7-field `AgentMarketStateView`) | 2 | NEW `src/runtime/agent_pnl.rs` | SG-G3.1..G3.3 + SG-G3.9: genesis returns zero-pnl / post-BuyRouter cash drops + unrealized updates / 5 scenarios covered / 7 fields present (source-grep) |
| **G3.2** Solvency emitter + **sequencer-side risk-cap admission** (4 admission arms: WorkTx + BuyRouter + Challenge + Verify); new `BankruptcyRiskCapExceeded` RejectionClass | **4 STEP_B** | `src/runtime/agent_pnl.rs` + `src/state/sequencer.rs` (4 admission sites) + `src/state/typed_tx.rs` (tail-append RejectionClass) + reuse `AgentAutopsyCapsule::new` at problem-end boundary | SG-G3.4 + SG-G3.10..G3.12: AutopsyCapsule emit on bankrupt / WorkTx >cap → L4.E / BuyRouter >cap → L4.E / `BankruptcyRiskCapExceeded` display <64B |
| **G3.3** `=== Your Position ===` prompt block (7 fields, per-viewer, Drucker framing) | 3 | `src/sdk/prompt.rs` + `evaluator.rs:2098..` | SG-G3.6..G3.7 + SG-G3.13: per-viewer source-grep / non-default render witnessed in smoke / Drucker verbatim framing string present |
| **G3.4** §G PnL trajectory report + G1 SG-G1.7 dual-bind | 2 | `src/bin/audit_dashboard.rs::render_tb_n3_run_report` + `tests/constitution_g1_pnl_trajectory_evidence_binding.rs` | SG-G3.8 + SG-G1.7-bind: §G has ≥1 non-flat trajectory row / dual-binding test reads evidence dir and confirms |

### Module G4 — Multi-LLM Mix (Hayek lens)

| Atom | Class | Code surface | Ship gate |
|------|-------|--------------|-----------|
| **G4.1** 3-model-family CSV + `PHASE_D_HETERO_OK=1` activation | 2 | `scripts/run_g_phase_batch.sh` + `genesis_payload.toml` model-snapshot | SG-G4.1..G4.2: heterogeneous vector resolution / >1 distinct `model_name` in CAS index |
| **G4.2** `[agent_model_assignment]` genesis schema | **4 STEP_B** | `src/runtime/bootstrap.rs` + `src/runtime/genesis_report.rs` + `genesis_payload.toml` | SG-G4.3..G4.4: replay reads assignment; Trust Root rehash |
| **G4.3** §H model-family activity breakdown | 2 | `src/bin/audit_dashboard.rs::render_tb_n3_run_report` | SG-G4.5: ≥1 cell with non-identical no-trade-reason distribution between models |
| **G4.4** No-hidden-model-switch detector | 2 | `tests/constitution_g4_no_hidden_model_switch.rs` (NEW) + `src/runtime/audit_assertions.rs` (NEW assertion id) | SG-G4.6..G4.7: walker finds zero `AttemptTelemetry.model_name ≠ assignment` rows / no runtime mutation of `agent_models[]` |

### Module G5 — Opportunity Scheduler + Role Classifier

| Atom | Class | Code surface | Ship gate |
|------|-------|--------------|-----------|
| **G5.1** Opportunity scheduler + 7-action menu; `TURINGOS_SCHEDULER=opportunity\|round_robin` env-gated; default round-robin (back-compat) | 3 | `evaluator.rs:1987` + NEW `src/runtime/agent_scheduler.rs` + `src/sdk/protocol.rs::AgentAction` (extend) + `src/sdk/prompt.rs` (menu render) | SG-G5.1..G5.3 + SG-G5.7..G5.9: round-robin back-compat / opportunity weights bankrupt down / Boltzmann determinism / 7 actions reachable / Abstain selection works / bankrupt selection prob ≤0.1× peers |
| **G5.2** Role classifier (chain+CAS only) | 2 | NEW `src/runtime/agent_role_classifier.rs` | SG-G5.4..G5.5: 5+ unit tests one-per-role / total assignment |
| **G5.3** §I roles + mechanism-bottleneck explainer | 2 | `src/bin/audit_dashboard.rs::render_tb_n3_run_report` | SG-G5.6 + SG-G5.10: ≥2 distinct roles in batch / single-role explainer renders ≥3 candidate causes |

### Module G6 — Epistemic Pricing Feedback (observe-only; Hayek strict)

| Atom | Class | Code surface | Ship gate |
|------|-------|--------------|-----------|
| **G6.1** market_context per-row trace-hint counts | 2 | `src/sdk/market_context.rs:45` + reuse `MarketDecisionTrace` walk | SG-G6.1..G6.2: `Predicate` source-grep clean / fixture renders hint counts |
| **G6.2** §J citation-vs-price correlation + high-price selection rate | 1 | `src/bin/audit_dashboard.rs::render_tb_n3_run_report` | SG-G6.3..G6.4: correlation row + selection-rate row distinct |
| **G6.3** Unresolved-challenged-not-promoted-as-safe gate | 2 | `tests/constitution_g6_unresolved_challenged_not_safe.rs` (NEW) + `src/sdk/market_context.rs` (filter step) | SG-G6.5..G6.6: open-Challenge WorkTx filtered from market_context top-K + still admissible to L4 (Predicate untouched) / filter source-grep |

### Module G7 — Structural Smoke + Empty-Market Discipline

| Atom | Class | Code surface | Ship gate |
|------|-------|--------------|-----------|
| **G7.1** 9-problem persistent batch smoke (Minimum-tier 13 sub-gates) | 2 | `handover/evidence/g_phase_smoke_2026-05-11T*Z/` | SG-G7.1..G7.13 |
| **G7.2** §K mechanism-witness OR clean-negative + forward-TB stub | 1 | `src/bin/audit_dashboard.rs::render_tb_n3_run_report` + `handover/tracer_bullets/TB_G_FORWARD_HYPOTHESES_DRAFT.md` (NEW only-if-empty) | SG-G7.14..G7.15 |
| **G7.3** Mid-tier `--mid-tier` flag (non-blocking) | 1 | `src/bin/audit_dashboard.rs --mid-tier` | SG-G7.16 |
| **G7.4** Late-tier TB-G+1 charter stub | 1 | `handover/tracer_bullets/TB_G_PLUS_1_LATE_TIER_charter_DRAFT_2026-05-11.md` (NEW; draft only) | SG-G7.17 |

## §2. Phase ship-gate aggregate

| Aggregate | Witness |
|-----------|---------|
| FC1/FC2/FC3 GREEN under G-Phase smoke | matrix snapshot at G-Phase batch HEAD |
| `CONSTITUTION_EXECUTION_MATRIX.md` §R all rows GREEN | matrix sync after each Module |
| `assert_total_ctf_conserved` GREEN end-of-batch | G7.1 SG-G7.11 |
| `assert_no_post_init_mint` GREEN end-of-batch | G7.1 SG-G7.12 |
| `assert_complete_set_balanced` GREEN end-of-batch | G7.1 SG-G7.13 |
| Cross-problem PnL trajectory present | G3.4 SG-G3.8 dual-bound to G1 SG-G1.7 |
| ≥2 distinct roles detected on persistent batch | G5.3 SG-G5.6 + G7.1 SG-G7.9 |
| ≥1 non-solver VerifyTx witness | G2P SG-G2P.5 + G7.1 SG-G7.5 |
| No-hidden-model-switch witness | G4.4 SG-G4.6 |
| Bankruptcy risk-cap admission active | G3.2 SG-G3.10 + SG-G3.11 |
| Unresolved-challenged-node not promoted as safe | G6.3 SG-G6.5 |
| Minimum-tier 13-gate smoke passes OR §K clean-negative + forward-TB stub | G7 SG-G7.1..G7.17 |

## §3. Architect §8 packets required (3 Class-4 atoms)

Per `feedback_no_batch_class4_signoff`, each Class-4 atom HALTs and produces
its own §8 packet:

1. **G1.1** resume-mode genesis branch — `handover/directives/2026-05-1X_TB_G_G1_1_§8_PACKET.md` (drafted in G0 closure)
2. **G3.2** sequencer risk-cap admission — `handover/directives/2026-05-1X_TB_G_G3_2_§8_PACKET.md`
3. **G4.2** `agent_model_assignment` genesis schema — `handover/directives/2026-05-1X_TB_G_G4_2_§8_PACKET.md`

Phase-overall §8 packet at phase end (architect ship-or-veto on G-Phase
aggregate evidence) is in addition, not in lieu.

## §4. Forbidden list (architect §10 verbatim + plan derivations)

- NO new mint sites (architect "no ghost liquidity" preserved).
- NO predicate change that reads price / market / trace data.
- NO real funds.
- NO public chain anchor.
- NO automatic role-promotion of unresolved-challenged nodes.
- NO model-name change at runtime without ChainTape record.
- NO bypass of `BANKRUPTCY_RISK_CAP_MICRO` for any agent.
- NO substitute for selective-shielding: prompt blocks (`=== Your Position ===`,
  `=== Pending Peer Reviews ===`) MUST be per-viewer; never broadcast another
  agent's PnL or peer-review queue.
- NO global latest pointer reintroduction (CLAUDE.md §0).
- NO `f64` in money path (CLAUDE.md §12 + §13).
- NO shadow-ledger source of truth (CLAUDE.md §12).

## §5. Cross-references

- Architect directive: `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`
- Matrix §R rows: `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` §R
- Predecessor TB: `handover/tracer_bullets/TB_N3_POLYMARKET_AGENT_BRIDGE_charter_2026-05-11.md`
- Predecessor evidence: TB-N3 Phase 2 batch (6/9 solved, 6/6 auto-emit node-survive markets, 0 invest)
- CLAUDE.md §13 economy laws (binding throughout G-Phase)
- `feedback_no_batch_class4_signoff` (Class-4 §8 cadence)

## §6. Status

- 2026-05-11 — G0.1 charter drafted (this file); G0.2 archive verified in
  place (directive 586 lines committed at `2c110dc`); G0.3 matrix §R rows
  pending; G1.1 §8 packet draft pending; G-Phase HALT after G0 ship + G1.1
  packet draft per Class-4 §8 cadence.
