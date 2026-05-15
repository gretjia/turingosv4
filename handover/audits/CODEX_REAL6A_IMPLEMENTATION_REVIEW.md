# CODEX REAL-6A Implementation Review

Reviewer: clean-context Codex (`gpt-5.5`, `xhigh`)
Date: 2026-05-15
Verdict: VETO

## Findings

- [P0] `EventResolveTx` compatibility is not proven and appears non-tail-add. The directive requires preserving old EventResolve evidence through a schema bump/dual-reader and a regression test, but the new `outcome` field is inserted before `epoch` in the canonical wire struct, not appended after prior fields: `src/state/typed_tx.rs`. The project’s canonical codec is bincode fixed-int with full-byte consumption. `#[serde(default)]` is not enough evidence for historical bincode payload replay, and the reviewer did not see the required old-EventResolve decode/grandfather regression. This blocks Class 4 signing/replay compatibility.

- [P0] SG-6A.7 is not implemented on the production exhaustion/deadline path. The directive says `EventResolveTx NO` on exhausted/deadline. The only runtime EventResolve helper emits `OutcomeSide::Yes`. The MaxTxExhausted cleanup emits `TerminalSummary`, with optional `TaskBankruptcy`, but no EventResolve NO. The real smoke hit `solved=false`, `hit_max_tx=true`, yet `event_resolve: 0`.

- [P1] Trust Root normalization promotes broad dirty pinned files outside the REAL-6A semantic surface. Passing Trust Root verify proves current bytes match the manifest; it does not prove those extra authority bytes were semantically audited for this atom.

- [P2] TraderView coverage is test-scaffold-thin relative to the stated requirement. The plan requires active TaskOutcomeMarket plus pool depth, price, budget/deadline, and scoped PnL. The helper emits only `event_id`, `price`, and `depth`, and the SG-6A.2 test only asserts the event id is present in `price_signals`.

## Positive Checks

- No ghost liquidity / CTF has meaningful source and audit support.
- Shared AttemptTelemetry slot hardening is fail-closed for unknown JSON and skips recognized `MarketDecisionTrace`.
- The REAL-6A report does not overclaim E2/E3; it explicitly limits the claim to substrate/visibility and rejects spontaneous-emergence claims.

## Verdict

VETO

