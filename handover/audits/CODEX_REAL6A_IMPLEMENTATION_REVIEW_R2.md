# CODEX REAL-6A Implementation Review R2

Reviewer: clean-context Codex (`gpt-5.5`, `xhigh`)
Date: 2026-05-15
Verdict: VETO

## Findings

- **P0 - EventResolve 兼容解码仍然 fail-open。** `EventResolveTx` 的序列解码把 outcome 字段的任何解码错误都当作 legacy 缺字段并降级为 `OutcomeSide::Yes`：见 `src/state/typed_tx.rs` 和 `src/state/typed_tx.rs`。这不只是“旧 B2 wire 无尾字段”的兼容；它会吞掉坏的/畸形的 outcome 尾部错误，削弱 typed tx canonical replay 的 fail-closed 边界。现有测试只覆盖 legacy 缺字段解码为 YES，见 `tests/constitution_real6_task_outcome_market.rs`，没有覆盖“带损坏 outcome 尾字段必须拒绝”。R1 的合法旧 EventResolve 可读性和 legacy YES 签名 grandfathering 大体已补，但这个 Class 4 wire-schema 宽容解码仍是 ship blocker。

- **P1 - MarketDecisionTrace CAS 写入仍是 best-effort，存在隐藏 evidence skip 回归风险。** SG-6A.5 把 invest/no-trade decision trace 作为 CAS 证据锚点，但 evaluator 多处忽略 `write_market_decision_trace_to_cas` 的错误，例如 no-trade path 在 `experiments/minif2f_v4/src/bin/evaluator.rs`，buy submit path 在 `experiments/minif2f_v4/src/bin/evaluator.rs`。底层函数返回 `Result`，见 `src/runtime/market_decision_trace.rs`。当前 r6 evidence 里确实有 no-trade traces，所以这不是 r6 artifact 本身的缺失；问题是未来 smoke 可以在 CAS trace 写失败时仍通过 stdout/tool_dist 声称 no-trade evidence，违背 tape-first 边界。

## 核对结果

R1 的生产 exhaustion/deadline EventResolve NO 已补上：r6 smoke `aggregate_verdict.json` 为 `PROCEED`，`event_resolve=1`，PPUT `hit_max_tx=true`，且 CAS payload 显示 outcome 为 `No`。r3b/r4/r5 在报告中被明确列为 invalid/superseded，claim boundary 清楚。

runner build fail-closed 已补：`scripts/run_g_phase_batch.sh` 先 `cargo build --release`，失败即 `exit 6`，并有 0076 RED / 0077 GREEN artifact 支撑。Trust Root/restricted surface 风险对 REAL-6A claim 有边界说明；我没有看到额外 price-as-truth、ghost liquidity、f64 money conservation、或 off-tape truth 的阻塞问题。

## Verdict

VETO
