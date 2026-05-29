# Phase 1 / Track CLI-B — 现有 bin 集成清单

**本文档为 forward-bound research 提案；任何派生 TB 须独立 §8 ratification；本文档不构成已批准方案。**

> **SUPERSEDED 2026-05-29 (forward-only note; body below unchanged):** the
> "保留独立 (keep `turingos_dev`)" decision in the bin table and §2.10 below is
> superseded by user §8 directive 2026-05-29 — the `turingos_dev` dev-evidence
> sidecar is retired (commit `4264b14f`;
> `handover/alignment/OBS_R022_TURINGOS_DEV_REMOVAL_2026-05-29.md`). All
> `turingos_dev` / `dev_harness::*` references below are now historical.

**目的**: 盘点全部 12 个现有 bin (11 个 `src/bin/` + 1 个 `experiments/`)，给出每个 bin 的 entry/exit point + library function 调用关系 + 统一 `turingos` CLI 的集成路径。

---

## 1. 12 个现有 bin 总览

| Bin | 路径 | LOC | Library function 入口 | 输出 | TISR 提议 subcommand |
|---|---|---:|---|---|---|
| `audit_dashboard` | `src/bin/` | 3544 | (内部 build_report) | text + json | `turingos audit dashboard` |
| `audit_tape` | `src/bin/` | 283 | (待 lib 化) | json | `turingos audit tape` |
| `audit_tape_tamper` | `src/bin/` | 407 | (待 lib 化) | json | `turingos audit tamper` |
| `gen_run_summary` | `src/bin/` | 128 | `RunSummary::from_chaintape` | json | `turingos report run` |
| `generate_markov_capsule` | `src/bin/` | 416 | (内部) | json | `turingos report markov` |
| `real14_e2_candidate_verifier` | `src/bin/` | 103 | `verify_market_e2_candidate` | json | `turingos verify e2-candidate` |
| `resume_preflight` | `src/bin/` | 77 | `resume_preflight::check` | json | `turingos preflight` |
| `tb_18r_compute_invariant` | `src/bin/` | 117 | `chain_derived_run_facts::*` | json | `turingos report attempt-invariant` |
| `tb_g_persistence_report` | `src/bin/` | 118 | `persistence_evidence::*` | json (写到 disk) | `turingos report g-persistence` |
| `turingos_dev` | `src/bin/` | 255 | `dev_harness::*` | text | (保留独立) |
| `verify_chaintape` | `src/bin/` | 110 | `verify::verify_chaintape` | json | `turingos verify chaintape` |
| `lean_market` | `experiments/minif2f_v4/src/bin/` | 848 | (内部, 调 evaluator child) | text | `turingos task/report/...` 重组 |

**总 LOC**: 6,406 行已存在 CLI 代码

---

## 2. 每个 bin 详细 — entry + library + 集成路径

### 2.1 `audit_dashboard` — 16-section 综合 dashboard

**Entry** (`src/bin/audit_dashboard.rs:378-430`):
```rust
fn main() {
    // 1. parse_args (manual)
    // 2. build_report(repo, cas, markov_capsule_cid) -> DashboardReport
    // 3. render_text(report) OR serde_json::to_string_pretty(report)
    // 4. write to --out path OR stdout
}
```

**关键发现**:
- 已支持 `--json` 输出选项 ✅
- 已支持 `--out <path>` 写文件 ✅
- 16 个 render_section 函数 (render_section_13 / 14 / 15 / 16 + render_tb_n3_run_report)
- `build_report()` 是内部函数，**需要 lib 化** 用于 turingos CLI

**集成方式**:
- **Lib 化**: 把 `build_report()` 从 `main.rs` 提取到 `src/runtime/audit_dashboard_report.rs`
- **新 subcommand**: `turingos audit dashboard --repo <path> --cas <path> [--format text|json|html] [--out <path>] [--run-report]`
- **保留独立 bin**: `audit_dashboard` 不删除（向后兼容旧脚本）

**Class**: Phase 6 实施时 Class 1（lib 化重构 + 新 bin 包装）

### 2.2 `audit_tape` — Pure tape audit emitter

**Entry** (`src/bin/audit_tape.rs`):
```rust
// 输入: runtime_repo + cas + agent_pubkeys + pinned_pubkeys + genesis + constitution
// 可选: --markov-pointer / --prior-chain-runtime-repo
// 输出: verdict.json (38 assertions × 8 layers, tape_root, verdict ∈ {PROCEED, BLOCK})
```

**关键发现**:
- **TB-16 architect mandate**: "audit-from-tape only" — 不读 live Sequencer / state.db / process logs / handover/ai-direct/ / global Markov pointer
- 38 assertions × 8 layers (A-H)
- 输出 `verdict.json` 是 audit_tape 的核心 deliverable

**集成方式**:
- **Lib 化**: 提取核心函数到 `src/runtime/audit_tape_runner.rs`
- **新 subcommand**: `turingos audit tape --repo <path> --cas <path> --pubkeys <path> --pinned <path> --genesis <path> --constitution <path> [--markov-pointer <path>]`
- 保留独立 bin (TB-16 重要审计基础设施)

**Class**: Phase 6 实施时 Class 1（lib 化）；保留独立 bin Class 0

### 2.3 `audit_tape_tamper` — Tamper-detection harness

**Entry** (`src/bin/audit_tape_tamper.rs`):
```rust
// 1. Fork input tape into 3 temp copies
// 2. Introduce 1 corruption per copy:
//    - flip 1 byte in random L4 row
//    - flip 1 byte in random CAS object
//    - remove 1 L4 row by truncating
// 3. Re-run audit_tape on each
// 4. Each must emit verdict.json = BLOCK
// 5. Output tamper_report.json
```

**集成方式**:
- 直接 wrap 为 `turingos audit tamper --repo <path> --cas <path> --tamper-dir <work-dir> --out <report.json>`
- 共享 `audit_tape_runner` lib

**Class**: Phase 6 实施时 Class 1

### 2.4 `gen_run_summary` — Run summary emitter

**Entry** (`src/bin/gen_run_summary.rs:38-50`):
```rust
let summary = RunSummary::from_chaintape(&repo, &cas, &run_id, failed_branch_count, rollback_count)?;
let json = serde_json::to_string_pretty(&summary)?;
// write to --out or stdout
```

**关键发现**:
- ✅ 已经是 thin wrapper around `RunSummary::from_chaintape`
- ✅ JSON-only output (简单)
- ✅ Lib function 已存在，可直接复用

**集成方式**:
- 直接 wrap 为 `turingos report run --repo <path> --cas <path> --run-id <id> [--out <path>]`
- 无需 lib 化重构

**Class**: Phase 6 实施时 Class 1（薄包装）

### 2.5 `generate_markov_capsule` — Markov evidence capsule emitter

**Entry** (`src/bin/generate_markov_capsule.rs`):
```rust
// 1. Read constitution.md -> SHA-256
// 2. Open chain runtime_repo + CAS -> derive L4 / L4.E / CAS roots
// 3. Scan handover/alignment/OBS_*.md -> unresolved-OBS list
// 4. Cluster CAS AgentAutopsyCapsules -> TypicalErrorSummary list
// 5. Write MarkovEvidenceCapsule to CAS + JSON pointer file
```

**关键发现**:
- 复杂 pipeline (5 步)
- 默认 deny 深度历史读 (需 `TURINGOS_MARKOV_OVERRIDE=1`)
- `--no-cas` mode (pointer-only)

**集成方式**:
- **Lib 化**: 提取核心函数到 `src/runtime/markov_capsule_runner.rs`
- 新 subcommand: `turingos report markov --tb-id <N> --constitution <path> --runtime-repo <path> --cas <path> [--prev-cid-hex <hex>] [--no-cas]`

**Class**: Phase 6 实施时 Class 1

### 2.6 `real14_e2_candidate_verifier` — REAL-14 E2 candidate verifier

**Entry** (`src/bin/real14_e2_candidate_verifier.rs`):
```rust
let verdict = verify_market_e2_candidate(&repo, &cas, opts)?;
// Output: json or md
```

**关键发现**:
- Thin wrapper around `verify_market_e2_candidate` (lib function 已存在)
- 简单 args: --repo / --cas / --expect-count / --json-out / --md-out

**集成方式**:
- 直接 wrap 为 `turingos verify e2-candidate --repo <path> --cas <path> [--expect-count <n>] [--out <path>]`

**Class**: Phase 6 实施时 Class 1（薄包装）

### 2.7 `resume_preflight` — Resume contract preflight

**Entry** (`src/bin/resume_preflight.rs`):
```rust
let contract: ResumeContract = serde_json::from_slice(&bytes)?;
let verdict = resume_preflight::check(&contract);
println!("{}", serde_json::to_string(&verdict)?);
```

**关键发现**:
- Thin wrapper around `resume_preflight::check`
- Input: contract JSON file
- Output: `{"verdict": "Ok"}` or `{"verdict": "Fail", "failure": {...}}`
- TB-G G1.2-1 (resume contract 是 G-Phase 关键基础设施)

**集成方式**:
- 直接 wrap 为 `turingos preflight --contract <path>`

**Class**: Phase 6 实施时 Class 1（薄包装）

### 2.8 `tb_18r_compute_invariant` — Attempt count invariant computer

**Entry** (`src/bin/tb_18r_compute_invariant.rs`):
```rust
let (run_facts, verdict) = compute_run_facts_from_chain_with_invariant(
    &repo, &cas, &AttemptCountInvariantInputs { ... }
)?;
// JSON output with 6 FR-18R.4 fields + invariant verdict
```

**关键发现**:
- Thin wrapper around `chain_derived_run_facts::compute_run_facts_from_chain_with_invariant`
- TB-18R R6 helper (FC1-N43 evidence)
- 输入: --runtime-repo / --cas / --expected-completed / --halt-class

**集成方式**:
- 直接 wrap 为 `turingos report attempt-invariant --runtime-repo <path> --cas <path> --expected <n> --halt-class <class>`

**Class**: Phase 6 实施时 Class 1（薄包装）

### 2.9 `tb_g_persistence_report` — G-Phase persistence evidence enricher

**Entry** (`src/bin/tb_g_persistence_report.rs`):
```rust
// 1. Read BatchContinuationManifest.json from <RUN_DIR>
// 2. For each task boundary, replay_full_transition
// 3. bind_persistence -> classify 6 fields (balances/positions/reputation/PnL/autopsy/model)
// 4. Write PERSISTENCE_BINDING_REPORT.json
```

**关键发现**:
- TB-G G1.2-6/7 (Option B+ orchestration)
- Codex G1.2-6 micro-audit Q6 closure
- Exit code: 0 (no Reset) / 1 (at least one Reset) / 2 (IO error)

**集成方式**:
- 直接 wrap 为 `turingos report g-persistence --run-dir <path>`

**Class**: Phase 6 实施时 Class 1（薄包装）

### 2.10 `turingos_dev` — Dev evidence sidecar (保留独立)

**Entry** (`src/bin/turingos_dev.rs:33-49`):
```rust
match subcommand {
    "open" => cmd_open(rest),           // 创建 dev run
    "record-diff" => cmd_record_diff(rest),
    "record-command" => cmd_record_command(rest),
    "record-audit" => cmd_record_audit(rest),
    "validate" => cmd_validate(rest),
    "close" => cmd_close(rest),
    "summarize" => cmd_summarize(rest),
}
```

**关键发现**:
- **Dev sidecar 角色**：不是 user CLI；服务于 developer 评估流
- 7 个 subcommand 全部围绕 dev run lifecycle
- Manual subcommand split (无 clap)

**集成方式**:
- **保留独立** (不入 turingos 主 CLI)
- 长期可考虑 `turingos dev <sub>` 包装 (Phase 7+ 选项)
- Phase 1 CLI 不包含

**Class**: 不变更（保留现状）

### 2.11 `verify_chaintape` — Chain verifier

**Entry** (`src/bin/verify_chaintape.rs:21-50`):
```rust
let opts = VerifyOptions { expected_run_id: parsed.run_id.clone() };
let report = verify_chaintape(&parsed.repo, &parsed.cas, &opts)?;
// JSON output
```

**关键发现**:
- Thin wrapper around `runtime::verify::verify_chaintape`
- 输入: --repo / --cas / --run-id / --out

**集成方式**:
- 直接 wrap 为 `turingos verify chaintape --repo <path> --cas <path> [--run-id <id>] [--out <path>]`

**Class**: Phase 6 实施时 Class 1（薄包装）

### 2.12 `lean_market` — User-facing Lean proof market CLI

**Entry** (`experiments/minif2f_v4/src/bin/lean_market.rs:42-65`):
```rust
match subcommand {
    "run-task" => cmd_run_task(&sub_args),       // fresh chaintape + post task + run evaluator
    "view-task" => cmd_view_task(&sub_args),
    "view-wallet" => cmd_view_wallet(&sub_args),
    "view-replay" => cmd_view_replay(&sub_args),
    "tick" => cmd_tick(&sub_args),
    "view-bankruptcy" => cmd_view_bankruptcy(&sub_args),
    "view-positions" => cmd_view_positions(&sub_args),
}
```

**关键发现**:
- 848 行已 ship 的 user CLI (TB-10)
- 7 个 subcommand 全部支持基础 user journey
- 位于 `experiments/`（不在主 `src/bin/`）
- **架构限制**: Lean-only / single-process / fresh chaintape only / no JSON

**集成方式** (TISR 核心提议):
- **重新组织**: `lean_market` 拆分为 turingos 多个子命令:
  - `lean_market run-task` → `turingos task open --type lean` (功能更通用)
  - `lean_market view-task` → `turingos task view <id>`
  - `lean_market view-wallet` → `turingos report wallet --agent <id>`
  - `lean_market view-replay` → `turingos replay --runtime-repo <path>`
  - `lean_market tick` → `turingos task tick`
  - `lean_market view-bankruptcy` → `turingos report bankruptcy --repo <path>`
  - `lean_market view-positions` → `turingos report positions --agent <id>`
- **位置升级**: 从 `experiments/minif2f_v4/src/bin/` 提升到 `src/bin/turingos.rs`
- **保留 lean_market**: 旧 bin 不删除（TB-10 evidence 链向后兼容）
- **去除 child process fork**: 不是 essential；统一 turingos 直接 in-process bootstrap (Phase 6 实施时考虑)

**Class**: Phase 6 实施时 Class 1-2

---

## 3. 集成架构图

```
┌─────────────────────────────────────────────────────────────┐
│ turingos (新统一 CLI; src/bin/turingos.rs; clap-based)     │
│   ├── init / config / spec                                  │
│   ├── batch new/start/list/view/add/persistence              │
│   ├── agent deploy/list/view                                │
│   ├── role assign                                           │
│   ├── task open/view/tick                                   │
│   ├── watch chain/agent/market/broadcast                    │
│   ├── market trigger seed/cpmm-pool/event-resolve/finalize  │
│   ├── audit dashboard/tape/tamper                           │  ← wrap audit_dashboard/audit_tape/audit_tape_tamper
│   ├── verify chaintape/e2-candidate                         │  ← wrap verify_chaintape/real14_e2_candidate_verifier
│   ├── report run/markov/wallet/positions/bankruptcy/        │  ← wrap gen_run_summary/generate_markov_capsule/lean_market
│   │       g-persistence/attempt-invariant/judgments/pnl/    │  ← wrap tb_g_persistence_report/tb_18r_compute_invariant
│   │       roles                                             │
│   ├── preflight                                             │  ← wrap resume_preflight
│   ├── resume                                                │  ← 新功能, Class 2
│   ├── replay                                                │  ← wrap lean_market view-replay
│   └── export evidence                                       │  ← 新功能, Class 1
│
├─── Library layer (待 lib 化 + 已 lib 化):                    │
│    ├── runtime::audit_dashboard_report::build_report  [新]  │
│    ├── runtime::audit_tape_runner::run_audit          [新]  │
│    ├── runtime::run_summary::RunSummary::from_chaintape    │
│    ├── runtime::markov_capsule_runner::run            [新]  │
│    ├── runtime::market_e2_candidate_verifier::verify       │
│    ├── runtime::resume_preflight::check                    │
│    ├── runtime::chain_derived_run_facts::compute_run_facts │
│    ├── runtime::persistence_evidence::*                    │
│    ├── runtime::verify::verify_chaintape                   │
│    └── (lean_market split 后) runtime::user_task_runner::* │
│
└─── 保留独立 bin (向后兼容):                                  │
     audit_dashboard / audit_tape / audit_tape_tamper /       │
     gen_run_summary / generate_markov_capsule /              │
     real14_e2_candidate_verifier / resume_preflight /        │
     tb_18r_compute_invariant / tb_g_persistence_report /     │
     verify_chaintape / lean_market                           │
     (turingos_dev 仍是独立 dev sidecar)                       │
└─────────────────────────────────────────────────────────────┘
```

---

## 4. Lib 化重构清单

为支持 `turingos` 主 CLI 调用现有功能，需要 **lib 化** 以下函数（从 bin entry 提取到 lib module）:

| 现有 bin | 当前位置 | Lib 化目标 | LOC 估算 |
|---|---|---|---:|
| `audit_dashboard` | `src/bin/audit_dashboard.rs:build_report` (内部) | `src/runtime/audit_dashboard_report.rs::build_report` | ~3000 (大部分是 render_section) |
| `audit_tape` | `src/bin/audit_tape.rs` (内部) | `src/runtime/audit_tape_runner.rs::run_audit` | ~250 |
| `audit_tape_tamper` | `src/bin/audit_tape_tamper.rs` (内部) | `src/runtime/audit_tape_tamper_runner.rs::run_tamper_harness` | ~350 |
| `generate_markov_capsule` | `src/bin/generate_markov_capsule.rs` (内部) | `src/runtime/markov_capsule_runner.rs::generate` | ~350 |
| `lean_market run-task` 核心逻辑 | `experiments/.../lean_market.rs:cmd_run_task` | `src/runtime/user_task_runner.rs::open_task` | ~150 |
| `lean_market view-*` 系列 | `experiments/.../lean_market.rs:cmd_view_*` | `src/runtime/user_view::*` (多个函数) | ~200 |

**已 lib 化的** (turingos CLI 可直接调用，无需重构):
- `verify::verify_chaintape` ✅
- `RunSummary::from_chaintape` ✅
- `verify_market_e2_candidate` ✅
- `resume_preflight::check` ✅
- `chain_derived_run_facts::compute_run_facts_from_chain_with_invariant` ✅
- `persistence_evidence::bind_persistence` ✅

**总重构 LOC**: ~4300（主要是 `audit_dashboard.rs` 的 build_report + render_sections 提取）；Phase 6 实施时 Class 1，因为只是 move + signature 不变。

---

## 5. 共同模式 — 6 个观察

### 5.1 Manual subcommand parsing 是 universal pattern

12 个 bin 中**全部**使用 `std::env::args().skip(1)` + 手动 match。**无任何 bin 使用 clap**。

**含义**: Phase 1 CLI 引入 clap 是 **重大架构升级**，但所有现有 bin 行为可以通过 clap subcommand argument 转发保留。

### 5.2 JSON output 已是 universal pattern

8/12 bin 默认输出 JSON 或支持 `--json` 选项。

**含义**: `turingos` CLI 的 `--format text|json|html` 中 **JSON 已默认覆盖**。Text 模式只需 audit_dashboard / lean_market / turingos_dev 三处转换。HTML 是 Phase 5 候选。

### 5.3 `--repo <path> --cas <path>` 是 universal flag

10/12 bin 接受 `--runtime-repo`（或 `--repo`）+ `--cas`（或 `--cas-dir`）。

**含义**: `turingos` 可设置全局 `--repo` / `--cas` flag (或 toml config)，所有 subcommand 默认继承。

### 5.4 Exit code 风格不统一

- `verify_chaintape`: 0=pass / 1=fail / 2=arg
- `audit_tape_tamper`: 0=all detected / 1=undetected / 2=arg
- `tb_g_persistence_report`: 0=no Reset / 1=Reset / 2=arg
- `lean_market`: ??? (待查)

**含义**: `turingos` 应统一 exit code 风格 (0=success / 1=verification fail / 2=arg error)；通过 clap + `--strict` flag 控制。

### 5.5 输出文件命名约定

- verdict.json
- replay_report.json
- tamper_report.json
- run_summary.json
- PERSISTENCE_BINDING_REPORT.json
- DashboardReport (audit_dashboard)

**含义**: Phase 6 实施时建议统一 evidence dir 内的文件命名 (camelCase vs snake_case vs SCREAMING)；不修改向后兼容旧 bin。

### 5.6 `lean_market` 是唯一**真正**的 user-facing CLI

其他 11 个 bin 都是 audit/verify/report 工具（read-only 或 evidence emitter）；**只有 `lean_market`** 包含真正的 user state-changing 操作（post TaskOpen / EscrowLock / launch evaluator）。

**含义**: Phase 1 CLI 真正的 user product 中心是 lean_market 的演化；其他都是包装支持。

---

## 6. Phase 1 CLI-B 完成 — 输出 + 下一步

### 6.1 本 Track 完成

- ✅ 12 个 bin 全盘点 (entry + library + 集成方式)
- ✅ Lib 化重构清单 (~4300 LOC, 主要是 audit_dashboard)
- ✅ 6 个共同模式观察
- ✅ 统一 `turingos` 主 CLI 集成架构图
- ✅ `lean_market` 拆分映射 → 7 个 lean_market subcommand → 7+ turingos subcommand

### 6.2 Track CLI-C 输入

Track CLI-C (缺失能力清单) 应聚焦本 Track 未覆盖的工作流:
- **场景 1**: init / config / spec (3 个新 subcommand)
- **场景 2**: batch (4 个新 subcommand)
- **场景 3**: agent / role (4 个新 subcommand)
- **场景 4**: task open generic (1 个新 subcommand, lean_market run-task 演化)
- **场景 5**: watch (4 个新 subcommand)
- **场景 6**: market trigger (4 个新 subcommand) — Class 1-2 边界关键
- **场景 8**: report judgments/pnl/roles (3 个新 subcommand)
- **场景 10**: resume (1 个新, Class 2)
- **场景 11**: export evidence (1 个新)

**总计**: ~25 个新 subcommand vs ~12 个 wrap subcommand = 25/37 ≈ 67% 新增。

### 6.3 Track CLI-D 输入

CLI-D 应规划:
- clap-based parent command 结构
- Lib 化路径 (重构 from `audit_dashboard.rs:build_report` 等)
- 共用 `--repo` / `--cas` / `--format` flag 系统
- 与 `turingos_dev` 的并存策略
- 与 `lean_market` 的并存 + 演化策略 (旧 bin 保留向后兼容)

---

## 7. 验收 — CLI-B 出口自检

- ✅ 12 个 bin 全盘点 (LOC + entry + library)
- ✅ 每个 bin 的集成方式明确 (subcommand 名 + 是否需要 lib 化)
- ✅ 重构 LOC 量化 (~4300, 主要 audit_dashboard)
- ✅ 6 个共同模式总结
- ✅ Kill Condition 4 检查: 12/30+ ≈ 40% 覆盖, 不触发缩为薄包装
- ✅ forward-bound 头部声明
- ✅ Track CLI-C/D 输入 hand-off 清晰
