# TuringOS v4 — Obligation Ledger

Schema and rules: [skills/OBLIGATIONS_LEDGER.md](skills/OBLIGATIONS_LEDGER.md)

Per-project ledger of user-stated obligations to the agent. One file, one
schema, append-only IDs. Agents must reconcile at every implementation /
audit / completion turn.

Current overall status: **PARTIAL** — OBL-001 open.

---

## OBL-001: 15 画像 DeepSeek 真实 Chrome 用户模拟全流程测试
- Source: "我要睡觉了，你派一个agent模拟我来进行完整测试，在本地.env文件内有deepseek api key，你让模拟agent使用一个deepseek api key，设置meta ai为deepseek v4 pro thinking mode on. worker ai 为deepseek v4 flash thinking mode off. 然后你来准备至少15个中等到艰难的测试题，重点要测试完整的流程，复杂问题的处理能力，以及后面的node看板是否完全接入了。以及node的丰富程度，agent market的实际运行程度。我需要你的agent要完全模拟真实用户，不要预设问题的答案，只给一个画像，让模拟尽可能真实。完整记录全部过程在本地，全过程中发现任何问题，你随时参与debug和提升。" + "模拟用户在chrome中操作，例如操作鼠标等，模拟的越贴近真实越好。另外我不要中间停机等我确认任何消息，我要求必须是所有测试完全完成。有任何bug你去修复，不要等我确认"
- Level: must
- Status: open
- Evidence: TBD — expected `sessions/nightly_deepseek_user_sim_<ts>/summary.md` + `metrics.json` + per-persona transcripts/screenshots
- Last-touched: 2026-05-24

## OBL-002: Node/Polymarket UI 可见性修复（生成代码后立即可见进度 node）
- Source: "我第一次手动测试，在我点击"生成代码"后，没有进度node显示的界面。这就是最大的失败，我确认后台已经接入了，claude design也提供了设计，只是不知道为什么frontend看不到"
- Level: must
- Status: satisfied
- Evidence: codex 9:30 回执 — `frontend/src/components/agent-attempts-panel.ts` 重写为 design-system/preview/pattern-agent-presence.html 节点画布；`src/web/market_view.rs` 移除默认 stake/bounty 常量；`tests/cli_web_routes_smoke.rs` 94 passed；4 page CDP 验证 panelCount=1 canvasCount=1 LIVE=true transportOnly=true；multi-agent audit (Constitution + Runtime-evidence + Frontend-design) 均 PROCEED
- Last-touched: 2026-05-24

## OBL-003: 跨平台 Obligation Ledger Harness（本机制本身）
- Source: "harness的提升不是为了本次任务，是为了通用性，保证以后我们沟通的重要事项不被后续的任务书沟通而丢掉上下文" + "我的要求是你的harness提升应该是跨平台的，不仅仅是为了codex" + "派Karpathy agent审计你的计划，harness的架构要有karpathy的架构哲学观"
- Level: must
- Status: satisfied
- Evidence: `skills/OBLIGATIONS_LEDGER.md` (canonical skill, 195 行) + `OBLIGATIONS.md` (本 ledger) + `AGENTS.md §11/§14/§16` patches (+44 行) + `CLAUDE.md §5` patch (+5 行) + user "批准" 2026-05-24
- Last-touched: 2026-05-24
