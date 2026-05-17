# TISR Project Index

**TuringOS Interaction Substrate Research (TISR-001)** | 2026-05-17 | 30 docs / 10289 lines (incl. INDEX + EXECUTIVE_SUMMARY)

**本文档为 forward-bound research 提案；任何派生 TB 须独立 §8 ratification；本文档不构成已批准方案。**

---

## 30 秒导航

| 你是谁 | 先读哪份 |
|---|---|
| **架构师** (准备 PR 审计) | `50_deliverables/01_MASTER_PLAN.md` → `02_CONSTITUTIONAL_ALIGNMENT.md` → `EXECUTIVE_SUMMARY.md` |
| **AI Coder** (准备 Phase 6 实施) | `50_deliverables/00_UNIFIED_CLI_SPEC.md` → `03_CODE_INTEGRATION_SPEC.md` |
| **新加入研究员** | `EXECUTIVE_SUMMARY.md` → `00_charter/PHILOSOPHY.md` → `00_charter/CHARTER.md` |
| **Codex 独立审计** | `00_charter/CHARTER.md` (kill conditions) → `40_synthesis/*` → `50_deliverables/*` |
| **想看具体技术细节** | `20_architecture/*` (代码级) + `30_frontier/*` (前沿) |

---

## 完整文档清单 (按 Phase 组织)

### Phase 0 — 基础设施 (5 docs / 1172 lines)

`00_charter/`:
- `CHARTER.md` (250) — 项目宣言 + G-Phase carve-out + 8 kill conditions + Phase exit self-check
- `PHILOSOPHY.md` (197) — v2 阶段化双轴愿景 + v2.1 修订 (在 phased_axis_synthesis 中)
- `REFERENCE_INPUTS.md` (222) — 用户上传 Generative HTML 报告 + plan-mode 答复 + auditor 输出归档
- `EXISTING_SURFACE_INVENTORY.md` (320) — TuringOS v4 现存 surface 全盘点; src/runtime (48 files) + src/bin (11 files) + src/state (7 files); 14 个 TISR Track 命题 vs 覆盖度量化

`90_references/`:
- `FOUNDATION_READ.md` (183) — 7 个关键文档摘要 (constitution / G-Phase directive / REAL-5/12 / REAL-9 / typed_tx / sequencer)

### Phase 1 — Unified CLI Design (4 docs / 2134 lines)

`10_cli_design/`:
- `track_cli_a_user_journey.md` (421) — 11 用户场景 + happy path 12 步 + 5 用户类型矩阵
- `track_cli_b_existing_bin_integration.md` (463) — 12 bin 全盘点 (含 lean_market 重大发现) + lib 化重构清单 ~4750 LOC
- `track_cli_c_missing_capabilities.md` (619) — 25 个新 subcommand + 优先级 (P0/P1/P2) + Class 评估
- `track_cli_d_architecture.md` (631) — clap 三层架构 + 37 subcommand 完整树 + 共用层设计

### Phase 2 — 并行架构映射 (5 docs / 2480 lines)

`20_architecture/`:
- `track_a_typed_tx.md` (338) — typed_tx + sequencer **0-touch 边界**; 4 个 Class 4 forward-bound candidates; emit_system_tx 唯一入口
- `track_b_evidence_chain.md` (385) — ChainTape / CAS / HEAD_t; UIEventCapsule + A2AMessageCapsule + ArtifactStorageManifest schemas; HEAD_t 6-field 不扩展
- `track_c_materializer.md` (582) — Web layer **from-scratch** (不扩展 audit_dashboard); 4-layer 信息架构; Turing UI IR + Event Bridge + Policy Engine
- `track_d_economy.md` (562) — REAL-5/12 集成; 信息免费 vs 投资花钱; 三层价格协议; Designer/Curator/Auditor 派生视图
- `track_e_predicate.md` (613) — UIContentPredicate {0,1}; ProvenanceCapsule (human vs agent); VetoAI policy gate; A2A verification asymmetry

### Phase 3 — 并行前沿调研 (5 docs / 1733 lines)

`30_frontier/`:
- `track_f_a2a_protocols.md` (411) — Google A2A / Agora / CP-WBFT (85.7% 恶意容忍) / Stigmergy; 13 WebSearch + 4 WebFetch
- `track_g_multimodal.md` (441) — GPT-4o Realtime ($0.30/min) / VR 不推荐 / 具身 AI 不写 tape / **dual-hash** (BLAKE3+pHash)
- `track_h_software_3.md` (255) — Karpathy 2025-2026 / Anthropic Effective Agents; **TuringOS 缺 agent-writable memory + LLM-as-judge + 终端用户层**
- `track_i_verifiable_ai.md` (435) — zkML 2025-2026 拐点 (Lagrange DeepProve-1 + ERC-8004); **EU AI Act §12 与 ChainTape spec 重叠**
- `track_j_agi_cognition.md` (191) — **N≥16-32 涌现门槛** (REAL-12 差 3-4x); ForecastCapsule + PlanCapsule + ReflectionTx; Active Inference vs Hayek

### Phase 4 — 综合 + 差距分析 (3 docs / 754 lines)

`40_synthesis/`:
- `gap_analysis.md` (134) — 用户报告 = 优秀轴 A 起点; **8 强项保留 + 12 弱项补充 + 6 错误纠正**; TuringOS 主动定位为 A2A substrate 而非 generic UI
- `architecture_integration.md` (316) — 4-layer 统一架构图; ~12500 LOC 估算; **0 个 Class 4 修改** (cas/schema.rs ObjectType 扩展是 Phase 7 候选, 需独立 §8)
- `phased_axis_synthesis.md` (304) — v2 命题验证 → 修订为 **v2.1**: 人 = constitution-level 立法者; Signature 二元 + ingress 三模式

### Phase 5 — 最终交付 (6 docs / 1633 lines)

`50_deliverables/`:
- `00_UNIFIED_CLI_SPEC.md` (210) — Phase 6 CLI MVP 源头规范; 37 subcommand; lean_market → turingos 演化
- `01_MASTER_PLAN.md` (229) — 项目主纲领; TuringOS 长期定位 = AGI 时代可验证 A2A substrate
- `02_CONSTITUTIONAL_ALIGNMENT.md` (199) — 每特性 vs 宪法条款映射; 0 Class 4 修改; 7 Class 4 forward-bound candidates
- `03_CODE_INTEGRATION_SPEC.md` (284) — Phase 6/7/8+ 模块级清单 + Cargo.toml + test 架构
- `04_A2A_PROTOCOL_DESIGN.md` (412) — 轴 B 重头戏; 4 层协议 (Broadcast/Quotation/Coin/Covenant); 验证不对称
- `05_ROADMAP_AND_KILL_CRITERIA.md` (299) — 12 月路线图; 真问题 witness 清单; Phase 化回滚策略

---

## 按主题快速查找

### 轴 A (Software 3.0 HCI)
- 总体: `01_MASTER_PLAN.md` §4.1
- 命题修订: `40_synthesis/phased_axis_synthesis.md`
- CLI: `Phase 1` 全 4 docs + `00_UNIFIED_CLI_SPEC.md`
- Web: `track_c_materializer.md` + `03_CODE_INTEGRATION_SPEC.md` §3
- 安全: `track_c_materializer.md` §5 (Event Bridge)
- Policy: `track_c_materializer.md` §6 + `03_CODE_INTEGRATION_SPEC.md` §3.5

### 轴 B (A2A 自治通信)
- 总体: `04_A2A_PROTOCOL_DESIGN.md`
- 协议: `track_a_typed_tx.md` + `track_b_evidence_chain.md` + `track_f_a2a_protocols.md`
- 角色: `track_d_economy.md` + REAL-5/12 现有基础
- 价格信号: `track_d_economy.md` §3 + `track_f_a2a_protocols.md`
- 涌现验证: `track_j_agi_cognition.md` Q4 + `05_ROADMAP` Phase 9 (REAL-N)

### TuringOS 经济模型
- 集成: `track_d_economy.md`
- 派生视图: `track_d_economy.md` §3 (Designer/Curator/Auditor)
- 价格三层: `track_d_economy.md` §4 + `04_A2A_PROTOCOL_DESIGN.md` §2
- 声誉/破产: `track_d_economy.md` §5

### 谓词 + 真问题 Witness
- 集成: `track_e_predicate.md`
- UIContentPredicate: `track_e_predicate.md` §3
- Witness 清单: `track_e_predicate.md` §7 + `05_ROADMAP` §5

### 多模态
- 现状: `track_g_multimodal.md`
- Artifact 存储: `track_b_evidence_chain.md` Q4 + `track_g_multimodal.md` Q4 (dual-hash)
- Realtime API: `track_g_multimodal.md` Q1

### 可验证 AI / 监管
- zkML: `track_i_verifiable_ai.md` Q1
- TEE: `track_i_verifiable_ai.md` Q2
- EU AI Act: `track_i_verifiable_ai.md` Q4 + Q5
- DID 桥: `track_i_verifiable_ai.md` Q5 + `04_A2A_PROTOCOL_DESIGN.md` §11

### AGI 认知架构
- World model: `track_j_agi_cognition.md` Q1
- Active Inference: `track_j_agi_cognition.md` Q2
- Meta-cognition: `track_j_agi_cognition.md` Q3
- 涌现门槛: `track_j_agi_cognition.md` Q4

### Class 边界 / 宪法对齐
- 总: `02_CONSTITUTIONAL_ALIGNMENT.md`
- Kill conditions: `00_charter/CHARTER.md` §5 + `05_ROADMAP` §3
- Class 4 candidates (forward-bound): `02_CONSTITUTIONAL_ALIGNMENT.md` §5

---

## 关键引用

### 重大发现
1. **lean_market.rs 是 TB-10 已 ship user CLI** (848 行 / 7 subcommand) → Phase 6 是演化非 from-scratch — `track_cli_a_user_journey.md` §0
2. **TISR Phase 6 与 sequencer 0-touch** — `track_a_typed_tx.md` §8
3. **N≥16-32 涌现门槛** (REAL-12 当前 4-5 agent 差 3-4x) — `track_j_agi_cognition.md` Q4
4. **EU AI Act §12 与 ChainTape spec 重叠** — `track_i_verifiable_ai.md` Q4
5. **v2 → v2.1 命题**: 人 = constitution-level + Signature 二元 + ingress 三模式 — `phased_axis_synthesis.md` §9

### Phase 6 启动条件
见 `00_UNIFIED_CLI_SPEC.md` §13 (7 个 pre-condition, 当前 0/7 满足).

### 8 Kill Conditions (持续监控)
见 `00_charter/CHARTER.md` §5 (KC1-KC8) + `05_ROADMAP` §3.

---

## 引用约定

- **现存代码**: `src/<path>:<line>` (e.g., `src/state/typed_tx.rs:2327`)
- **TISR 内部**: 相对路径 (e.g., `track_a_typed_tx.md` §3)
- **handover/**: 绝对路径从 worktree 根 (e.g., `handover/ai-direct/LATEST.md`)
- **外部资源**: markdown 链接 `[Title](URL)`

---

## 状态摘要

- **TISR 研究阶段**: Phase 0-5 100% 完成 ✅
- **0 个 Class 4 surface 修改实施** ✅
- **8 个 Kill Condition 全未触发** ✅
- **3 个 exit check 全 Phase 通过** ✅
- **总文档**: 30 (含本 INDEX 和 EXECUTIVE_SUMMARY)

**Worktree**: `claude/tisr-2026-05-17` (`worktree-tisr-2026-05-17` branch) @ HEAD `2dd4820` = main, 无 src/tests 修改.

**后续**: Phase 6 (CLI MVP) 待 G-Phase 收口 + 架构师独立 §8 ratification.
