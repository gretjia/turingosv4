# CODEX REAL-11 Execution Plan Alignment Review

## Findings

1. **[P1] live REAL-6B 禁止项还不是可失败的 evidence gate。** 架构师原文明示 REAL-11 阶段“暂不授权” live REAL-6B，并要求先回答 router path、actionable market、PnL/risk visibility 三个问题（`handover/directives/2026-05-15_REAL11_AGENT_ECONOMIC_ACTION_ACTIVATION_ARCHITECT_ORIGINAL.md:617-647`）。方案也把 Atom 5 写成 “No live REAL-6B”（`handover/directives/2026-05-15_REAL11_AGENT_ECONOMIC_ACTION_ACTIVATION_EXECUTION_PLAN.md:824-837`），并在 Audit 3 中列为审计项（`...EXECUTION_PLAN.md:1044-1054`）。但 Atom 5 report/verdict 字段只要求 router/provenance/audit/no forced-scripted（`...EXECUTION_PLAN.md:854-880`），Verification Matrix 也只列测试命令和 evidence command（`...EXECUTION_PLAN.md:935-974`）；唯一明确的 REAL-6B 检查是“报告声称 live REAL-6B approval”时失败（`...EXECUTION_PLAN.md:979-988`）。这只能抓过度声称，抓不住 micro-probe 实际误开 live AttemptPrediction/REAL-6B 但报告没写的关键错误。需要补一个硬 gate：扫描 runner env/config manifest/aggregate verdict/CAS or trace schema，发现任何未授权 live REAL-6B/AttemptPrediction live path 即 fail closed。

2. **[P2] Atom 4 的文件清单与 Harness allowed paths 不一致，会让低思考 worker 卡住或越界。** Harness Contract 的 allowed planned paths 没有列 `src/sdk/prompt.rs`（`...EXECUTION_PLAN.md:201-240`），但 Atom 4 implementation detail 明确把 `src/sdk/prompt.rs` 列为 PnL/TraderView 可见性相关文件（`...EXECUTION_PLAN.md:756-768`）。如果当前代码不能纯靠测试满足 SG-11.4.1，worker 必须编辑 prompt renderer；但按 allowed paths 打开的 `turingos_dev` run 会把它视为未授权路径，或者 worker 会为了避开 allowed-path 冲突而不实现 prompt visibility。需要二选一：把 `src/sdk/prompt.rs` 纳入 allowed paths，或在 Atom 4 明确规定只能通过现有 renderer/PromptCapsule hook 满足，不允许改该文件。

3. **[P2] SG-11.0.2 的“all reports”证据卫生在测试细节里被缩窄了。** 架构师要求 contaminated evidence 只能作为 contamination/remediation record，不能用于结论（`...ARCHITECT_ORIGINAL.md:27-40`），Atom 0 验收要求 “invalid evidence dir is excluded from all reports”（`...ARCHITECT_ORIGINAL.md:398-405`；方案也复制在 `...EXECUTION_PLAN.md:260-265`）。但 Atom 0 test assertions 只写成“the report does not cite the contaminated path as canonical”（`...EXECUTION_PLAN.md:497-504`）。这会漏掉两类关键错误：某个 REAL-10/REAL-11 report 把 contaminated run 当非 canonical 的结论输入使用，或后续 handover/report 没标明它 invalid/remediation-only。需要把测试改成扫描全部 conclusion-bearing REAL-10/REAL-11 reports/handover outputs：contaminated path 若出现，必须仅在 invalid/remediation-only 语境中出现，且不得参与结论统计。

4. **[P2] 新 REAL-11 gate 没有纳入 Execution/Trace matrix 更新路径。** 当前方案给出了 FC mapping（`...EXECUTION_PLAN.md:98-132`），但 allowed paths 和 Done Definition 都没有包含 `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` 或 `handover/alignment/TRACE_FLOWCHART_MATRIX.md`（`...EXECUTION_PLAN.md:201-240`, `...EXECUTION_PLAN.md:1080-1094`）。矩阵自己的 update protocol 要求新 TB/REAL ship 时为新 constitution clause / FC node touched 添加或更新 row，并引用新测试（`handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:253-259`；FC enforcement changes 也要同步 trace matrix，`handover/alignment/TRACE_FLOWCHART_MATRIX.md:190-196`）。REAL-11 会新增 `constitution_real11_*` gate 和 FC1/FC2/FC3/economy 证据面；如果计划不把矩阵更新列为 Atom 6/Done gate，低思考 worker 可以在 alignment metadata stale 的情况下宣布完成。

## Open Questions

1. 哪些具体 env/config/schema 字段是 live REAL-6B 的 fail-closed sentinel？建议方案直接列出要扫描的 flag/schema/tx kind，而不是只写 “No live REAL-6B”。
2. Atom 4 是否预期必须修改 `src/sdk/prompt.rs`？如果是，应把它加入 allowed paths；如果否，应说明现有 PromptCapsule/TraderView hook 的精确入口。
3. REAL-11 是否作为 `CONSTITUTION_EXECUTION_MATRIX.md` §S 的新行落地，还是更新 REAL-10 行为 REAL-10/11 bundle？方案需要指定，避免 ship 后矩阵漂移。

## Verdict

**CHALLENGE**

核心方向与架构师原文一致：原文已保存并引用，Atom 0-6 和 SG-11.0.*-SG-11.5.* 基本覆盖，REAL-10 clean/contaminated boundary、四类 MarketTxCategory、scripted fixture / `market_tx_count` 不得算 E2、风险等级、FC mapping、restricted surfaces 都有明确表达。但上面四个 gap 会让关键禁令和收束面缺少可失败 gate，尤其 live REAL-6B 禁止项必须在方案层收紧后再进入实现。
