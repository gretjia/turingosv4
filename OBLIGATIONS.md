# TuringOS v4 — Obligation Ledger

Schema and rules: [skills/OBLIGATIONS_LEDGER.md](skills/OBLIGATIONS_LEDGER.md)

Per-project ledger of user-stated obligations to the agent. One file, one
schema, append-only IDs. Agents must reconcile at every implementation /
audit / completion turn.

Current overall status: **PARTIAL** — OBL-001 open, OBL-004 in-progress.

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

## OBL-004: TuringOS v4 全量违宪代码修复（守宪法 + 保留工程实践）
- Source: "你马上要进行的任务是一个非常严肃且重要的，就是修复本项目所有的违宪代码，要求非常细致的排查和修正，需要最高思考深度的模型来完成。不能有任何失误。我的原则是严格守宪法，尽可能保留本项目已经实现的优秀工程开发，在确定不违反宪法的情况下，给你自主裁决权力。如果你发现不守宪法，不需要问我意见，直接千万kill那个方案" + "我无论你是什么方案，我要的是全量修复，不留任何违宪。我提供给你的 GPT 方案也是他做了大量的研究的，我希望你认真参考、综合全部的意见" + "让 Turing OS v4 这个项目完全守宪法，在这个原则上尽可能保留已经开发的工程实践"
- Level: must
- Status: in_progress
- Scope: R1+R2+R3 audit 共 30 finding 去重至 24 atom，分 3 wave 实施 (Wave 1 自主 = PR-A/B/C；Wave 2 §6 batch 预授权 = PR-D；Wave 3 W3-1 retire legacy Node 折入 PR-D；W3-2 PredicateRegistry 起草 Class 4 charter)
- User-decisions (plan-grill 2026-05-24): Wave 1 拆 3 PR / Wave 2 批量预授权 / W3-1 retire Node / W3-2 起草 charter + per-atom §8
- Evidence: TBD — expected
  - `handover/audits/CONSTITUTION_REPAIR_R1R2R3_SYNTHESIS_2026-05-24.md` (synthesis report)
  - PR-A: `constitution-repair/wave1-pr-a-orphan-delete` branch + merged PR
  - PR-B: `constitution-repair/wave1-pr-b-shielding-judge` branch + merged PR
  - PR-C: `constitution-repair/wave1-pr-c-librarian-disjointness` branch + merged PR
  - PR-D: `constitution-repair/wave2-bus-cleanup-node-retire` branch + merged PR (Wave 2 5 atoms + W1-4 + W3-1)
  - Charter: `handover/tracer_bullets/TB-PREDICATE-REGISTRY-BIND_charter_2026-05-2X.md` + §8 ratification
  - cargo check + cargo test --workspace 全程 GREEN
- GPT-plan disposition: KILLED as TS prototype, NOT v4 — content was for `gretjia/turingos` (deprecated TS register kernel `delta(q,s)→(q',s',d')` with `.reg_q/.reg_d/MAIN_TAPE.md/FileChronos`)；仅 CanonicalEvent 14 种 + Gate 测试 15 条作为对照清单复用，余皆弃
- Last-touched: 2026-05-24
