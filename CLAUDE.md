# TuringOS v4

## What
Silicon-Native Microkernel for LLM Formal Verification Swarm.
Rust 2021, tokio, serde_json. Mission: MiniF2F Lean 4.

## Why
- 唯一对齐文档: `constitution.md` (反奥利奥架构的反奥利奥架构)

## How
- `cargo check` must pass before commit
- `cargo test` must pass before deploy
- `.env` never committed
- kernel.rs / bus.rs / wallet.rs edits: human confirm
- Economic changes: grep experiments/ (Run 6 lesson)
- Generator ≠ Evaluator: code author cannot be sole auditor

## Common Law (宪法 + 判例)
宪法高度压缩。不确定时查判例: `cases/C-xxx.yaml`
- 按条款查: `grep -l "Art. I.1" cases/*.yaml`
- 35 个判例 (C-001 ~ C-035)，覆盖全部宪法条款
- 50 个 v3 教训的完整映射: `cases/V3_LESSONS.md`
- 每个判例: facts → ruling → precedent (事实→裁决→先例)

## Docs (按需加载)
| 文档 | 何时加载 |
|------|---------|
| `docs/architecture.md` | 修改 src/ 核心模块时 |
| `docs/economics.md` | 修改经济引擎 (wallet/market) 时 |
| `docs/hardware.md` | SSH/部署/远程操作时 |
| `docs/experiments.md` | 创建或运行实验时 |
| `docs/rules.md` | 触发规则或修改规则时 |

## User
独狼研究员, 零编程基础 vibe coder. 中文为主, 技术术语英文可.
