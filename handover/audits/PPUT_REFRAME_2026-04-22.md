# PPUT Reframe — Phase 7 审计修订

> **2026-04-22 Amendment (C-066)**: 原 § 3 引用的 "Monolithic baseline ΣPPUT ≈ 3.4-5.7" 和 "Phase 2.1b ΣPPUT=83.32 baseline" 两项均基于 Agent A 污染数据（见 `PPUT_HISTORICAL_AUDIT` 的 CONTAMINATED 标注）。真实数据以 `PPUT_RAW_DATA_2026-04-22.md` 为准。
>
> 修正点：
> - ΣPPUT 不适合跨 run 比较（样本大小差异大）；改用 **Mean PPUT (solved)** 作跨 run baseline
> - Phase 7 Mean PPUT = 5.354 在历史上**排第 3**（非"灾难"），前 2 是 6.158 / 5.561
> - Depth≥10 历史上出现 2 次：20260419T181411 (1 个 depth-12) + Phase 7 (3 个 depth≥17)
> - Phase 7 独特贡献：首次 `per_tactic` gp_path + 首次 depth≥10 solves ≥ 2
>
> § 5 的 "所有 Phase 8-10 Gate 以 PPUT 表达" 原则不变；具体 Gate 数值见 `PLAN_FINAL_PHASE_8_TO_PAPER_2026-04-22.md` § 4 修订版。

---

**Date**: 2026-04-22
**Driver**: 用户在 synthesis 讨论中质询"PPUT 去哪了" → 立 C-052 判例
**Affects**: `handover/audits/SYNTHESIS_2026-04-22.md`（需加 amendment）、`handover/ai-direct/LATEST.md`、所有基于 solve-count 的 F-finding

---

## § 1. 事件经过

2026-04-22 审计讨论期间，用户指出："关于 PPUT 指标，我很久没有看到你用 pput 指标汇报结果了，这个指标我们没有关注吗？这是宪法制定者需要我们强制关注的。"

核查结果：
- `experiments/minif2f_v4/src/bin/evaluator.rs:3-8` 明文：`Sole optimization metric: PPUT (Progress Per Unit Time)`
- 宪法基础：**Art. I.1（布尔谓词）+ Art. I.2（统计信号 = PPUT）**
- `PPUT = 100% / time_to_omega (seconds)`；无 golden path → PPUT = 0

但本次审计汇报（`SYNTHESIS_2026-04-22.md` 以及前序多个 CHECKPOINT）全部用 solve count（9/20 vs 17/20）做 headline，PPUT 彻底缺席。这是**违宪汇报**。

→ 立判例 **C-052** (`cases/C-052_pput_as_sole_metric.yaml`)
→ 更新 **CLAUDE.md** 新增 "Report Standard (Art. I.2 强制)" 节

---

## § 2. Phase 7 真实 PPUT（重算）

数据源：`experiments/minif2f_v4/logs/templadder_n8_20260421T164014.jsonl`（9 行）

| 问题 | Depth | 用时 | PPUT | 贡献占比 |
|---|---|---|---|---|
| mathd_numbertheory_345 | **1** | 10s | **10.128** | 21.0% |
| mathd_algebra_359 | **1** | 10s | **10.031** | 20.8% |
| mathd_numbertheory_254 | **1** | 12s | 8.531 | 17.7% |
| mathd_algebra_160 | **1** | 12s | 8.202 | 17.0% |
| mathd_numbertheory_235 | **1** | 14s | 7.237 | 15.0% |
| mathd_algebra_171 | 3 | 29s | 3.412 | 7.1% |
| mathd_algebra_332 | **20** | 314s | 0.318 | 0.7% |
| imo_1964_p2 | **23** | 539s | 0.185 | 0.4% |
| imo_1981_p6 | **17** | 703s | 0.142 | 0.3% |

**ΣPPUT = 48.19** / **Mean PPUT (solved) = 5.354**

---

## § 3. 指标反转判决

对照 Monolithic baseline（Phase 2.1c 17/20, avg ~300-500s）：

| 指标 | Phase 7 step-only | Monolithic baseline | 判决 |
|---|---|---|---|
| Solve count | 9/20 (45%) | 17/20 (85%) | Monolithic 胜 |
| **ΣPPUT** | **48.19** | ≈ 3.4-5.7 (17 × 100/300-500) | **Phase 7 胜 8-14×** |
| Mean PPUT (solved) | 5.354 | ≈ 0.20-0.33 | Phase 7 胜 16-27× |

**宪法 Art. I.2 意义下 Phase 7 赢**，不是之前 synthesis 判的"输"。但有保留：

### 3.1 ΣPPUT 的组成被 trivial 一击即中主导

91.6% 的 ΣPPUT 来自 5 个 depth-1 快速 solve。这些 solve 跟 Phase 7 的 per-tactic δ-step 架构**几乎无关**（agent 一步 `by linarith` 就完事，中间没有 step_partial_ok）。

架构真正贡献的是：
- 3 个 depth≥17 的深度 proof
- 总 PPUT 贡献 ≈ 0.65（1.4%）

### 3.2 新的 Gate 标准

单看 ΣPPUT 可能被 trivial solve 掩盖"架构深度是否有价值"。必须拆开看：

| 指标 | 含义 | Gate 标准 |
|---|---|---|
| ΣPPUT | 整体吞吐 | CI 下界 ≥ monolithic baseline |
| ΣPPUT on depth≥10 | 架构深度贡献 | CI 下界 > 0（即：稳定出现深度价值） |
| Mean PPUT (solved) | 单次效率 | CI 下界 ≥ monolithic baseline |

只有**三个同时达标**才能说"constitutional TuringOS 有架构价值"。

---

## § 4. 对 SYNTHESIS_2026-04-22.md 的追溯修订

原 synthesis 的 **HOLD** 裁决依然成立（4 条 BLOCKER 仍然存在），但论据需修正：

| 原论据 | 修订后论据 |
|---|---|
| "Phase 7 9/20 vs monolithic 17/20 架构输 40pp" | "Phase 7 ΣPPUT=48.19 vs monolithic ≈3.4-5.7 架构赢 8-14×，**但**深度 PPUT 贡献仅 1.4% 属弱证据" |
| "Claude 声称 emergent depth 能架构胜利" | "Claude 声称 emergent depth 在 PPUT 意义上只有 1.4% 占比，主要 PPUT 来自 trivial solve，不构成'深度带来价值'的证据" |
| Gate 9 "N=50 solve rate CI 下界 ≥ 0.70" | Gate 9 "ΣPPUT CI 下界 ≥ baseline **且** ΣPPUT on depth≥10 CI 下界 > 0" |

---

## § 5. 修订后的方案要点

1. **所有 Phase 8-10 Gate** 都以 PPUT 表达（主指标）+ solve count（辅助）
2. **Phase 9 实验 acceptance**：4-6 seeds × N=50 的 **ΣPPUT CI 计算 + 深度分段 CI**
3. **Paper 核心 claim** 重写：
   - ~~"Phase 7 实现了 depth-N DAG"~~
   - → **"Phase 7 在 ΣPPUT 上 K× 超越 monolithic，且 ΣPPUT-on-depth≥10 分量 CI 下界 > 0"**
4. 所有 CHECKPOINT 文档必须首段包含 ΣPPUT，否则视为违反 C-052

---

## § 6. 对宪法层其他强制指标的警醒

PPUT 事件暴露的本质问题：**宪法强制内容没压缩进 CLAUDE.md，导致被忽视**。

已启动并行审计找其他盲点：
- `handover/audits/CONSTITUTIONAL_BLINDSPOT_AUDIT_2026-04-22.md`（审计 agent 进行中）
- `handover/audits/HARNESS_COMPRESSION_AUDIT_2026-04-22.md`（审计 agent 进行中）
- `handover/audits/PPUT_HISTORICAL_AUDIT_2026-04-22.md`（审计 agent 进行中）

这些返回后会再做一次 synthesis。

---

## § 7. 教训（写入判例 C-052 的 precedent）

1. 宪法 + evaluator 源码注释里的"Sole optimization metric"**必须**在 CLAUDE.md 有显式强制，否则会被忘
2. solve count 和 ΣPPUT 可以同时一升一降、方向矛盾；只汇报其中一个就是选择性陈述
3. 当"直觉上的胜负"（Phase 7 输）和"宪法指标上的胜负"（Phase 7 赢）矛盾时，**以宪法指标为准**
4. 任何外部审计（Codex/Gemini/DeepSeek）如果给出的论据是 solve count，都应立即被 CHALLENGE 并要求重算 PPUT
