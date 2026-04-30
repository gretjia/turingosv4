# Architect Directive — TB-4 P3 RSP-2 Scope & Decisions (2026-04-30)

**Received**: 2026-04-30 session (post TB-3 SHIPPED `e99b158`; post TB-4 charter v1 draft `handover/tracer_bullets/TB-4_charter_2026-04-30.md`).
**Mode**: chat directive from user `gretjia` (human architect, sole sudo authority); delivered as a TB-3 phase-audit + TB-4 decision report.
**Status**: ARCHIVED (per architect-ingest § 1). Authority for TB-4 charter v2 rewrite. Decisions Q1–Q7 + 防漂移 clauses 8.1–8.4 binding on AI coder.
**Impact category**: TB-scoped operational directive, NOT Layer-1 axiom amendment. Closes 7 open scope-review questions in TB-4 charter v1 § 10.

---

## 1. TB-3 phase-audit conclusion (verbatim)

> TB-3 可以认定为合格 shipped。
>
> 接受 TB-3 claim：
>
> - P3 RSP-1 formal transaction surface
> - TaskOpenTx + EscrowLockTx
> - WorkTx inline YES stake
> - TaskMarket TaskId-keyed
> - no ghost liquidity
> - no bridge resurrection
>
> 不要把 TB-3 解释成：
>
> - RSP-2 verifier/challenger 已完成
> - challenge window 已完成
> - slash 已完成
> - settlement 已完成
> - reputation 已完成
>
> 这些都仍是后续工作。

## 2. WP-canonical reconciliation (continued binding rule)

> Roadmap 的 logical transaction list 不等于 Rust TypedTx variant list。
>
> ```text
> Roadmap yes_stake_tx
>   = WorkTx.stake 的语义角色
>   ≠ 一个独立 YesStakeTx variant
> ```
>
> 这个裁决正确，必须继续沿用到 TB-4。

Memory `feedback_wp_vs_roadmap_reconciliation` already codifies this rule from TB-3. The directive re-affirms it for TB-4 explicitly:

```text
Roadmap logical no_stake_tx     = ChallengeTx.stake 的语义角色   ≠ NoStakeTx variant
Roadmap logical verifier bond   = VerifyTx.bond 的语义角色       ≠ VerifierBondTx variant
```

## 3. TB-4 direction approval

> TB-4 应该进入：**P3 RSP-2 Verifier + Challenge formal surface**。
>
> 理由成立：
> - P3:4 challenge_tx 必须 lock NO stake 仍未完成；
> - P3:9 失败 Solver slash 仍未完成，但 slash 属 RSP-3；
> - RSP-2 是 RSP-1 后自然下一步。
>
> 但 TB-4 必须严格收窄：
>
> ```text
> TB-4 做：
>   VerifyTx bond admission
>   ChallengeTx NO stake admission
>   target WorkTx liveness check
>   challenge window anchor
>
> TB-4 不做：
>   slash execution
>   challenge resolution
>   settlement
>   payout
>   attribution
>   reputation mutation
> ```

## 4. Verdicts on TB-4 charter v1 § 10 OPEN questions

### Q1 — per-(verifier, target) idempotency dedup

**DEFER.** TB-4 不做 hard dedup.

> 原因：
> - RSP-2 只建立 verifier/challenger admission surface；
> - dedup 本质是 anti-spam / reputation / verifier policy；
> - 这些属于 RSP-3/RSP-4/P4。
>
> TB-4 只要保证：
> - 每个 VerifyTx 有唯一 tx_id；
> - 每个 VerifyTx.bond 不能复用；
> - 每次验证都产生独立账本事件；
> - 未来 settlement/reputation 可以处理重复验证。
>
> 不要现在引入：one verifier can only verify one target once
>
> 因为这可能误杀合法场景，例如 verifier 先 Doubt 后 Confirm，或不同 evidence 版本下再次验证。

### Q2 — VerifyTx / ChallengeTx schema bump (parent_state_root)

**ACCEPT Option A**: schema bump.

> 当前 WorkTx 已有 parent_state_root，这是状态转移的基本锚点。VerifyTx / ChallengeTx 如果没有 parent root，会导致验证/挑战绑定到隐式当前状态，而不是显式状态根。这会增加 replay 歧义。
>
> 同步要求：
> - signing payload 更新；
> - canonical digest golden tests 更新；
> - fixtures 更新；
> - dispatch_transition stale-parent check 更新。
>
> 这是 STEP_B 级别 schema change，必须走 STEP_B protocol。

### Q3 — TargetWorkInactive vs TargetNotVerifiable

**新增 `TargetWorkInactive`，不要折叠。** 区分三类：

```text
TargetNotFound:
  target_work_tx 不存在。

TargetWorkInactive:
  target 存在，但不处于可验证/可挑战状态。

TargetNotVerifiable:
  target 类型或结构不适合验证，比如不是 WorkTx。
```

> 这符合宪法的信号量化原则：错误类型应成为明确、低维、可统计的布尔/分类信号。
> 不要把不同失败语义塞进一个模糊的 PolicyViolation。

### Q4 — Multi-challenger representability

**MUST be explicit test.**

> TB-4 不做 challenge resolution，但必须证明系统能表达：
> - 多个 Challenger 针对同一个 target_work_tx 提交挑战；
> - 每个挑战有独立 tx_id；
> - 每个挑战有独立 stake；
> - 每个挑战有独立 ChallengeCase；
> - challenge_cases_t 可以保留多条挑战。
>
> 不要让 ChallengeCasesIndex 隐式变成：target_work_tx -> single challenge.
>
> 否则 RSP-3 settlement 时会被卡死。
>
> 最低测试：
> - same target WorkTx
> - two different ChallengeTx
> - two different challenger_agent
> - both accepted
> - challenge_cases_t has 2 entries
> - target_work_tx unchanged
> - no slash executed

### Q5 — Audit mode

**Option A with NARROWED dual audit, not full audit.**

> TB-3 跳过 Phase-0 + Phase-1c dual audit 是一次被授权的例外，不应沉淀为默认流程。
>
> TB-4 涉及：
> - VerifyTx / ChallengeTx schema bump
> - ChallengeCase schema
> - verifier bond
> - challenger stake
> - challenge window anchor
>
> 这是经济裁决路径，不是普通 instrumentation。因此建议：
>
> ```text
> charter 阶段：
>   先不审，先让用户 review。
>
> STEP_B Phase-0 后：
>   做一次窄化 dual audit，审 charter + schema plan + forbidden list。
>
> ship 前：
>   Codex implementation audit + Gemini architecture audit。
> ```
>
> 不需要像大型白皮书那样 150KB prompt，但不能完全跳过。

### Q6 — ReputationsIndex touch

**DEFER.** TB-4 不动 Reputation.

> 都依赖 challenge resolution / settlement / attribution，这些属于 RSP-3 / RSP-4 之后。
>
> TB-4 只能记录足够证据，让未来 reputation system 能消费：
> - VerifyTx accepted
> - ChallengeTx accepted
> - target_work_tx
> - bond/stake
> - counterexample_cid
> - opened_at_round
> - verdict

### Q7 — EmptyCounterexample variant

**KEEP.**

> EmptyCounterexample 是重要的 L4.E / P4 信号。
> 它和 MalformedPayload、PolicyViolation 不同。
>
> ```text
> EmptyCounterexample:
>   ChallengeTx 结构合法，但没有有效 counterexample_cid / evidence。
>
> MalformedPayload:
>   tx 无法 decode 或字段结构不合法。
>
> PolicyViolation:
>   违反策略，但不是证据缺失。
> ```
>
> P4 Information Loom 后续需要聚类"垃圾挑战 / 无证挑战"，所以这个信号不能丢。

## 5. TB-4 anti-drift clauses (binding)

### 5.1 RSP-2 ≠ RSP-3

> 一看到 challenge，就很容易顺手写 slash。不要这样做。
>
> ```text
> RSP-2 = admission + bond/stake + window anchor
> RSP-3 = challenge window + slash + provisional/final
> RSP-4 = Contribution DAG + settlement_tx
> ```
>
> 这是三层，不可合并。

### 5.2 VerifyTx is signal+stake, not subjective judge

> Verifier 的任务不是说"我觉得好"，而是提交结构化 verdict 与 bond。
>
> ```text
> VerifyVerdict::Confirm
> VerifyVerdict::Doubt
> ```
>
> 可以保留，但它只是 signed/economic signal，不是最终真理。最终仍由 predicates / challenge window / settlement 决定。
>
> 宪法里顶层白盒不做主观审阅，而是运行确定性统计与谓词。

### 5.3 ChallengeTx ≠ slash_tx

> ChallengeTx 只是提交反例与 NO stake。
>
> ```text
> challenge submitted ≠ challenge upheld
> challenge submitted ≠ solver slashed
> challenge submitted ≠ verifier slashed
> ```
>
> slash 必须等 RSP-3 的 challenge resolution。

### 5.4 No P6 capability metric in TB-4 ship gate

> TB-4 不能碰：
>
> ```text
> h_vppu
> MiniF2F capability metric
> prompt_context_hash
> PPUT
> MetaTape
> ArchitectAI
> ```
>
> TB-4 是 P3 RSP-2。能力 smoke 可以作为不阻塞 evidence，但不能进 ship gate。

## 6. WorkTx state machine — defer

> AI coder 提到：Pending → Accepted → ProvisionalAccepted → Finalized.
>
> 我的裁决：**TB-4 不给 WorkTx 加 status 字段。**
>
> 当前 TypedTx 里 TxStatus 已明确是 runtime bookkeeping，不进入 wire bytes；WorkTx 本身不应被改造成状态机对象。
> TB-4 的 target liveness 可以通过 accepted WorkTx 的 stake/task binding 与 ledger index 推导。
>
> 不要在 TB-4 引入：
> - WorkTx.status
> - ProvisionalAccepted field
> - Finalized field
>
> 这会把 RSP-3/RSP-4 的结算状态提前拖进 RSP-2。

## 7. Recommended atom sequence (binding)

```text
Atom 0  — Charter v2 + PLAN sync (doc only)
Atom 1  — STEP_B Phase-0 preflight (typed_tx.rs / q_state.rs / sequencer.rs)
Atom 2  — ABI bump (parent_state_root on VerifyTx + ChallengeTx; signing payload + golden digest sync)
Atom 3  — ChallengeCase schema (target_work_tx; opened_at_round if not already enough; serde-default)
Atom 4  — Verify dispatch arm (target exists / active / bond > 0 / balance / debit→stakes_t insert; no reputation, no slash)
Atom 5  — Challenge dispatch arm (target / active / stake > 0 / counterexample non-empty / debit→challenge_cases_t insert; opened_at_round; no slash, no settlement)
Atom 6  — Multi-challenger + window anchor tests
Atom 7  — Replay / property / no-drift tests (total supply conserved; no double counting; no slash side effect; no NoStakeTx / VerifierBondTx variant in src)
Atom 8  — Ship audit (Codex implementation audit + Gemini architecture audit; small prompt; TB-4 scope only)
```

## 8. Final instruction (verbatim)

```text
我 review 了 TB-4 charter v1。方向正确，但按以下裁决改成 v2，
先不要写 production code，不要 launch STEP_B。

Final decisions:

1.  TB-4 phase_id = P3 RSP-2.
2.  使用现有 VerifyTx / ChallengeTx，不新增 VerifierBondTx / NoStakeTx.
3.  Roadmap no_stake_tx = ChallengeTx.stake 的语义角色.
4.  VerifyTx.bond inline，ChallengeTx.stake inline.
5.  VerifyTx + ChallengeTx 都加 parent_state_root，走 schema bump.
6.  新增 TargetWorkInactive，不折叠进 TargetNotVerifiable.
7.  Multi-challenger representability 必须显式测试.
8.  ReputationsIndex 不动，defer 到 RSP-3/RSP-4/P4.
9.  EmptyCounterexample variant 保留.
10. ChallengeCase 加 target_work_tx；opened_at_round 作为 challenge-window anchor.
11. TB-4 只 open/anchor challenge window，不 close，不 resolve.
12. TB-4 不执行 slash，不 settlement，不 payout，不 attribution.
13. WorkTx 不加 status 字段；Pending/Provisional/Finalized 状态机 defer 到 RSP-3.
14. 审计模式恢复为窄化 dual audit：STEP_B Phase-0 后审 charter+diff plan，ship 前审 implementation.
15. 重写 TB-4 charter v2 后等我 review，再进入 STEP_B Phase-0.
```

## 9. Cross-references

- TB-4 charter v1 (superseded by v2 incorporating this directive): `handover/tracer_bullets/TB-4_charter_2026-04-30.md`
- TB-3 charter (precedent shape): `handover/tracer_bullets/TB-3_charter_2026-04-30.md`
- TB-3 self-audit (precedent for charter-stage decisions): `handover/audits/RECURSIVE_AUDIT_TB_3_2026-04-30.md`
- 9-phase roadmap directive: `handover/directives/2026-04-29_9_phase_roadmap.md`
- WP economic § 18 + § 19: `handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md`
- WP architecture § 1.3 / § 5: `handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md`
- WP-vs-Roadmap reconciliation memory rule: `feedback_wp_vs_roadmap_reconciliation`
