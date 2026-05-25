# TuringOS v4 — Obligation Ledger

Schema and rules: [skills/OBLIGATIONS_LEDGER.md](skills/OBLIGATIONS_LEDGER.md)

Per-project ledger of user-stated obligations to the agent. One file, one
schema, append-only IDs. Agents must reconcile at every implementation /
audit / completion turn.

Current overall status: **PARTIAL** — OBL-001 open, OBL-004 in-progress, OBL-005 blocked on the remaining FC2 map-reduce tick and FC3 feedback/reinit production gaps.

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
- Scope: R1+R2+R3 audit 共 30 finding 去重至 24 atom；user 2026-05-24 second-round directive "完全修复，不要凑合" 触发重审，扩至 **28 atom 跨 5 PR + 1 Class 4 charter**（补回 4 个 defer 的 finding：R2-NEW#3 `bus.snapshot()` 3-axis 签名 / R3 FC3-INV6 registry immutability 结构守卫 / R3-B4+W1-2 `build_agent_prompt` 删除+G3/G5/REAL-12 测试 port / R3-B3 `SanitizedErrorTag` newtype + Karpathy F6 V3L-09 legacy sweep）。Wave 结构：Wave 1 自主 = PR-A/B/C；PR-E 新增（W1-2 死表面清理+测试 port）；Wave 2 §6 batch 预授权 = PR-D 扩 9 atoms；W3-1 retire legacy Node 折入 PR-D；W3-2 PredicateRegistry charter 含 FC3-INV6 enforcement
- User-decisions (plan-grill 2026-05-24): Wave 1 拆 3 PR / Wave 2 批量预授权 / W3-1 retire Node / W3-2 起草 charter + per-atom §8
- Evidence: TBD — expected
  - `handover/audits/CONSTITUTION_REPAIR_R1R2R3_SYNTHESIS_2026-05-24.md` (synthesis report)
  - PR-A: **PR #139 MERGED 2026-05-24T11:22:55Z** (`constitution-repair/wave1-pr-a-orphan-delete`; W1-1+W1-3+W1-6; Class 1+2; Witness NO-VIOLATION)
  - PR-B: `constitution-repair/wave1-pr-b-shielding-judge` branch + merged PR
  - PR-C: `constitution-repair/wave1-pr-c-librarian-disjointness` branch + merged PR
  - PR-D: `constitution-repair/wave2-pr-d-bus-cleanup-node-retire` branch + merged PR (Wave 2 5 atoms + W1-4 + W3-1 + R2-NEW#3 bus.snapshot 3-axis + V3L-09 sweep = **9 atoms**)
  - PR-E (NEW for 完全修复): `constitution-repair/wave1-pr-e-build-agent-prompt-retire` branch + merged PR (W1-2 delete build_agent_prompt + port G3/G5/REAL-12 测试 to production surfaces `src/sdk/your_position.rs` + action menu source + `route_role_action` ; +R3-B3 SanitizedErrorTag moot if removed)
  - Charter: `handover/tracer_bullets/TB-PREDICATE-REGISTRY-BIND_charter_2026-05-24.md` — **v8 user/architect ratified 2026-05-24 as APPROVED v8 ALL-IN-ONE** after PR #139 merged. Implemented on `codex/w3-predicate-registry-bind`: W3-2A snapshot/activation/replay loader + W3-2B sequencer binding + W3-2C fixture migration + W3-2D predicate trait/verify_proof. Fresh evidence: `cargo check --workspace` exit 0; `cargo test --workspace --no-fail-fast` exit 0; `bash scripts/run_constitution_gates.sh` exit 0 `[k-1-5] total=138 failed=0`; W3-2 predicate tests (`constitution_predicate_registry_binding`, `constitution_predicate_binding_activation`, `constitution_predicate_registry_replay`, `constitution_predicate_registry_immutability`, `constitution_predicate_result_wire_freeze`) all GREEN; `constitution_matrix_drift` GREEN; clean-context Codex shipping witness `NO-VIOLATION`.
  - cargo check + cargo test --workspace 全程 GREEN
- GPT-plan disposition: KILLED as TS prototype, NOT v4 — content was for `gretjia/turingos` (deprecated TS register kernel `delta(q,s)→(q',s',d')` with `.reg_q/.reg_d/MAIN_TAPE.md/FileChronos`)；仅 CanonicalEvent 14 种 + Gate 测试 15 条作为对照清单复用，余皆弃
- Last-touched: 2026-05-24

## OBL-005: 三 Flowchart 全链路覆盖测试集设计与落地
- Source: "设计测试集，验证turingos 可以按照宪法要求全部跑通，宪法的三个flowchat全部参与工作，没有僵尸模块（有代码但是没有接通），也没有flowchat中有，但是目前代码中缺失的部分。"
- Level: must
- Status: blocked
- Scope: Design and, where feasible without Class 4 runtime changes, implement an executable constitution/flowchart coverage harness proving FC1 runtime loop, FC2 boot/replay, and FC3 meta-architecture are all connected; detect zombie code surfaces and flowchart nodes missing code/test bindings.
- Evidence: `handover/tracer_bullets/TB-FLOWCHART-COVERAGE-TESTSET_2026-05-24.md` (test-set contract + current blockers) + `handover/audits/FLOWCHART_LIVENESS_INVENTORY_2026-05-25.md` (constitution-only liveness inventory; no closure claim) + `handover/audits/TURINGOSV4_ARCHITECTURE_LIVENESS_MAP_2026-05-25.md` (complete architecture liveness map separating constitutional elements from required substrate / support invariants / product workload / legacy candidates) + `handover/tracer_bullets/TB-FLOWCHART-FC2-FC3-CLOSURE_charter_2026-05-25.md` (draft Class 4 closure design for FC2 tick + FC3 feedback/reinit; constitution/source-of-truth critic `CHARTER-CONSTITUTION-PROCEED`; replay/data-shape critic `CHARTER-REPLAY-PROCEED`; not §8-ratified and not implemented). 2026-05-25 source-of-truth cleanup updated active matrices/tests to treat only `constitution.md` FC blocks + pinned hashes as current flowchart topology authority. Old extracted element files and old trace matrices are archival derived views only. FC1 real WorkTx provenance now binds `cas.proposal_telemetry:<cid>` and task-output keys instead of `k.read`/`k.write`; FC2 map-reduce tick is still marked `MISSING`; FC3 logs feedback to ArchitectAI and re-init semantics are still marked `MISSING`/`EXTERNAL_ONLY` instead of being covered by deep-history support invariants. Full certification remains blocked until missing production paths are implemented or constitutionally superseded.
- Verification: 2026-05-25 PASS — `cargo test --test constitution_flowchart_source_alignment` (7 passed), `cargo test --test constitution_flowchart_livenow` (4 passed), `cargo test --test tb_7_authoritative_routing` (5 passed), `cargo test --test generate_emits_work_tx_smoke` (4 passed), `cargo test --test fc_alignment_conformance` (27 passed, 6 ignored), `cargo test --test constitution_matrix_drift` (3 passed), `bash scripts/run_constitution_gates.sh` (`[k-1-5] total=140 failed=0`), and `cargo test --workspace --no-fail-fast` (exit 0).
- Last-touched: 2026-05-25
