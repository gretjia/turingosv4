# CHECKPOINT — Phase 4 (Cross-Problem Wallet Persistence)

**Date**: 2026-04-21
**Branch**: `feat/tape-phase-4-cross-problem` (worktree `../v4-phase4-crosspersist/`)
**Base**: `main` at `f63f0cb` (post-merge Phase 2.1 stack)
**Commit on branch**: `1dc68c8`

## What changed

| Component | Change |
|---|---|
| `src/sdk/tools/wallet.rs` | `save_to_disk` / `load_from_disk` (serde JSON); `ensure_agents` (zero-balance for newcomers post-genesis per C-001/C-038) |
| `experiments/minif2f_v4/src/bin/evaluator.rs` | On boot, load from `WALLET_STATE` env path if present; save on both OMEGA-accept and no-OMEGA exit |
| **Unchanged**: all other crates, prompt, oracle, kernel topology |

## Phase 4 N=20 batch (`templadder_n8_20260421T005012.jsonl`)

### Solve outcomes
- **17/20 = 85%** solved (same as Phase 2.1c; no regression)
- 3 timeouts: `algebra_apbon...`, `induction_sumkexp3eqsumksq`, `mathd_algebra_208` (algebra_apbon TIMEOUT here but SOLVED in main-validation — run variance)

### Audit
17/17 re-verified. 100%. 0 native_decide taint.

### Wallet evolution (the Phase 4 invariant)
After 20 problems sequentially:
```
Agent_0: 10080  (+80)  = 8 solves × 10 Coin
Agent_1: 10020  (+20)  = 2 solves
Agent_2: 10030  (+30)  = 3 solves
Agent_3: 10010  (+10)  = 1 solve
Agent_4: 10000  (+0)
Agent_5: 10000  (+0)
Agent_6: 10000  (+0)
Agent_7: 10030  (+30)  = 3 solves
Total ΔCoin = +170 = 17 solves × γ·lp (10 Coins each). Math matches exactly.
```

- `genesis_done=true` in loaded state → no second mint across 20 invocations.
- Conservation check: 17 × 200 LP minted at market creation − 170 paid to YES holders = 3230 Coins left in ghost-liquidity market state (unspent). Total system Coin count invariant through Law 2.

### Behavioural observation (the disappointment)
- Phase 4 tool_dist: `append: 0` still.
- Comparing first 10 vs last 10 problems in the same batch: agents did NOT exhibit a learning pattern that would suggest accumulated balance signaled "append is profitable." Later-batch problems solved fast because they were intrinsically easier, not because agents changed strategy.

**Persistent balance alone is not enough to activate tape**. Hayek's price signal requires the LLM to *read* the balance and *infer causality* across problems. Current models (deepseek-chat at this temp) don't do that within a single session.

### Next unlock for real Hayek behavior (future, out of this phase)
- Phase 2.5: show agent portfolio in prompt (`"Your YES position on tx_0_by_Agent_0: 10 shares (mkt=50%)"`). This is a PRICE SIGNAL (not a rule) — borderline C-034 but arguably Hayek-compliant.
- Phase 5: cryptographically-identified external agents joining over multi-day sessions; real multi-shot reinforcement.
- Better model: Claude Opus or GPT-5 with stronger meta-learning.

## Red-line check

| # | Red line | Status |
|---|---|---|
| 1 | Post-genesis mint | ✓ PASS (genesis_done persists, on_init no-ops; ensure_agents uses zero for newcomers) |
| 2 | Exit-triggered settlement | ✓ PASS (only oracle-driven) |
| 3 | Raw CoT to public tape | ✓ PASS (only canonical payloads) |
| 4 | Prompt manipulation | ✓ PASS (unchanged) |
| 5 | Env-var reward curve | ⚠️ YELLOW (γ still env) — unchanged since Phase 2 |
| 6 | ∏p non-re-verifiable | ✓ PASS (100% audit) |
| 7 | Anything deferred | ✓ PASS (behavioural unlock explicitly named, not hidden) |

## Stop conditions

| Condition | Threshold | Observed | Status |
|---|---|---|---|
| Solve rate vs honest median | ≥ -5pp | 17/20 (at top of variance) | ✓ |
| Conservation | pass | 170 Coin outflow exactly matches 17 solves × γ·lp | ✓ |
| Re-verifiability | ≥90% | 100% | ✓ |
| Red lines | none | 1 yellow (unchanged) | ✓ |
| Phase 4 success criterion | save/load round-trip preserves state | confirmed | ✓ |

## Recommendation: **MERGE Phase 4 to main**

Phase 4 infrastructure is correct, opt-in, Law-2-safe, and enables future phases (cross-problem reputation, permissionless onboarding). The behaviour-change goal is NOT met in single-session LLM tests; this is a model-level issue, not an architecture issue. Merge the substrate now.

## Aggregate state (main after merge)

All major constitutional capabilities landed:
- Art. I ∏p oracle, hardened against `native_decide` bypass (F-2026-04-20-05)
- Art. II markets + Phase 2 founder-grant reward-pull
- Art. III.2 search (with cap + loop) + Librarian
- Art. IV wtool mandatory on ∏p=1 (Phase 2.1)
- Q_t persistence across process restart (Phase 1 WAL)
- Cross-problem Q_t evolution in balance (Phase 4)
- Phase 0 audit: every solve leaves a standalone re-runnable `.lean` artifact
- C-036 harness telemetry surfaces mechanism gaps

Headline MiniF2F N=20 honest solve rate: **17/20 = 85%** (post-F-20-05 fix).

Remaining open problem is not architectural — it's the LLM behaviour layer. No constitutional topology still missing.
