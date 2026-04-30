# TuringOS v4 — Canonical 9-Phase Roadmap (P0–P9)

**Authority**: architect directive 2026-04-29, archived at `handover/directives/2026-04-29_9_phase_roadmap.md`. User `gretjia` authorized full P0–P9 ordering on 2026-04-29 in chat (`我同意全部按新的phases划分和次序执行`). No Layer-1 axiom violation; no constitution.md edit.

**Amended 2026-04-29 (post-audit)**: external auditor's CF-1, CF-2, CF-6 + dependency-graph clarification incorporated per `handover/audits/2026-04-29_external_audit.md` and user authorization on 2026-04-29 ("授权 6 个 items 全部执行"). Specific amendments: P0 split into P0 + P0.R (RootBox Ratification Ceremony); P1 Build adds `rejection_evidence.rs`; P1 Exit 6+9 wording updated to L4-accepted-only / L4.E-rejection-evidence split; P3 Forbidden adds per-node-auto-injection / ghost-liquidity-without-treasury-debit; § 12 dependency graph added.

**Status**: canonical. Every TB charter (current + future) MUST declare `phase_id ∈ {P0..P9}` + `kill_criteria_tested` + `roadmap_exit_criteria_addressed`. Sessions inheriting work read this doc + `MEMORY.md` first.

**Supersedes (as the operational ordering)**: PPUT-CCL Phase A–E roadmap as the *primary* sequencing axis. PPUT-CCL persists as the **P6 Epistemic Lab** product line (one of four; see § 5).

**One-line manifesto**:
> TuringOS 不是热力学第二定律本身，而是 AGI 时代对抗信息熵增的一套工程纪律：通过谓词、ChainTape、选择性屏蔽、RSP 经济和 Go Meta，把黑盒智能的高熵生成压缩为可验证、可结算、可回滚、可继承的低熵状态转移。

---

## 1. Three immovable tracks

```text
Architecture Track:  Anti-Oreo
                     Agent ≠ direct state writer
                     rtool + wtool + PredicateRunner + sandbox + ChainTape

Ledger Track:        ChainTape (5 layers)
                     constitution_root → predicate_registry → CAS
                                       → append-only ledger → materialized read view

Economy Track:       RSP (Reward Settlement Protocol)
                     Information is Free
                     Only Investment Costs Money
                     1 Coin = 1 YES + 1 NO
                     on_init unique mint
```

Every phase exit criterion ultimately discharges to one of these three tracks.

## 2. Per-phase 6-field contract (mandatory)

```text
Goal              本阶段证明什么
Build             本阶段开发什么
Transactions      本阶段新增哪些 tx 类型
Predicates        本阶段新增哪些谓词
Exit Criteria     怎样才算通过 (NUMBERED list of falsifiable assertions)
Forbidden         本阶段禁止做什么
```

A phase is "green" iff every Exit Criterion has at least one passing test or evidence artifact. **A phase is "DEAD" the moment any Kill Criterion (§ 4) becomes provable** — work stops, OBS_<phase>_FAILED.md is written, charter must change before retry.

## 3. The 9 phases — P0 to P9

### P0 Constitution-to-Code

**Goal**: 把宪法从思想文本变成可执行不变量。
**Build**:
```text
/trust_root/{constitution.md, constitution.hash, root_sudo.sig,
             invariant_registry.json, forbidden_actions.json}
/core_whitebox/predicates/{no_direct_agent_write, predicate_must_be_deterministic,
                            read_write_separation, monetary_invariant,
                            no_private_predicate_leak, no_context_cross_contamination,
                            no_post_init_mint, constitution_compliant}
/tests/constitution/test_*.py
```
**Transactions**: `genesis_tx`, `constitution_anchor_tx`, `predicate_register_tx`, `tool_register_tx`.
**Predicates**: enumerated in Build above.
**Exit Criteria**:
1. constitution.md 删除 → 系统不能启动。
2. constitution.md 修改 + 无 root_sudo.sig → 不能启动。
3. Agent 调用 write API 必须失败。
4. 只有 wtool 可以改变 state_root。
5. on_init 之后任何 mint tx 都被拒绝。
6. 所有 predicate 必须可 hash、可版本化、可测试。

**Forbidden**: 公链 / 真实奖金 / LLM 最终裁决 / 自然语言系统边界。

**Current state (2026-04-29)**: 🟢 engineering gate mostly done. constitution.md + `genesis_payload.toml [trust_root]` 24-entry manifest + `boot::verify_trust_root` + `boot::verify_constitution_root` + R-018 sudo rule + 5/5 boot tests green. **P0.R ceremony incomplete** — see sub-phase below. Other remaining gaps: explicit `forbidden_actions.json`; `invariant_registry.json` as discrete file; `no_context_cross_contamination` predicate test (currently only loosely enforced via FC1-N12 oracle scope).

### P0.R RootBox Ratification Ceremony (sub-phase, post-audit 2026-04-29 CF-6)

**Goal**: replace `[constitution_root]` placeholder strings with real human-root attestation and document the sudo rotation/revocation flow. Until P0.R is green, P0's overall status is "engineering gate done; RootBox ceremony pending" — not "fully green".

**Build**:
- Replace `creator_signature = "PENDING_USER_PGP_SSH_SIGNATURE_v4_FIRST_ENACTMENT"` in `genesis_payload.toml [constitution_root]` with a real PGP/SSH signature over the canonical constitution.md hash.
- Replace `boot_attestation_hash = "PENDING_v4_X_SELF_REFERENTIAL_COMPUTE"` with the recomputed self-referential hash defined in the constitution_root computation note.
- Document the sudo rotation/revocation flow in a new `handover/architect-insights/ROOTBOX_CEREMONY_v0.md`.
- Add a boot-time fail-closed check: if either `creator_signature` or `boot_attestation_hash` is a `PENDING_*` placeholder AND the `TURINGOS_PRODUCTION` env is set, refuse to boot.

**Transactions**: none (this is constitution-layer ceremony, not state machine).

**Predicates**: `creator_signature_valid`, `boot_attestation_recomputable`, `production_no_pending_placeholders`.

**Exit Criteria**:
1. `creator_signature` is a real human-root signature, not a `PENDING_*` placeholder.
2. `boot_attestation_hash` is recomputable per the documented formula.
3. `root_sudo.sig` rotation/revocation flow is documented.
4. `constitution.md` modification without valid signature fail-closed.
5. No Agent — including ArchitectAI — can author or amend `constitution.md`.

**Forbidden**: Agent-signed constitution root; auto-generated sudo; silent placeholder acceptance in production-mode boot.

**Current state (2026-04-29)**: ❌ pending — both placeholders present in `genesis_payload.toml`. P0.R is **not blocking** P1/P3 development today, but P0 status MUST be reported as "P0 engineering green; P0.R pending" when reporting overall phase status.

### P1 GitTape Kernel

**Goal**: 证明最小闭环 `Agent → Predicate → Commit/Reject → Ledger Append`. **Two ledgers, not one**: accepted transitions go to L4; rejected submissions go to L4.E (separate hash chain, separate identifiers, no `logical_t`, no `state_root` advance).
**Build**:
```text
/ledger/{chaintape.jsonl, cas_objects/, state_roots/}
/core_whitebox/engine/{rtool, wtool, predicate_runner, materializer, signal_router}
/state/{current_state.db, task_index.db, view_index.db}
src/bottom_white/ledger/transition_ledger.rs   (L4: accepted-only)
src/bottom_white/ledger/rejection_evidence.rs  (L4.E: rejected-only; new, post-audit 2026-04-29 CF-1)
```
Reference tx schema:
```json
{"tx_id":"tx:sha256:...","tx_type":"work","parent_state_root":"sha256:...",
 "agent_id":"agent:solver:001","task_id":"task:demo_patch",
 "read_set":["cid:spec"],"write_set":["/workspace/demo.py"],
 "artifact_cid":"cid:patch","predicate_results":[],
 "stake":null,"signature":"agent_signature","status":"pending"}
```
**Transactions**: `work_tx`, `accept_tx`, `reject_tx`, `revert_tx`, `view_materialize_tx`.
**Predicates**: `schema_valid`, `signature_valid`, `parent_state_root_valid`, `read_set_authorized`, `write_set_authorized`, `no_forbidden_path_write`, `patch_applies_cleanly`, `unit_tests_pass`, `state_root_recomputable`.
**Exit Criteria**:
1. Agent 只能通过 rtool 读取局部上下文。
2. Agent 不能读取完整 ledger。
3. Agent 不能直接修改文件系统状态。
4. wtool 是唯一合法写入口。
5. tx 通过谓词后，accepted transition 进入 L4，`logical_t` 单调推进，`state_root` / `ledger_root` 改变。
6. tx 被拒绝后，`state_root` 不变；accepted transition ledger (L4) 不增加 `logical_t`；rejection evidence ledger (L4.E) 增加 `submit_id`-scoped 记录。
7. L4 ledger 删除任意中间行后，hash chain 校验失败；L4.E 同样验证：删除任意 rejection 记录后，rejection-chain 校验失败。
8. state.db 删除后，可以从 L4 (accepted only) 重建 — L4.E 不参与 `state_root` 重建（rejection 不应改变状态）。
9. L4.E 的 raw diagnostic 不进入其他 Agent 的 materialized read view；只允许 `aggregate counter` 或 `public_summary` 作为派生信号出现。

**Forbidden**: 经济结算 / 多组织共识 / 上链 / Go Meta 自动改规则。
**Current state (2026-04-30, post-TB-2 ship)**: 🟢 runtime closure GREEN. Exits 5/6/7/8/9 all discharged.
- TB-1 primitives + TB-2 runtime spine. Five-atom merge `d9df271..a82f73e` on main; 16/16 acceptance battery PASS through `Sequencer::submit` (3 in-crate unit U1-U3 + 13 integration I1-I13). Phase-1c dual audit cleared (Gemini PASS 5/5; Codex CHALLENGE 4/5 → r1 remediation `abf3581` → r2 narrowed strict-CHALLENGE / substance-PASS).
- Exit 5 ✅ accepted WorkTx via `Sequencer::submit` advances `state_root_t` (interim `WORKTX_ACCEPT_DOMAIN_V1` hash; real patch semantics still P5) + `ledger_root_t` (canonical `transition_ledger`) + accepted `logical_t` by 1 (tests I9-I11).
- Exit 6 ✅ rejected WorkTx (predicate-fail / stale-parent / stakeless / no-escrow / monetary-invariant-violation) → `state_root_t` unchanged + 1 L4.E row keyed by `submit_id` from the SubmissionEnvelope (tests I3-I7).
- Exit 7 ✅ L4 + L4.E hash-chain tamper detection (TB-1 Tier-A).
- Exit 8 ✅ state.db reconstruction from canonical L4 alone reaches the same `state_root_t` as the live sequencer; L4.E records ignored (test I13 via `replay_full_transition`).
- Exit 9 ✅ raw diagnostics absent from materialized read view at the runtime path (test I8 — re-confirms TB-1 P0-3 serde shield through both `RejectedSubmissionRecord` JSON and `PublicRejectionView`).
- **Production claim**: "TuringOS runtime kernel honors the L4 / L4.E split."
- Note: `economy::ledger::AcceptedLedger` retains its TB-1 RSP-0 primitive role (in-memory `Vec`); the production accepted spine is `bottom_white::ledger::transition_ledger` + `LedgerWriter` per `Sequencer::apply_one`. Single-spine ChainTape preserved.

### P2 Agent Runtime

**Goal**: 接入真实 Agent，但仍不引入复杂经济。证明黑盒群体可以大规模生成，但系统边界仍由白盒工具与谓词决定。
**Build**: `/agents/{solver, verifier, challenger, planner}_agent.py` + `/runtime/{agent_scheduler, sandbox_runner, context_builder, tool_permission}.py`.
**Transactions**: `plan_tx`, `work_tx`, `verify_tx`, `challenge_draft_tx` (no economic effect yet), `review_tx`.
**Exit Criteria**:
1. 同一任务可以并发派发给多个 Solver。
2. 不同 Solver 拿到不同 read view 或不同随机种子。
3. Solver 不能看到其他 Solver 的中间思考。
4. Verifier 不能修改 Solver 的 artifact，只能提交 verify_tx。
5. Challenger 可以提交反例，但不能直接回滚状态。
6. 所有 Agent 输出都必须进入 CAS，ledger 只记录 CID。

**Forbidden**: Agent 自决任务完成 / Agent 自写奖金分配 / 完整失败日志广播。

**Current state**: 🟨 partial. Swarm = Solver-only; Verifier/Challenger/Planner roles not separated. CAS-on-emit is partial (gp_payload persisted; intermediate proposals not always content-addressed).

### P3 RSP Economy Core (most critical phase per directive)

**Goal**: 证明 TuringOS 经济不是发币叙事，而是状态转移的问责协议。
**Build**:
```text
/economy/{monetary_invariant, escrow_vault, balances, stake_manager,
          task_market, contribution_dag, settlement_engine,
          reputation_index, challenge_court}
```
Economic state:
```json
{"economic_state_t":{"balances_root":"sha256:...","escrows_root":"sha256:...",
 "stakes_root":"sha256:...","claims_root":"sha256:...",
 "reputation_root":"sha256:...","royalty_graph_root":"sha256:..."}}
```
**Transactions**: `on_init_tx`, `task_open_tx`, `escrow_lock_tx`, `yes_stake_tx`, `no_stake_tx`, `work_tx`, `verify_tx`, `challenge_tx`, `provisional_accept_tx`, `challenge_resolve_tx`, `settlement_tx`, `slash_tx`, `reputation_update_tx`.
**Predicates**: `read_is_free`, `stake_required_for_write`, `escrow_sufficient`, `no_post_init_mint`, `coin_conservation_valid`, `yes_no_split_valid`, `payout_sum_valid`, `challenge_window_closed`, `no_valid_challenge`, `contribution_dag_valid`, `settlement_rule_hash_valid`.
**State machine**:
```text
OPEN → SUBMITTED → VERIFIED → PROVISIONAL_ACCEPTED → CHALLENGE_WINDOW → FINALIZED → PAID
fail:    SUBMITTED → REJECTED → STAKE_SLASHED
chal-OK: PROVISIONAL_ACCEPTED → CHALLENGED → REVERTED/COMPENSATED → SLASHED → CHALLENGER_REWARDED
```
**Reward formula**:
```text
reward_i = Finalize(
    Escrow(task) × Accept(tx_i) × Attribution(tx_i, ContributionDAG)
    × Survival(challenge_window) × Utility(post_acceptance_metrics) × Constitution(Q_t)
)
```
**Exit Criteria** (12):
1. on_init 初始化后，总 Coin 不再增加。
2. rtool 调用不扣核心 Coin。
3. work_tx 必须锁定 YES stake。
4. challenge_tx 必须锁定 NO stake。
5. 没有 escrow 的任务不能进入正式市场。
6. 通过谓词只产生 provisional_accept，不立即全额付款。
7. 挑战期结束且无有效挑战后才能 settlement。
8. settlement_tx 的 payout 总和不能超过 escrow。
9. 失败 Solver 被 slash。
10. 成功 Challenger 获得 challenge reward。
11. Agent 自称"我贡献 90%"对结算无效。
12. Contribution DAG 能解释每一笔奖金来源。

**Forbidden**: 按 token 数发钱 / 按运行时间发钱 / 按 Agent 自评发钱 / post-init mint / 未过挑战期全额付款 / verifier 无责任盖章 / **per-node automatic liquidity injection**（任务创建时自动注入 YES + NO 做市，等同 post-init mint 化身；post-audit 2026-04-29 CF-2）/ **ghost liquidity without explicit treasury debit**（任何在 task market / risk market / AMM 中出现的流动性必须能在 `economic_state_t` 中追溯到 `balances_t` debit / sponsor escrow / LP stake；不能凭空出现）。
**Current state (2026-04-30, post-TB-4 ship)**: 🟢 RSP-0 + RSP-1 (formal tx surface) + **RSP-2 (admission spine)** GREEN at the runtime spine. Exits 3/4/5 discharged through formal-tx surface; RSP-2 admits VerifyTx (verifier bond locked into stakes_t) + ChallengeTx (challenger NO stake locked into challenge_cases_t with opened_at_round anchor + target_work_tx backref). RSP-3 through RSP-7 sequenced for future TBs.
- RSP-0 green (TB-1): `monetary_invariant.rs` enforces `assert_no_post_init_mint` + `assert_total_ctf_conserved` + `assert_read_is_free`. **Holding count migrated 6 → 5 in TB-3** (drop `task_markets_t.bounty` term — money has migrated to `escrows_t.amount`; total_escrow becomes derived cache).
- RSP-1 formal-tx surface green (TB-3): production WorkTx admission is structural via formal surface — `task_markets_t[work.task_id].total_escrow > 0` (populated only by accepted EscrowLockTx) AND `balances_t[work.agent_id] >= work.stake` (solver solvency). The TB-2 P0-B option (a) bridge at `src/state/sequencer.rs:197-215` is **DELETED** (Atom 6 commit `fa85350`); enforced as a CI invariant by `tests/tb_3_bridge_deletion_invariant.rs`. Tests U9 + I23-I28 prove the new admission.
- **2 new TypedTx variants** (TB-3): `TaskOpenTx` (metadata-only; sponsor opens task market entry) + `EscrowLockTx` (sole RSP-1 bounty funding path; atomic balance → escrow transfer). Per WP § 14.1 + § 18 Inv 5, **WorkTx.stake stays inline** — NO YesStakeTx variant; ROADMAP `yes_stake_tx` interpreted as semantic role of `WorkTx.stake` field. Memory `feedback_wp_vs_roadmap_reconciliation` codifies the WP-canonical reconciliation rule.
- **Lock-on-accept** (TB-3 charter § 3.4 + WP § 18 Inv 5): accepted WorkTx debits `balances_t[agent_id]` by `work.stake` AND inserts `stakes_t[work.tx_id] = StakeEntry { amount, staker, task_id }`. Rejected WorkTx leaves `economic_state_t` bit-identical (L4.E never mutates economic state per user verdict #14). Slashing deferred to RSP-2/3 explicit accepted ChallengeResolveTx (no L4.E-driven slash).
- **§ 3 P3 Forbidden CF-2 structurally enforced** (TB-3): "no ghost liquidity without explicit treasury debit" — every `task_markets_t.total_escrow` growth is a single accepted EscrowLockTx with paired balances_t debit. "No per-node automatic liquidity injection" — no code path mutates total_escrow outside the EscrowLock dispatch arm.
- **Cache=truth invariant** (TB-3 charter § 3.2): `task_market.total_escrow == Σ escrows_t[e].amount where e.task_id == task_id` enforced by `assert_task_market_total_escrow_matches_locks` at every accepted EscrowLock + by integration tests I22 + I29 + I30. `monetary_invariant.total_supply_micro` does NOT count `total_escrow` (else double-mint on every lock).
- TB-3 added: 2 new TypedTx variants, 3 new TransitionError variants (TaskAlreadyOpen, TaskNotOpen, InsufficientBalance), 1 new L4ERejectionClass variant (InsufficientBalance — own class for P4 Information Loom discriminator richness).
- **RSP-2 admission spine green (TB-4)**: production VerifyTx + ChallengeTx admission filled (was NotYetImplemented stubs through TB-3). VerifyTx debits `balances_t[verifier_agent]` by `verify.bond` and credits `stakes_t[verify.tx_id]` (verifier puts skin in the game; charter § 3.4 + § 3.10 signal-not-judge — verdict rides L4 only, never mutates Q_t). ChallengeTx debits `balances_t[challenger_agent]` by `challenge.stake` and credits `challenge_cases_t[challenge.tx_id]` with `opened_at_round = q.q_t.current_round` (challenge-window structural anchor; closure deferred RSP-3 per directive § 5.1) + `target_work_tx` backref (replay-deterministic; multi-challenger representable). Per WP § 14.1 + § 18 Inv 5, **VerifyTx.bond + ChallengeTx.stake stay inline** — NO VerifierBondTx / NoStakeTx variants (CI-enforced by I44 anti-drift scanner per directive § 5.1). Memory `feedback_wp_vs_roadmap_reconciliation` re-applied: ROADMAP `verifier bond` ↔ `VerifyTx.bond` semantic role; ROADMAP `no_stake_tx` ↔ `ChallengeTx.stake` semantic role.
- **TB-4 added**: 3 new TransitionError variants (BondInsufficient, TargetWorkInactive, EmptyCounterexample); existing TargetWorkTxNotFound + TargetWorkTxNotVerifiable repurposed as RESERVED (3-class error taxonomy per directive Q3); ChallengeCase entry-shape +target_work_tx (additive serde-default per charter § 3.3); VerifyTx + ChallengeTx schema bump (+parent_state_root field#2 per charter § 4.1; goldens rotated; pre-TB-4 had zero accepted L4 rows of these kinds). 9-sub-field EconomicState invariant + 5-holding CTF invariant preserved.
- **§ 3 P3 Forbidden "verifier 无责任盖章" structurally discharged** (TB-4): verifier cannot stamp Confirm/Doubt without bonded exposure. Slash deferred RSP-3 alongside ChallengeResolveTx.
- RSP-3 (challenge window closure + slash + provisional/final) through RSP-7 (settlement_engine + contribution_dag + reputation_update_tx + price index) remain RED, sequenced into TB-5 onwards per § 6.

### P4 Information Loom (Signal Layer)

**Goal**: 实现宪法中的量化、广播、屏蔽。
**Build**:
```text
/core_whitebox/statistics/{price_index, reputation_index, failure_clusterer,
                            reuse_counter, risk_price, exploration_scheduler}
/core_whitebox/engine/{signal_router, broadcast_policy, shielding_policy}
```
**Signal types**: boolean (predicate pass/fail, challenge success/fail, settlement valid/invalid) + statistical (reputation, reuse_count, failure_rate, challenge_rate, bounty_price, YES/NO risk price, downstream impact).
**Transactions**: `signal_update_tx`, `price_update_tx`, `reputation_update_tx`, `error_cluster_tx`, `broadcast_rule_tx`, `shield_rule_tx`.
**Predicates**: `no_raw_error_broadcast`, `private_predicate_hidden`, `broadcast_rule_has_evidence`, `reputation_non_transferable`, `price_signal_not_settlement_truth`.
**Exit Criteria**:
1. 单个 Agent 失败，只给该 Agent 发送局部错误。
2. 多个 Agent 同类失败，系统聚类为典型错误。
3. 广播的是抽象规则，不是原始失败日志。
4. reputation 只能由 accepted / rejected / challenge / reuse 事件更新。
5. price signal 可以影响任务优先级，但不能覆盖 predicate。
6. YES/NO 风险价格可以提示风险，但不能决定事实真假。
7. Goodhart-sensitive 的评分器不能被 Solver 读取。

**Forbidden**: 全量广播 ledger / 全量广播 rejected logs / 公开隐藏 benchmark / 价格信号替代宪法谓词。
**Current state**: 🟨 partial. fc_trace + librarian + signal routing exist; no proper error clusterer / Goodhart shield distinct from `prompt_guard::assert_no_metric_leak`.

### P5 MetaTape

**Goal**: 让系统开始"自己修自己的脚手架"。Go Meta = 架构升级也进入状态转移协议。
**Build**: `/meta/{architect_agent, judge_agent, meta_proposal_builder, meta_sandbox, canary_deployer, rollback_planner, constitution_checker}`.
**Transactions**: `meta_observation_tx`, `meta_proposal_tx`, `predicate_patch_tx`, `tool_patch_tx`, `judge_veto_tx`, `meta_canary_tx`, `meta_merge_tx`, `meta_revert_tx`, `human_sudo_tx`.
**Flow**:
```text
ArchitectAI reads {failure_clusters, rejected_tx, challenge_cases}
  → meta_proposal (new predicate / tool)
  → meta_sandbox  (offline)
  → JudgeAI constitution_check (only output: PASS | VETO)
  → canary_deploy
  → canary_period (regression + challenge collection)
  → no_anomaly  → predicate_registry_root_t (merged)
     anomaly    → meta_revert
```
**Predicates**: `meta_patch_schema_valid`, `constitution_compliant`, `read_write_separation_preserved`, `monetary_invariant_preserved`, `determinism_preserved`, `rollback_plan_exists`, `canary_required`, `human_sudo_required_for_constitution_change`.
**Exit Criteria**:
1. ArchitectAI 不能直接修改 predicate registry。
2. JudgeAI 只能 veto，不做主观奖励裁判。
3. 合宪只是必要条件，不是充分条件。
4. 新 predicate 必须先 sandbox，再 canary，再 merge。
5. 所有 meta 更新都可回滚。
6. 修改 constitution.md 必须 Human Sudo。

**Forbidden**: ArchitectAI 直接写生产规则 / JudgeAI "看起来不错"批准 / 没有 rollback plan 的 meta merge / Go Meta 修改 on_init 铸币法。
**Current state**: ❌ not started. **Important**: the "5-step compile loop" Step 4 (Capability Compilation) lives here, NOT in P6 evaluator instrumentation. Per directive, P5 is sequenced AFTER P3 RSP and P4 Information Loom.

### P6 Permissioned ChainTape / Epistemic Lab

(The directive uses two cuts; this canonical merges them into a "P6 = multi-org coordination + automated epistemic lab" phase. Code Foundry single-repo work — directive § 5 — also lands here as one of the four product lines, see § 5 below.)

**Goal**: 多组织 / 多领域协作；自动认识论账本（数学+程序合成 first，物理仿真 second）。
**Build**:
```text
/multi_org/{org_identity, endorsement_policy, multi_sig_settlement, channel_policy, audit_peer}
/epistemic_lab/{hypothesis_ledger, evidence_oracle, replication_market}
```
**Transactions**: org_register_tx, endorsement_policy_tx, multi_org_task_open_tx, multi_sig_verify_tx, multi_sig_settlement_tx, audit_challenge_tx, hypothesis_tx, simulation_tx, oracle_evidence_tx, replication_tx, falsification_tx.
**Exit Criteria**:
1. 任务 sponsor 组织、执行组织、验证组织可以分离。
2. settlement 需要满足 endorsement policy。
3. 单个组织不能私自改结算结果。
4. 跨组织任务仍然保持 Agent read view 隔离。
5. 高风险任务可要求审计组织背书。
6. (epistemic) 假说必须有 hypothesis_id；evidence 必须有 source provenance。
7. (epistemic) 失败路径进入 falsification ledger，但证伪证据保留在 ChainTape；后续 Agent 不重复浪费算力。

**Forbidden**: LLM 推理上链 / 完整上下文上链 / 隐藏测试公开给所有组织 / 链上共识替代现实事实验证 / Agent 自称"我发现真理"。
**Current state**: 🟢 actively running on the **Epistemic Lab subset only**. MiniF2F evaluator IS the P6 Epistemic Lab oracle (Lean = formal oracle). PPUT-CCL Phase A ✅ Phase B ✅ Phase C 🛑 frozen. **Multi-org subset not started**.

### P7 Public Settlement

**Goal**: 把 TuringOS 的经济结算锚定到开放网络，但仍不把计算搬上链。
**Build**: `/public_settlement/{state_root_anchor, settlement_batcher, fraud_proof_interface, validity_proof_adapter, bridge_accounting, public_reputation_root}`.
**Transactions**: `state_root_checkpoint_tx`, `settlement_batch_tx`, `public_escrow_tx`, `fraud_proof_tx`, `validity_proof_tx`, `bridge_deposit_tx`, `bridge_withdraw_tx`.
**Exit Criteria**:
1. 链上只看到 state root / settlement root / proof，不看到完整 Agent 推理。
2. settlement batch 可被挑战。
3. formal predicates 可使用 validity proof。
4. 非形式化任务仍然依赖 challenge window。
5. 公链结算不改变 TuringOS 宪法根。

**Forbidden**: 为发币而上链 / LLM 推理日志上链 / 私有谓词上链公开 / 公链最终性替代 TuringOS challenge finality。
**Current state**: ❌ not started.

### P8 Autonomous Agent Economy

**Goal**: Agent 自发任务 + Builder royalty + ArchitectAI/JudgeAI/Human-RootBox 三权分立。
**Build**: `/autonomous_market/{task_discovery, auto_bounty_allocator, tool_market, predicate_market, royalty_graph, long_term_impact_index, agent_specialization}`.
**Transactions**: `auto_task_proposal_tx`, `auto_bounty_allocate_tx`, `tool_publish_tx`, `predicate_publish_tx`, `reuse_royalty_tx`, `impact_bonus_tx`, `agent_specialization_tx`.
**Exit Criteria**:
1. Agent 可以基于价格信号选择任务。
2. Builder Agent 可因工具复用获得 royalty。
3. Verifier / Challenger 市场能降低系统 bug 存活率。
4. ArchitectAI 能从失败日志中提出新规则，但仍被 JudgeAI 和 Human Sudo 约束。
5. 人类不批准普通任务，只维护 constitution root。

**Forbidden**: 完全取消 Human Sudo / 经济价格覆盖宪法不变量 / 长期 royalty 变成不可挑战的永久租金 / Agent 自我复制绕过身份与信誉系统。
**Current state**: ❌ not started.

### P9 — reserved (full-release MetaTape under autonomous economy)

Per directive's "Phase 0–8" cut this is folded into P5; per "P0–P9" cut it is reserved for the autonomous-economy variant of MetaTape (where ArchitectAI proposals are themselves auctioned in P8's predicate_market). Activation requires P5 + P8 simultaneously green.

## 4. Kill criteria (mandatory failure-stop)

> 路线图不应只写成功路径，还要写失败即停止的条件。否则系统会被愿景绑架。

### P1 kill
```text
Agent 可以绕过 wtool 修改状态 → STOP
rejected tx 会改变 state_root → STOP
ledger 不能重建 state.db → STOP
失败日志污染其他 Agent read view → STOP
```

### P3 kill
```text
post-init 可以增发 Coin → STOP
Agent 可以无 stake 写入 → STOP
settlement 超过 escrow → STOP
Agent 自报贡献能影响奖金 → STOP
通过谓词后立即全额付款 → STOP
```

### P5 kill
```text
ArchitectAI 能直接修改 predicate registry → STOP
JudgeAI 能主观批准架构变更 → STOP
Meta 更新没有 rollback plan → STOP
Go Meta 能修改 on_init 规则 → STOP
```

When a kill criterion goes live, work stops, OBS_<phase>_FAILED.md is written, charter must change before retry. **No "kill-with-OBS"; the kill criteria are not negotiable enforcement gates.**

## 5. Four product lines (cross-cutting)

The 9 phases are **infrastructure**. The four product lines are how that infrastructure is exercised and demonstrated.

```text
Code Foundry Roadmap
  v0 single-repo patch market   →  P5 anchor
  v1 multi-Agent code-fix + CI predicates
  v2 multi-repo dependency graph
  v3 formal verification + canary deploy
  v4 critical infrastructure with human sudo gate
  v5 zero-downtime autonomous refactoring network

Epistemic Lab Roadmap
  v0 ATP + program synthesis (← MiniF2F PPUT-CCL is here)
  v1 仿真实验 ledger
  v2 multi-oracle evidence
  v3 replication market
  v4 robotic lab integration
  v5 cross-lab scientific settlement network

Agent Economy Roadmap
  v0 internal bounty market (← TB-1 RSP-0 demo target)
  v1 Solver / Verifier / Challenger
  v2 YES/NO risk market
  v3 Contribution DAG settlement
  v4 reuse royalty
  v5 cross-org permissioned settlement
  v6 public settlement network

MetaTape Roadmap
  v0 manual predicate registry (current state)
  v1 ArchitectAI proposes predicates
  v2 JudgeAI vetoes unconstitutional proposals
  v3 staging registry + canary
  v4 automatic tool proposal
  v5 architecture migration proposals
  v6 RootBox-gated constitution updates
```

Per-product-line core indicators (selection):
```text
Code Foundry:    accepted patch rate / regression rate / mean revert time / reuse royalty distribution
Epistemic Lab:   hypothesis acceptance rate / replication rate / falsification latency / cost-per-validated-hypothesis
Agent Economy:   escrow coverage / payout conservation / slash rate / verifier accuracy / sybil resistance
MetaTape:        predicate proposal survival rate / false-accept_reject rate / rollback success / human sudo frequency
```

## 6. RSP-N micro-versions (P3 internal sequencing)

Because P3 is the largest single phase, it ships in 8 micro-versions:

```text
RSP-0  on_init + balances + monetary_invariant
       Exit: total Coin invariant after on_init / rtool + think 不扣 Coin / mint tx 被拒绝
RSP-1  task escrow + work_tx + yes_stake
       Exit: 任务必须先 escrow / Solver lock YES / 无 stake 不能 work_tx
RSP-2  verifier + challenge_tx + no_stake
       Exit: 独立验证 / Challenger 押 NO / 挑战失败 slash Challenger / 挑战成功 slash Solver+bad Verifier
RSP-3  challenge window + slash + provisional reward
       Exit: 通过谓词只 provisional / 挑战期结束才 final
RSP-4  Contribution DAG + settlement_tx
       Exit: 奖金来自 DAG / Agent 自报无效 / payout_sum ≤ escrow_pool
RSP-5  deferred impact bonus + reuse royalty
       Exit: 复用工具 royalty / cap+decay+bug-clawback
RSP-6  price index + risk market
RSP-7  public settlement adapter (= P7 entry)
```

## 7. First 90-day build plan

```text
Day 1-15:   constitution predicates / genesis state / ledger hash chain / CAS put_get / rtool_wtool skeleton
            → discharges P0 + P1 partial
Day 16-30:  local GitTape demo / unit-test predicate / state_root recomputation / rejected-tx isolation
            → discharges P1 Exit 7,8,9 + P1 kill 1,2,3,4
Day 31-45:  Solver + Verifier + Challenger agents / sandbox runner / context isolation / failure routing
            → discharges P2 + P4 partial
Day 46-60:  on_init economy / escrow vault / YES_NO stake / challenge window
            → discharges P3 RSP-0..RSP-3 + P3 kill 1,2,3,5
Day 61-75:  Contribution DAG / settlement_tx / slash + reward / reputation index
            → discharges P3 RSP-4 Exit 8,9,10,11,12 + P3 kill 4
Day 76-90:  error clusterer / price signal / demo task market / red-team attacks / public demo
            → discharges P4 + P3 RSP-6 + final Exit cross-checks
```

> 第一版 demo 的目标只有一个：
> **证明一个 Agent 不能直接改世界，但可以通过可验证状态转移获得奖金；另一个 Agent 可以通过挑战错误获得奖金；整个过程不增发、不污染、不绕过谓词。**

## 8. Final ordering principle

```text
1. 先有宪法根                    P0
2. 再有 GitTape                 P1
3. 再有 predicate gate           (P0 ∩ P1)
4. 再有 RSP 经济                 P3
5. 再有 Information Loom         P4
6. 再有单领域 Code Foundry       P5 / Code Foundry v0
7. 再有 Epistemic Lab            P6 / Epistemic Lab v0..v2
8. 再有多组织 ChainTape          P6 multi-org
9. 再有开放 Public Settlement    P7
10. 最后才谈行星级自治系统       P8
```

> 不要反过来。如果反过来，一开始就做开放市场、公链、AGI 科研、自治公司，你会得到一个不可控的黑盒赌场。

## 9. TB methodology integration

Every TB charter (current TB-1 included after re-charter) MUST include in commit message body and TB_LOG.tsv row:

```text
phase_id:                            P0..P9
roadmap_exit_criteria_addressed:     subset of the phase's numbered Exit list
kill_criteria_tested:                subset of P1/P3/P5 kill clauses this TB tries to keep green
```

A TB whose acceptance test passing flips a kill criterion from RED to GREEN is preferred over a TB that only adds Exit-criterion evidence. A TB cannot ship if any tested kill criterion went RED during its run.

## 10. Manifesto vs. Roadmap layer separation

Per directive § 9, the previous "Future Vision" prose stays in the whitepaper but moves to **Appendix A — Information Loom Manifesto**, and the path to it is exactly this 9-phase roadmap.

```text
Layer A   Manifesto / 终局愿景                Appendix A in whitepaper (Information Loom Manifesto)
Layer B   Architecture Translation table     Section 3.x of whitepaper, derived from § 5 of directive archive
Layer C   Implementation Roadmap (this doc)  P0–P9 with 6-field per phase + kill criteria + RSP-N + 90-day plan
```

## 11. Phase dependency graph (post-audit 2026-04-29 clarification)

The narrative ordering in § 3 (P0 → P1 → P2 → P3 → P4 → P5 → P6 → P7 → P8) is the *exposition* order. The *execution-dependency* ordering is more granular and enforced as follows:

```text
P0  Constitution-to-Code               (P0.R RootBox ceremony deferred; non-blocking for P1/P3)
 │
 └─ P1  GitTape Kernel  (kill criteria 1–4 must be RED→GREEN before next step)
     │
     ├─ P3  RSP Economy Core
     │   ├─ RSP-0 monetary invariant      (TB-1 Day-2)
     │   ├─ RSP-1 task escrow + YES stake (TB-2 default)
     │   ├─ P2  Agent Runtime           (depends on RSP-1: stakeless agents = no real role separation)
     │   ├─ P4  Information Loom         (depends on P1 L4.E rejection evidence + P3 reputation/stake events)
     │   ├─ RSP-2 Verifier bond + NO stake
     │   ├─ RSP-3 challenge window + provisional payout
     │   │   └─ P5  MetaTape v1          (depends on RSP-3 + P4: ArchitectAI proposal flow needs error clusters
     │   │                                   from L4.E + economic stake to make patches accountable)
     │   ├─ RSP-4 Contribution DAG + settlement
     │   ├─ RSP-5 reuse royalty
     │   └─ RSP-6 price index
     │
     └─ P6  Epistemic Lab v0 (PPUT-CCL / MiniF2F)
         (out-of-order anchor evidence; allowed to advance ahead of P1+P3 but cannot
          define infrastructure green; does NOT discharge P1/P3 kill criteria)
```

Critical reading of this graph:

- **P2 Agent Runtime depends on P3 RSP-1, not on P1 alone.** A multi-Solver swarm without stake/escrow is just one Solver wearing N hats — it doesn't prove role separation. The directive's narrative had P2 immediately after P1, but the audit-driven reading is: P2's role-separation Exit criteria (Solver / Verifier / Challenger / Planner with distinct economic responsibilities) cannot be demonstrated until RSP-1 is at minimum partial-green.
- **P4 Information Loom depends on P1 L4.E + P3 reputation events.** The clusterer / signal router / read-view compiler need real rejection-evidence records to cluster and real reputation-update events to broadcast. Without them, P4 has no input. So P4 follows P1 L4.E + RSP-2's reputation_update_tx, not the narrative position alone.
- **P5 MetaTape v1 depends on RSP-3 + P4.** ArchitectAI proposes patches based on error clusters (P4 output) and accepts economic accountability for them via stake (RSP-3). Without both, P5 is either a recommender system that no one funds or a stake market without a signal source.
- **P6 Epistemic Lab is permitted out-of-order** as anchor evidence (the MiniF2F / PPUT-CCL work). Per `feedback_tb_phase_tag_required`, every P6 TB MUST stamp `phase_id: P6` and explicitly disclaim that it does not advance P1/P3 kill criteria.

When TB selection conflicts, the rule is: **lowest-numbered phase with a RED kill criterion wins, regardless of narrative position.** P6 anchor evidence never wins this tie-break.

## 12. Cross-references

- Directive archive (verbatim): `handover/directives/2026-04-29_9_phase_roadmap.md`
- External audit (verbatim): `handover/audits/2026-04-29_external_audit.md`
- Decision record (L4 / L4.E split): `handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md`
- TB methodology: `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` § "TB methodology"
- TB log: `handover/tracer_bullets/TB_LOG.tsv` (schema upgraded 2026-04-29)
- TB-1 re-charter: `handover/tracer_bullets/TB-1_recharter_2026-04-29.md`
- TB-1 Day-1 evidence: `handover/tracer_bullets/TB-1_day1_spike_2026-04-29.md`
- Constitution + sudo: `constitution.md` + `genesis_payload.toml [trust_root]` + R-018
- PPUT-CCL pre-reg (now P6 Epistemic Lab v0 product line): `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md`
