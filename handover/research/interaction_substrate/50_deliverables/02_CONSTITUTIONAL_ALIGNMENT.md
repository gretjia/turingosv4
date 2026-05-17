# TISR Deliverable 02 — Constitutional Alignment

**本文档为 forward-bound research 提案；任何派生 TB 须独立 §8 ratification；本文档不构成已批准方案。**

**目的**: 每个 TISR 特性映射到 TuringOS 宪法条款 + Risk Class + 验收 SG; CONSTITUTION_EXECUTION_MATRIX 风格.

---

## 1. 总览

| TISR 特性 | 宪法条款 | Risk Class | 验收 SG | Status |
|---|---|---|---|---|
| Phase 6 CLI (37 subcommand) | Art. IV Boot + Art. II.1 广播 | 1-2 | SG-TISR-6.1.* | Forward-bound |
| Phase 7 Web Layer (from-scratch) | Art. III.4 屏蔽 + Art. V.1 元架构 | 1-2 + cas extension Class 4 候选 | SG-TISR-7.* | Forward-bound |
| UIEventCapsule (CAS schema) | Art. III.2 封装细节 | 3 | SG-TISR-CAP-UI.* | Forward-bound |
| A2AMessageCapsule (CAS schema) | Art. II.2 广播价格 + Art. III shielding | 3 | SG-TISR-CAP-A2A.* | Forward-bound |
| Designer/Curator/Auditor View | Art. V.2 宪法边界 | 1 | SG-TISR-VIEW.* | Forward-bound |
| Turing UI IR + Materializer | Art. III.2 封装 + Art. V.1.3 Veto | 1-2 | SG-TISR-IR.* | Forward-bound |
| Event Bridge + Policy Engine | Art. V.1 三权分立 | 1-2 | SG-TISR-BRIDGE.* | Forward-bound |
| AgentReputation (派生只读) | Art. III.4 Goodhart 屏蔽 | 1 | SG-TISR-REP.* | Forward-bound |
| Price Quotation (Tier 2 escrow) | Art. II.2 价格信号 + Law 2 投资 | 1-2 | SG-TISR-PRICE.* | Forward-bound |
| ProvenanceCapsule (human vs agent) | Art. IV Boot + Art. V.1.2 | 1 | SG-TISR-PROV.* | Forward-bound |
| Multimodal Artifact (2-tier dual-hash) | Art. III.2 封装 | 1-3 | SG-TISR-ART.* | Forward-bound |
| UIContentPredicate (PCP {0,1}) | Art. I.1 + Art. I.1.1 PCP | 1 | SG-TISR-PRED.* | Forward-bound |
| ForecastCapsule + ReflectionTx | Art. III.2 + Art. V.1.2 | 3 | SG-TISR-COG.* | Forward-bound (Phase 9+) |
| DID 桥 (did:turingos:) | Art. IV Boot + Art. V.2 | 1-2 | SG-TISR-DID.* | Forward-bound (Phase 8+) |

---

## 2. 详细 Alignment (按宪法条款分组)

### 2.1 Art. 0 (图灵机原教旨)

| 条款 | TISR 影响 | 维持/扩展 |
|---|---|---|
| 0.1 四要素映射 (tape/state/transition/halt) | 全 TISR 维持; ChainTape + CAS + state_root + replay 即四要素 | ✅ 维持 |
| 0.2 Tape Canonical (append-only) | UIEventCapsule + A2AMessageCapsule 是 CAS objects (引用 via Cid; 不破坏 append-only) | ✅ 维持 |
| 0.3 区块链化保留 (state_root immutable) | TISR 0 修改 state_root 规则 | ✅ 维持 |
| 0.4 Q_t 版本控制 | HEAD_t witness 6-field 不扩展; TISR 通过 cas_root 间接 anchor | ✅ 维持 |
| Laws 1+2 (Information free + Investment costs) | TISR 信息免费 (dashboard/view) + 投资花钱 (escrow/stake) 严格遵守 | ✅ 维持 |

### 2.2 Art. I (信号的量化)

| 条款 | TISR 影响 | SG |
|---|---|---|
| I.1 布尔信号 | UIContentPredicate {0,1} verdict 遵守 | SG-TISR-PRED.1: 谓词必须返回 boolean |
| I.1.1 PCP + 疑罪从无 | 正确 UI content 100% 通过; 错误高概率拒绝 | SG-TISR-PRED.2: 边界 fail-closed |
| I.2 统计信号 | A2A consensus aggregation (Bull/Bear count) 走统计路径 | SG-TISR-A2A.1: NoTradeReason 聚合 derived |

### 2.3 Art. II (信号的选择性广播)

| 条款 | TISR 影响 | SG |
|---|---|---|
| II.1 广播典型错误 | LibrarianDigest TypicalErrorCluster 已落地 (REAL-BCAST-1) | ✅ 现有 |
| II.2 广播价格信号 | TISR Tier 1-3 价格协议; price = signal not truth | SG-TISR-PRICE.1: price 不进 admission predicate |
| II.2.1 探索 vs 利用 | (REAL-12 EconomicJudgment 已涵盖; TISR 不扩展) | ✅ 现有 |

### 2.4 Art. III (信号的选择性屏蔽)

| 条款 | TISR 影响 | SG |
|---|---|---|
| III.1 屏蔽错误 | UIEventCapsule + A2AMessageCapsule 默认 AuditOnly privacy | SG-TISR-CAP.1: 默认 shielding |
| III.2 封装细节 | Turing UI IR 是 declarative; 不暴露 raw HTML/JS; PromptCapsule 已落地 | SG-TISR-IR.1: 不暴露 system prompt template raw |
| III.3 屏蔽相关性 | Role-scoped DerivedView (Designer/Curator/Auditor) 视图分离 | SG-TISR-VIEW.1: cross-role 不可见 |
| III.4 屏蔽 Goodhart | AgentReputation 派生只读; 不进 admission predicate | SG-TISR-REP.1: reputation 不影响 sequencer admission |

### 2.5 Art. IV (Boot/Replay)

| 条款 | TISR 影响 | SG |
|---|---|---|
| IV 整体 | Phase 6 CLI 走 emit_system_tx (系统裁决) + submit_agent_tx via Agent_user_0 (任务发起) | SG-TISR-BOOT.1: 0 新 ingress type |
| genesis_report 完整 | ProvenanceCapsule 扩展 audit trail (human-triggered vs agent-submitted) | SG-TISR-PROV.1: provenance 可重建 |
| replay 一致性 | TISR 新 capsules 全 Class-3+ (audit-level); replay 完整不破 | SG-TISR-REPLAY.1: replay_full_transition 通过 |

### 2.6 Art. V (Go Meta - 三权分立)

| 条款 | TISR 影响 | SG |
|---|---|---|
| V.1 三权分立 | Constitution + ArchitectAI (提议) + VetoAI (验证) 维持; TISR 不修改 | ✅ 维持 |
| V.1.2 ArchitectAI 提议 | TISR 所有新 capsule schemas 是 forward-bound proposals; 待架构师 §8 | ✅ 维持 |
| V.1.3 VetoAI 验证 | UIVetoRequest interface (Track E Q3) 扩展 VetoAI 角色到 UI policy; non-binding signal | SG-TISR-VETO.1: VetoAI Reject 不阻断 policy engine |
| V.2 宪法边界 | TISR 0 修改 constitution; 0 Class 4 surface 修改 | ✅ 维持 |
| V.3 宪法修订日志 | TISR 是 Class 0 docs; 不进入 constitution amendment | ✅ 维持 |

---

## 3. 经济不变量 (CLAUDE.md §13)

| 不变量 | TISR 验证 |
|---|---|
| 1 Coin = 1 YES + 1 NO | ✅ 不破坏 (UI artifacts 不 mint coin; reputation 是派生值不是 coin) |
| Information is free | ✅ Dashboard / view / query CAS = 0 cost |
| Investment costs money | ✅ TaskOpen escrow / BuyWithCoinRouter 等保持 cost; A2A optional escrow |
| 整数算术 / 无 f64 money path | ✅ TISR 0 引入 f64; AgentReputation 用 i64 basis points |
| on_init 唯一 base-Coin mint | ✅ TISR 0 新 minting path; UI artifact reuse → reputation (非 coin) |

---

## 4. 真问题 Witness Enforcement

按 `feedback_real_problems_not_designed`:

| TISR 特性 | Real-Problem Witness 要求 | Phase | 来源 |
|---|---|---|---|
| Phase 6 CLI happy path | ≥ 1 完整 12-step run; PROCEED audit | 6 | CLI-A §2.2 |
| UIEventCapsule | ≥ 3 不同 content_class 提交 | 7 | Track E Q5 |
| A2AMessageCapsule | ≥ 5 有效 + 2 reject judgment | 7 | Track E Q5 |
| ProvenanceCapsule | ≥ 1 Human-triggered case | 6 | Track E Q5 |
| Multimodal artifact | ≥ 3 文件类型 (.lean/.py/.json/.png/.mp3) | 7 | Track E Q5 |
| Price Quotation | ≥ 1 accepted + ≥ 1 expired | 8 | Track D Q3 |
| N≥16 涌现验证 | 16-32 agent batch run + 涌现现象记录 | 9 | Track J Q4 |

**Witness 存储**: `handover/evidence/stage_phase{N}_*`

---

## 5. Class 4 Forward-Bound Candidates (需独立 §8)

| 候选 | 来源 | Phase | 风险 |
|---|---|---|---|
| cas/schema.rs ObjectType +5 variants | Track B + Architecture Integration | 7 | 向后兼容 (serde default); 仍需架构师 §8 |
| AgentProposedTaskOpen typed_tx variant | Track A Q2 | 9+ AGI | agent 自主 propose 任务; 改 admission barrier |
| AgentMarketSeeding typed_tx variant | Track A Q2 | 9+ AGI | 同上 |
| DirectSwapTx typed_tx variant | Track D | 8+ | 多 agent 原子 swap; covenant 验证 |
| HumanSignature type | Track A Q5 + Track E Q2 | 9+ | 人类 PKI; 改 typed_tx 二元 signature |
| New AgentRole variants (Designer/Curator/Editor/Auditor) | Track D Q2 | 9+ | 改 enum + role classifier |
| ReputationScorePolicyFilter (predicate, admission-bound) | Track E §9 | 9+ AGI | reputation → admission; 触发 Goodhart 风险 |

**总**: 7 Class 4 候选, 全部 forward-bound, 0 个进入 TISR Phase 6-7 实施.

---

## 6. Constitution Execution Matrix (TISR-specific rows)

按 `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` 格式 (TISR Phase 6+ 实施时填):

| Clause | Code Surface | Executable Test | Smoke/Evidence | Status | Kill Condition |
|---|---|---|---|---|---|
| TISR-CLI-37 | `src/bin/turingos.rs` + `src/cli/*` | `tests/cli_integration_*.rs` | `handover/evidence/stage_phase6_*` | NOT-LANDED | Phase 6 charter VETO |
| TISR-CAP-UI | `src/runtime/ui_event_capsule.rs` | `tests/constitution_tisr_ui_events.rs` | `handover/evidence/stage_phase7_*` | NOT-LANDED | cas/schema §8 VETO |
| TISR-CAP-A2A | `src/runtime/a2a_message_capsule.rs` | `tests/constitution_tisr_a2a_messages.rs` | `handover/evidence/stage_phase7_*` | NOT-LANDED | (同上) |
| TISR-IR | `src/webui/ui_ir_renderer.rs` | `tests/cli_web_ir_roundtrip.rs` | `handover/evidence/stage_phase7_*` | NOT-LANDED | Web charter VETO |
| TISR-BRIDGE | `src/webui/event_bridge.rs` | `tests/cli_web_bridge_dispatch.rs` | `handover/evidence/stage_phase7_*` | NOT-LANDED | (同上) |
| TISR-POLICY | `src/runtime/policy_engine.rs` | `tests/constitution_tisr_policy_engine.rs` | `handover/evidence/stage_phase7_*` | NOT-LANDED | (同上) |
| TISR-REP | `src/runtime/agent_reputation.rs` | `tests/constitution_tisr_reputation.rs` | `handover/evidence/stage_phase7_*` | NOT-LANDED | (同上) |
| TISR-PROV | `src/runtime/provenance_capsule.rs` | `tests/constitution_tisr_provenance.rs` | `handover/evidence/stage_phase6_*` | NOT-LANDED | Phase 6 charter VETO |

**所有 TISR rows 当前状态**: NOT-LANDED (TISR 是 Class 0 research; 实施阶段独立 charter ratify 后才 LANDING).

---

## 7. 验收 Self-Check (Phase 6+ 实施时跑)

```bash
# 1. Class 0 边界 (TISR research 阶段)
git diff --stat HEAD..main | grep -E "src/|tests/" && echo "ALERT" || echo "OK: docs only"

# 2. G-Phase 收口未冲突
ls handover/directives/2026-*_TISR_VETO_* 2>/dev/null && echo "ALERT" || echo "OK"

# 3. 所有 deliverable forward-bound 标记
grep -L "forward-bound\|批准方案" handover/research/interaction_substrate/**/*.md && echo "ALERT" || echo "OK"

# 4. 0 Class 4 surface 修改
grep -r "src/state/typed_tx.rs\|src/state/sequencer.rs\|src/bottom_white/cas/schema.rs" handover/research/interaction_substrate/50_deliverables/ | grep "fn\|pub struct\|pub enum" && echo "ALERT: Class 4 surface in deliverable" || echo "OK"

# 5. 经济不变量未破
grep -r "f64\|f32" handover/research/interaction_substrate/50_deliverables/ | grep -v "comment\|//" && echo "ALERT: f64 in money path" || echo "OK"
```

---

## 8. 长期宪法演化 (AGI 时代)

| 阶段 | 宪法演化 | TISR 影响 |
|---|---|---|
| 现阶段 (2026) | 宪法 = 防御基底 + 竞技场边界 (REAL-9) | TISR 维持 |
| 中期 (2027-2028) | 宪法 = AGI 工作环境 (Phase 8 A2A 深化) | TISR Phase 8 charter 独立 ratify |
| 长期 (2029+) | 宪法 = AGI 社会运作法律 (跨实例联邦) | TISR Phase 9+ 评估 |

**关键**: TISR 现阶段 0 宪法修改; 仅作 forward-bound 提议供未来阶段引用.

---

## 9. 结论

TISR 6 份 deliverable + 5 Phase 调研严格遵守 TuringOS 宪法:
- ✅ 0 个 Class 4 surface 修改实施 (全 forward-bound)
- ✅ 0 个新 typed_tx variant 提议作为实施方案
- ✅ HEAD_t 6-field 不扩展
- ✅ CompleteSet 守恒不破
- ✅ price = signal not truth
- ✅ predicate {0,1} verdict
- ✅ Signature 二元 (AgentSignature / SystemSignature)
- ✅ 三权分立 (Constitution / ArchitectAI / VetoAI) 维持
- ✅ Information free + Investment costs (Law 1 + 2)
- ✅ 真问题 witness 强 enforcement

TISR Phase 6+ 实施时, 每 Phase 必须独立通过架构师 §8 ratification, 并填入 CONSTITUTION_EXECUTION_MATRIX 的对应 row.
