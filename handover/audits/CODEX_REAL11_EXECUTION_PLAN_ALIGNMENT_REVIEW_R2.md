# CODEX REAL-11 Execution Plan Alignment Review R2

## Findings

1. **[P2] Atom 2 router positive-control 的 `audit_tape PROCEED` 仍被实现细节弱化为可选 evidence run。** 架构师原文把 router positive-control 定义为 wire proof，明确列出 `active market pool`、`scripted trader payload`、`BuyWithCoinRouterTx`、`audit_tape PROCEED`、`CTF conserved`、`PnL updated`（`handover/directives/2026-05-15_REAL11_AGENT_ECONOMIC_ACTION_ACTIVATION_ARCHITECT_ORIGINAL.md:452-463`），并要求 SG-11.2.1..7 覆盖 L4/L4.E、conservation、no ghost liquidity、no f64（同文件 `:465-475`）。当前计划在 verbatim gate source 里保留了 `audit_tape PROCEED`（`handover/directives/2026-05-15_REAL11_AGENT_ECONOMIC_ACTION_ACTIVATION_EXECUTION_PLAN.md:304-326`），但 Atom 2 implementation detail 将它写成 `assert audit_tape/aggregate verdict PROCEED if evidence run`（同文件 `:660-670`），而 Verification Matrix 的 concrete evidence command 只指定了 Atom 5 micro-probe runner（同文件 `:1108-1117`）。这会让低思考 worker 以 unit gate 代替 router positive-control evidence，在没有 `real11_router_positive_control_*` 运行和 `audit_tape/aggregate verdict PROCEED` 的情况下宣布 SG-11.2 green。需要把 Atom 2 evidence run、report path、audit_tape/aggregate PROCEED 设为 mandatory close gate，而不是 `if evidence run`。

2. **[P2] Forbidden claim / launch boundary 被声明了，但 overclaim 测试清单没有覆盖架构师原文的全部上线边界。** 架构师原文禁止把 REAL-10/REAL-11 说成 `E2/E3/E4`、`live REAL-6B approval`、`market-caused solve improvement`、`model ranking`、`autonomous secondary market alive`（`handover/directives/2026-05-15_REAL11_AGENT_ECONOMIC_ACTION_ACTIVATION_ARCHITECT_ORIGINAL.md:117-129`），并在上线策略中进一步规定当前只能称 `TuringOS Research Alpha`，不能称 `autonomous prediction market` / `emergent agent economy` / `market-proven performance improvement`，且 `agent economy beta` 至少需要一次 live non-scripted E2，`emergent market beta` 需要 E3（同文件 `:673-717`）。计划的 Non-Negotiable Claim Boundary 声明了大部分 forbidden claims（`handover/directives/2026-05-15_REAL11_AGENT_ECONOMIC_ACTION_ACTIVATION_EXECUTION_PLAN.md:36-72`），但 executable overclaim list 只覆盖 `E2/E3/E4`、`market_tx_count increase as emergence`、scripted fixture、live REAL-6B、price/forced/ghost/f64/off-tape/private/raw/dashboard 等（同文件 `:1119-1136`），漏掉 `market-caused solve improvement`、`model ranking`、`autonomous secondary market alive`、`real-world readiness`，以及 section 9 的 `autonomous prediction market`、`emergent agent economy`、`agent economy beta` / `emergent market beta` 阈值。按用户要求“验证测试能抓关键错误”，这些 launch/claim overclaims 应进入 `constitution_real11_claim_boundary` 或同等 gate。

## R1 Remediation Status

- **live REAL-6B 禁止项**：充分修复。Atom 5 现在要求 pre-run 和 post-run fail-closed（`handover/directives/2026-05-15_REAL11_AGENT_ECONOMIC_ACTION_ACTIVATION_EXECUTION_PLAN.md:876-927`），列出 env/config、manifest/report/dashboard、CAS/schema、tx/report label sentinels，并加入 `tests/constitution_real11_no_live_real6b.rs`（同文件 `:849-857`, `:1077-1086`, `:1235`）。
- **Atom 4 allowed paths**：充分修复。Harness allowed planned paths 现在包含 `src/sdk/prompt.rs`（同文件 `:201-244`），Atom 4 implementation detail 也列出该文件（同文件 `:792-804`）。
- **SG-11.0.2 contaminated evidence hygiene**：基本修复。Atom 0 现在要求所有 conclusion-bearing REAL-10/REAL-11 reports 和 handover outputs 只能以 invalid/remediation-only/excluded 语境引用 contaminated path，且不得进入 statistics、tables、E1/E2/E3/E4 verdict 或 next-step claims（同文件 `:504-540`）。
- **REAL-11 matrix update path**：充分修复。Allowed paths 和 Atom 6 均包含 `CONSTITUTION_EXECUTION_MATRIX.md`、`TRACE_FLOWCHART_MATRIX.md`、`REAL11_TRACE_MATRIX_UPDATE.md`，并新增 `constitution_real11_matrix_update` gate，要求 Execution Matrix 引用 REAL-11 和全部 REAL-11 tests，Trace Matrix 改动/不改动都有可检查路径（同文件 `:201-244`, `:981-1038`, `:1228-1230`）。

## Open Questions

1. Atom 2 是否应新增一个 explicit runner/command（例如 `scripts/run_real11_router_positive_control.sh` 或等价 cargo/evidence command）来生成 `handover/evidence/real11_router_positive_control_*`，再对该 runtime repo 跑 `audit_tape`？
2. `constitution_real11_claim_boundary` 的扫描范围是否应覆盖 launch/whitepaper/handover docs，而不仅是 REAL-11 reports？

## Verdict

**CHALLENGE**

R1 的四个问题已经实质修复，整体方案也与 REAL-11 的主方向高度一致：不扩大 REAL-8Y、不授权 live REAL-6B、先拆 StructuralMarketTx vs AgentEconomicActionTx、验证 router path、MarketOpportunityTrace、PnL visibility、micro-probe 和 decision branch。但 Atom 2 的 mandatory evidence/audit_tape 仍有可选化缝隙，claim-boundary gate 也没有覆盖架构师 section 9 的上线标签和 beta 阈值。修复这两点后，方案应可进入实现。
