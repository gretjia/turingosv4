# Architect ruling — TB-12 Node Exposure Index
## Lossless archive (Kolmogorov compression policy: no summary, full original)

**Date**: 2026-05-03 morning.

**Trigger**: User chose path (B) "directly into TB-12, TB-11 wire-up
absorbed into TB-12 setup". AI coder posed Q1-Q6 open decisions; the
architect responded with a 12-section ruling that revises the
proposed atom plan + sets Q1-Q6 with explanations.

**Authority chain**:
- 2026-05-03 morning user ruling (verbatim §1 below).
- TB-11 SHIPPED state (commits 870cd29 → 3eea2a3) per
  `handover/ai-direct/LATEST.md` head section.
- This archive is the operative spec for TB-12.

**Per `feedback_kolmogorov_compression`**: full original archived;
annotation layer at
`handover/architect-insights/RULING_TB12_NODE_EXPOSURE_INDEX_2026-05-03.md`
(non-authoritative, references this archive for citations).

---

## §0 Wrapper sentence from user (verbatim)

```text
下面是我的最终裁决。可以直接发给 AI coder 执行。
```

---

## §1 Architect ruling text (verbatim)

> 我先给一句话版：
>
> TB-12 可以启动，但它必须是"Node exposure index"，不是"Node trading market"。
> 它只记录 WorkTx.stake -> FirstLong exposure 与 ChallengeTx.stake -> Short exposure，不做价格、不做 CompleteSet、不做交易、不做 AMM、不做 ghost liquidity。
> 同时，TB-11 遗留的 evidence-capsule runtime wire-up 必须先补齐或作为 TB-12 的 Atom 0.5 先关掉，不能埋在中段。
>
> ⸻
>
> 0. 背景校准
>
> 当前阶段已经有三个关键事实：
>
> 1. TB-10 已经把 Lean Proof Task Market MVP 跑通，durable identity、minimal payout、FinalizeRewardTx、Frame B ChainTape routing 都已经存在；下一步原路线曾指向 NodeMarket Decision + Position Index. ￼
> 2. Zeta 压力测试暴露了 "Invisible Graveyard"：大量失败尝试在 evaluator 私有日志里，ChainTape 只看到任务开立与资金锁定，这要求 TB-11 先补 EvidenceCapsule / RunExhausted / TaskExpire 这类失败证据与资金释放机制. ￼
> 3. 宪法基本法仍然不可破坏：Information is Free、Only Investment Costs Money、1 Coin = 1 YES + 1 NO、on_init 是唯一合法铸币点. ￼
>
> 所以 TB-12 的定位必须很窄：
>
> TB-12 = Node exposure index
> 不是 prediction market
> 不是 trading market
> 不是 AMM
> 不是 CompleteSet
> 不是 price oracle
>
> ⸻
>
> 1. 对 TB-12 总体方案的裁决
>
> 1.1 是否启动 TB-12？
>
> 可以启动。
>
> 但必须满足两个前置条件：
>
> 1. TB-11 的核心成果已经 ship；
> 2. TB-11 遗留 G3/G4 必须作为 TB-12 Atom 0.5 先补齐，不能藏在 Atom 3 中段。
>
> 你给的 Atom 3 包含：
>
> TB-11 wire-up carry-forward:
>   evaluator binary MAX_TX exhausted -> write_evidence_capsule + tb11_emit_terminal_summary_for_run
>   lean_market tick / view-bankruptcy / view-positions
>
> 我的裁决：
>
> 把 TB-11 carry-forward 从 Atom 3 移到 Atom 0.5。
>
> 原因：
>
> EvidenceCapsule / bankruptcy / exhausted run 是 NodeMarket 的前置地基。
> 如果失败任务不能被锚定，ShortPosition 就没有可靠的失败对象。DeepThink 对 "你不能做空幽灵" 的批判是正确的. ￼
>
> ⸻
>
> 2. Q1 — risk class
>
> 裁决：Class 3 envelope，内部 atom 可分级
>
> 你的默认是：
>
> Class 3
>
> 我接受，但更精确地说：
>
> TB-12 总体按 Class 3 管理；
> Atom 1 schema 可能是 Class 2；
> Atom 2 dispatch wire 是 Class 3；
> Atom 3 dashboard / CLI 是 Class 1；
> ship audit 按 Class 3。
>
> 理由：
>
> 虽然 NodePosition 本身不是 money holding，但它接入：
>
> WorkTx accept arm
> ChallengeTx accept arm
> EconomicState schema
> future market settlement semantics
>
> 这属于未来经济系统的地基。一旦错了，后面 PriceIndex / CompleteSet / CPMM 都会继承错误。
>
> 所以：
>
> Q1 = Class 3
> Codex + Gemini ship audit
>
> Gemini exhausted 可以 degraded label，但不能假装 full dual audit。
>
> ⸻
>
> 3. Q2 — NodePosition 索引形状
>
> 你给了两个选项：
>
> 默认：
>   node_positions_t: BTreeMap<TxId/*position_id*/, NodePosition>
> 备选：
>   node_market_t: BTreeMap<TxId/*node_id*/, NodeMarketEntry>
>
> 你推荐备选，想提前对齐 TB-14 PriceIndex。
>
> 裁决：选择默认 flat NodePositionsIndex，不选择 nested NodeMarketEntry 作为 canonical state
>
> 最终：
>
> pub struct NodePositionsIndex(
>     pub BTreeMap<TxId, NodePosition>
> );
>
> 不要现在新增 canonical：
>
> node_market_t: BTreeMap<NodeId, NodeMarketEntry>
>
> 原因：
>
> 3.1 TB-12 的 source of truth 是 position record，不是 market aggregate
>
> TB-12 应该只建立：
>
> 某个 accepted WorkTx / ChallengeTx 产生了某个 exposure record。
>
> 而不是建立：
>
> 某个 node 的市场状态。
>
> NodeMarketEntry 是 TB-14 PriceIndex / market-view 的派生聚合，不应在 TB-12 成为 EconomicState 的 canonical sub-field。
>
> 3.2 过早引入 NodeMarketEntry 会制造第二个 source of truth
>
> 如果 TB-12 同时有：
>
> node_positions_t
> node_market_t
>
> 很快会遇到：
>
> long_interest 是从 positions 计算？
> 还是 node_market_t 自己维护？
> 如果两者不一致，谁是真？
>
> 这和之前 TaskMarket.total_escrow 双计风险类似。我们已经学过一次教训：derived aggregate 不能伪装成 money/source-of-truth。
>
> 3.3 正确方案
>
> TB-12：
>
> canonical:
>   node_positions_t
> derived view:
>   NodeExposureView / NodeMarketPreview
>
> TB-14 再做：
>
> NodeMarketEntry / PriceIndex
>
> 并且明确它从 node_positions_t 派生。
>
> 所以：
>
> Q2 = 默认 flat NodePositionsIndex
>
> 但可以在 dashboard 生成：
>
> node_market_preview_by_node_id
>
> 这只是 view，不进入 EconomicState。
>
> ⸻
>
> 4. Q3 — node_id 选择
>
> 裁决：采用你给的默认
>
> FirstLong.node_id = 自身 work_tx_id
> ChallengeShort.node_id = challenge.target_work_tx
>
> 正式定义：
>
> pub struct NodePosition {
>     pub position_id: TxId,
>     pub node_id: TxId,
>     pub task_id: TaskId,
>     pub owner: AgentId,
>     pub side: PositionSide,
>     pub kind: PositionKind,
>     pub amount: MicroCoin,
>     pub source_tx: TxId,
>     pub opened_at_round: u64,
> }
>
> 其中：
>
> position_id = source_tx
>
> 在 TB-12 中成立，因为：
>
> one accepted WorkTx -> one FirstLong
> one accepted ChallengeTx -> one ChallengeShort
>
> 未来 MarketBuyTx / MarketTradeTx 如果会一笔交易产生多个 lots，可以再引入新的 position_id 生成规则。TB-12 不需要。
>
> ⸻
>
> 5. Q4 — dual audit
>
> 裁决：采用默认 Class 3 双审
>
> Codex implementation audit
> Gemini architecture audit
>
> Gemini 如果 exhausted：
>
> use degraded label
> do not call it full strategic dual audit
>
> 不采用 TB-11 honest deferral 模式。
>
> 原因：
>
> TB-12 会接触：
>
> EconomicState schema 10 -> 11 sub-fields
> WorkTx accept side-effect
> ChallengeTx accept side-effect
> future market semantics
>
> 这不是单纯 docs 或 dashboard work。它属于 economic skeleton。
>
> 所以：
>
> Q4 = 默认 dual audit
>
> ⸻
>
> 6. Q5 — smoke 模式
>
> 裁决：默认 deterministic integration tests 为 ship gate；真实 LLM zeta rerun 作为 carry-forward evidence，不作为 TB-12 NodePosition 主 gate
>
> 你给的默认是：
>
> deterministic integration tests 覆盖 SG-12.1..6
> 可选 real-LLM zeta re-run 验 G4
>
> 我接受，但做两个调整：
>
> 6.1 TB-12 主 gate 用 deterministic tests
>
> NodePosition 是确定性的经济索引，不需要靠 LLM smoke 证明。
>
> 核心 tests：
>
> accepted_worktx_creates_firstlong_position
> accepted_challengetx_creates_challengeshort_position
> verifytx_bond_does_not_create_position
> node_position_not_counted_in_total_supply
> replay_reconstructs_node_positions
> dashboard_displays_exposures
>
> 6.2 zeta rerun 是 TB-11 carry-forward gate
>
> 如果 Atom 0.5 补 TB-11 G3/G4，那么 zeta rerun 应该验证：
>
> MAX_TX exhausted -> EvidenceCapsule
> RunExhaustedTx / TerminalSummary emitted
> view-bankruptcy works
>
> 它不是验证 NodePosition 的主要测试。
>
> 所以：
>
> Q5 = deterministic integration tests + optional/targeted zeta carry-forward smoke
>
> 如果 TB-11 G4 被标为必须闭合，那 zeta rerun 变成 Atom 0.5 ship gate，而不是 TB-12 market gate。
>
> ⸻
>
> 7. Q6 — 同步模式
>
> 你给三个选项：
>
> (i) atom-by-atom 停下
> (ii) charter ratify 后跑到 ship-gate 再停
> (iii) 全自动跑到 SHIP
>
> 裁决：选择 (ii)
>
> charter ratify 后跑到 ship-gate 再停。
>
> 不要每个 atom 都停。这样太慢。
>
> 也不要全自动 SHIP。因为 TB-12 是 Class 3，涉及经济索引与未来市场语义。
>
> 执行规则：
>
> 1. 我批准修正版 charter 后，AI coder 可连续跑 Atom 0.5 -> Atom 5。
> 2. 到 recursive self-audit + external audit 前停下。
> 3. 审计 verdict 出来后，再由你确认 SHIP。
>
> 如果出现：
>
> CTF conservation failure
> WorkTx/ChallengeTx position mismatch
> NodePosition counted as Coin
> replay divergence
> Gemini/Codex VETO
>
> 必须立即停止。
>
> ⸻
>
> 8. 我建议的修正版 TB-12 atom 计划
>
> 你原计划是 Atom 0–7。
> 我建议改成以下结构：
>
> ⸻
>
> Atom 0 — Charter + ratification
>
> 内容：
>
> TB-12 charter
> phase_id / roadmap_exit / kill criteria
> flowchart_trace
> forbidden list
>
> 必须写明：
>
> TB-12 is Node exposure index, not trading market.
>
> ⸻
>
> Atom 0.5 — TB-11 carry-forward closure
>
> 把你原 Atom 3 的 TB-11 carry-forward 移到这里。
>
> 内容：
>
> evaluator MAX_TX exhausted -> write_evidence_capsule
> tb11_emit_terminal_summary_for_run
> lean_market tick
> view-bankruptcy
>
> 不在这里做：
>
> view-positions
>
> view-positions 应该等 NodePositionsIndex 落地后再做。
>
> Ship gate：
>
> zeta / hard-fail run produces EvidenceCapsule
> RunExhausted / TerminalSummary visible
> view-bankruptcy works
> raw evidence shielded
>
> ⸻
>
> Atom 1 — Schemas: NodePosition + NodePositionsIndex
>
> 新增：
>
> PositionSide
> PositionKind
> NodePosition
> NodePositionsIndex
> EconomicState: 10 -> 11 sub-fields
>
> 但必须写入 doc comment：
>
> NodePosition is an exposure record / derived index.
> It is not a Coin holding.
>
> 建议：
>
> pub enum PositionSide {
>     Long,
>     Short,
> }
> pub enum PositionKind {
>     FirstLong,
>     ChallengeShort,
> }
>
> 不要加：
>
> MarketBuy
> MarketSell
>
> 它们是未来交易层。
>
> ⸻
>
> Atom 2 — Dispatch wire: WorkTx / ChallengeTx exposure records
>
> 规则：
>
> accepted WorkTx with stake > 0:
>   create FirstLong NodePosition
> accepted ChallengeTx with stake > 0:
>   create ChallengeShort NodePosition
> accepted VerifyTx:
>   no NodePosition
>
> 必须不动：
>
> balances_t
> stakes_t
> challenge_cases_t
> total_supply
>
> NodePosition 是 side-effect index，不是 money movement。
>
> ⸻
>
> Atom 3 — Replay invariants + total supply guard
>
> 必须证明：
>
> replay reconstructs node_positions_t
> node_positions_t has no effect on total_supply_micro
> node_position.source_tx exists and is accepted L4
> node_position.node_id points to accepted WorkTx
> VerifyTx does not create position
>
> 这一步很重要，不能省。
>
> ⸻
>
> Atom 4 — CLI / dashboard view-positions
>
> 现在才加：
>
> lean_market view-positions
> dashboard §13 long/short exposure per node
>
> 输出应该叫：
>
> Exposure records
>
> 不要叫：
>
> Open market balances
>
> 因为 TB-12 还没有交易市场，也没有 live share balances。
>
> ⸻
>
> Atom 5 — Smoke / tests
>
> Deterministic tests
>
> SG-12.1 WorkTx -> FirstLong
> SG-12.2 ChallengeTx -> Short
> SG-12.3 VerifyTx -> no position
> SG-12.4 no total supply impact
> SG-12.5 dashboard exposure view
> SG-12.6 no trading tx variants
>
> Optional zeta smoke
>
> 如果 Atom 0.5 需要：
>
> real LLM zeta re-run verifies EvidenceCapsule / TerminalSummary path
>
> ⸻
>
> Atom 6 — Recursive self-audit + external audit
>
> Class 3：
>
> Codex + Gemini
>
> Audit prompt must ask:
>
> Does NodePosition create a second money ledger?
> Does replay reconstruct positions deterministically?
> Does VerifyTx bond avoid market classification?
> Does NodePosition avoid total supply counting?
> Does TB-12 accidentally implement trading?
>
> ⸻
>
> Atom 7 — SHIP
>
> Only after audit.
>
> ⸻
>
> 9. TB-12 requirements 更新版
>
> 9.1 Functional requirements
>
> FR-12.1
> Accepted WorkTx with stake creates FirstLong NodePosition.
> FR-12.2
> Accepted ChallengeTx with stake creates ChallengeShort NodePosition.
> FR-12.3
> VerifyTx.bond does not create NodePosition.
> FR-12.4
> FirstLong.node_id = work_tx_id.
> FR-12.5
> ChallengeShort.node_id = challenge.target_work_tx.
> FR-12.6
> NodePosition.position_id = source_tx for TB-12.
> FR-12.7
> NodePosition can be replayed from ChainTape.
> FR-12.8
> Dashboard / view-positions displays exposure per node.
>
> ⸻
>
> 9.2 Constitutional requirements
>
> CR-12.1
> NodePosition is not Coin.
> CR-12.2
> NodePosition.amount is not counted in total_supply_micro.
> CR-12.3
> NodePosition cannot make a failed proposal accepted.
> CR-12.4
> Price is not introduced in TB-12.
> CR-12.5
> Trading is not introduced in TB-12.
> CR-12.6
> No ghost liquidity.
> CR-12.7
> No automatic YES/NO injection.
> CR-12.8
> VerifyTx.bond remains responsibility bond, not market side.
>
> ⸻
>
> 9.3 Ship gates
>
> SG-12.1
> accepted_worktx_creates_firstlong_position passes.
> SG-12.2
> accepted_challengetx_creates_challengeshort_position passes.
> SG-12.3
> verifytx_does_not_create_node_position passes.
> SG-12.4
> node_positions_do_not_change_total_supply passes.
> SG-12.5
> replay_reconstructs_node_positions passes.
> SG-12.6
> dashboard_view_positions_works passes.
> SG-12.7
> no_market_trading_variants_introduced passes.
> SG-12.8
> no_node_market_entry_as_canonical_state passes.
>
> ⸻
>
> 9.4 Forbidden list
>
> No CompleteSet.
> No MarketSeedTx.
> No CPMM.
> No AMM.
> No CLOB.
> No MarketOrderTx.
> No MarketTradeTx.
> No price_yes / price_no calculation.
> No automatic liquidity.
> No ghost liquidity.
> No NodeMarketEntry as canonical EconomicState field.
> No settlement change.
> No payout change.
> No slash change.
> No VerifyTx market position.
>
> ⸻
>
> 10. 最重要的洞察：NodePosition 是"历史暴露记录"，不是"活跃持仓余额"
>
> 这点必须写进 charter。
>
> TB-12 中：
>
> NodePosition = immutable exposure record
>
> 它记录：
>
> 这个 Agent 在这个 node 上承担了 Long / Short 方向的经济风险。
>
> 它不是：
>
> 可交易份额余额
> 可赎回 claim
> Coin holding
> LP share
> market order
>
> 所以 TB-12 不需要处理：
>
> position close
> position settlement
> position transfer
> position mark-to-market
>
> 后续 TB-13 CompleteSet 才会引入真正的 YES/NO claim。
> 后续 TB-14 PriceIndex 才会引入价格。
> 后续 TB-16 市场烟测才会看仓位盈亏。
>
> 这能防止 TB-12 过早膨胀。
>
> ⸻
>
> 11. 给 AI coder 的直接执行口令
>
> 可以直接发：
>
> Architect ruling for TB-12:
> 1. Authorize TB-12, but revise charter before coding.
> 2. Q1 risk class = Class 3 envelope.
> 3. Q2 choose flat NodePositionsIndex as canonical state:
>    node_positions_t: BTreeMap<TxId, NodePosition>.
>    Do NOT add canonical node_market_t / NodeMarketEntry in TB-12.
>    NodeMarketEntry is TB-14 derived view.
> 4. Q3 node_id:
>    FirstLong.node_id = its own WorkTx tx_id.
>    ChallengeShort.node_id = challenge.target_work_tx.
>    position_id = source_tx_id for TB-12.
> 5. Q4 dual audit:
>    Codex + Gemini ship audit. Gemini exhausted => degraded label.
> 6. Q5 smoke:
>    deterministic integration tests are TB-12 ship gate.
>    zeta real smoke only for TB-11 carry-forward if needed.
> 7. Q6 sync mode:
>    choose (ii) — after charter ratification, run to ship-gate, then stop for audit/user review. Do not fully auto-ship.
> Revised atom plan:
> Atom 0:
>   charter + ratification.
> Atom 0.5:
>   TB-11 carry-forward closure:
>   evaluator MAX_TX exhausted -> EvidenceCapsule / TerminalSummary,
>   lean_market tick + view-bankruptcy.
>   Do not implement view-positions here.
> Atom 1:
>   NodePosition schema:
>   PositionSide, PositionKind, NodePosition, NodePositionsIndex.
>   EconomicState 10->11 sub-fields.
>   NodePosition is exposure record, not Coin holding.
> Atom 2:
>   dispatch wire:
>   accepted WorkTx -> FirstLong.
>   accepted ChallengeTx -> ChallengeShort.
>   VerifyTx -> no position.
>   Do not touch balances/stakes/challenge_cases/total_supply.
> Atom 3:
>   replay + invariants:
>   positions replay deterministically.
>   positions not counted in total supply.
>   source_tx exists.
>   node_id points to accepted WorkTx.
> Atom 4:
>   dashboard + lean_market view-positions.
> Atom 5:
>   deterministic integration tests + optional zeta carry-forward smoke.
> Atom 6:
>   recursive self-audit + Codex/Gemini Class 3 audit.
> Atom 7:
>   SHIP only after audit.
> Forbidden:
>   no CompleteSet,
>   no MarketSeedTx,
>   no AMM,
>   no price,
>   no trading,
>   no ghost liquidity,
>   no automatic YES/NO injection,
>   no NodeMarketEntry as canonical state,
>   no VerifyTx market position.
>
> ⸻
>
> 12. 最终判断
>
> TB-12 的正确定位是：
>
> 从"Agent 做了什么"进一步升级为：
> Agent 在每个 node 上承担了什么方向的经济风险。
>
> 但它还不是：
>
> 这个风险如何交易；
> 这个风险如何定价；
> 这个风险如何兑付。
>
> 所以最稳的实现是：
>
> flat NodePositionsIndex
> immutable exposure records
> no money movement
> no price
> no trading
>
> 这会给 TB-13 的 CompleteSet 和 TB-14 的 PriceIndex 打一个干净、可审计、不双计、不 ghost liquidity 的地基。

---

## §2 Reconciliation with prior 2026-05-02 supplementary directive

This 2026-05-03 ruling **overrides** the AI coder's earlier
recommendation (nested `NodeMarketEntry`) and **refines** the
2026-05-02 supplementary directive's TB-12 spec on three axes:

1. **Index shape**: flat `node_positions_t: BTreeMap<TxId, NodePosition>`
   is canonical; `NodeMarketEntry` is **explicitly forbidden** in
   TB-12 (deferred to TB-14 derived view). The supplementary
   directive's TB-12 §2 struct schema (NodePosition fields) is
   accepted; the implicit "node_market_entry" tendency is rejected.

2. **TB-11 carry-forward placement**: Atom 0.5 (not Atom 3). Real
   zeta-LLM smoke is a TB-11 carry-forward gate, not a TB-12 market
   gate.

3. **Audit mode**: full Class 3 dual audit at Atom 6. No TB-11-style
   "honest deferral" §8.

4. **Sync mode**: Q6 (ii) — charter ratification, then continuous
   run to ship-gate, then stop for audit/user review.

5. **Position is immutable exposure record**, not active position
   balance. Forbidden TB-12 ops: position close / settlement /
   transfer / mark-to-market.

---

## §3 Forward annotations (non-authoritative)

### Atom mapping (charter)

| Architect §  | Charter atom            | Class | Touches restricted file? |
|---|---|---|---|
| §1.1 prereq + §8 Atom 0 | Atom 0 charter + ratification | 0 | additive doc |
| §8 Atom 0.5 (TB-11 carry-forward closure) | Atom 0.5 evaluator + lean_market wire-up | 2 | evaluator.rs + lean_market.rs |
| §8 Atom 1 (schema) | Atom 1 NodePosition + NodePositionsIndex | 2 (schema-only; per Q1) | typed_tx.rs + q_state.rs (additive) |
| §8 Atom 2 (dispatch wire) | Atom 2 WorkTx/ChallengeTx accept-arm side-effect | 3 | sequencer.rs (touches accept paths; per Q1) |
| §8 Atom 3 (replay + invariants) | Atom 3 invariant tests | 1-2 | tests/ |
| §8 Atom 4 (dashboard + CLI) | Atom 4 §13 + view-positions | 1 | audit_dashboard.rs + lean_market.rs |
| §8 Atom 5 (smoke / tests) | Atom 5 deterministic integration | 1 | tests/ |
| §8 Atom 6 (audit) | Atom 6 recursive + Codex + Gemini | 3 | audit docs |
| §8 Atom 7 (SHIP) | Atom 7 ship after audit | 0 | doc + commit |

### Memory implications

Updates required:
- new `project_tb_12_node_exposure_index.md` pointing to this archive
- update `project_tb11_to_tb17_roadmap.md` — note that TB-12 architect
  ruling **flattened** the canonical state (no NodeMarketEntry; TB-14
  derived view). This refines the earlier supplementary directive.

### Constitutional check

- **Art. 0** Turing fundamentalism: NodePosition is an L4-derived
  index entry; canonical bytes (BTreeMap encoding) preserved.
- **Art. I.1** 5-step compile loop: WorkTx accept now produces TWO
  side-effects (existing Q_t mutation + new NodePosition record);
  pure-additive on existing accept arm.
- **Art. III.4** no fake accepted: NodePosition cannot make a failed
  proposal accepted (CR-12.3 explicit).
- **Art. V** Anti-Oreo: NodePosition is sequencer-derived from
  accepted typed-tx; no agent-callable surface.
- **CTF conservation**: NodePosition.amount is NOT in
  total_supply_micro (CR-12.2) — explicitly excluded from the 5-holding
  sum.

No constitution.md edit required.

### Halting conditions (architect §7 verbatim)

The agent MUST halt if any of:
- CTF conservation failure
- WorkTx/ChallengeTx position mismatch
- NodePosition counted as Coin
- replay divergence
- Gemini/Codex VETO
