# TuringOS v4 — Handover State
**Updated**: 2026-04-25
**Session Summary**: Paper 1 v2.1 round-3 双审 PASS/PASS (首次 PASS in 3 轮), v2.1.1 hygiene cleanup, 仓库 9 GB 减重, 全部推 origin. Paper arXiv-ready.

## Current State

### What works
- **Paper 1 终稿完成**: `handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md` @ tag `paper1-v2.1.1` (commit `4314805`)
  - 主张: 异质 prompt portfolio (B) vs 同质 (A) 在 hard MiniF2F 上 McNemar 单侧 p=0.0039 (Bonferroni-clear at α=0.0125)
  - 4 seeds × 3 conditions × 10 hard problems = 120 paired trials, 0 MEASUREMENT_ERROR
  - 所有 12 jsonl + 8 个 B-unique winning `.lean` 在 `handover/evidence/v2/` (含 `proofs/` 子目录)
- **3 轮 dual-audit 全部归档**:
  - R1 (v1, `2687882`): CHALLENGE/CHALLENGE
  - R2 (v2, `210f19b`): CHALLENGE/CHALLENGE — Gemini 抓出 mathd_algebra_246 模型漂移
  - R3 (v2.1, `d349a86`): **PASS/PASS** — 5 个 P0 全闭合
  - v2.1.1 (`4314805`): 应用 round-3 P1 hygiene (family wording / § 2 over-isolation / Appendix C path)
- **仓库干净**: `git status` 空; 18 个旧 worktree 删除释放 9 GB; 24 个 branch ref 全部存活
- **远端同步**: origin/main HEAD = `fd291d7`; tags `paper1-v2.1.1` + `archive/art-ii1-v3-abandoned-20260416` 已推

### What's broken/incomplete
- **Markdown → LaTeX 未转换**: arXiv 投稿需 `.tex`，paper 当前为 Markdown
- **arXiv 元数据未定**: title 已锁定，但 categories / license / authors / orcid 未填
- **v2.2 deferred items** (双审都说"留给 camera-ready"，不 gate arXiv):
  - P1-A: problem-cluster sensitivity analysis (cluster-bootstrap 或 mixed-logistic with problem random effect)
  - P1-D: per-condition token-budget table
  - P1-E: Docker build/run transcript
  - P2-B: Appendix C node-count + winning-agent extraction

### Active experiments
- 无运行中实验
- 仍有 1 个外部进程: codex CLI broker (`pid 348391`) 指向 `phase-8a-snapshot` worktree (保留中)

## Next Steps

1. **arXiv 投稿准备** (主路径)
   - Markdown → LaTeX 转换 (pandoc + 手工 polish)
   - arXiv categories: 候选 cs.AI (primary) / cs.LG / cs.LO (secondary)
   - License: CC-BY 4.0 推荐
   - 提交后 24-48h 内审核 + 上线

2. **v2.2 polishing** (camera-ready 前；可平行做)
   - P1-A: cluster-bootstrap or mixed logistic (~1h python)
   - P1-D: token-budget table (从 jsonl 抽 token 字段)
   - P1-B: easy-set 在 BUILD_SHA `29ab43a` 下重跑 (~3h)
   - P1-E: Docker build/run transcript

3. **Paper 2 / Paper 3 路线** (推迟到 Paper 1 接收后)
   - Paper 2: 跨模型验证 (gpt-5 / opus / qwen-* 在同 harness 上)
   - Paper 3: 跨基准 (LeanDojo / MiniF2F-validation / FLT)

## Open Questions
- arXiv title 是否需要再压缩? 当前: *"Prompt Heterogeneity Improves Multi-Agent LLM Solve Rate on Hard MiniF2F Problems: A Pre-Registered Paired A/B Study"* (~17 词)
- 投稿后若 reviewer 要 v2.2 deferred 的某项，优先级如何排?
- 是否同时投递 OpenReview 公开评议轨道?

## Reference
- Paper: `handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md` (tag `paper1-v2.1.1`)
- Round-3 verdict: `handover/audits/DUAL_AUDIT_V2_1_VERDICT_2026-04-25.md`
- Pre-reg: `handover/preregistration/PREREG_E1V2_HETEROGENEITY_2026-04-23.md` + `E1v2_RESULTS_2026-04-24.json`
- Notepad: `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` (F-2026-04-25-01)
- Latest commits: `fd291d7` (hygiene) ← `4314805` (v2.1.1) ← `d349a86` (v2.1)
- C-070 validated: pre-submission dual-audit + pre-reg + N≥3 ablation 制度通过 3 轮独立 adversarial 审计
