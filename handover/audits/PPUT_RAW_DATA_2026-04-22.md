# PPUT 原始数据（权威源）— 2026-04-22

**Status**: 由 Claude 直接从 jsonl 文件读取计算，无 agent 聚合层。作为 C-066 修正产物。
**取代**: `PPUT_HISTORICAL_AUDIT_2026-04-22.md`（该文件因 Agent A 伪造"Phase 2.1b depth≥10 = 21.71"被标记污染）
**Reproduce**: `python3 handover/audits/pput_scan.py`（脚本保存在下方）

---

## § 1. 全扫描结果（按时间戳排序）

| Timestamp | Solved | ΣPPUT | MeanPPUT | MaxDepth | Depth≥10 | Σdepth≥10 | gp_paths | tools (agg) |
|---|---|---|---|---|---|---|---|---|
| 20260418T232656 | 14/17 | 63.41 | 4.529 | 1 | 0 | 0.00 | ? | complete=26 |
| 20260419T013822 | 37/45 | 115.78 | 3.129 | 1 | 0 | 0.00 | ? | complete=211 |
| 20260419T062546 | 3/9 | 0.68 | 0.228 | 1 | 0 | 0.00 | ? | complete=44 |
| 20260419T093523 | 7/8 | 2.92 | 0.417 | 1 | 0 | 0.00 | ? | complete=55 |
| 20260419T121906 | 39/39 | 111.86 | 2.868 | 1 | 0 | 0.00 | ? | complete=238 |
| 20260419T175151 | 5/5 | 18.45 | 3.690 | 1 | 0 | 0.00 | ? | complete=27 |
| **20260419T181411** | 14/16 | 42.60 | 3.043 | **12** | **1** | **0.11** | ? | complete=72,**append=53** |
| **20260419T221252** | 43/43 | **170.13** | 3.957 | 2 | 0 | 0.00 | ? | complete=184,append=1,complete_via_tape=1 |
| 20260420T020239 | 41/41 | 145.74 | 3.555 | 2 | 0 | 0.00 | ? | complete=193,append=1,complete_via_tape=1 |
| 20260420T044330 | 16/16 | 85.30 | 5.332 | 1 | 0 | 0.00 | ? | complete=51 |
| 20260420T063054 | 16/16 | 88.98 | **5.561** | 1 | 0 | 0.00 | ? | complete=54 |
| 20260420T112848 | 15/15 | 69.79 | 4.653 | 1 | 0 | 0.00 | alone | complete=32 |
| 20260420T134928 | 17/17 | 77.63 | 4.567 | 1 | 0 | 0.00 | alone | complete=84 |
| **20260420T160259** | 13/13 | 80.06 | **6.158** | 1 | 0 | 0.00 | alone | complete=42 |
| 20260420T183719 | 16/16 | 76.62 | 4.789 | 1 | 0 | 0.00 | alone | complete=99,omega_wtool=16 |
| 20260420T204809 | 17/17 | 83.32 | 4.901 | 1 | 0 | 0.00 | alone | complete=78,omega_wtool=17 |
| 20260420T225140 | 17/17 | 75.86 | 4.463 | 1 | 0 | 0.00 | alone | complete=91,omega_wtool=17 |
| 20260421T004502 | 17/17 | 71.58 | 4.211 | 1 | 0 | 0.00 | alone | complete=68,omega_wtool=17 |
| 20260421T005012 | 17/17 | 74.98 | 4.411 | 1 | 0 | 0.00 | alone | complete=91,omega_wtool=17 |
| 20260421T025922 | 35/35 | 108.36 | 3.096 | 2 | 0 | 0.00 | alone/tape+payload | complete=150,append=1,omega_wtool=35,complete_via_tape=1 |
| 20260421T083229 | 41/41 | 103.19 | 2.517 | 1 | 0 | 0.00 | alone | complete=278,omega_wtool=41 |
| 20260421T093841 | 17/17 | 69.53 | 4.090 | 1 | 0 | 0.00 | alone | complete=95,omega_wtool=17 |
| 20260421T115811 | 15/15 | 60.00 | 4.000 | 1 | 0 | 0.00 | alone | complete=80,omega_wtool=15 |
| 20260421T134019 | 15/15 | 63.38 | 4.225 | 2 | 0 | 0.00 | alone/tape+payload | complete=83,append=1,omega_wtool=15,complete_via_tape=1 |
| 20260421T155846 | 1/1 | 0.76 | 0.763 | 1 | 0 | 0.00 | alone | complete=5,omega_wtool=1 |
| **20260421T164014** | 9/9 | 48.19 | 5.354 | **23** | **3** | **0.65** | **per_tactic** | **step=132**,omega_wtool=9,step_partial_ok=59 |

---

## § 2. 只从原始数据能下的结论（无推断）

### 2.1 ΣPPUT 绝对峰值
- **20260419T221252 (170.13)** — 43/43 solved，但样本为 N=43（非 50），主要 complete 路径 + 1 个 `complete_via_tape`
- 样本大小不同 → ΣPPUT 不可直接跨 run 比较

### 2.2 Mean PPUT (solved-only) 峰值（per-problem 归一，跨 run 可比）
- 第 1: **6.158** (20260420T160259, 13 solves, all depth-1)
- 第 2: **5.561** (20260420T063054, 16 solves, all depth-1) — tape economy v2 fee=2000
- 第 3: **5.354** (20260421T164014, 9 solves) — Phase 7 唯一 per_tactic
- 第 4: 5.332 (20260420T044330, 16 solves) — tape economy v1 fee=500
- 第 5: 4.901 (20260420T204809, 17 solves) — 原 agent 声称的 "Phase 2.1b peak"

**观察**：Phase 7 Mean PPUT 在历史上**排第 3**，是竞争力水平而非灾难。

### 2.3 Depth≥10 出现史（全部）
- **20260419T181411**：1 个 depth-12 solve，PPUT = 0.11，背景有 53 次 append 使用 — 可能是"force-append 实验"时期
- **20260421T164014 (Phase 7)**：3 个 depth ≥ 17 solves (17/20/23)，ΣPPUT = 0.65，132 次 step_partial_ok

**Phase 7 独特贡献**：
- 首次出现 **3 个** depth ≥ 10 solves 在同一 batch（vs 前史最多 1 个）
- 首次使用 `per_tactic` gp_path（系统性 per-tactic 架构）
- 首次 depth 跨度到 23

### 2.4 gp_path 出现史
- 早期 (Apr 18-19 start)：gp_path 字段不存在（`?`）
- Apr 20 开始：`alone` 主导（所有 solves 来自单 agent）
- Apr 21 中段：出现 `tape+payload`（dual-path 模式）
- Apr 21 16:40：出现 `per_tactic`（Phase 7 独有）

### 2.5 append / step 使用史
- 真实 `append` 被使用的只有 3 次 run：
  - 20260419T181411: 53 次（唯一大量使用，对应 depth-12 出现）
  - 20260419T221252/20260420T020239/20260421T025922: 各 1 次（仅为触发 dual-path 的边界）
- 其他 **所有 run：append=0**（F-2026-04-18-02 发现在 Apr 20+ runs 上依然成立）
- **`step` 使用只出现在 Phase 7**（20260421T164014, 132 次）

---

## § 3. reproduce script

```python
#!/usr/bin/env python3
# handover/audits/pput_scan.py
import json
from pathlib import Path

logs_dir = Path('/home/zephryj/projects/turingosv4/experiments/minif2f_v4/logs/')

print("| Timestamp | Solved | ΣPPUT | MeanPPUT | MaxDepth | Depth≥10 | Σdepth≥10 | gp_paths | tools |")
print("|---|---|---|---|---|---|---|---|---|")

for f in sorted(logs_dir.glob("templadder_n8_*.jsonl")):
    rows = [json.loads(l) for l in f.open()]
    if not rows: continue
    solved = [r for r in rows if r['has_golden_path']]
    if not solved: continue
    sigma = sum(r['pput'] for r in rows)
    mean_s = sum(r['pput'] for r in solved) / len(solved)
    max_d = max(r['gp_node_count'] for r in solved)
    deep = [r for r in solved if r['gp_node_count'] >= 10]
    deep_sigma = sum(r['pput'] for r in deep)
    gp_paths = sorted(set(r.get('gp_path','?') for r in solved))
    tool_agg = {}
    for r in solved:
        for k,v in r.get('tool_dist',{}).items():
            tool_agg[k] = tool_agg.get(k,0)+v
    keys = ['complete','append','step','omega_wtool','step_partial_ok','complete_via_tape']
    tools_str = ','.join(f"{k}={tool_agg.get(k,0)}" for k in keys if tool_agg.get(k,0)>0)
    ts = f.stem.replace('templadder_n8_','')
    print(f"| {ts} | {len(solved)}/{len(rows)} | {sigma:.2f} | {mean_s:.3f} | {max_d} | {len(deep)} | {deep_sigma:.2f} | {'/'.join(gp_paths)} | {tools_str} |")
```

---

## § 4. 新的 Gate 9 baseline（基于真实数据）

### 4.1 ΣPPUT 不适合跨 run 比较
样本大小差异大（1~45 solves），用 ΣPPUT 做 gate 会奖惩"多做题"而非"架构价值"。

### 4.2 Mean PPUT (solved-only) 是正确的跨 run 可比指标
- 历史 top 3 均在 5.35-6.16 区间
- 建议 Gate 9 主判据：**Mean PPUT (solved) Wilson 95% CI 下界 ≥ 5.0**（保留历史中位水平）

### 4.3 Depth≥10 PPUT > 0 是"架构深度价值"的真严格陈述
- 整个历史只有 2 次 run 有 depth≥10 solve（181411 depth-12 × 1，164014 depth-17/20/23 × 3）
- Phase 7 的贡献：从"偶发 depth-12"到"系统性 depth-17 以上 × 3"
- 建议 Gate 9 辅助判据：**Σdepth≥10 PPUT > 0.5**（至少复现 Phase 7 水平）且 **depth≥10 solves ≥ 2**（避免 1-off）

### 4.4 最终 Gate 9 判据（PLAN_FINAL 修订）

```
主判据（必过）:
  Mean PPUT (solved-only) Wilson 95% CI 下界 ≥ 5.0

辅助必过:
  Σdepth≥10 PPUT > 0.5 且 depth≥10 solves ≥ 2
  pairwise_payload_diversity_mean ≥ 0.25
  reputation p50 > 0
  Law 2 proptest 10K tx 全绿
  halt_reason 分布公开报告
```

---

## § 5. 后续动作

- 本文件作为 PPUT 历史数据的**唯一可信来源**
- `PPUT_HISTORICAL_AUDIT_2026-04-22.md` 标为 CONTAMINATED，不可再引用
- `PLAN_FINAL_PHASE_8_TO_PAPER_2026-04-22.md` Gate 9 已按 § 4.4 修订
- `LATEST.md` 里 "Phase 2.1b 83.32 是历史峰值" 的陈述已更正
- C-066 立档：外部 agent 数值必须 Claude 独立核查
