# TISR Deliverable 05 — Roadmap and Kill Criteria

**本文档为 forward-bound research 提案；任何派生 TB 须独立 §8 ratification；本文档不构成已批准方案。**

**目的**: TISR 12 个月 + 长期 roadmap; 每里程碑的真问题 witness 清单; kill condition + 回滚策略.

---

## 1. 12 月路线图概览

```
2026-05-17 (TISR 研究 ship)
    ↓
[等待 G-Phase 收口 + REAL-13/BCAST-1/13A ship]
    ↓
2026-06-15 (estimated) → Phase 6 启动条件成熟
    ↓
Phase 6 CLI MVP (~6-8 周) → 2026-08-15 ship
    ↓
Phase 7 Web MVP (~10-12 周) → 2026-11-01 ship
    ↓
2027 Q1: Phase 8 A2A 深化启动 (~10-12 周) → 2027 Q2 ship
    ↓
2027 Q3+: REAL-N (N≥16) 涌现验证 / Phase 9 长期评估
```

---

## 2. Phase-by-Phase 详细路线

### 2.1 Phase 6: CLI MVP (6-8 周, Class 1-2)

**Pre-condition** (必须全满足):
1. ✋ G-Phase 收口完成 (SG-G overall §8 packet ship)
2. ✋ REAL-13 ship
3. ✋ REAL-BCAST-1 ship
4. ✋ REAL-13A ship
5. ✋ `git log --since="2026-05-17" -- src/state/typed_tx.rs` 显示无 schema 变更
6. ✋ Phase 6 separate charter §8 ratification
7. ✋ WebSearch / 实施工具预算授权 (若 Phase 6 需要 frontend / 调研)

**Deliverables**:
- `src/bin/turingos.rs` + `src/cli/*` (37 subcommand, ~1300 LOC)
- 5 个 `runtime/*_runner.rs` lib 化 (~4750 LOC 移动)
- `runtime/user_task_runner.rs` + `runtime/user_view.rs` (~600 LOC 提自 lean_market)
- `runtime/provenance_capsule.rs` (~150 LOC)
- 现有 11 bin 简化为薄 entry (保留向后兼容)

**Real-problem Witness**:
- ✅ 完整 12-step happy path 跑通 (turingos init → batch new → agent deploy → task open → audit dashboard → export evidence)
- ✅ Evidence bundle PROCEED (audit_tape verdict)
- ✅ Replay 重建 HEAD_t
- ✅ ProvenanceCapsule ≥ 1 Human-triggered + ≥ 1 agent-submitted case
- ✅ lean_market 命令向后兼容 (老脚本仍能跑)

**Risk**:
- Lean_market evaluator child fork 重构难度 (中)
- 11 个现有 bin 简化时 STEP_B 边界 (低; audit_dashboard 不在 STEP_B 列表)

**Exit Gate**:
- `cargo test --workspace --no-fail-fast` 通过
- `bash scripts/run_constitution_gates.sh` 通过
- Codex audit PROCEED (Class 1-2 dual audit per CLAUDE.md §9)

### 2.2 Phase 7: Web MVP (10-12 周, Class 1-3)

**Pre-condition**:
1. ✋ Phase 6 ship + happy path witness 已 collected
2. ✋ Phase 7 separate charter §8 ratification
3. ✋ cas/schema.rs ObjectType 扩展 §8 ratification (Class 4 候选, **独立 §8**)
4. ✋ WebSearch / WebFetch 预算授权 (若 frontend 需要外部资源)

**Deliverables**:
- `src/webui/*` (~2050 LOC Rust)
- React frontend (~5000+ LOC npm package)
- 6 个新 CAS schema (UIEventCapsule / A2AMessageCapsule / ArtifactStorageManifest / UIArtifact / CuratorEndorsement / AuditFinding) ~1500 LOC
- 3 个新 predicate (UIContent / A2ASignature / VetoAIPolicy) ~600 LOC
- 4 个新 derived view (AgentReputation / Designer / Curator / Auditor) ~1000 LOC
- 2 个新 test file (`constitution_tisr_ui_events.rs` + `_a2a_messages.rs`) ~700 LOC

**Real-problem Witness**:
- ✅ ≥ 3 用户独立通过 Web form 提交 task
- ✅ ≥ 1 VetoAI Reject case + ≥ 1 Approve case (logged directive)
- ✅ Multimodal artifact 上传 ≥ 3 文件类型 (.lean/.py/.png/.mp3)
- ✅ EconomicJudgment ≥ 5 valid + ≥ 2 abstain in single batch
- ✅ Statistical consensus signal extracted (confidence_ratio > 0.6 in ≥ 1 batch)

**Risk** (中):
- cas/schema.rs §8 ratification failure → Phase 7 拆分: Phase 7.0 fallback 用 Generic ObjectType + 嵌套 schema_id
- React frontend LOC 超预算 → 严格 IR + Materializer 边界
- Web layer 安全 (XSS / CSRF) → Strict CSP + Trusted Types

**Exit Gate**:
- 全 unit + integration tests 通过
- E2E Playwright 测试通过 (3+ user scenarios)
- Codex audit PROCEED + Gemini DeepThink (dual audit per Class 3 § CLAUDE.md §9)

### 2.3 Phase 8: A2A 深化 + 协作 + 联邦 (10-12 周, Class 2-3)

**Pre-condition**:
1. ✋ Phase 7 ship
2. ✋ Phase 8 separate charter §8

**Deliverables**:
- Task-granular lease (`task_lease.json` ~300 LOC)
- DID 桥 (`did:turingos:<pubkey>`, ~400 LOC)
- MCP server (read-only tape view, ~1000 LOC)
- ChainTape Merkle root 周期性外锚 (optional, Class 3)
- Active Inference 角色 agent 参考实现 (optional)
- Multi-granular collaboration UI (optimistic update + conflict resolution)

**Real-problem Witness**:
- ✅ ≥ 1 DID resolve case (跨实例 agent 互读身份)
- ✅ ≥ 1 MCP server query case (外部 LLM 读取 TuringOS tape)
- ✅ Multi-user 协作: 2+ users 同时编辑 task; lease 机制工作
- ✅ ChainTape Merkle 外锚 evidence (若实施)

**Risk** (中):
- Class 4 候选 (DirectSwapTx 等) 可能触发 → 严格 forward-bound, 不实施
- 跨实例联邦协议复杂度 (高)

### 2.4 Phase 9: 涌现验证 + 认知扩展 (长期, Class 3)

**Pre-condition**:
1. ✋ Phase 8 ship
2. ✋ REAL-N (N≥16) charter §8

**Deliverables**:
- PlanCapsule + ReflectionTx schema (~600 LOC)
- ForecastCapsule schema (~400 LOC)
- REAL-16 → REAL-32 → REAL-128 涌现验证 (separate runs)

**Real-problem Witness**:
- ✅ REAL-16: 16-agent batch + 价格信号自发收敛
- ✅ REAL-32: 32-agent + 自发 challenge 涌现
- ✅ REAL-128: 128-agent + 跨 task 身份持久 + reputation 显著差异

### 2.5 Phase 10+: AGI 时代 (长期 Class 4, 每个独立 §8)

- DirectSwapTx (多 agent 原子 swap)
- AgentProposedTaskOpen (agent 自主 propose 任务)
- AgentMarketSeeding (agent 自治 seed market)
- HumanSignature type + 人类 PKI
- 跨 TuringOS 实例联邦 (Ethereum/IPFS 公链外锚)
- zkML proof in EvidenceCapsule (低频高价值对外公证)

---

## 3. Kill Conditions (8 条; 每 Phase 持续监控)

| # | Kill Condition | Trigger 测量 | 动作 | 严重性 |
|---|---|---|---|---|
| KC1 | 架构师 PR 阶段 VETO TISR | 架构师 §8 directive 引用 TISR | archive 全部 TISR | 🔴 致命 |
| KC2 | G-Phase 收口期间新 directive 禁止并行 research | `grep -i "no parallel research\|TISR" handover/directives/2026-05-*.md` | pause TISR Phase 6+ | 🟡 中断 |
| KC3 | Phase 0-3 任 Track 命题被现 surface 覆盖 80%+ | inventory checklist 量化 | archive 该 Track (Phase 0 已验证 OK) | 🟢 已通过 |
| KC4 | Phase 6 CLI 实际覆盖 ≥ 90% (薄包装) | bin coverage matrix | 缩减 scope, 只做 P0 wrap; P1/P2 archive | 🟡 缩减 |
| KC5 | 任 Phase 提议需 typed_tx schema 修改 | `grep "new TxKind\|new TransitionError" deliverables/*.md` | 标 Class 4 forward-bound, 该 atom 推后 | 🟡 推后 |
| KC6 | Phase 3 调研发现用户报告 70%+ 覆盖 | gap analysis 量化对照 | 缩减或 archive 该 Track (Phase 3 验证 OK) | 🟢 已通过 |
| KC7 | 任 deliverable 暗藏 Class 4 surface 修改但未标 forward-bound | `grep -L "forward-bound" 50_deliverables/*.md` | 立即 Charter 补丁 + 警告头部 | 🟡 修复 |
| KC8 | user/architect 给出 G-Phase 完成时间表 | 新 LATEST.md update 或 directive | re-evaluate scheduling; 加速进入 Phase 6 | 🟢 加速 |

---

## 4. 回滚策略 (按 Phase)

### 4.1 Phase 6 CLI 回滚

- **触发**: KC4 (薄包装) 或 KC5 (需 typed_tx 修改)
- **回滚**: 缩减为 P0 wrap subcommand only (~12 wrap, ~800 LOC); 25 新 subcommand archive 或 forward-bound
- **保留**: lean_market 独立 bin 仍可用

### 4.2 Phase 7 Web 回滚

- **触发**: cas/schema.rs §8 失败 (KC5 + KC7 衍生)
- **回滚**:
  - Phase 7.0 fallback: 用 Generic ObjectType + 嵌套 schema_id (避免新 ObjectType variant)
  - 或: Phase 7 推后, 等架构师重新批准
- **保留**: Phase 6 CLI 已 ship, 用户仍可继续 (CLI-only mode)

### 4.3 Phase 8+ 回滚

- **触发**: 任 Class 4 候选 §8 失败 (DirectSwapTx / AgentProposedTaskOpen 等)
- **回滚**: 该 atom archive; 不影响已 ship 的 Phase 6/7
- **保留**: 现有 LibrarianBroadcast + EconomicJudgment + Tier 1-2 A2A 仍可用

---

## 5. 真问题 Witness 总清单 (跨 Phase)

| Witness | Phase | 验收 | 存储 |
|---|---|---|---|
| CLI happy path | 6 | 12-step 跑通 + audit PROCEED | `evidence/stage_phase6_happy_path/` |
| ProvenanceCapsule | 6 | ≥ 1 Human + ≥ 1 agent | `evidence/stage_phase6_provenance/` |
| UIEventCapsule | 7 | ≥ 3 content_class 提交 | `evidence/stage_phase7_ui_events/` |
| A2AMessageCapsule | 7 | ≥ 5 + 2 abstain | `evidence/stage_phase7_a2a/` |
| Multimodal artifact | 7 | ≥ 3 文件类型 | `evidence/stage_phase7_multimodal/` |
| VetoAI gate | 7 | ≥ 1 Reject + ≥ 1 Approve | `evidence/stage_phase7_veto/` |
| Statistical consensus | 7 | confidence_ratio > 0.6 | `evidence/stage_phase7_consensus/` |
| Multi-user collab | 8 | 2+ users + lease 工作 | `evidence/stage_phase8_collab/` |
| DID resolve | 8 | ≥ 1 跨实例读取 | `evidence/stage_phase8_did/` |
| MCP server query | 8 | ≥ 1 外部 LLM 读取 | `evidence/stage_phase8_mcp/` |
| REAL-16 涌现 | 9 | 16-agent + 价格收敛 | `evidence/stage_phase9_real16/` |
| REAL-32 涌现 | 9 | 32-agent + 挑战涌现 | `evidence/stage_phase9_real32/` |
| REAL-128 持续身份 | 9 | 128-agent + reputation 显著 | `evidence/stage_phase9_real128/` |

---

## 6. 与 G-Phase 收口的协调

按 Charter §2 + 架构师 2026-05-14 verbatim:

| G-Phase 状态 | TISR 状态 | 动作 |
|---|---|---|
| G5/G6/G7 in flight | TISR 研究 (Phase 0-5) | 继续 (worktree 物理隔离) |
| SG-G overall §8 packet ship | TISR Phase 6 可启动 | 申请 Phase 6 charter §8 |
| G-Phase 全部 ship | TISR Phase 6 → 7 → 8 顺序启动 | 每 Phase 独立 §8 |
| G-Phase 收口期间架构师 VETO | TISR pause (KC2) | 等 architect 进一步指令 |

---

## 7. 监管对齐与外部 Hook

### 7.1 EU AI Act §12 record-keeping (Track I 发现)

- TuringOS ChainTape + CAS + replay 几乎 spec 上等价物
- Phase 8 可考虑生成 EU AI Act compliance report (自动从 ChainTape 派生)

### 7.2 MCP 互操作 (Phase 8)

- MCP server (read-only tape view) 允许外部 LLM (Claude Desktop / ChatGPT) 直接读 TuringOS 状态
- AGI 时代 agent 互操作基础设施

### 7.3 DID 跨实例联邦 (Phase 8+)

- `did:turingos:<pubkey>` URI 兼容 W3C DID v1.1
- 跨 TuringOS 实例 agent 身份可读
- 不直接上 ETH 主网 (gas + 经济冲突); 通过 Merkle 外锚

---

## 8. 长期愿景 (Phase 10+)

| 长期目标 | Phase | 真问题 Witness |
|---|---|---|
| AGI agent 自治市场 | 10+ | AgentProposedTaskOpen ship + ≥ 100 agent-proposed tasks accepted |
| 人类升级为 constitution-level | 11+ | 人不再直接 propose task; 只批准 constitution amendment |
| 跨实例联邦 | 11+ | 3+ TuringOS instances 互信任 + 跨 chain agent 迁移 |
| 监管合规默认 | 12+ | EU AI Act / US AI Bill compliance 自动生成 |
| zkML 对外公证 | 12+ | 关键 EvidenceCapsule 含 zkML proof |
| AGI 社会基底 | 13+ | TuringOS 作 AGI 社会运作 substrate; 跨行业部署 |

---

## 9. Roadmap 自检 (本文档 health)

```bash
# 1. 所有 Phase pre-condition 显式列出
grep -c "Pre-condition" 50_deliverables/05_ROADMAP_AND_KILL_CRITERIA.md  # ≥ 5

# 2. 每 Phase 有 real-problem witness
grep -A 3 "### 2\." 50_deliverables/05_ROADMAP_AND_KILL_CRITERIA.md | grep "Real-problem Witness"  # ≥ 5

# 3. 8 个 kill condition 全列
grep -c "^| KC" 50_deliverables/05_ROADMAP_AND_KILL_CRITERIA.md  # = 8

# 4. 全 forward-bound 标记
grep -c "forward-bound" 50_deliverables/05_ROADMAP_AND_KILL_CRITERIA.md  # ≥ 5
```

---

## 10. 结语: TISR 现阶段 vs 未来

**TISR 现阶段** (2026-05-17 ship):
- ✅ 完整 forward-bound research 文档 (19 + 6 = 25 份)
- ✅ 0 个 Class 4 修改实施
- ✅ 与架构师 2026-05-14 verbatim "不要再开新方向" 兼容 (worktree 物理隔离)
- ✅ Phase 6-10+ 完整路线图

**TISR 未来** (待 G-Phase 收口后):
- Phase 6 CLI MVP → 用户视角完整接通 TuringOS 全功能
- Phase 7 Web MVP → HTML-first Workspace 替代 CLI 主入口
- Phase 8 A2A 深化 → DID + MCP + 跨实例联邦
- Phase 9 涌现验证 → REAL-N (N≥16) 大规模 agent 集群
- Phase 10+ AGI 时代 → 人立法 + agent 自治市场

**TISR 不是工程项目, 是 AGI 时代 TuringOS 演化的 forward-bound 蓝图**.

---

## 11. 完整 6 份 Deliverable 引用

- `00_UNIFIED_CLI_SPEC.md` — Phase 6 CLI 源头规范 (本文 §2.1 引用)
- `01_MASTER_PLAN.md` — 项目主纲领
- `02_CONSTITUTIONAL_ALIGNMENT.md` — 宪法对齐表
- `03_CODE_INTEGRATION_SPEC.md` — 代码级 module 详细
- `04_A2A_PROTOCOL_DESIGN.md` — 轴 B 重点
- `05_ROADMAP_AND_KILL_CRITERIA.md` (本文)

**Phase 5 完成**: 6 份 deliverable 全 ship; TISR 研究阶段正式完成; Phase 6+ 实施待 G-Phase 收口 + 架构师独立 §8 ratification.
