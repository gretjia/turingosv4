# Gemini 外部审计任务 — 数学、算法、性能数值

**项目**：TuringOS v4，commit `e0a75ec`
**任务性质**：数学证明 + 统计推断 + 性能数值复核。允许读代码、读日志、读 proof 文件。
**返回**：Markdown 报告。每个数值必须是可重新推导的。长度自行决定。

---

## 上下文位置（审计必读）

项目根目录：`/home/zephryj/projects/turingosv4/`

| 类型 | 路径 |
|---|---|
| 宪法（唯一 ground truth） | `constitution.md` |
| Law 2 CTF 守恒相关代码 | `src/sdk/tools/wallet.rs`、`src/bus.rs`（`invest` / `settle_portfolios` / `halt_and_settle` / `append_oracle_accepted`）、`src/kernel.rs`（Hayek bounty `kernel.rs:63-103`） |
| 守恒测试 | `tests/reward_pull_conservation.rs`（5 个测试）、`tests/wal_resume.rs` |
| Prediction market (f64 精度) | `src/prediction_market.rs` |
| 热路径代码（Karpathy 数值复核目标） | `src/bus.rs`、`src/ledger.rs`、`src/wal.rs`、`experiments/minif2f_v4/src/bin/evaluator.rs` |
| Phase 7 实验数据 | `experiments/minif2f_v4/logs/templadder_n8_20260421T164014.jsonl`（9 PPUT rows） |
| Phase 7 proof artifacts（用于判断 DAG 真实性） | `experiments/minif2f_v4/proofs/imo_1964_p2_*.lean`（depth-23）、`proofs/mathd_algebra_332_*.lean`（depth-20）、`proofs/imo_1981_p6_*.lean`（depth-17） |
| Phase 7 DAG 分析报告 | `experiments/minif2f_v4/analysis/PHASE7_DAG_ANALYSIS.md` |
| Phase 7 checkpoint | `handover/ai-direct/CHECKPOINT_PHASE_7_TURING_2026-04-21.md` |
| 历史 N=50 数据（variance 对照用） | `experiments/minif2f_v4/logs/templadder_n8_20260420T020239.jsonl`、`templadder_n8_20260419T013822.jsonl` |
| Lean oracle（∏p 实现） | `experiments/minif2f_v4/src/lean4_oracle.rs` |
| Claude 内部审计摘要（对照基线） | `handover/ai-direct/EXT_AUDIT_2026-04-21/claude_internal_findings.md` |
| 当前研究状态 | `handover/ai-direct/LATEST.md`、`handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` |
| 判例库（Law 2 / C-001 post-genesis minting 等） | `cases/C-001_post_genesis_minting.yaml` 等 |

commit: `e0a75ec`

## 背景

Claude 内部审计 / DAG 分析给出若干**数值声明**，需要独立复核。重点：Law 2 CTF 守恒完整性、Phase 7 DAG depth 分布显著性、Karpathy 优化收益估计、女巫防御机制的数学设计。

## 必答 7 项

### Q1. Law 2 CTF 守恒的完备性证明
目标：证明"1 Coin = 1 YES + 1 NO，on_init 是唯一合法铸币点"在所有代码路径上 invariant。

输入：
- `src/sdk/tools/wallet.rs`（credit/debit API）
- `src/bus.rs` `invest` / `settle_portfolios` / `halt_and_settle` / `append_oracle_accepted` 路径
- `src/kernel.rs` Hayek bounty `kernel.rs:63-103`
- `tests/reward_pull_conservation.rs`（5 个测试）

要求：
- 列出所有可能改变 total_supply 的代码点
- 对每个点证明：`Σ debit = Σ credit`（在 tx 完成后）
- 特别审计：**invest refund 路径**（`bus.rs:240` 附近）+ **Hayek bounty payout**（`bus.rs:357-360`）是否被 5 个测试覆盖？
- 如果有任何路径未覆盖，构造具体反例（tx 序列 → 最终余额之和 ≠ genesis_total）

### Q2. Phase 7 DAG depth 分布的"涌现"显著性
数据：`experiments/minif2f_v4/logs/templadder_n8_20260421T164014.jsonl`（9 solved，N=20）
Histogram: `{1: 5, 3: 1, 17: 1, 20: 1, 23: 1}`

判断：
- 这个分布能否从**单 agent LLM 背诵已知证明**得到？还是只能从**多 agent δ-step 真实构造**得到？
- 对 depth-23 的 imo_1964_p2、depth-20 的 mathd_algebra_332、depth-17 的 imo_1981_p6，分别读 `proofs/*.lean` 文件，判断：每个 tactic 是 LLM-plausible 还是必须通过 runtime feedback 才能得到？（给出每问题 1 个具体 tactic 证据）
- 对"emergent" 声明给出统计上合理的判定：(a) N=20 sample 是否足够区分"真涌现"和"幸运"？(b) 需要多少 seeds 才能 95% 置信度确认 DAG 多样性？

### Q3. Karpathy TOP-10 性能数值复核
Claude 声明 "append 热路径 20-40% 加速 + heap 30-50% 降低"。
对以下 3 条做独立数值估计：
- `src/bus.rs:244-246` 三重 clone 消除 — 估算每 tx 节省的 allocation bytes（假设 Event struct ~200B）
- `src/bus.rs:256-262` `author.to_string() + payload.to_string()` — 假设 N=50 problems × 8 agents × 180 tx avg，总节省多少 heap？
- `src/bus.rs:416-424` `Box<dyn TuringTool>` downcast — vtable lookup 的 cycle 估计是多少？在当前 tool 数量（~3-5 个）下改成 enum dispatch 的实际加速倍率？

给出每条：(a) Claude 数字 (b) 你的独立估计 (c) 差异说明

### Q4. reputation-weighted γ 曲线（P0-2 女巫防御）数学设计
目标：设计一个 founder_grant γ(n_solved) 函数，使得：
- 新 agent 零成本进入但首 10 题只能拿 ≤10% 正常 grant
- 解决 n 题后达到 100% grant
- 曲线光滑（无 step jumps）
- 累积总量 ≤ 宪法设定的 max_founder_outflow

给出：
- γ(n) 的数学形式（如 sigmoid / linear saturating / exponential）
- 参数选择（给出默认值）
- 证明：如果女巫攻击者开 M 个账号，总获利上限与 M 的关系是 O(?)（希望次线性）
- 与 C-001 (post-genesis minting forbidden) 的兼容性证明

### Q5. fingerprint cache（P0-5 oracle DoS 防御）数学模型
假设 N=50 problems × 8 agents × 平均 180 tx × 50% Lean verify 调用率。
- 估计 payload hash 去重后的实际 cache hit rate
- 假设 cache 存 10000 条 entries，每条 `<hash, PartialVerdict>` ~ 100B，总 memory 开销？
- 与直接扩 oracle worker 到 2× 并发对比，哪个 cost-effective？

### Q6. dual-mode N=50 实验的 power analysis
现状：Phase 7 step_only 模式 N=20 = 9/20；monolithic baseline = 17/20。
问题：dual-mode（step + complete 并存）预期 ≥ 15/20。
- N=50 下能否以 80% power 区分：H0 `dual == monolithic (17/20)` vs H1 `dual < monolithic (by 2+)`
- 需要几个 seed 配对才能压制 Bernoulli 噪声？
- 如果 dual-mode 结果是 `{35/50, 37/50, 33/50}` 三 seed，能否说"已恢复到 monolithic 水平"？

### Q7. prediction_market.rs f64 epsilon 风险评估
`src/prediction_market.rs`:
- CPMM 价格计算是否存在累积浮点误差会导致 `yes_price + no_price != 1.0` 的路径？
- 对 settle_portfolios 的 resolved_outcomes 计算，f64 → f64 加法在多少次累积后误差超过 ε=1e-6？
- 建议：quantize 到 fixed-point (e.g. u64 micro-coin) 是否必要？

## 返回格式

```markdown
# Gemini External Audit — 2026-04-21

## Q1 Law 2 守恒
- 未覆盖路径: ...
- 反例（若存在）: ...

## Q2 DAG emergence
- depth-23 tactic 证据: ...
- 统计判决: PASS/VETO/CHALLENGE

## Q3 Karpathy 数值
| 条目 | Claude | Gemini | 差异 |
...

## Q4-Q7 同上
```

## 禁止事项
- 不要对 Rust 语法做非数学评论
- 不要重复 Claude 已给出的定性结论，只做定量复核
- 遇到数据不足时明示 "insufficient data"，不要脑补
