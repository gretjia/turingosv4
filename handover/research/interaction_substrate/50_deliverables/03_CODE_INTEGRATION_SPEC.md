# TISR Deliverable 03 — Code Integration Spec

**本文档为 forward-bound research 提案；任何派生 TB 须独立 §8 ratification；本文档不构成已批准方案。**

**目的**: TISR Phase 6+ 实施的代码级集成规范; 每模块的 interface + invariant + test + Class + Phase. 详细技术规范见 Phase 2 Track A-E + Phase 4 architecture_integration.

---

## 1. 总览

- **Phase 6 LOC**: ~5900 (Class 1-2; 0 Class 4)
- **Phase 7 LOC**: ~10000+ (Class 1-3; cas/schema.rs +5 ObjectType variants 是 Class 4 候选, 需独立 §8)
- **Phase 8+ LOC**: ~5000+ (Class 2-3; 部分 typed_tx 扩展 Class 4 候选)
- **总新增/重构 LOC**: ~21000 (3 Phase 合计)

---

## 2. Phase 6 模块清单 (CLI MVP, Class 1-2)

### 2.1 CLI 入口层

| 文件 | LOC | Class | Interface | 测试 |
|---|---:|---|---|---|
| `src/bin/turingos.rs` | ~150 | 1 | clap parent + global flags + dispatch | `tests/cli_init_smoke.rs` |
| `src/cli/mod.rs` | ~50 | 1 | 公共类型 + re-export | (内嵌) |
| `src/cli/args.rs` | ~100 | 1 | clap derive structs | (clap 测试) |
| `src/cli/format.rs` | ~200 | 1 | text/json/html (Phase 7 stub) Renderable trait | `tests/cli_format_roundtrip.rs` |
| `src/cli/config.rs` | ~150 | 1 | toml load + env var + default | `tests/cli_config_load.rs` |
| `src/cli/error.rs` | ~80 | 1 | TuringosCliError + exit code | (单元) |
| `src/cli/commands/*` (16 文件) | ~1000 | 1 | 每 subcommand 一文件 | 集成测试 |

### 2.2 Lib runner 层 (lib 化重构)

| 文件 | LOC | Class | Source | Interface |
|---|---:|---|---|---|
| `src/runtime/audit_dashboard_report.rs` | ~3000 | 1 | 移自 `bin/audit_dashboard.rs:build_report` | `pub fn build_report(repo, cas, markov_cid) -> DashboardReport` |
| `src/runtime/audit_tape_runner.rs` | ~250 | 1 | 移自 `bin/audit_tape.rs` | `pub fn run_audit(opts: AuditTapeOpts) -> TapeVerdict` |
| `src/runtime/audit_tape_tamper_runner.rs` | ~350 | 1 | 移自 `bin/audit_tape_tamper.rs` | `pub fn run_tamper_harness(opts) -> TamperReport` |
| `src/runtime/markov_capsule_runner.rs` | ~350 | 1 | 移自 `bin/generate_markov_capsule.rs` | `pub fn generate(opts) -> Result<MarkovEvidenceCapsule>` |
| `src/runtime/user_task_runner.rs` | ~400 | 2 | 提自 `lean_market.rs:cmd_run_task` | `pub fn open_task(spec, sponsor_keystore) -> Result<TaskOpenReceipt>` |
| `src/runtime/user_view.rs` | ~200 | 1 | 提自 `lean_market.rs:cmd_view_*` | `pub fn view_task / view_wallet / view_positions / view_bankruptcy` |
| `src/runtime/export_runner.rs` | ~200 | 1 | 新增 | `pub fn export_bundle(repo, cas, opts) -> Result<()>` |
| `src/runtime/provenance_capsule.rs` | ~150 | 1 | 新增 (Track E Q2) | ProvenanceCapsule struct + writer |

### 2.3 现有 bin 简化 (保留向后兼容)

```rust
// src/bin/audit_dashboard.rs (简化前 3544 行; 简化后 ~50 行)
fn main() {
    let args = parse_args();
    let report = runtime::audit_dashboard_report::build_report(&args.repo, &args.cas, ...).expect(...);
    let output = match args.format {
        Json => serde_json::to_string_pretty(&report).expect(...),
        Text => audit_dashboard_report::render_text(&report),
    };
    // write to --out or stdout
}
```

10 个现有 bin 同此模式简化.

---

## 3. Phase 7 模块清单 (Web MVP, Class 1-3)

### 3.1 Capsule schema 层 (Class 3 forward-bound)

| 文件 | LOC | Class | Interface |
|---|---:|---|---|
| `src/runtime/ui_event_capsule.rs` | ~250 | 3 | UIEventCapsule + UIContentClass + UIEventOutcome + CAS writer; schema id `tisr.ui_event_capsule.v1` |
| `src/runtime/a2a_message_capsule.rs` | ~300 | 3 | A2AMessageCapsule + A2AMessageKind + VerificationScope + CAS writer; schema id `tisr.a2a_message.v1` |
| `src/runtime/artifact_manifest.rs` | ~400 | 3 | ArtifactStorageManifest + ArtifactMetadata + 2-tier storage + dual-hash (BLAKE3 + perceptual) |
| `src/runtime/ui_artifact_capsule.rs` | ~200 | 3 | UIArtifactCapsule (Designer reuse signal); schema id `turingosv4.ui_artifact.v1` |
| `src/runtime/curator_endorsement.rs` | ~150 | 3 | CuratorEndorsement; schema id `turingosv4.curator_endorsement.v1` |
| `src/runtime/audit_finding.rs` | ~150 | 3 | AuditFinding; schema id `turingosv4.audit_finding.v1` |

### 3.2 CAS schema 扩展 (Class 4 候选, 需独立 §8)

```rust
// src/bottom_white/cas/schema.rs (+50 行)
pub enum ObjectType {
    // ...existing 21+ variants...
    UIEventCapsule,           // [NEW Phase 7]
    A2AMessageCapsule,        // [NEW Phase 7]
    ArtifactStorageManifest,  // [NEW Phase 7]
    ImageBlob,                // [NEW Phase 7]
    AudioBlob,                // [NEW Phase 7]
    VideoBlob,                // [NEW Phase 7]
    Model3dBlob,              // [NEW Phase 7]
}
```

**风险**: cas/schema.rs 是 STEP_B frozen surface. Phase 7 启动前**必须**独立请求架构师 §8 ratify. 若未批准, Phase 7.0 fallback 用 Generic ObjectType + 嵌套 schema_id.

### 3.3 Predicate 层 (Class 1 forward-bound)

| 文件 | LOC | Class | Interface |
|---|---:|---|---|
| `src/top_white/predicates/ui_content.rs` | ~250 | 1 | UIContentPredicate {0,1} verdict; 4 content_class | 
| `src/top_white/predicates/a2a_signature.rs` | ~150 | 1 | A2AMessageSignatureVerifier |
| `src/top_white/oracle/veto_ai_policy_gate.rs` | ~200 | 1 | UIPolicyVetoRequest + VetoAIUIDecision enum + consult fn |

### 3.4 Economy 派生视图 (Class 1)

| 文件 | LOC | Class | Interface |
|---|---:|---|---|
| `src/runtime/agent_reputation.rs` | ~400 | 1 | AgentReputation struct + compute_reputation 派生函数 |
| `src/runtime/designer_view.rs` | ~200 | 1 | DesignerView + compute_designer_view |
| `src/runtime/curator_view.rs` | ~200 | 1 | CuratorView + compute_curator_view |
| `src/runtime/auditor_view.rs` | ~200 | 1 | AuditorView + compute_auditor_view |

### 3.5 Web Layer (Class 1-2 from-scratch)

```
src/webui/
├── mod.rs                      # web server orchestration (~50 LOC)
├── event_bridge.rs             # UI Action → Typed Event (~400 LOC, Class 1-2)
├── ui_ir_renderer.rs           # Turing UI IR → JSON serialization (~300 LOC, Class 1)
├── websocket_handler.rs        # WebSocket upgrade + subscription (~400 LOC, Class 1)
├── json_rpc.rs                 # JSON-RPC 2.0 dispatch (~300 LOC, Class 1)
└── policy_engine.rs            # Rust 原生 policy engine (~600 LOC, Class 1)
```

**Total Web Layer (Rust)**: ~2050 LOC

**Frontend (单独 npm package)**: ~5000+ LOC (React + TypeScript + Tailwind + Web Components; 不在 cargo workspace)

### 3.6 现有模块扩展 (向后兼容, Class 1)

```rust
// src/runtime/markov_capsule.rs (+30 行)
pub struct MarkovEvidenceCapsule {
    // ...existing fields...
    #[serde(default)] pub ui_event_cids: Vec<Cid>,            // [NEW]
    #[serde(default)] pub a2a_dialog_cids: Vec<Cid>,          // [NEW]
    #[serde(default)] pub artifact_manifest_cid: Option<Cid>, // [NEW]
}

// src/runtime/librarian_broadcast.rs (+50 行)
pub enum LibrarianEvidenceKind {
    // ...existing variants...
    UIEvent,        // [NEW]
    A2AMessage,     // [NEW]
    ArtifactRef,    // [NEW]
}
```

---

## 4. Phase 8+ 模块清单 (A2A 深化, Class 2-4)

### 4.1 DID 桥 (Class 1-2)

| 文件 | LOC | Interface |
|---|---:|---|
| `src/runtime/did_bridge.rs` | ~300 | `did:turingos:<pubkey>` URI resolver + W3C DID v1.1 compat |
| `src/runtime/external_anchor.rs` | ~250 | ChainTape Merkle root 周期性外锚到公链 (optional, Class 3) |

### 4.2 MCP Server (Class 1-2)

| 文件 | LOC | Interface |
|---|---:|---|
| `src/mcp_server/mod.rs` | ~400 | MCP server 暴露 read-only tape view (跨实例 agent 互操作) |
| `src/mcp_server/handlers.rs` | ~600 | get_tape / get_capsule / get_dashboard handlers |

### 4.3 Future Class 4 候选 (Phase 9+ AGI)

| 候选 | Class | 风险 | 触发条件 |
|---|---|---|---|
| AgentProposedTaskOpen typed_tx variant | 4 | sequencer admission barrier 扩展 | agent 自治市场需求 |
| AgentMarketSeeding typed_tx variant | 4 | (同上) | (同上) |
| DirectSwapTx typed_tx variant | 4 | 多 agent 原子 swap; covenant 验证 | agent 间合约自动化 |
| HumanSignature type | 4 | 人类 PKI + signature 二元改三元 | 跨实例人代理 |
| New AgentRole variants | 4 | enum 扩展 + classifier 修改 | Designer/Curator 角色制度化 |

---

## 5. Test 架构

### 5.1 单元测试 (每模块必带)

```rust
// 模板: src/runtime/<module>.rs
#[cfg(test)]
mod tests {
    // - happy path
    // - failure path (CAS missing / repo missing / invalid args)
    // - boundary (size limit / null / empty)
    // - shielding (public_summary forbid raw)
}
```

### 5.2 集成测试 (Phase 6 + 7)

```
tests/
├── cli_init_smoke.rs                       # Phase 6: turingos init
├── cli_batch_lifecycle.rs                  # Phase 6: new → start → add → view
├── cli_audit_e2e.rs                        # Phase 6: batch → audit → verify
├── cli_lean_market_compat.rs               # Phase 6: 老 lean_market 命令兼容
├── constitution_tisr_ui_events.rs          # Phase 7: UIEventCapsule + Predicate
├── constitution_tisr_a2a_messages.rs       # Phase 7: A2A + verification asymmetry
├── constitution_tisr_provenance.rs         # Phase 6: human vs agent audit trail
├── constitution_tisr_reputation.rs         # Phase 7: AgentReputation 派生
├── constitution_tisr_policy_engine.rs      # Phase 7: Policy Engine
├── cli_web_ir_roundtrip.rs                 # Phase 7: UI IR serialize/deserialize
├── cli_web_bridge_dispatch.rs              # Phase 7: Event Bridge
└── cli_phase7_e2e.rs                       # Phase 7: full Web workflow
```

### 5.3 真问题 Witness (Phase 6+ 验收)

按 `feedback_real_problems_not_designed`:
- Phase 6: 完整 12-step happy path 跑通 + evidence bundle PROCEED + replay 重建
- Phase 7: ≥ 3 用户 Web form 提交 + ≥ 1 VetoAI Reject + ≥ 3 multimodal type
- Phase 8: ≥ 1 DID resolve + ≥ 1 MCP server query
- Phase 9: REAL-N (N≥16) batch run + 涌现现象记录

存储: `handover/evidence/stage_phase{N}_*`

---

## 6. Cargo.toml 配置

```toml
[package]
name = "turingosv4"
# ...existing...

[[bin]]
name = "turingos"
path = "src/bin/turingos.rs"

# ...其他 10 个 bin 保留...

[dependencies]
# 现有...
clap = { version = "4.5", features = ["derive", "env"] }  # NEW
thiserror = "1"                                          # 已有
tokio = { version = "1", features = ["full"] }           # 已有
axum = "0.7"                                             # Phase 7 NEW
tower-http = "0.5"                                       # Phase 7 NEW
serde_json = "1"                                         # 已有
blake3 = "1"                                             # Phase 7 NEW (multimodal dedup)
```

---

## 7. 实施约束

### 7.1 不做的事 (硬约束)

- ❌ 不修改 `src/state/typed_tx.rs` (Class 4 STEP_B frozen)
- ❌ 不修改 `src/state/sequencer.rs` admission rules (Class 4)
- ❌ 不修改 `src/economy/` 经济状态 mutator
- ❌ 不引入 f64 money path
- ❌ 不破坏 CompleteSet (1 Coin = 1 YES + 1 NO) 守恒
- ❌ 不让 price 进 admission predicate (Art. II.2)
- ❌ 不让 reputation 进 admission predicate (Art. III.4)
- ❌ 不引入新 signature type (二元 SystemSignature + AgentSignature)
- ❌ 不写 dynamic HTML (用户报告禁; 走 Turing UI IR + Materializer)

### 7.2 Pre-condition

- ✋ G-Phase 收口 (SG-G overall §8) 完成
- ✋ REAL-13 / REAL-BCAST-1 / REAL-13A ship
- ✋ Phase 6 charter §8 ratification (TISR research charter 之外的独立 charter)
- ✋ Phase 7 cas/schema.rs ObjectType 扩展 §8 ratification (独立 Class 4)
- ✋ Phase 8+ 每 typed_tx 扩展独立 Class 4 §8

---

## 8. 完整详细规范引用

本文档为概要; 完整代码级规范见:

- Phase 2 Track A (`20_architecture/track_a_typed_tx.md`) — 0-touch sequencer 边界
- Phase 2 Track B (`track_b_evidence_chain.md`) — CAS schema 详细 + replay 算法
- Phase 2 Track C (`track_c_materializer.md`) — Web layer from-scratch 设计
- Phase 2 Track D (`track_d_economy.md`) — 经济学集成 + 派生视图
- Phase 2 Track E (`track_e_predicate.md`) — Predicate + VetoAI + 测试草案
- Phase 4 Architecture Integration (`40_synthesis/architecture_integration.md`) — 统一架构 + 模块矩阵

**Code Integration Spec 完成**: ~21000 LOC 估算; 0 个 Class 4 修改 (实施阶段); 7 个 Class 4 候选 forward-bound 标明.
