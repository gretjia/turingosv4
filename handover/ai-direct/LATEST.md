# TuringOS v4 — Handover State
**Updated**: 2026-04-13
**Session Summary**: v4 harness 从零搭建，通过 40/40 沙盒验证

## 本次 Session 完成的工作

### 1. v3 深度宪法审计
- 3 个并行 Opus agent 审计 v3 代码库 vs constitution.md
- 发现 12 个违宪项 (V-001~V-012)，按 CRITICAL/HIGH/LOW 分级
- 代码质量评分 6/10："结构良好但积累了显著技术债的研究原型"
- 结论：**在 v3 基础上做宪法重构 (v3.5)，而非 v4 全部重写** — 但 harness 从零搭建

### 2. Harness Best Practice Research (6 篇文献)
- [Meta-Harness (arXiv:2603.28052)](https://arxiv.org/html/2603.28052v1) — 原始 trace > 摘要，环境引导
- [Anthropic Harness Design](https://www.anthropic.com/engineering/harness-design-long-running-apps) — Sprint 分解，生成者≠评估者
- [Anthropic Effective Harnesses](https://www.anthropic.com/engineering/effective-harnesses-for-long-running-agents) — JSON > Markdown，Git-based 状态
- [HumanLayer CLAUDE.md Guide](https://www.humanlayer.dev/blog/writing-a-good-claude-md) — < 60 行，Progressive Disclosure
- [HumanLayer Skill Issue](https://www.humanlayer.dev/blog/skill-issue-harness-engineering-for-coding-agents) — 最小启动，back-pressure 静默
- [Hermes Agent](https://github.com/nousresearch/hermes-agent) — Skills 程序性记忆，FTS5 搜索

### 3. v4 Harness 架构设计 (3 轮迭代)
- v1: 直接迁移 v3 → 被双审计否决 (只实现宪法 40%)
- v2: 架构师指出 "harness 不需要实现运行时架构" → 层次澄清
- v3 (最终): **compression is intelligence** — 通用内核 + SDK 外层

### 4. v4 Harness 搭建完成
详见下方 Current State。

### 5. Common Law 系统
架构师创意升级：宪法(抽象原则) + 判例库(具体释法) = Common Law。
11 个判例 (C-001~C-011) 从 v3 的 9 个 incident + 2 个设计决策提取。

### 6. 沙盒验证 40/40 PASS
`tests/harness_validation.sh` — 纯 bash 模拟，零 API 费用。

---

## Current State

### Harness 组件清单

| 组件 | 数量 | 状态 |
|------|------|------|
| CLAUDE.md | 35 行 | DONE — Progressive Disclosure |
| constitution.md | 从 v3 复制 | DONE |
| Hooks | 3 个 (judge + build-check + session-end) | DONE — 40/40 验证通过 |
| Rules Engine | engine.py (150 行纯 Python) | DONE — 正确拦截+零误报 |
| Active Rules | 10 条 YAML | DONE — 从 v3 迁移 |
| Agents | 3 个 (auditor + monitor + proposer) | DONE — 清洁无 GAIA |
| Skills | 6 个 | DONE — 从 v3 迁移+升级 |
| Docs | 5 个 (Progressive Disclosure) | DONE |
| Cases | 11 个 (Common Law 判例库) | DONE — 全部有 constitution 链接 |
| Incidents | 9 个 (从 v3 迁移) | DONE |
| Traces | schema + sessions/ 目录 | DONE |
| VIA_NEGATIVA.md | 从 v3 迁移 | DONE |
| handover/ | bible + directives/ + ai-direct/ | DONE |
| Tests | 40 个验证 (harness_validation.sh) | DONE — 40/40 PASS |

### 与 v3 的压缩对照

| 维度 | v3 | v4 |
|------|----|----|
| CLAUDE.md | 116 行 | 35 行 (-70%) |
| Hooks | 8 个 (~23KB) | 3 个 (~3KB, -87%) |
| Rule engine | 10KB Python-in-Bash | 150 行纯 Python |
| Rules | 14 条 | 10 条 (-29%) |
| **新增** | — | cases/ (11 判例) + docs/ (5 文档) + traces/ |

### 未迁移 (后续工作)

| 组件 | 说明 | 优先级 |
|------|------|--------|
| `src/` Rust 代码 | kernel.rs, bus.rs, prediction_market.rs 等 | HIGH — 下一步 |
| `experiments/` | minif2f_v2, zeta_sum_proof (活跃实验) | HIGH |
| `Cargo.toml` | 工作区配置 | HIGH |
| bus.rs 分解 | 999 行 → 3 模块 (~333 行/模块) | HIGH |
| 领域知识去耦合 | bus.rs forbidden_patterns, prompt.rs lean4_tools | MEDIUM |
| 单元测试补全 | kernel.rs, bus.rs, actor.rs 零测试覆盖 | MEDIUM |
| `autoresearch/` | Phase 0b 自动研究管线 | LOW |

---

## 关键设计决策 (本次 session)

1. **Harness ≠ Runtime**: 宪法描述运行时架构 (信号量化/广播/屏蔽)。Harness 只做开发环境脚手架。不要在 harness 里实现运行时功能。

2. **Compression is Intelligence**: 不迁移 v3 的 54 个文件。压缩为通用内核 (engine.py + judge.sh) + 外部 SDK (rules/ + docs/ + cases/)。内核零领域知识。

3. **Common Law**: 宪法高度压缩导致释法困难。判例库 (cases/) 提供具体先例。每个判例: facts → ruling → precedent。Agent 不确定时查判例。

4. **双审计流程**: SPEC 写完后送 Codex + 独立 Claude Agent 双审。v1 被打回 (只覆盖宪法 40%)。修正后通过。Generator ≠ Evaluator 原则贯彻。

5. **Hook API 真实性**: v3 hook 使用 JSON over stdin (jq 解析)，不是 argv。v4 所有 hook 严格匹配 Claude Code 真实协议。

---

## Next Steps (给下一个 session)

1. **迁移 Rust 代码**: 从 v3 复制 src/ → v4，做宪法重构
   - bus.rs 拆分为 bus_core.rs + bus_market.rs + bus_lifecycle.rs
   - 移除 bus.rs 中 Lean 4 硬编码 (forbidden_payload_patterns → 配置注入)
   - kernel.rs get_market_ticker() 展示逻辑 → 移到 SDK 层
2. **迁移活跃实验**: minif2f_v2, zeta_sum_proof
3. **补全测试**: kernel.rs, bus.rs, actor.rs unit tests
4. **首次 cargo check + cargo test** 在 v4 中通过
5. **constitutional_check.sh** 路径更新为 v4

## Open Questions

- bus.rs 领域知识 (forbidden_patterns): 注入配置 vs 保持硬编码 (pragmatic)?
- kernel.rs presentation logic (get_market_ticker): 重构到 SDK 还是留着?
- v3 仓库保留还是 archive?
