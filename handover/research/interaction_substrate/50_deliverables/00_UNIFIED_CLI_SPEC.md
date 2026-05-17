# TISR Deliverable 00 — Unified CLI Spec (Phase 6 MVP Source-of-Truth)

**本文档为 forward-bound research 提案；任何派生 TB 须独立 §8 ratification；本文档不构成已批准方案。**

**Phase**: TISR Final Deliverable 00 | **Class**: Phase 6 MVP = Class 1-2 | **依赖**: Phase 1 CLI-A/B/C/D + Phase 2 Track A + Phase 4 architecture_integration

---

## 1. 项目身份

| 字段 | 取值 |
|---|---|
| 主 CLI 名 | `turingos` (新, `src/bin/turingos.rs`) |
| 演化基础 | `lean_market` (TB-10 ship, 848 行, 7 subcommand) |
| Phase 6 工时 | ~6-8 周 (Class 1-2) |
| LOC 估算 | ~5900 (新增 ~3000 + lib 化重构 ~3000) |
| Class 4 修改 | **0** (typed_tx + sequencer 完全 0-touch) |
| Pre-condition | G-Phase 收口完成 + REAL-13 / REAL-BCAST-1 / REAL-13A ship |

---

## 2. 双轨架构

```
turingos (新主 CLI; clap-based)
  ├─ Phase 6 MVP: 37 subcommand (12 wrap 现有 bin + 25 新增)
  ├─ 共享 lib layer (runtime/*_runner.rs)
  └─ 走 sequencer 现有 emit_system_tx + submit_agent_tx 路径

并存 (不替换):
  ├─ turingos_dev (dev evidence sidecar; 保留)
  ├─ lean_market (TB-10 evidence 向后兼容; 保留)
  └─ 11 个独立 bin (audit_dashboard / verify_chaintape / 等; 保留 + lib 化)
```

---

## 3. 37 Subcommand 完整树 (详见 Phase 1 CLI-D)

```
turingos
├── init                              [P0, Class 1]    工程目录 + 配置向导
├── config set/get/list               [P0, Class 1]    配置管理
├── spec new --type <type>            [P1, Class 1]    spec 模板生成
├── batch new/start/list/view/add     [P0, Class 1-2]  batch 管理 (start 是 Class 2)
├── agent deploy/list/view            [P0, Class 1-2]  agent 部署 (deploy 是 Class 2)
├── role assign                       [P0, Class 2]    role 分配 (genesis-phase only)
├── task open/view/tick               [P0, Class 1-2]  通用 task (open 是 Class 2)
├── watch chain/agent/market/broadcast [P1, Class 1]   实时 tail
├── market trigger seed/cpmm-pool/event-resolve/finalize [P0, Class 2] 走 emit_system_tx
├── audit dashboard/tape/tamper       [P0, Class 1]    wrap audit_dashboard/tape/tamper
├── verify chaintape/e2-candidate     [P0, Class 1]    wrap verify_chaintape/real14_*
├── report run/markov/wallet/...      [P0-P1, Class 1] wrap gen_run_summary/lean_market view-*
├── preflight                         [P0, Class 1]    wrap resume_preflight
├── resume                            [P2, Class 2]    从 chaintape 恢复 (风险, 可能 Class 4)
├── replay                            [P1, Class 1]    wrap lean_market view-replay
└── export evidence                   [P1, Class 1]    一键打包 evidence bundle
```

**优先级**: P0 (13 个 happy path 必须) + P1 (11 个扩展) + P2 (1 个风险/可选)

**详细规范**: 见 Phase 1 CLI-A/B/C/D (`handover/research/interaction_substrate/10_cli_design/`)

---

## 4. Phase 6 实施 3 阶段

### 4.1 Phase 6.1 Bootstrap (P0 only, ~6-8 周)

- 13 P0 subcommand + 12 wrap = **25 个 MVP subcommand**
- LOC: ~2330 (P0 新增) + ~600 (clap + 共用层) + ~3000 (lib 化重构 audit_dashboard 等) = **~5930 LOC**
- Class: 1-2
- 验收: Happy path 12 步 (CLI-A §2.2) 完整跑通 + real-problem witness

### 4.2 Phase 6.2 Watch + Report (P1, ~4-6 周)

- 11 P1 subcommand (watch / report / spec / export)
- LOC: ~1280, Class 1
- 验收: 长 batch 稳定 tail + report cross-check 与 audit_dashboard 一致

### 4.3 Phase 6.3 Resume + Stretch (P2, optional ~2-4 周)

- `turingos resume` (Class 2, 风险: 可能触发 Class 4 NonEmptyRuntimeRepo gate 修改)
- LOC: ~250
- **风险**: 若发现需 sequencer 修改, 立即 archive 该功能 → 走 Class 4 separate charter

---

## 5. lean_market → turingos 演化路径

| lean_market subcommand | turingos subcommand | 演化方式 |
|---|---|---|
| `run-task --problem X --bounty Y` | `turingos task open --type lean --problem X --bounty Y` | scope 扩展 (支持 polymarket / custom) |
| `view-task <id>` | `turingos task view <id>` | 直接 wrap |
| `view-wallet` | `turingos report wallet --agent <id>` | 重组 |
| `view-replay` | `turingos replay --runtime-repo <path>` | 直接 wrap |
| `tick` | `turingos task tick` | 直接 wrap |
| `view-bankruptcy` | `turingos report bankruptcy --repo <path>` | 直接 wrap |
| `view-positions` | `turingos report positions --agent <id>` | 直接 wrap |

**保留独立 lean_market binary** (TB-10 evidence chain 向后兼容). 内部调用同一 `runtime/user_task_runner` lib.

---

## 6. 共享 Lib Layer 重构清单 (Phase 6 关键工作)

| 现有 bin | Lib 化目标 | LOC | Phase 6 风险 |
|---|---|---:|---|
| `audit_dashboard.rs` | `runtime/audit_dashboard_report.rs` | ~3000 | 低 (移动重构) |
| `audit_tape.rs` | `runtime/audit_tape_runner.rs` | ~250 | 低 |
| `audit_tape_tamper.rs` | `runtime/audit_tape_tamper_runner.rs` | ~350 | 低 |
| `generate_markov_capsule.rs` | `runtime/markov_capsule_runner.rs` | ~350 | 低 |
| `lean_market.rs` run-task | `runtime/user_task_runner.rs` | ~400 | 中 (evaluator child fork) |
| `lean_market.rs` view-* | `runtime/user_view.rs` | ~200 | 低 |

已 lib 化 (turingos 直接调): `verify::verify_chaintape` / `RunSummary::from_chaintape` / `verify_market_e2_candidate` / `resume_preflight::check` / `chain_derived_run_facts::*` / `persistence_evidence::*`

---

## 7. 全局 Flag (clap parser)

```rust
#[derive(clap::Parser)]
#[command(name = "turingos", version)]
struct Cli {
    #[arg(long, global = true)] repo: Option<PathBuf>,
    #[arg(long, global = true)] cas: Option<PathBuf>,
    #[arg(long, value_enum, global = true, default_value = "text")] format: OutputFormat,
    #[arg(long, global = true)] out: Option<PathBuf>,
    #[arg(long, global = true)] strict: bool,
    #[arg(short, long, global = true)] verbose: bool,
    #[command(subcommand)] command: Commands,
}

enum OutputFormat { Text, Json, Html }  // Html = Phase 5/7 候选, 初期 stub
```

**统一 Exit Code**: 0 success / 1 verification fail / 2 arg error / 3 sequencer error / 4 config error

---

## 8. Test 架构

- 单元测试: 每 `runtime/*_runner.rs` 必带 happy / failure path tests
- 集成测试:
  - `tests/cli_init_smoke.rs`: turingos init 验证
  - `tests/cli_batch_lifecycle.rs`: new → start → add task → view 端到端
  - `tests/cli_audit_e2e.rs`: 完整 batch → dashboard → verify
  - `tests/cli_lean_market_compat.rs`: 老 lean_market 命令仍能跑

---

## 9. 真问题 Witness (Phase 6 验收)

按 `feedback_real_problems_not_designed` 强 enforcement:

1. ✅ 至少 1 个完整 happy path (12 步) 跑通
2. ✅ 输出 evidence bundle (tar.gz)
3. ✅ Bundle 通过 audit_tape verdict PROCEED
4. ✅ Bundle 通过 replay 完全重建 HEAD_t
5. ✅ ProvenanceCapsule 区分 human-triggered vs agent-submitted (≥ 1 case each)

**Witness 存储**: `handover/evidence/stage_phase6_cli_*`

---

## 10. Kill Conditions (Phase 6 实施期)

按 Charter §5 + CLI-C §5:

| # | Trigger | 动作 |
|---|---|---|
| KC4 | 实际覆盖发现 ≥ 90% (薄包装) | 缩减 scope, 只做 P0 wrap; P1/P2 archive |
| KC5 | 任 subcommand 需要 typed_tx schema 修改 | 标 Class 4 forward-bound, 该 sub archive |
| KC7 | 任 deliverable 暗藏 Class 4 surface | Charter 补丁 + 警告头部 |
| KC8 | G-Phase 收口未完成或新 VETO directive | pause Phase 6 |

---

## 11. 与 Phase 7+ Web 关系

CLI 是**专家回退入口**, 不是 Web UI 的替代品:
- Phase 6 CLI + Phase 7 Web **共享 Event Bridge + Policy Engine + lib layer**
- CLI 支持 scripting + automation (Web 不支持)
- Web 是一等公民 (用户日常入口); CLI 是开发者 + 高级用户入口

---

## 12. 完整规范引用

本文档为概要; 完整代码级规范见:
- Phase 1 CLI-A (`10_cli_design/track_cli_a_user_journey.md`) — 11 用户场景 + happy path
- Phase 1 CLI-B (`track_cli_b_existing_bin_integration.md`) — 12 bin 集成 + lib 化清单
- Phase 1 CLI-C (`track_cli_c_missing_capabilities.md`) — 25 新 subcommand 详细
- Phase 1 CLI-D (`track_cli_d_architecture.md`) — clap 架构 + test + 文档

---

## 13. Phase 6 启动条件 (重申)

✋ **必须满足才能启动**:
1. G-Phase 收口完成 (SG-G overall §8 packet ship)
2. REAL-13 (market pressure loop) ship
3. REAL-BCAST-1 (Librarian broadcast) ship
4. REAL-13A (EV scaffolding) ship
5. `git log --since="2026-05-17" --pretty=format:"%s" src/state/typed_tx.rs | head -5` 显示**无** schema changes
6. 架构师 §8 ratification for TISR Phase 6 charter (separate from TISR research charter)
7. WebSearch 预算授权 (若 Phase 6 需要 frontier research)

**当前 (2026-05-17) 满足度**: 0/7 → Phase 6 不可启动. TISR 研究继续, Phase 6 实施等条件成熟.
