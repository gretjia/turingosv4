# Stage C Polymarket CPMM Lifecycle — Gap Audit (2026-05-10)

> **Class 0 audit** drafted by Claude post user re-affirmation
> "我不要凑合 ... CPMM 那个算法完整落地" 2026-05-10 session #37.
> Companion document to `handover/tracer_bullets/TB_N2_POLYMARKET_CPMM_LIFECYCLE_charter_2026-05-10.md`.

---

## §1. Audit scope + methodology

**In-scope**: Polymarket CPMM lifecycle per architect Part C §2.1 + §2.2 + §2.3
(`handover/directives/2026-05-02_lossless_constitution_polymarket_directive__part_C_updated_final_ruling.md`).

**Out-of-scope** (cuts per TB-N2 charter §2):
- RSP-M4 CLOB
- K.1-K.6 Stage D readiness gates
- A6 Polymarket-agent-bridge (broader prompt tool advertising)
- C.5 PromptCapsule evaluator wire-up

**Methodology**:
1. Architect Part C §2.1 verbatim formula extraction
2. Code grep of `src/state/typed_tx.rs`, `src/state/sequencer.rs`,
   `src/state/router_quote.rs`, `src/state/q_state.rs` for CPMM / pool /
   LP / market-resolution surfaces
3. Status comparison: spec requirement vs implemented transition
4. Gap classification: COMPLETE / PARTIAL / MISSING / OUT-OF-SCOPE

## §2. Architect Part C §2.1 verbatim spec (reference)

```
poolY * poolN = k

用户支付 payC，先 split 出：
  payC YES + payC NO

保留 YES，拿 NO 去 AMM 换更多 YES：
  dN = payC
  dY = - payC * poolY / (payC + poolN)
  getY = payC - dY
       = payC + payC * poolY / (payC + poolN)

但必须满足：
  1. payC 必须来自用户 balances_t debit；
  2. split 必须锁定 Coin 生成完整 YES/NO set；
  3. AMM poolY / poolN 必须来自 LP / Treasury 已存在资产；
  4. router 不得创造 ghost liquidity；
  5. pool reserves 是 outcome-token reserves，不是 Coin supply。
```

## §3. CPMM lifecycle inventory — current vs target

The complete CPMM lifecycle per architect §2.1 + §2.3 (Polymarket CTF):

```
                  [PROVIDE]                [TRADE]                  [RESOLVE]                  [WITHDRAW]
                  ─────────                ───────                  ─────────                  ──────────
on_init mint  →  MarketSeed/CpmmPool  →  BuyWithCoinRouter   →  EventResolve(YesWins)  →  CpmmLpUnwind   →  end
              →  (LP shares minted)   →  CpmmSwap            →  PoolResolve(Active→     →  CompleteSetRedeem
              →                       →  CompleteSetMint     →   Resolved)              →
              →                       →  CompleteSetMerge    →
```

### §3.1 PROVIDE — fully implemented ✅

| Transition | Source | Status | Architect §2.1 guard |
|------------|--------|--------|----------------------|
| `MarketSeedTx` (LP seeds Treasury-funded liquidity) | TB-13 (P-M3 re-applied) | ✅ COMPLETE | Guard 3 (LP/Treasury asset existed); Guard 4 (debit balances_t) |
| `CpmmPoolTx` (agent creates pool + seeds) | P-M4 | ✅ COMPLETE | Guard 3 + 4 |
| `CompleteSetMintTx` (Coin → YES + NO at 1:1) | TB-13 | ✅ COMPLETE | Guard 2 (lock Coin → mint complete-set) |

**Verified by code reading**: `src/state/sequencer.rs:2587` ([LP share insert
on accepted MarketSeed]), `src/state/q_state.rs:761` (`LpShareBalancesIndex`),
`src/state/typed_tx.rs:1255` (`CompleteSetRedeemTx`), monetary_invariant.rs
guards.

### §3.2 TRADE — fully implemented ✅

| Transition | Source | Status | Architect §2.1 guard |
|------------|--------|--------|----------------------|
| `BuyWithCoinRouterTx` (payC → split → swap one side → getY) | P-M6 | ✅ COMPLETE | All 5 guards (verbatim §2.1 formula) |
| `CpmmSwapTx` (YES ↔ NO via constant-product) | P-M5 | ✅ COMPLETE | Guard 4 (integer math; floor leaves dust in pool) |
| `CompleteSetMergeTx` (pre-resolution YES + NO → Coin) | P-M2 | ✅ COMPLETE | Guard 4 + 5 (no ghost liquidity at merge) |

**Verified**: `src/state/sequencer.rs:2694..2939` (admission arms for swap +
router); event-state-gate Phase F.9 (`task_markets_t[event].state == Open`
check; fail-closed via `ok_or(EventNotOpen)?`).

### §3.3 RESOLVE — **CRITICAL GAP** ❌

| Transition | Status | Code surface that NEEDS it |
|------------|--------|----------------------------|
| `EventResolveTx` (flip `task_markets_t[event].state: Open → Finalized(YesWins/NoWins)`) | ❌ **MISSING** | `src/state/typed_tx.rs:2701..2713` `CompleteSetRedeem` admission requires `task_markets_t.state == Finalized(side)`; **no path emits this state** |
| `CpmmPoolResolveTx` (flip `pool.status: Active → Resolved`) | ❌ **MISSING** | `src/state/router_quote.rs:284` writes `pool.status = PoolStatus::Resolved` in TEST ONLY; **no production transition writes this** |

**Code evidence of missing transitions** (`grep TaskMarketState::Finalized
src/`):
- `src/state/sequencer.rs:1543` — READ in apply path
- `src/state/sequencer.rs:1708` — READ
- `src/state/sequencer.rs:2107..2111` — READ in admission
- `src/runtime/adapter.rs:738` — READ in skip list

**Zero writes**. The state value is read by 5+ admission paths but written
nowhere. This makes the entire post-resolution path (CompleteSetRedeem,
CpmmLpUnwind if it existed) **unreachable**.

**Lifecycle implication**: agents that mint complete-sets / swap / provide LP
**have their funds permanently locked** because the event will never finalize.
Per `feedback_no_workarounds_strict_constitution` this is unacceptable
deposit-only state.

**TB-N2 atoms B2 + B3 close this gap.**

### §3.4 WITHDRAW — **CRITICAL GAP** ❌

| Transition | Status | Architect §2.1 implication |
|------------|--------|----------------------------|
| `CpmmLpUnwindTx` (agent burns LP shares → withdraws pro-rata reserves) | ❌ **MISSING** | Guard 3 ("AMM 来自 LP / Treasury 已存在资产") requires symmetric exit path |
| `CompleteSetRedeemTx` (winning side share → Coin at 1:1) | ⚠️ **ADMISSION-READY-BUT-UNREACHABLE** | Code admission lives at `src/state/typed_tx.rs:2701..2713`; rejects on `EventNotOpen → RedeemBeforeResolution` for events stuck in Open. With B2 landing, redeem becomes reachable. |

**Code evidence** (`grep -E "(LpUnwind|lp_unwind)" src/`):
- Zero hits. The transition simply does not exist in the codebase.

**LP share lifecycle**:
- `src/state/sequencer.rs:2587` — INSERT on accepted MarketSeed
- `src/state/sequencer.rs:2466` — comment "credit lp_share_balances_t[(provider, event_id)] += seed_yes"
- `src/runtime/audit_views.rs:179` — iterates LP balances for audit view (READ)

**INSERT + READ only. No DECREMENT path.** LP funds are dust-locked indefinitely.

**TB-N2 atom B4 closes this gap.**

### §3.5 Asymmetric pool seed — minor gap ⚠️

| Transition | Status |
|------------|--------|
| `CpmmPoolTx` with `seed_yes != seed_no` | ⚠️ **REJECTED** with `UnbalancedPoolSeed` per v4 simplification |

**Code evidence**: `src/state/typed_tx.rs` rejection class `UnbalancedPoolSeed`
documented as "P-M4 v4 symmetric-init simplification; asymmetric seed deferred
to future TB".

**Architect spec implication**: §2.1 formula `poolY * poolN = k` is general for
any positive `poolY`, `poolN`. The symmetric restriction is **not** in the
architect spec — it's a v4 implementation simplification. Real Polymarket pools
seed asymmetrically based on prior probability beliefs.

**TB-N2 atom B5 closes this gap.**

## §4. Architectural ambiguity — resolution authority

Architect Part C §2.1 specifies the math but does NOT specify the resolution
authority for events in the TuringOS proof-task context. Three options
documented in TB-N2 charter §5:

| Option | Class | TB scope |
|--------|-------|----------|
| 1. System-emit on lean-verify outcome | 3 | TB-N2 B2 (RECOMMENDED minimal path) |
| 2. Agent-signed oracle role threshold | 4 | Stage D K.3 ORACLE (forward) |
| 3. External real-world oracle | 4 | Stage D K.6 IRREVERSIBLE (forward) |

**This audit recommends Option 1 for TB-N2 B2** with explicit forward path to
Option 2/3 via wrapping (Option 1 emits the typed tx; Option 2/3 can later
delegate the emission decision to vote aggregation / external feed without
breaking B2/B3/B4 invariants).

## §5. Monetary invariant impact analysis

CLAUDE.md §13 economy laws + P3 Forbidden (ghost liquidity guard):

**B2 EventResolveTx impact**: NONE. Status-only mutation on `task_markets_t`.
`balances_t` / `conditional_collateral_t` / `lp_share_balances_t` / pool
reserves all UNCHANGED. monetary_invariant `total_supply_micro` UNCHANGED.

**B3 CpmmPoolResolveTx impact**: NONE. Pool status field flip. Reserves
preserved (architect §2.1 implication: reserves remain in pool until B4 LP
unwinds them; pool reserves are NOT redeemed at resolve, only LP shares
released for unwind).

**B4 CpmmLpUnwindTx impact** (the critical one):
```
Agent submits unwind(provider, event_id, lp_share_amount)
System computes:
  share_frac = lp_share_amount / pool.lp_total_shares
  withdraw_yes = pool.pool_yes * share_frac (integer floor)
  withdraw_no  = pool.pool_no  * share_frac (integer floor)

Mutations (atomic):
  pool.pool_yes -= withdraw_yes
  pool.pool_no  -= withdraw_no
  pool.lp_total_shares -= lp_share_amount
  lp_share_balances_t[(provider, event_id)] -= lp_share_amount
  conditional_share_balances_t[(provider, event_id, Yes)] += withdraw_yes
  conditional_share_balances_t[(provider, event_id, No)]  += withdraw_no
```

**Invariant preserved**: total YES shares in system = sum(pool_yes across all
pools) + sum(conditional_share_balances_t[*, *, Yes]) UNCHANGED across B4
mutation. Same for NO. **Dust from integer floor stays in pool** (architect
§2.1 explicit; consistent with P-M5 swap dust pattern).

**Architect §2.1 guard 4 (router no ghost liquidity) extended to B4**: LP
withdraw sum across all providers MUST equal pool reserves at resolve time
(modulo integer-floor dust held by pool until last withdrawal). **SG-N2-B4.7
tests this.**

## §6. Gap classification summary

| Gap | Severity | TB-N2 atom | Class |
|-----|----------|-----------|-------|
| Event resolution never written | CRITICAL (algorithm dead-ends) | B2 | 3 |
| Pool resolution never written | CRITICAL (pool can't close) | B3 | 4 |
| LP shares insert-only (funds locked) | CRITICAL (no withdraw path) | B4 | 4 |
| Asymmetric pool seed rejected | MINOR (v4 simplification) | B5 | 3 |
| End-to-end real-LLM smoke | LATE-STAGE (depends on B2-B5) | B6 | 2 |

**No surprise** Class-4 boundaries identified. B2 may stay Class-3 if architect
explicitly allows minimal system-tx resolution; B3 + B4 are Class-4 STEP_B by
default (sequencer admission boundary + agent admission new surface).

## §7. Forward path

1. User reviews this gap audit + charter
2. User decides on resolution authority option (1 / 2 / 3 from §4)
3. If user authorizes B2 scope, I draft B2 §8 packet + dispatch PRE-§8 dual
   audit (Codex G2 + Gemini DT) per `feedback_dual_audit` Class-3/4 timing rule
4. R1 → user §8 → B2 ship
5. Repeat for B3 / B4 / B5
6. B6 real-LLM smoke (autonomous after B2-B5)
7. B7 overall §8 cap

Estimated wall: 10-14 days; 4 separate per-atom §8 events.

## §8. Constitutional alignment statement

This TB does NOT amend constitution.md. All atoms work within:
- CLAUDE.md §13 economy laws (`1 Coin = 1 YES + 1 NO`; `on_init` sole mint)
- CLAUDE.md §3.1 FC1 runtime loop (every admission goes through predicate gate;
  L4 accept or L4.E reject)
- CLAUDE.md §3.2 FC2 boot (every transition replayable from genesis + tape + CAS)
- CLAUDE.md §3.3 FC3 meta (capsule derived from tape + CAS; raw logs shielded)
- Architect Part C §2.1 verbatim formula
- `feedback_no_workarounds_strict_constitution` (strict landing of LP unwind;
  no Layer-G-Skip / OBS-bucket)
- `feedback_no_batch_class4_signoff` (per-atom §8 for B3 + B4)

---

**End of Stage C Polymarket CPMM Lifecycle Gap Audit.**
