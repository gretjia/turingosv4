# Architect Directive — TB-5 VETO Redesign (2026-04-30)

**Received**: 2026-04-30 (post-dual-audit round 1 verdict; Codex VETO landed; Gemini degraded round 1).
**Mode**: chat directive from user `gretjia` (human architect, sole sudo authority); delivered as a "phased process audit + TB-5 decision" comprehensive ruling.
**Status**: ARCHIVED (per architect-ingest § 1). **AUTHORITATIVE for TB-5 charter v2 rewrite.** Closes Q1-Q6 from prior session + supplements 2026-04-30_TB4_directive.md.
**Impact category**: TB-scoped operational directive + **constitutional ingress-architecture clarification** (not Layer-1 axiom amendment, but operational fixation of the existing Anti-Oreo / Agent ≠ direct state writer axiom into a concrete two-channel ingress design).

---

## 1. Top-line ruling (verbatim, condensed)

> 接受 Codex 的 Part B VETO。TB-5 charter v1 不能进入 STEP_B，也不能进入实现。必须先重写为 v2。
>
> 原因不是"小 bug"，而是它触碰了 TuringOS 的第一性边界：
>
> - Agent 不能伪造 system-emitted economic mutator。
> - Agent 不能通过 public submit path 触发 refund / release / resolve。
> - System tx 必须由系统白盒通道发出，并被系统签名或系统内部权限证明约束。
>
> 这正是反奥利奥架构的核心：中层黑盒只能提出候选状态转移，不能直接拥有系统裁决权。

## 2. TB-4 status review (no change to ship)

> TB-4 shipped 本身没有问题。它完成了 RSP-2 的 admission surface：
>
> - `ChallengeTx` debits balance → `challenge_cases_t.bond`，记录 `opened_at_round` 和 `target_work_tx`
> - `VerifyTx` debits balance → `stakes_t[verify.tx_id]`
> - slash / closure / settlement 仍明确 out-of-scope
>
> TB-4 后续补跑的中等难度真题测试也有价值：5 题 n1 / MAX_TX=30，4/5 solved，`mathd_algebra_148` 通过 23 笔交易收敛到复合策略 `rw [h₀ 2] at h₁; nlinarith`，说明 elevated MAX_TX 在 runtime 下确实能流动；`amc12a_2003_p5` 干净耗尽预算，无 false-positive、无 crash。
>
> 但这批真题测试只证明：ABI / serde / capability 兼容性 + runtime spine 未破坏 prompt pipeline + proof artifact 可重验。
>
> 它**不证明**：RSP-2 admission surface 已被 PPUT evaluator 路径触达 / `ChallengeResolveTx` 安全 / system-emitted tx 已有 live signature gate / slash / settlement 已可用。

## 3. TB-5 v1 problem (the VETO substance)

> Codex 抓到的问题是致命的：
>
> > 如果 `ChallengeResolveTx` 是 system-emitted，但 live `dispatch_transition` / `apply_one` 没有验证 system signature 或系统通道权限，那么任何 agent 都可以伪造 `ChallengeResolveTx { Released }`，提前释放 challenge bond。
>
> 这不是 ordinary CHALLENGE，而是 VETO 级别。
>
> AI coder 已经复核到：`sequencer.rs` 整个 live dispatch 路径没有 signature verification 调用；`PinnedSystemPubkeys` 验证只在 `replay_full_transition` 用；live dispatch 信任 wire bytes。这意味着风险不止 `ChallengeResolveTx`，还包括 agent-signed 的 `WorkTx / VerifyTx / ChallengeTx`。不过 TB-5 的即时阻断点是 system-emitted economic mutator。

## 4. Resolution of Q1–Q6 (binding for TB-5 v2)

### Q1 — 接受 Codex VETO？

**接受。** Codex 的 Part B VETO 是实质性、宪法级 VETO，不是审计风格分歧。理由：

```text
ChallengeResolveTx 是 system-emitted economic mutator。
如果 public submit() 能接受它，且 live dispatch 不验证 system signature，
Agent 就可以伪造系统裁决。
```

违反:
- Agent ≠ direct state writer (反奥利奥)
- system-emitted tx 必须由系统白盒通道产生
- 经济状态变更必须经过硬约束

也直接违背白皮书 / Manual 中的经济结算原则：Agent 只能提交贡献、验证、挑战；真正的结算、释放、裁决由白盒 settlement / ChallengeCourt / EscrowVault 执行，而不是由 Agent 自己声称。

**结论**: TB-5 charter v1 = VETO，不能进 STEP_B，不能实现，必须重写 v2。

### Q2 — B2 redesign 选 Option 1 还是 Option 2？

**选 Option 1 — 两通道 ingress 架构。**

```text
Sequencer::submit_agent_tx(...)
  只接受 agent-submitted variants:
    WorkTx
    VerifyTx
    ChallengeTx
    TaskOpenTx
    EscrowLockTx
    ReuseTx if applicable

Sequencer::emit_system_tx(...)
  只由系统内部调用，接受 system-emitted variants:
    ChallengeResolveTx
    FinalizeRewardTx
    TaskExpireTx
    TerminalSummaryTx
    future SlashTx / SettlementTx
```

**不要让 public `submit()` 接受所有 `TypedTx` variant。**

理由：

> Option 1 最符合反奥利奥：中层黑盒 Agent 只能提交 agent tx；底层/顶层白盒系统才能 emit system tx。system tx 不来自 Agent 的自由输入；由系统内部构造；由 system keypair 签名；或由内部 capability 证明生成。
>
> Option 2 的 pre-dispatch verifier 也能修 bug，但会保留一个混合入口 `submit(TypedTx)`，然后靠签名 verifier 拦截。这个做法短期省 API，但架构上更脆弱：它让 Agent 仍然可以"提交一个系统交易，只是希望 verifier 拦下"。

最终写法：

```text
Agent-submitted ingress:
  submit_agent_tx(agent_envelope)

System-emitted ingress:
  emit_system_tx(system_command)
```

如果为了兼容短期代码，还保留 `submit(tx)`，必须在 TB-5 v2 中把它语义改清楚：

```text
submit(tx) = agent ingress only
system variants via submit(tx) => reject before dispatch
```

### Q3 — Agent-tx 签名验证一起修吗？

**不在 TB-5 v2 一次性全修，但必须新增 TB-5.0 "Ingress Authentication Barrier"。**

避免两个极端:

**不可接受的窄修**: 只修 ChallengeResolveTx system signature，对 WorkTx / VerifyTx / ChallengeTx 的 agent signature 完全不登记 debt。短视。

**也不建议的过宽修**: TB-5 同时实现完整 AgentRegistry + agent pubkey registry + 所有 agent tx signature verification。会把 TB-5 从 RSP-3.1 变成 P0/P1/P3 混合大爆炸。

**中间方案** (TB-5 v2 binding):

```text
TB-5.0 Ingress Authentication Barrier 目标:
1. system-emitted variants cannot enter agent submit path.
2. ChallengeResolveTx cannot be accepted unless emitted through emit_system_tx.
3. emit_system_tx attaches / verifies system_signature.
4. agent tx signature verification remains documented debt unless AgentRegistry already exists.
5. no new system economic mutator may be added without system-only ingress.
```

新增 forbidden CI:
- `agent_submit_rejects_system_variants`
- `challenge_resolve_via_agent_submit_rejected`
- `system_emit_challenge_resolve_requires_system_signature`

Agent signature verification 应该成为下一阶段独立 TB:
- **建议命名**: `TB-5A or TB-6-pre: AgentRegistry + Live Agent Signature Verification`

但不要让 TB-5 v2 在没有 AgentRegistry 的情况下假装"agent sig 已完整验证"。

### Q4 — TB-5 v2 audit mode A 还是 B？

**Option A — dual external audit。**

TB-3 / TB-4 的 audit-skip / self-audit / 真题烟测不能继续常态化，尤其不能用于 system-emitted economic mutator。

TB-5 v2 引入：ChallengeResolveTx + system-emitted economic mutation + bond release + possible future slash path。这比 TB-4 的 verifier/challenger admission 更敏感。

**必须恢复**:
- STEP_B Phase-0 后：Codex + Gemini / strategic dual audit charter + diff plan
- ship 前：implementation audit

如果 Gemini 继续 capacity 不可用，可以降级，但要在 verdict 文件中明确标记:

```text
Gemini degraded / not strategic-tier
Codex full-fidelity VETO controls
```

不要把 degraded Gemini 当作完整战略审计。

### Q5 — 等 Gemini 还是先动？

**先动 charter v2 redesign，不等 Gemini。**

```text
Codex VETO 已经被 AI coder 独立复核为真实；
按 VETO > CHALLENGE > PASS，Gemini 即使 PASS 也不能推翻；
Gemini 只能提供补充发现，不能改变必须重写 charter 的事实。
```

执行策略:
- 立即重写 TB-5 charter v2
- Gemini 若后续返回有效结果，作为 v2 review 补充
- 不等 Gemini 才开始修 charter

### Q6 — Gemini round 2 策略？

**选 (a) — 接受 Codex full-fidelity + Gemini degraded 为 round 1 记录；不等 capacity；进入 charter v2 redesign。**

但必须写入 verdict:

```text
Gemini audit degraded:
  fallback model
  low file-read coverage
  did not catch B2
  cannot override Codex VETO
```

如果后续要做 TB-5 ship audit，建议再跑一次 strategic-tier Gemini / 或备用强模型，但不是现在阻塞 charter v2。

### A1 doc patch 授权

**授权。** 可以并行做：TB-4 charter `q.logical_t` → `q.q_t.current_round`。这是文档一致性修复，不涉及生产行为。

**但**: `A1 doc patch 不能混入 TB-5 v2 redesign commit。分开 commit。`

---

## 5. TB-5 v2 重新规划（binding structure）

### 5.1 新名称

> 建议改为：**"TB-5 — System-Emitted Resolution Gate + Challenge Bond Release"**
>
> 不要一开始就叫 "Challenge Resolve"，因为真正的 challenge resolve 会牵涉：window close / counterexample evaluation / upheld / rejected / slash / verifier penalty / solver stake / provisional/final。

### 5.2 拆成两个层次

```text
TB-5.0 — System Ingress Barrier
  目标:
    system-emitted tx 不能走 agent submit
    agent submit path 只接收 agent tx

TB-5.1 — Released-only challenge bond release
  目标:
    Released:
      challenge_cases_t.bond -> balances_t[challenger]
      challenge case status -> Released or moved to resolved index
      total CTF conserved
    UpheldDeferred:
      marker only, no slash, no payout
```

### 5.3 Allowed (TB-5 v2 binding)

```text
ChallengeResolveTx schema
system-emitted ingress only
system_signature required
agent submit path rejects ChallengeResolveTx
Released resolution:
  challenge_cases_t.bond -> balances_t[challenger]
  challenge case marked resolved or removed with audit trail
UpheldDeferred:
  marker only, no slash, no payout
CTF conservation
replay compatibility
```

### 5.4 Forbidden (TB-5 v2 binding)

```text
slash execution
verifier bond release
solver stake release
FinalizeRewardTx
SettlementTx
ReputationUpdateTx
Contribution DAG
automatic round tick
automatic window close
agent-submitted ChallengeResolveTx
system tx via public submit path
```

---

## 6. ChallengeResolveTx 结构裁决

### 6.1 ChallengeResolveTx 可以作为 first-class TypedTx variant

> 这里我同意 AI coder v1 的方向：`ChallengeResolveTx` 可以是 first-class system-emitted variant。
>
> 这不同于 TB-3 中的 `YesStakeTx` / TB-4 中的 `NoStakeTx`。
>
> ```text
> Yes stake = WorkTx.stake inline
> No stake = ChallengeTx.stake inline
> Verifier bond = VerifyTx.bond inline
> Challenge resolution = 系统裁决事件，不是 agent tx 内联字段
> ```
>
> 所以 `ChallengeResolveTx` 是合理 variant。

### 6.2 但它必须是 system-only

```rust
ChallengeResolveTx {
    tx_id,
    parent_state_root,
    target_challenge_tx_id,
    resolution: ChallengeResolution,
    epoch,
    system_signature,
    timestamp_logical
}
```

并且：

```text
submit_agent_tx(ChallengeResolveTx) -> rejected
emit_system_tx(ChallengeResolveTx) -> verify system signature/capability -> dispatch
```

`system_signature` 不能只是字段，必须有 live gate。

---

## 7. TB-5 v2 对 7 个开放问题的裁决（覆盖 charter v1 § 10）

### Q1: `challenge_window_length` 来源

**本 TB 不做 window close，所以不需要用于裁决。** 可以在 schema 中加 `TaskMarketEntry.challenge_window_length` default = 10，但 TB-5.1 不用它关闭窗口。**不要在 TB-5 中写 `if current_round - opened_at_round >= challenge_window_length`** — 那是 TB-6 / RSP-3.2。

### Q2: `accepted_at_round` 给 Solver 还是 Verifier

**本 TB 不要引入过多 round fields。** 如果必须加，只给需要窗口计算的 entry 加（`ChallengeCase.opened_at_round` 已有）。`StakeEntry.accepted_at_round` 是 future-use，**可以 defer**。**不要为了 TB-6 预先污染 TB-5 schema**，除非 TB-5 的 tests 需要它。

### Q3: 保留 `SystemSignatureForbiddenAtAgentSubmit` 错误？

**保留，但命名应更直接**: `SystemTxForbiddenOnAgentIngress` 或 `SystemVariantRejectedAtAgentSubmit`。

这是 TB-5.0 的核心 error，**不是 over-engineering**。如果 `ChallengeResolveTx` system-only，那么 agent submit path 遇到它必须有明确 rejection class / transition error。

### Q4: ChallengeResolution enum 还是 bool？

**enum**:

```rust
enum ChallengeResolution {
    Released,
    UpheldDeferred,
}
```

不要用 bool — bool 会让含义漂移 (`true = upheld?` / `true = released?` / `true = slash?`)。enum 更符合白盒信号量化。

### Q5: audit mode A vs B?

**A**. TB-5 v2 是 system-emitted economic mutator，不应沿用 TB-4 的 lighter audit。

### Q6: UpheldDeferred 是否删除 ChallengeCase？

**不删除。**

`UpheldDeferred` 含义:

```text
challenge 被系统标记为 upheld-like / pending slash path，
但 slash 与 settlement 尚未执行。
```

如果删除，TB-6 失去 slash 目标。

```text
Released:
  可以 remove challenge case，或 mark resolved=Released 后不再 active。
  但要保留 audit trail。

UpheldDeferred:
  保留 challenge case；
  标记 resolution_state = UpheldDeferred；
  不释放 bond；
  不 slash。
```

更安全的设计是不要直接删除，而是引入：

```text
ChallengeCase.status = Open | Released | UpheldDeferred
```

但如果 TB-5 不想改太大，可以保留 active map + resolved map。**关键**: 不要让 `UpheldDeferred` 丢失后续 slash 所需信息。

### Q7: 顺手加 ProvisionalAcceptTx？

**不加。**

ProvisionalAcceptTx 属于 RSP-3 更完整状态机，但 TB-5 是 system ingress + challenge bond release precursor。不要混入: ProvisionalAcceptTx / FinalizeRewardTx / SettlementTx / SlashTx。

---

## 8. 与宪法 / 白皮书 / 主研究方案对齐

### 8.1 宪法对齐

> 宪法核心是：顶层白盒做信号管理，中层黑盒生成候选，底层白盒做硬约束执行。
>
> TB-5 必须体现：system resolution 是白盒行为；Agent 不能通过黑盒输入伪造白盒裁决。
>
> 因此 system-emitted tx ingress gate **是宪法要求，不是工程偏好**。
>
> 宪法还规定：自然语言软约束无效，必须转成机器硬约束。所以不能只在 charter 写"Agent 不应提交 ChallengeResolveTx"。**必须写测试** `agent_submit_rejects_challenge_resolve_tx`。

### 8.2 白皮书 / Manual 对齐

> Manual 把系统分成：Top White (Signal & Policy Kernel) + Middle Black (Exploration Swarm) + Bottom White (Physical Discipline)。
>
> 底层白盒遇到权限不足、签名错误、不满足货币守恒时**必须 fail-closed**。
>
> TB-5 的 fail-closed 条款应是：
>
> ```text
> system tx on agent ingress => reject
> missing/invalid system signature => reject
> unknown challenge => reject
> already resolved => reject
> released without bond => reject
> CTF not conserved => reject
> ```

### 8.3 主研究方案对齐

> 主研究方案当前是 P0–P9 roadmapped TB methodology。
>
> TB-5 应继续遵守：phase_id = P3 / micro-version = RSP-3.0 / RSP-3.1 / no P6 capability expansion / no P5 MetaTape / no public chain / no settlement。
>
> **真题测试是 P6 evidence anchor，不是 roadmap sequencing driver。**

---

## 9. Execute order to AI coder (verbatim, binding)

```text
接受 Codex Part B VETO。TB-5 charter v1 不得进入 STEP_B，不得实现。

Proceed as follows:

1.  立即重写 TB-5 charter v2。
2.  接受 Option 1：两通道 ingress。
    - submit_agent_tx: agent variants only
    - emit_system_tx: system-emitted variants only
3.  ChallengeResolveTx 可以是 first-class TypedTx variant，但必须 system-only。
4.  Agent submit path 遇到 ChallengeResolveTx 必须 fail-closed，产生明确错误：
    SystemTxForbiddenOnAgentIngress。
5.  ChallengeResolveTx live dispatch 必须验证 system-only ingress / system_signature / system capability。
6.  不要用 public submit path 接收 ChallengeResolveTx。
7.  Agent-tx signature verification 记录为独立 debt，不在 TB-5 v2 一口气全修；但新增 TB-5.0 Ingress Authentication Barrier，至少阻断 system variants on agent ingress。
8.  TB-5 v2 audit mode 选 Option A：dual external audit。
9.  不等待 Gemini capacity；Codex VETO 已成立，Gemini degraded 只作为记录。
10. 授权 A1 doc patch：TB-4 charter 中 q.logical_t 改 q.q_t.current_round，单独 commit。
11. TB-5 v2 不做 slash、不做 settlement、不做 reputation、不做 ProvisionalAcceptTx、不做 FinalizeRewardTx。
12. UpheldDeferred 不删除 ChallengeCase，保留给后续 slash path。
13. Released 可以退还 challenger bond，但必须保持 audit trail，并通过 CTF conservation。
14. 所有 runtime monetary invariant 调用继续使用空 exemption list。
15. 不碰 P6 / h_vppu / MiniF2F 指标，除非作为非阻塞 smoke。
```

---

## 10. TB-5 v2 推荐结构（test plan binding）

### TB-5.0: System Ingress Barrier

测试:
- `agent_submit_rejects_challenge_resolve_tx`
- `agent_submit_rejects_finalize_reward_tx`
- `agent_submit_rejects_task_expire_tx`
- `emit_system_tx_accepts_system_variant_with_valid_signature`
- `emit_system_tx_rejects_missing_or_bad_signature`

### TB-5.1: ChallengeResolve Released-only

测试:
- `released_refunds_bond`
- `released_conserves_ctf`
- `released_cannot_run_twice`
- `released_unknown_challenge_rejected`
- `upheld_deferred_keeps_challenge_for_future_slash`
- `upheld_deferred_no_balance_mutation`

### TB-5.2: Replay / anti-drift

测试:
- `ChallengeResolveTx cannot be agent-submitted`
- `system variants forbidden on agent ingress`
- `no SlashTx introduced`
- `no SettlementTx introduced`
- `no ReputationUpdateTx introduced`
- `no ProvisionalAcceptTx introduced`
- `no P6 files touched`

---

## 11. 防漂移裁决（4 条 binding rules）

### 11.1 不要把"resolve"偷换成"judge"

> `ChallengeResolveTx` 在 TB-5 v2 中**不是**"系统判断 counterexample 真伪"的主观裁判。它只是 **system-emitted resolution signal**。TB-5 不做 counterexample evaluation。**不要让 LLM 或 Agent 当 judge。**

### 11.2 不要把 release 偷换成 settlement

> `Released` 只是把 challenger 的 bond 返还。它**不是**: solver 无罪最终确认 / verifier 无责最终确认 / task final settlement / reward payout。

### 11.3 不要把 UpheldDeferred 偷换成 slash

> `UpheldDeferred` 只能标记: challenge 进入后续 slash path。**不能在 TB-5 里扣 solver stake，不能扣 verifier bond。**

### 11.4 不要把系统签名当文档字段

> `system_signature` **不能只是 schema 上的字段**。它必须在 live ingress / dispatch 中被验证，或者由不可伪造的 internal `emit_system_tx` 构造。**否则它就是"签名形状的注释"。**

---

## 12. Final ruling

> TB-4 是正确的 RSP-2 admission surface。
>
> TB-5 v1 的方向有价值，但因为 **system-emitted tx ingress 未隔离 / live signature 未验证**，必须 VETO。
>
> 下一步不是实现 TB-5 v1，而是：
>
> ```text
> TB-5 v2 =
>   System Ingress Barrier
>   + Challenge Bond Release
>   + no slash
>   + no settlement
>   + no reputation
> ```
>
> 只要这个边界守住，TB-5 才能继续推进 RSP-3，而不会把 Agent 放进系统裁决通道里。

---

## 13. Cross-references

- TB-5 charter v1 (SUPERSEDED-by-VETO; commit `1b60237`): `handover/tracer_bullets/TB-5_charter_2026-04-30.md`
- TB-5 charter v2 (binding incarnation of this directive): same file (overwrite)
- Codex round 1 VETO verdict: `handover/audits/CODEX_TB_4_SHIP_TB_5_CHARTER_AUDIT_2026-04-30.md`
- Gemini round 1 degraded verdict: `handover/audits/GEMINI_TB_4_SHIP_TB_5_CHARTER_AUDIT_2026-04-30.md`
- Merged round 1 verdict: `handover/audits/DUAL_AUDIT_TB_4_SHIP_TB_5_CHARTER_VERDICT_2026-04-30.md`
- TB-4 ship directive: `handover/directives/2026-04-30_TB4_directive.md` (predecessor; supplemented by this)
- 9-phase roadmap directive: `handover/directives/2026-04-29_9_phase_roadmap.md` (root authority)
- WP architecture (Anti-Oreo + Top/Middle/Bottom White): `handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md`
- WP economic § 19 RSP-1 ChallengeCourt: `handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md`
- Constitution (Art V.1.3 + Top-White-Box authority): `constitution.md`
- audit prompt that produced round 1 verdicts: `handover/audits/EXTERNAL_AUDIT_PROMPT_TB_4_SHIP_TB_5_CHARTER_2026-04-30.md`
