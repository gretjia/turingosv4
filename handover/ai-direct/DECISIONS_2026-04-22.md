# 决策记录 — 2026-04-22

本文件锁定 2026-04-22 session 中用户对 5 个关键决策点的判定。后续任何改动需 append 新决策而非 overwrite。

---

## § 1. 战略方向重大调整

**原方向**（Phase 10 = Launch）：Phase 10 做完 → 开放给真实世界 agents 接入分工
**新方向**（Phase 10 = Paper）：Phase 10 做完 → 论文可发预印本、全网审计级证据 → **之后**再考虑接入

**用户原话**：
> "我要求严格合宪，在合宪的基础上进行工程优化。先能够写出发表在预印本可以接受全网挑战的 paper，也就是要求证据可被接受最严格的审计，数据扎实、统计学意义明显，之后再考虑接入真实世界 agents。"

**含义**：
- 所有 P0 项目按"论文级证据"而非"launch readiness"重新评估
- 工程妥协（如 decide/omega 白名单 B 方案）不可接受 → 必选 100% 合宪的 C 方案
- 外部接入（原决策 5）全部推迟到 Phase 11+（拿到 peer review 反馈后）
- 论文优先级 > launch 优先级

---

## § 2. 5 项决策（严格合宪框架下）

### 决策 1：`decide` / `omega` 禁令

**选项**：
- A 严格 ban — 违 Completeness=1
- ~~B 白名单 qualified 形式~~ — 工程近似，非 100% 合宪
- **C Mathlib 语境白名单** — 区分演绎 vs brute force，宪法纯粹
- ~~D Warning-only~~ — 违 Soundness

**用户选择**：**C** （严格合宪）

**含义**：Phase 8.D 需要 Lean 语法分析层识别调用上下文（Mathlib theorem 内部 vs agent top-level proof），工作量从 XS 升到 M。

### 决策 2：Phase 9 seed 池

**选项**：
- **A** `{74677, 31415, 2718, 141421}`（2 老 2 新）
- B 全新 4 个
- ~~C 2 seed~~ — 违 Art. I.2 统计收敛（power 46%）
- D 8 seed — 超 compliance

**用户选择**：升级到 **A+ 6 seed**（power ≈ 95%）

**含义**：Gemini power analysis 要求 N≥185；6×50=300 样本足够。Budget 预算需上调。

### 决策 3：ArchitectAI / JudgeAI 模型配置

**选项**：
- A Codex + Gemini
- B Codex + DeepSeek
- C Gemini + DeepSeek
- **D 三家共决（Codex + Gemini + DeepSeek）**

**用户选择**：**D**（paper 级审计必须代码+数学+战略三层独立签名）

**含义**：每次 JudgeAI veto 三家同时跑，任一 VETO → 停。月度 JudgeAI 运行成本预估 $1200-2400。

### 决策 4：硬 budget

**选项**：
- A $1000 — 原计划
- B $1500
- **新** $2000-2400 — paper 级 benchmark（N=244 × 3 seed × 2 condition）需更大预算

**用户选择**：**$2000 硬顶 + 20% 应急到 $2400**

### 决策 5：外部接入开放策略

**选项**：
- A 永久白名单 — 违 Art. V.2 精神
- B 完全 permissionless Day 1
- C 三阶段开放
- D 邀请码链
- **新** 推迟到 Phase 11+

**用户选择**：**推迟到 Phase 11+**（先 paper 后接入）

---

## § 3. 泛化能力（决策点外的补充）

**用户追问**：MiniF2F 之后是否考虑泛化到 v3 zeta_sum_proof（无现成 Lean 的问题）和 omegav4（开放式探索型开发）？

**回答**：
- **架构层**：现在就要做（半天工作量），预留 `trait Predicate` 接口让 Lean4Oracle 成为其一个实现，为 Paper 2/3 泛化不用返工微内核
- **实证层**：Paper 1 只 claim MiniF2F，泛化是 Paper 2/3 的工作

### M-1：Predicate trait 预留（现在做）

在 Phase 8.C 修 OracleReceipt 时，顺手加：
```rust
pub trait Predicate {
    fn verify(&self, payload: &str, context: &Q) -> Verdict;
}

pub enum Verdict {
    Complete,
    PartialOk { confidence: f64 },   // 为 PCP 谓词留位
    Reject(String),
}
```
`Lean4Oracle` 实现 `Predicate`。

### M-2：Paper 1 Future Work 明确泛化方向

Paper 写作时在 Future Work 节写：
- Paper 2 方向：v3 zeta_sum_proof（Lean 布尔谓词 + open-ended 问题）
- Paper 3 方向：omegav4（PCP 谓词，无 ground truth 的探索型开发）

---

## § 4. 宪法强制指标覆盖（2026-04-22 新立）

**C-052 立档**：PPUT 是宪法 Art. I.2 唯一优化指标，solve count 仅辅助

**CLAUDE.md 修订**：新增 "Report Standard (Art. I.2 强制, C-052)" 节

**追溯影响**：
- `SYNTHESIS_2026-04-22.md` 需加 PPUT amendment（见 `handover/audits/PPUT_REFRAME_2026-04-22.md`）
- 所有 Phase 8-10 Gate 改以 PPUT 表达

---

## § 5. 已启动的 harness 审计

三路并行后台：
- `PPUT_HISTORICAL_AUDIT_2026-04-22.md` — Phase 0-7 PPUT 时间线
- `CONSTITUTIONAL_BLINDSPOT_AUDIT_2026-04-22.md` — Art. I-V 其他被忽视的强制指标
- `HARNESS_COMPRESSION_AUDIT_2026-04-22.md` — CLAUDE.md / cases/ / rules/ 压缩原则审计

返回后做最终 synthesis，形成 `PLAN_FINAL_PHASE_8_TO_PAPER_2026-04-22.md`。

---

## § 6. 决策链的可追溯性保证

本文件作为 **source of truth** for 本次 session 决策。后续文件：
- `PLAN_PHASE_8_TO_10_2026-04-22.md`（需改名/补丁为 `PLAN_PHASE_8_TO_PAPER`）
- `ROADMAP_LAUNCH_2026-04-21.md`（Phase 10 部分标注"deferred to Phase 11+"）
- `SYNTHESIS_2026-04-22.md`（加 PPUT amendment 指针）

**引用本文件的标记格式**：`DECISION-2026-04-22 §N.M`
