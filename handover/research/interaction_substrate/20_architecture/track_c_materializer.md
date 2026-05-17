# Phase 2 / Track C — Materializer / Web Layer From-Scratch 设计

**本文档为 forward-bound research 提案；任何派生 TB 须独立 §8 ratification；本文档不构成已批准方案。**

**作者**: TISR Phase 2 Explore subagent (2026-05-17)
**Scope**: Web Layer **from-scratch** 设计 (不是 audit_dashboard.rs 扩展)

---

## 0. 关键 Scope 声明

`src/bin/audit_dashboard.rs` (3544 行) **纯 text-only** (line 1281 render_text). Phase 2 Track C 明确 **from-scratch web layer** — 不扩展 audit_dashboard, 不重用 16 个 render_section 函数. Phase 1 CLI-D 已确认 Phase 6 MVP 不实施 HTML; HTML/Web 全部 forward-bound to **Phase 7+**.

---

## 1. 任务 Scope

- Web layer 信息架构 (不是 audit_dashboard text section 一一映射)
- Turing UI IR schema 设计 (declarative; not user-submitted HTML/JS)
- Event Bridge ↔ sequencer 协调
- Policy Engine 选型 (扩展 rules/engine.py vs Rust 原生)
- 协作 UI 状态一致性 (chain_tape_lease vs OT/CRDT)

---

## 2. 现状盘点

### 2.1 audit_dashboard.rs text-only (3544 行)

- 16 个 render_section_* (line 1281-1550 render_text)
- 模式: section 顺序 + name-value pairs + ASCII status (✓/✗)
- 已支持 `--json` 输出 (但仍是 serialize, 不是 UI 中间表示)
- 16 sections 含: Run metadata / Chain stats / ChainDerivedRunFacts / Per-agent activity / Proposal flow / Branch lineage / Verification summary / Claims audit / User tasks / Exhausted / Exposures / Oracle verified / Error taxonomy / ...

### 2.2 rules/engine.py (154 行)

- 文件级别 grep-based rule check
- check_type: grep / grep_inverse / compound
- trigger: pre_edit / pre_commit
- 零 UI 感知; 零 permission model; YAML 配置驱动
- **关键缺失**: 不适合 runtime state 决策; 不适合 multi-step workflows; 不适合 economic 感知

### 2.3 snapshot.rs (immutable read-view)

- UniverseSnapshot for agents: tape + price_index + mask_set + sequencer_wired + generation/tx_count
- Art. III.3 decorrelation via independent snapshots
- Agents 只读不写

### 2.4 chain_tape_lease.rs (单写文件锁)

- 守护 refs/transitions/main 前进
- atomic file write via tempfile + rename
- 6 个 SG-G1.2-2.* gates (acquire_release / reject_second_writer / detect_stale_lock / etc.)
- Sequential-batch use only; concurrent expansion forward to G5+

### 2.5 audit_views.rs (4 个 pure view-aggregator)

- audit_view_shares / audit_view_pools / audit_view_prices / audit_view_positions
- 全部 `&EconomicState` immutable ref; replay-deterministic; integer-only
- 与 L5 materializer 共享"只读投影"语义但 scope 更窄

### 2.6 CO1_8_MATERIALIZER_v1_2026-04-29 状态

DEFERRED (Codex VETO/HIGH + Gemini CHALLENGE/HIGH). Track C 与延迟 L5 物化层独立 — L5 是 agent read surface (state projection); Track C 是 L0+ user interaction substrate (人机交互层). 栈中独立.

### 2.7 grep 验证: 无前端代码

(本 sub-agent 已 grep frontend / web / html / react / svelte / vue / iframe 等关键词; **confirmed 无 web 代码**).

---

## 3. Q1: 16 section 文本 → UI Component 映射 vs from-scratch

**答**: **不应**沿 audit_dashboard 16 section 直接映射为 16 web component. 应 **from-scratch UI 信息架构**.

### 3.1 理由

1. **设计语言根本不同**:
   - Text: 顺序段落, 嵌套缩进, 格式符号 (✓/✗), 适合 sequential reading
   - Web: 面板, 卡片, 表格, 图表, drill-down, 适合 spatial scanning + interactive navigation

2. **信息结构转换**:
   - "§2 Chain stats + 7 indicators" text → Web 应拆为: top-level summary card + expandable verification indicator table + 可选 verification timeline

3. **交互 affordance**:
   - "§5 Proposal flow (chronological list)" → Web 应支持 filter (by agent_id / tx_kind / status) + sort + drill-down + replay 链接

### 3.2 4-Layer Web Structure (from-scratch)

**Layer 0: Navigation Hub** (landing)
- Run selector (run_id dropdown / timeline)
- Quick status cards (verification / tx count / oracle %)
- 4 major workflow buttons

**Layer 1: Audit Overview** (role-scoped dashboard per REAL-5)
- Chain integrity summary (§2 → status board)
- Run facts snapshot (§3 → expandable panel)
- Agent roster (§4 → sortable table with filter)

**Layer 2: Transaction Flow** (temporal + causal visualization)
- Timeline view (§5 + §6 combined)
- 3 view modes: chronological list / DAG visualization / filter+search
- Action: click tx → side panel with details

**Layer 3: Audit Details** (specialized panels)
- 8 expandable panels for §7-§16 (claims / user tasks / exhausted / expired / exposures / oracle verified / error taxonomy / ...)
- 每 panel: filterable table + export button

### 3.3 Implementation Phasing

- **Phase 7.0 MVP**: Layers 0-1 (hub + overview)
- **Phase 7.1 Extended**: Layer 2 (transaction flow)
- **Phase 8+**: Layer 3 (specialized panels)

---

## 4. Q2: Turing UI IR Schema 设计

**答**: Turing UI IR 是 **declarative data structure**, 不是 user-submitted HTML/JS. 类型安全的人机交互协议. 应落地为新 **CAS schema** (Class 1-2 level).

### 4.1 IR v1 Schema (JSON 示例)

```json
{
  "version": "1.0",
  "page_id": "audit_dashboard_run_detail",
  "title": "Audit Dashboard — Run {run_id}",
  "permission_context": {
    "role": "auditor",
    "run_id": "run_2026-05-17_001",
    "visibility_tags": ["SYSTEM", "AGENT_ACTIVITY", "MARKET_TRACE"]
  },
  "sections": [
    {
      "section_id": "overview",
      "type": "card_grid",
      "rows": [
        { "label": "Run ID", "value": "{run_id}", "copy_button": true },
        { "label": "Verification Status", "value": "GREEN|RED", "type": "badge" }
      ]
    },
    {
      "section_id": "proposal_flow",
      "type": "timeline_or_table",
      "filters": [
        { "key": "agent_id", "type": "select" },
        { "key": "tx_kind", "type": "select" }
      ],
      "columns": [
        { "key": "logical_t", "type": "number" },
        { "key": "agent_id", "type": "string" },
        { "key": "tx_kind", "type": "enum" }
      ],
      "rows_source": { "type": "structured_data", "path": "proposal_flow[*]" },
      "actions": [
        {
          "action_id": "drill_down_tx_detail",
          "label": "View Details",
          "type": "open_panel",
          "target_panel": "tx_detail",
          "params": { "tx_id": "{logical_t}" }
        },
        {
          "action_id": "replay_from_tx",
          "label": "Replay from Here",
          "type": "navigation",
          "target_url": "/replay?start_logical_t={logical_t}",
          "permission_required": ["AUDITOR"]
        }
      ],
      "permissions": ["SYSTEM", "AUDITOR"]
    }
  ],
  "side_panels": [
    {
      "panel_id": "tx_detail",
      "type": "detail_view",
      "params": ["tx_id"],
      "content": {
        "type": "structured_view",
        "fields": [
          { "label": "TX ID", "value": "{tx_id}" },
          { "label": "CAS CID", "value": "{payload_cid}", "copy_button": true }
        ]
      },
      "actions": [
        { "action_id": "download_payload", "type": "export", "format": "json" }
      ]
    }
  ],
  "export_config": {
    "formats": ["json", "csv", "pdf"],
    "filename_template": "audit_run_{run_id}_{timestamp}"
  },
  "replay_anchors": [
    {
      "anchor_id": "replay_full_run",
      "type": "replay_link",
      "permission_required": ["AUDITOR"]
    }
  ]
}
```

### 4.2 IR 核心特点

1. **Declarative**: 描述"应该显示什么"; 不涉及"如何渲染"
2. **Permission-tagged**: 每 section/action 带 permission_required
3. **Data-bound**: rows_source 指向 structured_data path (不硬编码)
4. **Action-anchored**: actions 数组定义可用操作 (drill-down/replay/export)
5. **Panel-based**: side panels 作为 modal/drawer 补充

### 4.3 新 CAS Schema (Class 1-2 forward-bound)

```rust
// src/bottom_white/ui_ir/schema.rs (新文件, Phase 7+)
pub struct TuringUiIrPage {
    pub version: String,
    pub page_id: String,
    pub title: String,
    pub permission_context: PermissionContext,
    pub sections: Vec<UiSection>,
    pub side_panels: Vec<SidePanel>,
    pub export_config: ExportConfig,
    pub replay_anchors: Vec<ReplayAnchor>,
}
```

**Rationale**: IR 序列化到 CAS 使其成为 chain artifact 一部分, 便于 audit trail / 多 agent 协作 UI 一致性 / 离线 audit 完整性.

---

## 5. Q3: Event Bridge ↔ Sequencer Ingress 协调

**答**: Event Bridge 是 L0 user interaction 的 **受控网关**, 职责:
1. UI Action → Typed Event (deserialization + validation)
2. Permission check (role + visibility_tags)
3. Type safety (no arbitrary JS execution)

### 5.1 协调路径

```
┌──────────────────────────────────────────────────┐
│ UI Layer (React + Web Components)                │
│ [User clicks button]                             │
└──────────────────┬───────────────────────────────┘
                   │ UI Action Event
                   ▼
┌──────────────────────────────────────────────────┐
│ Event Bridge (L0 control layer)                  │
│ 1. Validate against component allowlist           │
│ 2. Check user permission (role + visibility)      │
│ 3. Type-dispatch to TypedTx factory               │
│ 4. Sign with user keypair (if required)           │
└──────────────────┬───────────────────────────────┘
                   │ Typed Event (SystemEmitCommand / TypedTx)
                   ▼
┌──────────────────────────────────────────────────┐
│ Sequencer (existing, NO MODIFICATION)            │
│ - emit_system_tx (人类入口, Track A Q1)            │
│ - submit_agent_tx (agent_user_0 入口, Track A Q4)  │
└──────────────────────────────────────────────────┘
```

### 5.2 与现有 sequencer 的关系

- **Event Bridge** (新, Phase 7+): UI action validation + permission check + type dispatch
- **Sequencer admission rules** (现有, STEP_B frozen): TX-level admission (economics, state preconditions)
- **Policy Engine** (Q4 详述): 高层规则 (multi-step workflows, conditional logic)

### 5.3 避免的反模式

- ❌ JSON-to-TypedTx pass-through (无验证就是危险)
- ❌ HTML template render engine (那是 Materializer 职责)
- ❌ AI planner (那是 orchestrator agent 职责)

**Event Bridge = declarative type gateway**. 唯一职责: 确保 UI 事件 + Sequencer TypedTx ABI 一致.

---

## 6. Q4: Policy Engine — 扩展 rules/engine.py vs Rust 原生

**答**: **新建 Rust 原生 policy engine** (与现有 rules/engine.py 并存, 不扩展).

### 6.1 现有 rules/engine.py 限制

- 文件级别 grep pattern match (不适合 runtime state)
- YAML 配置驱动 (不适合 multi-step workflows)
- pre_edit / pre_commit 离线 trigger
- 输出 WARN / BLOCK (passive guard, 非 active controller)

### 6.2 Policy Engine v1 设计 (Rust 原生, Phase 7+)

```rust
// src/runtime/policy_engine.rs (新文件)

pub struct PolicyEngine {
    pub rules: Vec<PolicyRule>,
    pub state: &'static EconomicState,
}

pub enum PolicyRule {
    // Admission rules (typed_tx.rs admission family 推广)
    Admission {
        target_tx_kind: TxKind,
        condition: Box<dyn Fn(&EconomicState, &TypedTx) -> PolicyDecision>,
    },
    // Workflow rules (sequential gate)
    Workflow {
        workflow_id: String,
        steps: Vec<WorkflowStep>,
    },
    // Economic rules (price signals, budget constraints)
    Economic {
        constraint_type: EconomicConstraintType,
        check_fn: Box<dyn Fn(&EconomicState) -> bool>,
    },
}

pub enum PolicyDecision {
    Allow,
    Deny(String),
    RequireApproval(ApprovalContext),
}
```

### 6.3 与现有体系协调

```
┌─────────────────────────────────────────┐
│ rules/engine.py (文件级别预检)          │
│ Used by: .claude/hooks/judge.sh         │
│ Trigger: pre_edit / pre_commit          │
│ Environment: 离线, 无 runtime state     │
└─────────────────────────────────────────┘
          ↓ [lint failures] → REJECT commit (Phase 0 已存在)

┌─────────────────────────────────────────┐
│ Policy Engine (runtime state + workflow)│
│ Invoked by: Sequencer.apply_one (post)  │
│ Trigger: post-TypedTx-formation         │
│ Environment: 在线, 完整 state access     │
└─────────────────────────────────────────┘
          ↓ [policy check] → Allow / Deny / RequireApproval
```

### 6.4 与 sequencer admission 关系

**不**替换现有 sequencer admission. 而**之后**作为可选 higher-level policy:

```rust
typed_tx = parse_and_admit()?;       // existing sequencer admission
policy_engine.check(&typed_tx, &state)?;  // 新 policy layer
state = materializer::apply(...)?;
```

**不建议合并** rules/engine.py 与 Policy Engine. 分离提高清晰度和测试性.

---

## 7. Q5: 协作 UI 状态一致性

**答**: **Phase 7 MVP sequential lease (现有 chain_tape_lease 扩展)**, **Phase 8+ multi-granular lease**, **不采用 CRDT (Yjs/Automerge)**.

### 7.1 现有 chain_tape_lease 能力

- 单写文件锁 chain_tape_lease.json
- start_head_t_hex 记录 lease 获得时 HEAD logical_t
- kill(holder_pid, 0) 失效检测
- Sequential batch 模式; 未来 G5+ 扩展

### 7.2 Web Layer 协作挑战

多用户同时编辑 UI panel:
1. Read-side: multiple readers 看同 snapshot ✓ (已覆盖)
2. Write-side: User A approve + User B reject 同时提交 → 仲裁?
3. UI State sync: 两 tab 打开同 run_id, 一 tab 点击后另一 tab 自动刷新?

### 7.3 Phase 7 MVP (不支持并发)

- Event Bridge 序列化提交 (同一时刻只有一用户可提交)
- UI 用 WebSocket long-poll 更新 (无 real-time sync)
- Conflicting actions (A approve + B reject) 由 sequencer admission rule 仲裁 (后者被拒)

### 7.4 Phase 8+ Extended (支持并发)

- **不**使用 CRDT/OT, 而是扩展 chain_tape_lease 模式
- lease 粒度从"整个 chain" 细化为"某个 task_id"
- 例: task_lease.json 保护特定 task 修改权
- 若 User A hold task_123_lease, User B 修改 task_123 时显示 "locked by User A"

### 7.5 不采用 CRDT 的理由

1. **Chain 是源真相**: UI state 最终回溯到 chain tx; CRDT 的"最终一致"与 chain 的"canonical ordering" 冲突
2. **Economic state 需严格仲裁**: approve vs reject 互斥; CRDT 无法 resolve
3. **Replay 需确定性**: CRDT merge 引入 non-determinism, 破坏 replay

### 7.6 路径建议

```
Phase 7 MVP:
  lease = chain_tape_lease (guard whole chain)
  UI = request-response (WebSocket + long-poll)
  Conflict = sequencer admission rule (last-write-wins)

Phase 8 Extended:
  lease = multi-granular (task / event level)
  UI = request-response + optimistic updates
  Conflict = same admission rule (cleaner UX)

Phase 9+ (评估):
  Yjs 仅当:
    (a) read-side concurrent audit viewing 需要 real-time sync, AND
    (b) write-side conflicts 可安全委托 CRDT merge semantics
        (经济决策不太可能)
```

---

## 8. From-Scratch Web Stack 候选

### 8.1 核心栈 (Phase 7 MVP)

```
Frontend:
  - React 18.x (UI composition)
  - TypeScript 5.x (type safety)
  - Web Components (custom elements for isolated audit panels)
  - Tailwind CSS (utility-first; avoid runtime CSS generation)

State Management:
  - TanStack Query (server-side data sync)
  - Zustand (client-side UI state; 最小 footprint)
  - NOT Redux (overkill for Phase 7)

Networking:
  - WebSocket (bidirectional event streaming)
  - JSON-RPC 2.0 (API contract)
  - Protobuf (optional; Phase 8+)

Visualization:
  - Recharts (charting; deterministic rendering)
  - react-flow-renderer (DAG/branch lineage)
  - NOT D3.js (harder deterministic for replay)

Build:
  - Vite (HMR)
  - esbuild (fast bundle)
  - CSP-compliant (no inline scripts)

Testing:
  - Vitest (unit)
  - Playwright (E2E against TuringOS)
```

### 8.2 不推荐

- ❌ Vue.js (用户报告 React 主线)
- ❌ Angular (体积太大)
- ❌ SvelteKit (Web Components integration 不如 React)
- ❌ Dynamic HTML generation by Claude (用户报告禁止; 必走 IR → Materializer)

### 8.3 Backend 集成 (Rust)

```rust
// src/webui/ (新目录, Phase 7+)
├── mod.rs                      # web server orchestration
├── event_bridge.rs             # Q3 Event Bridge impl
├── ui_ir_renderer.rs           # IR → JSON serialization
├── websocket_handler.rs        # WebSocket upgrade + subscription
├── json_rpc.rs                 # JSON-RPC 2.0 dispatch
└── policy_engine.rs            # Q4 Policy Engine impl
```

---

## 9. Phase 6 (CLI) ↔ Phase 7+ (Web) 关系

User clarification: "可以先研究一下是否先设计一个 cli, 可以先使用上"

**CLI 应该是**:
- 专家回退入口 (expert-mode fallback)
- 与 web UI **共享 Event Bridge 和 Policy Engine**
- 支持 scripting + automation (Web UI 不支持)

**CLI 不应该是**:
- audit_dashboard 扩展 (那仍是 text-only)
- 功能完整替代品 (web UI 是一等公民, Phase 7+)

### 协调架构

```
Phase 6 (CLI-first):
  turingos task open --problem X
  → Event Bridge (shared)
  → submit_agent_tx / emit_system_tx
  → sequencer

Phase 7 (HTML-first):
  Web UI [button: Create Task]
  → React action handler
  → Event Bridge (shared)
  → WebSocket JSON-RPC
  → submit_agent_tx / emit_system_tx
  → sequencer
```

**共享**: Event Bridge + Policy Engine
**差异**: UI 表现层 (CLI text vs Web HTML)

---

## 10. Phase 7+ Web Layer 路线图初稿

### Phase 7.0: MVP Foundation (8 周)

- Event Bridge + Typed Action ABI
- Turing UI IR schema + Materializer (React)
- WebSocket + JSON-RPC backend integration
- Audit Overview + Transaction Flow visualization
- Permission/role model (REAL-5)

**Release gate**: Phase 6 CLI 已 ship + user 可在 web UI 完成 audit 任务

### Phase 7.1: Extended Features (4 周)

- Drill-down detail panels
- Export (JSON/CSV/PDF)
- Replay anchors (UI 直接触发 replay)
- Accessibility (WCAG 2.1 AA)

### Phase 8.0: Advanced Collaboration (10 周)

- Task-granular leasing (Q5)
- Optimistic update UI pattern
- Real-time sync (WebSocket broadcast)
- Conflict resolution UI

### Phase 9+: Frontier

- Agent-facing UI subpanels (agents 可在 workspace 交互)
- zkML proof visualization
- Market microstructure real-time dashboard

---

## 11. Kill Condition 5/7 自检

### Kill Condition 5: "可落地的代码层面"
- ✅ Q1: UI 信息架构 (4 layer)
- ✅ Q2: IR schema (JSON + Rust struct)
- ✅ Q3: Event Bridge integration path
- ✅ Q4: Policy Engine (Rust 原生, 非 Python 扩展)
- ✅ Q5: Collaboration model (lease-based)
- ✅ 全部指向具体 file:line / module path

### Kill Condition 7: "从用户角度完整工作流"
- ✅ Audit Overview (read-only)
- ✅ Transaction Flow + Drill-down (interactive)
- ✅ Replay anchors
- ✅ Permission-scoped views (REAL-5)
- ✅ Phase 6 CLI 并行设计 (专家回退)

### 0 个 Class 4 surface 修改提议
- ✅ Event Bridge 不修改 sequencer
- ✅ Policy Engine 不修改现有 admission
- ✅ UI IR 是新 CAS schema (Class 1-2)
- ✅ Web stack 全 from-scratch (Phase 7+)

---

## 12. Track C 完成

关键点:
1. **不是 audit_dashboard 扩展** — 完全 from-scratch 信息架构
2. **Turing UI IR 作为中间表示** — 类型安全的人机交互协议; 序列化到 CAS
3. **Event Bridge + Policy Engine** — 受控的 UI-to-chain gateway
4. **Sequential lease 支持未来并发** — 阶段化, 不过度设计
5. **React + Web Components + Tailwind** — 符合用户报告主线, 禁止任意 HTML 生成
6. **Phase 6 CLI + Phase 7 Web 双轨** — 共享 Event Bridge 和 Policy Engine, UI 差异

5 个问题全回答; 具体 schema + 架构图 + forward-bound phases. 准备 Phase 7+ 实施 (待 Phase 5 deliverable 完成 + 架构师批准).
