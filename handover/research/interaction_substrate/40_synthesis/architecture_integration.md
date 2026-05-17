# Phase 4 — Architecture Integration (Phase 2 五 Track 合并)

**本文档为 forward-bound research 提案；任何派生 TB 须独立 §8 ratification；本文档不构成已批准方案。**

**目的**: 整合 Phase 2 五个架构 track (typed_tx / ChainTape-CAS-HEAD_t / Materializer / 经济学 / 谓词) 的代码级方案为统一架构图 + 模块依赖矩阵 + Class 风险分布; Phase 5 `03_CODE_INTEGRATION_SPEC.md` 的核心输入.

---

## 1. 统一架构图

```
┌────────────────────────────────────────────────────────────────────┐
│ Layer 0: User Interaction (Phase 6 CLI / Phase 7+ Web)             │
│  ├── Phase 6 (Class 1-2): turingos CLI (37 subcommand)             │
│  │    └── lean_market evolution (Track A Q4)                       │
│  └── Phase 7+ (Class 1-2 + new web layer):                         │
│       ├── React + Web Components + Tailwind                        │
│       ├── Turing UI IR (Track C Q2)                                │
│       └── Event Bridge (Track C Q3)                                │
└──────────────────────┬─────────────────────────────────────────────┘
                       │ (Action Event)
                       ▼
┌────────────────────────────────────────────────────────────────────┐
│ Layer 1: Policy + Permission (Class 1-2 from-scratch)              │
│  ├── Policy Engine (Track C Q4 - Rust 原生, 不扩展 rules/engine.py) │
│  ├── VetoAI Policy Gate (Track E Q3 - non-binding signal)          │
│  └── UIContentPredicate (Track E Q1 - PCP {0,1} verdict)           │
└──────────────────────┬─────────────────────────────────────────────┘
                       │ (Validated Typed Event)
                       ▼
┌────────────────────────────────────────────────────────────────────┐
│ Layer 2: TuringOS Sequencer (STEP_B Frozen - 0-touch)              │
│  ├── emit_system_tx (轴 A: 人 → system_emitted 间接发起)            │
│  │    └── 6 System variants: FinalizeReward / EventResolve / ...   │
│  └── submit_agent_tx (轴 B: agent → agent_submitted)               │
│       └── 13 Agent variants: Work / Verify / CpmmSwap / ...        │
└──────────────────────┬─────────────────────────────────────────────┘
                       │ (Accepted Transaction)
                       ▼
┌────────────────────────────────────────────────────────────────────┐
│ Layer 3: Evidence Layer (Class 1-3 forward-bound extensions)       │
│  ├── ChainTape L4 (accepted) + L4.E (rejected)                     │
│  ├── CAS (existing): PromptCapsule / EvidenceCapsule / MarkovEC    │
│  ├── CAS (new, Track B): UIEventCapsule / A2AMessageCapsule /      │
│  │    ArtifactStorageManifest / Image/Audio/Video/Model3dBlob      │
│  └── HEAD_t Witness (6-field constitution-pinned, 不扩展)           │
└──────────────────────┬─────────────────────────────────────────────┘
                       │ (Anchored Evidence)
                       ▼
┌────────────────────────────────────────────────────────────────────┐
│ Layer 4: Economy / Audit / Replay (Read-only views)                │
│  ├── Role-Scoped DerivedView (Track D Q2):                         │
│  │    DesignerView / CuratorView / AuditorView (CAS-derived)       │
│  ├── AgentReputation (Track D Q4 - 派生只读)                        │
│  ├── PriceIndex / LibrarianBroadcast (existing)                    │
│  ├── EconomicJudgment / NoTradeReason (existing REAL-12)           │
│  └── replay_full_transition (existing + 3 new integrity gates)     │
└────────────────────────────────────────────────────────────────────┘
```

---

## 2. 模块依赖矩阵

### 2.1 新增模块清单 (Class 1-3, forward-bound)

| 模块 | 路径 | Class | LOC | Phase | 依赖 |
|---|---|---|---:|---|---|
| **CLI 入口层** | | | | | |
| `src/bin/turingos.rs` | 主 CLI 入口 | 1 | ~150 | 6 | clap, runtime/* |
| `src/cli/` | clap routing + 16 commands | 1 | ~1000 | 6 | runtime/*_runner.rs |
| **Lib runner 层** | | | | | |
| `src/runtime/audit_dashboard_report.rs` | lib 化 from `bin/audit_dashboard.rs` | 1 | ~3000 | 6 | (移动重构) |
| `src/runtime/audit_tape_runner.rs` | lib 化 | 1 | ~250 | 6 | (移动) |
| `src/runtime/audit_tape_tamper_runner.rs` | lib 化 | 1 | ~350 | 6 | (移动) |
| `src/runtime/markov_capsule_runner.rs` | lib 化 | 1 | ~350 | 6 | (移动) |
| `src/runtime/user_task_runner.rs` | lib 化 from lean_market run-task | 2 | ~400 | 6 | sequencer.emit_system_tx / submit_agent_tx |
| `src/runtime/user_view.rs` | lib 化 from lean_market view-* | 1 | ~200 | 6 | (移动) |
| `src/runtime/export_runner.rs` | evidence bundle export | 1 | ~200 | 6 | tar / serde |
| **Capsule schema 层 (Track B)** | | | | | |
| `src/runtime/ui_event_capsule.rs` | UIEventCapsule schema + writer | 1-3 | ~250 | 7 | cas/schema, cas/store |
| `src/runtime/a2a_message_capsule.rs` | A2AMessageCapsule schema + writer | 3 | ~300 | 7 | cas/schema, real5_roles |
| `src/runtime/artifact_manifest.rs` | ArtifactStorageManifest + dedup | 1-3 | ~400 | 7 | blake3 / cas/store |
| `src/runtime/provenance_capsule.rs` | Human vs Agent audit trail (Track E Q2) | 1 | ~150 | 6 | (无新依赖) |
| **CAS ObjectType 扩展 (Track B)** | | | | | |
| `src/bottom_white/cas/schema.rs` | +5 new variants (UIEventCapsule / A2AMessageCapsule / ArtifactStorageManifest / ImageBlob / AudioBlob / VideoBlob / Model3dBlob) | 1 | +50 | 7 | (现有 schema 扩展, 不破坏 backward-compat) |
| **Predicate 层 (Track E)** | | | | | |
| `src/top_white/predicates/ui_content.rs` | UIContentPredicate | 1 | ~250 | 7 | predicates/registry |
| `src/top_white/predicates/a2a_signature.rs` | A2AMessageSignatureVerifier | 1 | ~150 | 7 | crypto |
| `src/top_white/oracle/veto_ai_policy_gate.rs` | UIPolicyVetoRequest interface | 1 | ~200 | 7 | (VetoAI endpoint) |
| **Economy 派生视图 (Track D)** | | | | | |
| `src/runtime/agent_reputation.rs` | AgentReputation 派生只读 | 1 | ~400 | 7 | economic_state, agent_pnl |
| `src/runtime/designer_view.rs` | DesignerView (UI artifact reuse) | 1 | ~200 | 7 | cas, ui_event_capsule |
| `src/runtime/curator_view.rs` | CuratorView (endorsement) | 1 | ~200 | 7 | cas |
| `src/runtime/auditor_view.rs` | AuditorView (audit finding) | 1 | ~200 | 7 | cas |
| **Policy + Web layer (Phase 7+)** | | | | | |
| `src/runtime/policy_engine.rs` | Rust 原生 Policy Engine | 1 | ~600 | 7 | economic_state, registry |
| `src/webui/` (新目录) | event_bridge / ir_renderer / websocket / json_rpc | 1-2 | ~2500 | 7+ | tokio, axum, react frontend |
| **Test 层** | | | | | |
| `tests/constitution_tisr_ui_events.rs` | UIEventCapsule + UIContentPredicate gates | 1 | ~300 | 7 | (沿 fc_alignment_conformance 模式) |
| `tests/constitution_tisr_a2a_messages.rs` | A2A + verification asymmetry | 1 | ~400 | 7 | (同) |
| `tests/cli_integration_*.rs` | CLI end-to-end | 1 | ~600 | 6 | (各 subcommand) |

### 2.2 现有模块扩展 (向后兼容, Class 1)

| 模块 | 扩展类型 | LOC | Phase |
|---|---|---:|---|
| `src/runtime/markov_capsule.rs` | +3 optional fields (ui_event_cids / a2a_dialog_cids / artifact_manifest_cid) with serde default | ~30 | 7 |
| `src/runtime/librarian_broadcast.rs` | +3 LibrarianEvidenceKind variants (UIEvent / A2AMessage / ArtifactRef) | ~50 | 7 |
| `src/bin/audit_dashboard.rs` | 简化为薄 entry (~50 行) 调 lib | -3000 → ~50 | 6 |
| `src/bin/audit_tape.rs` 等 | 同上, 简化为薄 entry | -200 → ~50 each | 6 |
| `experiments/minif2f_v4/src/bin/lean_market.rs` | **保留**作向后兼容 (TB-10 evidence chain); 内部调 user_task_runner lib | 0 (保持) | 6 |

### 2.3 Class 分布总览

| Class | 新增 LOC | 现有扩展 LOC | 总计 |
|---|---:|---:|---:|
| Class 0 (docs) | 0 | 0 | 0 |
| Class 1 (additive helper / wrapper) | ~7500 | ~80 | ~7580 |
| Class 2 (production wire-up, sequencer call) | ~3500 | 0 | ~3500 |
| Class 3 (CAS schema 扩展) | ~1500 | ~30 | ~1530 |
| Class 4 (sequencer admission / typed_tx schema) | **0** | **0** | **0** |
| **总计** | **~12500** | **~110** | **~12610** |

**关键**: **0 个 Class 4 修改**. 全部 Class 1-3, 符合 Charter §5 Kill Condition 5 + 7 ✅

---

## 3. 协议层次清单

### 3.1 4 个核心协议层 (借用用户报告的 4 核心 + 1 修订)

| 层 | 用户报告命题 | TISR 修订 | 实施模块 |
|---|---|---|---|
| **Turing UI IR** | declarative UI schema | ✅ 采纳 + 落地为 CAS schema | `src/runtime/ui_event_capsule.rs` (含 IR field) |
| **Event Bridge** | UI action → typed action ABI | ✅ 采纳 + 走 sequencer 两个 ingress | `src/webui/event_bridge.rs` |
| **Policy Engine** | schema 校验 + 权限 + 审批 | ✅ 采纳 + Rust 原生 (不扩展 rules/engine.py) | `src/runtime/policy_engine.rs` |
| **Audit Store** | UI 版本 + 事件流 + 工件 + replay | ✅ 采纳 + 复用 ChainTape + CAS + replay_full_transition | (复用现有 + Track B 新 capsule) |
| **+1: A2A Protocol** | (报告未涉) | TISR 新增 (Track F + 轴 B) | `src/runtime/a2a_message_capsule.rs` + `librarian_broadcast.rs` |

---

## 4. Trust Root + STEP_B 影响评估

### 4.1 STEP_B 保护文件清单 (CLAUDE.md §12)

按 CLAUDE.md §12, STEP_B 协议适用以下文件:
- `src/kernel.rs` ✓ 不动
- `src/bus.rs` ✓ 不动
- `src/sdk/tools/wallet.rs` ✓ 不动
- `src/state/sequencer.rs` ✓ 不动
- `src/state/typed_tx.rs` ✓ 不动
- `src/bottom_white/cas/schema.rs` ⚠️ **扩展 enum variant** (向后兼容, serde default; 是否构成 STEP_B 修改?)
- canonical signing payload surfaces ✓ 不动

### 4.2 cas/schema.rs ObjectType 扩展评估

TISR Phase 7 拟新增 5-7 个 ObjectType variants (UIEventCapsule / A2AMessageCapsule / ArtifactStorageManifest / ImageBlob / AudioBlob / VideoBlob / Model3dBlob).

**风险评估**:
- ObjectType enum 扩展是 **向后兼容**: 旧 capsule (无新 variant 引用) 仍可 deserialize
- 但 enum discriminant 修改本身可能触发 STEP_B (架构师需 ratify)
- **保守建议**: Phase 7 启动前 surface 给架构师, 要求 ObjectType 扩展独立 Class 4 §8 ratification (即使是 forward-bound 向后兼容)

**Forward-bound 标记**: 本文档不构成 cas/schema.rs 扩展授权; Phase 7 启动前必须独立请求架构师 §8.

### 4.3 Trust Root rehash 需求

Phase 6 实施时若 lib 化重构涉及 STEP_B 文件 (audit_dashboard 等), 需要重新计算 Trust Root sha256. 当前估算无 STEP_B 文件修改需求 (audit_dashboard 不在 STEP_B 列表内). 但 Phase 7 触及 cas/schema.rs 必然触发 Trust Root rehash + 架构师 §8.

---

## 5. 实施排序 (Phase 6 → Phase 7 → Phase 8+)

### 5.1 Phase 6 (CLI MVP, Class 1-2, ~6-8 周)

**Pre-condition**: G-Phase 收口完成 + REAL-13 / REAL-BCAST-1 / REAL-13A ship.

**Surface 修改**: 0 Class 4. 全部 Class 1-2 (lib 化 + 新 bin).

**Deliverable**:
- `src/bin/turingos.rs` (~150 LOC) + `src/cli/` (~1000 LOC)
- 5 个 `runtime/*_runner.rs` lib 化重构 (~4750 LOC 移动)
- `runtime/user_task_runner.rs` + `runtime/user_view.rs` (~600 LOC 提自 lean_market)
- `runtime/provenance_capsule.rs` (~150 LOC) — human vs agent audit trail
- 现有 11 个 bin 简化为薄 entry (保留向后兼容)

**Real-problem witness**: 完整 12-step happy path + audit_tape PROCEED + replay 重建 HEAD_t

### 5.2 Phase 7 (Web MVP, Class 1-2 + new CAS variants, ~10-12 周)

**Pre-condition**: Phase 6 ship + 架构师批准 cas/schema.rs ObjectType 扩展 (§8).

**Surface 修改**: cas/schema.rs +5-7 variants (Class 4 candidate, 需独立 §8).

**Deliverable**:
- `src/webui/` (~2500 LOC) — Event Bridge / IR Renderer / WebSocket / JSON-RPC
- `runtime/ui_event_capsule.rs` (~250 LOC) — Class 3 CAS schema
- `runtime/a2a_message_capsule.rs` (~300 LOC) — Class 3 CAS schema
- `runtime/artifact_manifest.rs` (~400 LOC) — multimodal 2-tier + dual-hash
- `runtime/policy_engine.rs` (~600 LOC) — Rust 原生
- `runtime/agent_reputation.rs` + Designer/Curator/Auditor View (~1000 LOC)
- `top_white/predicates/ui_content.rs` + `a2a_signature.rs` (~400 LOC)
- `top_white/oracle/veto_ai_policy_gate.rs` (~200 LOC)
- Test: `tests/constitution_tisr_ui_events.rs` + `_a2a_messages.rs` (~700 LOC)
- React frontend (单独 npm package, ~5000+ LOC)

**Real-problem witness**: 3+ 独立用户通过 Web form 完成 task; ≥1 VetoAI Reject case; multimodal artifact upload ≥3 type.

### 5.3 Phase 8 (A2A 深化 + 协作 + 联邦, Class 2-3+, ~10-12 周)

**Pre-condition**: Phase 7 ship + REAL-N (N≥16) 涌现验证里程碑 ship.

**Surface 修改**: 可能涉及 typed_tx 扩展 (DirectSwapTx 等), Class 4 候选.

**Deliverable**:
- Task-granular lease (`task_lease.json`)
- DID 桥 (`did:turingos:<pubkey>`)
- MCP server 暴露 read-only tape view
- ChainTape Merkle 周期性外锚 (公链)
- TEE remote attestation (optional)
- Active Inference 角色 agent 参考实现 (optional, Track J Q2)

### 5.4 Phase 9+ (评估 / 选项, 长期)

- Yjs / CRDT 评估 (仅 read-side concurrent audit viewing)
- zkML proof in EvidenceCapsule (Class 3+, Track I Q1)
- 跨 TuringOS 实例联邦 (Track I Q5)
- agent-facing UI subpanels (Track C Phase 9 frontier)

---

## 6. 模块间数据流图

```
UI Event Flow (轴 A, Phase 7):
  User clicks button
    → React component
    → Event Bridge (Layer 1)
        ├── UIContentPredicate.check() — PCP {0,1}
        ├── VetoAI policy gate consult — Approve/Reject/Escalate
        └── Policy Engine.check() — runtime state + workflow rule
    → typed event dispatch (Layer 2)
        ├── 人发起 → emit_system_tx(SystemEmitCommand)
        └── agent_user_0 发起 → submit_agent_tx(TypedTx)
    → Sequencer admission + state advance (Layer 2)
    → ChainTape L4 entry (Layer 3)
        + UIEventCapsule CAS write (Layer 3)
        + ProvenanceCapsule CAS write (Layer 3, Track E Q2)
    → HEAD_t advance + cas_root update (Layer 3)
    → Dashboard / replay 可观察 (Layer 4)

A2A Message Flow (轴 B, Phase 7):
  Bull/Bear agent forms EconomicJudgment
    → CAS write (no typed_tx; existing REAL-12 path)
  Or: agent emits A2AMessageCapsule
    → CAS write + optional escrow (Track D Q6)
  → LibrarianBroadcast 选择并裁剪
    → BroadcastEpoch CAS write
    → role-scoped DerivedView 嵌入 agent prompt
  → Peer agents 读取 + form 自己的 EconomicJudgment
  → 统计 consensus extraction (Track E Q6)
  → Policy engine 推荐 (observe-only)
  → 若 agent commitment: submit_agent_tx(BuyWithCoinRouterTx 等)
  → state advance + chain anchor

Multimodal Artifact Flow (Phase 7, Track G):
  User uploads image
    → Web form multipart/form-data
    → Event Bridge dispatch
    → ContentPredicate (size + type whitelist)
    → ArtifactStorageManifest.put_artifact()
        ├── < 1 MB: BLAKE3 hash → CAS BLOB direct
        └── >= 1 MB: BLAKE3 + perceptual hash + CDN/IPFS ref
    → UIEventCapsule { artifact_cids: [...] } CAS write
    → L4.E or L4 entry with ui_context_cid (optional)
```

---

## 7. 风险矩阵 + 缓解策略

| 风险 | 影响 | 概率 | 缓解 |
|---|---|---|---|
| cas/schema.rs ObjectType 扩展触发 Class 4 §8 要求 | 中 | 高 | Phase 7 启动前 surface 给架构师, 提前 ratify |
| Yjs/CRDT 与 chain canonical ordering 冲突 | 低 | 已识别 | Phase 7 采 lease 模式; CRDT 延迟到 Phase 9+ |
| 多模态 artifact CAS 体积失控 | 中 | 中 | 2-tier 策略 + Blake3 content dedup + 外部 CDN 委托 |
| Phase 7 web from-scratch LOC 超预算 (~5000-10000) | 中 | 中 | 严格 IR + Materializer 边界; 不写自由 HTML |
| Phase 6 lean_market evaluator child fork 重构难度 | 中 | 中 | 保留 spawn 模式; 不强求 evaluator lib 化 |
| 真问题 witness 跟不上设计速度 ("design over evidence") | 高 | 中 | 每 Phase 出口强制 real-problem witness; Phase 5 ROADMAP 详细 witness 清单 |

---

## 8. Architecture Integration 结论

### 8.1 总体可行性
- **Phase 6 完全可行**: 0 Class 4; Class 1-2; ~5900 LOC; 6-8 周
- **Phase 7 主要可行**: cas/schema.rs ObjectType 扩展是 Class 4 候选 (需独立 §8); 其余 Class 1-3
- **Phase 8+ 长期可行**: 部分 Class 4 (typed_tx 扩展 / DirectSwapTx), 每个独立 §8

### 8.2 关键架构 invariants 维持
- ✅ TuringOS sequencer + typed_tx 0-touch (Phase 6); 仅 Phase 7+ cas/schema.rs ObjectType 扩展
- ✅ HEAD_t 6-field 不扩展
- ✅ CompleteSet 守恒不破
- ✅ price = signal not truth
- ✅ predicate {0,1} verdict
- ✅ AgentSignature + SystemSignature 二元论
- ✅ 0 个 Class 4 surface 修改提议 (全 forward-bound)

### 8.3 Phase 5 输出指导
本文档作为 `03_CODE_INTEGRATION_SPEC.md` 的核心输入. Phase 5 应:
- 每 module 详细 to-spec (interface + invariant + test)
- 每 Phase 清晰 milestone + 真问题 witness
- Trust Root 影响明确标注 (cas/schema.rs 是唯一需要 ObjectType 扩展, Class 4 候选)

**Architecture Integration 完成**: 5 Phase 2 track 合并为统一 4-layer 架构 + 25+ 新模块清单 + 12500 LOC 估算 + 0 Class 4 修改.
