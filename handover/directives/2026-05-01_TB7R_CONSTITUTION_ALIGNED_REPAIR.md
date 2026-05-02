---
date: 2026-05-01
ingested_by: claude (architect-ingest skill)
phase_id: P2 (TB-7 carry-forward)
status: ARCHIVED — awaiting authorization
supersedes: handover/directives/2026-05-01_TB7_ARCHITECT_RULING.md (key correction in §4)
relates_to:
  - handover/ai-direct/HANDOVER_TB_7_7_D7_PENDING_2026-05-01.md (D7 A/B/C verdict)
  - handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md
  - handover/tracer_bullets/TB-7.7_charter_2026-05-01.md
classification: architect directive — Constitution-Aligned Frame B Repair scope
---

# Architect Ruling 2026-05-01 — Constitution-Aligned Frame B Repair

## Headline correction (vs. prior ruling)

**A real LLM proposal that fails Lean / predicates MUST go to L4.E rejection
evidence. It MUST NOT be force-written into L4 accepted just to manufacture
DAG / golden-path nodes.** This corrects the prior ruling's suggestion that
all real WorkTxs be configured with stake/escrow so that "at least one real
WorkTx enters L4 accepted." The corrected rule is:

```text
predicate pass  ->  L4 accepted transition
predicate fail  ->  L4.E rejection evidence
```

This is the first principle of this repair.

---

## Directive (verbatim)

> ultrathink 我的裁决：**你现在的修复方向必须重新压回宪法的三张 flowchart。**
>
> 我会修正我前一轮建议里的一个关键点：
>
> > **不能为了让真实 LLM proposal "进入 L4 accepted"，就给所有 proposal 配 stake / escrow 并让它们 accepted。**
> > 严格按宪法，`L4 accepted` 只能承载通过 predicates 的状态转移；没有通过 Lean / predicate 的候选，应该进入 `L4.E rejection evidence`，而不是被强行写入 accepted state。
> > 因此，真实 proposal 都必须进入 ChainTape，但不是都进入 L4；它们应分流为：
> >
> > ```text
> > predicate pass  -> L4 accepted transition
> > predicate fail  -> L4.E rejection evidence
> > ```
>
> 这是这次修复的第一原则。
>
> ---
>
> # 1. 以宪法三张 flowchart 重新定义修复边界
>
> 我把你说的"三个 flowchart"理解为当前 TuringOS 宪法和白皮书中反复出现的三层运行图：
>
> ```text
> Flowchart 1: Runtime State Transition
> Q_t -> rtool -> Agent -> proposal/output -> predicates -> wtool -> Q_{t+1}
>
> Flowchart 2: Boot / Genesis
> constitution/spec -> predicates/tools/genesis Q_0 -> on_init -> runtime loop
>
> Flowchart 3: Go Meta
> failure evidence -> ArchitectAI proposal -> Veto/JudgeAI -> sandbox/canary -> registry update
> ```
>
> 如果项目里这三张图的命名有不同，以这三个功能含义为准。
>
> 宪法第一段已经规定，顶层白盒不是 micromanagement 的全知独裁者，而是冷酷、透明、机械地对系统信息做 **量化、广播、屏蔽**；并且经济基本法是 `Information is Free` 与 `Only Investment Costs Money`，其中 `1 Coin = 1 YES + 1 NO`，`on_init` 是唯一合法铸币点。
>
> 这意味着：
>
> ```text
> Agent 可以自由生成候选；
> 但候选不能绕过 predicates / wtool / ChainTape；
> 失败候选不能污染其他 Agent 上下文；
> 经济 stake 不能让失败候选伪装成 accepted state。
> ```
>
> ---
>
> # 2. 对当前修复状态的审计
>
> 根据你贴出的最新进展，AI coder 已经把 5 个 audit action items 修完，并落在单 commit `f7f511b`：
>
> ```text
> #1  ChainTape 授权失败不再 warn+continue，而是 error+exit(3)
> #2  proposal_count 改为统计 L4.E tx_kind == Work，并解析 L4.E 的 ProposalTelemetry
> #3  self-audit 诚实修正为 3/7 fully closed + 4/7 partial
> #4  stale doc comment 修正
> #5  self-audit metadata 增加 commit 枚举
> ```
>
> 其中 #1 和 #2 是关键：授权失败继续 shadow-only 会破坏 authoritative ChainTape；proposal_count 不统计 L4.E WorkTx 会让真实失败分支消失。
>
> 这一步方向正确。
>
> 但你后续又开始 TB-8 Dashboard 和 5 道真题烟测，并暴露了几个本质问题：
>
> ```text
> CONDITION=oneshot 不走 ChainTape；
> n1 才走 run_swarm 的 ChainTape 路径；
> runs 2/5 暴露 CAS index 并发写竞态；
> 当前真实 proposal 的 DAG / golden path / node append 仍不完整。
> ```
>
> AI coder 也指出 CAS race 是 blocking，parent_tx wiring 和 n5 multi-agent smoke 是高性价比修复。
>
> 我的判断：
>
> > **CAS race 必须立即修。**
> > **parent_tx / proposal payload / ChainTape DAG 修复必须继续做。**
> > **但所有修复必须服从三张 flowchart，不能为了 dashboard 或 v3 分析而绕开 predicates。**
>
> ---
>
> # 3. 重新定义几个术语，防止 drift
>
> 以后不要混用 "node append" 这个词。严格按宪法，分三类：
>
> ## 3.1 Attempt Node
>
> ```text
> Agent 生成的每一个候选 proposal。
> ```
>
> 它必须进入 ChainTape，但可以进入：
>
> ```text
> L4 accepted
> 或
> L4.E rejected evidence
> ```
>
> ## 3.2 State Node
>
> ```text
> 通过 predicates 的 accepted transition。
> ```
>
> 它才进入 L4 accepted spine，并改变 `Q_t -> Q_{t+1}`。
>
> ## 3.3 Rejection Evidence Node
>
> ```text
> 未通过 predicates 的候选。
> ```
>
> 它进入 L4.E，不改变 state_root，不占 accepted logical_t。
>
> 这样才能严格对齐：
>
> ```text
> Q_t -> rtool -> Agent -> predicates -> wtool -> Q_{t+1}
> ```
>
> ---
>
> # 4. 我撤回并修正前一轮的一个建议
>
> 我之前建议过：
>
> ```text
> 给真实 LLM WorkTx 配 non-zero stake + escrow，让至少一个真实 WorkTx 进入 L4 accepted。
> ```
>
> 这句话需要严格修正。
>
> 正确说法是：
>
> ```text
> 给真实 LLM WorkTx 配合法 stake / task context，
> 但是否进入 L4 accepted 只能由 predicates 决定。
> ```
>
> 也就是说：
>
> ```text
> Lean / predicate pass:
>   WorkTx -> L4 accepted
>
> Lean / predicate fail:
>   WorkTx -> L4.E rejection evidence
> ```
>
> 不要为了得到 accepted node，而把 Lean failed candidate 写入 L4 accepted。
>
> 白皮书的基本定义就是：Agent 不直接改 world state，Agent 提交 transaction / proposal，predicate 验证 transition，只有通过后 wtool 才能写入。此前 TB-7 charter 也明确是要让真实 LLM proposal 通过 `bus.submit_typed_tx → Sequencer::apply_one → on-disk ChainTape`，而不是继续 legacy `bus.append`。
>
> ---
>
> # 5. Flowchart 1：Runtime State Transition 的严格修复方案
>
> ## 5.1 目标
>
> 每一个真实 LLM proposal 必须走：
>
> ```text
> rtool/context
> -> Agent proposal
> -> ProposalTelemetry CAS
> -> WorkTx
> -> predicates
> -> Sequencer::apply_one
> -> L4 or L4.E
> -> replay / dashboard / run facts
> ```
>
> 不能有：
>
> ```text
> LLM proposal -> legacy bus.append authoritative write
> ```
>
> TB-7 architect request 也已经指出，TB-6 虽然关闭了 Frame A，但真实 LLM activity 仍然没有进链；TB-7 的 proposal 是让每个 LLM proposal 进入 `bus.submit_typed_tx → Sequencer::apply_one → on-disk ChainTape`，并且不包含 FinalizeRewardTx / SlashTx。
>
> ---
>
> ## 5.2 修复项 A：proposal payload 必须真实进入 CAS
>
> 现在如果只是：
>
> ```rust
> hash(payload_bytes) -> proposal_artifact_cid
> ```
>
> 但没有 `cas.put(payload_bytes)`，那它不是 tape，只是一个不可取回的 hash。
>
> 严格要求：
>
> ```text
> WorkTx.proposal_cid
>   -> ProposalTelemetry CAS object
>
> ProposalTelemetry.proposal_artifact_cid
>   -> actual proposal payload bytes in CAS
> ```
>
> 测试必须证明：
>
> ```text
> proposal_cid resolves
> proposal_artifact_cid resolves
> payload bytes match original candidate
> ```
>
> 如果 payload 不可取回，就不能说 Agent 的真实输出进入 tape。
>
> ---
>
> ## 5.3 修复项 B：parent_tx 必须是 proposal-level DAG edge
>
> `parent_tx` 不是 v3 citation feature。
> 它是 ChainTape DAG 的基础边。
>
> 严格定义：
>
> ```text
> parent_tx = 当前 proposal 所依赖的上一个 attempt tx
> ```
>
> 可以是：
>
> ```text
> parent_tx = None
> ```
>
> 表示 root attempt。
>
> 也可以是：
>
> ```text
> parent_tx = Some(previous_attempt_tx_id)
> ```
>
> 表示同一 branch 的下一个尝试。
>
> 不要把所有节点都挂到 OMEGA root。
> 不要用 dashboard 事后猜 parent。
> parent edge 必须在 ProposalTelemetry 中写入 CAS。
>
> 最低实现：
>
> ```rust
> last_tx_by_agent_branch: BTreeMap<(AgentId, BranchId), TxId>
> ```
>
> 每次 proposal 后：
>
> ```text
> new parent_tx = last_tx_by_agent_branch[(agent, branch)]
> last_tx_by_agent_branch[(agent, branch)] = current_tx_id
> ```
>
> 测试：
>
> ```text
> second proposal on same branch has parent_tx
> dashboard has non-empty edges
> replay can build attempt DAG
> ```
>
> ---
>
> ## 5.4 修复项 C：Lean oracle 结果必须进入 predicate evidence
>
> 严格按 Flowchart 1，Lean 不是普通日志。
> Lean 是 predicate / oracle evidence。
>
> 所以每个 candidate 应该产生：
>
> ```text
> VerificationResult CAS object
> ```
>
> 建议结构：
>
> ```rust
> VerificationResult {
>     target_work_tx: TxId,
>     lean_exit_code: i32,
>     verified: bool,
>     proof_artifact_cid: Cid,
>     stdout_cid: Option<Cid>,
>     stderr_cid: Option<Cid>,
>     verifier: "lean",
> }
> ```
>
> 然后：
>
> ```text
> predicate_results.acceptance["lean_verified"].proof_cid
> 或 ProposalTelemetry.verification_result_cid
> ```
>
> 必须指向它。
>
> 如果 Lean fail：
>
> ```text
> WorkTx -> L4.E
> rejection_class = PredicateFailed / LeanFailed
> raw_diagnostic_cid shielded
> public_summary safe
> ```
>
> 如果 Lean pass：
>
> ```text
> WorkTx -> L4 accepted
> predicate_results.acceptance["lean_verified"].value = true
> ```
>
> 不要让 evaluator stdout 成为 oracle truth。
>
> ---
>
> ## 5.5 修复项 D：chain.solved 必须拆成两个字段
>
> 现在 `chain.solved` 和 `evaluator.solved` divergence 是设计上必然的。
>
> 严格说，TB-7 阶段没有 settlement，所以不能说 economic solved。
>
> 应该拆成：
>
> ```text
> chain_oracle_verified: bool
> chain_economic_finalized: bool
> ```
>
> 定义：
>
> ```text
> chain_oracle_verified:
>   L4 accepted WorkTx exists
>   Lean VerificationResult.verified = true
>   proof artifact retrievable
>   predicate result linked
>
> chain_economic_finalized:
>   FinalizeRewardTx / SettlementTx exists
> ```
>
> TB-7 可以让：
>
> ```text
> chain_oracle_verified = true
> chain_economic_finalized = false
> ```
>
> 这样就既能分析 golden path，又不越界到 RSP-4。
>
> ---
>
> ## 5.6 修复项 E：legacy path 必须 fail-closed 或 shadow-only
>
> 如果 ChainTape mode 下发生：
>
> ```text
> submit_typed_tx failure
> CAS failure
> signature failure
> ledger commit failure
> ```
>
> 正确处理是：
>
> ```text
> error!()
> exit(3)
> ```
>
> 而不是：
>
> ```text
> warn!()
> continue legacy path
> ```
>
> 这点 AI coder 已经修了 #1。
>
> 但还要继续检查：
>
> ```text
> CONDITION=oneshot
> OMEGA-full
> OMEGA-pertactic
> append branch
> ```
>
> 所有真实 proposal 生产路径都不能绕开 ChainTape。
>
> 如果某个 mode 暂时未接入 ChainTape：
>
> ```text
> 在 ChainTape mode 下必须 fail-closed，
> 或者明确标记该 mode unsupported。
> ```
>
> 不能允许它悄悄走 legacy path。
>
> ---
>
> # 6. Flowchart 2：Boot / Genesis 的严格修复方案
>
> 真实 ChainTape run 不应该靠内存里随手 pre-seed。
>
> 必须有清晰 boot 流程：
>
> ```text
> Q_0
> on_init
> TaskOpenTx
> EscrowLockTx
> Agent key registry
> runtime_repo
> CAS
> system keypair
> ```
>
> 最低要求：
>
> ## 6.1 Run genesis report
>
> 每次 ChainTape smoke 必须生成：
>
> ```text
> genesis_report.json
> ```
>
> 包含：
>
> ```json
> {
>   "constitution_hash": "...",
>   "runtime_repo": "...",
>   "cas_path": "...",
>   "system_pubkey": "...",
>   "agent_pubkeys": "...",
>   "initial_balances": "...",
>   "task_id": "...",
>   "task_open_tx": "...",
>   "escrow_lock_tx": "..."
> }
> ```
>
> ## 6.2 TaskOpen / EscrowLock 必须是 ChainTape 事件
>
> 如果这次 run 需要 task / escrow，不要只在内存里塞：
>
> ```text
> q.economic_state_t.task_markets_t.insert(...)
> q.economic_state_t.escrows_t.insert(...)
> ```
>
> 而应该生成 accepted：
>
> ```text
> TaskOpenTx
> EscrowLockTx
> ```
>
> 这样后续 replay 才能重建经济状态。
>
> ## 6.3 Agent keys 必须进 runtime repo
>
> AI coder 已经有 `agent_pubkeys.json` 的方案。
> 严格要求：
>
> ```text
> WorkTx signature verifies against agent_pubkeys.json
> agent_pubkeys.json included in evidence
> ```
>
> 否则 Agent 身份不可审计。
>
> ---
>
> # 7. Flowchart 3：Go Meta 的边界
>
> 这次修复不是 MetaTape。
>
> 不要做：
>
> ```text
> ArchitectAI 自动改 predicates
> dashboard 输出后自动改 rules
> 根据 5 道真题自动改提示词或工具
> ```
>
> 如果真实 run 暴露错误，只进入：
>
> ```text
> L4.E evidence
> audit report
> future Meta proposal
> ```
>
> 不能绕过：
>
> ```text
> ArchitectAI proposal -> Veto/JudgeAI -> sandbox/canary
> ```
>
> 所以当前修复只允许做：
>
> ```text
> runtime wiring
> CAS artifact
> ChainTape evidence
> dashboard materialized view
> ```
>
> 不允许做：
>
> ```text
> predicate registry mutation
> MetaTape
> NodeMarket
> role taxonomy
> whale/contested
> slash/payout
> ```
>
> ---
>
> # 8. TB-7 / TB-8 当前应执行的严格顺序
>
> 你现在说要按宪法和白皮书修复，我建议把当前工作改名为：
>
> ```text
> TB-7R — Constitution-Aligned Frame B Repair
> ```
>
> 或者如果 TB-8 已经开始：
>
> ```text
> TB-8 — Audit Dashboard over ChainTape
> ```
>
> 那也必须声明：
>
> ```text
> Dashboard is materialized view only.
> Dashboard is not source of truth.
> ```
>
> 执行顺序：
>
> ## Commit 1：CAS race / audit fixes
>
> 已经做了就保留：
>
> ```text
> CAS sidecar append atomic
> I90d/e/f tamper tests
> proposal_count includes L4.E WorkTx
> ```
>
> 这是 blocking 修复。
>
> ## Commit 2：Proposal payload CAS
>
> 所有真实 proposal：
>
> ```text
> payload bytes -> CAS
> ProposalTelemetry -> CAS
> WorkTx.proposal_cid -> ProposalTelemetry
> ```
>
> ## Commit 3：parent_tx DAG edge
>
> 所有真实 proposal：
>
> ```text
> ProposalTelemetry.parent_tx = ...
> ```
>
> dashboard 从 CAS + ChainTape 生成 DAG。
>
> ## Commit 4：Lean VerificationResult CAS
>
> 每个 proposal 的 Lean 结果：
>
> ```text
> VerificationResult -> CAS
> predicate proof / telemetry references it
> ```
>
> ## Commit 5：L4 / L4.E strict split
>
> ```text
> Lean pass -> L4
> Lean fail -> L4.E
> ```
>
> 不要为了有 accepted node，把 Lean fail 写进 L4。
>
> ## Commit 6：chain_oracle_verified
>
> 新增或修正 run facts：
>
> ```text
> chain_oracle_verified
> chain_economic_finalized
> ```
>
> ## Commit 7：真实 n5 smoke
>
> 运行：
>
> ```text
> CONDITION=n5
> MAX_TX >= 20
> ChainTape mode enabled
> ```
>
> 最低 evidence：
>
> ```text
> >=2 agent_ids
> >=1 L4 accepted WorkTx, 如果题目 solved
> >=1 L4.E rejected WorkTx
> >=1 parent_tx edge
> all proposal payload CIDs resolve
> Lean VerificationResult CIDs resolve
> dashboard DAG non-empty
> ```
>
> 如果 5 道题没有 solved，则不能伪造 L4 accepted。
> 可以另跑 `mathd_algebra_107` 或已知 solvable 题，作为 accepted-path smoke。
>
> ---
>
> # 9. 关于 "5 道真题" 的严格验收
>
> 5 道真题测试必须分两类结论：
>
> ## 9.1 每题都必须证明的
>
> ```text
> all LLM proposals are represented in ChainTape
> proposal_count_chain == proposal_count_runtime
> all proposal_cid resolves
> all proposal_artifact_cid resolves
> signatures verify
> replay verifies
> dashboard DAG generated
> ```
>
> ## 9.2 只有 solved 题才证明的
>
> ```text
> chain_oracle_verified = true
> golden path exists
> L4 accepted WorkTx exists
> Lean proof artifact resolves
> ```
>
> ## 9.3 unsolved 题应证明的
>
> ```text
> chain_oracle_verified = false
> no fake accepted WorkTx
> rejected attempts in L4.E
> failure branches visible
> ```
>
> 这比"5 题都要有 accepted node"更合宪。
>
> ---
>
> # 10. 给 AI coder 的明确指令
>
> 可以直接发：
>
> ```text
> Architect correction:
>
> We must repair strictly according to constitution flowcharts and the whitepaper.
>
> Do not force Lean-failed proposals into L4 accepted just to create accepted nodes.
> Accepted L4 = predicate-passed state transition only.
> Failed proposals must go to L4.E rejection evidence.
>
> Use these definitions:
> - Attempt node = every real LLM proposal, represented in L4 or L4.E.
> - State node = L4 accepted transition only.
> - Rejection evidence node = L4.E rejected WorkTx.
>
> Required repair order:
>
> 1. Keep the CAS race fix and I90d/e/f tamper tests.
> 2. Ensure every real proposal payload is actually stored in CAS.
>    Hash-only CID without stored payload is invalid.
> 3. WorkTx.proposal_cid must resolve to ProposalTelemetry.
>    ProposalTelemetry.proposal_artifact_cid must resolve to actual candidate bytes.
> 4. Wire parent_tx as proposal-level DAG edge.
>    Root attempts parent_tx=None; follow-up attempts parent_tx=previous tx on same branch.
> 5. Store Lean oracle result as CAS evidence:
>    VerificationResult { target_work_tx, lean_exit_code, verified, proof_artifact_cid, stdout/stderr hash/cid }.
> 6. Predicate pass/fail must determine L4 vs L4.E:
>    Lean verified -> L4 accepted.
>    Lean failed -> L4.E PredicateFailed / LeanFailed.
> 7. Split chain.solved into:
>    chain_oracle_verified
>    chain_economic_finalized
>    TB-7 can set oracle_verified=true; economic_finalized remains false.
> 8. In ChainTape mode, unsupported paths such as oneshot must fail-closed or be wired through submit_typed_tx.
>    No silent legacy fallback.
> 9. Dashboard is read-only materialized view from ChainTape + CAS.
>    It is not source of truth.
> 10. For 5-problem smoke:
>     each proposal must be represented in ChainTape;
>     solved problems must have golden path from ChainTape;
>     unsolved problems must show rejected branches without fake accepted nodes.
> 11. No NodeMarket, no role taxonomy, no whale/contested analysis, no slash, no payout, no MetaTape.
> ```
>
> ---
>
> # 11. 这次修复的最终验收标准
>
> 如果严格按宪法和白皮书，我只接受下面这种验收：
>
> ```text
> For each real LLM proposal:
>   CAS has proposal payload.
>   CAS has ProposalTelemetry.
>   WorkTx exists.
>   WorkTx is either L4 accepted or L4.E rejected.
>   Parent edge is recorded or explicitly root.
>   Predicate evidence is recorded.
> ```
>
> 对于 solved run：
>
> ```text
> There exists an L4 accepted WorkTx with lean_verified=true.
> Golden path is derivable from parent_tx edges.
> Proof artifact is retrievable from CAS.
> Replay verifies ledger root and signatures.
> ```
>
> 对于 failed run：
>
> ```text
> No fake accepted WorkTx.
> Rejected attempts are visible in L4.E.
> Failure reasons are shielded but auditable.
> ```
>
> 对于 dashboard：
>
> ```text
> Dashboard output can be deleted and regenerated from ChainTape + CAS.
> ```
>
> 如果 dashboard 需要 evaluator stdout 才能证明核心事实，则还不合格。
>
> ---
>
> # 12. 最终判断
>
> 你这次要求"严格按宪法和三张 flowchart"是对的，因为当前修复有一个潜在 drift：
>
> ```text
> 为了分析 DAG / golden path，
> 把所有 proposal 都推向 accepted L4。
> ```
>
> 这是不合宪的。
>
> 真正合宪的做法是：
>
> ```text
> 所有真实 proposal 都进入 ChainTape；
> 通过 predicates 的进入 L4；
> 失败的进入 L4.E；
> DAG 同时包含 L4 与 L4.E attempt nodes；
> golden path 只从 oracle-verified accepted node 推导；
> dashboard 只是 materialized view。
> ```
>
> 这样修完后，TuringOS 才能真正回答：
>
> ```text
> Agent 看到了什么？
> 提交了什么？
> 哪些失败？
> 哪些通过？
> 失败是否污染上下文？
> 成功是否可重放？
> 经济状态是否可重建？
> ```
>
> 这才是宪法、白皮书和 ChainTape 的一致实现。

---

## Ingestion analysis (claude, not authorization)

### Layer-1 invariant impact

| Invariant | Effect of directive |
|---|---|
| `kernel.rs` zero domain knowledge | **REINFORCED.** Directive routes all predicate logic through Sequencer + L4/L4.E split, never into kernel. |
| Append-Only DAG | **REINFORCED.** Directive mandates parent_tx DAG edges and forbids retroactive node mutation. |
| Economic conservation (`1 Coin = 1 YES + 1 NO`, `on_init` only mint) | **REINFORCED.** Directive forbids stake-driven L4-acceptance bypass; predicate pass alone advances state. |

**No Layer-1 violation.** Directive tightens existing invariants.

### Constitution / case-law alignment

- Art. III.4 selective broadcasting / shielding — affirmed (L4.E `raw_diagnostic_cid` stays shielded; only `public_summary` + counters in materialized view).
- C-021 / C-031 / C-034 / C-043 (mechanism > parameter > prompt) — affirmed (no prompt patches, no role taxonomy patches).
- C-023 (ArchitectAI ≠ swarm emergence) — affirmed (§7 Go-Meta boundary forbids automatic predicate / rule / prompt mutation).
- 2026-04-29 L4 / L4.E decision record — directive is direct extension of CF-1 boundary.

### Scope correction vs. prior 2026-05-01 ruling

The prior `2026-05-01_TB7_ARCHITECT_RULING.md` permitted "WorkTx with stake/escrow → at least one accepted L4 node." This directive **withdraws** that suggestion. New rule: stake/escrow are configured legitimately, but L4 vs L4.E is decided by predicates alone. Any reader of the prior ruling should defer to this file on that point.

### TB-7.7 D7 verdict (vs. handover A/B/C)

The handover at `handover/ai-direct/HANDOVER_TB_7_7_D7_PENDING_2026-05-01.md` lists three options for the calc-block-as-1-WorkTx finding:

- (A) accept + TB-8 — directive is silent; not directly addressed.
- (B) per-tactic split inside TB-7.7 — directive is silent; not directly addressed.
- (C) cut the `complete` tool, force one-tactic-per-turn — directive is silent; not directly addressed.

**Directive does NOT pick A/B/C.** It instead reframes: the per-tactic decomposition question lives below the directive's scope (which is L4/L4.E split + parent_tx + payload CAS + VerificationResult CAS). A/B/C verdict is still pending and must be re-asked separately or read as deferred to TB-8.

### Already-completed work that the directive validates

- D1 payload bytes → CAS (a39c31b) — matches §5.2 / Commit 2.
- D2 parent_tx wire (a39c31b) — matches §5.3 / Commit 3.
- D4 VerificationResult CAS object (89cd448) — matches §5.4 / Commit 4.
- D5 chain_oracle_verified / chain_economic_finalized split (901062b) — matches §5.5 / Commit 6.
- D6 audit_dashboard DAG + golden path (07b6067) — matches §9 dashboard role, **provided** dashboard remains read-only materialized view (verify).

### Items added vs. baseline plan

1. **Strict L4 vs L4.E predicate-driven split** (§4 + Commit 5) — must audit current accepted-WorkTx population: is any Lean-failed proposal currently in L4? If yes, that's a constitutional drift to fix in Commit 5.
2. **CAS race fix** (Commit 1) — referenced as already-staged; needs separate verification.
3. **Genesis report + on-chain TaskOpenTx / EscrowLockTx** (§6) — D3 pre-seed is in-memory; directive demands on-chain genesis events. **This may conflict with D3 commit 054254f** (pre-seed bootstrap). Needs review: does pre-seed bypass on-chain TaskOpenTx?
4. **5-problem acceptance criteria split (§9)** — relaxes "every problem must have accepted node" to "every problem must be represented in ChainTape; only solved problems need accepted node."
5. **Hard guardrail list (§7 + Commit 11)** — no NodeMarket, no role taxonomy, no whale/contested, no slash, no payout, no MetaTape. None of these are currently in scope, but the rail must hold for TB-7R / TB-8.

### Open questions before authorization

1. **D3 pre-seed vs §6.2:** Does `054254f` (pre-seed bootstrap for L4 accept) place initial state in memory only, or does it emit on-chain TaskOpenTx + EscrowLockTx? If memory-only, directive §6.2 demands rework.
2. **Current L4 population purity:** Is there any L4 accepted WorkTx in the existing evidence directories that did NOT pass Lean? If yes, those entries are constitutionally invalid under §4 and need migration to L4.E.
3. **TB rename:** Adopt `TB-7R` as the directive suggests, or keep TB-7.7 + TB-8? (Naming-only; non-blocking.)
4. **D7 A/B/C verdict:** The pending per-tactic decomposition question — does the user want to defer to TB-8 (consistent with §8 boundary) or still pick A/B/C inside TB-7.7?
5. **`oneshot` / `OMEGA-full` / `OMEGA-pertactic` / `append` paths in ChainTape mode:** §5.6 demands these all fail-closed or be wired through `submit_typed_tx`. Audit: which still fall back to legacy `bus.append`?

### Awaiting authorization

This file archives and analyzes. **No code changes have been made.** Per architect-ingest protocol: `接收指令 ≠ 授权执行`. Execute only after user authorizes which of the open questions above to resolve and in what order.
