# TB-N2-POLYMARKET-CPMM-LIFECYCLE — Charter (2026-05-10 session #37 open)

> **Class 0 charter** drafted autonomously by Claude post user re-affirmation
> "我不要凑合，我要的是宪法和 3 个 flow chart 完整落地。我要求的是 Poly Market
> 机制，根据架构师完整的算法，CPMM 那个算法完整落地。你来为我决定后续的方案"
> (2026-05-10 session #37).
>
> Awaiting per-atom architect §8 sign-off for B2 / B3 / B4 (Class-4 atoms;
> no batch §8 per `feedback_no_batch_class4_signoff`).

---

## §0. Tags (mandatory per `feedback_tb_phase_tag_required`)

- **phase_id**: P3 — RSP Economy Core (market sub-surface; 2026-05-02 amendment
  TB-11/12/13 ladder extended).
- **roadmap_exit_criteria_addressed**: P3 Exit #7 (resolution flow — market dimension);
  P3 Exit #8 (payout sum ≤ escrow — extended to LP withdrawal sum ≤ pool reserves);
  P3 Exit #12 (contribution DAG explains payout origin — extended to per-LP attribution).
- **kill_criteria_tested**: P3 Forbidden "ghost liquidity without explicit treasury
  debit" (LP unwind must not mint; only debit lp_share_balances_t against
  pro-rata reserve credit); CLAUDE.md §13 economy laws (`1 Coin = 1 YES + 1 NO`
  preserved at unwind; no post-init mint at resolve).
- **Authority basis**: user 2026-05-10 verbatim "你来为我决定后续的方案" +
  strict-landing principle "我不要凑合". Class-0 charter authority autonomous;
  Class-3 atoms (B2/B5) require fresh authorization; Class-4 atoms (B3/B4/B7)
  require per-atom architect §8.

## §1. Mission

Close the **CPMM algorithm completeness gap** per architect Part C §2.1
verbatim formula:

```
poolY * poolN = k
payC → split → payC YES + payC NO
retain YES, swap NO for more YES via constant-product AMM:
  dN = payC
  dY = - payC * poolY / (payC + poolN)
  getY = payC - dY
```

Architectural requirements (architect Part C §2.1 closing list):

1. payC 必须来自用户 balances_t debit ✅ (P-M6 BuyWithCoinRouter shipped)
2. split 必须锁定 Coin 生成完整 YES/NO set ✅ (P-M6 step 2 shipped)
3. AMM poolY / poolN 必须来自 LP / Treasury 已存在资产 ✅ (P-M3 MarketSeed shipped)
4. router 不得创造 ghost liquidity ✅ (P-M6 atomic shipped)
5. pool reserves 是 outcome-token reserves，不是 Coin supply ✅ (P-M4 sub-field shipped)

**Substrate level: ALL FIVE GUARDS SATISFIED**. So what's missing?

**The closing half of the lifecycle**:

```
Open event → Mint complete-set → Provide liquidity → Trade → ??? → Event resolves → ??? → Redeem winning side → ??? → LP withdraws
                                                                                                                  ▲
                                                                                                            three transitions
                                                                                                             never landed
```

Closing the lifecycle requires:

- **Event resolution transition** — `task_markets_t[event].state: Open → Finalized(side)`
- **Pool resolution transition** — `pool.status: Active → Resolved`
- **LP unwind transition** — `lp_share_balances_t` is currently INSERT-ONLY;
  agents who provided liquidity have no way to withdraw.

Without these three transitions, the CPMM algorithm is **deposit-only** —
liquidity goes in (via MarketSeed / CpmmPool / BuyWithCoinRouter mints
complete-sets) and never comes out. Per `feedback_no_workarounds_strict_constitution`
("我不要凑活") this is unacceptable AMBER-equivalent state even though the
constitution matrix marks all economy gate rows GREEN.

The matrix tracks **constitutional clause coverage**. This TB closes
**algorithmic CPMM completeness** — a different axis. Both must hold for
strict landing.

## §2. Out-of-scope (explicit cuts)

The following are NOT in TB-N2 scope:

- **CLOB-like signed orderbook (architect RSP-M4)** — Part C makes CLOB
  Polymarket's primary trading surface and CPMM optional. User asked specifically
  for "CPMM 那个算法完整落地" — CPMM. CLOB → forward separate TB if needed.

- **K.1-K.6 Stage D real-world readiness gates** — `REAL_WORLD_READINESS_REPORT`,
  `DOMAIN_SELECTION`, `ORACLE`, `CHALLENGE_COURT`, `SAFETY`, `IRREVERSIBLE_ACTION`
  are DEFERRED behind architect explicit ship gate per Stage C overall §8 §7.
  TB-N2 B2 uses a **minimal system-tx resolution path** (system-emit on lean-verify
  outcome) that is replaceable by K.3 ORACLE later WITHOUT breaking CPMM
  lifecycle invariants.

- **A6 Polymarket-agent-bridge (agent prompt-side tools for CompleteSet split /
  CpmmSwap / BuyWithCoin)** — Class-4 STEP_B per boot prompt §3. TB-N2 lands the
  agent-callable CpmmLpUnwindTx (B4); the broader Polymarket tool surface in
  prompt is forward TB scope.

- **Asymmetric pool seed across-the-board** — partial cut as **B5** (relax v4
  `UnbalancedPoolSeed` simplification per architect §2.1 general `k`). Architect
  spec explicitly allows poolY ≠ poolN; v4 narrowed it. B5 reopens it.

- **PromptCapsule evaluator wire-up (C.5)** — not Polymarket-specific.

## §3. Atom decomposition

Per `feedback_no_batch_class4_signoff`: every Class-4 atom requires
PER-ATOM architect §8. NO batching.

| Atom | Class | Authority needed | What it lands |
|------|-------|------------------|---------------|
| **B0** | 0 | autonomous | This charter (current doc) |
| **B1** | 0 | autonomous | Polymarket CPMM Lifecycle Gap Audit (companion doc) |
| **B2** | 3 | per-atom architect §8 | `EventResolveTx` system-tx: flip `task_markets_t[event].state: Open → Finalized(YesWins / NoWins)`. Resolution authority = system-emit on lean-verify outcome (minimal CPMM-completeness path; K.3 ORACLE layered later) |
| **B3** | 4 STEP_B | per-atom architect §8 | `CpmmPoolResolveTx` system-tx: flip `pool.status: Active → Resolved`. Triggered by B2 emit. Disables `CpmmSwap` + `BuyWithCoinRouter` (already enforced by Phase F.9 event-state-gate). Class-4 because touches sequencer admission boundary + extends pool state machine |
| **B4** | 4 STEP_B | per-atom architect §8 | `CpmmLpUnwindTx` agent-tx: agent submits with LP share amount, system burns shares from `lp_share_balances_t` + credits agent pro-rata reserves. Class-4 because Anti-Oreo agent → state-writer (mediated by sequencer); new agent admission. |
| **B5** | 3 | per-atom §8 (Class-3 if strictly additive; Class-4 if changes existing test semantics) | Relax `UnbalancedPoolSeed` rejection per architect §2.1 general `k`. Allow seed_yes ≠ seed_no. Constant-product invariant `poolY * poolN >= k_prev` preserved. |
| **B6** | 2-3 | autonomous after B2-B5 ship | End-to-end CPMM lifecycle **real-LLM smoke** (per `feedback_real_problems_not_designed`): proof task → MarketSeed → BuyWithCoinRouter → CpmmSwap → lean-verify resolves event → B2 system-emit EventResolve → B3 system-emit PoolResolve → B4 LP unwinds → CompleteSetRedeem winning shares. **First real-LLM full Polymarket cycle.** |
| **B7** | 4 | overall §8 cap | TB-N2 overall §8 sign-off after B2 + B3 + B4 individually shipped (per `feedback_no_batch_class4_signoff` does NOT replace per-atom §8) |

### Per-atom ship gates (preview; full spec drafted at each atom's pre-§8 packet)

**B2 SG-N2-B2.* (≥6 tests, system-tx admission):**
- B2.1 agent ingress rejects `EventResolveTx` with `SystemTxForbiddenOnAgentIngress`
- B2.2 system emit succeeds: `task_markets_t[event].state: Open → Finalized(YesWins)`
- B2.3 idempotent re-emit rejects with `EventAlreadyResolved`
- B2.4 unknown event rejects with `EventNotFound`
- B2.5 post-resolution `CompleteSetRedeem` admits with `outcome == YesWins` (existing TB-13 path engaged)
- B2.6 post-resolution `CompleteSetRedeem` rejects with `InvalidResolutionRef` if outcome mismatch
- B2.7 state_root advances; replay byte-equality
- B2.8 (defense-in-depth) lean-verify trigger → EventResolve emit chain works end-to-end on a fixture

**B3 SG-N2-B3.* (≥6 tests):**
- B3.1 agent ingress rejects `CpmmPoolResolveTx`
- B3.2 system emit: `pool.status: Active → Resolved` when event finalized
- B3.3 reject `CpmmPoolResolve` if event still Open (`EventNotFinalized`)
- B3.4 idempotent re-resolve rejects with `PoolAlreadyResolved`
- B3.5 post-resolve `CpmmSwap` admission rejects via existing `pool.status != Active` gate
- B3.6 post-resolve `BuyWithCoinRouter` admission rejects via existing gate
- B3.7 reserves UNCHANGED on resolve (resolve is status-only mutation; reserves preserved for LP unwind)

**B4 SG-N2-B4.* (≥8 tests, agent-tx admission + balance accounting):**
- B4.1 agent admission on resolved pool admits
- B4.2 agent admission on active pool rejects (`PoolNotResolved`; agents wait for resolve)
- B4.3 reject zero-share unwind (`LpShareZero`)
- B4.4 reject over-balance unwind (`LpShareBalanceExceeded`)
- B4.5 reject duplicate unwind (`LpAlreadyUnwound`) for full-balance unwind
- B4.6 partial unwind: lp_share_balances_t debit + pro-rata reserves credit; remaining share allowed
- B4.7 cumulative LP withdraw sum across all providers == total reserves at resolve time (architect §2.1 closing guard 4: no ghost liquidity at unwind)
- B4.8 monetary_invariant: total_supply_micro unchanged; conditional_collateral_t / lp_share_balances_t balanced shift
- B4.9 (defense-in-depth) integration test: B2 → B3 → multi-provider B4 → all reserves fully withdrawn

**B5 SG-N2-B5.* (≥3 tests):**
- B5.1 `CpmmPool` admits seed_yes ≠ seed_no (e.g. 5M / 3M)
- B5.2 subsequent `CpmmSwap` on asymmetric pool preserves `poolY * poolN >= k_prev`
- B5.3 `BuyWithCoinRouter` quote formula works on asymmetric pool per architect §2.1
  formula (`dY = - payC * poolY / (payC + poolN)` integer-floored)

## §4. Freeze conditions / kill criteria

TB-N2 is DEAD if any of:

- Codex G2 PRE-§8 dual audit + Gemini DT PRE-§8 dual audit BOTH VETO on any
  Class-4 atom (B3 / B4 / B7)
- Real-LLM B6 smoke produces `chain_invariant.delta != 0` or `verdict=NegativeDelta`
- Monetary invariant breaks at any B4 unwind (total Coin not conserved across
  on_init ceiling)
- Ghost liquidity introduced: B4 LP withdraw sum > pool reserves at resolve time
- Architect explicit §8 VETO

## §5. Architectural ambiguity to resolve before B2

**Resolution authority question** (architect-input desirable but not blocking):

The architect's Part C ruling §2.1 describes CPMM but does not specify event
resolution authority for TuringOS context. Three possible paths:

| Option | Class | What | Pros | Cons |
|--------|-------|------|------|------|
| **1. System-emit oracle (RECOMMENDED for B2)** | 3 | runtime injects `EventResolveTx` on lean-verify outcome | Class-3 minimal; tight coupling proof outcome ↔ market resolution; honest architectural representation; replaceable | Coupled to lean-verify success path; doesn't generalize to non-proof markets |
| **2. Agent-signed oracle role** | 4 + K.3 | designated agent role votes; threshold required | Generalizes; constitutional separation of powers preserved | Stage D K.3 scope; larger surface |
| **3. External oracle** | 4 + K.6 | real-world oracle (Polymarket-like) | Real-world fidelity | Stage D K.6 IRREVERSIBLE_ACTION scope; not v1 |

**Recommendation**: Option 1 for B2. Document as "minimal CPMM-completeness
resolution path; system-emit on accepted ProofTaskFinalize event; K.3 ORACLE
agent-signed layer is forward-bound and can wrap this with vote aggregation
without breaking B2/B3/B4 invariants."

User / architect may override at B2 §8 packet review.

## §6. Forbidden (during TB-N2 execution)

- ❌ Batch §8 across B2 / B3 / B4 (each Class-4 needs per-atom §8 per
  `feedback_no_batch_class4_signoff`)
- ❌ Class-4 audit dispatch BEFORE atom evidence (per `feedback_audit_after_evidence`)
- ❌ Workarounds / Layer-G-Skip / OBS-bucket for LP unwind invariant
  (per `feedback_no_workarounds_strict_constitution`)
- ❌ Synthetic-only B6 smoke (per `feedback_real_problems_not_designed`: real
  problems, not designed)
- ❌ Stage D K.1-K.6 work without separate architect §8 (TB-N2 stops at CPMM
  algorithm completeness; does NOT open Stage D)

## §7. Cadence + estimate

Per `feedback_iteration_cap_24h`: 24h cap per atom (Class 4 no AI-coder cap).

| Atom | Estimated wall | Class-4 §8 needed? |
|------|----------------|--------------------|
| B0 charter (current) | 30 min ✅ DONE-IN-PROGRESS | no |
| B1 gap audit | 30 min | no |
| B2 EventResolveTx | 1-2 days impl + dual audit | yes (Class 3 may not need if architect explicitly allows) |
| B3 CpmmPoolResolveTx | 1-2 days | yes |
| B4 CpmmLpUnwindTx | 2-3 days | yes |
| B5 Asymmetric seed | 1 day | depends on Class classification |
| B6 End-to-end real-LLM smoke | 0.5 day after B2-B5 | no |
| B7 Overall §8 cap | 1 day | yes (after all above) |
| **Total** | **~10-14 days** wall | 4 separate per-atom §8 events |

## §8. Critical-path summary

```
B0 charter (today) ✅ → B1 gap audit (today)
                       → architect §8 review of B2 scope (resolution authority option 1/2/3)
                       → B2 EventResolveTx (~1-2 day) → B2 §8
                       → B3 CpmmPoolResolveTx (~1-2 day) → B3 §8
                       → B4 CpmmLpUnwindTx (~2-3 day) → B4 §8
                       → B5 Asymmetric seed (~1 day) → B5 §8 or fold to B7
                       → B6 end-to-end real-LLM smoke (~0.5 day)
                       → B7 overall §8 cap
                       → CPMM ALGORITHM 完整落地 per architect Part C §2.1
```

Forward queue **after** TB-N2 ships (still requiring future architect §8):
- **A6 Polymarket-agent-bridge** — agent prompt tools for CompleteSet split / merge /
  CpmmSwap / BuyWithCoinRouter / LP add / LP unwind (B4 is admission only; agent
  prompt advertisement is A6)
- **RSP-M4 CLOB** — separate TB if Polymarket completeness later requires CLOB
- **K.1-K.6 Stage D readiness** — explicit architect ship gate
- **C.5 PromptCapsule evaluator wire-up** — orthogonal to Polymarket

## §9. Sign-off

- **Charter author**: Claude (autonomous Class-0 per `你来为我决定后续的方案`
  grant 2026-05-10 session #37)
- **Architect §8 status**: NOT-YET-REQUESTED. B0 + B1 are Class-0 documentation
  artifacts. B2 §8 request will follow B1 gap audit completion + Codex/Gemini
  PRE-§8 dual audit per `feedback_dual_audit` Class-3/4 timing rule.
- **Charter file**: this document (`handover/tracer_bullets/TB_N2_POLYMARKET_CPMM_LIFECYCLE_charter_2026-05-10.md`)
- **Companion gap audit**: `handover/audits/STAGE_C_POLYMARKET_CPMM_LIFECYCLE_GAP_AUDIT_2026-05-10.md`
- **TB_LOG.tsv update**: NOT-YET (charter draft state; TB_LOG entry created on
  first atom ship)

---

**End of TB-N2-POLYMARKET-CPMM-LIFECYCLE charter draft.**
