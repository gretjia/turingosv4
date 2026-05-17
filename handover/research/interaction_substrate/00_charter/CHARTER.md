# TISR Charter

**Project**: TuringOS Interaction Substrate Research
**Codename**: `tisr`
**Phase ID**: `TISR-001`
**Charter Date**: 2026-05-17
**Charter Author**: Claude (gretjia 主导)
**Worktree**: `claude/tisr-2026-05-17` (branch: `worktree-tisr-2026-05-17`)
**Risk Class**: Phase 0-5 = **Class 0** (research docs only); Phase 6 (后续 separate charter) = Class 1-2 (CLI 实施)
**FC Trace**: N/A (research phase — no kernel/tape/predicate modification)

---

## 0. 本文件性质声明 (顶部 anti-misappropriation 警告)

**本文件及 TISR 项目下所有产出文档为 forward-bound research 提案。任何派生 TB 必须独立通过对应 Class 的 §8 ratification 才能进入实施。本研究文档不构成已批准方案。**

本文件不修改：
- `constitution.md`
- `src/state/typed_tx.rs` / `src/state/sequencer.rs` / `src/bottom_white/cas/schema.rs` / `src/kernel.rs`
- ChainTape / CAS / HEAD_t / RootBox / 任何 STEP_B 受保护 surface
- 任何 economic-state mutator 或 money path

任何后续 commit 若试图借用本文件 design 直接修改上述 surface，必须先走完整 Class 4 STEP_B 流程。

---

## 1. 项目愿景

### 1.1 Dual-Axis Vision (v2 阶段化版本)

**轴 A — Software 3.0 HCI (人机互动层)**

- **现阶段 (2026 内)**: 人作为 **spec 批准者 + init 模块启动者 + read-view 一等公民**；UI/CLI 是人的主要交互入口；agent 仍是被人启动的工作者
- **未来 (AI 时代可能比预期更快到来)**: 留 agent 自接入扩展面（如 polymarket 环节让 agent 直接操作交易）；不在现阶段假设其成立

**轴 B — Agent-to-Agent 自治通信基底 (无人在环)**

- 沿 `src/runtime/real5_roles.rs` (10 个 AgentRole: REAL-5 原 8 + REAL-12 扩展 BullTrader/BearTrader) + `src/runtime/economic_judgment.rs` (REAL-12) + `src/runtime/librarian_broadcast.rs` (REAL-BCAST-1 in-flight) 现有基底
- 价格信号作为 Hayek 式通信原语（`constitution.md` Art. II.2 对齐）
- 大规模 agent 集群涌现 + 共识 + 声誉 + 自动合约

### 1.2 双轴交汇命题 (v2 修订)

~~原命题 (v1, 已弃)~~: "人 = agent 特例"

**v2 修订命题**: **"人 = 特权角色（spec 批准 + init 启动）；agent = 工作角色；两者通过同一条 tape + 同一套谓词通信，但 ingress 路径不同"**

修订原因 (auditor file:line 引用):
- `src/state/typed_tx.rs:2327` — TypedTx 19 variants 全为 agent-submitted 或 system-emitted；无 Human/User variant
- `src/state/sequencer.rs:4055` — `SystemTxForbiddenOnAgentIngress` 只识别 agent_submitted 和 system_emitted 两条 ingress
- `src/state/typed_tx.rs:365` — 注释明确 "human reading L4" — 人在当前模型中是 reader，不是 submitter

要让 "人 = agent 特例" 落地必然引入第三条 ingress 或包装 human 为特殊 agent — 两者都是 Class 4 surface 修改。v2 命题与现架构兼容。

---

## 2. G-Phase 关系 (架构师 verbatim carve-out)

### 2.1 架构师 verbatim 引用

来源: `handover/directives/2026-05-14_TB_G_G_PHASE_CLOSEOUT_ARCHITECT_UPDATE.md:147`

> **"现在不要再开新方向。先把 G-Phase 作为一个整体收口。"**

来源同 file:111

> "G4.2 是进入真正 multi-agent generative research 的前置项。"

### 2.2 TISR carve-out 承诺

TISR 在 worktree `claude/tisr-2026-05-17` 物理隔离地运行，对 main worktree 的 G-Phase 收口工作 **零干扰承诺**：

1. **物理隔离**: TISR 所有文件在独立 worktree；main 上 G-Phase 收口（G5/G6/G7/SG-G aggregate）继续不受影响
2. **不消耗主线工时**: TISR 在 sub-agent 中执行；主对话 context 不阻塞主线开发
3. **不修改 src/**: TISR Phase 0-5 全部输出为 `handover/research/interaction_substrate/` 下文档
4. **延后实施**: SG-G overall §8 packet 完成前，TISR **不进入 Phase 6 实施**；Phase 6 启动需 separate charter + 标准 TB 流程
5. **PR 时 surface 给架构师**: 若架构师在 PR 阶段明确 VETO，TISR 立即 archive

### 2.3 用户授权依据

用户 plan-mode 答复 verbatim (2026-05-17):

> "现阶段不需要架构师参与了，反正是在独立 worktree, 等 PR 时再让架构师审计。"

本 carve-out 在用户授权范围内运行；不构成对 2026-05-14 architect directive 的撤销，仅利用 worktree 物理隔离避免冲突。

---

## 3. 计划结构

```
Phase 0: 基础设施 + Existing Surface Inventory       ~1h
Phase 1: 🆕 Unified CLI Design (用户视角全连接)       ~3-4h
Phase 2: 并行架构映射 (5 track 拆 2 批)              ~3h
Phase 3: 并行前沿调研 (5 track 拆 2 批)              ~3h
Phase 4: 综合 + 差距分析                              ~1-2h
Phase 5: 6 份最终交付文档                             ~2-3h
[Phase 6 (后续 separate charter): CLI MVP 实施 Class 1-2]
```

**总工时**: ~13-16h Claude；墙钟 ~5-7h with parallelism

---

## 4. Phase 详细拆解

### Phase 0 — 基础设施 + Existing Surface Inventory
- worktree 创建 + 目录骨架（已完成）
- 本 CHARTER.md + PHILOSOPHY.md + REFERENCE_INPUTS.md
- **关键**: EXISTING_SURFACE_INVENTORY.md (audit 修维度 4 关键修复)
- Foundation read: 7 个关键文档摘要

### Phase 1 — Unified CLI Design (Class 1-2 spec)
**核心目标**: 从用户视角设计一个能立即用上的 CLI，把现有 TuringOS 全架构接通。

**约束** (修复 audit 维度 1+4+7):
- 不引入新 typed_tx variant
- 不修改 sequencer admission
- 只整合现有 bin + 添加缺失的薄包装
- 用户立即可用 = real-problem witness anchor

**子 Track**:
- CLI-A: User Journey 设计
- CLI-B: 现有 bin 集成清单
- CLI-C: 缺失能力清单
- CLI-D: CLI Architecture 设计

### Phase 2 — 并行架构映射 (5 track 拆 2 批)
- 第一批 (3 agent): Track A (typed_tx 重点采样) + B (ChainTape) + C (Materializer)
- 第二批 (2 agent): Track D (经济学) + E (谓词)

### Phase 3 — 并行前沿调研 (5 track 拆 2 批，需 user explicit WebSearch 授权)
- 第一批 (3 agent): Track F (A2A) + G (多模态) + H (Software 3.0)
- 第二批 (2 agent): Track I (可验证 AI) + J (AGI 认知)

### Phase 4 — 综合 + 差距分析
- gap_analysis.md / architecture_integration.md / phased_axis_synthesis.md
- Context 压缩策略: 每 sub-agent 500-1000 字摘要

### Phase 5 — 6 份最终交付文档
1. `00_UNIFIED_CLI_SPEC.md` 🆕
2. `01_MASTER_PLAN.md`
3. `02_CONSTITUTIONAL_ALIGNMENT.md`
4. `03_CODE_INTEGRATION_SPEC.md`
5. `04_A2A_PROTOCOL_DESIGN.md`
6. `05_ROADMAP_AND_KILL_CRITERIA.md`

每份头部含本 Charter §0 警告复制。

---

## 5. Kill Conditions (8 条具体可测 trigger)

| # | Trigger | 测量办法 | 动作 |
|---|---|---|---|
| 1 | PR 阶段架构师明确 VETO | 架构师 §8 directive 引用本项目 | archive 全部 TISR |
| 2 | SG-G 收口期间架构师 directive 禁止并行 research | `grep -i "no parallel research\|TISR" handover/directives/2026-05-*.md` 出现 negative directive | pause TISR |
| 3 | Phase 0 surface inventory 发现某 Track 命题被现存 surface 覆盖 80%+ | inventory checklist 量化打分 | archive 该 Track |
| 4 | Phase 1 (CLI) 发现现有 `src/bin/` 已 90%+ 覆盖用户工作流 | bin coverage matrix | CLI Phase 5 spec 缩为薄包装 |
| 5 | Phase 2 任何 track 提议需要 typed_tx schema 修改才能落地 | `grep "new typed_tx\|new TxKind\|new TransitionError" 20_architecture/*.md` | 标 forward-bound 并标 Class 4 候选；不深入实施 spec |
| 6 | Phase 3 前沿调研发现用户上传报告已覆盖 70%+ 该 track 议题 | gap_analysis.md 量化对照表 | 缩减或 archive 该 Track |
| 7 | Phase 5 任意 deliverable 出现暗藏 Class 4 修改 spec 但未标 forward-bound 警告 | `grep -L "本文档为 forward-bound" 50_deliverables/*.md` | 立即 Charter 补丁 + 添加警告头 |
| 8 | user/architect 给出 G-Phase 完成时间表 (G-Phase 接近收口) | 新 LATEST.md update 或新 directive | re-evaluate TISR scheduling — 是否加速进入 Phase 6 实施 |

---

## 6. Phase Exit Self-Check (机械化)

每个 Phase 出口运行 3 条自检:

```bash
# 1. 是否仍在 Class 0 边界内 (未修改 src/)
git diff --stat HEAD | grep -E "src/|tests/" && echo "ALERT: src/tests modified" || echo "OK: docs only"

# 2. 是否与 G-Phase 收口指令冲突 (检查是否有新架构师 directive)
ls handover/directives/2026-05-1[7-9]_* 2>/dev/null | grep -i "veto\|halt\|pause" && echo "ALERT: new VETO directive" || echo "OK: no veto"

# 3. 是否引入 "改宪法才能落地" 的设计 (grep 当前 Phase 输出)
grep -rL "本文档为 forward-bound" handover/research/interaction_substrate/ | grep "\.md$" && echo "ALERT: missing forward-bound header" || echo "OK: all docs flagged"
```

任一 ALERT 出现 → 暂停 Phase，向 user 汇报，按 kill condition 表决定动作。

---

## 7. 资源 + 工时

| Phase | 工时 | 子 agent 数 | 并行性 |
|---|---|---|---|
| Phase 0 | ~1h | 0 | 主 agent 顺序 |
| Phase 1 | ~3-4h | 0 | 主 agent 顺序 (4 个子 track 顺序做) |
| Phase 2 | ~3h | 5 (2 批: 3 + 2) | 每批并行 |
| Phase 3 | ~3h | 5 (2 批: 3 + 2) | 每批并行 |
| Phase 4 | ~1-2h | 0-1 | 主 agent 合成 |
| Phase 5 | ~2-3h | 0 | 主 agent 顺序 |

**总计**: ~13-16h Claude；墙钟 ~5-7h

---

## 8. 验收标准 (TISR Done = ?)

TISR 项目 Done 定义:

1. ✅ Phase 0-5 全部 deliverable 写出且通过 exit self-check
2. ✅ 6 份 deliverable 头部均含 forward-bound 警告
3. ✅ Kill condition 8 条均未触发 (或触发后 archive 对应 track)
4. ✅ Existing surface inventory 完成且与 Phase 1-3 的 track 命题对照过
5. ✅ Phase 1 CLI Spec 可直接作为 Phase 6 separate charter 的 source-of-truth
6. ✅ Phase 4 gap analysis 显示与用户上传报告的差异点 ≥ 10 个具体盲点
7. ✅ Phase 5 ROADMAP 含 Phase 6 (CLI MVP) + Phase 7 (轴 A) + Phase 8 (轴 B) 的初步排序

**不构成 Done**:
- ❌ 架构师批准（PR 时再 surface）
- ❌ 代码实施（属于 Phase 6 separate charter）
- ❌ Real-problem witness（属于 Phase 6 CLI MVP 验证阶段）

---

## 9. 后续路径 (TISR scope 之外)

TISR Phase 5 完成后:

1. **Phase 6 separate charter**: CLI MVP 实施
   - Class 1-2 risk class
   - 走标准 TB 流程: charter → harness → real run → audit → ship
   - 时间: TISR Phase 5 完成 + SG-G overall §8 完成后启动

2. **Phase 7 separate charter**: HCI 轴 A 深化
   - 基于 Phase 6 CLI 已用上经验 + Generative HTML 报告
   - Class 2-3

3. **Phase 8 separate charter**: A2A 轴 B 深化
   - 基于 REAL-5/REAL-12/REAL-BCAST-1 在 flight 进展
   - Class 3-4 (需架构师 §8)

均不属于 TISR scope；TISR 仅产出指向它们的 spec。

---

## 10. Charter Signature

| 字段 | 取值 |
|---|---|
| 创建时间 | 2026-05-17 |
| Worktree HEAD | (运行 `git rev-parse HEAD` 获取实时值) |
| 用户授权来源 | Plan mode 2026-05-17 全部批准 |
| 审计来源 | TISR v1 auditor verdict CHALLENGE (本文件已修复 6 个 CHALLENGE 项) |
| 下次 Charter 修订触发 | Kill condition 任一触发，或 Phase 5 完成 |
