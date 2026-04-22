# External Audit Package — 2026-04-21

本次审计针对 TuringOS v4 commit `e0a75ec`（Phase 7 Turing per-tactic δ-step 合并后），由 Claude Opus 内部审计（5 路 sub-agent）产生若干重大声明，需要外部独立复核。

## 分工原则

**基于各模型能力特征按职责拆分，避免重复工作**（C-023: Generator≠Evaluator）：

| 模型 | 擅长 | 任务文件 |
|---|---|---|
| **Codex (GPT-5.4)** | Rust 代码细节、grep/trace 精度、条款→文件行号映射 | `codex_constitutional_correctness.md` |
| **Gemini 2.5 Pro** | 数学证明、算法复杂度、性能/内存数值估计、统计推断 | `gemini_math_algorithm_perf.md` |
| **DeepSeek-Reasoner** | 长链战略推理、机制设计、对抗博弈、未来路径选择 | `deepseek_mechanism_design.md` |

## 冲突裁决规则

三审分歧时按 `feedback_dual_audit_conflict.md`：**保守裁决胜出**（VETO > CHALLENGE > PASS）。任何一个外部模型 VETO 都触发 Claude 停止相应建议并回到判例复查。

## 触发方式（推荐）

```bash
# Codex (通过 codex_companion 或 codex CLI)
codex run < handover/ai-direct/EXT_AUDIT_2026-04-21/codex_constitutional_correctness.md

# Gemini (v3 .env 中的 GEMINI_API_KEY, 模型 gemini-2.5-pro)
cat handover/ai-direct/EXT_AUDIT_2026-04-21/gemini_math_algorithm_perf.md | \
  python3 -c "import sys,os,json,urllib.request; ..." # 或现有 v3 审计脚本

# DeepSeek-Reasoner (.env 中 DEEPSEEK_API_KEY)
cat handover/ai-direct/EXT_AUDIT_2026-04-21/deepseek_mechanism_design.md | \
  curl ... -d '{"model":"deepseek-reasoner",...}'
```

## 返回归档

每个外部审计返回后，归档至：
- `handover/audits/EXT_CODEX_2026-04-21.md`
- `handover/audits/EXT_GEMINI_2026-04-21.md`
- `handover/audits/EXT_DEEPSEEK_2026-04-21.md`

三份到齐后 Claude 做 `dual_audit_synthesis_2026-04-21.md`，挑出全票通过、两票通过、VETO 三档。
