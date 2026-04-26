# TuringOS v4 FINAL BLUEPRINT — 宪法落地完整图

> **Date**: 2026-04-26
> **Authoritative inputs**:
> - `constitution.md` (with Art. 0–0.4 amendments 2026-04-26)
> - `handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md` (architecture chapter, 21 §)
> - `handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md` (economic chapter, 21 §)
>
> **Status**: ArchitectAI synthesis. **Supersedes** TFR v1 (handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md, 20% coverage) and **subsumes** CO_MEGA_PLAN v3 outline. Plan v3.1 (`CO_MEGA_PLAN_v3.1_2026-04-26.md`) atomizes this blueprint.
>
> **Audience**: This document is the single source of truth for "what code must exist for TuringOS v4 to be a faithful operationalization of the constitution + white paper + economic chapter". Every file path, every module, every invariant.

---

## § 0 单句论纲 (One-Sentence Thesis)

> **TuringOS v4 = (反奥利奥三层架构) × (ChainTape 六层基底) × (RSP 经济结算) × (DO-178C 一一对照)**
>
> **图灵机角度**: tape = ChainTape; control = predicates + Settlement; memory = CAS + LedgerTape; alphabet = boolean predicates ∪ statistical signals; halt = Constitution(Q_t).

Architecture chapter定义了**结构**（什么是状态、什么是信号、什么是层）。Economic chapter定义了**问责**（谁押注、谁挑战、谁结算、奖金从哪来）。两者必须**同时实现**——只做架构没有经济=AGI 没有重力；只做经济没有架构=代币炒作没有内核。

---

## § 1 Q_t 完整状态规范 (9 Components)

Architecture § 4 定义 8 components；Economic § 2 amendment 添加 `economic_state_t` 作为第 9 component。**Final canonical Q_t**:

```rust
// src/state/q_state.rs (NEW; CO P1.2)
pub struct QState {
    // === Architecture chapter § 4 ===
    pub q_t: AgentSwarmState,                  //  1  agent swarm 子状态
    pub head_t: NodeId,                        //  2  当前 ChainTape head (git commit SHA, Path B)
    pub state_root_t: Hash,                    //  3  materialized state Merkle root (git tree root)
    pub tape_view_t: AgentVisibleProjection,   //  4  agent 可见的 tape projection (filtered by visibility)
    pub ledger_root_t: Hash,                   //  5  L4 transition ledger root
    pub predicate_registry_root_t: Hash,       //  6  L1 predicate registry root
    pub tool_registry_root_t: Hash,            //  7  L2 tool registry root

    // === Economic chapter § 2 amendment ===
    pub economic_state_t: EconomicState,       //  8  完整经济子状态（见下）
    pub budget_state_t: BudgetSnapshot,        //  9  全局预算快照 (cost ceiling, wall clock, compute)
}

pub struct EconomicState {
    pub balances_t: BalancesIndex,             // 账户 → coin balance (founder grant + earned)
    pub escrows_t: EscrowsIndex,               // task_id → locked bounty pool
    pub stakes_t: StakesIndex,                 // (agent, task) → YES/NO stake amount
    pub claims_t: ClaimsIndex,                 // pending claims awaiting Settlement
    pub reputations_t: ReputationsIndex,       // agent → 非转让信誉 (Inv 9)
    pub task_markets_t: TaskMarketsIndex,      // active task list + price index
    pub royalty_graph_t: RoyaltyGraph,         // tool reuse → builder royalty edges
    pub challenge_cases_t: ChallengeCasesIndex, // open challenge windows
    pub price_index_t: PriceIndex,             // 广播信号: task price + risk price + scarcity
}
```

**Tape Canonical Axiom** (constitution Art. 0.2): every field in `QState` MUST be reconstructible from ChainTape replay. No field may be a "live cache only". Validation: `tests/q_state_reconstruct.rs` replays tape from genesis, asserts byte-identical `QState`.

---

## § 2 反奥利奥三层 — File-Level Module Organization

Architecture § 3 mandates 3 layers; every file in `src/` MUST be classifiable into exactly one layer. Mixed-layer modules (current `src/bus.rs`, `src/kernel.rs`) violate Art. I.1 and MUST be split.

### ⚪ Top White Layer — `src/top_white/` (NEW)

**Role**: predicates, signals, budgets. Sees agent inputs but NOT agent internal state. Decides accept/reject.

```
src/top_white/
├── mod.rs
├── predicates/
│   ├── mod.rs
│   ├── registry.rs              // L1 Predicate Registry (CO P1.5)
│   ├── visibility.rs            // Public | Private | CommitReveal (Goodhart shield)
│   ├── acceptance/              // 接受谓词 (Inv 6)
│   │   ├── lean4_oracle.rs      // 移自 experiments/.../src/lean4_oracle.rs
│   │   ├── format_check.rs
│   │   └── safety_class.rs
│   ├── settlement/              // 结算谓词 (Inv 7)
│   │   ├── monetary_invariant.rs   // Inv 3, 4 货币守恒
│   │   ├── attribution_check.rs    // Inv 8
│   │   └── reputation_update.rs
│   └── runner.rs                // PredicateRunner (RSP-1 § 19)
├── signals/
│   ├── mod.rs
│   ├── boolean.rs               // pass/fail (architecture § 7)
│   ├── statistical.rs           // price, reputation, scarcity (architecture § 7)
│   └── price_broadcast.rs       // 仅广播价格信号；评分器屏蔽 (Inv 10)
└── budget/
    ├── mod.rs
    ├── cost_ceiling.rs          // architecture § 8.1 budget law
    ├── wall_clock.rs            // 移自 experiments/.../src/wall_clock.rs
    └── compute_cap.rs
```

### ⚫ Middle Black Layer — `src/middle_black/` (NEW) + `experiments/.../agents/`

**Role**: agent reasoning, self-organization, market participation. **Top can NOT inspect Middle internal CoT**; only sees Middle's submitted `work_tx`.

```
src/middle_black/
├── mod.rs
├── agent_protocol.rs            // submit_work_tx / submit_verify_tx / submit_challenge_tx
├── role_self_select.rs          // emergent roles (memory: feedback_emergent_roles)
└── librarian_board.rs           // public team state for self-selection

experiments/minif2f_v4/src/
├── agents/                      // (NEW; reorganize current scattered code)
│   ├── solver.rs                // submits work_tx + YES_E stake (Economic § 7)
│   ├── verifier.rs              // submits verify_tx + reputation/bond stake
│   ├── challenger.rs            // submits challenge_tx + NO_E stake
│   ├── builder.rs               // creates reusable tool → deferred bonus + royalty
│   ├── architect_ai.rs          // proposes new architecture → meta bounty
│   └── judge_ai.rs              // vetoes constitution violations → low-error reward
├── experiment_mode.rs           // existing 5-mode CCL ablation
└── bin/
    ├── evaluator.rs             // swarm orchestrator (Middle wrapper)
    └── oneshot_evaluator.rs     // single-agent baseline
```

### ⚪ Bottom White Layer — `src/bottom_white/` (NEW)

**Role**: deterministic, append-only substrate. tape, CAS, ledger, sandbox, materializer.

```
src/bottom_white/
├── mod.rs
├── tape/
│   ├── mod.rs
│   ├── chain_tape.rs            // 6-layer ChainTape coordinator (NEW)
│   ├── git_substrate.rs         // gix integration (Path B; CO P1.3)
│   └── tape_canonical_check.rs  // V-01..V-24 conformance
├── cas/                         // L3 Content-Addressable Store (CO P1.4)
│   ├── mod.rs
│   ├── store.rs                 // git blob = CAS object
│   └── schema.rs                // cid + hash + type + creator + visibility
├── ledger/                      // L4 Transition Ledger (CO P1.7)
│   ├── mod.rs
│   ├── transition.rs            // 11-field tx schema
│   ├── wal.rs                   // 移自 src/wal.rs (Bottom)
│   └── ledger.rs                // 移自 src/ledger.rs (Bottom)
├── tools/                       // L2 Tool Registry (CO P1.6)
│   ├── mod.rs
│   ├── registry.rs              // tool_id + capability + permission_policy
│   ├── rtool/                   // Read-only tools
│   ├── wtool/                   // Write tools (must produce work_tx)
│   └── permission.rs
├── sandbox/
│   ├── mod.rs
│   └── exec.rs                  // 移自 src/sdk/sandbox.rs
├── materializer/                // L5 Materialized State (CO P1.8)
│   ├── mod.rs
│   ├── state_db.rs
│   ├── agent_view.rs            // visibility-filtered projection (Inv 10)
│   └── indices.rs               // task / reputation / error / price / permission
└── signal_index/                // L6 Signal Indices (CO P1.9)
    ├── mod.rs
    ├── boolean_index.rs
    ├── stat_index.rs
    └── reuse_count.rs
```

### Economy Cross-Cuts — `src/economy/` (NEW; **structurally Top White, but warrants its own root for Atom hygiene**)

```
src/economy/                     // RSP-1 modules (Economic § 19)
├── mod.rs
├── task_market.rs               // 发布任务 + 广播价格 + 锁定奖金
├── escrow_vault.rs              // bounty + stake + deferred + royalty pools
├── contribution_ledger.rs       // work_tx / verify_tx / challenge_tx / reuse_tx
├── attribution_engine.rs        // Contribution DAG → 贡献权重 (Inv 8)
├── challenge_court.rs           // 挑战期 + 反例 + slash + 回滚 (Inv 7)
├── settlement_engine.rs         // 3-layer rewards: immediate / deferred / royalty
├── reputation_index.rs          // 非转让信誉 (Inv 9)
├── price_index.rs               // 广播信号 (Inv 10)
└── invariants/                  // 12 经济不变式 conformance
    ├── inv01_no_thinking_reward.rs
    ├── inv02_no_direct_collect.rs
    ├── inv03_escrow_only.rs
    ├── inv04_no_post_mint.rs
    ├── inv05_yes_no_event_bound.rs
    ├── inv06_predicate_gated.rs
    ├── inv07_provisional_then_final.rs
    ├── inv08_dag_attribution.rs
    ├── inv09_reputation_immutable.rs
    ├── inv10_signal_vs_evaluator.rs
    ├── inv11_chain_record_only.rs
    └── inv12_consensus_not_truth.rs
```

**Layering rule**: `src/economy/*` may DEPEND ON `src/top_white/*` and `src/bottom_white/*` but `src/middle_black/*` may NOT depend on `src/economy/*` directly — agents only see economy via `librarian_board.rs` market broadcast (price signals only, never evaluator internals).

---

## § 3 ChainTape 6 层 — File Mapping

Architecture § 5 specifies 6 layers; below maps each to file + Phase scope.

| Layer | Spec | Files | Phase |
|---|---|---|---|
| **L0 Constitution Root** | constitution_hash + human_signature + sudo_policy | `genesis_payload.toml` + `src/bottom_white/tape/chain_tape.rs::genesis()` | CO P1.0 (formalize) |
| **L1 Predicate Registry** | predicate_id + version + code_hash + schema + visibility + owner + test_suite_hash | `src/top_white/predicates/registry.rs` + `src/top_white/predicates/visibility.rs` | CO P1.5 |
| **L2 Tool Registry** | tool_id + capability + schema + permission + determinism + side_effect | `src/bottom_white/tools/registry.rs` | CO P1.6 |
| **L3 CAS Object Store** | cid + hash + schema + type + creator + visibility | `src/bottom_white/cas/store.rs` (gix blob) | CO P1.4 |
| **L4 Transition Ledger** | 11-field tx | `src/bottom_white/ledger/transition.rs` + `wal.rs` + `ledger.rs` | CO P1.7 |
| **L5 Materialized State + Agent View** | state_db + indices + permission_view | `src/bottom_white/materializer/*` | CO P1.8 |
| **L6 Signal Indices** | boolean + price + reputation + scarcity + explore/exploit | `src/bottom_white/signal_index/*` + `src/top_white/signals/*` | CO P1.9 |

Each layer has a **conformance test**:
```
tests/chain_tape_L0_constitution_root.rs
tests/chain_tape_L1_predicate_registry.rs
tests/chain_tape_L2_tool_registry.rs
tests/chain_tape_L3_cas.rs
tests/chain_tape_L4_transition_ledger.rs
tests/chain_tape_L5_materialized_state.rs
tests/chain_tape_L6_signal_indices.rs
```

---

## § 4 状态转移协议 — Pseudo-Code

This is the canonical state transition that EVERY agent action goes through. Architecture § 6 + Economic § 6 + Constitution Art. I.

```rust
// src/transition/mod.rs (NEW; CO P1.7)
pub fn step_transition(
    q: &mut QState,
    agent: AgentId,
    proposal: Proposal,            // Middle Black opaque output
) -> Result<TransitionOutcome, RejectReason> {

    //  1. READ — Agent reads from L5 Agent View (visibility filtered per Inv 10)
    //     ALREADY HAPPENED before this fn; agent's prompt was assembled from agent_view.rs

    //  2. PROPOSE — Agent submits work_tx
    let work_tx = WorkTx {
        tx_id: TxId::new(),
        task_id: proposal.task_id,                   // CRITICAL: links to TaskMarket entry (WP § 5.L4 lines 357-369; Codex CO P0.7 fix)
        parent_state_root: q.state_root_t,
        agent_id: agent,
        read_set:  proposal.read_set,
        write_set: proposal.write_set,
        proposal_cid: cas::put(&proposal.payload)?,  // L3 CAS write
        predicate_results: PredicateResults::pending(),
        stake: proposal.stake,                       // YES_E stake (Inv 5)
        signature: agent.sign(&work_tx_digest),
        timestamp: now(),
        status: TxStatus::Pending,
    };  // 12 fields per WP L4 schema

    //  3. PREDICATE GATE — Top White acceptance predicates (Inv 6)
    let acceptance = top_white::predicates::runner::run_acceptance(
        &q.predicate_registry_root_t,
        &work_tx,
    )?;
    if !acceptance.all_passed() {
        return Err(RejectReason::AcceptanceFailed(acceptance));
        // 不改变 world state (Inv 6)
    }

    //  4. PROVISIONAL REWARD — issue claim, NOT money yet (Inv 7)
    let claim = economy::contribution_ledger::record_work_tx(work_tx)?;
    let provisional = economy::settlement_engine::issue_provisional(
        claim,
        &q.economic_state_t.escrows_t,
    )?;

    //  5. STATE TRANSITION — append to L4, update L5
    bottom_white::ledger::transition::append(&mut q, work_tx)?;
    bottom_white::materializer::state_db::apply(&mut q, work_tx)?;

    //  6. SIGNAL EMIT — L6 indices update (broadcast price + reputation; NOT evaluator internals; Inv 10)
    bottom_white::signal_index::update(&mut q, work_tx);
    top_white::signals::price_broadcast::emit(work_tx.price_signal());

    //  7. CHALLENGE WINDOW OPEN — challengers can submit challenge_tx (Inv 7)
    economy::challenge_court::open_window(claim);

    //  8. (deferred) FINALIZE — after challenge window closes:
    //     - if NO valid challenge: settlement_engine::finalize() pays bounty
    //     - if VALID challenge: rollback + slash + challenger reward

    Ok(TransitionOutcome::Provisional(claim))
}
```

**Final reward formula** (Economic § 21):
```
reward_i =
  Finalize(
      Escrow(task)               // 来自 EscrowVault 锁定额
    × Accept(tx_i)               // 0/1, acceptance predicates
    × Attribution(tx_i, DAG)     // [0,1], AttributionEngine
    × Survival(challenge_window) // 0/1, ChallengeCourt
    × Utility(post_metrics)      // [0,∞), downstream reuse
    × Constitution(Q_t)          // 0/1, JudgeAI veto
  )
```

每一项 → 至少 1 conformance test。

---

## § 5 12 Economic Invariants → Conformance Tests

Direct from Economic chapter § 18. Each invariant gets exactly one conformance test file.

| Inv | Statement (zh) | Test file | Phase |
|---|---|---|---|
| 1 | 不因思考获奖，只因被接受的状态转移 | `tests/economic_invariant_INV1_no_thinking_reward.rs` | CO P2.3 |
| 2 | Agent 提 claim，Settlement Engine 决定 | `tests/economic_invariant_INV2_no_direct_collect.rs` | CO P2.6 |
| 3 | 奖金来自预锁 escrow / treasury，不得事后增发 | `tests/economic_invariant_INV3_escrow_only.rs` | CO P2.2 |
| 4 | on_init 之后不得铸新基础 Coin | `tests/economic_invariant_INV4_no_post_mint.rs` | CO P2.0 |
| 5 | YES/NO 是事件绑定权利，非无抵押新币 | `tests/economic_invariant_INV5_yes_no_event_bound.rs` | CO P2.7 |
| 6 | 未通过谓词的 work_tx 不得改变 world state | `tests/economic_invariant_INV6_predicate_gated.rs` | CO P1.5 |
| 7 | 通过谓词→provisional；挑战窗结束→final | `tests/economic_invariant_INV7_provisional_then_final.rs` | CO P2.5 |
| 8 | 贡献归因来自 DAG + 统计信号，非自我声明 | `tests/economic_invariant_INV8_dag_attribution.rs` | CO P2.4 |
| 9 | 信誉不可转让，不可替代谓词 | `tests/economic_invariant_INV9_reputation_immutable.rs` | CO P2.9 |
| 10 | 价格信号广播，完整评分器屏蔽 | `tests/economic_invariant_INV10_signal_vs_evaluator.rs` | CO P1.5 |
| 11 | 链上：承诺+状态根+结算；链下：推理+测试+长上下文 | `tests/economic_invariant_INV11_chain_record_only.rs` | CO P1.7 |
| 12 | 共识只证记录被接受，不证现实事实为真 | `tests/economic_invariant_INV12_consensus_not_truth.rs` | CO P1.0 |

**Plus Tape Canonical V-01..V-24 conformance** (existing, see TAPE_CANONICAL_AUDIT_2026-04-26):
```
tests/tape_canonical_V01_completion_tokens.rs
... through ...
tests/tape_canonical_V24_audit_guard_provenance.rs
```

**Plus Economic Audit E-01..E-04**:
```
tests/economic_audit_E01_production_default_on.rs
tests/economic_audit_E02_jsonl_summary.rs
tests/economic_audit_E03_naming.rs
tests/economic_audit_E04_founder_grant_law2.rs
```

Total conformance tests: **12 + 24 + 4 = 40** dedicated tests. Plus per-layer ChainTape tests (7) = **47 minimum conformance tests**.

---

## § 6 Bidirectional Trace Matrix v3 — Seed

Constitution / White paper paragraph → code symbol(s). DO-178C standard. Bidirectional + production-invocation column.

**Seed entries** (full matrix in `handover/alignment/TRACE_MATRIX_v3_2026-04-26.md`, populated atom-by-atom in CO P1.13):

| Source | Para | Code symbol | Production invocation | Test |
|---|---|---|---|---|
| Constitution Art. 0 (Turing fundamentalism) | 4 elements | `bottom_white::tape::chain_tape::ChainTape` + `bottom_white::tools::wtool::*` + `bottom_white::ledger::wal::Wal` + `top_white::predicates::registry::PredicateRegistry` | All swarm ticks via evaluator.rs | `tests/turing_fundamentalism.rs` |
| Constitution Art. 0.2 (Tape Canonical Axiom) | 24 V's | `bottom_white::tape::tape_canonical_check::*` | Pre-flight at evaluator.rs::run_swarm start | `tests/tape_canonical_V01..V24.rs` |
| Constitution Art. 0.4 (Q_t versioning) | Path B | `bottom_white::tape::git_substrate::*` | runtime_repo per cell at evaluator.rs::on_cell_start | `tests/git_substrate_runtime_repo.rs` |
| Constitution Art. I.1 (反奥利奥) | 3 layers | `top_white/*` + `middle_black/*` + `bottom_white/*` (no cross-layer leak) | layer_audit.rs at every commit | `tests/anti_oreo_layer_audit.rs` |
| Constitution Art. II.2.1 (statistical signals) | reputation/PPUT/consensus | `top_white::signals::statistical::*` + `economy::reputation_index::*` | run_swarm summary emit | `tests/statistical_signals_complete.rs` |
| Constitution Art. IV (terminal categorization) | halt_reason | `experiments/.../bin/evaluator.rs::HaltReason` enum | Every solve summary | `tests/halt_reason_distribution.rs` |
| White paper Architecture § 3 (Anti-Oreo) | 3 layers | (same as Art. I.1) | (same) | (same) |
| White paper Architecture § 4 (Q_t) | 8 components | `state::q_state::QState` 9-field struct | Every step_transition | `tests/q_state_reconstruct.rs` |
| White paper Architecture § 5 (ChainTape 6-layer) | L0-L6 | `bottom_white::tape::chain_tape::ChainTape::layer_root(L)` | step_transition commit | `tests/chain_tape_L0..L6.rs` |
| White paper Architecture § 7 (boolean vs statistical signals) | dichotomy | `top_white::signals::boolean::*` vs `statistical::*` | predicate dispatch | `tests/signal_dichotomy.rs` |
| White paper Architecture § 9.4 (Goodhart shield) | public/private/commit-reveal | `top_white::predicates::visibility::Visibility` | predicate runner load | `tests/goodhart_shield.rs` |
| White paper Architecture § 12 (Go Meta) | ArchitectAI/JudgeAI | `experiments/.../agents/{architect_ai, judge_ai}.rs` | meta_proposals branch | `tests/go_meta_runtime.rs` (Phase 3, deferred) |
| White paper Economic § 2 (Q_t amendment) | economic_state_t | `state::q_state::EconomicState` | (same as § 4) | `tests/economic_state_reconstruct.rs` |
| White paper Economic § 18 Inv 1-12 | 12 invariants | `economy::invariants::inv01..inv12` | Every settlement | `tests/economic_invariant_INV1..12.rs` |
| White paper Economic § 19 (RSP-1 9 modules) | TaskMarket..PriceIndex | `economy::{task_market, escrow_vault, ...}` (9 modules) | step_transition stages 4-7 | `tests/rsp1_modules_smoke.rs` |
| White paper Economic § 7 (5 agent roles) | Solver/Verifier/Challenger/Builder/Architect/Judge | `experiments/.../agents/{solver, verifier, challenger, builder, architect_ai, judge_ai}.rs` | run_swarm role dispatch | `tests/agent_role_economic.rs` |
| White paper Economic § 21 (final formula) | reward_i = Finalize(...) | `economy::settlement_engine::finalize_reward` | challenge_window close | `tests/final_reward_formula.rs` |

**Bidirectional check**: every `pub` symbol in `src/{top_white,middle_black,bottom_white,economy,state,transition}/*.rs` MUST carry a `/// TRACE_MATRIX <C-art|WP-arch-§|WP-econ-§>: <role>` doc-comment, validated by `tests/trace_matrix_v3_bidirectional.rs`.

---

## § 6.5 Agent Role Count Reconciliation (Codex CO P0.7 fix)

White paper Economic chapter § 7 is titled "Agent **5** 经济角色" but lists **6** roles (Solver / Verifier / Challenger / Builder / ArchitectAI / JudgeAI). Blueprint § 2 Middle Black Layer carries the 6-role list. Codex audit flagged the 5-vs-6 inconsistency.

**Resolution** (PROVISIONAL — user reviews on wake): treat as **5 object-level roles** (Solver / Verifier / Challenger / Builder + one fused ArchitectAI/JudgeAI) **OR** **6 distinct roles** (split Architect from Judge). ArchitectAI defaults to **6 distinct roles** because:
- ArchitectAI proposes (Constitution Art. V.1.2)
- JudgeAI vetoes (Constitution Art. V.1.3)
- Different reward functions: Architect = "新架构提案的下游收益"; Judge = "低误判 + 低漏判 + 长期稳定"

If user prefers 5 fused, the fused role's reward function must combine both signals. Plan v3.1 CO2.7.* has 6 atoms (one per role); fused interpretation drops to 5.

## § 7 v4 Scope = Phase 1 + Phase 2 (per Economic § 20)

```text
Phase 1: Local Ledger Economy   (ledger.jsonl + SQLite + Python predicates)
Phase 2: Internal Task Market   (YES/NO stake + verifier + challenger + DAG + bonus)
———————— v4 SCOPE ENDS HERE ————————
Phase 3: Permissioned Settlement (multi-org chaincode escrow)        ← v4.x
Phase 4: Rollup Settlement       (batch + economic_state_root + fraud proof)  ← v4.x
Phase 5: Public AGI Market       (public escrow + cross-domain rep + oracle)  ← v4.x
```

**Phase 1 v4 mapping**:
- ChainTape L0-L6 实装 (CO P1.0-P1.9)
- Q_t 9-field struct + reconstruction tests
- Anti-Oreo 3-layer file split
- Predicate registry with public/private/commit-reveal
- Tool registry with permission classification
- gix substrate (Path B, real git)

**Phase 2 v4 mapping**:
- TaskMarket pre-locked bounty
- EscrowVault
- ContributionLedger (work/verify/challenge/reuse 4 tx types)
- AttributionEngine (Contribution DAG)
- ChallengeCourt (challenge window + slash)
- SettlementEngine 3-layer (immediate/deferred/royalty)
- Verifier + Challenger market roles
- ReputationIndex (non-transferable)
- PriceIndex (broadcast only)

**Out of v4 scope** (Phase 3-5 = v4.x): permissioned chaincode (Hyperledger Fabric), public escrow, cross-domain reputation, oracle integration, rollup proofs, ZK predicates.

---

## § 8 Atom Count + Wall Clock (Honest)

Mapped from CO_MEGA_PLAN v3 outline + economic chapter additions:

| Phase | Scope | Atoms | Wall clock | External audit cost |
|---|---|---|---|---|
| **CO P0** | Foundation (TR + plan freeze + amendment) | 7 | 1 week | $50-100 (one dual audit on plan v3.1) |
| **CO P1** | Phase 1 — GitTape + Anti-Oreo + Predicate/Tool registries + L0-L6 | 50-65 | 8-10 weeks | $100-200 (1 mid-phase + 1 exit) |
| **CO P2** | Phase 2 — RSP economy (9 modules + 12 invariants) | 50-70 | 8-10 weeks | $100-200 |
| **CO P3** | MetaTape (ArchitectAI/JudgeAI runtime) — DEFERRED to v4.1 | (40+) | (4-6 weeks) | (deferred) |

**v4 scope total**: 107-142 atoms, 17-21 weeks (4-5 months), $250-500 external audit budget. **MVP option** (Phase 1 + RSP MVP-light, no Challenger market, no royalty): 80-100 atoms, 12-14 weeks, $150-300.

PPUT-CCL Phase C (the 30-day arc) **cannot survive** this refactor; per CO_MEGA_PLAN § 8 PREREG amendment v2 must declare PAUSE (OPT-1) or NEGATIVE (OPT-2) or MVP-pivot (OPT-3).

---

## § 9 What Already Exists vs What's New

**Reusable from current code** (~30% of v4 surface):
- `src/wal.rs`, `src/ledger.rs` → move to `bottom_white/ledger/` (mostly intact)
- `src/sdk/sandbox.rs` → move to `bottom_white/sandbox/`
- `src/sdk/tool.rs` (TuringTool trait) → formalize into `bottom_white/tools/registry.rs`
- `experiments/.../src/lean4_oracle.rs` → move to `top_white/predicates/acceptance/`
- `experiments/.../src/wall_clock.rs` → move to `top_white/budget/`
- `experiments/.../src/experiment_mode.rs` → keep in place (CCL ablation harness, used by Phase 2 evaluation)
- `src/prediction_market.rs` → refactor into `economy/task_market.rs` + `economy/price_index.rs`
- `genesis_payload.toml` → extend to genesis Trust Root with all white paper docs

**Net new modules** (~70% of v4 surface):
- `src/state/q_state.rs` (NEW)
- `src/transition/mod.rs` (NEW)
- `src/top_white/predicates/{registry,visibility,runner}.rs` (NEW)
- `src/top_white/signals/{boolean,statistical,price_broadcast}.rs` (NEW)
- `src/middle_black/{agent_protocol,role_self_select,librarian_board}.rs` (NEW)
- `src/bottom_white/cas/*` (NEW)
- `src/bottom_white/tape/{chain_tape,git_substrate}.rs` (NEW)
- `src/bottom_white/materializer/*` (NEW)
- `src/bottom_white/signal_index/*` (NEW)
- `src/economy/*` 9 RSP modules + 12 invariant tests (NEW)
- `experiments/.../agents/{solver,verifier,challenger,builder,architect_ai,judge_ai}.rs` (NEW; current monolithic agent code splits into 6)

**Modules being SPLIT** (per Anti-Oreo, mixed-layer modules forbidden):
- `src/bus.rs` → splits across `top_white/predicates/runner.rs` + `top_white/signals/price_broadcast.rs` + `bottom_white/ledger/wal.rs` + `economy/contribution_ledger.rs` + `middle_black/agent_protocol.rs`
- `src/kernel.rs` → splits across `state/q_state.rs` + `transition/mod.rs` + `economy/{task_market,settlement_engine}.rs`

This is **why** TFR v1 was 20% — it kept bus/kernel as monoliths. Anti-Oreo demands they be dissolved.

---

## § 10 Decision Request to User

Six decisions needed before CO Phase 0 starts atomization (CO_MEGA_PLAN_v3.1 codifies them):

| # | Decision | Options | ArchitectAI rec |
|---|---|---|---|
| **D1** | PREREG / PPUT-CCL fate | A: PAUSE (resume after CO P2) / B: NEGATIVE (declare arc failed) / C: MVP-pivot | **C (MVP-pivot)** — preserve some Phase C output; pragmatic |
| **D2** | Constitution Art. 0.5 white paper integration | A: full text in constitution / B: pointer + 6 公理 only | **B** — keep constitution.md compact |
| **D3** | TFR v1 disposition | A: deprecate but preserve / B: delete | **A** — preserve for history |
| **D4** | CO P3 (MetaTape) inclusion in v4 | A: include (~22 wk) / B: defer to v4.1 (~17 wk) | **B** — MetaTape is post-MVP |
| **D5** | RSP depth | A: full per Economic § 19 / B: MVP (no royalty + no Challenger market) / C: minimal (immediate-only) | **A** — 12 invariants demand full RSP; B falls short of Inv 5/8 |
| **D6** | External audit cadence | A: full (per phase + per STEP_B atom, $250-500) / B: reduced ($150-250) | **A** — given trust erosion this turn, audit is the antidote |

---

## § 11 Self-Audit on This Blueprint

What this blueprint commits to:
- **Single Q_t with 9 fields**, not "8 + economic side car"
- **Anti-Oreo file-level enforcement** (3 root dirs: top_white/middle_black/bottom_white) + economy as 4th
- **All 12 economic invariants → 1 dedicated conformance test each**
- **All 24 V + 4 E → conformance tests** (carry from CO_MEGA_PLAN v3)
- **Bidirectional trace matrix** with **production-invocation column** (E-01 class regression-prevention)
- **5 agent roles** (Solver/Verifier/Challenger/Builder/Architect/Judge) with **economic stake symmetry** (YES_E for Solver, NO_E for Challenger, reputation/bond for Verifier)
- **3-layer rewards** (immediate / deferred / royalty)
- **gix Path B real git substrate** (per Constitution Art. 0.4)

What this blueprint deliberately defers:
- Atom-level dependency graph (CO_MEGA_PLAN_v3.1 task)
- Per-atom file-by-file diff sketches (CO P1/P2 sprint design tasks)
- Constitutional Art. 0.5 final text (depends on D2)
- PREREG_AMENDMENT_v2 final text (depends on D1)
- Phase 3-5 chaincode + rollup design (out of v4 scope)
- ZK / commit-reveal predicate cryptographic detail (CO P1.5 sub-task)

What this blueprint is **honest** about being uncertain on:
- gix multi-parent commit + concurrent runtime_repo write performance (CO P1.3 spike will validate)
- AttributionEngine determinism — Contribution DAG computation must be deterministic; subjectivity risks Inv 8 violation (CO P2.4 design challenge)
- Predicate visibility leak channels (private predicate must NOT leak via error messages, retry counts, log lines — CO P1.5 air-gap testing)
- 17-21 week timeline absorbing user attention without abandonment — Risk #2 of CO_MEGA_PLAN; mitigation = weekly progress reports + checkpoint demos at every CO P-gate

---

## § 12 What Comes Next

Immediate steps after user reviews this blueprint:

1. **User answers D1-D6** (could be "all ArchitectAI-recommended" or per-item override)
2. **CO_MEGA_PLAN_v3.1** atomizes this blueprint into 107-142 atoms with file paths, dependencies, exit criteria. (`handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md`, drafted in same commit as this blueprint)
3. **Constitution amendment Art. 0.5** drafted per D2 choice (cp workflow per R-018 V.3 entry)
4. **PREREG_AMENDMENT_v2** drafted per D1 choice
5. **External dual audit** on blueprint + v3.1 (Codex + Gemini) — PASS/PASS gate to CO P0 entry
6. **CO P0** kicks off (1 week foundation work)
7. **CO P1.0** entry begins after CO P0 exit audit

This blueprint **does not invent new architectural concepts** — every concept is from constitution, white paper architecture chapter, or white paper economic chapter. The blueprint's only contribution is **mapping every concept to a file path and a conformance test**, plus an honest atom + wall-clock count.

— ArchitectAI, 2026-04-26
