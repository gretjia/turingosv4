# Architect ruling — TB-11 Epistemic Exhaust & Capital Liberation
## Lossless archive (Kolmogorov compression policy: no summary, full original)

**Date**: 2026-05-02
**Trigger**: Post-TB-10 ship; user (architect) reviewed DeepThink critique of an
earlier proposal to make `ExternalizedAttemptEvent` a per-attempt L4 ledger
entry, accepted the critique partially, and issued a re-prioritized TB-11
spec that supersedes the previous "TB-11 = NodeMarket Decision + Position
Index" planned next-TB direction recorded in:
  - `handover/ai-direct/LATEST.md` (TB-10 ship-section bottom + TB-13 PREVIEW
    session-close section)
  - `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md`
  - 2026-05-02 directive Part C ruling 13 line 1617

**Scope of this archive**: full verbatim text of the architect's ruling, with
markdown structure preserved exactly. Per `feedback_kolmogorov_compression`:
"Never 'distill' or 'summarize' architect directives. Use lossless Kolmogorov
compression: full original text archived + structured annotation layer."

This file is the **full original**. Annotation layer lives in
`handover/architect-insights/RULING_TB11_EPISTEMIC_EXHAUST_2026-05-02.md`
(non-authoritative; references this archive for citations).

**Authority chain**:
  - User authorization 2026-05-02 evening: "make it your own understanding but
    always align with the constitution. then you can make it all the way to
    the end of the developing TB phases without my intervention, I am going
    to sleep and I look forward to see the end of the project and ready for
    launch"
  - This archive is the operative spec for TB-11.

---

## §0 Wrapper instructions from user (verbatim, English)

```text
ultrathink you are the best agents on the street, follow what the architector
say, make it your own understanding but alwyas algin with the constitution.
then you can make it all the way to the end of the developing TB phases
without my intervention, I am going to sleep and I look forward to see the end
of the project and ready for launch:
```

---

## §1 Architect ruling text (verbatim, Chinese + English mixed)

> 根据 2026-05-02 的项目文档，当前 TuringOS 已经到达一个关键节点：TB-10 已把 **Lean Proof Task Market MVP** 跑通，用户开任务、锁 bounty、Agent 解题、系统验证、系统付款、durable identity 归属，都已经能在 canonical ChainTape 字节层面验证；下一步原计划是 TB-11 做 NodeMarket Decision Record + Position Index。
>
> 但 DeepThink 对我的批判是有效的。我现在明确修正自己的认知：
>
> > **我之前把"失败尝试必须可审计"过快地理解成"每个失败尝试都应该进入 L4 / L4.E"。这是一个层级错误。**
> > ChainTape 是状态转移账本，不是 debug stdout；失败轨迹应该以 CAS EvidenceCapsule 的方式被压缩归档，再由一个 O(1) 的链上状态事件锚定。
> > 这比"132 次 Lean 错误都各自写链"更符合宪法、三张 flowchart、白皮书，也更符合正式上线的工程约束。
>
> 下面是我的新认知和后续完整方案。
>
> ---
>
> # 1. 我从第一性原理重新审视 DeepThink 的批判
>
> DeepThink 的核心批判是：我提出的 `ExternalizedAttemptEvent` 如果被理解为"每一个 Lean syntax error / failed attempt 都要进入 L4 ChainTape"，会污染 L4，因为 ChainTape 是 **State Transition Ledger**，不是调试日志；它建议用 **EvidenceCapsule + TaskBankruptcyTx / TaskExpireTx** 来解决"幽灵坟场"问题。这个批判是合理的。
>
> 我现在把问题重新拆成三个层次：
>
> ```text
> State transition:
>   改变系统状态、资金、任务状态、身份、position、resolution 的事件。
>   应进入 L4 accepted ledger。
>
> Rejection evidence:
>   某个已提交 transaction 被 predicate / policy 拒绝。
>   应进入 L4.E。
>
> Epistemic exhaust:
>   大量失败提示、Lean error、parse failure、sorry-block、intermediate failed attempts。
>   不应逐条污染 L4。
>   应进入 CAS EvidenceCapsule，然后由一个 O(1) 链上事件锚定。
> ```
>
> 这就是我新的核心原则：
>
> > **O(1) chain cost, O(N) auditability.**
> > 链上只记录状态事件；CAS 中保存完整失败证据；dashboard 是只读物化视图。
>
> 这同时保留两个目标：
>
> ```text
> 1. 不让失败过程变成"幽灵坟场"；
> 2. 不把 ChainTape 变成无限 debug log。
> ```
>
> ---
>
> # 2. 我的旧观点哪里错了
>
> 我之前说过类似：
>
> ```text
> 每个 externalized attempt 都应进入 L4 或 L4.E。
> ```
>
> 这句话需要改写。正确版本是：
>
> ```text
> 每个对系统状态产生外部影响的 proposal / transaction 必须进入 L4 或 L4.E；
> 每个运行过程中的失败尝试、错误日志、Lean stderr、parse failure，
> 如果没有形成正式 transaction，则应进入 CAS EvidenceCapsule，
> 并由 TaskBankruptcyTx / RunExhaustedTx / TaskExpireTx 在 L4 中锚定。
> ```
>
> 为什么？
>
> 因为 Flowchart 1 的主干是：
>
> ```text
> Q_t -> read tool -> Agent -> output -> predicates -> write tool -> Q_{t+1}
> ```
>
> 只有经过 predicates 的候选状态转移才改变 `Q_t`。如果我们把每个内部失败都写成 L4 transition，就会模糊：
>
> ```text
> proposal
> debug trace
> tool stderr
> state transition
> ```
>
> 这四者的层级。
>
> 宪法里顶层白盒的职责是量化、广播、屏蔽，并且布尔谓词只回答"有没有通过验证"；价格、信誉、错误率等属于统计信号，而不是主观理解。 所以失败日志应该被压缩为证据与信号，而不是把所有原始噪声塞进状态主链。
>
> ---
>
> # 3. DeepThink 对"幽灵坟场"的洞察是正确的
>
> Zeta / `1 + 2 + 3 + ... = -1/12` 的压力测试暴露了一个真实漏洞：
>
> ```text
> LLM 实际尝试了 132 次；
> 有 73 个 Lean kernel errors；
> 14 次 sorry-block；
> 26 次 protocol parse failures；
> 但纯 ChainTape + CAS 审计只能看到：
>   用户发了任务；
>   用户锁了 bounty；
>   没有 solver 返回；
>   bounty 卡在 escrow。
> ```
>
> 项目记录也明确说：132 次 LLM proposals、32 个 partial-tactic accepts、73 个 Lean 错误、14 个 sorry-block、26 个 protocol parse failures 只在 evaluator-private log 中，ChainTape 中没有 CAS proposal telemetry，也没有 verification_result；纯 chain+CAS audit 只能看到"用户发了 500k bounty 任务，没人返回"。
>
> 这说明 TuringOS 当前的认识论保证是单边的：
>
> ```text
> 可以证明没有 fake accepted；
> 不能从链上证明系统确实抵抗了大量 fake attempts。
> ```
>
> DeepThink 把这称为 "Invisible Graveyard"，这个命名准确。
>
> 但解决方式不是把 132 次错误逐条放进 L4。
> 解决方式是：
>
> ```text
> EvidenceCapsule in CAS
> +
> TaskBankruptcyTx / RunExhaustedTx in L4
> +
> TaskExpireTx for capital liberation
> ```
>
> ---
>
> # 4. 我新的核心认知：负空间也必须有链上锚点
>
> TuringOS 以前主要证明：
>
> ```text
> positive truth:
>   一个 proposal 通过 predicate，进入 L4 accepted。
> ```
>
> 现在必须补上：
>
> ```text
> negative truth:
>   一个任务/运行经历了大量失败，并且最终没有通过 predicate。
> ```
>
> 这对未来 Polymarket / NodeMarket 非常关键。
>
> DeepThink 说得对：**你不能做空一个幽灵。**
> 如果失败只存在本地日志里，未来 `NO` / short position 没有可结算对象。
>
> 所以，未来 NodeMarket 的前提不是马上做 `FirstLongPosition`，而是先做：
>
> ```text
> failure event anchoring
> ```
>
> 也就是：
>
> ```text
> TaskBankruptcyTx
> RunExhaustedTx
> TaskExpireTx
> EvidenceCapsule
> ```
>
> 这会为未来的 `ShortPosition` 提供链上"死亡证明"。
>
> ---
>
> # 5. 重新定义 TB-11：不要先做 NodeMarket，先做 Epistemic Exhaust & Capital Liberation
>
> 我现在撤回之前"TB-11 先做 NodeMarket Decision + Position Index"的建议。
>
> **新的 TB-11 应该是：**
>
> ```text
> TB-11 — Epistemic Exhaust & Capital Liberation
> P4 Information Loom + P3 capital unlock carry-forward
> ```
>
> 理由：
>
> ```text
> 没有失败锚点，就没有可靠 Short；
> 没有 EvidenceCapsule，就没有失败学习；
> 没有 TaskExpireTx，就有 escrow 死锁；
> 没有 capital liberation，任务市场会逐渐僵死。
> ```
>
> 这不是功能扩展，而是 TuringOS 经济与认识论系统的缺环。
>
> ---
>
> # 6. TB-11 完整方案
>
> ## 6.1 核心对象一：EvidenceCapsule
>
> 当 evaluator 遇到：
>
> ```text
> MAX_TX exhausted
> timeout
> protocol collapse
> solver give-up
> repeated Lean rejection
> ```
>
> 系统必须把失败过程压缩成一个 CAS artifact。
>
> ### EvidenceCapsule 结构
>
> ```rust
> pub struct EvidenceCapsule {
>     pub capsule_id: Cid,
>     pub run_id: RunId,
>     pub task_id: TaskId,
>     pub solver_agent: Option<AgentId>,
>
>     pub attempt_count: u64,
>     pub lean_error_count: u64,
>     pub sorry_block_count: u64,
>     pub protocol_parse_failure_count: u64,
>     pub partial_accept_count: u64,
>
>     pub started_at_round: u64,
>     pub ended_at_round: u64,
>     pub terminal_reason: ExhaustionReason,
>
>     pub public_summary: String,
>     pub evidence_manifest_cid: Cid,
>     pub compressed_log_cid: Cid,
>
>     pub privacy_policy: CapsulePrivacyPolicy,
>     pub sha256: Hash,
> }
> ```
>
> ### EvidenceCapsule 内容
>
> ```text
> compressed_log_cid:
>   .jsonl 或 .tar.gz，包含失败 attempts、Lean errors、parse failures 等。
>
> evidence_manifest_cid:
>   列出所有子文件 hash / cid。
>
> public_summary:
>   低污染摘要，例如：
>   "132 attempts; 73 Lean failures; 14 sorry-blocks; 26 parse failures; no accepted proof."
> ```
>
> ### 屏蔽规则
>
> ```text
> public_summary 可以进入 dashboard / broadcast；
> compressed raw logs 只进入 audit view / authorized CAS；
> 不能默认进入 Agent read view。
> ```
>
> 这对齐宪法的"选择性屏蔽"：错误日志不能无差别污染后续 Agent 上下文；高频典型错误才应被抽象广播。
>
> ---
>
> ## 6.2 核心对象二：RunExhaustedTx / TaskBankruptcyTx
>
> DeepThink 提议 `TaskBankruptcyTx`。我建议把命名更精细化：
>
> ```text
> RunExhaustedTx:
>   某一次 solver/evaluator run 失败。
>   不一定关闭整个 task。
>
> TaskBankruptcyTx:
>   某个 task 在预算/期限/重试上限内失败，可作为未来 NO resolution anchor。
>
> TaskExpireTx:
>   task 到期，无 accepted proof，执行 escrow refund / capital liberation。
> ```
>
> ### RunExhaustedTx
>
> ```rust
> pub struct RunExhaustedTx {
>     pub tx_id: TxId,
>     pub parent_state_root: Hash,
>     pub task_id: TaskId,
>     pub run_id: RunId,
>     pub solver_agent: Option<AgentId>,
>     pub attempt_count: u64,
>     pub evidence_capsule_cid: Cid,
>     pub terminal_reason: ExhaustionReason,
>     pub system_signature: SystemSignature,
> }
> ```
>
> 作用：
>
> ```text
> 把一次失败运行锚定到 L4；
> 不释放 escrow；
> 不关闭 task；
> 不支付；
> 不 slash。
> ```
>
> ### TaskBankruptcyTx
>
> ```rust
> pub struct TaskBankruptcyTx {
>     pub tx_id: TxId,
>     pub parent_state_root: Hash,
>     pub task_id: TaskId,
>     pub evidence_capsule_cid: Cid,
>     pub bankruptcy_reason: BankruptcyReason,
>     pub system_signature: SystemSignature,
> }
> ```
>
> 作用：
>
> ```text
> 标记 task-level failure;
> 为未来 Short / NO resolution 提供死亡证明；
> 可触发 TaskExpireTx 或进入 challenge period。
> ```
>
> ### TaskExpireTx
>
> ```rust
> pub struct TaskExpireTx {
>     pub tx_id: TxId,
>     pub parent_state_root: Hash,
>     pub task_id: TaskId,
>     pub sponsor_agent: AgentId,
>     pub escrow_tx_id: TxId,
>     pub refund_amount: MicroCoin,
>     pub reason: ExpireReason,
>     pub system_signature: SystemSignature,
> }
> ```
>
> 作用：
>
> ```text
> 释放锁死的 bounty；
> escrow -> sponsor balance；
> CTF 守恒；
> dashboard 可见；
> replay 可重建。
> ```
>
> ---
>
> # 7. TB-11 的关键不变量
>
> ## 7.1 L4 不存 O(N) 错误明细
>
> ```text
> 132 attempts 不等于 132 L4 entries。
> ```
>
> L4 应记录：
>
> ```text
> RunExhaustedTx / TaskBankruptcyTx / TaskExpireTx
> ```
>
> CAS 存储：
>
> ```text
> 132 attempts 的 compressed evidence。
> ```
>
> 这就是：
>
> ```text
> O(1) chain cost
> O(N) auditability
> ```
>
> ---
>
> ## 7.2 L4.E 仍然只记录 transaction rejection
>
> 如果某个 transaction 被提交给 `submit_typed_tx`，但被 predicate / policy 拒绝：
>
> ```text
> 进入 L4.E。
> ```
>
> 如果只是 evaluator 内部的 Lean attempt，没有形成 tx：
>
> ```text
> 进入 EvidenceCapsule。
> ```
>
> 不要混淆：
>
> ```text
> transaction rejection
> vs
> internal failed attempt
> ```
>
> ---
>
> ## 7.3 Task failure 不等于 solver failure
>
> 这点很关键。
>
> ```text
> RunExhaustedTx:
>   某次 solver run 失败。
>
> TaskBankruptcyTx:
>   task-level 失败或长期无解。
>
> TaskExpireTx:
>   资金释放 / refund。
> ```
>
> 一个 solver run 失败，不应自动退还整个 task bounty。
> 只有满足：
>
> ```text
> deadline
> max_total_attempts
> manual sponsor expire
> system policy
> ```
>
> 才触发 `TaskExpireTx`。
>
> ---
>
> ## 7.4 Capital must flow
>
> DeepThink 指出 bounty 卡死的问题，这是对的。
>
> 正式规则：
>
> ```text
> escrow 不能无限期无状态锁死。
> ```
>
> 每个 TaskOpen / EscrowLock 必须有：
>
> ```text
> expiry policy
> max_run_count
> max_round
> refund policy
> ```
>
> 如果任务到期且没有 accepted proof：
>
> ```text
> TaskExpireTx refund sponsor
> ```
>
> 如果任务仍开放：
>
> ```text
> RunExhaustedTx 只记录失败，不退钱
> ```
>
> ---
>
> # 8. TB-11 ship gates
>
> TB-11 通过标准应是：
>
> ```text
> 1. MAX_TX exhausted run creates EvidenceCapsule in CAS.
> 2. EvidenceCapsule contains counts:
>    attempt_count, lean_error_count, sorry_block_count, parse_failure_count.
> 3. RunExhaustedTx is system-emitted and accepted in L4.
> 4. L4 entry references evidence_capsule_cid.
> 5. Dashboard renders exhausted / bankrupt run.
> 6. Raw logs are not in ordinary Agent read view.
> 7. TaskExpireTx refunds escrow after expiry policy.
> 8. Refund preserves CTF conservation.
> 9. No fake accepted WorkTx created.
> 10. Future ShortPosition can reference TaskBankruptcyTx / RunExhaustedTx.
> ```
>
> ---
>
> # 9. 后续路线重新排序
>
> 我现在的完整路线是：
>
> ```text
> TB-11:
>   Epistemic Exhaust & Capital Liberation
>   EvidenceCapsule + RunExhaustedTx + TaskExpireTx
>
> TB-12:
>   NodeMarket M0/M1
>   NodePosition / FirstLong / ChallengeShort
>   no trading
>
> TB-13:
>   CompleteSet + MarketSeedTx
>   1 locked Coin = YES + NO
>   no ghost liquidity
>
> TB-14:
>   CPMM Router / Mint-and-Swap
>   integer math
>   no f64
>   no automatic liquidity
>
> TB-15:
>   PriceIndex + Boltzmann Masking
>   price as scheduling signal, not truth
>
> TB-16:
>   Lamarckian Autopsy + Markov Loom
>   private autopsy capsules
>   Kelly risk policy suggestions
>   no raw global broadcast
>
> TB-17:
>   Pre-Real-World PCP Gate
>   domain oracle requirements
>   challenge court
>   time-locked settlement
>   human escalation
> ```
>
> 这吸收了 DeepThink 的"真实世界前必须处理失败坟场"的批判，也保留了你想要的 Polymarket 方向。
>
> ---
>
> # 10. 对 DeepThink 的哪些观点我仍然不同意
>
> ## 10.1 不同意"拒绝 GPT-5.5 Pro 的所有建议"
>
> DeepThink 说要拒绝我关于 `ExternalizedAttemptEvent` 的建议。
> 我现在同意它对 **L4 bloat** 的批判，但不同意把所有 failed attempt 都完全排除出可审计结构。
>
> 我的修正是：
>
> ```text
> 不逐条进入 L4；
> 但必须进入 EvidenceCapsule；
> 并由 L4 的 RunExhaustedTx 锚定。
> ```
>
> 所以不是：
>
> ```text
> per-attempt L4
> ```
>
> 也不是：
>
> ```text
> private evaluator log only
> ```
>
> 而是：
>
> ```text
> CAS rollup + L4 anchor
> ```
>
> ---
>
> ## 10.2 不同意"TaskBankruptcyTx 可以直接替代 WorkTx 失败证据"
>
> 如果某个失败是 transaction-level 的：
>
> ```text
> WorkTx 被 submit_typed_tx 拒绝
> ```
>
> 它仍然应进入 L4.E。
>
> 如果失败是 evaluator 内部多次尝试但从未形成 WorkTx：
>
> ```text
> EvidenceCapsule + RunExhaustedTx
> ```
>
> 两者并存，不互相替代。
>
> ---
>
> ## 10.3 不同意过早把短空经济接入 TB-11
>
> DeepThink 说 TaskBankruptcyTx 是未来 shorting 机制需要的死亡证明，这对。
> 但 TB-11 不应立刻做：
>
> ```text
> ShortPosition payout
> NodeMarket resolve
> NO token redemption
> ```
>
> TB-11 只建立死亡证明。
> NodeMarket 仍在 TB-12+。
>
> ---
>
> # 11. 给 AI coder 的直接指令
>
> 可以直接发：
>
> ```text
> Architect ruling after first-principles review:
>
> 1. We accept the DeepThink critique that L4 ChainTape must not become debug stdout.
> 2. We reject per-Lean-error L4 entries.
> 3. We also reject leaving failed attempts only in evaluator.log.
> 4. Implement TB-11 as Epistemic Exhaust & Capital Liberation.
>
> TB-11 scope:
>
> A. EvidenceCapsule in CAS
>    - on MAX_TX exhausted / timeout / solver give-up
>    - package failed attempts, Lean errors, sorry-blocks, parse failures
>    - public_summary + compressed private evidence bundle
>    - raw logs are audit-only, not ordinary Agent read view
>
> B. RunExhaustedTx
>    - system-emitted L4 accepted tx
>    - references evidence_capsule_cid
>    - records attempt_count and terminal_reason
>    - does not refund escrow by itself
>    - does not slash
>    - does not settle reward
>
> C. TaskBankruptcyTx
>    - task-level failure marker
>    - future NO / Short settlement anchor
>    - system-emitted only
>
> D. TaskExpireTx
>    - system-emitted
>    - refunds escrow to sponsor when expiry policy is met
>    - preserves CTF conservation
>    - dashboard must render refund
>
> E. Dashboard
>    - show Exhausted / Bankrupt / Expired states
>    - show evidence_capsule_cid
>    - show refund status
>    - dashboard is materialized view only
>
> Forbidden:
>    - No NodeMarket trading
>    - No CompleteSet
>    - No CPMM
>    - No AMM
>    - No Short payout
>    - No Slash
>    - No per-attempt L4 spam
>    - No raw failure log broadcast
>    - No ghost liquidity
>
> Ship gates:
>    1. zeta / hard-fail run produces EvidenceCapsule.
>    2. RunExhaustedTx appears in L4.
>    3. TaskExpireTx refunds bounty after expiry.
>    4. Chain replay reconstructs refund.
>    5. Raw evidence is shielded.
>    6. Dashboard can regenerate from ChainTape + CAS.
> ```
>
> ---
>
> # 12. 我的最终新认知
>
> 我现在的独立判断是：
>
> > **TuringOS 的核心不是"把一切都上链"，而是"把每个状态事实上链，把每个高维证据入 CAS，把每个可广播结论压缩成白盒信号"。**
>
> 这是一条比我之前更清晰的原则：
>
> ```text
> State facts -> L4
> Rejected transactions -> L4.E
> High-dimensional evidence -> CAS
> Evidence summaries -> dashboard / Information Loom
> Future learning -> Markov capsule / Autopsy
> Market prices -> statistical signals
> Truth -> predicates / oracle / ChallengeCourt
> ```
>
> 这也给整个系统一个更稳定的分层：
>
> ```text
> ChainTape:
>   不可变状态事实。
>
> CAS:
>   高维证据与失败尸检。
>
> Dashboard:
>   可再生物化视图。
>
> Information Loom:
>   将 EvidenceCapsule 压缩成下一轮可读信号。
>
> NodeMarket:
>   在有死亡证明和成功证明后，才进行 long/short 定价。
> ```
>
> 所以，真正的后续路线不是：
>
> ```text
> 先 NodeMarket，再补失败证据。
> ```
>
> 而是：
>
> ```text
> 先埋葬失败，释放资本；
> 再允许市场围绕成功/失败进行定价。
> ```
>
> 这就是我吸收 DeepThink 批判后的新方案。

---

## §2 Reconstructibility check

This archive's `§1 Architect ruling text` block is reproducible from the original
user message by:
1. Stripping the leading wrapper (`ultrathink you are the best agents...`,
   archived in §0 above).
2. Indenting every line of the remainder with `> ` (markdown blockquote) for
   visual separation; without the `> ` prefix every line matches verbatim.
3. The internal code-fence triple-backticks are preserved (with their `text`
   / `rust` language tags).

Per `feedback_kolmogorov_compression`: "If unsure, ask: 'can the original be
exactly reconstructed from this archive?'" — yes, by stripping the `> ` prefix
from §1 and concatenating with §0.

---

## §3 Forward annotations (non-authoritative)

These are working interpretations for AI coder + future humans. They do NOT
modify the ruling.

### Atom mapping (charter draft)

| Architect §  | Charter atom         | Class | Restricted file? |
|---|---|---|---|
| §6.1         | Atom 1 EvidenceCapsule + cas schema | 1 | new module — not restricted |
| §6.2 RunExhausted | Atom 1 typed_tx variant            | 3 | typed_tx.rs (kernel-adjacent) |
| §6.2 TaskBankruptcy | Atom 1 typed_tx variant          | 3 | typed_tx.rs (kernel-adjacent) |
| §6.2 TaskExpire  | Atom 2 sequencer wire (was NotYetImplemented) | 3 | sequencer.rs (restricted) |
| §6.1 屏蔽规则     | Atom 3 CAS writer + privacy policy     | 1 | new module |
| §7.4             | Atom 4 evaluator emit + lean_market tick | 2 | evaluator.rs + lean_market.rs |
| §11 E            | Atom 5 audit_dashboard §12             | 1 | additive |
| Ship gate 1-6    | Atom 6 zeta-corpus smoke               | 1 | net-new evidence dir |

### Out of scope

Anything in §11 "Forbidden" is **structurally** forbidden from TB-11. Charter
must repeat each forbidden item in §5 to prevent scope creep.

### Naming reconciliation with existing code

The codebase pre-defines `TerminalSummaryTx` (q_state.rs / typed_tx.rs § 5)
and `TaskExpireTx` (typed_tx.rs § 5) with `NotYetImplemented` dispatch arms
(sequencer.rs:962-963). The architect's §6.2 schemas are similar but not
identical:

- `TerminalSummaryTx` ≈ architect's `RunExhaustedTx` semantically (both anchor
  a run-level outcome). Differences: pre-existing has
  `failure_class_histogram` + `last_logical_t`; architect adds
  `parent_state_root`, `solver_agent: Option<AgentId>`, `evidence_capsule_cid`,
  `terminal_reason: ExhaustionReason`. **Resolution**: extend
  `TerminalSummaryTx` additively with the 3 new fields (no production rows
  exist; safe per `feedback_no_retroactive_evidence_rewrite`); keep
  `failure_class_histogram` (richer than ExhaustionReason; the histogram
  IS the failure-class signal Art. IV requires); rename in code-comments
  as "RunExhaustedTx (architect §6.2 vocabulary) ≡ TerminalSummaryTx".

- `TaskExpireTx` is largely identical between code and architect §6.2; the
  pre-existing schema has `bounty_refunded` (matches architect's
  `refund_amount`); architect adds `sponsor_agent` + `escrow_tx_id` +
  `reason: ExpireReason`. **Resolution**: extend additively (no production
  rows exist on this variant either).

- `TaskBankruptcyTx` is genuinely net-new — no precursor in code.

### Memory implications

Updates required (per `feedback_kolmogorov_compression`):
- new `project_tb_11_epistemic_exhaust.md` pointing to this archive
- new `feedback_o1_chain_on_auditability.md` with the principle:
  state facts → L4, rejected tx → L4.E, evidence → CAS, anchor via
  system-emitted RunExhausted/TaskBankruptcy/TaskExpire

### Constitutional check

- **Art. 0** Turing fundamentalism: tape-canonical preserved (CAS Cid + L4
  anchor are both canonical bytes).
- **Art. I.1** 5-step compile loop: failure path now has a witness. Closes
  the negative-truth gap exposed by TB-13 PREVIEW.
- **Art. II.2.1** entropy: dashboard now aware of failure-class distribution
  (pairwise diversity gains a reference cohort).
- **Art. III.4** no fake accepted: still holds — RunExhausted is
  system-emitted with system_signature; no agent-callable surface.
- **Art. IV** halt_reason taxonomy: `RunOutcome::MaxTxExhausted` already
  enumerated; EvidenceCapsule.terminal_reason maps 1:1.
- **Art. V** Anti-Oreo: explicitly preserved (architect §6.2 "system-emitted
  only" annotations on all 3 new variants).

No constitution.md edit required.
