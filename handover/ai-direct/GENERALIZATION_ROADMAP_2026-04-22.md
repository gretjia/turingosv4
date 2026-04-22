# 泛化路线图 — Paper 1 → 2 → 3

**Date**: 2026-04-22
**Status**: Paper 1 (MiniF2F) 规划中；Paper 2/3 架构预留已定

---

## § 1. 宪法对泛化的态度

宪法 **Art. I.1.1** 明文支持两种谓词：
- **完美谓词**（Completeness=1, soundness error=0）→ MiniF2F / zeta_sum_proof
- **PCP 谓词**（Completeness=1, soundness error ε 极小）→ omegav4

TuringOS 微内核**必须**能承载两种，不能只写死在布尔谓词上。

---

## § 2. 三篇论文规划

### Paper 1 (当前): MiniF2F — 完美谓词基线
- **主 claim**：constitutional TuringOS 在 Lean4 布尔谓词域的完整实现 + 统计胜出
- **目标期刊**：arXiv 预印本（全网审计级证据）
- **Gate**：
  - ΣPPUT CI 下界 ≥ monolithic baseline
  - ΣPPUT on depth≥10 CI 下界 > 0
  - 6 seeds × N=50 (dual + step-only) + 3 seeds × N=244 (full MiniF2F)
  - 三家外审（Codex + Gemini + DeepSeek）全 PASS
- **Future Work 节**：预告 Paper 2/3 方向

### Paper 2: v3 zeta_sum_proof — 完美谓词 + 开放问题
- **主 claim**：同一 TuringOS 架构在 Lean4 open-ended proof（无预置 ground truth）域同样有效
- **关键差别 vs Paper 1**：问题空间不再是"MiniF2F 预定义 244 题"，而是"agent 自主探索 Riemann zeta 求和相关引理"
- **依赖**：
  - Paper 1 通过 peer review
  - M-1 `Predicate` trait 预留（此时用同一 `Lean4Oracle` 实现即可）
  - 换 `prompt_dir` + problem source → v3 `experiments/zeta_sum_proof/prompt/`
  - 换 axioms → v3 的 AUTORESEARCH_PLAN 里的 success criteria（depth 指标为主）
- **预估工作量**：2-4 周（90% 代码复用）

### Paper 3: omegav4 — PCP 谓词 + 无 ground truth
- **主 claim**：TuringOS 架构可承载 PCP 谓词，用于探索型开发（用户只有一个洞察，最终结果未必符合洞察）
- **关键差别**：
  - ∏p 不再是 Lean 编译器，而是统计 + OOS 检验 + external audit 组合
  - 已经出现过多次 false positive（F-23/26/28 REFUTED）
  - 架构价值是"快速 kill 错误假说"而非"发现正确假说"
- **依赖**：
  - M-1 `Predicate` trait 已就位
  - 新实现 `OmegaStatOracle` 跑 `scripts/paired_delta_ic_audit.py`
  - 宪法 = omegav4 的 `axioms.py` Ω1-Ω7
  - ArchitectAI/JudgeAI 就是 Art. V 的本来设计
- **预估工作量**：1-2 月

---

## § 3. 架构层预留（M-1：现在做）

### 3.1 `trait Predicate` 定义

在 Phase 8.C（OracleReceipt 重构时）加入：

```rust
// src/sdk/predicate.rs (新文件)

pub trait Predicate: Send + Sync {
    /// 验证 payload 在当前上下文 Q 中是否满足谓词
    fn verify(&self, payload: &str, context: &Q) -> Verdict;
    
    /// 谓词类型标识（用于 receipt 里识别哪个 oracle 判的）
    fn kind(&self) -> PredicateKind;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Verdict {
    Complete,
    PartialOk { confidence: f64 },  // Lean4Oracle 返回 1.0 表示 "unsolved goals" 但结构合法
    Reject(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PredicateKind {
    Lean4Boolean,       // Paper 1 & 2
    StatisticalPCP,     // Paper 3 (omegav4 用)
    ExternalAudit,      // 将来外部 agent challenge 通道用
}
```

### 3.2 `OracleReceipt` 扩展

```rust
pub struct OracleReceipt {
    pub payload_hash: [u8; 32],
    pub verdict: Verdict,
    pub predicate_kind: PredicateKind,
    pub oracle_id: Uuid,
    pub signature: Option<[u8; 64]>,  // Ed25519, Phase 10c 使用
}
```

### 3.3 `Lean4Oracle` 实现 `Predicate`

已有的 `verify_omega_detailed` / `verify_partial` 重构为 `Lean4Oracle::verify`。不改业务逻辑，只改 trait impl。

### 3.4 Bus API

`append_oracle_accepted(agent_id, payload, parent, receipt)` — 不变，但内部校验 `receipt.predicate_kind` 是否是允许的种类（当前只允许 `Lean4Boolean`）。

### 3.5 工作量

- 新文件 `predicate.rs`: ~30 行
- `Lean4Oracle` 重构为 trait impl: ~50 行改动
- `OracleReceipt` 扩展：~10 行
- 测试：`tests/predicate_trait.rs` 新 3 个测试
- **估计 2-4 小时**

不改 Paper 1 任何数值，也不改 Phase 8-10 任何计划。

---

## § 4. 泛化的宪法合规论证

### 4.1 Paper 2 合规性
- ∏p 仍然是 Lean 编译器（Art. I.1 布尔谓词）→ 全面合宪
- 问题空间变成 open-ended → Art. III.2 "渐进式披露" 要求更细致（agent 需从 Mathlib 检索相关引理）
- 其他条款（Art. II 广播 / Art. V 三权分立）与 Paper 1 完全一样

### 4.2 Paper 3 合规性
- ∏p 降级为 PCP 谓词（Art. I.1.1）— **宪法已许可**
- 关键挑战：soundness error ε 必须可量化并写入 axioms
- ∏p 对 agent 保密（Art. III.4）依然成立，但开源场景下已经部分失效（详见 DeepSeek 战略审计 S-1）
- Law 2 守恒需要定义：omegav4 里"Coin"对应什么？可能是 compute budget 或 audit veto power

---

## § 5. 时间线

| 里程碑 | 预估日期 | 关键交付 |
|---|---|---|
| Phase 8-10 完成 | 2026-05-15 | Paper 1 draft + arXiv submit |
| Peer review 反馈 | 2026-06-15 | 修订 + resubmit |
| Paper 2 启动（迁 v3 zeta_sum_proof）| Paper 1 接受后 | 架构 0 改动，只换 oracle config |
| Paper 2 实验完成 | Paper 1 接受后 +3 周 | Paper 2 draft |
| Paper 3 启动（迁 omegav4）| Paper 2 draft 后 | 新 `OmegaStatOracle` 实现 + axioms |
| Paper 3 实验完成 | Paper 2 draft 后 +2 月 | Paper 3 draft |

---

## § 6. 风险

- **风险 1**：Paper 1 被拒，整个泛化链条延后
  - 缓解：全网审计级证据减少被拒概率；若被拒按 reviewer 建议修，不放弃
- **风险 2**：Paper 2 zeta_sum_proof 问题太难，PPUT 长期为 0
  - 缓解：v3 已有 baseline 数据（v3 run6 90 agents × 6000 tx 在 zeta_sum_proof 上有 depth 积累），先复现 v3 再扩展
- **风险 3**：Paper 3 omegav4 的 PCP 谓词无法稳定收敛（soundness error 不小）
  - 缓解：参考 omegav4 自己 R1-R4 rebuild 里学到的方法（paired HAC_t, Bonferroni 等）；若真不收敛 → 论文本身就是"PCP 谓词在此类问题上不足够"的 negative result，仍有学术价值

---

## § 7. 链接

- 决策记录：`DECISIONS_2026-04-22.md` § 3
- PPUT reframe：`handover/audits/PPUT_REFRAME_2026-04-22.md`
- 最终方案（待写）：`PLAN_FINAL_PHASE_8_TO_PAPER_2026-04-22.md`
- v3 zeta_sum_proof：`https://github.com/gretjia/turingosv3/tree/main/experiments/zeta_sum_proof`
- omegav4：`/home/zephryj/projects/omegav4/`
