# Phase 1 / Track CLI-D — CLI Architecture Spec

**本文档为 forward-bound research 提案；任何派生 TB 须独立 §8 ratification；本文档不构成已批准方案。**

**目的**: Phase 1 收尾，给出统一 `turingos` CLI 的架构规范。整合 CLI-A 用户场景 + CLI-B 现有 bin + CLI-C 缺失能力。Phase 5 `00_UNIFIED_CLI_SPEC.md` 终极交付的核心架构层。

---

## 1. 总体架构

### 1.1 三层结构

```
┌──────────────────────────────────────────────────┐
│  Layer 1: turingos CLI (src/bin/turingos.rs)     │
│    - clap-based parent command                    │
│    - Subcommand routing                           │
│    - Global flags (--repo / --cas / --format)     │
│    - Error abstraction + exit codes               │
│    - Config loading (~/.config/turingos/ + ./)    │
└────────────────────┬─────────────────────────────┘
                     │
┌────────────────────▼─────────────────────────────┐
│  Layer 2: Library functions (src/runtime/)       │
│    - audit_dashboard_report::build_report  [新]  │
│    - audit_tape_runner::run_audit          [新]  │
│    - markov_capsule_runner::generate       [新]  │
│    - user_task_runner::open_task           [新]  │
│    - resume_runner::resume                 [新]  │
│    - export_runner::bundle                 [新]  │
│    - (复用) verify::verify_chaintape              │
│    - (复用) RunSummary::from_chaintape            │
│    - (复用) verify_market_e2_candidate            │
│    - (复用) resume_preflight::check               │
│    - (复用) chain_derived_run_facts::*            │
│    - (复用) persistence_evidence::*               │
└────────────────────┬─────────────────────────────┘
                     │
┌────────────────────▼─────────────────────────────┐
│  Layer 3: Core kernel + sequencer (UNCHANGED)    │
│    - src/state/sequencer.rs (STEP_B frozen)      │
│    - src/state/typed_tx.rs (STEP_B frozen)       │
│    - src/bottom_white/cas/* (STEP_B frozen)      │
│    - src/runtime/real5_roles.rs (现有)            │
│    - src/runtime/economic_judgment.rs (现有)      │
│    - src/runtime/librarian_broadcast.rs (现有)    │
└──────────────────────────────────────────────────┘
```

**Class 严格隔离**:
- Layer 1: **Class 1** (clap wiring + I/O)
- Layer 2: **Class 1-2** (lib functions; 2 个 Class 2 涉及 sequencer 调用)
- Layer 3: **不变更** (任何修改触发 Class 4 STEP_B)

### 1.2 Crate 组织

```
turingosv4/
├── src/
│   ├── bin/
│   │   ├── turingos.rs              [新] 主 CLI ~700 LOC
│   │   ├── turingos_dev.rs          [现有] dev sidecar 保留
│   │   ├── audit_dashboard.rs       [现有] 保留, 调 lib
│   │   ├── audit_tape.rs            [现有] 保留, 调 lib
│   │   └── ... (其他 8 个 bin 保留)
│   ├── cli/                          [新模块]
│   │   ├── mod.rs                    # 公共类型
│   │   ├── args.rs                   # clap 结构体
│   │   ├── format.rs                 # text/json/html 输出层
│   │   ├── config.rs                 # config 加载
│   │   ├── error.rs                  # 错误抽象
│   │   └── commands/                 # 每个 subcommand 一个文件
│   │       ├── init.rs
│   │       ├── config.rs
│   │       ├── spec.rs
│   │       ├── batch.rs
│   │       ├── agent.rs
│   │       ├── role.rs
│   │       ├── task.rs
│   │       ├── watch.rs
│   │       ├── market.rs
│   │       ├── audit.rs
│   │       ├── verify.rs
│   │       ├── report.rs
│   │       ├── preflight.rs
│   │       ├── resume.rs
│   │       ├── replay.rs
│   │       └── export.rs
│   ├── runtime/
│   │   ├── audit_dashboard_report.rs [新 lib, ~3000 LOC 移自 audit_dashboard.rs]
│   │   ├── audit_tape_runner.rs      [新 lib, ~250 LOC]
│   │   ├── audit_tape_tamper_runner.rs [新 lib, ~350 LOC]
│   │   ├── markov_capsule_runner.rs  [新 lib, ~350 LOC]
│   │   ├── user_task_runner.rs       [新 lib, ~400 LOC 提自 lean_market]
│   │   ├── user_view.rs              [新 lib, ~200 LOC 提自 lean_market view-*]
│   │   ├── export_runner.rs          [新 lib, ~200 LOC]
│   │   └── ... (其他不变更)
│   └── ... (其他 src/ 树不变更)
└── experiments/minif2f_v4/src/bin/
    ├── lean_market.rs                [现有] 保留 (TB-10 evidence 向后兼容)
    ├── evaluator.rs                  [现有] 保留
    └── ... (不变更)
```

**新增 module 总量**:
- `src/cli/`: ~1000 LOC (16 个 command file + 共用 4 个 module)
- `src/runtime/*_runner.rs`: ~4750 LOC (5 个新 runner; 包括 audit_dashboard 重构)
- `src/bin/turingos.rs`: ~150 LOC (薄 entry point)
- **总计**: ~5900 LOC 新增/重构

---

## 2. Subcommand 树形结构 (clap)

### 2.1 命名规范

**模式**: `turingos <noun> <verb>` 优先 (sub-noun 时 `turingos <noun> <sub-noun> <verb>`)

| 类型 | 模式 | 例 |
|---|---|---|
| 直接动作 | `turingos <verb>` | `turingos init` / `turingos preflight` / `turingos resume` / `turingos replay` |
| 名词-动词 | `turingos <noun> <verb>` | `turingos batch new` / `turingos agent deploy` / `turingos task open` |
| 名词-子名词-动词 | `turingos <noun> <sub-noun> <verb>` | `turingos market trigger seed` |
| 视图 | `turingos <noun> view` 或 `turingos <noun> list` | `turingos batch view` / `turingos agent list` |
| watch (实时) | `turingos watch <noun>` | `turingos watch chain` / `turingos watch agent` |
| report | `turingos report <noun>` | `turingos report run` / `turingos report pnl` |
| audit | `turingos audit <noun>` | `turingos audit dashboard` / `turingos audit tape` |
| verify | `turingos verify <noun>` | `turingos verify chaintape` / `turingos verify e2-candidate` |
| 配置 | `turingos config <verb> <key>` | `turingos config set model.default deepseek-chat` |

### 2.2 完整 subcommand 树

```text
turingos
├── init                              [Class 1, P0]    创建工程目录
├── config
│   ├── set <key> <value>             [Class 1, P0]    配置写入
│   ├── get <key>                     [Class 1, P0]    配置读取
│   └── list                          [Class 1, P0]    列出配置
├── spec
│   └── new --type <type>             [Class 1, P1]    新建 spec 模板
├── batch
│   ├── new --name <name>             [Class 1, P0]    创建 batch manifest
│   ├── start --id <id>               [Class 2, P0]    启动 batch (system_emitted genesis)
│   ├── list                          [Class 1, P0]    列出 batch
│   ├── view --id <id>                [Class 1, P0]    看 batch 详细
│   └── add --batch <id> --task <id>  [Class 1, P0]    追加 task 到 batch
├── agent
│   ├── deploy --model <m> --role <r> [Class 2, P0]    部署 agent (genesis-phase)
│   ├── list                          [Class 1, P0]    列出 agent
│   └── view --id <id>                [Class 1, P0]    看 agent role-scoped view
├── role
│   └── assign --agent <id> --role <r> [Class 2, P0]   修改 agent role (genesis-phase)
├── task
│   ├── open --type <t> ...           [Class 2, P0]    通用 task open
│   ├── view --id <id>                [Class 1, P0]    看 task 状态 (wraps lean_market view-task)
│   └── tick                          [Class 2, P0]    推进时间 (wraps lean_market tick)
├── watch                             [Class 1, P1]
│   ├── chain --repo <p>              tail ChainTape
│   ├── agent --id <id>               tail agent decisions
│   ├── market                        tail market activity
│   └── broadcast                     tail Librarian broadcast
├── market
│   └── trigger
│       ├── seed --task <id>          [Class 2, P0]    MarketSeed (system_emitted)
│       ├── cpmm-pool --task <id>     [Class 2, P0]    CpmmPool (system_emitted)
│       ├── event-resolve --task <id> [Class 2, P0]    EventResolve (system_emitted oracle)
│       └── finalize --task <id>      [Class 2, P0]    FinalizeReward (system_emitted)
├── audit
│   ├── dashboard --repo <p>          [Class 1, P0]    wraps audit_dashboard build_report
│   ├── tape --repo <p>               [Class 1, P0]    wraps audit_tape
│   └── tamper --repo <p>             [Class 1, P0]    wraps audit_tape_tamper
├── verify
│   ├── chaintape --repo <p>          [Class 1, P0]    wraps verify_chaintape
│   └── e2-candidate --repo <p>       [Class 1, P0]    wraps real14_e2_candidate_verifier
├── report
│   ├── run --run-id <id>             [Class 1, P0]    wraps gen_run_summary
│   ├── markov --tb-id <n>            [Class 1, P1]    wraps generate_markov_capsule
│   ├── attempt-invariant ...         [Class 1, P1]    wraps tb_18r_compute_invariant
│   ├── g-persistence --run-dir <p>   [Class 1, P1]    wraps tb_g_persistence_report
│   ├── wallet --agent <id>           [Class 1, P0]    wraps lean_market view-wallet
│   ├── positions --agent <id>        [Class 1, P1]    wraps lean_market view-positions
│   ├── bankruptcy                    [Class 1, P1]    wraps lean_market view-bankruptcy
│   ├── judgments --agent <id>        [Class 1, P1]    EconomicJudgment 历史 (新)
│   ├── pnl --agent <id>              [Class 1, P1]    PnL 走势 (新)
│   └── roles --batch <id>            [Class 1, P1]    role 分化报告 (新)
├── preflight --contract <p>          [Class 1, P0]    wraps resume_preflight
├── resume --repo <p>                 [Class 2, P2]    从 chaintape 恢复 (风险, 可能 Class 4)
├── replay --bundle <p>               [Class 1, P1]    wraps lean_market view-replay 升级
└── export
    └── evidence --out <p>            [Class 1, P1]    一键打包 evidence bundle
```

**总 subcommand 数**: 37 个 (与 CLI-C §3 一致)

---

## 3. 共用层设计

### 3.1 Global Flag

**所有 subcommand 继承**:

```rust
#[derive(clap::Parser)]
#[command(name = "turingos", version, about)]
struct Cli {
    /// runtime_repo 路径 (默认从 config 或 ./runtime_repo)
    #[arg(long, global = true)]
    repo: Option<PathBuf>,

    /// CAS 路径 (默认从 config 或 ./cas)
    #[arg(long, global = true)]
    cas: Option<PathBuf>,

    /// 输出格式 (text/json/html)
    #[arg(long, value_enum, global = true, default_value = "text")]
    format: OutputFormat,

    /// 输出文件 (默认 stdout)
    #[arg(long, global = true)]
    out: Option<PathBuf>,

    /// 严格模式 (任何 warning 触发 exit 1)
    #[arg(long, global = true)]
    strict: bool,

    /// 详细日志
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::ValueEnum, Clone)]
enum OutputFormat {
    Text,
    Json,
    Html,
}
```

### 3.2 输出格式抽象 (`src/cli/format.rs`)

```rust
pub trait Renderable {
    fn render_text(&self, writer: &mut dyn Write) -> Result<()>;
    fn render_json(&self, writer: &mut dyn Write) -> Result<()> {
        // 默认实现: serde_json::to_writer_pretty
    }
    fn render_html(&self, writer: &mut dyn Write) -> Result<()> {
        // 默认实现: minimal HTML template (Phase 5 候选; Phase 6 MVP 可 stub 或 error)
    }
}

pub fn render_output<R: Renderable>(
    item: &R,
    format: OutputFormat,
    out: Option<&Path>,
) -> Result<()> {
    let mut writer: Box<dyn Write> = match out {
        Some(p) => Box::new(File::create(p)?),
        None => Box::new(io::stdout()),
    };
    match format {
        OutputFormat::Text => item.render_text(&mut writer),
        OutputFormat::Json => item.render_json(&mut writer),
        OutputFormat::Html => item.render_html(&mut writer),
    }
}
```

**实施约束**:
- Phase 6.1 MVP: 仅实现 text + json (html stub 返回 "HTML output is Phase 5 deliverable; please use --format text|json")
- Phase 6.2-3: 实现 html (Phase 5 spec 完成后)
- 所有 Layer 2 lib function 返回的 result struct 都要实现 `Renderable`

### 3.3 错误抽象 (`src/cli/error.rs`)

```rust
#[derive(Debug, thiserror::Error)]
pub enum TuringosCliError {
    #[error("config error: {0}")]
    Config(#[from] ConfigError),

    #[error("filesystem error: {0}")]
    Fs(#[from] io::Error),

    #[error("sequencer error: {0}")]
    Sequencer(#[from] SequencerError),

    #[error("audit error: {0}")]
    Audit(String),

    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("verification failed: {0}")]
    VerificationFailed(String),

    #[error("other: {0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

pub type CliResult<T> = Result<T, TuringosCliError>;
```

**Exit code 统一**:
- 0: success
- 1: verification fail (audit BLOCK / verify fail / preflight fail)
- 2: argument error / IO error
- 3: sequencer error (typed_tx 拒绝)
- 4: config error

### 3.4 Config 加载 (`src/cli/config.rs`)

**优先级** (高到低):
1. CLI flag (--repo, --cas, etc.)
2. 环境变量 (TURINGOS_REPO, TURINGOS_CAS, etc.)
3. 项目 config (./turingos.toml)
4. 全局 config (~/.config/turingos/config.toml)
5. 默认值

```toml
# 示例 ~/.config/turingos/config.toml
[default]
repo = "~/turingos_workspace/runtime_repo"
cas = "~/turingos_workspace/cas"
format = "text"

[model]
default = "deepseek-chat"
api_key_env = "DEEPSEEK_API_KEY"

[bounty]
default_micro = 100_000
```

### 3.5 Logging

- 使用 `tracing` + `tracing-subscriber` (与现有项目一致)
- `-v` flag 启用 DEBUG 日志
- `-vv` flag 启用 TRACE 日志
- 默认 INFO

---

## 4. Lib 化路径详细

### 4.1 audit_dashboard.rs → runtime/audit_dashboard_report.rs

**当前**: `src/bin/audit_dashboard.rs` 3544 行，内部函数 `build_report()` + 16 个 `render_section_*`.

**目标**:
- 提取 `build_report() -> DashboardReport` 到 `runtime/audit_dashboard_report.rs` (lib)
- 提取 `render_text() + render_section_*` 到同 module
- `bin/audit_dashboard.rs` 简化为 ~50 行薄 entry：调 lib + clap 解析

**重构 LOC**: ~3000 (主要是 move + signature 不变)
**Class**: 1 (代码移动, 行为不变)
**风险**: 低

### 4.2 audit_tape.rs → runtime/audit_tape_runner.rs

**当前**: `src/bin/audit_tape.rs` 283 行，内部完整 audit pipeline.

**目标**:
- 提取核心函数 `run_audit(opts: AuditTapeOpts) -> Result<TapeVerdict>` 到 lib
- `bin/audit_tape.rs` 简化为 ~50 行薄 entry

**重构 LOC**: ~250
**Class**: 1
**风险**: 低

### 4.3 audit_tape_tamper.rs → runtime/audit_tape_tamper_runner.rs

类似 4.2; ~350 LOC; Class 1.

### 4.4 generate_markov_capsule.rs → runtime/markov_capsule_runner.rs

类似 4.2; ~350 LOC; Class 1.

### 4.5 lean_market.rs (experiments) → runtime/user_task_runner.rs + user_view.rs

**关键复杂度**:
- lean_market 用 `Command::new("evaluator").spawn()` 模式 spawn child process
- TISR Phase 6 应**消除 child fork**: 直接 in-process 调用 evaluator function (如果 evaluator 已 lib 化)
- 如果 evaluator 未 lib 化, 保留 spawn 模式但移到 `user_task_runner` lib

**重构 LOC**: ~600 (含两个 module)
**Class**: 1-2 (`open_task` 涉及 system_emitted TaskOpen + EscrowLock; Class 2)
**风险**: 中 (evaluator child process 模式)

---

## 5. 与现存 bin 并存策略

### 5.1 三类策略

**A. 完全保留独立 (turingos_dev / experiments/*)**:
- `turingos_dev`: dev sidecar 角色不变
- `experiments/minif2f_v4/src/bin/*`: evaluator / batch_evaluator / comprehensive_arena 保留
- `experiments/.../lean_market.rs`: 保留 (TB-10 evidence 向后兼容)

**B. lib 化重构 + 保留独立 bin (10 个)**:
- audit_dashboard / audit_tape / audit_tape_tamper / gen_run_summary / generate_markov_capsule
- real14_e2_candidate_verifier / resume_preflight / tb_18r_compute_invariant / tb_g_persistence_report / verify_chaintape
- 现有 bin 保留 (旧脚本向后兼容), 但内部调 lib function (Layer 2)
- 新 turingos CLI 也调同 lib function

**C. 新建 + 完全替代旧 bin (0 个)**:
- 无替代; 所有现有 bin 保留

### 5.2 Cargo.toml 配置

```toml
[package]
name = "turingosv4"
# ...

[[bin]]
name = "turingos"
path = "src/bin/turingos.rs"

[[bin]]
name = "turingos_dev"
path = "src/bin/turingos_dev.rs"

[[bin]]
name = "audit_dashboard"
path = "src/bin/audit_dashboard.rs"

# ... 其他 9 个 bin 不变

[dependencies]
# 现有依赖
clap = { version = "4.5", features = ["derive", "env"] }  # 新增
tracing = "0.1"  # 已有
tracing-subscriber = "0.3"  # 已有
thiserror = "1"  # 已有
```

---

## 6. Test 架构

### 6.1 单元测试

每个 `src/cli/commands/<name>.rs` 必带 unit test:
- arg parsing 边界 (missing required / invalid format)
- output formatting (text/json roundtrip)
- error handling (各 exit code 触发条件)

每个 `src/runtime/*_runner.rs` 必带 unit test:
- happy path
- failure path (CAS missing / repo missing / etc.)

### 6.2 集成测试

`tests/cli_integration_*.rs`:
- `tests/cli_init_smoke.rs`: turingos init → 验证目录创建
- `tests/cli_batch_lifecycle.rs`: new → start → add task → view → end-to-end
- `tests/cli_audit_e2e.rs`: 跑完一个 batch → audit dashboard → verify chaintape
- `tests/cli_lean_market_compat.rs`: 老 lean_market 命令仍能跑

### 6.3 真问题 witness

Phase 6 MVP 验收要求：
- 至少 1 个完整 happy path (12 步, CLI-A §2.2) 跑通
- 输出 evidence bundle (tar.gz)
- Bundle 通过 audit_tape verdict PROCEED
- Bundle 通过 replay 完全重建 HEAD_t

---

## 7. 文档结构

```
docs/cli/
├── README.md                      入门指南
├── reference/
│   ├── init.md                    每个 subcommand 一文档
│   ├── batch.md
│   ├── agent.md
│   ├── ...
├── tutorials/
│   ├── 01_first_proof.md          Hello-world Lean proof
│   ├── 02_multi_agent.md          多 agent + role 分化
│   ├── 03_polymarket.md           Polymarket event
│   └── 04_replay.md               导出 + 复盘
└── architecture/
    ├── design.md                  本文档导出
    └── migration_from_lean_market.md
```

每个 subcommand 文档含 (Phase 6 实施时填):
- 用途
- 用法 (含示例)
- 参数详解
- 输出格式 (text + json)
- 退出码
- 关联 subcommand

---

## 8. 与 in-flight 工作的并存 timing

### 8.1 REAL-13 (market pressure loop)

REAL-13 (2026-05-16 directive) 是 market pressure loop 在 G-Phase 收口。如果 REAL-13 引入新 typed_tx variant, Phase 6 启动应延后到 REAL-13 ship 之后。

**检测命令** (Phase 6 启动前):
```bash
git log --since="2026-05-16" --pretty=format:"%s" src/state/typed_tx.rs | head -5
# 如果显示 schema changes, Phase 6 等
```

### 8.2 REAL-BCAST-1 (Librarian broadcast)

`turingos watch broadcast` 直接依赖 `runtime::librarian_broadcast`. 如果 REAL-BCAST-1 在 Phase 6 启动前 ship, 直接调用稳定 API; 否则 watch broadcast 应 P2 延后。

### 8.3 REAL-13A (EV scaffolding)

REAL-13A 2026-05-16 directive 涉及 Bull/Bear EV 推理深化. 与 `turingos report judgments / pnl / roles` 高度相关; Phase 6 P1 应等 REAL-13A ship.

### 8.4 G-Phase Closeout (SG-G overall §8 packet)

按 Charter §2 carve-out 承诺: **SG-G overall §8 packet 完成前**, TISR 不进入 Phase 6 实施.

**当前状态** (2026-05-16 LATEST.md): G5/G6/G7/SG-G aggregate 待完成.

**Phase 6 启动时机**: G-Phase 完成 + REAL-13/BCAST-1/13A 全部 ship 后 (估计 2026-06-15+).

---

## 9. Phase 6 实施风险评估

### 9.1 高风险点

1. **lean_market 提升路径** (LOC ~600 重构): evaluator child process 模式可能需要 evaluator lib 化; 如果 evaluator 还不能 lib 化, 保留 spawn 模式. 风险中.
2. **turingos resume** (Class 2, P2): NonEmptyRuntimeRepo gate 可能要求 sequencer 修改 → Class 4 forward-bound 风险.
3. **--format html stub** (Phase 5 候选): Phase 6 MVP 不实施; Phase 6.1 stub 返回 "Phase 5 deliverable" 错误.
4. **clap-based 迁移**: 现有 11 个 bin 全部 manual parsing; 引入 clap 是显著架构升级, 需 cargo build 检查.
5. **跨 in-flight 工作协调**: REAL-13/BCAST-1/13A timing 影响 Phase 6 启动.

### 9.2 中风险点

- `turingos init` 模板设计 (toml schema 稳定性)
- `turingos batch start` 与现有 evaluator init 流程的合并
- `turingos market trigger event-resolve` 的 oracle 输入策略 (Phase 7 轴 A 应统一)

### 9.3 低风险点

- 大部分 wrap subcommand (read-only lib function 调用)
- Class 1 audit/verify/report 子命令
- watch 子命令 (tail 模式)

---

## 10. 1-2 个 Open question

### 10.1 lean_market 是否 deprecate?

**选项**:
- **A (推荐)**: 保留 lean_market 独立 bin, 但 internal call into `runtime::user_task_runner::*` lib; turingos 也调同 lib. 老脚本 + TB-10 evidence chain 不受影响.
- B: Deprecate lean_market, 文档警告, Phase 6.2 后删除. (向后兼容性损失)

**TISR 默认选 A** (Phase 6 不动 lean_market binary, 只 lib 化复用).

### 10.2 turingos 是否包含 watch broadcast (依赖 REAL-BCAST-1)?

**选项**:
- A: watch broadcast 在 Phase 6.1 MVP 内 (依赖 REAL-BCAST-1 ship)
- **B (推荐)**: watch broadcast 在 Phase 6.2 (P1) — 不阻塞 MVP

**TISR 默认选 B**.

---

## 11. Phase 1 CLI-D 完成 — 输出 + 下一步

### 11.1 本 Track 完成

- ✅ 三层架构图 (CLI / Library / Kernel)
- ✅ Crate 组织 (`src/cli/` + `src/runtime/*_runner.rs` + `src/bin/turingos.rs`)
- ✅ 37 个 subcommand 完整 clap 树
- ✅ Global flag 设计 + 输出格式抽象 + 错误抽象
- ✅ Config 加载策略
- ✅ Lib 化路径详细 (5 个新 runner ~4750 LOC)
- ✅ 与现有 11 个 bin 并存策略
- ✅ Test 架构 + 文档结构
- ✅ 与 in-flight (REAL-13/BCAST-1/13A) 协调 timing
- ✅ Phase 6 启动条件: G-Phase 收口 + REAL-13/BCAST-1/13A ship

### 11.2 Phase 1 整体完成

CLI-A (User Journey) ✅
CLI-B (Existing bin Catalog) ✅
CLI-C (Missing Capabilities) ✅
CLI-D (Architecture Spec) ✅

**Phase 1 总输出**:
- 4 份 markdown 文档 (含本文档约 ~3500 字)
- 37 个 subcommand 完整规划 (12 wrap + 25 新)
- ~5900 LOC Phase 6 实施估算 (Class 1-2; 无 Class 3/4)
- 0 个新 typed_tx variant 提议 (Kill Condition 5 不触发)
- 0 个 Class 4 surface 修改 (Kill Condition 7 不触发)

### 11.3 Phase 2 输入

Phase 1 输出是 Phase 5 `00_UNIFIED_CLI_SPEC.md` 主要输入；同时为 Phase 2-3 提供:
- **Phase 2 Track A** (typed_tx): TISR CLI 不动 typed_tx 已确认；Track A scope 缩小为 "现有 19 variant 如何重组" + 未来 Class 4 候选 (如 HumanTx) forward-bound
- **Phase 2 Track C** (Materializer): CLI text 模式已确认从 audit_dashboard 来; HTML 模式延后到 Phase 5 (web-from-scratch)
- **Phase 2 Track D** (经济学): turingos role assign + agent deploy 已确认走 genesis-phase, 不冲击 REAL-5/12 in-flight

---

## 12. 验收 — CLI-D 出口自检

- ✅ 三层架构 + Class 严格隔离
- ✅ 37 个 subcommand 完整 clap 树
- ✅ Global flag + 输出格式 + 错误抽象统一
- ✅ Lib 化重构 LOC 量化 (~4750)
- ✅ Cargo.toml 配置具体
- ✅ Test 架构 + 文档结构
- ✅ in-flight 协调 timing
- ✅ Kill Condition 4/5/7 全检查通过
- ✅ forward-bound 头部声明

**Phase 1 整体出口**: 4 个 track 全完成 + 无 Class 4 候选 surface + Kill Condition 全不触发 + Phase 6 实施路径清晰.
